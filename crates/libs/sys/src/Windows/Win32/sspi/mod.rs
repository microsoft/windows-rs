#[cfg(feature = "Win32_ncrypt")]
windows_link::link!("secur32.dll" "system" fn AcceptSecurityContext(phcredential : PCredHandle, phcontext : PCtxtHandle, pinput : *const SecBufferDesc, fcontextreq : u32, targetdatarep : u32, phnewcontext : PCtxtHandle, poutput : *mut SecBufferDesc, pfcontextattr : *mut u32, ptsexpiry : *mut SECURITY_INTEGER) -> super::ncrypt::SECURITY_STATUS);
#[cfg(feature = "Win32_ncrypt")]
windows_link::link!("secur32.dll" "system" fn AcquireCredentialsHandleA(pszprincipal : windows_sys::core::PCSTR, pszpackage : windows_sys::core::PCSTR, fcredentialuse : u32, pvlogonid : *const core::ffi::c_void, pauthdata : *const core::ffi::c_void, pgetkeyfn : SEC_GET_KEY_FN, pvgetkeyargument : *const core::ffi::c_void, phcredential : PCredHandle, ptsexpiry : *mut SECURITY_INTEGER) -> super::ncrypt::SECURITY_STATUS);
#[cfg(feature = "Win32_ncrypt")]
windows_link::link!("secur32.dll" "system" fn AcquireCredentialsHandleW(pszprincipal : windows_sys::core::PCWSTR, pszpackage : windows_sys::core::PCWSTR, fcredentialuse : u32, pvlogonid : *const core::ffi::c_void, pauthdata : *const core::ffi::c_void, pgetkeyfn : SEC_GET_KEY_FN, pvgetkeyargument : *const core::ffi::c_void, phcredential : PCredHandle, ptsexpiry : *mut SECURITY_INTEGER) -> super::ncrypt::SECURITY_STATUS);
#[cfg(feature = "Win32_ncrypt")]
windows_link::link!("secur32.dll" "system" fn AddCredentialsA(hcredentials : PCredHandle, pszprincipal : windows_sys::core::PCSTR, pszpackage : windows_sys::core::PCSTR, fcredentialuse : u32, pauthdata : *const core::ffi::c_void, pgetkeyfn : SEC_GET_KEY_FN, pvgetkeyargument : *const core::ffi::c_void, ptsexpiry : *mut SECURITY_INTEGER) -> super::ncrypt::SECURITY_STATUS);
#[cfg(feature = "Win32_ncrypt")]
windows_link::link!("secur32.dll" "system" fn AddCredentialsW(hcredentials : PCredHandle, pszprincipal : windows_sys::core::PCWSTR, pszpackage : windows_sys::core::PCWSTR, fcredentialuse : u32, pauthdata : *const core::ffi::c_void, pgetkeyfn : SEC_GET_KEY_FN, pvgetkeyargument : *const core::ffi::c_void, ptsexpiry : *mut SECURITY_INTEGER) -> super::ncrypt::SECURITY_STATUS);
#[cfg(feature = "Win32_ncrypt")]
windows_link::link!("secur32.dll" "system" fn AddSecurityPackageA(pszpackagename : windows_sys::core::PCSTR, poptions : *const SECURITY_PACKAGE_OPTIONS) -> super::ncrypt::SECURITY_STATUS);
#[cfg(feature = "Win32_ncrypt")]
windows_link::link!("secur32.dll" "system" fn AddSecurityPackageW(pszpackagename : windows_sys::core::PCWSTR, poptions : *const SECURITY_PACKAGE_OPTIONS) -> super::ncrypt::SECURITY_STATUS);
#[cfg(feature = "Win32_ncrypt")]
windows_link::link!("secur32.dll" "system" fn ApplyControlToken(phcontext : PCtxtHandle, pinput : *const SecBufferDesc) -> super::ncrypt::SECURITY_STATUS);
#[cfg(feature = "Win32_ncrypt")]
windows_link::link!("secur32.dll" "system" fn ChangeAccountPasswordA(pszpackagename : *const SEC_CHAR, pszdomainname : *const SEC_CHAR, pszaccountname : *const SEC_CHAR, pszoldpassword : *const SEC_CHAR, psznewpassword : *const SEC_CHAR, bimpersonating : bool, dwreserved : u32, poutput : *mut SecBufferDesc) -> super::ncrypt::SECURITY_STATUS);
#[cfg(feature = "Win32_ncrypt")]
windows_link::link!("secur32.dll" "system" fn ChangeAccountPasswordW(pszpackagename : *const SEC_WCHAR, pszdomainname : *const SEC_WCHAR, pszaccountname : *const SEC_WCHAR, pszoldpassword : *const SEC_WCHAR, psznewpassword : *const SEC_WCHAR, bimpersonating : bool, dwreserved : u32, poutput : *mut SecBufferDesc) -> super::ncrypt::SECURITY_STATUS);
#[cfg(feature = "Win32_ncrypt")]
windows_link::link!("secur32.dll" "system" fn CompleteAuthToken(phcontext : PCtxtHandle, ptoken : *const SecBufferDesc) -> super::ncrypt::SECURITY_STATUS);
#[cfg(feature = "Win32_ncrypt")]
windows_link::link!("secur32.dll" "system" fn DecryptMessage(phcontext : PCtxtHandle, pmessage : *const SecBufferDesc, messageseqno : u32, pfqop : *mut u32) -> super::ncrypt::SECURITY_STATUS);
#[cfg(feature = "Win32_ncrypt")]
windows_link::link!("secur32.dll" "system" fn DeleteSecurityContext(phcontext : PCtxtHandle) -> super::ncrypt::SECURITY_STATUS);
#[cfg(feature = "Win32_ncrypt")]
windows_link::link!("secur32.dll" "system" fn DeleteSecurityPackageA(pszpackagename : windows_sys::core::PCSTR) -> super::ncrypt::SECURITY_STATUS);
#[cfg(feature = "Win32_ncrypt")]
windows_link::link!("secur32.dll" "system" fn DeleteSecurityPackageW(pszpackagename : windows_sys::core::PCWSTR) -> super::ncrypt::SECURITY_STATUS);
#[cfg(feature = "Win32_ncrypt")]
windows_link::link!("secur32.dll" "system" fn EncryptMessage(phcontext : PCtxtHandle, fqop : u32, pmessage : *const SecBufferDesc, messageseqno : u32) -> super::ncrypt::SECURITY_STATUS);
#[cfg(feature = "Win32_ncrypt")]
windows_link::link!("secur32.dll" "system" fn EnumerateSecurityPackagesA(pcpackages : *mut u32, pppackageinfo : *mut PSecPkgInfoA) -> super::ncrypt::SECURITY_STATUS);
#[cfg(feature = "Win32_ncrypt")]
windows_link::link!("secur32.dll" "system" fn EnumerateSecurityPackagesW(pcpackages : *mut u32, pppackageinfo : *mut PSecPkgInfoW) -> super::ncrypt::SECURITY_STATUS);
#[cfg(feature = "Win32_ncrypt")]
windows_link::link!("secur32.dll" "system" fn ExportSecurityContext(phcontext : PCtxtHandle, fflags : u32, ppackedcontext : *mut SecBuffer, ptoken : *mut *mut core::ffi::c_void) -> super::ncrypt::SECURITY_STATUS);
#[cfg(feature = "Win32_ncrypt")]
windows_link::link!("secur32.dll" "system" fn FreeContextBuffer(pvcontextbuffer : *mut core::ffi::c_void) -> super::ncrypt::SECURITY_STATUS);
#[cfg(feature = "Win32_ncrypt")]
windows_link::link!("secur32.dll" "system" fn FreeCredentialsHandle(phcredential : PCredHandle) -> super::ncrypt::SECURITY_STATUS);
#[cfg(feature = "Win32_ncrypt")]
windows_link::link!("secur32.dll" "system" fn ImpersonateSecurityContext(phcontext : PCtxtHandle) -> super::ncrypt::SECURITY_STATUS);
#[cfg(feature = "Win32_ncrypt")]
windows_link::link!("secur32.dll" "system" fn ImportSecurityContextA(pszpackage : windows_sys::core::PCSTR, ppackedcontext : *const SecBuffer, token : *const core::ffi::c_void, phcontext : PCtxtHandle) -> super::ncrypt::SECURITY_STATUS);
#[cfg(feature = "Win32_ncrypt")]
windows_link::link!("secur32.dll" "system" fn ImportSecurityContextW(pszpackage : windows_sys::core::PCWSTR, ppackedcontext : *const SecBuffer, token : *const core::ffi::c_void, phcontext : PCtxtHandle) -> super::ncrypt::SECURITY_STATUS);
#[cfg(feature = "Win32_ncrypt")]
windows_link::link!("secur32.dll" "system" fn InitSecurityInterfaceA() -> PSecurityFunctionTableA);
#[cfg(feature = "Win32_ncrypt")]
windows_link::link!("secur32.dll" "system" fn InitSecurityInterfaceW() -> PSecurityFunctionTableW);
#[cfg(feature = "Win32_ncrypt")]
windows_link::link!("secur32.dll" "system" fn InitializeSecurityContextA(phcredential : PCredHandle, phcontext : PCtxtHandle, psztargetname : *const SEC_CHAR, fcontextreq : u32, reserved1 : u32, targetdatarep : u32, pinput : *const SecBufferDesc, reserved2 : u32, phnewcontext : PCtxtHandle, poutput : *mut SecBufferDesc, pfcontextattr : *mut u32, ptsexpiry : *mut SECURITY_INTEGER) -> super::ncrypt::SECURITY_STATUS);
#[cfg(feature = "Win32_ncrypt")]
windows_link::link!("secur32.dll" "system" fn InitializeSecurityContextW(phcredential : PCredHandle, phcontext : PCtxtHandle, psztargetname : *const SEC_WCHAR, fcontextreq : u32, reserved1 : u32, targetdatarep : u32, pinput : *const SecBufferDesc, reserved2 : u32, phnewcontext : PCtxtHandle, poutput : *mut SecBufferDesc, pfcontextattr : *mut u32, ptsexpiry : *mut SECURITY_INTEGER) -> super::ncrypt::SECURITY_STATUS);
#[cfg(feature = "Win32_ncrypt")]
windows_link::link!("secur32.dll" "system" fn MakeSignature(phcontext : PCtxtHandle, fqop : u32, pmessage : *const SecBufferDesc, messageseqno : u32) -> super::ncrypt::SECURITY_STATUS);
#[cfg(feature = "Win32_ncrypt")]
windows_link::link!("secur32.dll" "system" fn QueryContextAttributesA(phcontext : PCtxtHandle, ulattribute : u32, pbuffer : *mut core::ffi::c_void) -> super::ncrypt::SECURITY_STATUS);
#[cfg(feature = "Win32_ncrypt")]
windows_link::link!("sspicli.dll" "system" fn QueryContextAttributesExA(phcontext : PCtxtHandle, ulattribute : u32, pbuffer : *mut core::ffi::c_void, cbbuffer : u32) -> super::ncrypt::SECURITY_STATUS);
#[cfg(feature = "Win32_ncrypt")]
windows_link::link!("sspicli.dll" "system" fn QueryContextAttributesExW(phcontext : PCtxtHandle, ulattribute : u32, pbuffer : *mut core::ffi::c_void, cbbuffer : u32) -> super::ncrypt::SECURITY_STATUS);
#[cfg(feature = "Win32_ncrypt")]
windows_link::link!("secur32.dll" "system" fn QueryContextAttributesW(phcontext : PCtxtHandle, ulattribute : u32, pbuffer : *mut core::ffi::c_void) -> super::ncrypt::SECURITY_STATUS);
#[cfg(feature = "Win32_ncrypt")]
windows_link::link!("secur32.dll" "system" fn QueryCredentialsAttributesA(phcredential : PCredHandle, ulattribute : u32, pbuffer : *mut core::ffi::c_void) -> super::ncrypt::SECURITY_STATUS);
#[cfg(feature = "Win32_ncrypt")]
windows_link::link!("sspicli.dll" "system" fn QueryCredentialsAttributesExA(phcredential : PCredHandle, ulattribute : u32, pbuffer : *mut core::ffi::c_void, cbbuffer : u32) -> super::ncrypt::SECURITY_STATUS);
#[cfg(feature = "Win32_ncrypt")]
windows_link::link!("sspicli.dll" "system" fn QueryCredentialsAttributesExW(phcredential : PCredHandle, ulattribute : u32, pbuffer : *mut core::ffi::c_void, cbbuffer : u32) -> super::ncrypt::SECURITY_STATUS);
#[cfg(feature = "Win32_ncrypt")]
windows_link::link!("secur32.dll" "system" fn QueryCredentialsAttributesW(phcredential : PCredHandle, ulattribute : u32, pbuffer : *mut core::ffi::c_void) -> super::ncrypt::SECURITY_STATUS);
#[cfg(feature = "Win32_ncrypt")]
windows_link::link!("secur32.dll" "system" fn QuerySecurityContextToken(phcontext : PCtxtHandle, token : *mut *mut core::ffi::c_void) -> super::ncrypt::SECURITY_STATUS);
#[cfg(feature = "Win32_ncrypt")]
windows_link::link!("secur32.dll" "system" fn QuerySecurityPackageInfoA(pszpackagename : windows_sys::core::PCSTR, pppackageinfo : *mut PSecPkgInfoA) -> super::ncrypt::SECURITY_STATUS);
#[cfg(feature = "Win32_ncrypt")]
windows_link::link!("secur32.dll" "system" fn QuerySecurityPackageInfoW(pszpackagename : windows_sys::core::PCWSTR, pppackageinfo : *mut PSecPkgInfoW) -> super::ncrypt::SECURITY_STATUS);
#[cfg(feature = "Win32_ncrypt")]
windows_link::link!("secur32.dll" "system" fn RevertSecurityContext(phcontext : PCtxtHandle) -> super::ncrypt::SECURITY_STATUS);
#[cfg(feature = "Win32_ncrypt")]
windows_link::link!("secur32.dll" "system" fn SaslAcceptSecurityContext(phcredential : PCredHandle, phcontext : PCtxtHandle, pinput : *const SecBufferDesc, fcontextreq : u32, targetdatarep : u32, phnewcontext : PCtxtHandle, poutput : *mut SecBufferDesc, pfcontextattr : *mut u32, ptsexpiry : *mut SECURITY_INTEGER) -> super::ncrypt::SECURITY_STATUS);
#[cfg(feature = "Win32_ncrypt")]
windows_link::link!("secur32.dll" "system" fn SaslEnumerateProfilesA(profilelist : *mut windows_sys::core::PSTR, profilecount : *mut u32) -> super::ncrypt::SECURITY_STATUS);
#[cfg(feature = "Win32_ncrypt")]
windows_link::link!("secur32.dll" "system" fn SaslEnumerateProfilesW(profilelist : *mut windows_sys::core::PWSTR, profilecount : *mut u32) -> super::ncrypt::SECURITY_STATUS);
#[cfg(feature = "Win32_ncrypt")]
windows_link::link!("secur32.dll" "system" fn SaslGetContextOption(contexthandle : PCtxtHandle, option : u32, value : *mut core::ffi::c_void, size : u32, needed : *mut u32) -> super::ncrypt::SECURITY_STATUS);
#[cfg(feature = "Win32_ncrypt")]
windows_link::link!("secur32.dll" "system" fn SaslGetProfilePackageA(profilename : windows_sys::core::PCSTR, packageinfo : *mut PSecPkgInfoA) -> super::ncrypt::SECURITY_STATUS);
#[cfg(feature = "Win32_ncrypt")]
windows_link::link!("secur32.dll" "system" fn SaslGetProfilePackageW(profilename : windows_sys::core::PCWSTR, packageinfo : *mut PSecPkgInfoW) -> super::ncrypt::SECURITY_STATUS);
#[cfg(feature = "Win32_ncrypt")]
windows_link::link!("secur32.dll" "system" fn SaslIdentifyPackageA(pinput : *const SecBufferDesc, packageinfo : *mut PSecPkgInfoA) -> super::ncrypt::SECURITY_STATUS);
#[cfg(feature = "Win32_ncrypt")]
windows_link::link!("secur32.dll" "system" fn SaslIdentifyPackageW(pinput : *const SecBufferDesc, packageinfo : *mut PSecPkgInfoW) -> super::ncrypt::SECURITY_STATUS);
#[cfg(feature = "Win32_ncrypt")]
windows_link::link!("secur32.dll" "system" fn SaslInitializeSecurityContextA(phcredential : PCredHandle, phcontext : PCtxtHandle, psztargetname : windows_sys::core::PCSTR, fcontextreq : u32, reserved1 : u32, targetdatarep : u32, pinput : *const SecBufferDesc, reserved2 : u32, phnewcontext : PCtxtHandle, poutput : *mut SecBufferDesc, pfcontextattr : *mut u32, ptsexpiry : *mut SECURITY_INTEGER) -> super::ncrypt::SECURITY_STATUS);
#[cfg(feature = "Win32_ncrypt")]
windows_link::link!("secur32.dll" "system" fn SaslInitializeSecurityContextW(phcredential : PCredHandle, phcontext : PCtxtHandle, psztargetname : windows_sys::core::PCWSTR, fcontextreq : u32, reserved1 : u32, targetdatarep : u32, pinput : *const SecBufferDesc, reserved2 : u32, phnewcontext : PCtxtHandle, poutput : *mut SecBufferDesc, pfcontextattr : *mut u32, ptsexpiry : *mut SECURITY_INTEGER) -> super::ncrypt::SECURITY_STATUS);
#[cfg(feature = "Win32_ncrypt")]
windows_link::link!("secur32.dll" "system" fn SaslSetContextOption(contexthandle : PCtxtHandle, option : u32, value : *const core::ffi::c_void, size : u32) -> super::ncrypt::SECURITY_STATUS);
#[cfg(feature = "Win32_ncrypt")]
windows_link::link!("sspicli.dll" "system" fn SecAllocateAndSetCallTarget(lpipaddress : *const u8, cchipaddress : u32, targetname : windows_sys::core::PCWSTR, freecallcontext : *mut i32) -> super::ncrypt::SECURITY_STATUS);
#[cfg(feature = "Win32_ncrypt")]
windows_link::link!("sspicli.dll" "system" fn SecAllocateAndSetIPAddress(lpipaddress : *const u8, cchipaddress : u32, freecallcontext : *mut i32) -> super::ncrypt::SECURITY_STATUS);
windows_link::link!("sspicli.dll" "system" fn SecFreeCallContext());
#[cfg(feature = "Win32_ncrypt")]
windows_link::link!("secur32.dll" "system" fn SetContextAttributesA(phcontext : PCtxtHandle, ulattribute : u32, pbuffer : *const core::ffi::c_void, cbbuffer : u32) -> super::ncrypt::SECURITY_STATUS);
#[cfg(feature = "Win32_ncrypt")]
windows_link::link!("secur32.dll" "system" fn SetContextAttributesW(phcontext : PCtxtHandle, ulattribute : u32, pbuffer : *const core::ffi::c_void, cbbuffer : u32) -> super::ncrypt::SECURITY_STATUS);
#[cfg(feature = "Win32_ncrypt")]
windows_link::link!("secur32.dll" "system" fn SetCredentialsAttributesA(phcredential : PCredHandle, ulattribute : u32, pbuffer : *const core::ffi::c_void, cbbuffer : u32) -> super::ncrypt::SECURITY_STATUS);
#[cfg(feature = "Win32_ncrypt")]
windows_link::link!("secur32.dll" "system" fn SetCredentialsAttributesW(phcredential : PCredHandle, ulattribute : u32, pbuffer : *const core::ffi::c_void, cbbuffer : u32) -> super::ncrypt::SECURITY_STATUS);
#[cfg(feature = "Win32_ncrypt")]
windows_link::link!("secur32.dll" "system" fn SspiCompareAuthIdentities(authidentity1 : PSEC_WINNT_AUTH_IDENTITY_OPAQUE, authidentity2 : PSEC_WINNT_AUTH_IDENTITY_OPAQUE, samesupplieduser : *mut bool, samesuppliedidentity : *mut bool) -> super::ncrypt::SECURITY_STATUS);
#[cfg(feature = "Win32_ncrypt")]
windows_link::link!("secur32.dll" "system" fn SspiCopyAuthIdentity(authdata : PSEC_WINNT_AUTH_IDENTITY_OPAQUE, authdatacopy : *mut PSEC_WINNT_AUTH_IDENTITY_OPAQUE) -> super::ncrypt::SECURITY_STATUS);
#[cfg(feature = "Win32_ncrypt")]
windows_link::link!("secur32.dll" "system" fn SspiDecryptAuthIdentity(encryptedauthdata : PSEC_WINNT_AUTH_IDENTITY_OPAQUE) -> super::ncrypt::SECURITY_STATUS);
#[cfg(feature = "Win32_ncrypt")]
windows_link::link!("sspicli.dll" "system" fn SspiDecryptAuthIdentityEx(options : u32, encryptedauthdata : PSEC_WINNT_AUTH_IDENTITY_OPAQUE) -> super::ncrypt::SECURITY_STATUS);
#[cfg(feature = "Win32_ncrypt")]
windows_link::link!("secur32.dll" "system" fn SspiEncodeAuthIdentityAsStrings(pauthidentity : PSEC_WINNT_AUTH_IDENTITY_OPAQUE, ppszusername : *mut windows_sys::core::PCWSTR, ppszdomainname : *mut windows_sys::core::PCWSTR, ppszpackedcredentialsstring : *mut windows_sys::core::PCWSTR) -> super::ncrypt::SECURITY_STATUS);
#[cfg(feature = "Win32_ncrypt")]
windows_link::link!("secur32.dll" "system" fn SspiEncodeStringsAsAuthIdentity(pszusername : windows_sys::core::PCWSTR, pszdomainname : windows_sys::core::PCWSTR, pszpackedcredentialsstring : windows_sys::core::PCWSTR, ppauthidentity : *mut PSEC_WINNT_AUTH_IDENTITY_OPAQUE) -> super::ncrypt::SECURITY_STATUS);
#[cfg(feature = "Win32_ncrypt")]
windows_link::link!("secur32.dll" "system" fn SspiEncryptAuthIdentity(authdata : PSEC_WINNT_AUTH_IDENTITY_OPAQUE) -> super::ncrypt::SECURITY_STATUS);
#[cfg(feature = "Win32_ncrypt")]
windows_link::link!("sspicli.dll" "system" fn SspiEncryptAuthIdentityEx(options : u32, authdata : PSEC_WINNT_AUTH_IDENTITY_OPAQUE) -> super::ncrypt::SECURITY_STATUS);
#[cfg(feature = "Win32_ncrypt")]
windows_link::link!("secur32.dll" "system" fn SspiExcludePackage(authidentity : PSEC_WINNT_AUTH_IDENTITY_OPAQUE, pszpackagename : windows_sys::core::PCWSTR, ppnewauthidentity : *mut PSEC_WINNT_AUTH_IDENTITY_OPAQUE) -> super::ncrypt::SECURITY_STATUS);
windows_link::link!("secur32.dll" "system" fn SspiFreeAuthIdentity(authdata : PSEC_WINNT_AUTH_IDENTITY_OPAQUE));
#[cfg(feature = "Win32_ncrypt")]
windows_link::link!("secur32.dll" "system" fn SspiGetTargetHostName(psztargetname : windows_sys::core::PCWSTR, pszhostname : *mut windows_sys::core::PWSTR) -> super::ncrypt::SECURITY_STATUS);
windows_link::link!("secur32.dll" "system" fn SspiIsAuthIdentityEncrypted(encryptedauthdata : PSEC_WINNT_AUTH_IDENTITY_OPAQUE) -> bool);
windows_link::link!("credui.dll" "system" fn SspiIsPromptingNeeded(errororntstatus : u32) -> bool);
windows_link::link!("secur32.dll" "system" fn SspiLocalFree(databuffer : *const core::ffi::c_void));
#[cfg(feature = "Win32_ncrypt")]
windows_link::link!("secur32.dll" "system" fn SspiMarshalAuthIdentity(authidentity : PSEC_WINNT_AUTH_IDENTITY_OPAQUE, authidentitylength : *mut u32, authidentitybytearray : *mut *mut i8) -> super::ncrypt::SECURITY_STATUS);
#[cfg(feature = "Win32_ncrypt")]
windows_link::link!("secur32.dll" "system" fn SspiPrepareForCredRead(authidentity : PSEC_WINNT_AUTH_IDENTITY_OPAQUE, psztargetname : windows_sys::core::PCWSTR, pcredmancredentialtype : *mut u32, ppszcredmantargetname : *mut windows_sys::core::PCWSTR) -> super::ncrypt::SECURITY_STATUS);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_ncrypt"))]
windows_link::link!("secur32.dll" "system" fn SspiPrepareForCredWrite(authidentity : PSEC_WINNT_AUTH_IDENTITY_OPAQUE, psztargetname : windows_sys::core::PCWSTR, pcredmancredentialtype : *mut u32, ppszcredmantargetname : *mut windows_sys::core::PCWSTR, ppszcredmanusername : *mut windows_sys::core::PCWSTR, ppcredentialblob : *mut super::minwindef::PUCHAR, pcredentialblobsize : *mut u32) -> super::ncrypt::SECURITY_STATUS);
windows_link::link!("credui.dll" "system" fn SspiPromptForCredentialsA(psztargetname : windows_sys::core::PCSTR, puiinfo : *const core::ffi::c_void, dwautherror : u32, pszpackage : windows_sys::core::PCSTR, pinputauthidentity : PSEC_WINNT_AUTH_IDENTITY_OPAQUE, ppauthidentity : *mut PSEC_WINNT_AUTH_IDENTITY_OPAQUE, pfsave : *mut i32, dwflags : u32) -> u32);
windows_link::link!("credui.dll" "system" fn SspiPromptForCredentialsW(psztargetname : windows_sys::core::PCWSTR, puiinfo : *const core::ffi::c_void, dwautherror : u32, pszpackage : windows_sys::core::PCWSTR, pinputauthidentity : PSEC_WINNT_AUTH_IDENTITY_OPAQUE, ppauthidentity : *mut PSEC_WINNT_AUTH_IDENTITY_OPAQUE, pfsave : *mut i32, dwflags : u32) -> u32);
#[cfg(feature = "Win32_ncrypt")]
windows_link::link!("sspicli.dll" "system" fn SspiSetChannelBindingFlags(pbindings : *mut SecPkgContext_Bindings, flags : u32) -> super::ncrypt::SECURITY_STATUS);
#[cfg(feature = "Win32_ncrypt")]
windows_link::link!("secur32.dll" "system" fn SspiUnmarshalAuthIdentity(authidentitylength : u32, authidentitybytearray : *const i8, ppauthidentity : *mut PSEC_WINNT_AUTH_IDENTITY_OPAQUE) -> super::ncrypt::SECURITY_STATUS);
#[cfg(feature = "Win32_ncrypt")]
windows_link::link!("secur32.dll" "system" fn SspiValidateAuthIdentity(authdata : PSEC_WINNT_AUTH_IDENTITY_OPAQUE) -> super::ncrypt::SECURITY_STATUS);
windows_link::link!("secur32.dll" "system" fn SspiZeroAuthIdentity(authdata : PSEC_WINNT_AUTH_IDENTITY_OPAQUE));
#[cfg(feature = "Win32_ncrypt")]
windows_link::link!("secur32.dll" "system" fn VerifySignature(phcontext : PCtxtHandle, pmessage : *const SecBufferDesc, messageseqno : u32, pfqop : *mut u32) -> super::ncrypt::SECURITY_STATUS);
#[cfg(feature = "Win32_ncrypt")]
pub type ACCEPT_SECURITY_CONTEXT_FN = Option<unsafe extern "system" fn(param0: PCredHandle, param1: PCtxtHandle, param2: *mut SecBufferDesc, param3: u32, param4: u32, param5: PCtxtHandle, param6: *mut SecBufferDesc, param7: *mut u32, param8: *mut SECURITY_INTEGER) -> super::ncrypt::SECURITY_STATUS>;
#[cfg(feature = "Win32_ncrypt")]
pub type ACQUIRE_CREDENTIALS_HANDLE_FN_A = Option<unsafe extern "system" fn(param0: *mut SEC_CHAR, param1: *mut SEC_CHAR, param2: u32, param3: *mut core::ffi::c_void, param4: *mut core::ffi::c_void, param5: SEC_GET_KEY_FN, param6: *mut core::ffi::c_void, param7: PCredHandle, param8: *mut SECURITY_INTEGER) -> super::ncrypt::SECURITY_STATUS>;
#[cfg(feature = "Win32_ncrypt")]
pub type ACQUIRE_CREDENTIALS_HANDLE_FN_W = Option<unsafe extern "system" fn(param0: *mut SEC_WCHAR, param1: *mut SEC_WCHAR, param2: u32, param3: *mut core::ffi::c_void, param4: *mut core::ffi::c_void, param5: SEC_GET_KEY_FN, param6: *mut core::ffi::c_void, param7: PCredHandle, param8: *mut SECURITY_INTEGER) -> super::ncrypt::SECURITY_STATUS>;
#[cfg(feature = "Win32_ncrypt")]
pub type ADD_CREDENTIALS_FN_A = Option<unsafe extern "system" fn(param0: PCredHandle, param1: *mut SEC_CHAR, param2: *mut SEC_CHAR, param3: u32, param4: *mut core::ffi::c_void, param5: SEC_GET_KEY_FN, param6: *mut core::ffi::c_void, param7: *mut SECURITY_INTEGER) -> super::ncrypt::SECURITY_STATUS>;
#[cfg(feature = "Win32_ncrypt")]
pub type ADD_CREDENTIALS_FN_W = Option<unsafe extern "system" fn(param0: PCredHandle, param1: *mut SEC_WCHAR, param2: *mut SEC_WCHAR, param3: u32, param4: *mut core::ffi::c_void, param5: SEC_GET_KEY_FN, param6: *mut core::ffi::c_void, param7: *mut SECURITY_INTEGER) -> super::ncrypt::SECURITY_STATUS>;
#[cfg(feature = "Win32_ncrypt")]
pub type APPLY_CONTROL_TOKEN_FN = Option<unsafe extern "system" fn(param0: PCtxtHandle, param1: *mut SecBufferDesc) -> super::ncrypt::SECURITY_STATUS>;
pub const ASC_REQ_ALLOCATE_MEMORY: u32 = 256;
pub const ASC_REQ_ALLOW_CONTEXT_REPLAY: u32 = 4194304;
pub const ASC_REQ_ALLOW_MISSING_BINDINGS: u32 = 268435456;
pub const ASC_REQ_ALLOW_NON_USER_LOGONS: u32 = 2097152;
pub const ASC_REQ_ALLOW_NULL_SESSION: u32 = 1048576;
pub const ASC_REQ_CALL_LEVEL: u32 = 4096;
pub const ASC_REQ_CONFIDENTIALITY: u32 = 16;
pub const ASC_REQ_CONNECTION: u32 = 2048;
pub const ASC_REQ_DATAGRAM: u32 = 1024;
pub const ASC_REQ_DELEGATE: u32 = 1;
pub const ASC_REQ_EXPLICIT_SESSION: u64 = 68719476736;
pub const ASC_REQ_EXTENDED_ERROR: u32 = 32768;
pub const ASC_REQ_FRAGMENT_SUPPLIED: u32 = 8192;
pub const ASC_REQ_FRAGMENT_TO_FIT: u32 = 8388608;
pub const ASC_REQ_IDENTIFY: u32 = 524288;
pub const ASC_REQ_INTEGRITY: u32 = 131072;
pub const ASC_REQ_LICENSING: u32 = 262144;
pub const ASC_REQ_MESSAGES: u64 = 4294967296;
pub const ASC_REQ_MUTUAL_AUTH: u32 = 2;
pub const ASC_REQ_NO_TOKEN: u32 = 16777216;
pub const ASC_REQ_PROXY_BINDINGS: u32 = 67108864;
pub const ASC_REQ_REPLAY_DETECT: u32 = 4;
pub const ASC_REQ_SEQUENCE_DETECT: u32 = 8;
pub const ASC_REQ_SESSION_TICKET: u32 = 64;
pub const ASC_REQ_STREAM: u32 = 65536;
pub const ASC_REQ_USE_DCE_STYLE: u32 = 512;
pub const ASC_REQ_USE_SESSION_KEY: u32 = 32;
pub const ASC_RET_ALLOCATED_MEMORY: u32 = 256;
pub const ASC_RET_ALLOW_CONTEXT_REPLAY: u32 = 4194304;
pub const ASC_RET_ALLOW_NON_USER_LOGONS: u32 = 2097152;
pub const ASC_RET_CALL_LEVEL: u32 = 8192;
pub const ASC_RET_CONFIDENTIALITY: u32 = 16;
pub const ASC_RET_CONNECTION: u32 = 2048;
pub const ASC_RET_DATAGRAM: u32 = 1024;
pub const ASC_RET_DELEGATE: u32 = 1;
pub const ASC_RET_EXPLICIT_SESSION: u64 = 68719476736;
pub const ASC_RET_EXTENDED_ERROR: u32 = 32768;
pub const ASC_RET_FRAGMENT_ONLY: u32 = 8388608;
pub const ASC_RET_IDENTIFY: u32 = 524288;
pub const ASC_RET_INTEGRITY: u32 = 131072;
pub const ASC_RET_LICENSING: u32 = 262144;
pub const ASC_RET_MESSAGES: u64 = 4294967296;
pub const ASC_RET_MUTUAL_AUTH: u32 = 2;
pub const ASC_RET_NO_ADDITIONAL_TOKEN: u32 = 33554432;
pub const ASC_RET_NO_TOKEN: u32 = 16777216;
pub const ASC_RET_NULL_SESSION: u32 = 1048576;
pub const ASC_RET_REPLAY_DETECT: u32 = 4;
pub const ASC_RET_REUSE_SESSION_TICKETS: u64 = 34359738368;
pub const ASC_RET_SEQUENCE_DETECT: u32 = 8;
pub const ASC_RET_SESSION_TICKET: u32 = 64;
pub const ASC_RET_STREAM: u32 = 65536;
pub const ASC_RET_THIRD_LEG_FAILED: u32 = 16384;
pub const ASC_RET_USED_DCE_STYLE: u32 = 512;
pub const ASC_RET_USE_SESSION_KEY: u32 = 32;
#[cfg(feature = "Win32_ncrypt")]
pub type CHANGE_PASSWORD_FN_A = Option<unsafe extern "system" fn(param0: *mut SEC_CHAR, param1: *mut SEC_CHAR, param2: *mut SEC_CHAR, param3: *mut SEC_CHAR, param4: *mut SEC_CHAR, param5: bool, param6: u32, param7: *mut SecBufferDesc) -> super::ncrypt::SECURITY_STATUS>;
#[cfg(feature = "Win32_ncrypt")]
pub type CHANGE_PASSWORD_FN_W = Option<unsafe extern "system" fn(param0: *mut SEC_WCHAR, param1: *mut SEC_WCHAR, param2: *mut SEC_WCHAR, param3: *mut SEC_WCHAR, param4: *mut SEC_WCHAR, param5: bool, param6: u32, param7: *mut SecBufferDesc) -> super::ncrypt::SECURITY_STATUS>;
#[cfg(feature = "Win32_ncrypt")]
pub type COMPLETE_AUTH_TOKEN_FN = Option<unsafe extern "system" fn(param0: PCtxtHandle, param1: *mut SecBufferDesc) -> super::ncrypt::SECURITY_STATUS>;
pub type CredHandle = SecHandle;
pub type CtxtHandle = SecHandle;
#[cfg(feature = "Win32_ncrypt")]
pub type DECRYPT_MESSAGE_FN = Option<unsafe extern "system" fn(param0: PCtxtHandle, param1: *mut SecBufferDesc, param2: u32, param3: *mut u32) -> super::ncrypt::SECURITY_STATUS>;
#[cfg(feature = "Win32_ncrypt")]
pub type DELETE_SECURITY_CONTEXT_FN = Option<unsafe extern "system" fn(param0: PCtxtHandle) -> super::ncrypt::SECURITY_STATUS>;
#[cfg(feature = "Win32_ncrypt")]
pub type ENCRYPT_MESSAGE_FN = Option<unsafe extern "system" fn(param0: PCtxtHandle, param1: u32, param2: *mut SecBufferDesc, param3: u32) -> super::ncrypt::SECURITY_STATUS>;
#[cfg(feature = "Win32_ncrypt")]
pub type ENUMERATE_SECURITY_PACKAGES_FN_A = Option<unsafe extern "system" fn(param0: *mut u32, param1: *mut PSecPkgInfoA) -> super::ncrypt::SECURITY_STATUS>;
#[cfg(feature = "Win32_ncrypt")]
pub type ENUMERATE_SECURITY_PACKAGES_FN_W = Option<unsafe extern "system" fn(param0: *mut u32, param1: *mut PSecPkgInfoW) -> super::ncrypt::SECURITY_STATUS>;
#[cfg(feature = "Win32_ncrypt")]
pub type EXPORT_SECURITY_CONTEXT_FN = Option<unsafe extern "system" fn(param0: PCtxtHandle, param1: u32, param2: *mut SecBuffer, param3: *mut *mut core::ffi::c_void) -> super::ncrypt::SECURITY_STATUS>;
#[cfg(feature = "Win32_ncrypt")]
pub type FREE_CONTEXT_BUFFER_FN = Option<unsafe extern "system" fn(param0: *mut core::ffi::c_void) -> super::ncrypt::SECURITY_STATUS>;
#[cfg(feature = "Win32_ncrypt")]
pub type FREE_CREDENTIALS_HANDLE_FN = Option<unsafe extern "system" fn(param0: PCredHandle) -> super::ncrypt::SECURITY_STATUS>;
#[cfg(feature = "Win32_ncrypt")]
pub type IMPERSONATE_SECURITY_CONTEXT_FN = Option<unsafe extern "system" fn(param0: PCtxtHandle) -> super::ncrypt::SECURITY_STATUS>;
#[cfg(feature = "Win32_ncrypt")]
pub type IMPORT_SECURITY_CONTEXT_FN_A = Option<unsafe extern "system" fn(param0: *mut SEC_CHAR, param1: *mut SecBuffer, param2: *mut core::ffi::c_void, param3: PCtxtHandle) -> super::ncrypt::SECURITY_STATUS>;
#[cfg(feature = "Win32_ncrypt")]
pub type IMPORT_SECURITY_CONTEXT_FN_W = Option<unsafe extern "system" fn(param0: *mut SEC_WCHAR, param1: *mut SecBuffer, param2: *mut core::ffi::c_void, param3: PCtxtHandle) -> super::ncrypt::SECURITY_STATUS>;
#[cfg(feature = "Win32_ncrypt")]
pub type INITIALIZE_SECURITY_CONTEXT_FN_A = Option<unsafe extern "system" fn(param0: PCredHandle, param1: PCtxtHandle, param2: *mut SEC_CHAR, param3: u32, param4: u32, param5: u32, param6: *mut SecBufferDesc, param7: u32, param8: PCtxtHandle, param9: *mut SecBufferDesc, param10: *mut u32, param11: *mut SECURITY_INTEGER) -> super::ncrypt::SECURITY_STATUS>;
#[cfg(feature = "Win32_ncrypt")]
pub type INITIALIZE_SECURITY_CONTEXT_FN_W = Option<unsafe extern "system" fn(param0: PCredHandle, param1: PCtxtHandle, param2: *mut SEC_WCHAR, param3: u32, param4: u32, param5: u32, param6: *mut SecBufferDesc, param7: u32, param8: PCtxtHandle, param9: *mut SecBufferDesc, param10: *mut u32, param11: *mut SECURITY_INTEGER) -> super::ncrypt::SECURITY_STATUS>;
#[cfg(feature = "Win32_ncrypt")]
pub type INIT_SECURITY_INTERFACE_A = Option<unsafe extern "system" fn() -> PSecurityFunctionTableA>;
#[cfg(feature = "Win32_ncrypt")]
pub type INIT_SECURITY_INTERFACE_W = Option<unsafe extern "system" fn() -> PSecurityFunctionTableW>;
pub const ISC_REQ_ALLOCATE_MEMORY: u32 = 256;
pub const ISC_REQ_CALL_LEVEL: u32 = 4096;
pub const ISC_REQ_CONFIDENTIALITY: u32 = 16;
pub const ISC_REQ_CONFIDENTIALITY_ONLY: u32 = 1073741824;
pub const ISC_REQ_CONNECTION: u32 = 2048;
pub const ISC_REQ_DATAGRAM: u32 = 1024;
pub const ISC_REQ_DEFERRED_CRED_VALIDATION: u64 = 8589934592;
pub const ISC_REQ_DELEGATE: u32 = 1;
pub const ISC_REQ_EXPLICIT_SESSION: u64 = 68719476736;
pub const ISC_REQ_EXTENDED_ERROR: u32 = 16384;
pub const ISC_REQ_FORWARD_CREDENTIALS: u32 = 4194304;
pub const ISC_REQ_FRAGMENT_SUPPLIED: u32 = 8192;
pub const ISC_REQ_FRAGMENT_TO_FIT: u32 = 2097152;
pub const ISC_REQ_IDENTIFY: u32 = 131072;
pub const ISC_REQ_INTEGRITY: u32 = 65536;
pub const ISC_REQ_MANUAL_CRED_VALIDATION: u32 = 524288;
pub const ISC_REQ_MESSAGES: u64 = 4294967296;
pub const ISC_REQ_MUTUAL_AUTH: u32 = 2;
pub const ISC_REQ_NO_INTEGRITY: u32 = 8388608;
pub const ISC_REQ_NO_POST_HANDSHAKE_AUTH: u64 = 17179869184;
pub const ISC_REQ_NULL_SESSION: u32 = 262144;
pub const ISC_REQ_PROMPT_FOR_CREDS: u32 = 64;
pub const ISC_REQ_REPLAY_DETECT: u32 = 4;
pub const ISC_REQ_RESERVED1: u32 = 1048576;
pub const ISC_REQ_REUSE_SESSION_TICKETS: u64 = 34359738368;
pub const ISC_REQ_SEQUENCE_DETECT: u32 = 8;
pub const ISC_REQ_STREAM: u32 = 32768;
pub const ISC_REQ_UNVERIFIED_TARGET_NAME: u32 = 536870912;
pub const ISC_REQ_USE_DCE_STYLE: u32 = 512;
pub const ISC_REQ_USE_HTTP_STYLE: u32 = 16777216;
pub const ISC_REQ_USE_SESSION_KEY: u32 = 32;
pub const ISC_REQ_USE_SUPPLIED_CREDS: u32 = 128;
pub const ISC_RET_ALLOCATED_MEMORY: u32 = 256;
pub const ISC_RET_CALL_LEVEL: u32 = 8192;
pub const ISC_RET_CONFIDENTIALITY: u32 = 16;
pub const ISC_RET_CONFIDENTIALITY_ONLY: u32 = 1073741824;
pub const ISC_RET_CONNECTION: u32 = 2048;
pub const ISC_RET_DATAGRAM: u32 = 1024;
pub const ISC_RET_DEFERRED_CRED_VALIDATION: u64 = 8589934592;
pub const ISC_RET_DELEGATE: u32 = 1;
pub const ISC_RET_EXPLICIT_SESSION: u64 = 68719476736;
pub const ISC_RET_EXTENDED_ERROR: u32 = 16384;
pub const ISC_RET_FORWARD_CREDENTIALS: u32 = 4194304;
pub const ISC_RET_FRAGMENT_ONLY: u32 = 2097152;
pub const ISC_RET_IDENTIFY: u32 = 131072;
pub const ISC_RET_INTEGRITY: u32 = 65536;
pub const ISC_RET_INTERMEDIATE_RETURN: u32 = 4096;
pub const ISC_RET_MANUAL_CRED_VALIDATION: u32 = 524288;
pub const ISC_RET_MESSAGES: u64 = 4294967296;
pub const ISC_RET_MUTUAL_AUTH: u32 = 2;
pub const ISC_RET_NO_ADDITIONAL_TOKEN: u32 = 33554432;
pub const ISC_RET_NO_POST_HANDSHAKE_AUTH: u64 = 17179869184;
pub const ISC_RET_NULL_SESSION: u32 = 262144;
pub const ISC_RET_REAUTHENTICATION: u32 = 134217728;
pub const ISC_RET_REPLAY_DETECT: u32 = 4;
pub const ISC_RET_RESERVED1: u32 = 1048576;
pub const ISC_RET_REUSE_SESSION_TICKETS: u64 = 34359738368;
pub const ISC_RET_SEQUENCE_DETECT: u32 = 8;
pub const ISC_RET_STREAM: u32 = 32768;
pub const ISC_RET_USED_COLLECTED_CREDS: u32 = 64;
pub const ISC_RET_USED_DCE_STYLE: u32 = 512;
pub const ISC_RET_USED_HTTP_STYLE: u32 = 16777216;
pub const ISC_RET_USED_SUPPLIED_CREDS: u32 = 128;
pub const ISC_RET_USE_SESSION_KEY: u32 = 32;
pub const ISSP_LEVEL: u32 = 32;
pub const ISSP_MODE: u32 = 1;
pub const KDC_NETWORK_DISCOVERY_FLAGS_DS13_REQUIRED: u32 = 2147483648;
pub const KDC_NETWORK_SETTINGS_FLAGS_CONFIGURE_DISCOVERY: u32 = 1073741824;
pub const KDC_NETWORK_SETTINGS_FLAGS_CONFIGURE_PROXY: u32 = 2147483648;
pub const KDC_NETWORK_SETTINGS_FLAGS_FORCEPROXY: u32 = 1;
pub const KDC_NETWORK_SETTINGS_V2: u32 = 2;
pub const KDC_PROXY_SETTINGS_FLAGS_FORCEPROXY: u32 = 1;
pub const KDC_PROXY_SETTINGS_V1: u32 = 1;
#[cfg(feature = "Win32_ncrypt")]
pub type MAKE_SIGNATURE_FN = Option<unsafe extern "system" fn(param0: PCtxtHandle, param1: u32, param2: *mut SecBufferDesc, param3: u32) -> super::ncrypt::SECURITY_STATUS>;
pub const MAX_PROTOCOL_ID_SIZE: u32 = 255;
pub type PCredHandle = PSecHandle;
pub type PCtxtHandle = PSecHandle;
pub type PSECPKG_APP_MODE_INFO = *mut SECPKG_APP_MODE_INFO;
pub type PSECPKG_ATTR_LCT_STATUS = *mut SECPKG_ATTR_LCT_STATUS;
pub type PSECPKG_CRED_CLASS = *mut SECPKG_CRED_CLASS;
pub type PSECURITY_INTEGER = *mut i64;
pub type PSECURITY_PACKAGE_OPTIONS = *mut SECURITY_PACKAGE_OPTIONS;
pub type PSECURITY_STRING = *mut SECURITY_STRING;
pub type PSEC_APPLICATION_PROTOCOLS = *mut SEC_APPLICATION_PROTOCOLS;
pub type PSEC_APPLICATION_PROTOCOL_LIST = *mut SEC_APPLICATION_PROTOCOL_LIST;
pub type PSEC_APPLICATION_PROTOCOL_NEGOTIATION_EXT = *mut SEC_APPLICATION_PROTOCOL_NEGOTIATION_EXT;
pub type PSEC_APPLICATION_PROTOCOL_NEGOTIATION_STATUS = *mut SEC_APPLICATION_PROTOCOL_NEGOTIATION_STATUS;
pub type PSEC_APP_SESSION_STATE = *mut SEC_APP_SESSION_STATE;
pub type PSEC_CERTIFICATE_REQUEST_CONTEXT = *mut SEC_CERTIFICATE_REQUEST_CONTEXT;
pub type PSEC_CHANNEL_BINDINGS = *mut SEC_CHANNEL_BINDINGS;
pub type PSEC_CHANNEL_BINDINGS_EX = *mut SEC_CHANNEL_BINDINGS_EX;
pub type PSEC_CHANNEL_BINDINGS_RESULT = *mut SEC_CHANNEL_BINDINGS_RESULT;
pub type PSEC_DTLS_MTU = *mut SEC_DTLS_MTU;
pub type PSEC_FLAGS = *mut SEC_FLAGS;
pub type PSEC_NEGOTIATION_INFO = *mut SEC_NEGOTIATION_INFO;
pub type PSEC_PRESHAREDKEY = *mut SEC_PRESHAREDKEY;
pub type PSEC_PRESHAREDKEY_IDENTITY = *mut SEC_PRESHAREDKEY_IDENTITY;
pub type PSEC_SESSION_TICKET = *mut SEC_SESSION_TICKET;
pub type PSEC_SRTP_MASTER_KEY_IDENTIFIER = *mut SEC_SRTP_MASTER_KEY_IDENTIFIER;
pub type PSEC_SRTP_PROTECTION_PROFILES = *mut SEC_SRTP_PROTECTION_PROFILES;
pub type PSEC_TOKEN_BINDING = *mut SEC_TOKEN_BINDING;
pub type PSEC_TRAFFIC_SECRETS = *mut SEC_TRAFFIC_SECRETS;
pub type PSEC_TRAFFIC_SECRET_TYPE = *mut SEC_TRAFFIC_SECRET_TYPE;
pub type PSEC_WINNT_AUTH_IDENTITY_EX2 = *mut SEC_WINNT_AUTH_IDENTITY_EX2;
pub type PSEC_WINNT_AUTH_IDENTITY_EXA = *mut SEC_WINNT_AUTH_IDENTITY_EXA;
pub type PSEC_WINNT_AUTH_IDENTITY_EXW = *mut SEC_WINNT_AUTH_IDENTITY_EXW;
#[cfg(feature = "Win32_rpc")]
pub type PSEC_WINNT_AUTH_IDENTITY_INFO = *mut SEC_WINNT_AUTH_IDENTITY_INFO;
pub type PSEC_WINNT_AUTH_IDENTITY_OPAQUE = *mut core::ffi::c_void;
pub type PSecBuffer = *mut SecBuffer;
pub type PSecBufferDesc = *mut SecBufferDesc;
pub type PSecDelegationType = *mut SecDelegationType;
pub type PSecHandle = *mut SecHandle;
pub type PSecPkgContext_AccessToken = *mut SecPkgContext_AccessToken;
pub type PSecPkgContext_ApplicationProtocol = *mut SecPkgContext_ApplicationProtocol;
pub type PSecPkgContext_AuthorityA = *mut SecPkgContext_AuthorityA;
pub type PSecPkgContext_AuthorityW = *mut SecPkgContext_AuthorityW;
pub type PSecPkgContext_AuthzID = *mut SecPkgContext_AuthzID;
pub type PSecPkgContext_Bindings = *mut SecPkgContext_Bindings;
pub type PSecPkgContext_ClientSpecifiedTarget = *mut SecPkgContext_ClientSpecifiedTarget;
pub type PSecPkgContext_CredInfo = *mut SecPkgContext_CredInfo;
pub type PSecPkgContext_CredentialNameA = *mut SecPkgContext_CredentialNameA;
pub type PSecPkgContext_CredentialNameW = *mut SecPkgContext_CredentialNameW;
pub type PSecPkgContext_DatagramSizes = PSecPkgContext_StreamSizes;
pub type PSecPkgContext_DceInfo = *mut SecPkgContext_DceInfo;
pub type PSecPkgContext_Flags = *mut SecPkgContext_Flags;
pub type PSecPkgContext_KeyInfoA = *mut SecPkgContext_KeyInfoA;
pub type PSecPkgContext_KeyInfoW = *mut SecPkgContext_KeyInfoW;
pub type PSecPkgContext_LastClientTokenStatus = *mut SecPkgContext_LastClientTokenStatus;
pub type PSecPkgContext_Lifespan = *mut SecPkgContext_Lifespan;
pub type PSecPkgContext_LogoffTime = *mut SecPkgContext_LogoffTime;
pub type PSecPkgContext_NamesA = *mut SecPkgContext_NamesA;
pub type PSecPkgContext_NamesW = *mut SecPkgContext_NamesW;
pub type PSecPkgContext_NativeNamesA = *mut SecPkgContext_NativeNamesA;
pub type PSecPkgContext_NativeNamesW = *mut SecPkgContext_NativeNamesW;
pub type PSecPkgContext_NegoKeys = *mut SecPkgContext_NegoKeys;
pub type PSecPkgContext_NegoPackageInfo = *mut SecPkgContext_NegoPackageInfo;
pub type PSecPkgContext_NegoStatus = *mut SecPkgContext_NegoStatus;
pub type PSecPkgContext_NegotiatedTlsExtensions = *mut SecPkgContext_NegotiatedTlsExtensions;
pub type PSecPkgContext_NegotiationInfoA = *mut SecPkgContext_NegotiationInfoA;
pub type PSecPkgContext_NegotiationInfoW = *mut SecPkgContext_NegotiationInfoW;
pub type PSecPkgContext_PackageInfoA = *mut SecPkgContext_PackageInfoA;
pub type PSecPkgContext_PackageInfoW = *mut SecPkgContext_PackageInfoW;
pub type PSecPkgContext_PasswordExpiry = *mut SecPkgContext_PasswordExpiry;
pub type PSecPkgContext_ProtoInfoA = *mut SecPkgContext_ProtoInfoA;
pub type PSecPkgContext_ProtoInfoW = *mut SecPkgContext_ProtoInfoW;
pub type PSecPkgContext_SessionKey = *mut SecPkgContext_SessionKey;
pub type PSecPkgContext_Sizes = *mut SecPkgContext_Sizes;
pub type PSecPkgContext_StreamSizes = *mut SecPkgContext_StreamSizes;
pub type PSecPkgContext_SubjectAttributes = *mut SecPkgContext_SubjectAttributes;
pub type PSecPkgContext_Target = *mut SecPkgContext_Target;
pub type PSecPkgContext_TargetInformation = *mut SecPkgContext_TargetInformation;
pub type PSecPkgContext_UserFlags = *mut SecPkgContext_UserFlags;
pub type PSecPkgCredentials_Cert = *mut SecPkgCredentials_Cert;
pub type PSecPkgCredentials_KdcNetworkSettingsW = *mut SecPkgCredentials_KdcNetworkSettingsW;
pub type PSecPkgCredentials_KdcProxySettingsW = *mut SecPkgCredentials_KdcProxySettingsW;
pub type PSecPkgCredentials_NamesA = *mut SecPkgCredentials_NamesA;
pub type PSecPkgCredentials_NamesW = *mut SecPkgCredentials_NamesW;
pub type PSecPkgCredentials_SSIProviderA = *mut SecPkgCredentials_SSIProviderA;
pub type PSecPkgCredentials_SSIProviderW = *mut SecPkgCredentials_SSIProviderW;
pub type PSecPkgInfoA = *mut SecPkgInfoA;
pub type PSecPkgInfoW = *mut SecPkgInfoW;
#[cfg(feature = "Win32_ncrypt")]
pub type PSecurityFunctionTableA = *mut SecurityFunctionTableA;
#[cfg(feature = "Win32_ncrypt")]
pub type PSecurityFunctionTableW = *mut SecurityFunctionTableW;
pub type PTimeStamp = *mut SECURITY_INTEGER;
#[cfg(feature = "Win32_ncrypt")]
pub type QUERY_CONTEXT_ATTRIBUTES_EX_FN_A = Option<unsafe extern "system" fn(param0: PCtxtHandle, param1: u32, param2: *mut core::ffi::c_void, param3: u32) -> super::ncrypt::SECURITY_STATUS>;
#[cfg(feature = "Win32_ncrypt")]
pub type QUERY_CONTEXT_ATTRIBUTES_EX_FN_W = Option<unsafe extern "system" fn(param0: PCtxtHandle, param1: u32, param2: *mut core::ffi::c_void, param3: u32) -> super::ncrypt::SECURITY_STATUS>;
#[cfg(feature = "Win32_ncrypt")]
pub type QUERY_CONTEXT_ATTRIBUTES_FN_A = Option<unsafe extern "system" fn(param0: PCtxtHandle, param1: u32, param2: *mut core::ffi::c_void) -> super::ncrypt::SECURITY_STATUS>;
#[cfg(feature = "Win32_ncrypt")]
pub type QUERY_CONTEXT_ATTRIBUTES_FN_W = Option<unsafe extern "system" fn(param0: PCtxtHandle, param1: u32, param2: *mut core::ffi::c_void) -> super::ncrypt::SECURITY_STATUS>;
#[cfg(feature = "Win32_ncrypt")]
pub type QUERY_CREDENTIALS_ATTRIBUTES_EX_FN_A = Option<unsafe extern "system" fn(param0: PCredHandle, param1: u32, param2: *mut core::ffi::c_void, param3: u32) -> super::ncrypt::SECURITY_STATUS>;
#[cfg(feature = "Win32_ncrypt")]
pub type QUERY_CREDENTIALS_ATTRIBUTES_EX_FN_W = Option<unsafe extern "system" fn(param0: PCredHandle, param1: u32, param2: *mut core::ffi::c_void, param3: u32) -> super::ncrypt::SECURITY_STATUS>;
#[cfg(feature = "Win32_ncrypt")]
pub type QUERY_CREDENTIALS_ATTRIBUTES_FN_A = Option<unsafe extern "system" fn(param0: PCredHandle, param1: u32, param2: *mut core::ffi::c_void) -> super::ncrypt::SECURITY_STATUS>;
#[cfg(feature = "Win32_ncrypt")]
pub type QUERY_CREDENTIALS_ATTRIBUTES_FN_W = Option<unsafe extern "system" fn(param0: PCredHandle, param1: u32, param2: *mut core::ffi::c_void) -> super::ncrypt::SECURITY_STATUS>;
#[cfg(feature = "Win32_ncrypt")]
pub type QUERY_SECURITY_CONTEXT_TOKEN_FN = Option<unsafe extern "system" fn(param0: PCtxtHandle, param1: *mut *mut core::ffi::c_void) -> super::ncrypt::SECURITY_STATUS>;
#[cfg(feature = "Win32_ncrypt")]
pub type QUERY_SECURITY_PACKAGE_INFO_FN_A = Option<unsafe extern "system" fn(param0: *mut SEC_CHAR, param1: *mut PSecPkgInfoA) -> super::ncrypt::SECURITY_STATUS>;
#[cfg(feature = "Win32_ncrypt")]
pub type QUERY_SECURITY_PACKAGE_INFO_FN_W = Option<unsafe extern "system" fn(param0: *mut SEC_WCHAR, param1: *mut PSecPkgInfoW) -> super::ncrypt::SECURITY_STATUS>;
#[cfg(feature = "Win32_ncrypt")]
pub type REVERT_SECURITY_CONTEXT_FN = Option<unsafe extern "system" fn(param0: PCtxtHandle) -> super::ncrypt::SECURITY_STATUS>;
pub type SASL_AUTHZID_STATE = i32;
pub const SASL_OPTION_AUTHZ_PROCESSING: u32 = 4;
pub const SASL_OPTION_AUTHZ_STRING: u32 = 3;
pub const SASL_OPTION_RECV_SIZE: u32 = 2;
pub const SASL_OPTION_SEND_SIZE: u32 = 1;
pub const SECBUFFER_ALERT: u32 = 17;
pub const SECBUFFER_APPLICATION_PROTOCOLS: u32 = 18;
pub const SECBUFFER_APP_SESSION_STATE: u32 = 31;
pub const SECBUFFER_ATTRMASK: u32 = 4026531840;
pub const SECBUFFER_CERTIFICATE_REQUEST_CONTEXT: u32 = 29;
pub const SECBUFFER_CHANGE_PASS_RESPONSE: u32 = 15;
pub const SECBUFFER_CHANNEL_BINDINGS: u32 = 14;
pub const SECBUFFER_CHANNEL_BINDINGS_RESULT: u32 = 30;
pub const SECBUFFER_DATA: u32 = 1;
pub const SECBUFFER_DTLS_MTU: u32 = 24;
pub const SECBUFFER_EMPTY: u32 = 0;
pub const SECBUFFER_EXTRA: u32 = 5;
pub const SECBUFFER_FLAGS: u32 = 27;
pub const SECBUFFER_MECHLIST: u32 = 11;
pub const SECBUFFER_MECHLIST_SIGNATURE: u32 = 12;
pub const SECBUFFER_MISSING: u32 = 4;
pub const SECBUFFER_NEGOTIATION_INFO: u32 = 8;
pub const SECBUFFER_PADDING: u32 = 9;
pub const SECBUFFER_PKG_PARAMS: u32 = 3;
pub const SECBUFFER_PRESHARED_KEY: u32 = 22;
pub const SECBUFFER_PRESHARED_KEY_IDENTITY: u32 = 23;
pub const SECBUFFER_READONLY: u32 = 2147483648;
pub const SECBUFFER_READONLY_WITH_CHECKSUM: u32 = 268435456;
pub const SECBUFFER_RESERVED: u32 = 1610612736;
pub const SECBUFFER_SEND_GENERIC_TLS_EXTENSION: u32 = 25;
pub const SECBUFFER_SESSION_TICKET: u32 = 32;
pub const SECBUFFER_SRTP_MASTER_KEY_IDENTIFIER: u32 = 20;
pub const SECBUFFER_SRTP_PROTECTION_PROFILES: u32 = 19;
pub const SECBUFFER_STREAM: u32 = 10;
pub const SECBUFFER_STREAM_HEADER: u32 = 7;
pub const SECBUFFER_STREAM_TRAILER: u32 = 6;
pub const SECBUFFER_SUBSCRIBE_GENERIC_TLS_EXTENSION: u32 = 26;
pub const SECBUFFER_TARGET: u32 = 13;
pub const SECBUFFER_TARGET_HOST: u32 = 16;
pub const SECBUFFER_TOKEN: u32 = 2;
pub const SECBUFFER_TOKEN_BINDING: u32 = 21;
pub const SECBUFFER_TRAFFIC_SECRETS: u32 = 28;
pub const SECBUFFER_VERSION: u32 = 0;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SECPKG_APP_MODE_INFO {
    pub UserFunction: u32,
    pub Argument1: usize,
    pub Argument2: usize,
    pub UserData: SecBuffer,
    pub ReturnToLsa: bool,
}
pub const SECPKG_ATTR_ACCESS_TOKEN: u32 = 18;
pub const SECPKG_ATTR_APPLICATION_PROTOCOL: u32 = 35;
pub const SECPKG_ATTR_AUTHENTICATION_ID: u32 = 20;
pub const SECPKG_ATTR_AUTHORITY: u32 = 6;
pub const SECPKG_ATTR_CLIENT_SPECIFIED_TARGET: u32 = 27;
pub const SECPKG_ATTR_CONTEXT_DELETED: u32 = 33;
pub const SECPKG_ATTR_CREDENTIAL_NAME: u32 = 16;
pub const SECPKG_ATTR_DATAGRAM_SIZES: u32 = 4;
pub const SECPKG_ATTR_DCE_INFO: u32 = 3;
pub const SECPKG_ATTR_DTLS_MTU: u32 = 34;
pub const SECPKG_ATTR_ENDPOINT_BINDINGS: u32 = 26;
pub const SECPKG_ATTR_FLAGS: u32 = 14;
pub const SECPKG_ATTR_IS_LOOPBACK: u32 = 37;
pub const SECPKG_ATTR_KEY_INFO: u32 = 5;
pub const SECPKG_ATTR_LAST_CLIENT_TOKEN_STATUS: u32 = 30;
pub type SECPKG_ATTR_LCT_STATUS = i32;
pub const SECPKG_ATTR_LIFESPAN: u32 = 2;
pub const SECPKG_ATTR_LOGOFF_TIME: u32 = 21;
pub const SECPKG_ATTR_NAMES: u32 = 1;
pub const SECPKG_ATTR_NATIVE_NAMES: u32 = 13;
pub const SECPKG_ATTR_NEGOTIATED_TLS_EXTENSIONS: u32 = 36;
pub const SECPKG_ATTR_NEGOTIATION_INFO: u32 = 12;
pub const SECPKG_ATTR_NEGO_INFO_FLAG_NO_KERBEROS: u32 = 1;
pub const SECPKG_ATTR_NEGO_INFO_FLAG_NO_NTLM: u32 = 2;
pub const SECPKG_ATTR_NEGO_KEYS: u32 = 22;
pub const SECPKG_ATTR_NEGO_PKG_INFO: u32 = 31;
pub const SECPKG_ATTR_NEGO_STATUS: u32 = 32;
pub const SECPKG_ATTR_PACKAGE_INFO: u32 = 10;
pub const SECPKG_ATTR_PASSWORD_EXPIRY: u32 = 8;
pub const SECPKG_ATTR_PROMPTING_NEEDED: u32 = 24;
pub const SECPKG_ATTR_PROTO_INFO: u32 = 7;
pub const SECPKG_ATTR_SESSION_KEY: u32 = 9;
pub const SECPKG_ATTR_SIZES: u32 = 0;
pub const SECPKG_ATTR_STREAM_SIZES: u32 = 4;
pub const SECPKG_ATTR_SUBJECT_SECURITY_ATTRIBUTES: u32 = 128;
pub const SECPKG_ATTR_TARGET: u32 = 19;
pub const SECPKG_ATTR_TARGET_INFORMATION: u32 = 17;
pub const SECPKG_ATTR_UNIQUE_BINDINGS: u32 = 25;
pub const SECPKG_ATTR_USER_FLAGS: u32 = 11;
pub const SECPKG_ATTR_USE_VALIDATED: u32 = 15;
pub const SECPKG_CALLFLAGS_APPCONTAINER: u32 = 1;
pub const SECPKG_CALLFLAGS_APPCONTAINER_AUTHCAPABLE: u32 = 2;
pub const SECPKG_CALLFLAGS_APPCONTAINER_UPNCAPABLE: u32 = 8;
pub const SECPKG_CALLFLAGS_FORCE_SUPPLIED: u32 = 4;
pub const SECPKG_CONTEXT_EXPORT_DELETE_OLD: u32 = 2;
pub const SECPKG_CONTEXT_EXPORT_RESET_NEW: u32 = 1;
pub const SECPKG_CONTEXT_EXPORT_TO_KERNEL: u32 = 4;
pub const SECPKG_CRED_ATTR_CERT: u32 = 4;
pub const SECPKG_CRED_ATTR_KDC_NETWORK_SETTINGS: u32 = 3;
pub const SECPKG_CRED_ATTR_KDC_PROXY_SETTINGS: u32 = 3;
pub const SECPKG_CRED_ATTR_NAMES: u32 = 1;
pub const SECPKG_CRED_ATTR_PAC_BYPASS: u32 = 5;
pub const SECPKG_CRED_ATTR_SSI_PROVIDER: u32 = 2;
pub const SECPKG_CRED_AUTOLOGON_RESTRICTED: u32 = 16;
pub const SECPKG_CRED_BOTH: u32 = 3;
pub type SECPKG_CRED_CLASS = i32;
pub const SECPKG_CRED_DEFAULT: u32 = 4;
pub const SECPKG_CRED_INBOUND: u32 = 1;
pub const SECPKG_CRED_KERB_ANCHOR_DS_VERSION: u32 = 64;
pub const SECPKG_CRED_OUTBOUND: u32 = 2;
pub const SECPKG_CRED_PROCESS_POLICY_ONLY: u32 = 32;
pub const SECPKG_CRED_RESERVED: u32 = 4026531840;
pub const SECPKG_FLAG_ACCEPT_WIN32_NAME: u32 = 512;
pub const SECPKG_FLAG_APPCONTAINER_CHECKS: u32 = 8388608;
pub const SECPKG_FLAG_APPCONTAINER_PASSTHROUGH: u32 = 4194304;
pub const SECPKG_FLAG_APPLY_LOOPBACK: u32 = 33554432;
pub const SECPKG_FLAG_ASCII_BUFFERS: u32 = 16384;
pub const SECPKG_FLAG_CLIENT_ONLY: u32 = 64;
pub const SECPKG_FLAG_CONNECTION: u32 = 16;
pub const SECPKG_FLAG_CREDENTIAL_ISOLATION_ENABLED: u32 = 16777216;
pub const SECPKG_FLAG_DATAGRAM: u32 = 8;
pub const SECPKG_FLAG_DELEGATION: u32 = 131072;
pub const SECPKG_FLAG_EXTENDED_ERROR: u32 = 128;
pub const SECPKG_FLAG_FRAGMENT: u32 = 32768;
pub const SECPKG_FLAG_GSS_COMPATIBLE: u32 = 4096;
pub const SECPKG_FLAG_IMPERSONATION: u32 = 256;
pub const SECPKG_FLAG_INTEGRITY: u32 = 1;
pub const SECPKG_FLAG_LOGON: u32 = 8192;
pub const SECPKG_FLAG_MULTI_REQUIRED: u32 = 32;
pub const SECPKG_FLAG_MUTUAL_AUTH: u32 = 65536;
pub const SECPKG_FLAG_NEGOTIABLE: u32 = 2048;
pub const SECPKG_FLAG_NEGOTIABLE2: u32 = 2097152;
pub const SECPKG_FLAG_NEGO_EXTENDER: u32 = 1048576;
pub const SECPKG_FLAG_PRIVACY: u32 = 2;
pub const SECPKG_FLAG_READONLY_WITH_CHECKSUM: u32 = 262144;
pub const SECPKG_FLAG_RESTRICTED_TOKENS: u32 = 524288;
pub const SECPKG_FLAG_STREAM: u32 = 1024;
pub const SECPKG_FLAG_TOKEN_ONLY: u32 = 4;
pub const SECPKG_ID_NONE: u32 = 65535;
pub const SECPKG_NEGOTIATION_COMPLETE: u32 = 0;
pub const SECPKG_NEGOTIATION_DIRECT: u32 = 3;
pub const SECPKG_NEGOTIATION_IN_PROGRESS: u32 = 2;
pub const SECPKG_NEGOTIATION_OPTIMISTIC: u32 = 1;
pub const SECPKG_NEGOTIATION_TRY_MULTICRED: u32 = 4;
pub const SECPKG_OPTIONS_PERMANENT: u32 = 1;
pub const SECPKG_OPTIONS_TYPE_LSA: u32 = 1;
pub const SECPKG_OPTIONS_TYPE_SSPI: u32 = 2;
pub const SECPKG_OPTIONS_TYPE_UNKNOWN: u32 = 0;
pub const SECQOP_WRAP_NO_ENCRYPT: u32 = 2147483649;
pub const SECQOP_WRAP_OOB_DATA: u32 = 1073741824;
pub const SECURITY_ENTRYPOINT16: windows_sys::core::PCSTR = windows_sys::core::s!("INITSECURITYINTERFACEA");
pub const SECURITY_ENTRYPOINT_ANSIA: windows_sys::core::PCSTR = windows_sys::core::s!("InitSecurityInterfaceA");
pub const SECURITY_ENTRYPOINT_ANSIW: windows_sys::core::PCSTR = windows_sys::core::s!("InitSecurityInterfaceW");
pub type SECURITY_INTEGER = i64;
pub const SECURITY_NATIVE_DREP: u32 = 16;
pub const SECURITY_NETWORK_DREP: u32 = 0;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SECURITY_PACKAGE_OPTIONS {
    pub Size: u32,
    pub Type: u32,
    pub Flags: u32,
    pub SignatureSize: u32,
    pub Signature: *mut core::ffi::c_void,
}
impl Default for SECURITY_PACKAGE_OPTIONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type SECURITY_PCSTR = *const SEC_CHAR;
pub type SECURITY_PSTR = *mut SEC_CHAR;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SECURITY_STRING {
    pub Length: u16,
    pub MaximumLength: u16,
    pub Buffer: *mut u16,
}
impl Default for SECURITY_STRING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SECURITY_SUPPORT_PROVIDER_INTERFACE_VERSION: u32 = 1;
pub const SECURITY_SUPPORT_PROVIDER_INTERFACE_VERSION_2: u32 = 2;
pub const SECURITY_SUPPORT_PROVIDER_INTERFACE_VERSION_3: u32 = 3;
pub const SECURITY_SUPPORT_PROVIDER_INTERFACE_VERSION_4: u32 = 4;
pub const SECURITY_SUPPORT_PROVIDER_INTERFACE_VERSION_5: u32 = 5;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SEC_APPLICATION_PROTOCOLS {
    pub ProtocolListsSize: u32,
    pub ProtocolLists: [SEC_APPLICATION_PROTOCOL_LIST; 1],
}
impl Default for SEC_APPLICATION_PROTOCOLS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SEC_APPLICATION_PROTOCOL_LIST {
    pub ProtoNegoExt: SEC_APPLICATION_PROTOCOL_NEGOTIATION_EXT,
    pub ProtocolListSize: u16,
    pub ProtocolList: [u8; 1],
}
impl Default for SEC_APPLICATION_PROTOCOL_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type SEC_APPLICATION_PROTOCOL_NEGOTIATION_EXT = i32;
pub type SEC_APPLICATION_PROTOCOL_NEGOTIATION_STATUS = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SEC_APP_SESSION_STATE {
    pub AppSessionStateSize: u16,
    pub AppSessionState: [u8; 1],
}
impl Default for SEC_APP_SESSION_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SEC_CERTIFICATE_REQUEST_CONTEXT {
    pub cbCertificateRequestContext: u8,
    pub rgCertificateRequestContext: [u8; 1],
}
impl Default for SEC_CERTIFICATE_REQUEST_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SEC_CHANNEL_BINDINGS {
    pub dwInitiatorAddrType: u32,
    pub cbInitiatorLength: u32,
    pub dwInitiatorOffset: u32,
    pub dwAcceptorAddrType: u32,
    pub cbAcceptorLength: u32,
    pub dwAcceptorOffset: u32,
    pub cbApplicationDataLength: u32,
    pub dwApplicationDataOffset: u32,
}
pub const SEC_CHANNEL_BINDINGS_AUDIT_BINDINGS: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SEC_CHANNEL_BINDINGS_EX {
    pub magicNumber: u32,
    pub flags: u32,
    pub cbHeaderLength: u32,
    pub cbStructureLength: u32,
    pub dwInitiatorAddrType: u32,
    pub cbInitiatorLength: u32,
    pub dwInitiatorOffset: u32,
    pub dwAcceptorAddrType: u32,
    pub cbAcceptorLength: u32,
    pub dwAcceptorOffset: u32,
    pub cbApplicationDataLength: u32,
    pub dwApplicationDataOffset: u32,
}
pub const SEC_CHANNEL_BINDINGS_EX_MAGIC: u32 = 1480933955;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SEC_CHANNEL_BINDINGS_RESULT {
    pub flags: u32,
}
pub const SEC_CHANNEL_BINDINGS_RESULT_ABSENT: u32 = 2;
pub const SEC_CHANNEL_BINDINGS_RESULT_CLIENT_SUPPORT: u32 = 1;
pub const SEC_CHANNEL_BINDINGS_RESULT_NOTVALID: u32 = 12;
pub const SEC_CHANNEL_BINDINGS_RESULT_NOTVALID_MISMATCH: u32 = 4;
pub const SEC_CHANNEL_BINDINGS_RESULT_NOTVALID_MISSING: u32 = 8;
pub const SEC_CHANNEL_BINDINGS_RESULT_VALID: u32 = 112;
pub const SEC_CHANNEL_BINDINGS_RESULT_VALID_MATCHED: u32 = 16;
pub const SEC_CHANNEL_BINDINGS_RESULT_VALID_MISSING: u32 = 64;
pub const SEC_CHANNEL_BINDINGS_RESULT_VALID_PROXY: u32 = 32;
pub const SEC_CHANNEL_BINDINGS_VALID_FLAGS: u32 = 1;
pub type SEC_CHAR = i8;
pub const SEC_DELETED_HANDLE: i32 = -2;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SEC_DTLS_MTU {
    pub PathMTU: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SEC_FLAGS {
    pub Flags: u64,
}
#[cfg(feature = "Win32_ncrypt")]
pub type SEC_GET_KEY_FN = Option<unsafe extern "system" fn(arg: *mut core::ffi::c_void, principal: *mut core::ffi::c_void, keyver: u32, key: *mut *mut core::ffi::c_void, status: *mut super::ncrypt::SECURITY_STATUS)>;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SEC_NEGOTIATION_INFO {
    pub Size: u32,
    pub NameLength: u32,
    pub Name: *mut SEC_WCHAR,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for SEC_NEGOTIATION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SEC_PRESHAREDKEY {
    pub KeySize: u16,
    pub Key: [u8; 1],
}
impl Default for SEC_PRESHAREDKEY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SEC_PRESHAREDKEY_IDENTITY {
    pub KeyIdentitySize: u16,
    pub KeyIdentity: [u8; 1],
}
impl Default for SEC_PRESHAREDKEY_IDENTITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SEC_SESSION_TICKET {
    pub SessionTicketSize: u16,
    pub SessionTicket: [u8; 1],
}
impl Default for SEC_SESSION_TICKET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SEC_SRTP_MASTER_KEY_IDENTIFIER {
    pub MasterKeyIdentifierSize: u8,
    pub MasterKeyIdentifier: [u8; 1],
}
impl Default for SEC_SRTP_MASTER_KEY_IDENTIFIER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SEC_SRTP_PROTECTION_PROFILES {
    pub ProfilesSize: u16,
    pub ProfilesList: [u16; 1],
}
impl Default for SEC_SRTP_PROTECTION_PROFILES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SEC_TOKEN_BINDING {
    pub MajorVersion: u8,
    pub MinorVersion: u8,
    pub KeyParametersSize: u16,
    pub KeyParameters: [u8; 1],
}
impl Default for SEC_TOKEN_BINDING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SEC_TRAFFIC_SECRETS {
    pub SymmetricAlgId: [u16; 64],
    pub ChainingMode: [u16; 64],
    pub HashAlgId: [u16; 64],
    pub KeySize: u16,
    pub IvSize: u16,
    pub MsgSequenceStart: u16,
    pub MsgSequenceEnd: u16,
    pub TrafficSecretType: SEC_TRAFFIC_SECRET_TYPE,
    pub TrafficSecretSize: u16,
    pub TrafficSecret: [u8; 1],
}
impl Default for SEC_TRAFFIC_SECRETS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type SEC_TRAFFIC_SECRET_TYPE = i32;
pub type SEC_WCHAR = u16;
pub const SEC_WINNT_AUTH_IDENTITY_ENCRYPT_FOR_SYSTEM: u32 = 4;
pub const SEC_WINNT_AUTH_IDENTITY_ENCRYPT_SAME_LOGON: u32 = 1;
pub const SEC_WINNT_AUTH_IDENTITY_ENCRYPT_SAME_PROCESS: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SEC_WINNT_AUTH_IDENTITY_EX2 {
    pub Version: u32,
    pub cbHeaderLength: u16,
    pub cbStructureLength: u32,
    pub UserOffset: u32,
    pub UserLength: u16,
    pub DomainOffset: u32,
    pub DomainLength: u16,
    pub PackedCredentialsOffset: u32,
    pub PackedCredentialsLength: u16,
    pub Flags: u32,
    pub PackageListOffset: u32,
    pub PackageListLength: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SEC_WINNT_AUTH_IDENTITY_EXA {
    pub Version: u32,
    pub Length: u32,
    pub User: *mut u8,
    pub UserLength: u32,
    pub Domain: *mut u8,
    pub DomainLength: u32,
    pub Password: *mut u8,
    pub PasswordLength: u32,
    pub Flags: u32,
    pub PackageList: *mut u8,
    pub PackageListLength: u32,
}
impl Default for SEC_WINNT_AUTH_IDENTITY_EXA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SEC_WINNT_AUTH_IDENTITY_EXW {
    pub Version: u32,
    pub Length: u32,
    pub User: *mut u16,
    pub UserLength: u32,
    pub Domain: *mut u16,
    pub DomainLength: u32,
    pub Password: *mut u16,
    pub PasswordLength: u32,
    pub Flags: u32,
    pub PackageList: *mut u16,
    pub PackageListLength: u32,
}
impl Default for SEC_WINNT_AUTH_IDENTITY_EXW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SEC_WINNT_AUTH_IDENTITY_FLAGS_ID_PROVIDER: u32 = 524288;
pub const SEC_WINNT_AUTH_IDENTITY_FLAGS_NULL_DOMAIN: u32 = 262144;
pub const SEC_WINNT_AUTH_IDENTITY_FLAGS_NULL_USER: u32 = 131072;
pub const SEC_WINNT_AUTH_IDENTITY_FLAGS_PROCESS_ENCRYPTED: u32 = 16;
pub const SEC_WINNT_AUTH_IDENTITY_FLAGS_RESERVED: u32 = 65536;
pub const SEC_WINNT_AUTH_IDENTITY_FLAGS_SSPIPFC_CREDPROV_DO_NOT_LOAD: u32 = 268435456;
pub const SEC_WINNT_AUTH_IDENTITY_FLAGS_SSPIPFC_CREDPROV_DO_NOT_SAVE: u32 = 2147483648;
pub const SEC_WINNT_AUTH_IDENTITY_FLAGS_SSPIPFC_NO_CHECKBOX: u32 = 536870912;
pub const SEC_WINNT_AUTH_IDENTITY_FLAGS_SSPIPFC_SAVE_CRED_BY_CALLER: i32 = -2147483648;
pub const SEC_WINNT_AUTH_IDENTITY_FLAGS_SSPIPFC_SAVE_CRED_CHECKED: u32 = 1073741824;
pub const SEC_WINNT_AUTH_IDENTITY_FLAGS_SSPIPFC_USE_MASK: u32 = 4278190080;
pub const SEC_WINNT_AUTH_IDENTITY_FLAGS_SYSTEM_ENCRYPTED: u32 = 128;
pub const SEC_WINNT_AUTH_IDENTITY_FLAGS_SYSTEM_PROTECTED: u32 = 32;
pub const SEC_WINNT_AUTH_IDENTITY_FLAGS_USER_PROTECTED: u32 = 64;
pub const SEC_WINNT_AUTH_IDENTITY_FLAGS_VALID_SSPIPFC_FLAGS: i32 = -268435456;
#[repr(C)]
#[cfg(feature = "Win32_rpc")]
#[derive(Clone, Copy)]
pub union SEC_WINNT_AUTH_IDENTITY_INFO {
    pub AuthIdExw: SEC_WINNT_AUTH_IDENTITY_EXW,
    pub AuthIdExa: SEC_WINNT_AUTH_IDENTITY_EXA,
    pub AuthId_a: super::rpc::SEC_WINNT_AUTH_IDENTITY_A,
    pub AuthId_w: super::rpc::SEC_WINNT_AUTH_IDENTITY_W,
    pub AuthIdEx2: SEC_WINNT_AUTH_IDENTITY_EX2,
}
#[cfg(feature = "Win32_rpc")]
impl Default for SEC_WINNT_AUTH_IDENTITY_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SEC_WINNT_AUTH_IDENTITY_MARSHALLED: u32 = 4;
pub const SEC_WINNT_AUTH_IDENTITY_ONLY: u32 = 8;
pub const SEC_WINNT_AUTH_IDENTITY_VERSION: u32 = 512;
pub const SEC_WINNT_AUTH_IDENTITY_VERSION_2: u32 = 513;
#[cfg(feature = "Win32_ncrypt")]
pub type SET_CONTEXT_ATTRIBUTES_FN_A = Option<unsafe extern "system" fn(param0: PCtxtHandle, param1: u32, param2: *mut core::ffi::c_void, param3: u32) -> super::ncrypt::SECURITY_STATUS>;
#[cfg(feature = "Win32_ncrypt")]
pub type SET_CONTEXT_ATTRIBUTES_FN_W = Option<unsafe extern "system" fn(param0: PCtxtHandle, param1: u32, param2: *mut core::ffi::c_void, param3: u32) -> super::ncrypt::SECURITY_STATUS>;
#[cfg(feature = "Win32_ncrypt")]
pub type SET_CREDENTIALS_ATTRIBUTES_FN_A = Option<unsafe extern "system" fn(param0: PCredHandle, param1: u32, param2: *mut core::ffi::c_void, param3: u32) -> super::ncrypt::SECURITY_STATUS>;
#[cfg(feature = "Win32_ncrypt")]
pub type SET_CREDENTIALS_ATTRIBUTES_FN_W = Option<unsafe extern "system" fn(param0: PCredHandle, param1: u32, param2: *mut core::ffi::c_void, param3: u32) -> super::ncrypt::SECURITY_STATUS>;
pub const SSPIPFC_CREDPROV_DO_NOT_LOAD: u32 = 4;
pub const SSPIPFC_CREDPROV_DO_NOT_SAVE: u32 = 1;
pub const SSPIPFC_NO_CHECKBOX: u32 = 2;
pub const SSPIPFC_SAVE_CRED_BY_CALLER: u32 = 1;
pub const SSPIPFC_USE_CREDUIBROKER: u32 = 8;
pub const SSPIPFC_VALID_FLAGS: u32 = 15;
pub const SZ_ALG_MAX_SIZE: u32 = 64;
pub const Sasl_AuthZIDForbidden: SASL_AUTHZID_STATE = 0;
pub const Sasl_AuthZIDProcessed: SASL_AUTHZID_STATE = 1;
pub const SecApplicationProtocolNegotiationExt_ALPN: SEC_APPLICATION_PROTOCOL_NEGOTIATION_EXT = 2;
pub const SecApplicationProtocolNegotiationExt_NPN: SEC_APPLICATION_PROTOCOL_NEGOTIATION_EXT = 1;
pub const SecApplicationProtocolNegotiationExt_None: SEC_APPLICATION_PROTOCOL_NEGOTIATION_EXT = 0;
pub const SecApplicationProtocolNegotiationStatus_None: SEC_APPLICATION_PROTOCOL_NEGOTIATION_STATUS = 0;
pub const SecApplicationProtocolNegotiationStatus_SelectedClientOnly: SEC_APPLICATION_PROTOCOL_NEGOTIATION_STATUS = 2;
pub const SecApplicationProtocolNegotiationStatus_Success: SEC_APPLICATION_PROTOCOL_NEGOTIATION_STATUS = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SecBuffer {
    pub cbBuffer: u32,
    pub BufferType: u32,
    pub pvBuffer: *mut core::ffi::c_void,
}
impl Default for SecBuffer {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SecBufferDesc {
    pub ulVersion: u32,
    pub cBuffers: u32,
    pub pBuffers: PSecBuffer,
}
impl Default for SecBufferDesc {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type SecDelegationType = i32;
pub const SecDirectory: SecDelegationType = 3;
pub const SecFull: SecDelegationType = 0;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SecHandle {
    pub dwLower: usize,
    pub dwUpper: usize,
}
pub const SecObject: SecDelegationType = 4;
pub const SecPkgAttrLastClientTokenMaybe: SECPKG_ATTR_LCT_STATUS = 2;
pub const SecPkgAttrLastClientTokenNo: SECPKG_ATTR_LCT_STATUS = 1;
pub const SecPkgAttrLastClientTokenYes: SECPKG_ATTR_LCT_STATUS = 0;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SecPkgContext_AccessToken {
    pub AccessToken: *mut core::ffi::c_void,
}
impl Default for SecPkgContext_AccessToken {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SecPkgContext_ApplicationProtocol {
    pub ProtoNegoStatus: SEC_APPLICATION_PROTOCOL_NEGOTIATION_STATUS,
    pub ProtoNegoExt: SEC_APPLICATION_PROTOCOL_NEGOTIATION_EXT,
    pub ProtocolIdSize: u8,
    pub ProtocolId: [u8; 255],
}
impl Default for SecPkgContext_ApplicationProtocol {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SecPkgContext_AuthorityA {
    pub sAuthorityName: *mut SEC_CHAR,
}
impl Default for SecPkgContext_AuthorityA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SecPkgContext_AuthorityW {
    pub sAuthorityName: *mut SEC_WCHAR,
}
impl Default for SecPkgContext_AuthorityW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SecPkgContext_AuthzID {
    pub AuthzIDLength: u32,
    pub AuthzID: *mut i8,
}
impl Default for SecPkgContext_AuthzID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SecPkgContext_Bindings {
    pub BindingsLength: u32,
    pub Bindings: *mut SEC_CHANNEL_BINDINGS,
}
impl Default for SecPkgContext_Bindings {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SecPkgContext_ClientSpecifiedTarget {
    pub sTargetName: *mut SEC_WCHAR,
}
impl Default for SecPkgContext_ClientSpecifiedTarget {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SecPkgContext_CredInfo {
    pub CredClass: SECPKG_CRED_CLASS,
    pub IsPromptingNeeded: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SecPkgContext_CredentialNameA {
    pub CredentialType: u32,
    pub sCredentialName: *mut SEC_CHAR,
}
impl Default for SecPkgContext_CredentialNameA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SecPkgContext_CredentialNameW {
    pub CredentialType: u32,
    pub sCredentialName: *mut SEC_WCHAR,
}
impl Default for SecPkgContext_CredentialNameW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type SecPkgContext_DatagramSizes = SecPkgContext_StreamSizes;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SecPkgContext_DceInfo {
    pub AuthzSvc: u32,
    pub pPac: *mut core::ffi::c_void,
}
impl Default for SecPkgContext_DceInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SecPkgContext_Flags {
    pub Flags: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SecPkgContext_KeyInfoA {
    pub sSignatureAlgorithmName: *mut SEC_CHAR,
    pub sEncryptAlgorithmName: *mut SEC_CHAR,
    pub KeySize: u32,
    pub SignatureAlgorithm: u32,
    pub EncryptAlgorithm: u32,
}
impl Default for SecPkgContext_KeyInfoA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SecPkgContext_KeyInfoW {
    pub sSignatureAlgorithmName: *mut SEC_WCHAR,
    pub sEncryptAlgorithmName: *mut SEC_WCHAR,
    pub KeySize: u32,
    pub SignatureAlgorithm: u32,
    pub EncryptAlgorithm: u32,
}
impl Default for SecPkgContext_KeyInfoW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SecPkgContext_LastClientTokenStatus {
    pub LastClientTokenStatus: SECPKG_ATTR_LCT_STATUS,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SecPkgContext_Lifespan {
    pub tsStart: TimeStamp,
    pub tsExpiry: TimeStamp,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SecPkgContext_LogoffTime {
    pub tsLogoffTime: TimeStamp,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SecPkgContext_NamesA {
    pub sUserName: *mut SEC_CHAR,
}
impl Default for SecPkgContext_NamesA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SecPkgContext_NamesW {
    pub sUserName: *mut SEC_WCHAR,
}
impl Default for SecPkgContext_NamesW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SecPkgContext_NativeNamesA {
    pub sClientName: *mut SEC_CHAR,
    pub sServerName: *mut SEC_CHAR,
}
impl Default for SecPkgContext_NativeNamesA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SecPkgContext_NativeNamesW {
    pub sClientName: *mut SEC_WCHAR,
    pub sServerName: *mut SEC_WCHAR,
}
impl Default for SecPkgContext_NativeNamesW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SecPkgContext_NegoKeys {
    pub KeyType: u32,
    pub KeyLength: u16,
    pub KeyValue: *mut u8,
    pub VerifyKeyType: u32,
    pub VerifyKeyLength: u16,
    pub VerifyKeyValue: *mut u8,
}
impl Default for SecPkgContext_NegoKeys {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SecPkgContext_NegoPackageInfo {
    pub PackageMask: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SecPkgContext_NegoStatus {
    pub LastStatus: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SecPkgContext_NegotiatedTlsExtensions {
    pub ExtensionsCount: u32,
    pub Extensions: *mut u16,
}
impl Default for SecPkgContext_NegotiatedTlsExtensions {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SecPkgContext_NegotiationInfoA {
    pub PackageInfo: PSecPkgInfoA,
    pub NegotiationState: u32,
}
impl Default for SecPkgContext_NegotiationInfoA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SecPkgContext_NegotiationInfoW {
    pub PackageInfo: PSecPkgInfoW,
    pub NegotiationState: u32,
}
impl Default for SecPkgContext_NegotiationInfoW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SecPkgContext_PackageInfoA {
    pub PackageInfo: PSecPkgInfoA,
}
impl Default for SecPkgContext_PackageInfoA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SecPkgContext_PackageInfoW {
    pub PackageInfo: PSecPkgInfoW,
}
impl Default for SecPkgContext_PackageInfoW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SecPkgContext_PasswordExpiry {
    pub tsPasswordExpires: TimeStamp,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SecPkgContext_ProtoInfoA {
    pub sProtocolName: *mut SEC_CHAR,
    pub majorVersion: u32,
    pub minorVersion: u32,
}
impl Default for SecPkgContext_ProtoInfoA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SecPkgContext_ProtoInfoW {
    pub sProtocolName: *mut SEC_WCHAR,
    pub majorVersion: u32,
    pub minorVersion: u32,
}
impl Default for SecPkgContext_ProtoInfoW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SecPkgContext_SessionKey {
    pub SessionKeyLength: u32,
    pub SessionKey: *mut u8,
}
impl Default for SecPkgContext_SessionKey {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SecPkgContext_Sizes {
    pub cbMaxToken: u32,
    pub cbMaxSignature: u32,
    pub cbBlockSize: u32,
    pub cbSecurityTrailer: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SecPkgContext_StreamSizes {
    pub cbHeader: u32,
    pub cbTrailer: u32,
    pub cbMaximumMessage: u32,
    pub cBuffers: u32,
    pub cbBlockSize: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SecPkgContext_SubjectAttributes {
    pub AttributeInfo: *mut core::ffi::c_void,
}
impl Default for SecPkgContext_SubjectAttributes {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SecPkgContext_Target {
    pub TargetLength: u32,
    pub Target: *mut i8,
}
impl Default for SecPkgContext_Target {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SecPkgContext_TargetInformation {
    pub MarshalledTargetInfoLength: u32,
    pub MarshalledTargetInfo: *mut u8,
}
impl Default for SecPkgContext_TargetInformation {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SecPkgContext_UserFlags {
    pub UserFlags: u32,
}
pub const SecPkgCredClass_Ephemeral: SECPKG_CRED_CLASS = 10;
pub const SecPkgCredClass_Explicit: SECPKG_CRED_CLASS = 40;
pub const SecPkgCredClass_None: SECPKG_CRED_CLASS = 0;
pub const SecPkgCredClass_PersistedGeneric: SECPKG_CRED_CLASS = 20;
pub const SecPkgCredClass_PersistedSpecific: SECPKG_CRED_CLASS = 30;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SecPkgCredentials_Cert {
    pub EncodedCertSize: u32,
    pub EncodedCert: *mut u8,
}
impl Default for SecPkgCredentials_Cert {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SecPkgCredentials_KdcNetworkSettingsW {
    pub Version: u32,
    pub Flags: u32,
    pub ProxyServerOffset: u16,
    pub ProxyServerLength: u16,
    pub ClientTlsCredOffset: u16,
    pub ClientTlsCredLength: u16,
    pub DcDiscoveryFlags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SecPkgCredentials_KdcProxySettingsW {
    pub Version: u32,
    pub Flags: u32,
    pub ProxyServerOffset: u16,
    pub ProxyServerLength: u16,
    pub ClientTlsCredOffset: u16,
    pub ClientTlsCredLength: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SecPkgCredentials_NamesA {
    pub sUserName: *mut SEC_CHAR,
}
impl Default for SecPkgCredentials_NamesA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SecPkgCredentials_NamesW {
    pub sUserName: *mut SEC_WCHAR,
}
impl Default for SecPkgCredentials_NamesW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SecPkgCredentials_SSIProviderA {
    pub sProviderName: *mut SEC_CHAR,
    pub ProviderInfoLength: u32,
    pub ProviderInfo: *mut i8,
}
impl Default for SecPkgCredentials_SSIProviderA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SecPkgCredentials_SSIProviderW {
    pub sProviderName: *mut SEC_WCHAR,
    pub ProviderInfoLength: u32,
    pub ProviderInfo: *mut i8,
}
impl Default for SecPkgCredentials_SSIProviderW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SecPkgInfoA {
    pub fCapabilities: u32,
    pub wVersion: u16,
    pub wRPCID: u16,
    pub cbMaxToken: u32,
    pub Name: *mut SEC_CHAR,
    pub Comment: *mut SEC_CHAR,
}
impl Default for SecPkgInfoA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SecPkgInfoW {
    pub fCapabilities: u32,
    pub wVersion: u16,
    pub wRPCID: u16,
    pub cbMaxToken: u32,
    pub Name: *mut SEC_WCHAR,
    pub Comment: *mut SEC_WCHAR,
}
impl Default for SecPkgInfoW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SecService: SecDelegationType = 1;
pub const SecTrafficSecret_Client: SEC_TRAFFIC_SECRET_TYPE = 1;
pub const SecTrafficSecret_None: SEC_TRAFFIC_SECRET_TYPE = 0;
pub const SecTrafficSecret_Server: SEC_TRAFFIC_SECRET_TYPE = 2;
pub const SecTree: SecDelegationType = 2;
#[repr(C)]
#[cfg(feature = "Win32_ncrypt")]
#[derive(Clone, Copy)]
pub struct SecurityFunctionTableA {
    pub dwVersion: u32,
    pub EnumerateSecurityPackagesA: ENUMERATE_SECURITY_PACKAGES_FN_A,
    pub QueryCredentialsAttributesA: QUERY_CREDENTIALS_ATTRIBUTES_FN_A,
    pub AcquireCredentialsHandleA: ACQUIRE_CREDENTIALS_HANDLE_FN_A,
    pub FreeCredentialsHandle: FREE_CREDENTIALS_HANDLE_FN,
    pub Reserved2: *mut core::ffi::c_void,
    pub InitializeSecurityContextA: INITIALIZE_SECURITY_CONTEXT_FN_A,
    pub AcceptSecurityContext: ACCEPT_SECURITY_CONTEXT_FN,
    pub CompleteAuthToken: COMPLETE_AUTH_TOKEN_FN,
    pub DeleteSecurityContext: DELETE_SECURITY_CONTEXT_FN,
    pub ApplyControlToken: APPLY_CONTROL_TOKEN_FN,
    pub QueryContextAttributesA: QUERY_CONTEXT_ATTRIBUTES_FN_A,
    pub ImpersonateSecurityContext: IMPERSONATE_SECURITY_CONTEXT_FN,
    pub RevertSecurityContext: REVERT_SECURITY_CONTEXT_FN,
    pub MakeSignature: MAKE_SIGNATURE_FN,
    pub VerifySignature: VERIFY_SIGNATURE_FN,
    pub FreeContextBuffer: FREE_CONTEXT_BUFFER_FN,
    pub QuerySecurityPackageInfoA: QUERY_SECURITY_PACKAGE_INFO_FN_A,
    pub Reserved3: *mut core::ffi::c_void,
    pub Reserved4: *mut core::ffi::c_void,
    pub ExportSecurityContext: EXPORT_SECURITY_CONTEXT_FN,
    pub ImportSecurityContextA: IMPORT_SECURITY_CONTEXT_FN_A,
    pub AddCredentialsA: ADD_CREDENTIALS_FN_A,
    pub Reserved8: *mut core::ffi::c_void,
    pub QuerySecurityContextToken: QUERY_SECURITY_CONTEXT_TOKEN_FN,
    pub EncryptMessage: ENCRYPT_MESSAGE_FN,
    pub DecryptMessage: DECRYPT_MESSAGE_FN,
    pub SetContextAttributesA: SET_CONTEXT_ATTRIBUTES_FN_A,
    pub SetCredentialsAttributesA: SET_CREDENTIALS_ATTRIBUTES_FN_A,
    pub ChangeAccountPasswordA: CHANGE_PASSWORD_FN_A,
    pub QueryContextAttributesExA: QUERY_CONTEXT_ATTRIBUTES_EX_FN_A,
    pub QueryCredentialsAttributesExA: QUERY_CREDENTIALS_ATTRIBUTES_EX_FN_A,
}
#[cfg(feature = "Win32_ncrypt")]
impl Default for SecurityFunctionTableA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_ncrypt")]
#[derive(Clone, Copy)]
pub struct SecurityFunctionTableW {
    pub dwVersion: u32,
    pub EnumerateSecurityPackagesW: ENUMERATE_SECURITY_PACKAGES_FN_W,
    pub QueryCredentialsAttributesW: QUERY_CREDENTIALS_ATTRIBUTES_FN_W,
    pub AcquireCredentialsHandleW: ACQUIRE_CREDENTIALS_HANDLE_FN_W,
    pub FreeCredentialsHandle: FREE_CREDENTIALS_HANDLE_FN,
    pub Reserved2: *mut core::ffi::c_void,
    pub InitializeSecurityContextW: INITIALIZE_SECURITY_CONTEXT_FN_W,
    pub AcceptSecurityContext: ACCEPT_SECURITY_CONTEXT_FN,
    pub CompleteAuthToken: COMPLETE_AUTH_TOKEN_FN,
    pub DeleteSecurityContext: DELETE_SECURITY_CONTEXT_FN,
    pub ApplyControlToken: APPLY_CONTROL_TOKEN_FN,
    pub QueryContextAttributesW: QUERY_CONTEXT_ATTRIBUTES_FN_W,
    pub ImpersonateSecurityContext: IMPERSONATE_SECURITY_CONTEXT_FN,
    pub RevertSecurityContext: REVERT_SECURITY_CONTEXT_FN,
    pub MakeSignature: MAKE_SIGNATURE_FN,
    pub VerifySignature: VERIFY_SIGNATURE_FN,
    pub FreeContextBuffer: FREE_CONTEXT_BUFFER_FN,
    pub QuerySecurityPackageInfoW: QUERY_SECURITY_PACKAGE_INFO_FN_W,
    pub Reserved3: *mut core::ffi::c_void,
    pub Reserved4: *mut core::ffi::c_void,
    pub ExportSecurityContext: EXPORT_SECURITY_CONTEXT_FN,
    pub ImportSecurityContextW: IMPORT_SECURITY_CONTEXT_FN_W,
    pub AddCredentialsW: ADD_CREDENTIALS_FN_W,
    pub Reserved8: *mut core::ffi::c_void,
    pub QuerySecurityContextToken: QUERY_SECURITY_CONTEXT_TOKEN_FN,
    pub EncryptMessage: ENCRYPT_MESSAGE_FN,
    pub DecryptMessage: DECRYPT_MESSAGE_FN,
    pub SetContextAttributesW: SET_CONTEXT_ATTRIBUTES_FN_W,
    pub SetCredentialsAttributesW: SET_CREDENTIALS_ATTRIBUTES_FN_W,
    pub ChangeAccountPasswordW: CHANGE_PASSWORD_FN_W,
    pub QueryContextAttributesExW: QUERY_CONTEXT_ATTRIBUTES_EX_FN_W,
    pub QueryCredentialsAttributesExW: QUERY_CREDENTIALS_ATTRIBUTES_EX_FN_W,
}
#[cfg(feature = "Win32_ncrypt")]
impl Default for SecurityFunctionTableW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type TimeStamp = SECURITY_INTEGER;
#[cfg(feature = "Win32_ncrypt")]
pub type VERIFY_SIGNATURE_FN = Option<unsafe extern "system" fn(param0: PCtxtHandle, param1: *mut SecBufferDesc, param2: u32, param3: *mut u32) -> super::ncrypt::SECURITY_STATUS>;
pub type _SECURITY_INTEGER = i64;
