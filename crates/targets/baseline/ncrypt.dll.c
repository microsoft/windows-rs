void __stdcall GetKeyStorageInterface(int p0, int p1, int p2) {}
void __stdcall GetSChannelInterface(int p0, int p1, int p2) {}
void __stdcall NCryptCloseProtectionDescriptor(int p0) {}
void __stdcall NCryptCreateClaim(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7) {}
void __stdcall NCryptCreatePersistedKey(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall NCryptCreateProtectionDescriptor(int p0, int p1, int p2) {}
void __stdcall NCryptDecrypt(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7) {}
void __stdcall NCryptDeleteKey(int p0, int p1) {}
void __stdcall NCryptDeriveKey(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall NCryptEncrypt(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7) {}
void __stdcall NCryptEnumAlgorithms(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall NCryptEnumKeys(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall NCryptEnumStorageProviders(int p0, int p1, int p2) {}
void __stdcall NCryptExportKey(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7) {}
void __stdcall NCryptFinalizeKey(int p0, int p1) {}
void __stdcall NCryptFreeBuffer(int p0) {}
void __stdcall NCryptFreeObject(int p0) {}
void __stdcall NCryptGetProperty(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall NCryptGetProtectionDescriptorInfo(int p0, int p1, int p2, int p3) {}
void __stdcall NCryptImportKey(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7) {}
void __stdcall NCryptIsAlgSupported(int p0, int p1, int p2) {}
void __stdcall NCryptIsKeyHandle(int p0) {}
void __stdcall NCryptKeyDerivation(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall NCryptNotifyChangeKey(int p0, int p1, int p2) {}
void __stdcall NCryptOpenKey(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall NCryptOpenStorageProvider(int p0, int p1, int p2) {}
void __stdcall NCryptProtectSecret(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7) {}
void __stdcall NCryptQueryProtectionDescriptorName(int p0, int p1, int p2, int p3) {}
void __stdcall NCryptRegisterProtectionDescriptorName(int p0, int p1, int p2) {}
void __stdcall NCryptSecretAgreement(int p0, int p1, int p2, int p3) {}
void __stdcall NCryptSetProperty(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall NCryptSignHash(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7) {}
void __stdcall NCryptStreamClose(int p0) {}
void __stdcall NCryptStreamOpenToProtect(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall NCryptStreamOpenToUnprotect(int p0, int p1, int p2, int p3) {}
void __stdcall NCryptStreamOpenToUnprotectEx(int p0, int p1, int p2, int p3) {}
void __stdcall NCryptStreamUpdate(int p0, int p1, int p2, int p3) {}
void __stdcall NCryptTranslateHandle(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall NCryptUnprotectSecret(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7) {}
void __stdcall NCryptVerifyClaim(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7) {}
void __stdcall NCryptVerifySignature(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall SslChangeNotify(int p0, int p1) {}
void __stdcall SslComputeClientAuthHash(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7) {}
void __stdcall SslComputeEapKeyBlock(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7) {}
void __stdcall SslComputeFinishedHash(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall SslComputeSessionHash(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall SslCreateClientAuthHash(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall SslCreateEphemeralKey(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8) {}
void __stdcall SslCreateHandshakeHash(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall SslDecrementProviderReferenceCount(int p0) {}
void __stdcall SslDecryptPacket(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8, int p9) {}
void __stdcall SslDuplicateTranscriptHash(int p0, int p1, int p2, int p3) {}
void __stdcall SslEncryptPacket(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8, int p9, int p10) {}
void __stdcall SslEnumCipherSuites(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall SslEnumCipherSuitesEx(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall SslEnumEccCurves(int p0, int p1, int p2, int p3) {}
void __stdcall SslEnumProtocolProviders(int p0, int p1, int p2) {}
void __stdcall SslExpandBinderKey(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall SslExpandExporterMasterKey(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall SslExpandPreSharedKey(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall SslExpandResumptionMasterKey(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall SslExpandTrafficKeys(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall SslExpandWriteKey(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall SslExportKey(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall SslExportKeyingMaterial(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8, int p9) {}
void __stdcall SslExtractEarlyKey(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall SslExtractHandshakeKey(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall SslExtractMasterKey(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall SslFreeBuffer(int p0) {}
void __stdcall SslFreeObject(int p0, int p1) {}
void __stdcall SslGenerateMasterKey(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8, int p9, int p10) {}
void __stdcall SslGeneratePreMasterKey(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8, int p9) {}
void __stdcall SslGenerateSessionKeys(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall SslGetCipherSuitePRFHashAlgorithm(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall SslGetKeyProperty(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall SslGetProviderProperty(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall SslHashHandshake(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall SslImportKey(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall SslImportMasterKey(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8) {}
void __stdcall SslIncrementProviderReferenceCount(int p0) {}
void __stdcall SslLookupCipherLengths(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall SslLookupCipherSuiteInfo(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall SslOpenPrivateKey(int p0, int p1, int p2, int p3) {}
void __stdcall SslOpenProvider(int p0, int p1, int p2) {}
void __stdcall SslSignHash(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7) {}
void __stdcall SslVerifySignature(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
