void __stdcall AcceptSecurityContext(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8) {}
void __stdcall AcquireCredentialsHandleA(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8) {}
void __stdcall AcquireCredentialsHandleW(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8) {}
void __stdcall AddCredentialsA(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7) {}
void __stdcall AddCredentialsW(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7) {}
void __stdcall AddSecurityPackageA(int p0, int p1) {}
void __stdcall AddSecurityPackageW(int p0, int p1) {}
void __stdcall ApplyControlToken(int p0, int p1) {}
void __stdcall ChangeAccountPasswordA(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7) {}
void __stdcall ChangeAccountPasswordW(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7) {}
void __stdcall CompleteAuthToken(int p0, int p1) {}
void __stdcall CredMarshalTargetInfo(int p0, int p1, int p2) {}
void __stdcall CredUnmarshalTargetInfo(int p0, int p1, int p2, int p3) {}
void __stdcall DecryptMessage(int p0, int p1, int p2, int p3) {}
void __stdcall DeleteSecurityContext(int p0) {}
void __stdcall DeleteSecurityPackageA(int p0) {}
void __stdcall DeleteSecurityPackageW(int p0) {}
void __stdcall EncryptMessage(int p0, int p1, int p2, int p3) {}
void __stdcall EnumerateSecurityPackagesA(int p0, int p1) {}
void __stdcall EnumerateSecurityPackagesW(int p0, int p1) {}
void __stdcall ExportSecurityContext(int p0, int p1, int p2, int p3) {}
void __stdcall FreeContextBuffer(int p0) {}
void __stdcall FreeCredentialsHandle(int p0) {}
void __stdcall GetComputerObjectNameA(int p0, int p1, int p2) {}
void __stdcall GetComputerObjectNameW(int p0, int p1, int p2) {}
void __stdcall GetSecurityUserInfo(int p0, int p1, int p2) {}
void __stdcall GetUserNameExA(int p0, int p1, int p2) {}
void __stdcall GetUserNameExW(int p0, int p1, int p2) {}
void __stdcall ImpersonateSecurityContext(int p0) {}
void __stdcall ImportSecurityContextA(int p0, int p1, int p2, int p3) {}
void __stdcall ImportSecurityContextW(int p0, int p1, int p2, int p3) {}
void __stdcall InitSecurityInterfaceA() {}
void __stdcall InitSecurityInterfaceW() {}
void __stdcall InitializeSecurityContextA(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8, int p9, int p10, int p11) {}
void __stdcall InitializeSecurityContextW(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8, int p9, int p10, int p11) {}
void __stdcall LsaCallAuthenticationPackage(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall LsaConnectUntrusted(int p0) {}
void __stdcall LsaDeregisterLogonProcess(int p0) {}
void __stdcall LsaEnumerateLogonSessions(int p0, int p1) {}
void __stdcall LsaFreeReturnBuffer(int p0) {}
void __stdcall LsaGetLogonSessionData(int p0, int p1) {}
void __stdcall LsaLogonUser(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8, int p9, int p10, int p11, int p12, int p13) {}
void __stdcall LsaLookupAuthenticationPackage(int p0, int p1, int p2) {}
void __stdcall LsaRegisterLogonProcess(int p0, int p1, int p2) {}
void __stdcall LsaRegisterPolicyChangeNotification(int p0, int p1) {}
void __stdcall LsaUnregisterPolicyChangeNotification(int p0, int p1) {}
void __stdcall MakeSignature(int p0, int p1, int p2, int p3) {}
void __stdcall QueryContextAttributesA(int p0, int p1, int p2) {}
void __stdcall QueryContextAttributesW(int p0, int p1, int p2) {}
void __stdcall QueryCredentialsAttributesA(int p0, int p1, int p2) {}
void __stdcall QueryCredentialsAttributesW(int p0, int p1, int p2) {}
void __stdcall QuerySecurityContextToken(int p0, int p1) {}
void __stdcall QuerySecurityPackageInfoA(int p0, int p1) {}
void __stdcall QuerySecurityPackageInfoW(int p0, int p1) {}
void __stdcall RevertSecurityContext(int p0) {}
void __stdcall SaslAcceptSecurityContext(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8) {}
void __stdcall SaslEnumerateProfilesA(int p0, int p1) {}
void __stdcall SaslEnumerateProfilesW(int p0, int p1) {}
void __stdcall SaslGetContextOption(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall SaslGetProfilePackageA(int p0, int p1) {}
void __stdcall SaslGetProfilePackageW(int p0, int p1) {}
void __stdcall SaslIdentifyPackageA(int p0, int p1) {}
void __stdcall SaslIdentifyPackageW(int p0, int p1) {}
void __stdcall SaslInitializeSecurityContextA(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8, int p9, int p10, int p11) {}
void __stdcall SaslInitializeSecurityContextW(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8, int p9, int p10, int p11) {}
void __stdcall SaslSetContextOption(int p0, int p1, int p2, int p3) {}
void __stdcall SetContextAttributesA(int p0, int p1, int p2, int p3) {}
void __stdcall SetContextAttributesW(int p0, int p1, int p2, int p3) {}
void __stdcall SetCredentialsAttributesA(int p0, int p1, int p2, int p3) {}
void __stdcall SetCredentialsAttributesW(int p0, int p1, int p2, int p3) {}
void __stdcall SspiCompareAuthIdentities(int p0, int p1, int p2, int p3) {}
void __stdcall SspiCopyAuthIdentity(int p0, int p1) {}
void __stdcall SspiDecryptAuthIdentity(int p0) {}
void __stdcall SspiEncodeAuthIdentityAsStrings(int p0, int p1, int p2, int p3) {}
void __stdcall SspiEncodeStringsAsAuthIdentity(int p0, int p1, int p2, int p3) {}
void __stdcall SspiEncryptAuthIdentity(int p0) {}
void __stdcall SspiExcludePackage(int p0, int p1, int p2) {}
void __stdcall SspiFreeAuthIdentity(int p0) {}
void __stdcall SspiGetTargetHostName(int p0, int p1) {}
void __stdcall SspiIsAuthIdentityEncrypted(int p0) {}
void __stdcall SspiLocalFree(int p0) {}
void __stdcall SspiMarshalAuthIdentity(int p0, int p1, int p2) {}
void __stdcall SspiPrepareForCredRead(int p0, int p1, int p2, int p3) {}
void __stdcall SspiPrepareForCredWrite(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall SspiUnmarshalAuthIdentity(int p0, int p1, int p2) {}
void __stdcall SspiValidateAuthIdentity(int p0) {}
void __stdcall SspiZeroAuthIdentity(int p0) {}
void __stdcall TranslateNameA(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall TranslateNameW(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall VerifySignature(int p0, int p1, int p2, int p3) {}
