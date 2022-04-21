#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredDeleteA(targetname: ::windows_sys::core::PCSTR, r#type: u32, flags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredDeleteW(targetname: ::windows_sys::core::PCWSTR, r#type: u32, flags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredEnumerateA(filter: ::windows_sys::core::PCSTR, flags: CRED_ENUMERATE_FLAGS, count: *mut u32, credential: *mut *mut *mut CREDENTIALA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredEnumerateW(filter: ::windows_sys::core::PCWSTR, flags: CRED_ENUMERATE_FLAGS, count: *mut u32, credential: *mut *mut *mut CREDENTIALW) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredFindBestCredentialA(targetname: ::windows_sys::core::PCSTR, r#type: u32, flags: u32, credential: *mut *mut CREDENTIALA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredFindBestCredentialW(targetname: ::windows_sys::core::PCWSTR, r#type: u32, flags: u32, credential: *mut *mut CREDENTIALW) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
    pub fn CredFree(buffer: *const ::core::ffi::c_void);
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredGetSessionTypes(maximumpersistcount: u32, maximumpersist: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredGetTargetInfoA(targetname: ::windows_sys::core::PCSTR, flags: u32, targetinfo: *mut *mut CREDENTIAL_TARGET_INFORMATIONA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredGetTargetInfoW(targetname: ::windows_sys::core::PCWSTR, flags: u32, targetinfo: *mut *mut CREDENTIAL_TARGET_INFORMATIONW) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredIsMarshaledCredentialA(marshaledcredential: ::windows_sys::core::PCSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredIsMarshaledCredentialW(marshaledcredential: ::windows_sys::core::PCWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredIsProtectedA(pszprotectedcredentials: ::windows_sys::core::PCSTR, pprotectiontype: *mut CRED_PROTECTION_TYPE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredIsProtectedW(pszprotectedcredentials: ::windows_sys::core::PCWSTR, pprotectiontype: *mut CRED_PROTECTION_TYPE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredMarshalCredentialA(credtype: CRED_MARSHAL_TYPE, credential: *const ::core::ffi::c_void, marshaledcredential: *mut ::windows_sys::core::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredMarshalCredentialW(credtype: CRED_MARSHAL_TYPE, credential: *const ::core::ffi::c_void, marshaledcredential: *mut ::windows_sys::core::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredPackAuthenticationBufferA(dwflags: CRED_PACK_FLAGS, pszusername: ::windows_sys::core::PCSTR, pszpassword: ::windows_sys::core::PCSTR, ppackedcredentials: *mut u8, pcbpackedcredentials: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredPackAuthenticationBufferW(dwflags: CRED_PACK_FLAGS, pszusername: ::windows_sys::core::PCWSTR, pszpassword: ::windows_sys::core::PCWSTR, ppackedcredentials: *mut u8, pcbpackedcredentials: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredProtectA(fasself: super::super::Foundation::BOOL, pszcredentials: ::windows_sys::core::PCSTR, cchcredentials: u32, pszprotectedcredentials: ::windows_sys::core::PSTR, pcchmaxchars: *mut u32, protectiontype: *mut CRED_PROTECTION_TYPE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredProtectW(fasself: super::super::Foundation::BOOL, pszcredentials: ::windows_sys::core::PCWSTR, cchcredentials: u32, pszprotectedcredentials: ::windows_sys::core::PWSTR, pcchmaxchars: *mut u32, protectiontype: *mut CRED_PROTECTION_TYPE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredReadA(targetname: ::windows_sys::core::PCSTR, r#type: u32, flags: u32, credential: *mut *mut CREDENTIALA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredReadDomainCredentialsA(targetinfo: *const CREDENTIAL_TARGET_INFORMATIONA, flags: u32, count: *mut u32, credential: *mut *mut *mut CREDENTIALA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredReadDomainCredentialsW(targetinfo: *const CREDENTIAL_TARGET_INFORMATIONW, flags: u32, count: *mut u32, credential: *mut *mut *mut CREDENTIALW) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredReadW(targetname: ::windows_sys::core::PCWSTR, r#type: u32, flags: u32, credential: *mut *mut CREDENTIALW) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredRenameA(oldtargetname: ::windows_sys::core::PCSTR, newtargetname: ::windows_sys::core::PCSTR, r#type: u32, flags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredRenameW(oldtargetname: ::windows_sys::core::PCWSTR, newtargetname: ::windows_sys::core::PCWSTR, r#type: u32, flags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredUICmdLinePromptForCredentialsA(psztargetname: ::windows_sys::core::PCSTR, pcontext: *mut SecHandle, dwautherror: u32, username: ::windows_sys::core::PSTR, uluserbuffersize: u32, pszpassword: ::windows_sys::core::PSTR, ulpasswordbuffersize: u32, pfsave: *mut super::super::Foundation::BOOL, dwflags: CREDUI_FLAGS) -> u32;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredUICmdLinePromptForCredentialsW(psztargetname: ::windows_sys::core::PCWSTR, pcontext: *mut SecHandle, dwautherror: u32, username: ::windows_sys::core::PWSTR, uluserbuffersize: u32, pszpassword: ::windows_sys::core::PWSTR, ulpasswordbuffersize: u32, pfsave: *mut super::super::Foundation::BOOL, dwflags: CREDUI_FLAGS) -> u32;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredUIConfirmCredentialsA(psztargetname: ::windows_sys::core::PCSTR, bconfirm: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredUIConfirmCredentialsW(psztargetname: ::windows_sys::core::PCWSTR, bconfirm: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
    pub fn CredUIParseUserNameA(username: ::windows_sys::core::PCSTR, user: ::windows_sys::core::PSTR, userbuffersize: u32, domain: ::windows_sys::core::PSTR, domainbuffersize: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
    pub fn CredUIParseUserNameW(username: ::windows_sys::core::PCWSTR, user: ::windows_sys::core::PWSTR, userbuffersize: u32, domain: ::windows_sys::core::PWSTR, domainbuffersize: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn CredUIPromptForCredentialsA(puiinfo: *const CREDUI_INFOA, psztargetname: ::windows_sys::core::PCSTR, pcontext: *mut SecHandle, dwautherror: u32, pszusername: ::windows_sys::core::PSTR, ulusernamebuffersize: u32, pszpassword: ::windows_sys::core::PSTR, ulpasswordbuffersize: u32, save: *mut super::super::Foundation::BOOL, dwflags: CREDUI_FLAGS) -> u32;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn CredUIPromptForCredentialsW(puiinfo: *const CREDUI_INFOW, psztargetname: ::windows_sys::core::PCWSTR, pcontext: *mut SecHandle, dwautherror: u32, pszusername: ::windows_sys::core::PWSTR, ulusernamebuffersize: u32, pszpassword: ::windows_sys::core::PWSTR, ulpasswordbuffersize: u32, save: *mut super::super::Foundation::BOOL, dwflags: CREDUI_FLAGS) -> u32;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn CredUIPromptForWindowsCredentialsA(puiinfo: *const CREDUI_INFOA, dwautherror: u32, pulauthpackage: *mut u32, pvinauthbuffer: *const ::core::ffi::c_void, ulinauthbuffersize: u32, ppvoutauthbuffer: *mut *mut ::core::ffi::c_void, puloutauthbuffersize: *mut u32, pfsave: *mut super::super::Foundation::BOOL, dwflags: CREDUIWIN_FLAGS) -> u32;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn CredUIPromptForWindowsCredentialsW(puiinfo: *const CREDUI_INFOW, dwautherror: u32, pulauthpackage: *mut u32, pvinauthbuffer: *const ::core::ffi::c_void, ulinauthbuffersize: u32, ppvoutauthbuffer: *mut *mut ::core::ffi::c_void, puloutauthbuffersize: *mut u32, pfsave: *mut super::super::Foundation::BOOL, dwflags: CREDUIWIN_FLAGS) -> u32;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
    pub fn CredUIReadSSOCredW(pszrealm: ::windows_sys::core::PCWSTR, ppszusername: *mut ::windows_sys::core::PWSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredUIStoreSSOCredW(pszrealm: ::windows_sys::core::PCWSTR, pszusername: ::windows_sys::core::PCWSTR, pszpassword: ::windows_sys::core::PCWSTR, bpersist: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredUnPackAuthenticationBufferA(dwflags: CRED_PACK_FLAGS, pauthbuffer: *const ::core::ffi::c_void, cbauthbuffer: u32, pszusername: ::windows_sys::core::PSTR, pcchlmaxusername: *mut u32, pszdomainname: ::windows_sys::core::PSTR, pcchmaxdomainname: *mut u32, pszpassword: ::windows_sys::core::PSTR, pcchmaxpassword: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredUnPackAuthenticationBufferW(dwflags: CRED_PACK_FLAGS, pauthbuffer: *const ::core::ffi::c_void, cbauthbuffer: u32, pszusername: ::windows_sys::core::PWSTR, pcchmaxusername: *mut u32, pszdomainname: ::windows_sys::core::PWSTR, pcchmaxdomainname: *mut u32, pszpassword: ::windows_sys::core::PWSTR, pcchmaxpassword: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredUnmarshalCredentialA(marshaledcredential: ::windows_sys::core::PCSTR, credtype: *mut CRED_MARSHAL_TYPE, credential: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredUnmarshalCredentialW(marshaledcredential: ::windows_sys::core::PCWSTR, credtype: *mut CRED_MARSHAL_TYPE, credential: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredUnprotectA(fasself: super::super::Foundation::BOOL, pszprotectedcredentials: ::windows_sys::core::PCSTR, cchprotectedcredentials: u32, pszcredentials: ::windows_sys::core::PSTR, pcchmaxchars: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredUnprotectW(fasself: super::super::Foundation::BOOL, pszprotectedcredentials: ::windows_sys::core::PCWSTR, cchprotectedcredentials: u32, pszcredentials: ::windows_sys::core::PWSTR, pcchmaxchars: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredWriteA(credential: *const CREDENTIALA, flags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredWriteDomainCredentialsA(targetinfo: *const CREDENTIAL_TARGET_INFORMATIONA, credential: *const CREDENTIALA, flags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredWriteDomainCredentialsW(targetinfo: *const CREDENTIAL_TARGET_INFORMATIONW, credential: *const CREDENTIALW, flags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredWriteW(credential: *const CREDENTIALW, flags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetOpenCardNameA(param0: *mut OPENCARDNAMEA) -> i32;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetOpenCardNameW(param0: *mut OPENCARDNAMEW) -> i32;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
    pub fn KeyCredentialManagerFreeInformation(keycredentialmanagerinfo: *const KeyCredentialManagerInfo);
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
    pub fn KeyCredentialManagerGetInformation(keycredentialmanagerinfo: *mut *mut KeyCredentialManagerInfo) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn KeyCredentialManagerGetOperationErrorStates(keycredentialmanageroperationtype: KeyCredentialManagerOperationType, isready: *mut super::super::Foundation::BOOL, keycredentialmanageroperationerrorstates: *mut KeyCredentialManagerOperationErrorStates) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn KeyCredentialManagerShowUIOperation(hwndowner: super::super::Foundation::HWND, keycredentialmanageroperationtype: KeyCredentialManagerOperationType) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardAccessStartedEvent() -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
    pub fn SCardAddReaderToGroupA(hcontext: usize, szreadername: ::windows_sys::core::PCSTR, szgroupname: ::windows_sys::core::PCSTR) -> i32;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
    pub fn SCardAddReaderToGroupW(hcontext: usize, szreadername: ::windows_sys::core::PCWSTR, szgroupname: ::windows_sys::core::PCWSTR) -> i32;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
    pub fn SCardAudit(hcontext: usize, dwevent: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
    pub fn SCardBeginTransaction(hcard: usize) -> i32;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
    pub fn SCardCancel(hcontext: usize) -> i32;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
    pub fn SCardConnectA(hcontext: usize, szreader: ::windows_sys::core::PCSTR, dwsharemode: u32, dwpreferredprotocols: u32, phcard: *mut usize, pdwactiveprotocol: *mut u32) -> i32;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
    pub fn SCardConnectW(hcontext: usize, szreader: ::windows_sys::core::PCWSTR, dwsharemode: u32, dwpreferredprotocols: u32, phcard: *mut usize, pdwactiveprotocol: *mut u32) -> i32;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
    pub fn SCardControl(hcard: usize, dwcontrolcode: u32, lpinbuffer: *const ::core::ffi::c_void, cbinbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, cboutbuffersize: u32, lpbytesreturned: *mut u32) -> i32;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
    pub fn SCardDisconnect(hcard: usize, dwdisposition: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
    pub fn SCardDlgExtendedError() -> i32;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
    pub fn SCardEndTransaction(hcard: usize, dwdisposition: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
    pub fn SCardEstablishContext(dwscope: SCARD_SCOPE, pvreserved1: *const ::core::ffi::c_void, pvreserved2: *const ::core::ffi::c_void, phcontext: *mut usize) -> i32;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
    pub fn SCardForgetCardTypeA(hcontext: usize, szcardname: ::windows_sys::core::PCSTR) -> i32;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
    pub fn SCardForgetCardTypeW(hcontext: usize, szcardname: ::windows_sys::core::PCWSTR) -> i32;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
    pub fn SCardForgetReaderA(hcontext: usize, szreadername: ::windows_sys::core::PCSTR) -> i32;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
    pub fn SCardForgetReaderGroupA(hcontext: usize, szgroupname: ::windows_sys::core::PCSTR) -> i32;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
    pub fn SCardForgetReaderGroupW(hcontext: usize, szgroupname: ::windows_sys::core::PCWSTR) -> i32;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
    pub fn SCardForgetReaderW(hcontext: usize, szreadername: ::windows_sys::core::PCWSTR) -> i32;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
    pub fn SCardFreeMemory(hcontext: usize, pvmem: *const ::core::ffi::c_void) -> i32;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
    pub fn SCardGetAttrib(hcard: usize, dwattrid: u32, pbattr: *mut u8, pcbattrlen: *mut u32) -> i32;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
    pub fn SCardGetCardTypeProviderNameA(hcontext: usize, szcardname: ::windows_sys::core::PCSTR, dwproviderid: u32, szprovider: ::windows_sys::core::PSTR, pcchprovider: *mut u32) -> i32;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
    pub fn SCardGetCardTypeProviderNameW(hcontext: usize, szcardname: ::windows_sys::core::PCWSTR, dwproviderid: u32, szprovider: ::windows_sys::core::PWSTR, pcchprovider: *mut u32) -> i32;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
    pub fn SCardGetDeviceTypeIdA(hcontext: usize, szreadername: ::windows_sys::core::PCSTR, pdwdevicetypeid: *mut u32) -> i32;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
    pub fn SCardGetDeviceTypeIdW(hcontext: usize, szreadername: ::windows_sys::core::PCWSTR, pdwdevicetypeid: *mut u32) -> i32;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
    pub fn SCardGetProviderIdA(hcontext: usize, szcard: ::windows_sys::core::PCSTR, pguidproviderid: *mut ::windows_sys::core::GUID) -> i32;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
    pub fn SCardGetProviderIdW(hcontext: usize, szcard: ::windows_sys::core::PCWSTR, pguidproviderid: *mut ::windows_sys::core::GUID) -> i32;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
    pub fn SCardGetReaderDeviceInstanceIdA(hcontext: usize, szreadername: ::windows_sys::core::PCSTR, szdeviceinstanceid: ::windows_sys::core::PSTR, pcchdeviceinstanceid: *mut u32) -> i32;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
    pub fn SCardGetReaderDeviceInstanceIdW(hcontext: usize, szreadername: ::windows_sys::core::PCWSTR, szdeviceinstanceid: ::windows_sys::core::PWSTR, pcchdeviceinstanceid: *mut u32) -> i32;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
    pub fn SCardGetReaderIconA(hcontext: usize, szreadername: ::windows_sys::core::PCSTR, pbicon: *mut u8, pcbicon: *mut u32) -> i32;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
    pub fn SCardGetReaderIconW(hcontext: usize, szreadername: ::windows_sys::core::PCWSTR, pbicon: *mut u8, pcbicon: *mut u32) -> i32;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
    pub fn SCardGetStatusChangeA(hcontext: usize, dwtimeout: u32, rgreaderstates: *mut SCARD_READERSTATEA, creaders: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
    pub fn SCardGetStatusChangeW(hcontext: usize, dwtimeout: u32, rgreaderstates: *mut SCARD_READERSTATEW, creaders: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
    pub fn SCardGetTransmitCount(hcard: usize, pctransmitcount: *mut u32) -> i32;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
    pub fn SCardIntroduceCardTypeA(hcontext: usize, szcardname: ::windows_sys::core::PCSTR, pguidprimaryprovider: *const ::windows_sys::core::GUID, rgguidinterfaces: *const ::windows_sys::core::GUID, dwinterfacecount: u32, pbatr: *const u8, pbatrmask: *const u8, cbatrlen: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
    pub fn SCardIntroduceCardTypeW(hcontext: usize, szcardname: ::windows_sys::core::PCWSTR, pguidprimaryprovider: *const ::windows_sys::core::GUID, rgguidinterfaces: *const ::windows_sys::core::GUID, dwinterfacecount: u32, pbatr: *const u8, pbatrmask: *const u8, cbatrlen: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
    pub fn SCardIntroduceReaderA(hcontext: usize, szreadername: ::windows_sys::core::PCSTR, szdevicename: ::windows_sys::core::PCSTR) -> i32;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
    pub fn SCardIntroduceReaderGroupA(hcontext: usize, szgroupname: ::windows_sys::core::PCSTR) -> i32;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
    pub fn SCardIntroduceReaderGroupW(hcontext: usize, szgroupname: ::windows_sys::core::PCWSTR) -> i32;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
    pub fn SCardIntroduceReaderW(hcontext: usize, szreadername: ::windows_sys::core::PCWSTR, szdevicename: ::windows_sys::core::PCWSTR) -> i32;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
    pub fn SCardIsValidContext(hcontext: usize) -> i32;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
    pub fn SCardListCardsA(hcontext: usize, pbatr: *const u8, rgquidinterfaces: *const ::windows_sys::core::GUID, cguidinterfacecount: u32, mszcards: ::windows_sys::core::PSTR, pcchcards: *mut u32) -> i32;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
    pub fn SCardListCardsW(hcontext: usize, pbatr: *const u8, rgquidinterfaces: *const ::windows_sys::core::GUID, cguidinterfacecount: u32, mszcards: ::windows_sys::core::PWSTR, pcchcards: *mut u32) -> i32;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
    pub fn SCardListInterfacesA(hcontext: usize, szcard: ::windows_sys::core::PCSTR, pguidinterfaces: *mut ::windows_sys::core::GUID, pcguidinterfaces: *mut u32) -> i32;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
    pub fn SCardListInterfacesW(hcontext: usize, szcard: ::windows_sys::core::PCWSTR, pguidinterfaces: *mut ::windows_sys::core::GUID, pcguidinterfaces: *mut u32) -> i32;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
    pub fn SCardListReaderGroupsA(hcontext: usize, mszgroups: ::windows_sys::core::PSTR, pcchgroups: *mut u32) -> i32;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
    pub fn SCardListReaderGroupsW(hcontext: usize, mszgroups: ::windows_sys::core::PWSTR, pcchgroups: *mut u32) -> i32;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
    pub fn SCardListReadersA(hcontext: usize, mszgroups: ::windows_sys::core::PCSTR, mszreaders: ::windows_sys::core::PSTR, pcchreaders: *mut u32) -> i32;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
    pub fn SCardListReadersW(hcontext: usize, mszgroups: ::windows_sys::core::PCWSTR, mszreaders: ::windows_sys::core::PWSTR, pcchreaders: *mut u32) -> i32;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
    pub fn SCardListReadersWithDeviceInstanceIdA(hcontext: usize, szdeviceinstanceid: ::windows_sys::core::PCSTR, mszreaders: ::windows_sys::core::PSTR, pcchreaders: *mut u32) -> i32;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
    pub fn SCardListReadersWithDeviceInstanceIdW(hcontext: usize, szdeviceinstanceid: ::windows_sys::core::PCWSTR, mszreaders: ::windows_sys::core::PWSTR, pcchreaders: *mut u32) -> i32;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
    pub fn SCardLocateCardsA(hcontext: usize, mszcards: ::windows_sys::core::PCSTR, rgreaderstates: *mut SCARD_READERSTATEA, creaders: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
    pub fn SCardLocateCardsByATRA(hcontext: usize, rgatrmasks: *const SCARD_ATRMASK, catrs: u32, rgreaderstates: *mut SCARD_READERSTATEA, creaders: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
    pub fn SCardLocateCardsByATRW(hcontext: usize, rgatrmasks: *const SCARD_ATRMASK, catrs: u32, rgreaderstates: *mut SCARD_READERSTATEW, creaders: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
    pub fn SCardLocateCardsW(hcontext: usize, mszcards: ::windows_sys::core::PCWSTR, rgreaderstates: *mut SCARD_READERSTATEW, creaders: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
    pub fn SCardReadCacheA(hcontext: usize, cardidentifier: *const ::windows_sys::core::GUID, freshnesscounter: u32, lookupname: ::windows_sys::core::PCSTR, data: *mut u8, datalen: *mut u32) -> i32;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
    pub fn SCardReadCacheW(hcontext: usize, cardidentifier: *const ::windows_sys::core::GUID, freshnesscounter: u32, lookupname: ::windows_sys::core::PCWSTR, data: *mut u8, datalen: *mut u32) -> i32;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
    pub fn SCardReconnect(hcard: usize, dwsharemode: u32, dwpreferredprotocols: u32, dwinitialization: u32, pdwactiveprotocol: *mut u32) -> i32;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
    pub fn SCardReleaseContext(hcontext: usize) -> i32;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
    pub fn SCardReleaseStartedEvent();
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
    pub fn SCardRemoveReaderFromGroupA(hcontext: usize, szreadername: ::windows_sys::core::PCSTR, szgroupname: ::windows_sys::core::PCSTR) -> i32;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
    pub fn SCardRemoveReaderFromGroupW(hcontext: usize, szreadername: ::windows_sys::core::PCWSTR, szgroupname: ::windows_sys::core::PCWSTR) -> i32;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
    pub fn SCardSetAttrib(hcard: usize, dwattrid: u32, pbattr: *const u8, cbattrlen: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
    pub fn SCardSetCardTypeProviderNameA(hcontext: usize, szcardname: ::windows_sys::core::PCSTR, dwproviderid: u32, szprovider: ::windows_sys::core::PCSTR) -> i32;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
    pub fn SCardSetCardTypeProviderNameW(hcontext: usize, szcardname: ::windows_sys::core::PCWSTR, dwproviderid: u32, szprovider: ::windows_sys::core::PCWSTR) -> i32;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
    pub fn SCardState(hcard: usize, pdwstate: *mut u32, pdwprotocol: *mut u32, pbatr: *mut u8, pcbatrlen: *mut u32) -> i32;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
    pub fn SCardStatusA(hcard: usize, mszreadernames: ::windows_sys::core::PSTR, pcchreaderlen: *mut u32, pdwstate: *mut u32, pdwprotocol: *mut u32, pbatr: *mut u8, pcbatrlen: *mut u32) -> i32;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
    pub fn SCardStatusW(hcard: usize, mszreadernames: ::windows_sys::core::PWSTR, pcchreaderlen: *mut u32, pdwstate: *mut u32, pdwprotocol: *mut u32, pbatr: *mut u8, pcbatrlen: *mut u32) -> i32;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
    pub fn SCardTransmit(hcard: usize, piosendpci: *const SCARD_IO_REQUEST, pbsendbuffer: *const u8, cbsendlength: u32, piorecvpci: *mut SCARD_IO_REQUEST, pbrecvbuffer: *mut u8, pcbrecvlength: *mut u32) -> i32;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn SCardUIDlgSelectCardA(param0: *mut OPENCARDNAME_EXA) -> i32;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn SCardUIDlgSelectCardW(param0: *mut OPENCARDNAME_EXW) -> i32;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
    pub fn SCardWriteCacheA(hcontext: usize, cardidentifier: *const ::windows_sys::core::GUID, freshnesscounter: u32, lookupname: ::windows_sys::core::PCSTR, data: *const u8, datalen: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
    pub fn SCardWriteCacheW(hcontext: usize, cardidentifier: *const ::windows_sys::core::GUID, freshnesscounter: u32, lookupname: ::windows_sys::core::PCWSTR, data: *const u8, datalen: u32) -> i32;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub struct BINARY_BLOB_CREDENTIAL_INFO {
    pub cbBlob: u32,
    pub pbBlob: *mut u8,
}
impl ::core::marker::Copy for BINARY_BLOB_CREDENTIAL_INFO {}
impl ::core::clone::Clone for BINARY_BLOB_CREDENTIAL_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub struct CERT_CREDENTIAL_INFO {
    pub cbSize: u32,
    pub rgbHashOfCert: [u8; 20],
}
impl ::core::marker::Copy for CERT_CREDENTIAL_INFO {}
impl ::core::clone::Clone for CERT_CREDENTIAL_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CERT_HASH_LENGTH: u32 = 20u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CREDENTIALA {
    pub Flags: CRED_FLAGS,
    pub Type: CRED_TYPE,
    pub TargetName: ::windows_sys::core::PSTR,
    pub Comment: ::windows_sys::core::PSTR,
    pub LastWritten: super::super::Foundation::FILETIME,
    pub CredentialBlobSize: u32,
    pub CredentialBlob: *mut u8,
    pub Persist: CRED_PERSIST,
    pub AttributeCount: u32,
    pub Attributes: *mut CREDENTIAL_ATTRIBUTEA,
    pub TargetAlias: ::windows_sys::core::PSTR,
    pub UserName: ::windows_sys::core::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CREDENTIALA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CREDENTIALA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CREDENTIALW {
    pub Flags: CRED_FLAGS,
    pub Type: CRED_TYPE,
    pub TargetName: ::windows_sys::core::PWSTR,
    pub Comment: ::windows_sys::core::PWSTR,
    pub LastWritten: super::super::Foundation::FILETIME,
    pub CredentialBlobSize: u32,
    pub CredentialBlob: *mut u8,
    pub Persist: CRED_PERSIST,
    pub AttributeCount: u32,
    pub Attributes: *mut CREDENTIAL_ATTRIBUTEW,
    pub TargetAlias: ::windows_sys::core::PWSTR,
    pub UserName: ::windows_sys::core::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CREDENTIALW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CREDENTIALW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub struct CREDENTIAL_ATTRIBUTEA {
    pub Keyword: ::windows_sys::core::PSTR,
    pub Flags: u32,
    pub ValueSize: u32,
    pub Value: *mut u8,
}
impl ::core::marker::Copy for CREDENTIAL_ATTRIBUTEA {}
impl ::core::clone::Clone for CREDENTIAL_ATTRIBUTEA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub struct CREDENTIAL_ATTRIBUTEW {
    pub Keyword: ::windows_sys::core::PWSTR,
    pub Flags: u32,
    pub ValueSize: u32,
    pub Value: *mut u8,
}
impl ::core::marker::Copy for CREDENTIAL_ATTRIBUTEW {}
impl ::core::clone::Clone for CREDENTIAL_ATTRIBUTEW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub struct CREDENTIAL_TARGET_INFORMATIONA {
    pub TargetName: ::windows_sys::core::PSTR,
    pub NetbiosServerName: ::windows_sys::core::PSTR,
    pub DnsServerName: ::windows_sys::core::PSTR,
    pub NetbiosDomainName: ::windows_sys::core::PSTR,
    pub DnsDomainName: ::windows_sys::core::PSTR,
    pub DnsTreeName: ::windows_sys::core::PSTR,
    pub PackageName: ::windows_sys::core::PSTR,
    pub Flags: u32,
    pub CredTypeCount: u32,
    pub CredTypes: *mut u32,
}
impl ::core::marker::Copy for CREDENTIAL_TARGET_INFORMATIONA {}
impl ::core::clone::Clone for CREDENTIAL_TARGET_INFORMATIONA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub struct CREDENTIAL_TARGET_INFORMATIONW {
    pub TargetName: ::windows_sys::core::PWSTR,
    pub NetbiosServerName: ::windows_sys::core::PWSTR,
    pub DnsServerName: ::windows_sys::core::PWSTR,
    pub NetbiosDomainName: ::windows_sys::core::PWSTR,
    pub DnsDomainName: ::windows_sys::core::PWSTR,
    pub DnsTreeName: ::windows_sys::core::PWSTR,
    pub PackageName: ::windows_sys::core::PWSTR,
    pub Flags: u32,
    pub CredTypeCount: u32,
    pub CredTypes: *mut u32,
}
impl ::core::marker::Copy for CREDENTIAL_TARGET_INFORMATIONW {}
impl ::core::clone::Clone for CREDENTIAL_TARGET_INFORMATIONW {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub type CREDSPP_SUBMIT_TYPE = i32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CredsspPasswordCreds: CREDSPP_SUBMIT_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CredsspSchannelCreds: CREDSPP_SUBMIT_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CredsspCertificateCreds: CREDSPP_SUBMIT_TYPE = 13i32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CredsspSubmitBufferBoth: CREDSPP_SUBMIT_TYPE = 50i32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CredsspSubmitBufferBothOld: CREDSPP_SUBMIT_TYPE = 51i32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CredsspCredEx: CREDSPP_SUBMIT_TYPE = 100i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub struct CREDSSP_CRED {
    pub Type: CREDSPP_SUBMIT_TYPE,
    pub pSchannelCred: *mut ::core::ffi::c_void,
    pub pSpnegoCred: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for CREDSSP_CRED {}
impl ::core::clone::Clone for CREDSSP_CRED {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub struct CREDSSP_CRED_EX {
    pub Type: CREDSPP_SUBMIT_TYPE,
    pub Version: u32,
    pub Flags: u32,
    pub Reserved: u32,
    pub Cred: CREDSSP_CRED,
}
impl ::core::marker::Copy for CREDSSP_CRED_EX {}
impl ::core::clone::Clone for CREDSSP_CRED_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CREDSSP_CRED_EX_VERSION: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CREDSSP_FLAG_REDIRECT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CREDSSP_NAME: &str = "CREDSSP";
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CREDSSP_SERVER_AUTH_CERTIFICATE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CREDSSP_SERVER_AUTH_LOOPBACK: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CREDSSP_SERVER_AUTH_NEGOTIATE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CREDUIWIN_DOWNLEVEL_HELLO_AS_SMART_CARD: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub type CREDUIWIN_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CREDUIWIN_GENERIC: CREDUIWIN_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CREDUIWIN_CHECKBOX: CREDUIWIN_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CREDUIWIN_AUTHPACKAGE_ONLY: CREDUIWIN_FLAGS = 16u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CREDUIWIN_IN_CRED_ONLY: CREDUIWIN_FLAGS = 32u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CREDUIWIN_ENUMERATE_ADMINS: CREDUIWIN_FLAGS = 256u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CREDUIWIN_ENUMERATE_CURRENT_USER: CREDUIWIN_FLAGS = 512u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CREDUIWIN_SECURE_PROMPT: CREDUIWIN_FLAGS = 4096u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CREDUIWIN_PREPROMPTING: CREDUIWIN_FLAGS = 8192u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CREDUIWIN_PACK_32_WOW: CREDUIWIN_FLAGS = 268435456u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CREDUIWIN_IGNORE_CLOUDAUTHORITY_NAME: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub type CREDUI_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CREDUI_FLAGS_ALWAYS_SHOW_UI: CREDUI_FLAGS = 128u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CREDUI_FLAGS_COMPLETE_USERNAME: CREDUI_FLAGS = 2048u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CREDUI_FLAGS_DO_NOT_PERSIST: CREDUI_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CREDUI_FLAGS_EXCLUDE_CERTIFICATES: CREDUI_FLAGS = 8u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CREDUI_FLAGS_EXPECT_CONFIRMATION: CREDUI_FLAGS = 131072u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CREDUI_FLAGS_GENERIC_CREDENTIALS: CREDUI_FLAGS = 262144u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CREDUI_FLAGS_INCORRECT_PASSWORD: CREDUI_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CREDUI_FLAGS_KEEP_USERNAME: CREDUI_FLAGS = 1048576u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CREDUI_FLAGS_PASSWORD_ONLY_OK: CREDUI_FLAGS = 512u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CREDUI_FLAGS_PERSIST: CREDUI_FLAGS = 4096u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CREDUI_FLAGS_REQUEST_ADMINISTRATOR: CREDUI_FLAGS = 4u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CREDUI_FLAGS_REQUIRE_CERTIFICATE: CREDUI_FLAGS = 16u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CREDUI_FLAGS_REQUIRE_SMARTCARD: CREDUI_FLAGS = 256u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CREDUI_FLAGS_SERVER_CREDENTIAL: CREDUI_FLAGS = 16384u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CREDUI_FLAGS_SHOW_SAVE_CHECK_BOX: CREDUI_FLAGS = 64u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CREDUI_FLAGS_USERNAME_TARGET_CREDENTIALS: CREDUI_FLAGS = 524288u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CREDUI_FLAGS_VALIDATE_USERNAME: CREDUI_FLAGS = 1024u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct CREDUI_INFOA {
    pub cbSize: u32,
    pub hwndParent: super::super::Foundation::HWND,
    pub pszMessageText: ::windows_sys::core::PCSTR,
    pub pszCaptionText: ::windows_sys::core::PCSTR,
    pub hbmBanner: super::super::Graphics::Gdi::HBITMAP,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for CREDUI_INFOA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for CREDUI_INFOA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct CREDUI_INFOW {
    pub cbSize: u32,
    pub hwndParent: super::super::Foundation::HWND,
    pub pszMessageText: ::windows_sys::core::PCWSTR,
    pub pszCaptionText: ::windows_sys::core::PCWSTR,
    pub hbmBanner: super::super::Graphics::Gdi::HBITMAP,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for CREDUI_INFOW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for CREDUI_INFOW {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CREDUI_MAX_CAPTION_LENGTH: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CREDUI_MAX_GENERIC_TARGET_LENGTH: u32 = 32767u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CREDUI_MAX_MESSAGE_LENGTH: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_ALLOW_NAME_RESOLUTION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_CACHE_TARGET_INFORMATION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub type CRED_ENUMERATE_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_ENUMERATE_ALL_CREDENTIALS: CRED_ENUMERATE_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub type CRED_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_FLAGS_PASSWORD_FOR_CERT: CRED_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_FLAGS_PROMPT_NOW: CRED_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_FLAGS_USERNAME_TARGET: CRED_FLAGS = 4u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_FLAGS_OWF_CRED_BLOB: CRED_FLAGS = 8u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_FLAGS_REQUIRE_CONFIRMATION: CRED_FLAGS = 16u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_FLAGS_WILDCARD_MATCH: CRED_FLAGS = 32u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_FLAGS_VSM_PROTECTED: CRED_FLAGS = 64u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_FLAGS_NGC_CERT: CRED_FLAGS = 128u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_FLAGS_VALID_FLAGS: CRED_FLAGS = 61695u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_FLAGS_VALID_INPUT_FLAGS: CRED_FLAGS = 61599u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_LOGON_TYPES_MASK: u32 = 61440u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub type CRED_MARSHAL_TYPE = i32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CertCredential: CRED_MARSHAL_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const UsernameTargetCredential: CRED_MARSHAL_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const BinaryBlobCredential: CRED_MARSHAL_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const UsernameForPackedCredentials: CRED_MARSHAL_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const BinaryBlobForSystem: CRED_MARSHAL_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_MAX_ATTRIBUTES: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_MAX_GENERIC_TARGET_NAME_LENGTH: u32 = 32767u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_MAX_STRING_LENGTH: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_MAX_TARGETNAME_ATTRIBUTE_LENGTH: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_MAX_TARGETNAME_NAMESPACE_LENGTH: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_MAX_VALUE_SIZE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub type CRED_PACK_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_PACK_PROTECTED_CREDENTIALS: CRED_PACK_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_PACK_WOW_BUFFER: CRED_PACK_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_PACK_GENERIC_CREDENTIALS: CRED_PACK_FLAGS = 4u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_PACK_ID_PROVIDER_CREDENTIALS: CRED_PACK_FLAGS = 8u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub type CRED_PERSIST = u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_PERSIST_NONE: CRED_PERSIST = 0u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_PERSIST_SESSION: CRED_PERSIST = 1u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_PERSIST_LOCAL_MACHINE: CRED_PERSIST = 2u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_PERSIST_ENTERPRISE: CRED_PERSIST = 3u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_PRESERVE_CREDENTIAL_BLOB: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub type CRED_PROTECTION_TYPE = i32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CredUnprotected: CRED_PROTECTION_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CredUserProtection: CRED_PROTECTION_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CredTrustedProtection: CRED_PROTECTION_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CredForSystemProtection: CRED_PROTECTION_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_PROTECT_AS_SELF: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_PROTECT_TO_SYSTEM: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_SESSION_WILDCARD_NAME: &str = "*Session";
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_SESSION_WILDCARD_NAME_A: &str = "*Session";
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_SESSION_WILDCARD_NAME_W: &str = "*Session";
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_TARGETNAME_ATTRIBUTE_BATCH: &str = "batch";
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_TARGETNAME_ATTRIBUTE_BATCH_A: &str = "batch";
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_TARGETNAME_ATTRIBUTE_BATCH_W: &str = "batch";
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_TARGETNAME_ATTRIBUTE_CACHEDINTERACTIVE: &str = "cachedinteractive";
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_TARGETNAME_ATTRIBUTE_CACHEDINTERACTIVE_A: &str = "cachedinteractive";
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_TARGETNAME_ATTRIBUTE_CACHEDINTERACTIVE_W: &str = "cachedinteractive";
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_TARGETNAME_ATTRIBUTE_INTERACTIVE: &str = "interactive";
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_TARGETNAME_ATTRIBUTE_INTERACTIVE_A: &str = "interactive";
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_TARGETNAME_ATTRIBUTE_INTERACTIVE_W: &str = "interactive";
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_TARGETNAME_ATTRIBUTE_NAME: &str = "name";
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_TARGETNAME_ATTRIBUTE_NAME_A: &str = "name";
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_TARGETNAME_ATTRIBUTE_NAME_W: &str = "name";
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_TARGETNAME_ATTRIBUTE_NETWORK: &str = "network";
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_TARGETNAME_ATTRIBUTE_NETWORKCLEARTEXT: &str = "networkcleartext";
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_TARGETNAME_ATTRIBUTE_NETWORKCLEARTEXT_A: &str = "networkcleartext";
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_TARGETNAME_ATTRIBUTE_NETWORKCLEARTEXT_W: &str = "networkcleartext";
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_TARGETNAME_ATTRIBUTE_NETWORK_A: &str = "network";
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_TARGETNAME_ATTRIBUTE_NETWORK_W: &str = "network";
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_TARGETNAME_ATTRIBUTE_REMOTEINTERACTIVE: &str = "remoteinteractive";
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_TARGETNAME_ATTRIBUTE_REMOTEINTERACTIVE_A: &str = "remoteinteractive";
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_TARGETNAME_ATTRIBUTE_REMOTEINTERACTIVE_W: &str = "remoteinteractive";
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_TARGETNAME_ATTRIBUTE_SERVICE: &str = "service";
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_TARGETNAME_ATTRIBUTE_SERVICE_A: &str = "service";
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_TARGETNAME_ATTRIBUTE_SERVICE_W: &str = "service";
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_TARGETNAME_ATTRIBUTE_TARGET: &str = "target";
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_TARGETNAME_ATTRIBUTE_TARGET_A: &str = "target";
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_TARGETNAME_ATTRIBUTE_TARGET_W: &str = "target";
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_TARGETNAME_DOMAIN_NAMESPACE: &str = "Domain";
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_TARGETNAME_DOMAIN_NAMESPACE_A: &str = "Domain";
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_TARGETNAME_DOMAIN_NAMESPACE_W: &str = "Domain";
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_TARGETNAME_LEGACYGENERIC_NAMESPACE_A: &str = "LegacyGeneric";
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_TARGETNAME_LEGACYGENERIC_NAMESPACE_W: &str = "LegacyGeneric";
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_TI_CREATE_EXPLICIT_CRED: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_TI_DNSTREE_IS_DFS_SERVER: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_TI_DOMAIN_FORMAT_UNKNOWN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_TI_ONLY_PASSWORD_REQUIRED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_TI_SERVER_FORMAT_UNKNOWN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_TI_USERNAME_TARGET: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_TI_VALID_FLAGS: u32 = 61567u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_TI_WORKGROUP_MEMBER: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub type CRED_TYPE = u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_TYPE_GENERIC: CRED_TYPE = 1u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_TYPE_DOMAIN_PASSWORD: CRED_TYPE = 2u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_TYPE_DOMAIN_CERTIFICATE: CRED_TYPE = 3u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_TYPE_DOMAIN_VISIBLE_PASSWORD: CRED_TYPE = 4u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_TYPE_GENERIC_CERTIFICATE: CRED_TYPE = 5u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_TYPE_DOMAIN_EXTENDED: CRED_TYPE = 6u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_TYPE_MAXIMUM: CRED_TYPE = 7u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_TYPE_MAXIMUM_EX: CRED_TYPE = 1007u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_UNPROTECT_ALLOW_TO_SYSTEM: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_UNPROTECT_AS_SELF: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const FILE_DEVICE_SMARTCARD: u32 = 49u32;
pub const GUID_DEVINTERFACE_SMARTCARD_READER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1356681776, data2: 47754, data3: 4561, data4: [191, 93, 0, 0, 248, 5, 245, 48] };
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub struct KeyCredentialManagerInfo {
    pub containerId: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for KeyCredentialManagerInfo {}
impl ::core::clone::Clone for KeyCredentialManagerInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub type KeyCredentialManagerOperationErrorStates = u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const KeyCredentialManagerOperationErrorStateNone: KeyCredentialManagerOperationErrorStates = 0u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const KeyCredentialManagerOperationErrorStateDeviceJoinFailure: KeyCredentialManagerOperationErrorStates = 1u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const KeyCredentialManagerOperationErrorStateTokenFailure: KeyCredentialManagerOperationErrorStates = 2u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const KeyCredentialManagerOperationErrorStateCertificateFailure: KeyCredentialManagerOperationErrorStates = 4u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const KeyCredentialManagerOperationErrorStateRemoteSessionFailure: KeyCredentialManagerOperationErrorStates = 8u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const KeyCredentialManagerOperationErrorStatePolicyFailure: KeyCredentialManagerOperationErrorStates = 16u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const KeyCredentialManagerOperationErrorStateHardwareFailure: KeyCredentialManagerOperationErrorStates = 32u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const KeyCredentialManagerOperationErrorStatePinExistsFailure: KeyCredentialManagerOperationErrorStates = 64u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub type KeyCredentialManagerOperationType = i32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const KeyCredentialManagerProvisioning: KeyCredentialManagerOperationType = 0i32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const KeyCredentialManagerPinChange: KeyCredentialManagerOperationType = 1i32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const KeyCredentialManagerPinReset: KeyCredentialManagerOperationType = 2i32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPOCNCHKPROC = ::core::option::Option<unsafe extern "system" fn(param0: usize, param1: usize, param2: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub type LPOCNCONNPROCA = ::core::option::Option<unsafe extern "system" fn(param0: usize, param1: ::windows_sys::core::PCSTR, param2: ::windows_sys::core::PCSTR, param3: *const ::core::ffi::c_void) -> usize>;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub type LPOCNCONNPROCW = ::core::option::Option<unsafe extern "system" fn(param0: usize, param1: ::windows_sys::core::PCWSTR, param2: ::windows_sys::core::PCWSTR, param3: *const ::core::ffi::c_void) -> usize>;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub type LPOCNDSCPROC = ::core::option::Option<unsafe extern "system" fn(param0: usize, param1: usize, param2: *const ::core::ffi::c_void)>;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const MAXIMUM_ATTR_STRING_LENGTH: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const MAXIMUM_SMARTCARD_READERS: u32 = 10u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct OPENCARDNAMEA {
    pub dwStructSize: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub hSCardContext: usize,
    pub lpstrGroupNames: ::windows_sys::core::PSTR,
    pub nMaxGroupNames: u32,
    pub lpstrCardNames: ::windows_sys::core::PSTR,
    pub nMaxCardNames: u32,
    pub rgguidInterfaces: *const ::windows_sys::core::GUID,
    pub cguidInterfaces: u32,
    pub lpstrRdr: ::windows_sys::core::PSTR,
    pub nMaxRdr: u32,
    pub lpstrCard: ::windows_sys::core::PSTR,
    pub nMaxCard: u32,
    pub lpstrTitle: ::windows_sys::core::PCSTR,
    pub dwFlags: u32,
    pub pvUserData: *mut ::core::ffi::c_void,
    pub dwShareMode: u32,
    pub dwPreferredProtocols: u32,
    pub dwActiveProtocol: u32,
    pub lpfnConnect: LPOCNCONNPROCA,
    pub lpfnCheck: LPOCNCHKPROC,
    pub lpfnDisconnect: LPOCNDSCPROC,
    pub hCardHandle: usize,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OPENCARDNAMEA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OPENCARDNAMEA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct OPENCARDNAMEW {
    pub dwStructSize: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub hSCardContext: usize,
    pub lpstrGroupNames: ::windows_sys::core::PWSTR,
    pub nMaxGroupNames: u32,
    pub lpstrCardNames: ::windows_sys::core::PWSTR,
    pub nMaxCardNames: u32,
    pub rgguidInterfaces: *const ::windows_sys::core::GUID,
    pub cguidInterfaces: u32,
    pub lpstrRdr: ::windows_sys::core::PWSTR,
    pub nMaxRdr: u32,
    pub lpstrCard: ::windows_sys::core::PWSTR,
    pub nMaxCard: u32,
    pub lpstrTitle: ::windows_sys::core::PCWSTR,
    pub dwFlags: u32,
    pub pvUserData: *mut ::core::ffi::c_void,
    pub dwShareMode: u32,
    pub dwPreferredProtocols: u32,
    pub dwActiveProtocol: u32,
    pub lpfnConnect: LPOCNCONNPROCW,
    pub lpfnCheck: LPOCNCHKPROC,
    pub lpfnDisconnect: LPOCNDSCPROC,
    pub hCardHandle: usize,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OPENCARDNAMEW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OPENCARDNAMEW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct OPENCARDNAME_EXA {
    pub dwStructSize: u32,
    pub hSCardContext: usize,
    pub hwndOwner: super::super::Foundation::HWND,
    pub dwFlags: u32,
    pub lpstrTitle: ::windows_sys::core::PCSTR,
    pub lpstrSearchDesc: ::windows_sys::core::PCSTR,
    pub hIcon: super::super::UI::WindowsAndMessaging::HICON,
    pub pOpenCardSearchCriteria: *mut OPENCARD_SEARCH_CRITERIAA,
    pub lpfnConnect: LPOCNCONNPROCA,
    pub pvUserData: *mut ::core::ffi::c_void,
    pub dwShareMode: u32,
    pub dwPreferredProtocols: u32,
    pub lpstrRdr: ::windows_sys::core::PSTR,
    pub nMaxRdr: u32,
    pub lpstrCard: ::windows_sys::core::PSTR,
    pub nMaxCard: u32,
    pub dwActiveProtocol: u32,
    pub hCardHandle: usize,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for OPENCARDNAME_EXA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for OPENCARDNAME_EXA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct OPENCARDNAME_EXW {
    pub dwStructSize: u32,
    pub hSCardContext: usize,
    pub hwndOwner: super::super::Foundation::HWND,
    pub dwFlags: u32,
    pub lpstrTitle: ::windows_sys::core::PCWSTR,
    pub lpstrSearchDesc: ::windows_sys::core::PCWSTR,
    pub hIcon: super::super::UI::WindowsAndMessaging::HICON,
    pub pOpenCardSearchCriteria: *mut OPENCARD_SEARCH_CRITERIAW,
    pub lpfnConnect: LPOCNCONNPROCW,
    pub pvUserData: *mut ::core::ffi::c_void,
    pub dwShareMode: u32,
    pub dwPreferredProtocols: u32,
    pub lpstrRdr: ::windows_sys::core::PWSTR,
    pub nMaxRdr: u32,
    pub lpstrCard: ::windows_sys::core::PWSTR,
    pub nMaxCard: u32,
    pub dwActiveProtocol: u32,
    pub hCardHandle: usize,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for OPENCARDNAME_EXW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for OPENCARDNAME_EXW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct OPENCARD_SEARCH_CRITERIAA {
    pub dwStructSize: u32,
    pub lpstrGroupNames: ::windows_sys::core::PSTR,
    pub nMaxGroupNames: u32,
    pub rgguidInterfaces: *const ::windows_sys::core::GUID,
    pub cguidInterfaces: u32,
    pub lpstrCardNames: ::windows_sys::core::PSTR,
    pub nMaxCardNames: u32,
    pub lpfnCheck: LPOCNCHKPROC,
    pub lpfnConnect: LPOCNCONNPROCA,
    pub lpfnDisconnect: LPOCNDSCPROC,
    pub pvUserData: *mut ::core::ffi::c_void,
    pub dwShareMode: u32,
    pub dwPreferredProtocols: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OPENCARD_SEARCH_CRITERIAA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OPENCARD_SEARCH_CRITERIAA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct OPENCARD_SEARCH_CRITERIAW {
    pub dwStructSize: u32,
    pub lpstrGroupNames: ::windows_sys::core::PWSTR,
    pub nMaxGroupNames: u32,
    pub rgguidInterfaces: *const ::windows_sys::core::GUID,
    pub cguidInterfaces: u32,
    pub lpstrCardNames: ::windows_sys::core::PWSTR,
    pub nMaxCardNames: u32,
    pub lpfnCheck: LPOCNCHKPROC,
    pub lpfnConnect: LPOCNCONNPROCW,
    pub lpfnDisconnect: LPOCNDSCPROC,
    pub pvUserData: *mut ::core::ffi::c_void,
    pub dwShareMode: u32,
    pub dwPreferredProtocols: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OPENCARD_SEARCH_CRITERIAW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OPENCARD_SEARCH_CRITERIAW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub struct READER_SEL_REQUEST {
    pub dwShareMode: u32,
    pub dwPreferredProtocols: u32,
    pub MatchType: READER_SEL_REQUEST_MATCH_TYPE,
    pub Anonymous: READER_SEL_REQUEST_0,
}
impl ::core::marker::Copy for READER_SEL_REQUEST {}
impl ::core::clone::Clone for READER_SEL_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub union READER_SEL_REQUEST_0 {
    pub ReaderAndContainerParameter: READER_SEL_REQUEST_0_0,
    pub SerialNumberParameter: READER_SEL_REQUEST_0_1,
}
impl ::core::marker::Copy for READER_SEL_REQUEST_0 {}
impl ::core::clone::Clone for READER_SEL_REQUEST_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub struct READER_SEL_REQUEST_0_0 {
    pub cbReaderNameOffset: u32,
    pub cchReaderNameLength: u32,
    pub cbContainerNameOffset: u32,
    pub cchContainerNameLength: u32,
    pub dwDesiredCardModuleVersion: u32,
    pub dwCspFlags: u32,
}
impl ::core::marker::Copy for READER_SEL_REQUEST_0_0 {}
impl ::core::clone::Clone for READER_SEL_REQUEST_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub struct READER_SEL_REQUEST_0_1 {
    pub cbSerialNumberOffset: u32,
    pub cbSerialNumberLength: u32,
    pub dwDesiredCardModuleVersion: u32,
}
impl ::core::marker::Copy for READER_SEL_REQUEST_0_1 {}
impl ::core::clone::Clone for READER_SEL_REQUEST_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub type READER_SEL_REQUEST_MATCH_TYPE = i32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const RSR_MATCH_TYPE_READER_AND_CONTAINER: READER_SEL_REQUEST_MATCH_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const RSR_MATCH_TYPE_SERIAL_NUMBER: READER_SEL_REQUEST_MATCH_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const RSR_MATCH_TYPE_ALL_CARDS: READER_SEL_REQUEST_MATCH_TYPE = 3i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub struct READER_SEL_RESPONSE {
    pub cbReaderNameOffset: u32,
    pub cchReaderNameLength: u32,
    pub cbCardNameOffset: u32,
    pub cchCardNameLength: u32,
}
impl ::core::marker::Copy for READER_SEL_RESPONSE {}
impl ::core::clone::Clone for READER_SEL_RESPONSE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_ABSENT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_ALL_READERS: &str = "SCard$AllReaders\u{0}00";
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub struct SCARD_ATRMASK {
    pub cbAtr: u32,
    pub rgbAtr: [u8; 36],
    pub rgbMask: [u8; 36],
}
impl ::core::marker::Copy for SCARD_ATRMASK {}
impl ::core::clone::Clone for SCARD_ATRMASK {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_ATR_LENGTH: u32 = 33u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_AUDIT_CHV_FAILURE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_AUDIT_CHV_SUCCESS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_CLASS_COMMUNICATIONS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_CLASS_ICC_STATE: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_CLASS_IFD_PROTOCOL: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_CLASS_MECHANICAL: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_CLASS_PERF: u32 = 32766u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_CLASS_POWER_MGMT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_CLASS_PROTOCOL: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_CLASS_SECURITY: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_CLASS_SYSTEM: u32 = 32767u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_CLASS_VENDOR_DEFINED: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_CLASS_VENDOR_INFO: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_COLD_RESET: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_DEFAULT_READERS: &str = "SCard$DefaultReaders\u{0}00";
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_EJECT_CARD: u32 = 3u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub struct SCARD_IO_REQUEST {
    pub dwProtocol: u32,
    pub cbPciLength: u32,
}
impl ::core::marker::Copy for SCARD_IO_REQUEST {}
impl ::core::clone::Clone for SCARD_IO_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_LEAVE_CARD: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_LOCAL_READERS: &str = "SCard$LocalReaders\u{0}00";
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_NEGOTIABLE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_POWERED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_POWER_DOWN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_PRESENT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_PROTOCOL_DEFAULT: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_PROTOCOL_OPTIMAL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_PROTOCOL_RAW: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_PROTOCOL_T0: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_PROTOCOL_T1: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_PROTOCOL_UNDEFINED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_PROVIDER_CSP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_PROVIDER_KSP: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_PROVIDER_PRIMARY: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub struct SCARD_READERSTATEA {
    pub szReader: ::windows_sys::core::PCSTR,
    pub pvUserData: *mut ::core::ffi::c_void,
    pub dwCurrentState: SCARD_STATE,
    pub dwEventState: SCARD_STATE,
    pub cbAtr: u32,
    pub rgbAtr: [u8; 36],
}
impl ::core::marker::Copy for SCARD_READERSTATEA {}
impl ::core::clone::Clone for SCARD_READERSTATEA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub struct SCARD_READERSTATEW {
    pub szReader: ::windows_sys::core::PCWSTR,
    pub pvUserData: *mut ::core::ffi::c_void,
    pub dwCurrentState: SCARD_STATE,
    pub dwEventState: SCARD_STATE,
    pub cbAtr: u32,
    pub rgbAtr: [u8; 36],
}
impl ::core::marker::Copy for SCARD_READERSTATEW {}
impl ::core::clone::Clone for SCARD_READERSTATEW {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_READER_CONFISCATES: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_READER_CONTACTLESS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_READER_EJECTS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_READER_SWALLOWS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_READER_TYPE_EMBEDDEDSE: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_READER_TYPE_IDE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_READER_TYPE_KEYBOARD: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_READER_TYPE_NFC: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_READER_TYPE_NGC: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_READER_TYPE_PARALELL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_READER_TYPE_PCMCIA: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_READER_TYPE_SCSI: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_READER_TYPE_SERIAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_READER_TYPE_TPM: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_READER_TYPE_UICC: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_READER_TYPE_USB: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_READER_TYPE_VENDOR: u32 = 240u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_RESET_CARD: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub type SCARD_SCOPE = u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_SCOPE_USER: SCARD_SCOPE = 0u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_SCOPE_SYSTEM: SCARD_SCOPE = 2u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_SCOPE_TERMINAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_SHARE_DIRECT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_SHARE_EXCLUSIVE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_SHARE_SHARED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_SPECIFIC: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub type SCARD_STATE = u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_STATE_UNAWARE: SCARD_STATE = 0u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_STATE_IGNORE: SCARD_STATE = 1u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_STATE_UNAVAILABLE: SCARD_STATE = 8u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_STATE_EMPTY: SCARD_STATE = 16u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_STATE_PRESENT: SCARD_STATE = 32u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_STATE_ATRMATCH: SCARD_STATE = 64u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_STATE_EXCLUSIVE: SCARD_STATE = 128u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_STATE_INUSE: SCARD_STATE = 256u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_STATE_MUTE: SCARD_STATE = 512u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_STATE_CHANGED: SCARD_STATE = 2u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_STATE_UNKNOWN: SCARD_STATE = 4u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_STATE_UNPOWERED: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_SWALLOWED: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_SYSTEM_READERS: &str = "SCard$SystemReaders\u{0}00";
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_T0_CMD_LENGTH: u32 = 5u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub struct SCARD_T0_COMMAND {
    pub bCla: u8,
    pub bIns: u8,
    pub bP1: u8,
    pub bP2: u8,
    pub bP3: u8,
}
impl ::core::marker::Copy for SCARD_T0_COMMAND {}
impl ::core::clone::Clone for SCARD_T0_COMMAND {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_T0_HEADER_LENGTH: u32 = 7u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub struct SCARD_T0_REQUEST {
    pub ioRequest: SCARD_IO_REQUEST,
    pub bSw1: u8,
    pub bSw2: u8,
    pub Anonymous: SCARD_T0_REQUEST_0,
}
impl ::core::marker::Copy for SCARD_T0_REQUEST {}
impl ::core::clone::Clone for SCARD_T0_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub union SCARD_T0_REQUEST_0 {
    pub CmdBytes: SCARD_T0_COMMAND,
    pub rgbHeader: [u8; 5],
}
impl ::core::marker::Copy for SCARD_T0_REQUEST_0 {}
impl ::core::clone::Clone for SCARD_T0_REQUEST_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_T1_EPILOGUE_LENGTH: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_T1_EPILOGUE_LENGTH_LRC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_T1_MAX_IFS: u32 = 254u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_T1_PROLOGUE_LENGTH: u32 = 3u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub struct SCARD_T1_REQUEST {
    pub ioRequest: SCARD_IO_REQUEST,
}
impl ::core::marker::Copy for SCARD_T1_REQUEST {}
impl ::core::clone::Clone for SCARD_T1_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_UNKNOWN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_UNPOWER_CARD: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_WARM_RESET: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCERR_NOCARDNAME: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCERR_NOGUIDS: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SC_DLG_FORCE_UI: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SC_DLG_MINIMAL_UI: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SC_DLG_NO_UI: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SECPKG_ALT_ATTR: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SECPKG_ATTR_C_FULL_IDENT_TOKEN: u32 = 2147483781u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const STATUS_ACCOUNT_DISABLED: super::super::Foundation::NTSTATUS = -1073741710i32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const STATUS_ACCOUNT_EXPIRED: super::super::Foundation::NTSTATUS = -1073741421i32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const STATUS_ACCOUNT_LOCKED_OUT: super::super::Foundation::NTSTATUS = -1073741260i32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const STATUS_ACCOUNT_RESTRICTION: super::super::Foundation::NTSTATUS = -1073741714i32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const STATUS_AUTHENTICATION_FIREWALL_FAILED: super::super::Foundation::NTSTATUS = -1073740781i32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const STATUS_DOWNGRADE_DETECTED: super::super::Foundation::NTSTATUS = -1073740920i32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const STATUS_LOGON_FAILURE: super::super::Foundation::NTSTATUS = -1073741715i32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const STATUS_LOGON_TYPE_NOT_GRANTED: super::super::Foundation::NTSTATUS = -1073741477i32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const STATUS_NO_SUCH_LOGON_SESSION: super::super::Foundation::NTSTATUS = -1073741729i32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const STATUS_NO_SUCH_USER: super::super::Foundation::NTSTATUS = -1073741724i32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const STATUS_PASSWORD_EXPIRED: super::super::Foundation::NTSTATUS = -1073741711i32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const STATUS_PASSWORD_MUST_CHANGE: super::super::Foundation::NTSTATUS = -1073741276i32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const STATUS_WRONG_PASSWORD: super::super::Foundation::NTSTATUS = -1073741718i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub struct SecHandle {
    pub dwLower: usize,
    pub dwUpper: usize,
}
impl ::core::marker::Copy for SecHandle {}
impl ::core::clone::Clone for SecHandle {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub struct SecPkgContext_ClientCreds {
    pub AuthBufferLen: u32,
    pub AuthBuffer: *mut u8,
}
impl ::core::marker::Copy for SecPkgContext_ClientCreds {}
impl ::core::clone::Clone for SecPkgContext_ClientCreds {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const TS_SSP_NAME: &str = "TSSSP";
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const TS_SSP_NAME_A: &str = "TSSSP";
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub struct USERNAME_TARGET_CREDENTIAL_INFO {
    pub UserName: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for USERNAME_TARGET_CREDENTIAL_INFO {}
impl ::core::clone::Clone for USERNAME_TARGET_CREDENTIAL_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const szOID_TS_KP_TS_SERVER_AUTH: &str = "1.3.6.1.4.1.311.54.1.2";
