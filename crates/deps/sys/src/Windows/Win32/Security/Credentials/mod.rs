#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredDeleteA(targetname: super::super::Foundation::PSTR, r#type: u32, flags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredDeleteW(targetname: super::super::Foundation::PWSTR, r#type: u32, flags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredEnumerateA(filter: super::super::Foundation::PSTR, flags: CRED_ENUMERATE_FLAGS, count: *mut u32, credential: *mut *mut *mut CREDENTIALA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredEnumerateW(filter: super::super::Foundation::PWSTR, flags: CRED_ENUMERATE_FLAGS, count: *mut u32, credential: *mut *mut *mut CREDENTIALW) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredFindBestCredentialA(targetname: super::super::Foundation::PSTR, r#type: u32, flags: u32, credential: *mut *mut CREDENTIALA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredFindBestCredentialW(targetname: super::super::Foundation::PWSTR, r#type: u32, flags: u32, credential: *mut *mut CREDENTIALW) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Credentials`*"]
    pub fn CredFree(buffer: *const ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredGetSessionTypes(maximumpersistcount: u32, maximumpersist: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredGetTargetInfoA(targetname: super::super::Foundation::PSTR, flags: u32, targetinfo: *mut *mut CREDENTIAL_TARGET_INFORMATIONA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredGetTargetInfoW(targetname: super::super::Foundation::PWSTR, flags: u32, targetinfo: *mut *mut CREDENTIAL_TARGET_INFORMATIONW) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredIsMarshaledCredentialA(marshaledcredential: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredIsMarshaledCredentialW(marshaledcredential: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredIsProtectedA(pszprotectedcredentials: super::super::Foundation::PSTR, pprotectiontype: *mut CRED_PROTECTION_TYPE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredIsProtectedW(pszprotectedcredentials: super::super::Foundation::PWSTR, pprotectiontype: *mut CRED_PROTECTION_TYPE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredMarshalCredentialA(credtype: CRED_MARSHAL_TYPE, credential: *const ::core::ffi::c_void, marshaledcredential: *mut super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredMarshalCredentialW(credtype: CRED_MARSHAL_TYPE, credential: *const ::core::ffi::c_void, marshaledcredential: *mut super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredPackAuthenticationBufferA(dwflags: CRED_PACK_FLAGS, pszusername: super::super::Foundation::PSTR, pszpassword: super::super::Foundation::PSTR, ppackedcredentials: *mut u8, pcbpackedcredentials: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredPackAuthenticationBufferW(dwflags: CRED_PACK_FLAGS, pszusername: super::super::Foundation::PWSTR, pszpassword: super::super::Foundation::PWSTR, ppackedcredentials: *mut u8, pcbpackedcredentials: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredProtectA(fasself: super::super::Foundation::BOOL, pszcredentials: super::super::Foundation::PSTR, cchcredentials: u32, pszprotectedcredentials: super::super::Foundation::PSTR, pcchmaxchars: *mut u32, protectiontype: *mut CRED_PROTECTION_TYPE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredProtectW(fasself: super::super::Foundation::BOOL, pszcredentials: super::super::Foundation::PWSTR, cchcredentials: u32, pszprotectedcredentials: super::super::Foundation::PWSTR, pcchmaxchars: *mut u32, protectiontype: *mut CRED_PROTECTION_TYPE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredReadA(targetname: super::super::Foundation::PSTR, r#type: u32, flags: u32, credential: *mut *mut CREDENTIALA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredReadDomainCredentialsA(targetinfo: *const CREDENTIAL_TARGET_INFORMATIONA, flags: u32, count: *mut u32, credential: *mut *mut *mut CREDENTIALA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredReadDomainCredentialsW(targetinfo: *const CREDENTIAL_TARGET_INFORMATIONW, flags: u32, count: *mut u32, credential: *mut *mut *mut CREDENTIALW) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredReadW(targetname: super::super::Foundation::PWSTR, r#type: u32, flags: u32, credential: *mut *mut CREDENTIALW) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredRenameA(oldtargetname: super::super::Foundation::PSTR, newtargetname: super::super::Foundation::PSTR, r#type: u32, flags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredRenameW(oldtargetname: super::super::Foundation::PWSTR, newtargetname: super::super::Foundation::PWSTR, r#type: u32, flags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredUICmdLinePromptForCredentialsA(psztargetname: super::super::Foundation::PSTR, pcontext: *mut SecHandle, dwautherror: u32, username: super::super::Foundation::PSTR, uluserbuffersize: u32, pszpassword: super::super::Foundation::PSTR, ulpasswordbuffersize: u32, pfsave: *mut super::super::Foundation::BOOL, dwflags: CREDUI_FLAGS) -> u32;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredUICmdLinePromptForCredentialsW(psztargetname: super::super::Foundation::PWSTR, pcontext: *mut SecHandle, dwautherror: u32, username: super::super::Foundation::PWSTR, uluserbuffersize: u32, pszpassword: super::super::Foundation::PWSTR, ulpasswordbuffersize: u32, pfsave: *mut super::super::Foundation::BOOL, dwflags: CREDUI_FLAGS) -> u32;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredUIConfirmCredentialsA(psztargetname: super::super::Foundation::PSTR, bconfirm: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredUIConfirmCredentialsW(psztargetname: super::super::Foundation::PWSTR, bconfirm: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredUIParseUserNameA(username: super::super::Foundation::PSTR, user: super::super::Foundation::PSTR, userbuffersize: u32, domain: super::super::Foundation::PSTR, domainbuffersize: u32) -> u32;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredUIParseUserNameW(username: super::super::Foundation::PWSTR, user: super::super::Foundation::PWSTR, userbuffersize: u32, domain: super::super::Foundation::PWSTR, domainbuffersize: u32) -> u32;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn CredUIPromptForCredentialsA(puiinfo: *const CREDUI_INFOA, psztargetname: super::super::Foundation::PSTR, pcontext: *mut SecHandle, dwautherror: u32, pszusername: super::super::Foundation::PSTR, ulusernamebuffersize: u32, pszpassword: super::super::Foundation::PSTR, ulpasswordbuffersize: u32, save: *mut super::super::Foundation::BOOL, dwflags: CREDUI_FLAGS) -> u32;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn CredUIPromptForCredentialsW(puiinfo: *const CREDUI_INFOW, psztargetname: super::super::Foundation::PWSTR, pcontext: *mut SecHandle, dwautherror: u32, pszusername: super::super::Foundation::PWSTR, ulusernamebuffersize: u32, pszpassword: super::super::Foundation::PWSTR, ulpasswordbuffersize: u32, save: *mut super::super::Foundation::BOOL, dwflags: CREDUI_FLAGS) -> u32;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn CredUIPromptForWindowsCredentialsA(puiinfo: *const CREDUI_INFOA, dwautherror: u32, pulauthpackage: *mut u32, pvinauthbuffer: *const ::core::ffi::c_void, ulinauthbuffersize: u32, ppvoutauthbuffer: *mut *mut ::core::ffi::c_void, puloutauthbuffersize: *mut u32, pfsave: *mut super::super::Foundation::BOOL, dwflags: CREDUIWIN_FLAGS) -> u32;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn CredUIPromptForWindowsCredentialsW(puiinfo: *const CREDUI_INFOW, dwautherror: u32, pulauthpackage: *mut u32, pvinauthbuffer: *const ::core::ffi::c_void, ulinauthbuffersize: u32, ppvoutauthbuffer: *mut *mut ::core::ffi::c_void, puloutauthbuffersize: *mut u32, pfsave: *mut super::super::Foundation::BOOL, dwflags: CREDUIWIN_FLAGS) -> u32;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredUIReadSSOCredW(pszrealm: super::super::Foundation::PWSTR, ppszusername: *mut super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredUIStoreSSOCredW(pszrealm: super::super::Foundation::PWSTR, pszusername: super::super::Foundation::PWSTR, pszpassword: super::super::Foundation::PWSTR, bpersist: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredUnPackAuthenticationBufferA(dwflags: CRED_PACK_FLAGS, pauthbuffer: *const ::core::ffi::c_void, cbauthbuffer: u32, pszusername: super::super::Foundation::PSTR, pcchlmaxusername: *mut u32, pszdomainname: super::super::Foundation::PSTR, pcchmaxdomainname: *mut u32, pszpassword: super::super::Foundation::PSTR, pcchmaxpassword: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredUnPackAuthenticationBufferW(dwflags: CRED_PACK_FLAGS, pauthbuffer: *const ::core::ffi::c_void, cbauthbuffer: u32, pszusername: super::super::Foundation::PWSTR, pcchmaxusername: *mut u32, pszdomainname: super::super::Foundation::PWSTR, pcchmaxdomainname: *mut u32, pszpassword: super::super::Foundation::PWSTR, pcchmaxpassword: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredUnmarshalCredentialA(marshaledcredential: super::super::Foundation::PSTR, credtype: *mut CRED_MARSHAL_TYPE, credential: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredUnmarshalCredentialW(marshaledcredential: super::super::Foundation::PWSTR, credtype: *mut CRED_MARSHAL_TYPE, credential: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredUnprotectA(fasself: super::super::Foundation::BOOL, pszprotectedcredentials: super::super::Foundation::PSTR, cchprotectedcredentials: u32, pszcredentials: super::super::Foundation::PSTR, pcchmaxchars: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredUnprotectW(fasself: super::super::Foundation::BOOL, pszprotectedcredentials: super::super::Foundation::PWSTR, cchprotectedcredentials: u32, pszcredentials: super::super::Foundation::PWSTR, pcchmaxchars: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredWriteA(credential: *const CREDENTIALA, flags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredWriteDomainCredentialsA(targetinfo: *const CREDENTIAL_TARGET_INFORMATIONA, credential: *const CREDENTIALA, flags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredWriteDomainCredentialsW(targetinfo: *const CREDENTIAL_TARGET_INFORMATIONW, credential: *const CREDENTIALW, flags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredWriteW(credential: *const CREDENTIALW, flags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetOpenCardNameA(param0: *mut ::core::mem::ManuallyDrop<OPENCARDNAMEA>) -> i32;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetOpenCardNameW(param0: *mut ::core::mem::ManuallyDrop<OPENCARDNAMEW>) -> i32;
    #[doc = "*Required features: `Win32_Security_Credentials`*"]
    pub fn KeyCredentialManagerFreeInformation(keycredentialmanagerinfo: *const KeyCredentialManagerInfo);
    #[doc = "*Required features: `Win32_Security_Credentials`*"]
    pub fn KeyCredentialManagerGetInformation(keycredentialmanagerinfo: *mut *mut KeyCredentialManagerInfo) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn KeyCredentialManagerGetOperationErrorStates(keycredentialmanageroperationtype: KeyCredentialManagerOperationType, isready: *mut super::super::Foundation::BOOL, keycredentialmanageroperationerrorstates: *mut KeyCredentialManagerOperationErrorStates) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn KeyCredentialManagerShowUIOperation(hwndowner: super::super::Foundation::HWND, keycredentialmanageroperationtype: KeyCredentialManagerOperationType) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardAccessStartedEvent() -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardAddReaderToGroupA(hcontext: usize, szreadername: super::super::Foundation::PSTR, szgroupname: super::super::Foundation::PSTR) -> i32;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardAddReaderToGroupW(hcontext: usize, szreadername: super::super::Foundation::PWSTR, szgroupname: super::super::Foundation::PWSTR) -> i32;
    #[doc = "*Required features: `Win32_Security_Credentials`*"]
    pub fn SCardAudit(hcontext: usize, dwevent: u32) -> i32;
    #[doc = "*Required features: `Win32_Security_Credentials`*"]
    pub fn SCardBeginTransaction(hcard: usize) -> i32;
    #[doc = "*Required features: `Win32_Security_Credentials`*"]
    pub fn SCardCancel(hcontext: usize) -> i32;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardConnectA(hcontext: usize, szreader: super::super::Foundation::PSTR, dwsharemode: u32, dwpreferredprotocols: u32, phcard: *mut usize, pdwactiveprotocol: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardConnectW(hcontext: usize, szreader: super::super::Foundation::PWSTR, dwsharemode: u32, dwpreferredprotocols: u32, phcard: *mut usize, pdwactiveprotocol: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Security_Credentials`*"]
    pub fn SCardControl(hcard: usize, dwcontrolcode: u32, lpinbuffer: *const ::core::ffi::c_void, cbinbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, cboutbuffersize: u32, lpbytesreturned: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Security_Credentials`*"]
    pub fn SCardDisconnect(hcard: usize, dwdisposition: u32) -> i32;
    #[doc = "*Required features: `Win32_Security_Credentials`*"]
    pub fn SCardDlgExtendedError() -> i32;
    #[doc = "*Required features: `Win32_Security_Credentials`*"]
    pub fn SCardEndTransaction(hcard: usize, dwdisposition: u32) -> i32;
    #[doc = "*Required features: `Win32_Security_Credentials`*"]
    pub fn SCardEstablishContext(dwscope: SCARD_SCOPE, pvreserved1: *const ::core::ffi::c_void, pvreserved2: *const ::core::ffi::c_void, phcontext: *mut usize) -> i32;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardForgetCardTypeA(hcontext: usize, szcardname: super::super::Foundation::PSTR) -> i32;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardForgetCardTypeW(hcontext: usize, szcardname: super::super::Foundation::PWSTR) -> i32;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardForgetReaderA(hcontext: usize, szreadername: super::super::Foundation::PSTR) -> i32;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardForgetReaderGroupA(hcontext: usize, szgroupname: super::super::Foundation::PSTR) -> i32;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardForgetReaderGroupW(hcontext: usize, szgroupname: super::super::Foundation::PWSTR) -> i32;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardForgetReaderW(hcontext: usize, szreadername: super::super::Foundation::PWSTR) -> i32;
    #[doc = "*Required features: `Win32_Security_Credentials`*"]
    pub fn SCardFreeMemory(hcontext: usize, pvmem: *const ::core::ffi::c_void) -> i32;
    #[doc = "*Required features: `Win32_Security_Credentials`*"]
    pub fn SCardGetAttrib(hcard: usize, dwattrid: u32, pbattr: *mut u8, pcbattrlen: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardGetCardTypeProviderNameA(hcontext: usize, szcardname: super::super::Foundation::PSTR, dwproviderid: u32, szprovider: super::super::Foundation::PSTR, pcchprovider: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardGetCardTypeProviderNameW(hcontext: usize, szcardname: super::super::Foundation::PWSTR, dwproviderid: u32, szprovider: super::super::Foundation::PWSTR, pcchprovider: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardGetDeviceTypeIdA(hcontext: usize, szreadername: super::super::Foundation::PSTR, pdwdevicetypeid: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardGetDeviceTypeIdW(hcontext: usize, szreadername: super::super::Foundation::PWSTR, pdwdevicetypeid: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardGetProviderIdA(hcontext: usize, szcard: super::super::Foundation::PSTR, pguidproviderid: *mut ::windows::runtime::GUID) -> i32;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardGetProviderIdW(hcontext: usize, szcard: super::super::Foundation::PWSTR, pguidproviderid: *mut ::windows::runtime::GUID) -> i32;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardGetReaderDeviceInstanceIdA(hcontext: usize, szreadername: super::super::Foundation::PSTR, szdeviceinstanceid: super::super::Foundation::PSTR, pcchdeviceinstanceid: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardGetReaderDeviceInstanceIdW(hcontext: usize, szreadername: super::super::Foundation::PWSTR, szdeviceinstanceid: super::super::Foundation::PWSTR, pcchdeviceinstanceid: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardGetReaderIconA(hcontext: usize, szreadername: super::super::Foundation::PSTR, pbicon: *mut u8, pcbicon: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardGetReaderIconW(hcontext: usize, szreadername: super::super::Foundation::PWSTR, pbicon: *mut u8, pcbicon: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardGetStatusChangeA(hcontext: usize, dwtimeout: u32, rgreaderstates: *mut SCARD_READERSTATEA, creaders: u32) -> i32;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardGetStatusChangeW(hcontext: usize, dwtimeout: u32, rgreaderstates: *mut SCARD_READERSTATEW, creaders: u32) -> i32;
    #[doc = "*Required features: `Win32_Security_Credentials`*"]
    pub fn SCardGetTransmitCount(hcard: usize, pctransmitcount: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardIntroduceCardTypeA(hcontext: usize, szcardname: super::super::Foundation::PSTR, pguidprimaryprovider: *const ::windows::runtime::GUID, rgguidinterfaces: *const ::windows::runtime::GUID, dwinterfacecount: u32, pbatr: *const u8, pbatrmask: *const u8, cbatrlen: u32) -> i32;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardIntroduceCardTypeW(hcontext: usize, szcardname: super::super::Foundation::PWSTR, pguidprimaryprovider: *const ::windows::runtime::GUID, rgguidinterfaces: *const ::windows::runtime::GUID, dwinterfacecount: u32, pbatr: *const u8, pbatrmask: *const u8, cbatrlen: u32) -> i32;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardIntroduceReaderA(hcontext: usize, szreadername: super::super::Foundation::PSTR, szdevicename: super::super::Foundation::PSTR) -> i32;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardIntroduceReaderGroupA(hcontext: usize, szgroupname: super::super::Foundation::PSTR) -> i32;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardIntroduceReaderGroupW(hcontext: usize, szgroupname: super::super::Foundation::PWSTR) -> i32;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardIntroduceReaderW(hcontext: usize, szreadername: super::super::Foundation::PWSTR, szdevicename: super::super::Foundation::PWSTR) -> i32;
    #[doc = "*Required features: `Win32_Security_Credentials`*"]
    pub fn SCardIsValidContext(hcontext: usize) -> i32;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardListCardsA(hcontext: usize, pbatr: *const u8, rgquidinterfaces: *const ::windows::runtime::GUID, cguidinterfacecount: u32, mszcards: super::super::Foundation::PSTR, pcchcards: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardListCardsW(hcontext: usize, pbatr: *const u8, rgquidinterfaces: *const ::windows::runtime::GUID, cguidinterfacecount: u32, mszcards: super::super::Foundation::PWSTR, pcchcards: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardListInterfacesA(hcontext: usize, szcard: super::super::Foundation::PSTR, pguidinterfaces: *mut ::windows::runtime::GUID, pcguidinterfaces: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardListInterfacesW(hcontext: usize, szcard: super::super::Foundation::PWSTR, pguidinterfaces: *mut ::windows::runtime::GUID, pcguidinterfaces: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardListReaderGroupsA(hcontext: usize, mszgroups: super::super::Foundation::PSTR, pcchgroups: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardListReaderGroupsW(hcontext: usize, mszgroups: super::super::Foundation::PWSTR, pcchgroups: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardListReadersA(hcontext: usize, mszgroups: super::super::Foundation::PSTR, mszreaders: super::super::Foundation::PSTR, pcchreaders: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardListReadersW(hcontext: usize, mszgroups: super::super::Foundation::PWSTR, mszreaders: super::super::Foundation::PWSTR, pcchreaders: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardListReadersWithDeviceInstanceIdA(hcontext: usize, szdeviceinstanceid: super::super::Foundation::PSTR, mszreaders: super::super::Foundation::PSTR, pcchreaders: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardListReadersWithDeviceInstanceIdW(hcontext: usize, szdeviceinstanceid: super::super::Foundation::PWSTR, mszreaders: super::super::Foundation::PWSTR, pcchreaders: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardLocateCardsA(hcontext: usize, mszcards: super::super::Foundation::PSTR, rgreaderstates: *mut SCARD_READERSTATEA, creaders: u32) -> i32;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardLocateCardsByATRA(hcontext: usize, rgatrmasks: *const SCARD_ATRMASK, catrs: u32, rgreaderstates: *mut SCARD_READERSTATEA, creaders: u32) -> i32;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardLocateCardsByATRW(hcontext: usize, rgatrmasks: *const SCARD_ATRMASK, catrs: u32, rgreaderstates: *mut SCARD_READERSTATEW, creaders: u32) -> i32;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardLocateCardsW(hcontext: usize, mszcards: super::super::Foundation::PWSTR, rgreaderstates: *mut SCARD_READERSTATEW, creaders: u32) -> i32;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardReadCacheA(hcontext: usize, cardidentifier: *const ::windows::runtime::GUID, freshnesscounter: u32, lookupname: super::super::Foundation::PSTR, data: *mut u8, datalen: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardReadCacheW(hcontext: usize, cardidentifier: *const ::windows::runtime::GUID, freshnesscounter: u32, lookupname: super::super::Foundation::PWSTR, data: *mut u8, datalen: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Security_Credentials`*"]
    pub fn SCardReconnect(hcard: usize, dwsharemode: u32, dwpreferredprotocols: u32, dwinitialization: u32, pdwactiveprotocol: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Security_Credentials`*"]
    pub fn SCardReleaseContext(hcontext: usize) -> i32;
    #[doc = "*Required features: `Win32_Security_Credentials`*"]
    pub fn SCardReleaseStartedEvent();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardRemoveReaderFromGroupA(hcontext: usize, szreadername: super::super::Foundation::PSTR, szgroupname: super::super::Foundation::PSTR) -> i32;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardRemoveReaderFromGroupW(hcontext: usize, szreadername: super::super::Foundation::PWSTR, szgroupname: super::super::Foundation::PWSTR) -> i32;
    #[doc = "*Required features: `Win32_Security_Credentials`*"]
    pub fn SCardSetAttrib(hcard: usize, dwattrid: u32, pbattr: *const u8, cbattrlen: u32) -> i32;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardSetCardTypeProviderNameA(hcontext: usize, szcardname: super::super::Foundation::PSTR, dwproviderid: u32, szprovider: super::super::Foundation::PSTR) -> i32;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardSetCardTypeProviderNameW(hcontext: usize, szcardname: super::super::Foundation::PWSTR, dwproviderid: u32, szprovider: super::super::Foundation::PWSTR) -> i32;
    #[doc = "*Required features: `Win32_Security_Credentials`*"]
    pub fn SCardState(hcard: usize, pdwstate: *mut u32, pdwprotocol: *mut u32, pbatr: *mut u8, pcbatrlen: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardStatusA(hcard: usize, mszreadernames: super::super::Foundation::PSTR, pcchreaderlen: *mut u32, pdwstate: *mut u32, pdwprotocol: *mut u32, pbatr: *mut u8, pcbatrlen: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardStatusW(hcard: usize, mszreadernames: super::super::Foundation::PWSTR, pcchreaderlen: *mut u32, pdwstate: *mut u32, pdwprotocol: *mut u32, pbatr: *mut u8, pcbatrlen: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Security_Credentials`*"]
    pub fn SCardTransmit(hcard: usize, piosendpci: *const SCARD_IO_REQUEST, pbsendbuffer: *const u8, cbsendlength: u32, piorecvpci: *mut SCARD_IO_REQUEST, pbrecvbuffer: *mut u8, pcbrecvlength: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn SCardUIDlgSelectCardA(param0: *mut ::core::mem::ManuallyDrop<OPENCARDNAME_EXA>) -> i32;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn SCardUIDlgSelectCardW(param0: *mut ::core::mem::ManuallyDrop<OPENCARDNAME_EXW>) -> i32;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardWriteCacheA(hcontext: usize, cardidentifier: *const ::windows::runtime::GUID, freshnesscounter: u32, lookupname: super::super::Foundation::PSTR, data: *const u8, datalen: u32) -> i32;
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardWriteCacheW(hcontext: usize, cardidentifier: *const ::windows::runtime::GUID, freshnesscounter: u32, lookupname: super::super::Foundation::PWSTR, data: *const u8, datalen: u32) -> i32;
}
