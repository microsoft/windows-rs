void __stdcall CertAddCRLContextToStore(int p0, int p1, int p2, int p3) {}
void __stdcall CertAddCRLLinkToStore(int p0, int p1, int p2, int p3) {}
void __stdcall CertAddCTLContextToStore(int p0, int p1, int p2, int p3) {}
void __stdcall CertAddCTLLinkToStore(int p0, int p1, int p2, int p3) {}
void __stdcall CertAddCertificateContextToStore(int p0, int p1, int p2, int p3) {}
void __stdcall CertAddCertificateLinkToStore(int p0, int p1, int p2, int p3) {}
void __stdcall CertAddEncodedCRLToStore(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall CertAddEncodedCTLToStore(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall CertAddEncodedCertificateToStore(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall CertAddEncodedCertificateToSystemStoreA(int p0, int p1, int p2) {}
void __stdcall CertAddEncodedCertificateToSystemStoreW(int p0, int p1, int p2) {}
void __stdcall CertAddEnhancedKeyUsageIdentifier(int p0, int p1) {}
void __stdcall CertAddRefServerOcspResponse(int p0) {}
void __stdcall CertAddRefServerOcspResponseContext(int p0) {}
void __stdcall CertAddSerializedElementToStore(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7) {}
void __stdcall CertAddStoreToCollection(int p0, int p1, int p2, int p3) {}
void __stdcall CertAlgIdToOID(int p0) {}
void __stdcall CertCloseServerOcspResponse(int p0, int p1) {}
void __stdcall CertCloseStore(int p0, int p1) {}
void __stdcall CertCompareCertificate(int p0, int p1, int p2) {}
void __stdcall CertCompareCertificateName(int p0, int p1, int p2) {}
void __stdcall CertCompareIntegerBlob(int p0, int p1) {}
void __stdcall CertComparePublicKeyInfo(int p0, int p1, int p2) {}
void __stdcall CertControlStore(int p0, int p1, int p2, int p3) {}
void __stdcall CertCreateCRLContext(int p0, int p1, int p2) {}
void __stdcall CertCreateCTLContext(int p0, int p1, int p2) {}
void __stdcall CertCreateCTLEntryFromCertificateContextProperties(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall CertCreateCertificateChainEngine(int p0, int p1) {}
void __stdcall CertCreateCertificateContext(int p0, int p1, int p2) {}
void __stdcall CertCreateContext(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall CertCreateSelfSignCertificate(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7) {}
void __stdcall CertDeleteCRLFromStore(int p0) {}
void __stdcall CertDeleteCTLFromStore(int p0) {}
void __stdcall CertDeleteCertificateFromStore(int p0) {}
void __stdcall CertDuplicateCRLContext(int p0) {}
void __stdcall CertDuplicateCTLContext(int p0) {}
void __stdcall CertDuplicateCertificateChain(int p0) {}
void __stdcall CertDuplicateCertificateContext(int p0) {}
void __stdcall CertDuplicateStore(int p0) {}
void __stdcall CertEnumCRLContextProperties(int p0, int p1) {}
void __stdcall CertEnumCRLsInStore(int p0, int p1) {}
void __stdcall CertEnumCTLContextProperties(int p0, int p1) {}
void __stdcall CertEnumCTLsInStore(int p0, int p1) {}
void __stdcall CertEnumCertificateContextProperties(int p0, int p1) {}
void __stdcall CertEnumCertificatesInStore(int p0, int p1) {}
void __stdcall CertEnumPhysicalStore(int p0, int p1, int p2, int p3) {}
void __stdcall CertEnumSubjectInSortedCTL(int p0, int p1, int p2, int p3) {}
void __stdcall CertEnumSystemStore(int p0, int p1, int p2, int p3) {}
void __stdcall CertEnumSystemStoreLocation(int p0, int p1, int p2) {}
void __stdcall CertFindAttribute(int p0, int p1, int p2) {}
void __stdcall CertFindCRLInStore(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall CertFindCTLInStore(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall CertFindCertificateInCRL(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall CertFindCertificateInStore(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall CertFindChainInStore(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall CertFindExtension(int p0, int p1, int p2) {}
void __stdcall CertFindRDNAttr(int p0, int p1) {}
void __stdcall CertFindSubjectInCTL(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall CertFindSubjectInSortedCTL(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall CertFreeCRLContext(int p0) {}
void __stdcall CertFreeCTLContext(int p0) {}
void __stdcall CertFreeCertificateChain(int p0) {}
void __stdcall CertFreeCertificateChainEngine(int p0) {}
void __stdcall CertFreeCertificateChainList(int p0) {}
void __stdcall CertFreeCertificateContext(int p0) {}
void __stdcall CertFreeServerOcspResponseContext(int p0) {}
void __stdcall CertGetCRLContextProperty(int p0, int p1, int p2, int p3) {}
void __stdcall CertGetCRLFromStore(int p0, int p1, int p2, int p3) {}
void __stdcall CertGetCTLContextProperty(int p0, int p1, int p2, int p3) {}
void __stdcall CertGetCertificateChain(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7) {}
void __stdcall CertGetCertificateContextProperty(int p0, int p1, int p2, int p3) {}
void __stdcall CertGetEnhancedKeyUsage(int p0, int p1, int p2, int p3) {}
void __stdcall CertGetIntendedKeyUsage(int p0, int p1, int p2, int p3) {}
void __stdcall CertGetIssuerCertificateFromStore(int p0, int p1, int p2, int p3) {}
void __stdcall CertGetNameStringA(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall CertGetNameStringW(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall CertGetPublicKeyLength(int p0, int p1) {}
void __stdcall CertGetServerOcspResponseContext(int p0, int p1, int p2) {}
void __stdcall CertGetStoreProperty(int p0, int p1, int p2, int p3) {}
void __stdcall CertGetSubjectCertificateFromStore(int p0, int p1, int p2) {}
void __stdcall CertGetValidUsages(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall CertIsRDNAttrsInCertificateName(int p0, int p1, int p2, int p3) {}
void __stdcall CertIsStrongHashToSign(int p0, int p1, int p2) {}
void __stdcall CertIsValidCRLForCertificate(int p0, int p1, int p2, int p3) {}
void __stdcall CertIsWeakHash(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall CertNameToStrA(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall CertNameToStrW(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall CertOIDToAlgId(int p0) {}
void __stdcall CertOpenServerOcspResponse(int p0, int p1, int p2) {}
void __stdcall CertOpenStore(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall CertOpenSystemStoreA(int p0, int p1) {}
void __stdcall CertOpenSystemStoreW(int p0, int p1) {}
void __stdcall CertRDNValueToStrA(int p0, int p1, int p2, int p3) {}
void __stdcall CertRDNValueToStrW(int p0, int p1, int p2, int p3) {}
void __stdcall CertRegisterPhysicalStore(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall CertRegisterSystemStore(int p0, int p1, int p2, int p3) {}
void __stdcall CertRemoveEnhancedKeyUsageIdentifier(int p0, int p1) {}
void __stdcall CertRemoveStoreFromCollection(int p0, int p1) {}
void __stdcall CertResyncCertificateChainEngine(int p0) {}
void __stdcall CertRetrieveLogoOrBiometricInfo(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8) {}
void __stdcall CertSaveStore(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall CertSelectCertificateChains(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7) {}
void __stdcall CertSerializeCRLStoreElement(int p0, int p1, int p2, int p3) {}
void __stdcall CertSerializeCTLStoreElement(int p0, int p1, int p2, int p3) {}
void __stdcall CertSerializeCertificateStoreElement(int p0, int p1, int p2, int p3) {}
void __stdcall CertSetCRLContextProperty(int p0, int p1, int p2, int p3) {}
void __stdcall CertSetCTLContextProperty(int p0, int p1, int p2, int p3) {}
void __stdcall CertSetCertificateContextPropertiesFromCTLEntry(int p0, int p1, int p2) {}
void __stdcall CertSetCertificateContextProperty(int p0, int p1, int p2, int p3) {}
void __stdcall CertSetEnhancedKeyUsage(int p0, int p1) {}
void __stdcall CertSetStoreProperty(int p0, int p1, int p2, int p3) {}
void __stdcall CertStrToNameA(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall CertStrToNameW(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall CertUnregisterPhysicalStore(int p0, int p1, int p2) {}
void __stdcall CertUnregisterSystemStore(int p0, int p1) {}
void __stdcall CertVerifyCRLRevocation(int p0, int p1, int p2, int p3) {}
void __stdcall CertVerifyCRLTimeValidity(int p0, int p1) {}
void __stdcall CertVerifyCTLUsage(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall CertVerifyCertificateChainPolicy(int p0, int p1, int p2, int p3) {}
void __stdcall CertVerifyRevocation(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall CertVerifySubjectCertificateContext(int p0, int p1, int p2) {}
void __stdcall CertVerifyTimeValidity(int p0, int p1) {}
void __stdcall CertVerifyValidityNesting(int p0, int p1) {}
void __stdcall CryptAcquireCertificatePrivateKey(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall CryptBinaryToStringA(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall CryptBinaryToStringW(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall CryptCloseAsyncHandle(int p0) {}
void __stdcall CryptCreateAsyncHandle(int p0, int p1) {}
void __stdcall CryptCreateKeyIdentifierFromCSP(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7) {}
void __stdcall CryptDecodeMessage(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8, int p9, int p10, int p11, int p12) {}
void __stdcall CryptDecodeObject(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall CryptDecodeObjectEx(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7) {}
void __stdcall CryptDecryptAndVerifyMessageSignature(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8) {}
void __stdcall CryptDecryptMessage(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall CryptEncodeObject(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall CryptEncodeObjectEx(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall CryptEncryptMessage(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall CryptEnumKeyIdentifierProperties(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall CryptEnumOIDFunction(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall CryptEnumOIDInfo(int p0, int p1, int p2, int p3) {}
void __stdcall CryptExportPKCS8(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall CryptExportPublicKeyInfo(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall CryptExportPublicKeyInfoEx(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7) {}
void __stdcall CryptExportPublicKeyInfoFromBCryptKeyHandle(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall CryptFindCertificateKeyProvInfo(int p0, int p1, int p2) {}
void __stdcall CryptFindLocalizedName(int p0) {}
void __stdcall CryptFindOIDInfo(int p0, int p1, int p2) {}
void __stdcall CryptFormatObject(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8) {}
void __stdcall CryptFreeOIDFunctionAddress(int p0, int p1) {}
void __stdcall CryptGetAsyncParam(int p0, int p1, int p2, int p3) {}
void __stdcall CryptGetDefaultOIDDllList(int p0, int p1, int p2, int p3) {}
void __stdcall CryptGetDefaultOIDFunctionAddress(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall CryptGetKeyIdentifierProperty(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall CryptGetMessageCertificates(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall CryptGetMessageSignerCount(int p0, int p1, int p2) {}
void __stdcall CryptGetOIDFunctionAddress(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall CryptGetOIDFunctionValue(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall CryptHashCertificate(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall CryptHashCertificate2(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall CryptHashMessage(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8) {}
void __stdcall CryptHashPublicKeyInfo(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall CryptHashToBeSigned(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall CryptImportPKCS8(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8) {}
void __stdcall CryptImportPublicKeyInfo(int p0, int p1, int p2, int p3) {}
void __stdcall CryptImportPublicKeyInfoEx(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall CryptImportPublicKeyInfoEx2(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall CryptInitOIDFunctionSet(int p0, int p1) {}
void __stdcall CryptInstallDefaultContext(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall CryptInstallOIDFunctionAddress(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall CryptMemAlloc(int p0) {}
void __stdcall CryptMemFree(int p0) {}
void __stdcall CryptMemRealloc(int p0, int p1) {}
void __stdcall CryptMsgCalculateEncodedLength(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall CryptMsgClose(int p0) {}
void __stdcall CryptMsgControl(int p0, int p1, int p2, int p3) {}
void __stdcall CryptMsgCountersign(int p0, int p1, int p2, int p3) {}
void __stdcall CryptMsgCountersignEncoded(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall CryptMsgDuplicate(int p0) {}
void __stdcall CryptMsgEncodeAndSignCTL(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall CryptMsgGetAndVerifySigner(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall CryptMsgGetParam(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall CryptMsgOpenToDecode(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall CryptMsgOpenToEncode(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall CryptMsgSignCTL(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall CryptMsgUpdate(int p0, int p1, int p2, int p3) {}
void __stdcall CryptMsgVerifyCountersignatureEncoded(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall CryptMsgVerifyCountersignatureEncodedEx(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8, int p9) {}
void __stdcall CryptProtectData(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall CryptProtectMemory(int p0, int p1, int p2) {}
void __stdcall CryptQueryObject(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8, int p9, int p10) {}
void __stdcall CryptRegisterDefaultOIDFunction(int p0, int p1, int p2, int p3) {}
void __stdcall CryptRegisterOIDFunction(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall CryptRegisterOIDInfo(int p0, int p1) {}
void __stdcall CryptRetrieveTimeStamp(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8, int p9) {}
void __stdcall CryptSIPAddProvider(int p0) {}
void __stdcall CryptSIPLoad(int p0, int p1, int p2) {}
void __stdcall CryptSIPRemoveProvider(int p0) {}
void __stdcall CryptSIPRetrieveSubjectGuid(int p0, int p1, int p2) {}
void __stdcall CryptSIPRetrieveSubjectGuidForCatalogFile(int p0, int p1, int p2) {}
void __stdcall CryptSetAsyncParam(int p0, int p1, int p2, int p3) {}
void __stdcall CryptSetKeyIdentifierProperty(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall CryptSetOIDFunctionValue(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall CryptSignAndEncodeCertificate(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8) {}
void __stdcall CryptSignAndEncryptMessage(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7) {}
void __stdcall CryptSignCertificate(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8) {}
void __stdcall CryptSignMessage(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall CryptSignMessageWithKey(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall CryptStringToBinaryA(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall CryptStringToBinaryW(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall CryptUninstallDefaultContext(int p0, int p1, int p2) {}
void __stdcall CryptUnprotectData(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall CryptUnprotectMemory(int p0, int p1, int p2) {}
void __stdcall CryptUnregisterDefaultOIDFunction(int p0, int p1, int p2) {}
void __stdcall CryptUnregisterOIDFunction(int p0, int p1, int p2) {}
void __stdcall CryptUnregisterOIDInfo(int p0) {}
void __stdcall CryptUpdateProtectedState(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall CryptVerifyCertificateSignature(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall CryptVerifyCertificateSignatureEx(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7) {}
void __stdcall CryptVerifyDetachedMessageHash(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7) {}
void __stdcall CryptVerifyDetachedMessageSignature(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7) {}
void __stdcall CryptVerifyMessageHash(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall CryptVerifyMessageSignature(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall CryptVerifyMessageSignatureWithKey(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall CryptVerifyTimeStampSignature(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7) {}
void __stdcall PFXExportCertStore(int p0, int p1, int p2, int p3) {}
void __stdcall PFXExportCertStoreEx(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall PFXImportCertStore(int p0, int p1, int p2) {}
void __stdcall PFXIsPFXBlob(int p0) {}
void __stdcall PFXVerifyPassword(int p0, int p1, int p2) {}
