#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredDeleteA<P0>(targetname: P0, r#type: u32, flags: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "advapi32.dll""system" fn CredDeleteA ( targetname : ::windows::core::PCSTR , r#type : u32 , flags : u32 ) -> super::super::Foundation:: BOOL );
    CredDeleteA(targetname.into_param().abi(), r#type, flags)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredDeleteW<P0>(targetname: P0, r#type: u32, flags: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "advapi32.dll""system" fn CredDeleteW ( targetname : ::windows::core::PCWSTR , r#type : u32 , flags : u32 ) -> super::super::Foundation:: BOOL );
    CredDeleteW(targetname.into_param().abi(), r#type, flags)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredEnumerateA<P0>(filter: P0, flags: CRED_ENUMERATE_FLAGS, count: *mut u32, credential: *mut *mut *mut CREDENTIALA) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "advapi32.dll""system" fn CredEnumerateA ( filter : ::windows::core::PCSTR , flags : CRED_ENUMERATE_FLAGS , count : *mut u32 , credential : *mut *mut *mut CREDENTIALA ) -> super::super::Foundation:: BOOL );
    CredEnumerateA(filter.into_param().abi(), flags, count, credential)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredEnumerateW<P0>(filter: P0, flags: CRED_ENUMERATE_FLAGS, count: *mut u32, credential: *mut *mut *mut CREDENTIALW) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "advapi32.dll""system" fn CredEnumerateW ( filter : ::windows::core::PCWSTR , flags : CRED_ENUMERATE_FLAGS , count : *mut u32 , credential : *mut *mut *mut CREDENTIALW ) -> super::super::Foundation:: BOOL );
    CredEnumerateW(filter.into_param().abi(), flags, count, credential)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredFindBestCredentialA<P0>(targetname: P0, r#type: u32, flags: u32, credential: *mut *mut CREDENTIALA) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "advapi32.dll""system" fn CredFindBestCredentialA ( targetname : ::windows::core::PCSTR , r#type : u32 , flags : u32 , credential : *mut *mut CREDENTIALA ) -> super::super::Foundation:: BOOL );
    CredFindBestCredentialA(targetname.into_param().abi(), r#type, flags, credential)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredFindBestCredentialW<P0>(targetname: P0, r#type: u32, flags: u32, credential: *mut *mut CREDENTIALW) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "advapi32.dll""system" fn CredFindBestCredentialW ( targetname : ::windows::core::PCWSTR , r#type : u32 , flags : u32 , credential : *mut *mut CREDENTIALW ) -> super::super::Foundation:: BOOL );
    CredFindBestCredentialW(targetname.into_param().abi(), r#type, flags, credential)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[inline]
pub unsafe fn CredFree(buffer: *const ::core::ffi::c_void) {
    ::windows_targets::link ! ( "advapi32.dll""system" fn CredFree ( buffer : *const ::core::ffi::c_void ) -> ( ) );
    CredFree(buffer)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredGetSessionTypes(maximumpersist: &mut [u32]) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "advapi32.dll""system" fn CredGetSessionTypes ( maximumpersistcount : u32 , maximumpersist : *mut u32 ) -> super::super::Foundation:: BOOL );
    CredGetSessionTypes(maximumpersist.len() as _, ::core::mem::transmute(maximumpersist.as_ptr()))
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredGetTargetInfoA<P0>(targetname: P0, flags: u32, targetinfo: *mut *mut CREDENTIAL_TARGET_INFORMATIONA) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "advapi32.dll""system" fn CredGetTargetInfoA ( targetname : ::windows::core::PCSTR , flags : u32 , targetinfo : *mut *mut CREDENTIAL_TARGET_INFORMATIONA ) -> super::super::Foundation:: BOOL );
    CredGetTargetInfoA(targetname.into_param().abi(), flags, targetinfo)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredGetTargetInfoW<P0>(targetname: P0, flags: u32, targetinfo: *mut *mut CREDENTIAL_TARGET_INFORMATIONW) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "advapi32.dll""system" fn CredGetTargetInfoW ( targetname : ::windows::core::PCWSTR , flags : u32 , targetinfo : *mut *mut CREDENTIAL_TARGET_INFORMATIONW ) -> super::super::Foundation:: BOOL );
    CredGetTargetInfoW(targetname.into_param().abi(), flags, targetinfo)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredIsMarshaledCredentialA<P0>(marshaledcredential: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "advapi32.dll""system" fn CredIsMarshaledCredentialA ( marshaledcredential : ::windows::core::PCSTR ) -> super::super::Foundation:: BOOL );
    CredIsMarshaledCredentialA(marshaledcredential.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredIsMarshaledCredentialW<P0>(marshaledcredential: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "advapi32.dll""system" fn CredIsMarshaledCredentialW ( marshaledcredential : ::windows::core::PCWSTR ) -> super::super::Foundation:: BOOL );
    CredIsMarshaledCredentialW(marshaledcredential.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredIsProtectedA<P0>(pszprotectedcredentials: P0, pprotectiontype: *mut CRED_PROTECTION_TYPE) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "advapi32.dll""system" fn CredIsProtectedA ( pszprotectedcredentials : ::windows::core::PCSTR , pprotectiontype : *mut CRED_PROTECTION_TYPE ) -> super::super::Foundation:: BOOL );
    CredIsProtectedA(pszprotectedcredentials.into_param().abi(), pprotectiontype)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredIsProtectedW<P0>(pszprotectedcredentials: P0, pprotectiontype: *mut CRED_PROTECTION_TYPE) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "advapi32.dll""system" fn CredIsProtectedW ( pszprotectedcredentials : ::windows::core::PCWSTR , pprotectiontype : *mut CRED_PROTECTION_TYPE ) -> super::super::Foundation:: BOOL );
    CredIsProtectedW(pszprotectedcredentials.into_param().abi(), pprotectiontype)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredMarshalCredentialA(credtype: CRED_MARSHAL_TYPE, credential: *const ::core::ffi::c_void, marshaledcredential: *mut ::windows::core::PSTR) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "advapi32.dll""system" fn CredMarshalCredentialA ( credtype : CRED_MARSHAL_TYPE , credential : *const ::core::ffi::c_void , marshaledcredential : *mut ::windows::core::PSTR ) -> super::super::Foundation:: BOOL );
    CredMarshalCredentialA(credtype, credential, marshaledcredential)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredMarshalCredentialW(credtype: CRED_MARSHAL_TYPE, credential: *const ::core::ffi::c_void, marshaledcredential: *mut ::windows::core::PWSTR) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "advapi32.dll""system" fn CredMarshalCredentialW ( credtype : CRED_MARSHAL_TYPE , credential : *const ::core::ffi::c_void , marshaledcredential : *mut ::windows::core::PWSTR ) -> super::super::Foundation:: BOOL );
    CredMarshalCredentialW(credtype, credential, marshaledcredential)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredPackAuthenticationBufferA<P0, P1>(dwflags: CRED_PACK_FLAGS, pszusername: P0, pszpassword: P1, ppackedcredentials: ::core::option::Option<*mut u8>, pcbpackedcredentials: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "credui.dll""system" fn CredPackAuthenticationBufferA ( dwflags : CRED_PACK_FLAGS , pszusername : ::windows::core::PCSTR , pszpassword : ::windows::core::PCSTR , ppackedcredentials : *mut u8 , pcbpackedcredentials : *mut u32 ) -> super::super::Foundation:: BOOL );
    CredPackAuthenticationBufferA(dwflags, pszusername.into_param().abi(), pszpassword.into_param().abi(), ::core::mem::transmute(ppackedcredentials.unwrap_or(::std::ptr::null_mut())), pcbpackedcredentials)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredPackAuthenticationBufferW<P0, P1>(dwflags: CRED_PACK_FLAGS, pszusername: P0, pszpassword: P1, ppackedcredentials: ::core::option::Option<*mut u8>, pcbpackedcredentials: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "credui.dll""system" fn CredPackAuthenticationBufferW ( dwflags : CRED_PACK_FLAGS , pszusername : ::windows::core::PCWSTR , pszpassword : ::windows::core::PCWSTR , ppackedcredentials : *mut u8 , pcbpackedcredentials : *mut u32 ) -> super::super::Foundation:: BOOL );
    CredPackAuthenticationBufferW(dwflags, pszusername.into_param().abi(), pszpassword.into_param().abi(), ::core::mem::transmute(ppackedcredentials.unwrap_or(::std::ptr::null_mut())), pcbpackedcredentials)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredProtectA<P0>(fasself: P0, pszcredentials: &[u8], pszprotectedcredentials: ::windows::core::PSTR, pcchmaxchars: *mut u32, protectiontype: ::core::option::Option<*mut CRED_PROTECTION_TYPE>) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "advapi32.dll""system" fn CredProtectA ( fasself : super::super::Foundation:: BOOL , pszcredentials : ::windows::core::PCSTR , cchcredentials : u32 , pszprotectedcredentials : ::windows::core::PSTR , pcchmaxchars : *mut u32 , protectiontype : *mut CRED_PROTECTION_TYPE ) -> super::super::Foundation:: BOOL );
    CredProtectA(fasself.into_param().abi(), ::core::mem::transmute(pszcredentials.as_ptr()), pszcredentials.len() as _, ::core::mem::transmute(pszprotectedcredentials), pcchmaxchars, ::core::mem::transmute(protectiontype.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredProtectW<P0>(fasself: P0, pszcredentials: &[u16], pszprotectedcredentials: ::windows::core::PWSTR, pcchmaxchars: *mut u32, protectiontype: ::core::option::Option<*mut CRED_PROTECTION_TYPE>) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "advapi32.dll""system" fn CredProtectW ( fasself : super::super::Foundation:: BOOL , pszcredentials : ::windows::core::PCWSTR , cchcredentials : u32 , pszprotectedcredentials : ::windows::core::PWSTR , pcchmaxchars : *mut u32 , protectiontype : *mut CRED_PROTECTION_TYPE ) -> super::super::Foundation:: BOOL );
    CredProtectW(fasself.into_param().abi(), ::core::mem::transmute(pszcredentials.as_ptr()), pszcredentials.len() as _, ::core::mem::transmute(pszprotectedcredentials), pcchmaxchars, ::core::mem::transmute(protectiontype.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredReadA<P0>(targetname: P0, r#type: u32, flags: u32, credential: *mut *mut CREDENTIALA) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "advapi32.dll""system" fn CredReadA ( targetname : ::windows::core::PCSTR , r#type : u32 , flags : u32 , credential : *mut *mut CREDENTIALA ) -> super::super::Foundation:: BOOL );
    CredReadA(targetname.into_param().abi(), r#type, flags, credential)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredReadDomainCredentialsA(targetinfo: *const CREDENTIAL_TARGET_INFORMATIONA, flags: u32, count: *mut u32, credential: *mut *mut *mut CREDENTIALA) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "advapi32.dll""system" fn CredReadDomainCredentialsA ( targetinfo : *const CREDENTIAL_TARGET_INFORMATIONA , flags : u32 , count : *mut u32 , credential : *mut *mut *mut CREDENTIALA ) -> super::super::Foundation:: BOOL );
    CredReadDomainCredentialsA(targetinfo, flags, count, credential)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredReadDomainCredentialsW(targetinfo: *const CREDENTIAL_TARGET_INFORMATIONW, flags: u32, count: *mut u32, credential: *mut *mut *mut CREDENTIALW) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "advapi32.dll""system" fn CredReadDomainCredentialsW ( targetinfo : *const CREDENTIAL_TARGET_INFORMATIONW , flags : u32 , count : *mut u32 , credential : *mut *mut *mut CREDENTIALW ) -> super::super::Foundation:: BOOL );
    CredReadDomainCredentialsW(targetinfo, flags, count, credential)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredReadW<P0>(targetname: P0, r#type: u32, flags: u32, credential: *mut *mut CREDENTIALW) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "advapi32.dll""system" fn CredReadW ( targetname : ::windows::core::PCWSTR , r#type : u32 , flags : u32 , credential : *mut *mut CREDENTIALW ) -> super::super::Foundation:: BOOL );
    CredReadW(targetname.into_param().abi(), r#type, flags, credential)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredRenameA<P0, P1>(oldtargetname: P0, newtargetname: P1, r#type: u32, flags: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "advapi32.dll""system" fn CredRenameA ( oldtargetname : ::windows::core::PCSTR , newtargetname : ::windows::core::PCSTR , r#type : u32 , flags : u32 ) -> super::super::Foundation:: BOOL );
    CredRenameA(oldtargetname.into_param().abi(), newtargetname.into_param().abi(), r#type, flags)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredRenameW<P0, P1>(oldtargetname: P0, newtargetname: P1, r#type: u32, flags: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "advapi32.dll""system" fn CredRenameW ( oldtargetname : ::windows::core::PCWSTR , newtargetname : ::windows::core::PCWSTR , r#type : u32 , flags : u32 ) -> super::super::Foundation:: BOOL );
    CredRenameW(oldtargetname.into_param().abi(), newtargetname.into_param().abi(), r#type, flags)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredUICmdLinePromptForCredentialsA<P0>(psztargetname: P0, pcontext: ::core::option::Option<*const SecHandle>, dwautherror: u32, username: &mut [u8], pszpassword: &mut [u8], pfsave: ::core::option::Option<*mut super::super::Foundation::BOOL>, dwflags: CREDUI_FLAGS) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "credui.dll""system" fn CredUICmdLinePromptForCredentialsA ( psztargetname : ::windows::core::PCSTR , pcontext : *const SecHandle , dwautherror : u32 , username : ::windows::core::PSTR , uluserbuffersize : u32 , pszpassword : ::windows::core::PSTR , ulpasswordbuffersize : u32 , pfsave : *mut super::super::Foundation:: BOOL , dwflags : CREDUI_FLAGS ) -> u32 );
    CredUICmdLinePromptForCredentialsA(psztargetname.into_param().abi(), ::core::mem::transmute(pcontext.unwrap_or(::std::ptr::null())), dwautherror, ::core::mem::transmute(username.as_ptr()), username.len() as _, ::core::mem::transmute(pszpassword.as_ptr()), pszpassword.len() as _, ::core::mem::transmute(pfsave.unwrap_or(::std::ptr::null_mut())), dwflags)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredUICmdLinePromptForCredentialsW<P0>(psztargetname: P0, pcontext: ::core::option::Option<*const SecHandle>, dwautherror: u32, username: &mut [u16], pszpassword: &mut [u16], pfsave: ::core::option::Option<*mut super::super::Foundation::BOOL>, dwflags: CREDUI_FLAGS) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "credui.dll""system" fn CredUICmdLinePromptForCredentialsW ( psztargetname : ::windows::core::PCWSTR , pcontext : *const SecHandle , dwautherror : u32 , username : ::windows::core::PWSTR , uluserbuffersize : u32 , pszpassword : ::windows::core::PWSTR , ulpasswordbuffersize : u32 , pfsave : *mut super::super::Foundation:: BOOL , dwflags : CREDUI_FLAGS ) -> u32 );
    CredUICmdLinePromptForCredentialsW(psztargetname.into_param().abi(), ::core::mem::transmute(pcontext.unwrap_or(::std::ptr::null())), dwautherror, ::core::mem::transmute(username.as_ptr()), username.len() as _, ::core::mem::transmute(pszpassword.as_ptr()), pszpassword.len() as _, ::core::mem::transmute(pfsave.unwrap_or(::std::ptr::null_mut())), dwflags)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredUIConfirmCredentialsA<P0, P1>(psztargetname: P0, bconfirm: P1) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "credui.dll""system" fn CredUIConfirmCredentialsA ( psztargetname : ::windows::core::PCSTR , bconfirm : super::super::Foundation:: BOOL ) -> u32 );
    CredUIConfirmCredentialsA(psztargetname.into_param().abi(), bconfirm.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredUIConfirmCredentialsW<P0, P1>(psztargetname: P0, bconfirm: P1) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "credui.dll""system" fn CredUIConfirmCredentialsW ( psztargetname : ::windows::core::PCWSTR , bconfirm : super::super::Foundation:: BOOL ) -> u32 );
    CredUIConfirmCredentialsW(psztargetname.into_param().abi(), bconfirm.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[inline]
pub unsafe fn CredUIParseUserNameA<P0>(username: P0, user: &mut [u8], domain: &mut [u8]) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "credui.dll""system" fn CredUIParseUserNameA ( username : ::windows::core::PCSTR , user : ::windows::core::PSTR , userbuffersize : u32 , domain : ::windows::core::PSTR , domainbuffersize : u32 ) -> u32 );
    CredUIParseUserNameA(username.into_param().abi(), ::core::mem::transmute(user.as_ptr()), user.len() as _, ::core::mem::transmute(domain.as_ptr()), domain.len() as _)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[inline]
pub unsafe fn CredUIParseUserNameW<P0>(username: P0, user: &mut [u16], domain: &mut [u16]) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "credui.dll""system" fn CredUIParseUserNameW ( username : ::windows::core::PCWSTR , user : ::windows::core::PWSTR , userbuffersize : u32 , domain : ::windows::core::PWSTR , domainbuffersize : u32 ) -> u32 );
    CredUIParseUserNameW(username.into_param().abi(), ::core::mem::transmute(user.as_ptr()), user.len() as _, ::core::mem::transmute(domain.as_ptr()), domain.len() as _)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn CredUIPromptForCredentialsA<P0>(puiinfo: ::core::option::Option<*const CREDUI_INFOA>, psztargetname: P0, pcontext: ::core::option::Option<*const SecHandle>, dwautherror: u32, pszusername: &mut [u8], pszpassword: &mut [u8], save: ::core::option::Option<*mut super::super::Foundation::BOOL>, dwflags: CREDUI_FLAGS) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "credui.dll""system" fn CredUIPromptForCredentialsA ( puiinfo : *const CREDUI_INFOA , psztargetname : ::windows::core::PCSTR , pcontext : *const SecHandle , dwautherror : u32 , pszusername : ::windows::core::PSTR , ulusernamebuffersize : u32 , pszpassword : ::windows::core::PSTR , ulpasswordbuffersize : u32 , save : *mut super::super::Foundation:: BOOL , dwflags : CREDUI_FLAGS ) -> u32 );
    CredUIPromptForCredentialsA(::core::mem::transmute(puiinfo.unwrap_or(::std::ptr::null())), psztargetname.into_param().abi(), ::core::mem::transmute(pcontext.unwrap_or(::std::ptr::null())), dwautherror, ::core::mem::transmute(pszusername.as_ptr()), pszusername.len() as _, ::core::mem::transmute(pszpassword.as_ptr()), pszpassword.len() as _, ::core::mem::transmute(save.unwrap_or(::std::ptr::null_mut())), dwflags)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn CredUIPromptForCredentialsW<P0>(puiinfo: ::core::option::Option<*const CREDUI_INFOW>, psztargetname: P0, pcontext: ::core::option::Option<*const SecHandle>, dwautherror: u32, pszusername: &mut [u16], pszpassword: &mut [u16], save: ::core::option::Option<*mut super::super::Foundation::BOOL>, dwflags: CREDUI_FLAGS) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "credui.dll""system" fn CredUIPromptForCredentialsW ( puiinfo : *const CREDUI_INFOW , psztargetname : ::windows::core::PCWSTR , pcontext : *const SecHandle , dwautherror : u32 , pszusername : ::windows::core::PWSTR , ulusernamebuffersize : u32 , pszpassword : ::windows::core::PWSTR , ulpasswordbuffersize : u32 , save : *mut super::super::Foundation:: BOOL , dwflags : CREDUI_FLAGS ) -> u32 );
    CredUIPromptForCredentialsW(::core::mem::transmute(puiinfo.unwrap_or(::std::ptr::null())), psztargetname.into_param().abi(), ::core::mem::transmute(pcontext.unwrap_or(::std::ptr::null())), dwautherror, ::core::mem::transmute(pszusername.as_ptr()), pszusername.len() as _, ::core::mem::transmute(pszpassword.as_ptr()), pszpassword.len() as _, ::core::mem::transmute(save.unwrap_or(::std::ptr::null_mut())), dwflags)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn CredUIPromptForWindowsCredentialsA(puiinfo: ::core::option::Option<*const CREDUI_INFOA>, dwautherror: u32, pulauthpackage: *mut u32, pvinauthbuffer: ::core::option::Option<*const ::core::ffi::c_void>, ulinauthbuffersize: u32, ppvoutauthbuffer: *mut *mut ::core::ffi::c_void, puloutauthbuffersize: *mut u32, pfsave: ::core::option::Option<*mut super::super::Foundation::BOOL>, dwflags: CREDUIWIN_FLAGS) -> u32 {
    ::windows_targets::link ! ( "credui.dll""system" fn CredUIPromptForWindowsCredentialsA ( puiinfo : *const CREDUI_INFOA , dwautherror : u32 , pulauthpackage : *mut u32 , pvinauthbuffer : *const ::core::ffi::c_void , ulinauthbuffersize : u32 , ppvoutauthbuffer : *mut *mut ::core::ffi::c_void , puloutauthbuffersize : *mut u32 , pfsave : *mut super::super::Foundation:: BOOL , dwflags : CREDUIWIN_FLAGS ) -> u32 );
    CredUIPromptForWindowsCredentialsA(::core::mem::transmute(puiinfo.unwrap_or(::std::ptr::null())), dwautherror, pulauthpackage, ::core::mem::transmute(pvinauthbuffer.unwrap_or(::std::ptr::null())), ulinauthbuffersize, ppvoutauthbuffer, puloutauthbuffersize, ::core::mem::transmute(pfsave.unwrap_or(::std::ptr::null_mut())), dwflags)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn CredUIPromptForWindowsCredentialsW(puiinfo: ::core::option::Option<*const CREDUI_INFOW>, dwautherror: u32, pulauthpackage: *mut u32, pvinauthbuffer: ::core::option::Option<*const ::core::ffi::c_void>, ulinauthbuffersize: u32, ppvoutauthbuffer: *mut *mut ::core::ffi::c_void, puloutauthbuffersize: *mut u32, pfsave: ::core::option::Option<*mut super::super::Foundation::BOOL>, dwflags: CREDUIWIN_FLAGS) -> u32 {
    ::windows_targets::link ! ( "credui.dll""system" fn CredUIPromptForWindowsCredentialsW ( puiinfo : *const CREDUI_INFOW , dwautherror : u32 , pulauthpackage : *mut u32 , pvinauthbuffer : *const ::core::ffi::c_void , ulinauthbuffersize : u32 , ppvoutauthbuffer : *mut *mut ::core::ffi::c_void , puloutauthbuffersize : *mut u32 , pfsave : *mut super::super::Foundation:: BOOL , dwflags : CREDUIWIN_FLAGS ) -> u32 );
    CredUIPromptForWindowsCredentialsW(::core::mem::transmute(puiinfo.unwrap_or(::std::ptr::null())), dwautherror, pulauthpackage, ::core::mem::transmute(pvinauthbuffer.unwrap_or(::std::ptr::null())), ulinauthbuffersize, ppvoutauthbuffer, puloutauthbuffersize, ::core::mem::transmute(pfsave.unwrap_or(::std::ptr::null_mut())), dwflags)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[inline]
pub unsafe fn CredUIReadSSOCredW<P0>(pszrealm: P0, ppszusername: *mut ::windows::core::PWSTR) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "credui.dll""system" fn CredUIReadSSOCredW ( pszrealm : ::windows::core::PCWSTR , ppszusername : *mut ::windows::core::PWSTR ) -> u32 );
    CredUIReadSSOCredW(pszrealm.into_param().abi(), ppszusername)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredUIStoreSSOCredW<P0, P1, P2, P3>(pszrealm: P0, pszusername: P1, pszpassword: P2, bpersist: P3) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P3: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "credui.dll""system" fn CredUIStoreSSOCredW ( pszrealm : ::windows::core::PCWSTR , pszusername : ::windows::core::PCWSTR , pszpassword : ::windows::core::PCWSTR , bpersist : super::super::Foundation:: BOOL ) -> u32 );
    CredUIStoreSSOCredW(pszrealm.into_param().abi(), pszusername.into_param().abi(), pszpassword.into_param().abi(), bpersist.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredUnPackAuthenticationBufferA(dwflags: CRED_PACK_FLAGS, pauthbuffer: *const ::core::ffi::c_void, cbauthbuffer: u32, pszusername: ::windows::core::PSTR, pcchlmaxusername: *mut u32, pszdomainname: ::windows::core::PSTR, pcchmaxdomainname: ::core::option::Option<*mut u32>, pszpassword: ::windows::core::PSTR, pcchmaxpassword: *mut u32) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "credui.dll""system" fn CredUnPackAuthenticationBufferA ( dwflags : CRED_PACK_FLAGS , pauthbuffer : *const ::core::ffi::c_void , cbauthbuffer : u32 , pszusername : ::windows::core::PSTR , pcchlmaxusername : *mut u32 , pszdomainname : ::windows::core::PSTR , pcchmaxdomainname : *mut u32 , pszpassword : ::windows::core::PSTR , pcchmaxpassword : *mut u32 ) -> super::super::Foundation:: BOOL );
    CredUnPackAuthenticationBufferA(dwflags, pauthbuffer, cbauthbuffer, ::core::mem::transmute(pszusername), pcchlmaxusername, ::core::mem::transmute(pszdomainname), ::core::mem::transmute(pcchmaxdomainname.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pszpassword), pcchmaxpassword)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredUnPackAuthenticationBufferW(dwflags: CRED_PACK_FLAGS, pauthbuffer: *const ::core::ffi::c_void, cbauthbuffer: u32, pszusername: ::windows::core::PWSTR, pcchmaxusername: *mut u32, pszdomainname: ::windows::core::PWSTR, pcchmaxdomainname: ::core::option::Option<*mut u32>, pszpassword: ::windows::core::PWSTR, pcchmaxpassword: *mut u32) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "credui.dll""system" fn CredUnPackAuthenticationBufferW ( dwflags : CRED_PACK_FLAGS , pauthbuffer : *const ::core::ffi::c_void , cbauthbuffer : u32 , pszusername : ::windows::core::PWSTR , pcchmaxusername : *mut u32 , pszdomainname : ::windows::core::PWSTR , pcchmaxdomainname : *mut u32 , pszpassword : ::windows::core::PWSTR , pcchmaxpassword : *mut u32 ) -> super::super::Foundation:: BOOL );
    CredUnPackAuthenticationBufferW(dwflags, pauthbuffer, cbauthbuffer, ::core::mem::transmute(pszusername), pcchmaxusername, ::core::mem::transmute(pszdomainname), ::core::mem::transmute(pcchmaxdomainname.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pszpassword), pcchmaxpassword)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredUnmarshalCredentialA<P0>(marshaledcredential: P0, credtype: *mut CRED_MARSHAL_TYPE, credential: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "advapi32.dll""system" fn CredUnmarshalCredentialA ( marshaledcredential : ::windows::core::PCSTR , credtype : *mut CRED_MARSHAL_TYPE , credential : *mut *mut ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    CredUnmarshalCredentialA(marshaledcredential.into_param().abi(), credtype, credential)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredUnmarshalCredentialW<P0>(marshaledcredential: P0, credtype: *mut CRED_MARSHAL_TYPE, credential: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "advapi32.dll""system" fn CredUnmarshalCredentialW ( marshaledcredential : ::windows::core::PCWSTR , credtype : *mut CRED_MARSHAL_TYPE , credential : *mut *mut ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    CredUnmarshalCredentialW(marshaledcredential.into_param().abi(), credtype, credential)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredUnprotectA<P0>(fasself: P0, pszprotectedcredentials: &[u8], pszcredentials: ::windows::core::PSTR, pcchmaxchars: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "advapi32.dll""system" fn CredUnprotectA ( fasself : super::super::Foundation:: BOOL , pszprotectedcredentials : ::windows::core::PCSTR , cchprotectedcredentials : u32 , pszcredentials : ::windows::core::PSTR , pcchmaxchars : *mut u32 ) -> super::super::Foundation:: BOOL );
    CredUnprotectA(fasself.into_param().abi(), ::core::mem::transmute(pszprotectedcredentials.as_ptr()), pszprotectedcredentials.len() as _, ::core::mem::transmute(pszcredentials), pcchmaxchars)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredUnprotectW<P0>(fasself: P0, pszprotectedcredentials: &[u16], pszcredentials: ::windows::core::PWSTR, pcchmaxchars: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "advapi32.dll""system" fn CredUnprotectW ( fasself : super::super::Foundation:: BOOL , pszprotectedcredentials : ::windows::core::PCWSTR , cchprotectedcredentials : u32 , pszcredentials : ::windows::core::PWSTR , pcchmaxchars : *mut u32 ) -> super::super::Foundation:: BOOL );
    CredUnprotectW(fasself.into_param().abi(), ::core::mem::transmute(pszprotectedcredentials.as_ptr()), pszprotectedcredentials.len() as _, ::core::mem::transmute(pszcredentials), pcchmaxchars)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredWriteA(credential: *const CREDENTIALA, flags: u32) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "advapi32.dll""system" fn CredWriteA ( credential : *const CREDENTIALA , flags : u32 ) -> super::super::Foundation:: BOOL );
    CredWriteA(credential, flags)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredWriteDomainCredentialsA(targetinfo: *const CREDENTIAL_TARGET_INFORMATIONA, credential: *const CREDENTIALA, flags: u32) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "advapi32.dll""system" fn CredWriteDomainCredentialsA ( targetinfo : *const CREDENTIAL_TARGET_INFORMATIONA , credential : *const CREDENTIALA , flags : u32 ) -> super::super::Foundation:: BOOL );
    CredWriteDomainCredentialsA(targetinfo, credential, flags)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredWriteDomainCredentialsW(targetinfo: *const CREDENTIAL_TARGET_INFORMATIONW, credential: *const CREDENTIALW, flags: u32) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "advapi32.dll""system" fn CredWriteDomainCredentialsW ( targetinfo : *const CREDENTIAL_TARGET_INFORMATIONW , credential : *const CREDENTIALW , flags : u32 ) -> super::super::Foundation:: BOOL );
    CredWriteDomainCredentialsW(targetinfo, credential, flags)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CredWriteW(credential: *const CREDENTIALW, flags: u32) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "advapi32.dll""system" fn CredWriteW ( credential : *const CREDENTIALW , flags : u32 ) -> super::super::Foundation:: BOOL );
    CredWriteW(credential, flags)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetOpenCardNameA(param0: *mut OPENCARDNAMEA) -> i32 {
    ::windows_targets::link ! ( "scarddlg.dll""system" fn GetOpenCardNameA ( param0 : *mut OPENCARDNAMEA ) -> i32 );
    GetOpenCardNameA(param0)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetOpenCardNameW(param0: *mut OPENCARDNAMEW) -> i32 {
    ::windows_targets::link ! ( "scarddlg.dll""system" fn GetOpenCardNameW ( param0 : *mut OPENCARDNAMEW ) -> i32 );
    GetOpenCardNameW(param0)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[inline]
pub unsafe fn KeyCredentialManagerFreeInformation(keycredentialmanagerinfo: *const KeyCredentialManagerInfo) {
    ::windows_targets::link ! ( "keycredmgr.dll""system" fn KeyCredentialManagerFreeInformation ( keycredentialmanagerinfo : *const KeyCredentialManagerInfo ) -> ( ) );
    KeyCredentialManagerFreeInformation(keycredentialmanagerinfo)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[inline]
pub unsafe fn KeyCredentialManagerGetInformation() -> ::windows::core::Result<*mut KeyCredentialManagerInfo> {
    ::windows_targets::link ! ( "keycredmgr.dll""system" fn KeyCredentialManagerGetInformation ( keycredentialmanagerinfo : *mut *mut KeyCredentialManagerInfo ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<*mut KeyCredentialManagerInfo>();
    KeyCredentialManagerGetInformation(&mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn KeyCredentialManagerGetOperationErrorStates(keycredentialmanageroperationtype: KeyCredentialManagerOperationType, isready: *mut super::super::Foundation::BOOL, keycredentialmanageroperationerrorstates: *mut KeyCredentialManagerOperationErrorStates) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "keycredmgr.dll""system" fn KeyCredentialManagerGetOperationErrorStates ( keycredentialmanageroperationtype : KeyCredentialManagerOperationType , isready : *mut super::super::Foundation:: BOOL , keycredentialmanageroperationerrorstates : *mut KeyCredentialManagerOperationErrorStates ) -> ::windows::core::HRESULT );
    KeyCredentialManagerGetOperationErrorStates(keycredentialmanageroperationtype, isready, keycredentialmanageroperationerrorstates).ok()
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn KeyCredentialManagerShowUIOperation<P0>(hwndowner: P0, keycredentialmanageroperationtype: KeyCredentialManagerOperationType) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
{
    ::windows_targets::link ! ( "keycredmgr.dll""system" fn KeyCredentialManagerShowUIOperation ( hwndowner : super::super::Foundation:: HWND , keycredentialmanageroperationtype : KeyCredentialManagerOperationType ) -> ::windows::core::HRESULT );
    KeyCredentialManagerShowUIOperation(hwndowner.into_param().abi(), keycredentialmanageroperationtype).ok()
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SCardAccessStartedEvent() -> ::windows::core::Result<super::super::Foundation::HANDLE> {
    ::windows_targets::link ! ( "winscard.dll""system" fn SCardAccessStartedEvent ( ) -> super::super::Foundation:: HANDLE );
    let result__ = SCardAccessStartedEvent();
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[inline]
pub unsafe fn SCardAddReaderToGroupA<P0, P1>(hcontext: usize, szreadername: P0, szgroupname: P1) -> i32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "winscard.dll""system" fn SCardAddReaderToGroupA ( hcontext : usize , szreadername : ::windows::core::PCSTR , szgroupname : ::windows::core::PCSTR ) -> i32 );
    SCardAddReaderToGroupA(hcontext, szreadername.into_param().abi(), szgroupname.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[inline]
pub unsafe fn SCardAddReaderToGroupW<P0, P1>(hcontext: usize, szreadername: P0, szgroupname: P1) -> i32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "winscard.dll""system" fn SCardAddReaderToGroupW ( hcontext : usize , szreadername : ::windows::core::PCWSTR , szgroupname : ::windows::core::PCWSTR ) -> i32 );
    SCardAddReaderToGroupW(hcontext, szreadername.into_param().abi(), szgroupname.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[inline]
pub unsafe fn SCardAudit(hcontext: usize, dwevent: u32) -> i32 {
    ::windows_targets::link ! ( "winscard.dll""system" fn SCardAudit ( hcontext : usize , dwevent : u32 ) -> i32 );
    SCardAudit(hcontext, dwevent)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[inline]
pub unsafe fn SCardBeginTransaction(hcard: usize) -> i32 {
    ::windows_targets::link ! ( "winscard.dll""system" fn SCardBeginTransaction ( hcard : usize ) -> i32 );
    SCardBeginTransaction(hcard)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[inline]
pub unsafe fn SCardCancel(hcontext: usize) -> i32 {
    ::windows_targets::link ! ( "winscard.dll""system" fn SCardCancel ( hcontext : usize ) -> i32 );
    SCardCancel(hcontext)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[inline]
pub unsafe fn SCardConnectA<P0>(hcontext: usize, szreader: P0, dwsharemode: u32, dwpreferredprotocols: u32, phcard: *mut usize, pdwactiveprotocol: *mut u32) -> i32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "winscard.dll""system" fn SCardConnectA ( hcontext : usize , szreader : ::windows::core::PCSTR , dwsharemode : u32 , dwpreferredprotocols : u32 , phcard : *mut usize , pdwactiveprotocol : *mut u32 ) -> i32 );
    SCardConnectA(hcontext, szreader.into_param().abi(), dwsharemode, dwpreferredprotocols, phcard, pdwactiveprotocol)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[inline]
pub unsafe fn SCardConnectW<P0>(hcontext: usize, szreader: P0, dwsharemode: u32, dwpreferredprotocols: u32, phcard: *mut usize, pdwactiveprotocol: *mut u32) -> i32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "winscard.dll""system" fn SCardConnectW ( hcontext : usize , szreader : ::windows::core::PCWSTR , dwsharemode : u32 , dwpreferredprotocols : u32 , phcard : *mut usize , pdwactiveprotocol : *mut u32 ) -> i32 );
    SCardConnectW(hcontext, szreader.into_param().abi(), dwsharemode, dwpreferredprotocols, phcard, pdwactiveprotocol)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[inline]
pub unsafe fn SCardControl(hcard: usize, dwcontrolcode: u32, lpinbuffer: *const ::core::ffi::c_void, cbinbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, cboutbuffersize: u32, lpbytesreturned: *mut u32) -> i32 {
    ::windows_targets::link ! ( "winscard.dll""system" fn SCardControl ( hcard : usize , dwcontrolcode : u32 , lpinbuffer : *const ::core::ffi::c_void , cbinbuffersize : u32 , lpoutbuffer : *mut ::core::ffi::c_void , cboutbuffersize : u32 , lpbytesreturned : *mut u32 ) -> i32 );
    SCardControl(hcard, dwcontrolcode, lpinbuffer, cbinbuffersize, lpoutbuffer, cboutbuffersize, lpbytesreturned)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[inline]
pub unsafe fn SCardDisconnect(hcard: usize, dwdisposition: u32) -> i32 {
    ::windows_targets::link ! ( "winscard.dll""system" fn SCardDisconnect ( hcard : usize , dwdisposition : u32 ) -> i32 );
    SCardDisconnect(hcard, dwdisposition)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[inline]
pub unsafe fn SCardDlgExtendedError() -> i32 {
    ::windows_targets::link ! ( "scarddlg.dll""system" fn SCardDlgExtendedError ( ) -> i32 );
    SCardDlgExtendedError()
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[inline]
pub unsafe fn SCardEndTransaction(hcard: usize, dwdisposition: u32) -> i32 {
    ::windows_targets::link ! ( "winscard.dll""system" fn SCardEndTransaction ( hcard : usize , dwdisposition : u32 ) -> i32 );
    SCardEndTransaction(hcard, dwdisposition)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[inline]
pub unsafe fn SCardEstablishContext(dwscope: SCARD_SCOPE, pvreserved1: ::core::option::Option<*const ::core::ffi::c_void>, pvreserved2: ::core::option::Option<*const ::core::ffi::c_void>, phcontext: *mut usize) -> i32 {
    ::windows_targets::link ! ( "winscard.dll""system" fn SCardEstablishContext ( dwscope : SCARD_SCOPE , pvreserved1 : *const ::core::ffi::c_void , pvreserved2 : *const ::core::ffi::c_void , phcontext : *mut usize ) -> i32 );
    SCardEstablishContext(dwscope, ::core::mem::transmute(pvreserved1.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pvreserved2.unwrap_or(::std::ptr::null())), phcontext)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[inline]
pub unsafe fn SCardForgetCardTypeA<P0>(hcontext: usize, szcardname: P0) -> i32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "winscard.dll""system" fn SCardForgetCardTypeA ( hcontext : usize , szcardname : ::windows::core::PCSTR ) -> i32 );
    SCardForgetCardTypeA(hcontext, szcardname.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[inline]
pub unsafe fn SCardForgetCardTypeW<P0>(hcontext: usize, szcardname: P0) -> i32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "winscard.dll""system" fn SCardForgetCardTypeW ( hcontext : usize , szcardname : ::windows::core::PCWSTR ) -> i32 );
    SCardForgetCardTypeW(hcontext, szcardname.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[inline]
pub unsafe fn SCardForgetReaderA<P0>(hcontext: usize, szreadername: P0) -> i32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "winscard.dll""system" fn SCardForgetReaderA ( hcontext : usize , szreadername : ::windows::core::PCSTR ) -> i32 );
    SCardForgetReaderA(hcontext, szreadername.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[inline]
pub unsafe fn SCardForgetReaderGroupA<P0>(hcontext: usize, szgroupname: P0) -> i32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "winscard.dll""system" fn SCardForgetReaderGroupA ( hcontext : usize , szgroupname : ::windows::core::PCSTR ) -> i32 );
    SCardForgetReaderGroupA(hcontext, szgroupname.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[inline]
pub unsafe fn SCardForgetReaderGroupW<P0>(hcontext: usize, szgroupname: P0) -> i32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "winscard.dll""system" fn SCardForgetReaderGroupW ( hcontext : usize , szgroupname : ::windows::core::PCWSTR ) -> i32 );
    SCardForgetReaderGroupW(hcontext, szgroupname.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[inline]
pub unsafe fn SCardForgetReaderW<P0>(hcontext: usize, szreadername: P0) -> i32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "winscard.dll""system" fn SCardForgetReaderW ( hcontext : usize , szreadername : ::windows::core::PCWSTR ) -> i32 );
    SCardForgetReaderW(hcontext, szreadername.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[inline]
pub unsafe fn SCardFreeMemory(hcontext: usize, pvmem: *const ::core::ffi::c_void) -> i32 {
    ::windows_targets::link ! ( "winscard.dll""system" fn SCardFreeMemory ( hcontext : usize , pvmem : *const ::core::ffi::c_void ) -> i32 );
    SCardFreeMemory(hcontext, pvmem)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[inline]
pub unsafe fn SCardGetAttrib(hcard: usize, dwattrid: u32, pbattr: ::core::option::Option<*mut u8>, pcbattrlen: *mut u32) -> i32 {
    ::windows_targets::link ! ( "winscard.dll""system" fn SCardGetAttrib ( hcard : usize , dwattrid : u32 , pbattr : *mut u8 , pcbattrlen : *mut u32 ) -> i32 );
    SCardGetAttrib(hcard, dwattrid, ::core::mem::transmute(pbattr.unwrap_or(::std::ptr::null_mut())), pcbattrlen)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[inline]
pub unsafe fn SCardGetCardTypeProviderNameA<P0>(hcontext: usize, szcardname: P0, dwproviderid: u32, szprovider: ::windows::core::PSTR, pcchprovider: *mut u32) -> i32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "winscard.dll""system" fn SCardGetCardTypeProviderNameA ( hcontext : usize , szcardname : ::windows::core::PCSTR , dwproviderid : u32 , szprovider : ::windows::core::PSTR , pcchprovider : *mut u32 ) -> i32 );
    SCardGetCardTypeProviderNameA(hcontext, szcardname.into_param().abi(), dwproviderid, ::core::mem::transmute(szprovider), pcchprovider)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[inline]
pub unsafe fn SCardGetCardTypeProviderNameW<P0>(hcontext: usize, szcardname: P0, dwproviderid: u32, szprovider: ::windows::core::PWSTR, pcchprovider: *mut u32) -> i32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "winscard.dll""system" fn SCardGetCardTypeProviderNameW ( hcontext : usize , szcardname : ::windows::core::PCWSTR , dwproviderid : u32 , szprovider : ::windows::core::PWSTR , pcchprovider : *mut u32 ) -> i32 );
    SCardGetCardTypeProviderNameW(hcontext, szcardname.into_param().abi(), dwproviderid, ::core::mem::transmute(szprovider), pcchprovider)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[inline]
pub unsafe fn SCardGetDeviceTypeIdA<P0>(hcontext: usize, szreadername: P0, pdwdevicetypeid: *mut u32) -> i32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "winscard.dll""system" fn SCardGetDeviceTypeIdA ( hcontext : usize , szreadername : ::windows::core::PCSTR , pdwdevicetypeid : *mut u32 ) -> i32 );
    SCardGetDeviceTypeIdA(hcontext, szreadername.into_param().abi(), pdwdevicetypeid)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[inline]
pub unsafe fn SCardGetDeviceTypeIdW<P0>(hcontext: usize, szreadername: P0, pdwdevicetypeid: *mut u32) -> i32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "winscard.dll""system" fn SCardGetDeviceTypeIdW ( hcontext : usize , szreadername : ::windows::core::PCWSTR , pdwdevicetypeid : *mut u32 ) -> i32 );
    SCardGetDeviceTypeIdW(hcontext, szreadername.into_param().abi(), pdwdevicetypeid)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[inline]
pub unsafe fn SCardGetProviderIdA<P0>(hcontext: usize, szcard: P0, pguidproviderid: *mut ::windows::core::GUID) -> i32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "winscard.dll""system" fn SCardGetProviderIdA ( hcontext : usize , szcard : ::windows::core::PCSTR , pguidproviderid : *mut ::windows::core::GUID ) -> i32 );
    SCardGetProviderIdA(hcontext, szcard.into_param().abi(), pguidproviderid)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[inline]
pub unsafe fn SCardGetProviderIdW<P0>(hcontext: usize, szcard: P0, pguidproviderid: *mut ::windows::core::GUID) -> i32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "winscard.dll""system" fn SCardGetProviderIdW ( hcontext : usize , szcard : ::windows::core::PCWSTR , pguidproviderid : *mut ::windows::core::GUID ) -> i32 );
    SCardGetProviderIdW(hcontext, szcard.into_param().abi(), pguidproviderid)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[inline]
pub unsafe fn SCardGetReaderDeviceInstanceIdA<P0>(hcontext: usize, szreadername: P0, szdeviceinstanceid: ::windows::core::PSTR, pcchdeviceinstanceid: *mut u32) -> i32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "winscard.dll""system" fn SCardGetReaderDeviceInstanceIdA ( hcontext : usize , szreadername : ::windows::core::PCSTR , szdeviceinstanceid : ::windows::core::PSTR , pcchdeviceinstanceid : *mut u32 ) -> i32 );
    SCardGetReaderDeviceInstanceIdA(hcontext, szreadername.into_param().abi(), ::core::mem::transmute(szdeviceinstanceid), pcchdeviceinstanceid)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[inline]
pub unsafe fn SCardGetReaderDeviceInstanceIdW<P0>(hcontext: usize, szreadername: P0, szdeviceinstanceid: ::windows::core::PWSTR, pcchdeviceinstanceid: *mut u32) -> i32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "winscard.dll""system" fn SCardGetReaderDeviceInstanceIdW ( hcontext : usize , szreadername : ::windows::core::PCWSTR , szdeviceinstanceid : ::windows::core::PWSTR , pcchdeviceinstanceid : *mut u32 ) -> i32 );
    SCardGetReaderDeviceInstanceIdW(hcontext, szreadername.into_param().abi(), ::core::mem::transmute(szdeviceinstanceid), pcchdeviceinstanceid)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[inline]
pub unsafe fn SCardGetReaderIconA<P0>(hcontext: usize, szreadername: P0, pbicon: *mut u8, pcbicon: *mut u32) -> i32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "winscard.dll""system" fn SCardGetReaderIconA ( hcontext : usize , szreadername : ::windows::core::PCSTR , pbicon : *mut u8 , pcbicon : *mut u32 ) -> i32 );
    SCardGetReaderIconA(hcontext, szreadername.into_param().abi(), pbicon, pcbicon)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[inline]
pub unsafe fn SCardGetReaderIconW<P0>(hcontext: usize, szreadername: P0, pbicon: *mut u8, pcbicon: *mut u32) -> i32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "winscard.dll""system" fn SCardGetReaderIconW ( hcontext : usize , szreadername : ::windows::core::PCWSTR , pbicon : *mut u8 , pcbicon : *mut u32 ) -> i32 );
    SCardGetReaderIconW(hcontext, szreadername.into_param().abi(), pbicon, pcbicon)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[inline]
pub unsafe fn SCardGetStatusChangeA(hcontext: usize, dwtimeout: u32, rgreaderstates: *mut SCARD_READERSTATEA, creaders: u32) -> i32 {
    ::windows_targets::link ! ( "winscard.dll""system" fn SCardGetStatusChangeA ( hcontext : usize , dwtimeout : u32 , rgreaderstates : *mut SCARD_READERSTATEA , creaders : u32 ) -> i32 );
    SCardGetStatusChangeA(hcontext, dwtimeout, rgreaderstates, creaders)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[inline]
pub unsafe fn SCardGetStatusChangeW(hcontext: usize, dwtimeout: u32, rgreaderstates: *mut SCARD_READERSTATEW, creaders: u32) -> i32 {
    ::windows_targets::link ! ( "winscard.dll""system" fn SCardGetStatusChangeW ( hcontext : usize , dwtimeout : u32 , rgreaderstates : *mut SCARD_READERSTATEW , creaders : u32 ) -> i32 );
    SCardGetStatusChangeW(hcontext, dwtimeout, rgreaderstates, creaders)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[inline]
pub unsafe fn SCardGetTransmitCount(hcard: usize, pctransmitcount: *mut u32) -> i32 {
    ::windows_targets::link ! ( "winscard.dll""system" fn SCardGetTransmitCount ( hcard : usize , pctransmitcount : *mut u32 ) -> i32 );
    SCardGetTransmitCount(hcard, pctransmitcount)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[inline]
pub unsafe fn SCardIntroduceCardTypeA<P0>(hcontext: usize, szcardname: P0, pguidprimaryprovider: ::core::option::Option<*const ::windows::core::GUID>, rgguidinterfaces: ::core::option::Option<*const ::windows::core::GUID>, dwinterfacecount: u32, pbatr: *const u8, pbatrmask: *const u8, cbatrlen: u32) -> i32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "winscard.dll""system" fn SCardIntroduceCardTypeA ( hcontext : usize , szcardname : ::windows::core::PCSTR , pguidprimaryprovider : *const ::windows::core::GUID , rgguidinterfaces : *const ::windows::core::GUID , dwinterfacecount : u32 , pbatr : *const u8 , pbatrmask : *const u8 , cbatrlen : u32 ) -> i32 );
    SCardIntroduceCardTypeA(hcontext, szcardname.into_param().abi(), ::core::mem::transmute(pguidprimaryprovider.unwrap_or(::std::ptr::null())), ::core::mem::transmute(rgguidinterfaces.unwrap_or(::std::ptr::null())), dwinterfacecount, pbatr, pbatrmask, cbatrlen)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[inline]
pub unsafe fn SCardIntroduceCardTypeW<P0>(hcontext: usize, szcardname: P0, pguidprimaryprovider: ::core::option::Option<*const ::windows::core::GUID>, rgguidinterfaces: ::core::option::Option<*const ::windows::core::GUID>, dwinterfacecount: u32, pbatr: *const u8, pbatrmask: *const u8, cbatrlen: u32) -> i32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "winscard.dll""system" fn SCardIntroduceCardTypeW ( hcontext : usize , szcardname : ::windows::core::PCWSTR , pguidprimaryprovider : *const ::windows::core::GUID , rgguidinterfaces : *const ::windows::core::GUID , dwinterfacecount : u32 , pbatr : *const u8 , pbatrmask : *const u8 , cbatrlen : u32 ) -> i32 );
    SCardIntroduceCardTypeW(hcontext, szcardname.into_param().abi(), ::core::mem::transmute(pguidprimaryprovider.unwrap_or(::std::ptr::null())), ::core::mem::transmute(rgguidinterfaces.unwrap_or(::std::ptr::null())), dwinterfacecount, pbatr, pbatrmask, cbatrlen)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[inline]
pub unsafe fn SCardIntroduceReaderA<P0, P1>(hcontext: usize, szreadername: P0, szdevicename: P1) -> i32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "winscard.dll""system" fn SCardIntroduceReaderA ( hcontext : usize , szreadername : ::windows::core::PCSTR , szdevicename : ::windows::core::PCSTR ) -> i32 );
    SCardIntroduceReaderA(hcontext, szreadername.into_param().abi(), szdevicename.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[inline]
pub unsafe fn SCardIntroduceReaderGroupA<P0>(hcontext: usize, szgroupname: P0) -> i32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "winscard.dll""system" fn SCardIntroduceReaderGroupA ( hcontext : usize , szgroupname : ::windows::core::PCSTR ) -> i32 );
    SCardIntroduceReaderGroupA(hcontext, szgroupname.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[inline]
pub unsafe fn SCardIntroduceReaderGroupW<P0>(hcontext: usize, szgroupname: P0) -> i32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "winscard.dll""system" fn SCardIntroduceReaderGroupW ( hcontext : usize , szgroupname : ::windows::core::PCWSTR ) -> i32 );
    SCardIntroduceReaderGroupW(hcontext, szgroupname.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[inline]
pub unsafe fn SCardIntroduceReaderW<P0, P1>(hcontext: usize, szreadername: P0, szdevicename: P1) -> i32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "winscard.dll""system" fn SCardIntroduceReaderW ( hcontext : usize , szreadername : ::windows::core::PCWSTR , szdevicename : ::windows::core::PCWSTR ) -> i32 );
    SCardIntroduceReaderW(hcontext, szreadername.into_param().abi(), szdevicename.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[inline]
pub unsafe fn SCardIsValidContext(hcontext: usize) -> i32 {
    ::windows_targets::link ! ( "winscard.dll""system" fn SCardIsValidContext ( hcontext : usize ) -> i32 );
    SCardIsValidContext(hcontext)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[inline]
pub unsafe fn SCardListCardsA(hcontext: usize, pbatr: ::core::option::Option<*const u8>, rgquidinterfaces: ::core::option::Option<&[::windows::core::GUID]>, mszcards: ::windows::core::PSTR, pcchcards: *mut u32) -> i32 {
    ::windows_targets::link ! ( "winscard.dll""system" fn SCardListCardsA ( hcontext : usize , pbatr : *const u8 , rgquidinterfaces : *const ::windows::core::GUID , cguidinterfacecount : u32 , mszcards : ::windows::core::PSTR , pcchcards : *mut u32 ) -> i32 );
    SCardListCardsA(hcontext, ::core::mem::transmute(pbatr.unwrap_or(::std::ptr::null())), ::core::mem::transmute(rgquidinterfaces.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), rgquidinterfaces.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(mszcards), pcchcards)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[inline]
pub unsafe fn SCardListCardsW(hcontext: usize, pbatr: ::core::option::Option<*const u8>, rgquidinterfaces: ::core::option::Option<&[::windows::core::GUID]>, mszcards: ::windows::core::PWSTR, pcchcards: *mut u32) -> i32 {
    ::windows_targets::link ! ( "winscard.dll""system" fn SCardListCardsW ( hcontext : usize , pbatr : *const u8 , rgquidinterfaces : *const ::windows::core::GUID , cguidinterfacecount : u32 , mszcards : ::windows::core::PWSTR , pcchcards : *mut u32 ) -> i32 );
    SCardListCardsW(hcontext, ::core::mem::transmute(pbatr.unwrap_or(::std::ptr::null())), ::core::mem::transmute(rgquidinterfaces.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), rgquidinterfaces.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(mszcards), pcchcards)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[inline]
pub unsafe fn SCardListInterfacesA<P0>(hcontext: usize, szcard: P0, pguidinterfaces: *mut ::windows::core::GUID, pcguidinterfaces: *mut u32) -> i32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "winscard.dll""system" fn SCardListInterfacesA ( hcontext : usize , szcard : ::windows::core::PCSTR , pguidinterfaces : *mut ::windows::core::GUID , pcguidinterfaces : *mut u32 ) -> i32 );
    SCardListInterfacesA(hcontext, szcard.into_param().abi(), pguidinterfaces, pcguidinterfaces)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[inline]
pub unsafe fn SCardListInterfacesW<P0>(hcontext: usize, szcard: P0, pguidinterfaces: *mut ::windows::core::GUID, pcguidinterfaces: *mut u32) -> i32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "winscard.dll""system" fn SCardListInterfacesW ( hcontext : usize , szcard : ::windows::core::PCWSTR , pguidinterfaces : *mut ::windows::core::GUID , pcguidinterfaces : *mut u32 ) -> i32 );
    SCardListInterfacesW(hcontext, szcard.into_param().abi(), pguidinterfaces, pcguidinterfaces)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[inline]
pub unsafe fn SCardListReaderGroupsA(hcontext: usize, mszgroups: ::windows::core::PSTR, pcchgroups: *mut u32) -> i32 {
    ::windows_targets::link ! ( "winscard.dll""system" fn SCardListReaderGroupsA ( hcontext : usize , mszgroups : ::windows::core::PSTR , pcchgroups : *mut u32 ) -> i32 );
    SCardListReaderGroupsA(hcontext, ::core::mem::transmute(mszgroups), pcchgroups)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[inline]
pub unsafe fn SCardListReaderGroupsW(hcontext: usize, mszgroups: ::windows::core::PWSTR, pcchgroups: *mut u32) -> i32 {
    ::windows_targets::link ! ( "winscard.dll""system" fn SCardListReaderGroupsW ( hcontext : usize , mszgroups : ::windows::core::PWSTR , pcchgroups : *mut u32 ) -> i32 );
    SCardListReaderGroupsW(hcontext, ::core::mem::transmute(mszgroups), pcchgroups)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[inline]
pub unsafe fn SCardListReadersA<P0>(hcontext: usize, mszgroups: P0, mszreaders: ::windows::core::PSTR, pcchreaders: *mut u32) -> i32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "winscard.dll""system" fn SCardListReadersA ( hcontext : usize , mszgroups : ::windows::core::PCSTR , mszreaders : ::windows::core::PSTR , pcchreaders : *mut u32 ) -> i32 );
    SCardListReadersA(hcontext, mszgroups.into_param().abi(), ::core::mem::transmute(mszreaders), pcchreaders)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[inline]
pub unsafe fn SCardListReadersW<P0>(hcontext: usize, mszgroups: P0, mszreaders: ::windows::core::PWSTR, pcchreaders: *mut u32) -> i32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "winscard.dll""system" fn SCardListReadersW ( hcontext : usize , mszgroups : ::windows::core::PCWSTR , mszreaders : ::windows::core::PWSTR , pcchreaders : *mut u32 ) -> i32 );
    SCardListReadersW(hcontext, mszgroups.into_param().abi(), ::core::mem::transmute(mszreaders), pcchreaders)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[inline]
pub unsafe fn SCardListReadersWithDeviceInstanceIdA<P0>(hcontext: usize, szdeviceinstanceid: P0, mszreaders: ::windows::core::PSTR, pcchreaders: *mut u32) -> i32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "winscard.dll""system" fn SCardListReadersWithDeviceInstanceIdA ( hcontext : usize , szdeviceinstanceid : ::windows::core::PCSTR , mszreaders : ::windows::core::PSTR , pcchreaders : *mut u32 ) -> i32 );
    SCardListReadersWithDeviceInstanceIdA(hcontext, szdeviceinstanceid.into_param().abi(), ::core::mem::transmute(mszreaders), pcchreaders)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[inline]
pub unsafe fn SCardListReadersWithDeviceInstanceIdW<P0>(hcontext: usize, szdeviceinstanceid: P0, mszreaders: ::windows::core::PWSTR, pcchreaders: *mut u32) -> i32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "winscard.dll""system" fn SCardListReadersWithDeviceInstanceIdW ( hcontext : usize , szdeviceinstanceid : ::windows::core::PCWSTR , mszreaders : ::windows::core::PWSTR , pcchreaders : *mut u32 ) -> i32 );
    SCardListReadersWithDeviceInstanceIdW(hcontext, szdeviceinstanceid.into_param().abi(), ::core::mem::transmute(mszreaders), pcchreaders)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[inline]
pub unsafe fn SCardLocateCardsA<P0>(hcontext: usize, mszcards: P0, rgreaderstates: *mut SCARD_READERSTATEA, creaders: u32) -> i32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "winscard.dll""system" fn SCardLocateCardsA ( hcontext : usize , mszcards : ::windows::core::PCSTR , rgreaderstates : *mut SCARD_READERSTATEA , creaders : u32 ) -> i32 );
    SCardLocateCardsA(hcontext, mszcards.into_param().abi(), rgreaderstates, creaders)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[inline]
pub unsafe fn SCardLocateCardsByATRA(hcontext: usize, rgatrmasks: *const SCARD_ATRMASK, catrs: u32, rgreaderstates: *mut SCARD_READERSTATEA, creaders: u32) -> i32 {
    ::windows_targets::link ! ( "winscard.dll""system" fn SCardLocateCardsByATRA ( hcontext : usize , rgatrmasks : *const SCARD_ATRMASK , catrs : u32 , rgreaderstates : *mut SCARD_READERSTATEA , creaders : u32 ) -> i32 );
    SCardLocateCardsByATRA(hcontext, rgatrmasks, catrs, rgreaderstates, creaders)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[inline]
pub unsafe fn SCardLocateCardsByATRW(hcontext: usize, rgatrmasks: *const SCARD_ATRMASK, catrs: u32, rgreaderstates: *mut SCARD_READERSTATEW, creaders: u32) -> i32 {
    ::windows_targets::link ! ( "winscard.dll""system" fn SCardLocateCardsByATRW ( hcontext : usize , rgatrmasks : *const SCARD_ATRMASK , catrs : u32 , rgreaderstates : *mut SCARD_READERSTATEW , creaders : u32 ) -> i32 );
    SCardLocateCardsByATRW(hcontext, rgatrmasks, catrs, rgreaderstates, creaders)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[inline]
pub unsafe fn SCardLocateCardsW<P0>(hcontext: usize, mszcards: P0, rgreaderstates: *mut SCARD_READERSTATEW, creaders: u32) -> i32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "winscard.dll""system" fn SCardLocateCardsW ( hcontext : usize , mszcards : ::windows::core::PCWSTR , rgreaderstates : *mut SCARD_READERSTATEW , creaders : u32 ) -> i32 );
    SCardLocateCardsW(hcontext, mszcards.into_param().abi(), rgreaderstates, creaders)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[inline]
pub unsafe fn SCardReadCacheA<P0>(hcontext: usize, cardidentifier: *const ::windows::core::GUID, freshnesscounter: u32, lookupname: P0, data: *mut u8, datalen: *mut u32) -> i32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "winscard.dll""system" fn SCardReadCacheA ( hcontext : usize , cardidentifier : *const ::windows::core::GUID , freshnesscounter : u32 , lookupname : ::windows::core::PCSTR , data : *mut u8 , datalen : *mut u32 ) -> i32 );
    SCardReadCacheA(hcontext, cardidentifier, freshnesscounter, lookupname.into_param().abi(), data, datalen)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[inline]
pub unsafe fn SCardReadCacheW<P0>(hcontext: usize, cardidentifier: *const ::windows::core::GUID, freshnesscounter: u32, lookupname: P0, data: *mut u8, datalen: *mut u32) -> i32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "winscard.dll""system" fn SCardReadCacheW ( hcontext : usize , cardidentifier : *const ::windows::core::GUID , freshnesscounter : u32 , lookupname : ::windows::core::PCWSTR , data : *mut u8 , datalen : *mut u32 ) -> i32 );
    SCardReadCacheW(hcontext, cardidentifier, freshnesscounter, lookupname.into_param().abi(), data, datalen)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[inline]
pub unsafe fn SCardReconnect(hcard: usize, dwsharemode: u32, dwpreferredprotocols: u32, dwinitialization: u32, pdwactiveprotocol: ::core::option::Option<*mut u32>) -> i32 {
    ::windows_targets::link ! ( "winscard.dll""system" fn SCardReconnect ( hcard : usize , dwsharemode : u32 , dwpreferredprotocols : u32 , dwinitialization : u32 , pdwactiveprotocol : *mut u32 ) -> i32 );
    SCardReconnect(hcard, dwsharemode, dwpreferredprotocols, dwinitialization, ::core::mem::transmute(pdwactiveprotocol.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[inline]
pub unsafe fn SCardReleaseContext(hcontext: usize) -> i32 {
    ::windows_targets::link ! ( "winscard.dll""system" fn SCardReleaseContext ( hcontext : usize ) -> i32 );
    SCardReleaseContext(hcontext)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[inline]
pub unsafe fn SCardReleaseStartedEvent() {
    ::windows_targets::link ! ( "winscard.dll""system" fn SCardReleaseStartedEvent ( ) -> ( ) );
    SCardReleaseStartedEvent()
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[inline]
pub unsafe fn SCardRemoveReaderFromGroupA<P0, P1>(hcontext: usize, szreadername: P0, szgroupname: P1) -> i32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "winscard.dll""system" fn SCardRemoveReaderFromGroupA ( hcontext : usize , szreadername : ::windows::core::PCSTR , szgroupname : ::windows::core::PCSTR ) -> i32 );
    SCardRemoveReaderFromGroupA(hcontext, szreadername.into_param().abi(), szgroupname.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[inline]
pub unsafe fn SCardRemoveReaderFromGroupW<P0, P1>(hcontext: usize, szreadername: P0, szgroupname: P1) -> i32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "winscard.dll""system" fn SCardRemoveReaderFromGroupW ( hcontext : usize , szreadername : ::windows::core::PCWSTR , szgroupname : ::windows::core::PCWSTR ) -> i32 );
    SCardRemoveReaderFromGroupW(hcontext, szreadername.into_param().abi(), szgroupname.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[inline]
pub unsafe fn SCardSetAttrib(hcard: usize, dwattrid: u32, pbattr: &[u8]) -> i32 {
    ::windows_targets::link ! ( "winscard.dll""system" fn SCardSetAttrib ( hcard : usize , dwattrid : u32 , pbattr : *const u8 , cbattrlen : u32 ) -> i32 );
    SCardSetAttrib(hcard, dwattrid, ::core::mem::transmute(pbattr.as_ptr()), pbattr.len() as _)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[inline]
pub unsafe fn SCardSetCardTypeProviderNameA<P0, P1>(hcontext: usize, szcardname: P0, dwproviderid: u32, szprovider: P1) -> i32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "winscard.dll""system" fn SCardSetCardTypeProviderNameA ( hcontext : usize , szcardname : ::windows::core::PCSTR , dwproviderid : u32 , szprovider : ::windows::core::PCSTR ) -> i32 );
    SCardSetCardTypeProviderNameA(hcontext, szcardname.into_param().abi(), dwproviderid, szprovider.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[inline]
pub unsafe fn SCardSetCardTypeProviderNameW<P0, P1>(hcontext: usize, szcardname: P0, dwproviderid: u32, szprovider: P1) -> i32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "winscard.dll""system" fn SCardSetCardTypeProviderNameW ( hcontext : usize , szcardname : ::windows::core::PCWSTR , dwproviderid : u32 , szprovider : ::windows::core::PCWSTR ) -> i32 );
    SCardSetCardTypeProviderNameW(hcontext, szcardname.into_param().abi(), dwproviderid, szprovider.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[inline]
pub unsafe fn SCardState(hcard: usize, pdwstate: *mut u32, pdwprotocol: *mut u32, pbatr: *mut u8, pcbatrlen: *mut u32) -> i32 {
    ::windows_targets::link ! ( "winscard.dll""system" fn SCardState ( hcard : usize , pdwstate : *mut u32 , pdwprotocol : *mut u32 , pbatr : *mut u8 , pcbatrlen : *mut u32 ) -> i32 );
    SCardState(hcard, pdwstate, pdwprotocol, pbatr, pcbatrlen)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[inline]
pub unsafe fn SCardStatusA(hcard: usize, mszreadernames: ::windows::core::PSTR, pcchreaderlen: ::core::option::Option<*mut u32>, pdwstate: ::core::option::Option<*mut u32>, pdwprotocol: ::core::option::Option<*mut u32>, pbatr: ::core::option::Option<*mut u8>, pcbatrlen: ::core::option::Option<*mut u32>) -> i32 {
    ::windows_targets::link ! ( "winscard.dll""system" fn SCardStatusA ( hcard : usize , mszreadernames : ::windows::core::PSTR , pcchreaderlen : *mut u32 , pdwstate : *mut u32 , pdwprotocol : *mut u32 , pbatr : *mut u8 , pcbatrlen : *mut u32 ) -> i32 );
    SCardStatusA(hcard, ::core::mem::transmute(mszreadernames), ::core::mem::transmute(pcchreaderlen.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pdwstate.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pdwprotocol.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pbatr.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pcbatrlen.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[inline]
pub unsafe fn SCardStatusW(hcard: usize, mszreadernames: ::windows::core::PWSTR, pcchreaderlen: ::core::option::Option<*mut u32>, pdwstate: ::core::option::Option<*mut u32>, pdwprotocol: ::core::option::Option<*mut u32>, pbatr: ::core::option::Option<*mut u8>, pcbatrlen: ::core::option::Option<*mut u32>) -> i32 {
    ::windows_targets::link ! ( "winscard.dll""system" fn SCardStatusW ( hcard : usize , mszreadernames : ::windows::core::PWSTR , pcchreaderlen : *mut u32 , pdwstate : *mut u32 , pdwprotocol : *mut u32 , pbatr : *mut u8 , pcbatrlen : *mut u32 ) -> i32 );
    SCardStatusW(hcard, ::core::mem::transmute(mszreadernames), ::core::mem::transmute(pcchreaderlen.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pdwstate.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pdwprotocol.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pbatr.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pcbatrlen.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[inline]
pub unsafe fn SCardTransmit(hcard: usize, piosendpci: *const SCARD_IO_REQUEST, pbsendbuffer: &[u8], piorecvpci: ::core::option::Option<*mut SCARD_IO_REQUEST>, pbrecvbuffer: *mut u8, pcbrecvlength: *mut u32) -> i32 {
    ::windows_targets::link ! ( "winscard.dll""system" fn SCardTransmit ( hcard : usize , piosendpci : *const SCARD_IO_REQUEST , pbsendbuffer : *const u8 , cbsendlength : u32 , piorecvpci : *mut SCARD_IO_REQUEST , pbrecvbuffer : *mut u8 , pcbrecvlength : *mut u32 ) -> i32 );
    SCardTransmit(hcard, piosendpci, ::core::mem::transmute(pbsendbuffer.as_ptr()), pbsendbuffer.len() as _, ::core::mem::transmute(piorecvpci.unwrap_or(::std::ptr::null_mut())), pbrecvbuffer, pcbrecvlength)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn SCardUIDlgSelectCardA(param0: *mut OPENCARDNAME_EXA) -> i32 {
    ::windows_targets::link ! ( "scarddlg.dll""system" fn SCardUIDlgSelectCardA ( param0 : *mut OPENCARDNAME_EXA ) -> i32 );
    SCardUIDlgSelectCardA(param0)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn SCardUIDlgSelectCardW(param0: *mut OPENCARDNAME_EXW) -> i32 {
    ::windows_targets::link ! ( "scarddlg.dll""system" fn SCardUIDlgSelectCardW ( param0 : *mut OPENCARDNAME_EXW ) -> i32 );
    SCardUIDlgSelectCardW(param0)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[inline]
pub unsafe fn SCardWriteCacheA<P0>(hcontext: usize, cardidentifier: *const ::windows::core::GUID, freshnesscounter: u32, lookupname: P0, data: &[u8]) -> i32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "winscard.dll""system" fn SCardWriteCacheA ( hcontext : usize , cardidentifier : *const ::windows::core::GUID , freshnesscounter : u32 , lookupname : ::windows::core::PCSTR , data : *const u8 , datalen : u32 ) -> i32 );
    SCardWriteCacheA(hcontext, cardidentifier, freshnesscounter, lookupname.into_param().abi(), ::core::mem::transmute(data.as_ptr()), data.len() as _)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[inline]
pub unsafe fn SCardWriteCacheW<P0>(hcontext: usize, cardidentifier: *const ::windows::core::GUID, freshnesscounter: u32, lookupname: P0, data: &[u8]) -> i32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "winscard.dll""system" fn SCardWriteCacheW ( hcontext : usize , cardidentifier : *const ::windows::core::GUID , freshnesscounter : u32 , lookupname : ::windows::core::PCWSTR , data : *const u8 , datalen : u32 ) -> i32 );
    SCardWriteCacheW(hcontext, cardidentifier, freshnesscounter, lookupname.into_param().abi(), ::core::mem::transmute(data.as_ptr()), data.len() as _)
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CERT_HASH_LENGTH: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CREDSSP_CRED_EX_VERSION: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CREDSSP_FLAG_REDIRECT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CREDSSP_NAME: ::windows::core::PCWSTR = ::windows::core::w!("CREDSSP");
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CREDSSP_SERVER_AUTH_CERTIFICATE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CREDSSP_SERVER_AUTH_LOOPBACK: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CREDSSP_SERVER_AUTH_NEGOTIATE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CREDUIWIN_DOWNLEVEL_HELLO_AS_SMART_CARD: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CREDUIWIN_IGNORE_CLOUDAUTHORITY_NAME: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CREDUI_MAX_CAPTION_LENGTH: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CREDUI_MAX_DOMAIN_TARGET_LENGTH: u32 = 337u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CREDUI_MAX_GENERIC_TARGET_LENGTH: u32 = 32767u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CREDUI_MAX_MESSAGE_LENGTH: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CREDUI_MAX_USERNAME_LENGTH: u32 = 513u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_ALLOW_NAME_RESOLUTION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_CACHE_TARGET_INFORMATION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_LOGON_TYPES_MASK: u32 = 61440u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_MAX_ATTRIBUTES: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_MAX_CREDENTIAL_BLOB_SIZE: u32 = 2560u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_MAX_DOMAIN_TARGET_NAME_LENGTH: u32 = 337u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_MAX_GENERIC_TARGET_NAME_LENGTH: u32 = 32767u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_MAX_STRING_LENGTH: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_MAX_TARGETNAME_ATTRIBUTE_LENGTH: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_MAX_TARGETNAME_NAMESPACE_LENGTH: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_MAX_USERNAME_LENGTH: u32 = 513u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_MAX_VALUE_SIZE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_PRESERVE_CREDENTIAL_BLOB: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_PROTECT_AS_SELF: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_PROTECT_TO_SYSTEM: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_SESSION_WILDCARD_NAME: ::windows::core::PCWSTR = ::windows::core::w!("*Session");
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_SESSION_WILDCARD_NAME_A: ::windows::core::PCSTR = ::windows::core::s!("*Session");
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_SESSION_WILDCARD_NAME_W: ::windows::core::PCWSTR = ::windows::core::w!("*Session");
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_TARGETNAME_ATTRIBUTE_BATCH: ::windows::core::PCWSTR = ::windows::core::w!("batch");
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_TARGETNAME_ATTRIBUTE_BATCH_A: ::windows::core::PCSTR = ::windows::core::s!("batch");
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_TARGETNAME_ATTRIBUTE_BATCH_W: ::windows::core::PCWSTR = ::windows::core::w!("batch");
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_TARGETNAME_ATTRIBUTE_CACHEDINTERACTIVE: ::windows::core::PCWSTR = ::windows::core::w!("cachedinteractive");
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_TARGETNAME_ATTRIBUTE_CACHEDINTERACTIVE_A: ::windows::core::PCSTR = ::windows::core::s!("cachedinteractive");
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_TARGETNAME_ATTRIBUTE_CACHEDINTERACTIVE_W: ::windows::core::PCWSTR = ::windows::core::w!("cachedinteractive");
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_TARGETNAME_ATTRIBUTE_INTERACTIVE: ::windows::core::PCWSTR = ::windows::core::w!("interactive");
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_TARGETNAME_ATTRIBUTE_INTERACTIVE_A: ::windows::core::PCSTR = ::windows::core::s!("interactive");
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_TARGETNAME_ATTRIBUTE_INTERACTIVE_W: ::windows::core::PCWSTR = ::windows::core::w!("interactive");
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_TARGETNAME_ATTRIBUTE_NAME: ::windows::core::PCWSTR = ::windows::core::w!("name");
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_TARGETNAME_ATTRIBUTE_NAME_A: ::windows::core::PCSTR = ::windows::core::s!("name");
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_TARGETNAME_ATTRIBUTE_NAME_W: ::windows::core::PCWSTR = ::windows::core::w!("name");
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_TARGETNAME_ATTRIBUTE_NETWORK: ::windows::core::PCWSTR = ::windows::core::w!("network");
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_TARGETNAME_ATTRIBUTE_NETWORKCLEARTEXT: ::windows::core::PCWSTR = ::windows::core::w!("networkcleartext");
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_TARGETNAME_ATTRIBUTE_NETWORKCLEARTEXT_A: ::windows::core::PCSTR = ::windows::core::s!("networkcleartext");
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_TARGETNAME_ATTRIBUTE_NETWORKCLEARTEXT_W: ::windows::core::PCWSTR = ::windows::core::w!("networkcleartext");
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_TARGETNAME_ATTRIBUTE_NETWORK_A: ::windows::core::PCSTR = ::windows::core::s!("network");
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_TARGETNAME_ATTRIBUTE_NETWORK_W: ::windows::core::PCWSTR = ::windows::core::w!("network");
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_TARGETNAME_ATTRIBUTE_REMOTEINTERACTIVE: ::windows::core::PCWSTR = ::windows::core::w!("remoteinteractive");
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_TARGETNAME_ATTRIBUTE_REMOTEINTERACTIVE_A: ::windows::core::PCSTR = ::windows::core::s!("remoteinteractive");
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_TARGETNAME_ATTRIBUTE_REMOTEINTERACTIVE_W: ::windows::core::PCWSTR = ::windows::core::w!("remoteinteractive");
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_TARGETNAME_ATTRIBUTE_SERVICE: ::windows::core::PCWSTR = ::windows::core::w!("service");
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_TARGETNAME_ATTRIBUTE_SERVICE_A: ::windows::core::PCSTR = ::windows::core::s!("service");
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_TARGETNAME_ATTRIBUTE_SERVICE_W: ::windows::core::PCWSTR = ::windows::core::w!("service");
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_TARGETNAME_ATTRIBUTE_TARGET: ::windows::core::PCWSTR = ::windows::core::w!("target");
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_TARGETNAME_ATTRIBUTE_TARGET_A: ::windows::core::PCSTR = ::windows::core::s!("target");
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_TARGETNAME_ATTRIBUTE_TARGET_W: ::windows::core::PCWSTR = ::windows::core::w!("target");
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_TARGETNAME_DOMAIN_NAMESPACE: ::windows::core::PCWSTR = ::windows::core::w!("Domain");
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_TARGETNAME_DOMAIN_NAMESPACE_A: ::windows::core::PCSTR = ::windows::core::s!("Domain");
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_TARGETNAME_DOMAIN_NAMESPACE_W: ::windows::core::PCWSTR = ::windows::core::w!("Domain");
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_TARGETNAME_LEGACYGENERIC_NAMESPACE_A: ::windows::core::PCSTR = ::windows::core::s!("LegacyGeneric");
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_TARGETNAME_LEGACYGENERIC_NAMESPACE_W: ::windows::core::PCWSTR = ::windows::core::w!("LegacyGeneric");
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
pub const CRED_UNPROTECT_ALLOW_TO_SYSTEM: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_UNPROTECT_AS_SELF: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const FILE_DEVICE_SMARTCARD: u32 = 49u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const GUID_DEVINTERFACE_SMARTCARD_READER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x50dd5230_ba8a_11d1_bf5d_0000f805f530);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const MAXIMUM_ATTR_STRING_LENGTH: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const MAXIMUM_SMARTCARD_READERS: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_ABSENT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_ALL_READERS: ::windows::core::PCWSTR = ::windows::core::w!("SCard$AllReaders\u{0}00");
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
pub const SCARD_DEFAULT_READERS: ::windows::core::PCWSTR = ::windows::core::w!("SCard$DefaultReaders\u{0}00");
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_EJECT_CARD: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_LEAVE_CARD: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_LOCAL_READERS: ::windows::core::PCWSTR = ::windows::core::w!("SCard$LocalReaders\u{0}00");
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
pub const SCARD_STATE_UNPOWERED: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_SWALLOWED: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_SYSTEM_READERS: ::windows::core::PCWSTR = ::windows::core::w!("SCard$SystemReaders\u{0}00");
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_T0_CMD_LENGTH: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_T0_HEADER_LENGTH: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_T1_EPILOGUE_LENGTH: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_T1_EPILOGUE_LENGTH_LRC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_T1_MAX_IFS: u32 = 254u32;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_T1_PROLOGUE_LENGTH: u32 = 3u32;
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
pub const STATUS_ACCOUNT_DISABLED: super::super::Foundation::NTSTATUS = super::super::Foundation::NTSTATUS(-1073741710i32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const STATUS_ACCOUNT_EXPIRED: super::super::Foundation::NTSTATUS = super::super::Foundation::NTSTATUS(-1073741421i32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const STATUS_ACCOUNT_LOCKED_OUT: super::super::Foundation::NTSTATUS = super::super::Foundation::NTSTATUS(-1073741260i32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const STATUS_ACCOUNT_RESTRICTION: super::super::Foundation::NTSTATUS = super::super::Foundation::NTSTATUS(-1073741714i32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const STATUS_AUTHENTICATION_FIREWALL_FAILED: super::super::Foundation::NTSTATUS = super::super::Foundation::NTSTATUS(-1073740781i32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const STATUS_DOWNGRADE_DETECTED: super::super::Foundation::NTSTATUS = super::super::Foundation::NTSTATUS(-1073740920i32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const STATUS_LOGON_FAILURE: super::super::Foundation::NTSTATUS = super::super::Foundation::NTSTATUS(-1073741715i32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const STATUS_LOGON_TYPE_NOT_GRANTED: super::super::Foundation::NTSTATUS = super::super::Foundation::NTSTATUS(-1073741477i32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const STATUS_NO_SUCH_LOGON_SESSION: super::super::Foundation::NTSTATUS = super::super::Foundation::NTSTATUS(-1073741729i32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const STATUS_NO_SUCH_USER: super::super::Foundation::NTSTATUS = super::super::Foundation::NTSTATUS(-1073741724i32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const STATUS_PASSWORD_EXPIRED: super::super::Foundation::NTSTATUS = super::super::Foundation::NTSTATUS(-1073741711i32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const STATUS_PASSWORD_MUST_CHANGE: super::super::Foundation::NTSTATUS = super::super::Foundation::NTSTATUS(-1073741276i32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const STATUS_WRONG_PASSWORD: super::super::Foundation::NTSTATUS = super::super::Foundation::NTSTATUS(-1073741718i32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const TS_SSP_NAME: ::windows::core::PCWSTR = ::windows::core::w!("TSSSP");
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const TS_SSP_NAME_A: ::windows::core::PCSTR = ::windows::core::s!("TSSSP");
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const szOID_TS_KP_TS_SERVER_AUTH: ::windows::core::PCSTR = ::windows::core::s!("1.3.6.1.4.1.311.54.1.2");
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CREDSPP_SUBMIT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CredsspPasswordCreds: CREDSPP_SUBMIT_TYPE = CREDSPP_SUBMIT_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CredsspSchannelCreds: CREDSPP_SUBMIT_TYPE = CREDSPP_SUBMIT_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CredsspCertificateCreds: CREDSPP_SUBMIT_TYPE = CREDSPP_SUBMIT_TYPE(13i32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CredsspSubmitBufferBoth: CREDSPP_SUBMIT_TYPE = CREDSPP_SUBMIT_TYPE(50i32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CredsspSubmitBufferBothOld: CREDSPP_SUBMIT_TYPE = CREDSPP_SUBMIT_TYPE(51i32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CredsspCredEx: CREDSPP_SUBMIT_TYPE = CREDSPP_SUBMIT_TYPE(100i32);
impl ::core::marker::Copy for CREDSPP_SUBMIT_TYPE {}
impl ::core::clone::Clone for CREDSPP_SUBMIT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CREDSPP_SUBMIT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CREDSPP_SUBMIT_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CREDSPP_SUBMIT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CREDSPP_SUBMIT_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CREDUIWIN_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CREDUIWIN_GENERIC: CREDUIWIN_FLAGS = CREDUIWIN_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CREDUIWIN_CHECKBOX: CREDUIWIN_FLAGS = CREDUIWIN_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CREDUIWIN_AUTHPACKAGE_ONLY: CREDUIWIN_FLAGS = CREDUIWIN_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CREDUIWIN_IN_CRED_ONLY: CREDUIWIN_FLAGS = CREDUIWIN_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CREDUIWIN_ENUMERATE_ADMINS: CREDUIWIN_FLAGS = CREDUIWIN_FLAGS(256u32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CREDUIWIN_ENUMERATE_CURRENT_USER: CREDUIWIN_FLAGS = CREDUIWIN_FLAGS(512u32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CREDUIWIN_SECURE_PROMPT: CREDUIWIN_FLAGS = CREDUIWIN_FLAGS(4096u32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CREDUIWIN_PREPROMPTING: CREDUIWIN_FLAGS = CREDUIWIN_FLAGS(8192u32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CREDUIWIN_PACK_32_WOW: CREDUIWIN_FLAGS = CREDUIWIN_FLAGS(268435456u32);
impl ::core::marker::Copy for CREDUIWIN_FLAGS {}
impl ::core::clone::Clone for CREDUIWIN_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CREDUIWIN_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CREDUIWIN_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CREDUIWIN_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CREDUIWIN_FLAGS").field(&self.0).finish()
    }
}
impl CREDUIWIN_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for CREDUIWIN_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CREDUIWIN_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CREDUIWIN_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CREDUIWIN_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CREDUIWIN_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CREDUI_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CREDUI_FLAGS_ALWAYS_SHOW_UI: CREDUI_FLAGS = CREDUI_FLAGS(128u32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CREDUI_FLAGS_COMPLETE_USERNAME: CREDUI_FLAGS = CREDUI_FLAGS(2048u32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CREDUI_FLAGS_DO_NOT_PERSIST: CREDUI_FLAGS = CREDUI_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CREDUI_FLAGS_EXCLUDE_CERTIFICATES: CREDUI_FLAGS = CREDUI_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CREDUI_FLAGS_EXPECT_CONFIRMATION: CREDUI_FLAGS = CREDUI_FLAGS(131072u32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CREDUI_FLAGS_GENERIC_CREDENTIALS: CREDUI_FLAGS = CREDUI_FLAGS(262144u32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CREDUI_FLAGS_INCORRECT_PASSWORD: CREDUI_FLAGS = CREDUI_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CREDUI_FLAGS_KEEP_USERNAME: CREDUI_FLAGS = CREDUI_FLAGS(1048576u32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CREDUI_FLAGS_PASSWORD_ONLY_OK: CREDUI_FLAGS = CREDUI_FLAGS(512u32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CREDUI_FLAGS_PERSIST: CREDUI_FLAGS = CREDUI_FLAGS(4096u32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CREDUI_FLAGS_REQUEST_ADMINISTRATOR: CREDUI_FLAGS = CREDUI_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CREDUI_FLAGS_REQUIRE_CERTIFICATE: CREDUI_FLAGS = CREDUI_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CREDUI_FLAGS_REQUIRE_SMARTCARD: CREDUI_FLAGS = CREDUI_FLAGS(256u32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CREDUI_FLAGS_SERVER_CREDENTIAL: CREDUI_FLAGS = CREDUI_FLAGS(16384u32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CREDUI_FLAGS_SHOW_SAVE_CHECK_BOX: CREDUI_FLAGS = CREDUI_FLAGS(64u32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CREDUI_FLAGS_USERNAME_TARGET_CREDENTIALS: CREDUI_FLAGS = CREDUI_FLAGS(524288u32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CREDUI_FLAGS_VALIDATE_USERNAME: CREDUI_FLAGS = CREDUI_FLAGS(1024u32);
impl ::core::marker::Copy for CREDUI_FLAGS {}
impl ::core::clone::Clone for CREDUI_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CREDUI_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CREDUI_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CREDUI_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CREDUI_FLAGS").field(&self.0).finish()
    }
}
impl CREDUI_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for CREDUI_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CREDUI_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CREDUI_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CREDUI_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CREDUI_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CRED_ENUMERATE_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_ENUMERATE_ALL_CREDENTIALS: CRED_ENUMERATE_FLAGS = CRED_ENUMERATE_FLAGS(1u32);
impl ::core::marker::Copy for CRED_ENUMERATE_FLAGS {}
impl ::core::clone::Clone for CRED_ENUMERATE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CRED_ENUMERATE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CRED_ENUMERATE_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CRED_ENUMERATE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CRED_ENUMERATE_FLAGS").field(&self.0).finish()
    }
}
impl CRED_ENUMERATE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for CRED_ENUMERATE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CRED_ENUMERATE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CRED_ENUMERATE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CRED_ENUMERATE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CRED_ENUMERATE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CRED_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_FLAGS_PASSWORD_FOR_CERT: CRED_FLAGS = CRED_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_FLAGS_PROMPT_NOW: CRED_FLAGS = CRED_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_FLAGS_USERNAME_TARGET: CRED_FLAGS = CRED_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_FLAGS_OWF_CRED_BLOB: CRED_FLAGS = CRED_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_FLAGS_REQUIRE_CONFIRMATION: CRED_FLAGS = CRED_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_FLAGS_WILDCARD_MATCH: CRED_FLAGS = CRED_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_FLAGS_VSM_PROTECTED: CRED_FLAGS = CRED_FLAGS(64u32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_FLAGS_NGC_CERT: CRED_FLAGS = CRED_FLAGS(128u32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_FLAGS_VALID_FLAGS: CRED_FLAGS = CRED_FLAGS(61695u32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_FLAGS_VALID_INPUT_FLAGS: CRED_FLAGS = CRED_FLAGS(61599u32);
impl ::core::marker::Copy for CRED_FLAGS {}
impl ::core::clone::Clone for CRED_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CRED_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CRED_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CRED_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CRED_FLAGS").field(&self.0).finish()
    }
}
impl CRED_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for CRED_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CRED_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CRED_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CRED_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CRED_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CRED_MARSHAL_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CertCredential: CRED_MARSHAL_TYPE = CRED_MARSHAL_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const UsernameTargetCredential: CRED_MARSHAL_TYPE = CRED_MARSHAL_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const BinaryBlobCredential: CRED_MARSHAL_TYPE = CRED_MARSHAL_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const UsernameForPackedCredentials: CRED_MARSHAL_TYPE = CRED_MARSHAL_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const BinaryBlobForSystem: CRED_MARSHAL_TYPE = CRED_MARSHAL_TYPE(5i32);
impl ::core::marker::Copy for CRED_MARSHAL_TYPE {}
impl ::core::clone::Clone for CRED_MARSHAL_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CRED_MARSHAL_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CRED_MARSHAL_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CRED_MARSHAL_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CRED_MARSHAL_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CRED_PACK_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_PACK_PROTECTED_CREDENTIALS: CRED_PACK_FLAGS = CRED_PACK_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_PACK_WOW_BUFFER: CRED_PACK_FLAGS = CRED_PACK_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_PACK_GENERIC_CREDENTIALS: CRED_PACK_FLAGS = CRED_PACK_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_PACK_ID_PROVIDER_CREDENTIALS: CRED_PACK_FLAGS = CRED_PACK_FLAGS(8u32);
impl ::core::marker::Copy for CRED_PACK_FLAGS {}
impl ::core::clone::Clone for CRED_PACK_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CRED_PACK_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CRED_PACK_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CRED_PACK_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CRED_PACK_FLAGS").field(&self.0).finish()
    }
}
impl CRED_PACK_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for CRED_PACK_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CRED_PACK_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CRED_PACK_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CRED_PACK_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CRED_PACK_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CRED_PERSIST(pub u32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_PERSIST_NONE: CRED_PERSIST = CRED_PERSIST(0u32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_PERSIST_SESSION: CRED_PERSIST = CRED_PERSIST(1u32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_PERSIST_LOCAL_MACHINE: CRED_PERSIST = CRED_PERSIST(2u32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_PERSIST_ENTERPRISE: CRED_PERSIST = CRED_PERSIST(3u32);
impl ::core::marker::Copy for CRED_PERSIST {}
impl ::core::clone::Clone for CRED_PERSIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CRED_PERSIST {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CRED_PERSIST {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CRED_PERSIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CRED_PERSIST").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CRED_PROTECTION_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CredUnprotected: CRED_PROTECTION_TYPE = CRED_PROTECTION_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CredUserProtection: CRED_PROTECTION_TYPE = CRED_PROTECTION_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CredTrustedProtection: CRED_PROTECTION_TYPE = CRED_PROTECTION_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CredForSystemProtection: CRED_PROTECTION_TYPE = CRED_PROTECTION_TYPE(3i32);
impl ::core::marker::Copy for CRED_PROTECTION_TYPE {}
impl ::core::clone::Clone for CRED_PROTECTION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CRED_PROTECTION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CRED_PROTECTION_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CRED_PROTECTION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CRED_PROTECTION_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CRED_TYPE(pub u32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_TYPE_GENERIC: CRED_TYPE = CRED_TYPE(1u32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_TYPE_DOMAIN_PASSWORD: CRED_TYPE = CRED_TYPE(2u32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_TYPE_DOMAIN_CERTIFICATE: CRED_TYPE = CRED_TYPE(3u32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_TYPE_DOMAIN_VISIBLE_PASSWORD: CRED_TYPE = CRED_TYPE(4u32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_TYPE_GENERIC_CERTIFICATE: CRED_TYPE = CRED_TYPE(5u32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_TYPE_DOMAIN_EXTENDED: CRED_TYPE = CRED_TYPE(6u32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_TYPE_MAXIMUM: CRED_TYPE = CRED_TYPE(7u32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const CRED_TYPE_MAXIMUM_EX: CRED_TYPE = CRED_TYPE(1007u32);
impl ::core::marker::Copy for CRED_TYPE {}
impl ::core::clone::Clone for CRED_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CRED_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CRED_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CRED_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CRED_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KeyCredentialManagerOperationErrorStates(pub i32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const KeyCredentialManagerOperationErrorStateNone: KeyCredentialManagerOperationErrorStates = KeyCredentialManagerOperationErrorStates(0i32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const KeyCredentialManagerOperationErrorStateDeviceJoinFailure: KeyCredentialManagerOperationErrorStates = KeyCredentialManagerOperationErrorStates(1i32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const KeyCredentialManagerOperationErrorStateTokenFailure: KeyCredentialManagerOperationErrorStates = KeyCredentialManagerOperationErrorStates(2i32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const KeyCredentialManagerOperationErrorStateCertificateFailure: KeyCredentialManagerOperationErrorStates = KeyCredentialManagerOperationErrorStates(4i32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const KeyCredentialManagerOperationErrorStateRemoteSessionFailure: KeyCredentialManagerOperationErrorStates = KeyCredentialManagerOperationErrorStates(8i32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const KeyCredentialManagerOperationErrorStatePolicyFailure: KeyCredentialManagerOperationErrorStates = KeyCredentialManagerOperationErrorStates(16i32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const KeyCredentialManagerOperationErrorStateHardwareFailure: KeyCredentialManagerOperationErrorStates = KeyCredentialManagerOperationErrorStates(32i32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const KeyCredentialManagerOperationErrorStatePinExistsFailure: KeyCredentialManagerOperationErrorStates = KeyCredentialManagerOperationErrorStates(64i32);
impl ::core::marker::Copy for KeyCredentialManagerOperationErrorStates {}
impl ::core::clone::Clone for KeyCredentialManagerOperationErrorStates {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KeyCredentialManagerOperationErrorStates {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for KeyCredentialManagerOperationErrorStates {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for KeyCredentialManagerOperationErrorStates {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KeyCredentialManagerOperationErrorStates").field(&self.0).finish()
    }
}
impl KeyCredentialManagerOperationErrorStates {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for KeyCredentialManagerOperationErrorStates {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for KeyCredentialManagerOperationErrorStates {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for KeyCredentialManagerOperationErrorStates {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for KeyCredentialManagerOperationErrorStates {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for KeyCredentialManagerOperationErrorStates {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KeyCredentialManagerOperationType(pub i32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const KeyCredentialManagerProvisioning: KeyCredentialManagerOperationType = KeyCredentialManagerOperationType(0i32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const KeyCredentialManagerPinChange: KeyCredentialManagerOperationType = KeyCredentialManagerOperationType(1i32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const KeyCredentialManagerPinReset: KeyCredentialManagerOperationType = KeyCredentialManagerOperationType(2i32);
impl ::core::marker::Copy for KeyCredentialManagerOperationType {}
impl ::core::clone::Clone for KeyCredentialManagerOperationType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KeyCredentialManagerOperationType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for KeyCredentialManagerOperationType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for KeyCredentialManagerOperationType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KeyCredentialManagerOperationType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct READER_SEL_REQUEST_MATCH_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const RSR_MATCH_TYPE_READER_AND_CONTAINER: READER_SEL_REQUEST_MATCH_TYPE = READER_SEL_REQUEST_MATCH_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const RSR_MATCH_TYPE_SERIAL_NUMBER: READER_SEL_REQUEST_MATCH_TYPE = READER_SEL_REQUEST_MATCH_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const RSR_MATCH_TYPE_ALL_CARDS: READER_SEL_REQUEST_MATCH_TYPE = READER_SEL_REQUEST_MATCH_TYPE(3i32);
impl ::core::marker::Copy for READER_SEL_REQUEST_MATCH_TYPE {}
impl ::core::clone::Clone for READER_SEL_REQUEST_MATCH_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for READER_SEL_REQUEST_MATCH_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for READER_SEL_REQUEST_MATCH_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for READER_SEL_REQUEST_MATCH_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("READER_SEL_REQUEST_MATCH_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SCARD_SCOPE(pub u32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_SCOPE_USER: SCARD_SCOPE = SCARD_SCOPE(0u32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_SCOPE_SYSTEM: SCARD_SCOPE = SCARD_SCOPE(2u32);
impl ::core::marker::Copy for SCARD_SCOPE {}
impl ::core::clone::Clone for SCARD_SCOPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SCARD_SCOPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SCARD_SCOPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SCARD_SCOPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCARD_SCOPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SCARD_STATE(pub u32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_STATE_UNAWARE: SCARD_STATE = SCARD_STATE(0u32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_STATE_IGNORE: SCARD_STATE = SCARD_STATE(1u32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_STATE_UNAVAILABLE: SCARD_STATE = SCARD_STATE(8u32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_STATE_EMPTY: SCARD_STATE = SCARD_STATE(16u32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_STATE_PRESENT: SCARD_STATE = SCARD_STATE(32u32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_STATE_ATRMATCH: SCARD_STATE = SCARD_STATE(64u32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_STATE_EXCLUSIVE: SCARD_STATE = SCARD_STATE(128u32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_STATE_INUSE: SCARD_STATE = SCARD_STATE(256u32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_STATE_MUTE: SCARD_STATE = SCARD_STATE(512u32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_STATE_CHANGED: SCARD_STATE = SCARD_STATE(2u32);
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub const SCARD_STATE_UNKNOWN: SCARD_STATE = SCARD_STATE(4u32);
impl ::core::marker::Copy for SCARD_STATE {}
impl ::core::clone::Clone for SCARD_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SCARD_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SCARD_STATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SCARD_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCARD_STATE").field(&self.0).finish()
    }
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
impl ::core::fmt::Debug for BINARY_BLOB_CREDENTIAL_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BINARY_BLOB_CREDENTIAL_INFO").field("cbBlob", &self.cbBlob).field("pbBlob", &self.pbBlob).finish()
    }
}
impl ::windows::core::TypeKind for BINARY_BLOB_CREDENTIAL_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for BINARY_BLOB_CREDENTIAL_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbBlob == other.cbBlob && self.pbBlob == other.pbBlob
    }
}
impl ::core::cmp::Eq for BINARY_BLOB_CREDENTIAL_INFO {}
impl ::core::default::Default for BINARY_BLOB_CREDENTIAL_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for CERT_CREDENTIAL_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_CREDENTIAL_INFO").field("cbSize", &self.cbSize).field("rgbHashOfCert", &self.rgbHashOfCert).finish()
    }
}
impl ::windows::core::TypeKind for CERT_CREDENTIAL_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for CERT_CREDENTIAL_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.rgbHashOfCert == other.rgbHashOfCert
    }
}
impl ::core::cmp::Eq for CERT_CREDENTIAL_INFO {}
impl ::core::default::Default for CERT_CREDENTIAL_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CREDENTIALA {
    pub Flags: CRED_FLAGS,
    pub Type: CRED_TYPE,
    pub TargetName: ::windows::core::PSTR,
    pub Comment: ::windows::core::PSTR,
    pub LastWritten: super::super::Foundation::FILETIME,
    pub CredentialBlobSize: u32,
    pub CredentialBlob: *mut u8,
    pub Persist: CRED_PERSIST,
    pub AttributeCount: u32,
    pub Attributes: *mut CREDENTIAL_ATTRIBUTEA,
    pub TargetAlias: ::windows::core::PSTR,
    pub UserName: ::windows::core::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CREDENTIALA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CREDENTIALA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CREDENTIALA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CREDENTIALA")
            .field("Flags", &self.Flags)
            .field("Type", &self.Type)
            .field("TargetName", &self.TargetName)
            .field("Comment", &self.Comment)
            .field("LastWritten", &self.LastWritten)
            .field("CredentialBlobSize", &self.CredentialBlobSize)
            .field("CredentialBlob", &self.CredentialBlob)
            .field("Persist", &self.Persist)
            .field("AttributeCount", &self.AttributeCount)
            .field("Attributes", &self.Attributes)
            .field("TargetAlias", &self.TargetAlias)
            .field("UserName", &self.UserName)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for CREDENTIALA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CREDENTIALA {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.Type == other.Type && self.TargetName == other.TargetName && self.Comment == other.Comment && self.LastWritten == other.LastWritten && self.CredentialBlobSize == other.CredentialBlobSize && self.CredentialBlob == other.CredentialBlob && self.Persist == other.Persist && self.AttributeCount == other.AttributeCount && self.Attributes == other.Attributes && self.TargetAlias == other.TargetAlias && self.UserName == other.UserName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CREDENTIALA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CREDENTIALA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CREDENTIALW {
    pub Flags: CRED_FLAGS,
    pub Type: CRED_TYPE,
    pub TargetName: ::windows::core::PWSTR,
    pub Comment: ::windows::core::PWSTR,
    pub LastWritten: super::super::Foundation::FILETIME,
    pub CredentialBlobSize: u32,
    pub CredentialBlob: *mut u8,
    pub Persist: CRED_PERSIST,
    pub AttributeCount: u32,
    pub Attributes: *mut CREDENTIAL_ATTRIBUTEW,
    pub TargetAlias: ::windows::core::PWSTR,
    pub UserName: ::windows::core::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CREDENTIALW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CREDENTIALW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CREDENTIALW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CREDENTIALW")
            .field("Flags", &self.Flags)
            .field("Type", &self.Type)
            .field("TargetName", &self.TargetName)
            .field("Comment", &self.Comment)
            .field("LastWritten", &self.LastWritten)
            .field("CredentialBlobSize", &self.CredentialBlobSize)
            .field("CredentialBlob", &self.CredentialBlob)
            .field("Persist", &self.Persist)
            .field("AttributeCount", &self.AttributeCount)
            .field("Attributes", &self.Attributes)
            .field("TargetAlias", &self.TargetAlias)
            .field("UserName", &self.UserName)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for CREDENTIALW {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CREDENTIALW {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.Type == other.Type && self.TargetName == other.TargetName && self.Comment == other.Comment && self.LastWritten == other.LastWritten && self.CredentialBlobSize == other.CredentialBlobSize && self.CredentialBlob == other.CredentialBlob && self.Persist == other.Persist && self.AttributeCount == other.AttributeCount && self.Attributes == other.Attributes && self.TargetAlias == other.TargetAlias && self.UserName == other.UserName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CREDENTIALW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CREDENTIALW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub struct CREDENTIAL_ATTRIBUTEA {
    pub Keyword: ::windows::core::PSTR,
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
impl ::core::fmt::Debug for CREDENTIAL_ATTRIBUTEA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CREDENTIAL_ATTRIBUTEA").field("Keyword", &self.Keyword).field("Flags", &self.Flags).field("ValueSize", &self.ValueSize).field("Value", &self.Value).finish()
    }
}
impl ::windows::core::TypeKind for CREDENTIAL_ATTRIBUTEA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for CREDENTIAL_ATTRIBUTEA {
    fn eq(&self, other: &Self) -> bool {
        self.Keyword == other.Keyword && self.Flags == other.Flags && self.ValueSize == other.ValueSize && self.Value == other.Value
    }
}
impl ::core::cmp::Eq for CREDENTIAL_ATTRIBUTEA {}
impl ::core::default::Default for CREDENTIAL_ATTRIBUTEA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub struct CREDENTIAL_ATTRIBUTEW {
    pub Keyword: ::windows::core::PWSTR,
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
impl ::core::fmt::Debug for CREDENTIAL_ATTRIBUTEW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CREDENTIAL_ATTRIBUTEW").field("Keyword", &self.Keyword).field("Flags", &self.Flags).field("ValueSize", &self.ValueSize).field("Value", &self.Value).finish()
    }
}
impl ::windows::core::TypeKind for CREDENTIAL_ATTRIBUTEW {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for CREDENTIAL_ATTRIBUTEW {
    fn eq(&self, other: &Self) -> bool {
        self.Keyword == other.Keyword && self.Flags == other.Flags && self.ValueSize == other.ValueSize && self.Value == other.Value
    }
}
impl ::core::cmp::Eq for CREDENTIAL_ATTRIBUTEW {}
impl ::core::default::Default for CREDENTIAL_ATTRIBUTEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub struct CREDENTIAL_TARGET_INFORMATIONA {
    pub TargetName: ::windows::core::PSTR,
    pub NetbiosServerName: ::windows::core::PSTR,
    pub DnsServerName: ::windows::core::PSTR,
    pub NetbiosDomainName: ::windows::core::PSTR,
    pub DnsDomainName: ::windows::core::PSTR,
    pub DnsTreeName: ::windows::core::PSTR,
    pub PackageName: ::windows::core::PSTR,
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
impl ::core::fmt::Debug for CREDENTIAL_TARGET_INFORMATIONA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CREDENTIAL_TARGET_INFORMATIONA").field("TargetName", &self.TargetName).field("NetbiosServerName", &self.NetbiosServerName).field("DnsServerName", &self.DnsServerName).field("NetbiosDomainName", &self.NetbiosDomainName).field("DnsDomainName", &self.DnsDomainName).field("DnsTreeName", &self.DnsTreeName).field("PackageName", &self.PackageName).field("Flags", &self.Flags).field("CredTypeCount", &self.CredTypeCount).field("CredTypes", &self.CredTypes).finish()
    }
}
impl ::windows::core::TypeKind for CREDENTIAL_TARGET_INFORMATIONA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for CREDENTIAL_TARGET_INFORMATIONA {
    fn eq(&self, other: &Self) -> bool {
        self.TargetName == other.TargetName && self.NetbiosServerName == other.NetbiosServerName && self.DnsServerName == other.DnsServerName && self.NetbiosDomainName == other.NetbiosDomainName && self.DnsDomainName == other.DnsDomainName && self.DnsTreeName == other.DnsTreeName && self.PackageName == other.PackageName && self.Flags == other.Flags && self.CredTypeCount == other.CredTypeCount && self.CredTypes == other.CredTypes
    }
}
impl ::core::cmp::Eq for CREDENTIAL_TARGET_INFORMATIONA {}
impl ::core::default::Default for CREDENTIAL_TARGET_INFORMATIONA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub struct CREDENTIAL_TARGET_INFORMATIONW {
    pub TargetName: ::windows::core::PWSTR,
    pub NetbiosServerName: ::windows::core::PWSTR,
    pub DnsServerName: ::windows::core::PWSTR,
    pub NetbiosDomainName: ::windows::core::PWSTR,
    pub DnsDomainName: ::windows::core::PWSTR,
    pub DnsTreeName: ::windows::core::PWSTR,
    pub PackageName: ::windows::core::PWSTR,
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
impl ::core::fmt::Debug for CREDENTIAL_TARGET_INFORMATIONW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CREDENTIAL_TARGET_INFORMATIONW").field("TargetName", &self.TargetName).field("NetbiosServerName", &self.NetbiosServerName).field("DnsServerName", &self.DnsServerName).field("NetbiosDomainName", &self.NetbiosDomainName).field("DnsDomainName", &self.DnsDomainName).field("DnsTreeName", &self.DnsTreeName).field("PackageName", &self.PackageName).field("Flags", &self.Flags).field("CredTypeCount", &self.CredTypeCount).field("CredTypes", &self.CredTypes).finish()
    }
}
impl ::windows::core::TypeKind for CREDENTIAL_TARGET_INFORMATIONW {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for CREDENTIAL_TARGET_INFORMATIONW {
    fn eq(&self, other: &Self) -> bool {
        self.TargetName == other.TargetName && self.NetbiosServerName == other.NetbiosServerName && self.DnsServerName == other.DnsServerName && self.NetbiosDomainName == other.NetbiosDomainName && self.DnsDomainName == other.DnsDomainName && self.DnsTreeName == other.DnsTreeName && self.PackageName == other.PackageName && self.Flags == other.Flags && self.CredTypeCount == other.CredTypeCount && self.CredTypes == other.CredTypes
    }
}
impl ::core::cmp::Eq for CREDENTIAL_TARGET_INFORMATIONW {}
impl ::core::default::Default for CREDENTIAL_TARGET_INFORMATIONW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
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
impl ::core::fmt::Debug for CREDSSP_CRED {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CREDSSP_CRED").field("Type", &self.Type).field("pSchannelCred", &self.pSchannelCred).field("pSpnegoCred", &self.pSpnegoCred).finish()
    }
}
impl ::windows::core::TypeKind for CREDSSP_CRED {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for CREDSSP_CRED {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.pSchannelCred == other.pSchannelCred && self.pSpnegoCred == other.pSpnegoCred
    }
}
impl ::core::cmp::Eq for CREDSSP_CRED {}
impl ::core::default::Default for CREDSSP_CRED {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for CREDSSP_CRED_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CREDSSP_CRED_EX").field("Type", &self.Type).field("Version", &self.Version).field("Flags", &self.Flags).field("Reserved", &self.Reserved).field("Cred", &self.Cred).finish()
    }
}
impl ::windows::core::TypeKind for CREDSSP_CRED_EX {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for CREDSSP_CRED_EX {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Version == other.Version && self.Flags == other.Flags && self.Reserved == other.Reserved && self.Cred == other.Cred
    }
}
impl ::core::cmp::Eq for CREDSSP_CRED_EX {}
impl ::core::default::Default for CREDSSP_CRED_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct CREDUI_INFOA {
    pub cbSize: u32,
    pub hwndParent: super::super::Foundation::HWND,
    pub pszMessageText: ::windows::core::PCSTR,
    pub pszCaptionText: ::windows::core::PCSTR,
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for CREDUI_INFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CREDUI_INFOA").field("cbSize", &self.cbSize).field("hwndParent", &self.hwndParent).field("pszMessageText", &self.pszMessageText).field("pszCaptionText", &self.pszCaptionText).field("hbmBanner", &self.hbmBanner).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for CREDUI_INFOA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for CREDUI_INFOA {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.hwndParent == other.hwndParent && self.pszMessageText == other.pszMessageText && self.pszCaptionText == other.pszCaptionText && self.hbmBanner == other.hbmBanner
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for CREDUI_INFOA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for CREDUI_INFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct CREDUI_INFOW {
    pub cbSize: u32,
    pub hwndParent: super::super::Foundation::HWND,
    pub pszMessageText: ::windows::core::PCWSTR,
    pub pszCaptionText: ::windows::core::PCWSTR,
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for CREDUI_INFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CREDUI_INFOW").field("cbSize", &self.cbSize).field("hwndParent", &self.hwndParent).field("pszMessageText", &self.pszMessageText).field("pszCaptionText", &self.pszCaptionText).field("hbmBanner", &self.hbmBanner).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for CREDUI_INFOW {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for CREDUI_INFOW {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.hwndParent == other.hwndParent && self.pszMessageText == other.pszMessageText && self.pszCaptionText == other.pszCaptionText && self.hbmBanner == other.hbmBanner
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for CREDUI_INFOW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for CREDUI_INFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub struct KeyCredentialManagerInfo {
    pub containerId: ::windows::core::GUID,
}
impl ::core::marker::Copy for KeyCredentialManagerInfo {}
impl ::core::clone::Clone for KeyCredentialManagerInfo {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for KeyCredentialManagerInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KeyCredentialManagerInfo").field("containerId", &self.containerId).finish()
    }
}
impl ::windows::core::TypeKind for KeyCredentialManagerInfo {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for KeyCredentialManagerInfo {
    fn eq(&self, other: &Self) -> bool {
        self.containerId == other.containerId
    }
}
impl ::core::cmp::Eq for KeyCredentialManagerInfo {}
impl ::core::default::Default for KeyCredentialManagerInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct OPENCARDNAMEA {
    pub dwStructSize: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub hSCardContext: usize,
    pub lpstrGroupNames: ::windows::core::PSTR,
    pub nMaxGroupNames: u32,
    pub lpstrCardNames: ::windows::core::PSTR,
    pub nMaxCardNames: u32,
    pub rgguidInterfaces: *const ::windows::core::GUID,
    pub cguidInterfaces: u32,
    pub lpstrRdr: ::windows::core::PSTR,
    pub nMaxRdr: u32,
    pub lpstrCard: ::windows::core::PSTR,
    pub nMaxCard: u32,
    pub lpstrTitle: ::windows::core::PCSTR,
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for OPENCARDNAMEA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OPENCARDNAMEA")
            .field("dwStructSize", &self.dwStructSize)
            .field("hwndOwner", &self.hwndOwner)
            .field("hSCardContext", &self.hSCardContext)
            .field("lpstrGroupNames", &self.lpstrGroupNames)
            .field("nMaxGroupNames", &self.nMaxGroupNames)
            .field("lpstrCardNames", &self.lpstrCardNames)
            .field("nMaxCardNames", &self.nMaxCardNames)
            .field("rgguidInterfaces", &self.rgguidInterfaces)
            .field("cguidInterfaces", &self.cguidInterfaces)
            .field("lpstrRdr", &self.lpstrRdr)
            .field("nMaxRdr", &self.nMaxRdr)
            .field("lpstrCard", &self.lpstrCard)
            .field("nMaxCard", &self.nMaxCard)
            .field("lpstrTitle", &self.lpstrTitle)
            .field("dwFlags", &self.dwFlags)
            .field("pvUserData", &self.pvUserData)
            .field("dwShareMode", &self.dwShareMode)
            .field("dwPreferredProtocols", &self.dwPreferredProtocols)
            .field("dwActiveProtocol", &self.dwActiveProtocol)
            .field("hCardHandle", &self.hCardHandle)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for OPENCARDNAMEA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OPENCARDNAMEA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct OPENCARDNAMEW {
    pub dwStructSize: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub hSCardContext: usize,
    pub lpstrGroupNames: ::windows::core::PWSTR,
    pub nMaxGroupNames: u32,
    pub lpstrCardNames: ::windows::core::PWSTR,
    pub nMaxCardNames: u32,
    pub rgguidInterfaces: *const ::windows::core::GUID,
    pub cguidInterfaces: u32,
    pub lpstrRdr: ::windows::core::PWSTR,
    pub nMaxRdr: u32,
    pub lpstrCard: ::windows::core::PWSTR,
    pub nMaxCard: u32,
    pub lpstrTitle: ::windows::core::PCWSTR,
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for OPENCARDNAMEW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OPENCARDNAMEW")
            .field("dwStructSize", &self.dwStructSize)
            .field("hwndOwner", &self.hwndOwner)
            .field("hSCardContext", &self.hSCardContext)
            .field("lpstrGroupNames", &self.lpstrGroupNames)
            .field("nMaxGroupNames", &self.nMaxGroupNames)
            .field("lpstrCardNames", &self.lpstrCardNames)
            .field("nMaxCardNames", &self.nMaxCardNames)
            .field("rgguidInterfaces", &self.rgguidInterfaces)
            .field("cguidInterfaces", &self.cguidInterfaces)
            .field("lpstrRdr", &self.lpstrRdr)
            .field("nMaxRdr", &self.nMaxRdr)
            .field("lpstrCard", &self.lpstrCard)
            .field("nMaxCard", &self.nMaxCard)
            .field("lpstrTitle", &self.lpstrTitle)
            .field("dwFlags", &self.dwFlags)
            .field("pvUserData", &self.pvUserData)
            .field("dwShareMode", &self.dwShareMode)
            .field("dwPreferredProtocols", &self.dwPreferredProtocols)
            .field("dwActiveProtocol", &self.dwActiveProtocol)
            .field("hCardHandle", &self.hCardHandle)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for OPENCARDNAMEW {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OPENCARDNAMEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
    pub lpstrTitle: ::windows::core::PCSTR,
    pub lpstrSearchDesc: ::windows::core::PCSTR,
    pub hIcon: super::super::UI::WindowsAndMessaging::HICON,
    pub pOpenCardSearchCriteria: *mut OPENCARD_SEARCH_CRITERIAA,
    pub lpfnConnect: LPOCNCONNPROCA,
    pub pvUserData: *mut ::core::ffi::c_void,
    pub dwShareMode: u32,
    pub dwPreferredProtocols: u32,
    pub lpstrRdr: ::windows::core::PSTR,
    pub nMaxRdr: u32,
    pub lpstrCard: ::windows::core::PSTR,
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::fmt::Debug for OPENCARDNAME_EXA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OPENCARDNAME_EXA")
            .field("dwStructSize", &self.dwStructSize)
            .field("hSCardContext", &self.hSCardContext)
            .field("hwndOwner", &self.hwndOwner)
            .field("dwFlags", &self.dwFlags)
            .field("lpstrTitle", &self.lpstrTitle)
            .field("lpstrSearchDesc", &self.lpstrSearchDesc)
            .field("hIcon", &self.hIcon)
            .field("pOpenCardSearchCriteria", &self.pOpenCardSearchCriteria)
            .field("pvUserData", &self.pvUserData)
            .field("dwShareMode", &self.dwShareMode)
            .field("dwPreferredProtocols", &self.dwPreferredProtocols)
            .field("lpstrRdr", &self.lpstrRdr)
            .field("nMaxRdr", &self.nMaxRdr)
            .field("lpstrCard", &self.lpstrCard)
            .field("nMaxCard", &self.nMaxCard)
            .field("dwActiveProtocol", &self.dwActiveProtocol)
            .field("hCardHandle", &self.hCardHandle)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::windows::core::TypeKind for OPENCARDNAME_EXA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for OPENCARDNAME_EXA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
    pub lpstrTitle: ::windows::core::PCWSTR,
    pub lpstrSearchDesc: ::windows::core::PCWSTR,
    pub hIcon: super::super::UI::WindowsAndMessaging::HICON,
    pub pOpenCardSearchCriteria: *mut OPENCARD_SEARCH_CRITERIAW,
    pub lpfnConnect: LPOCNCONNPROCW,
    pub pvUserData: *mut ::core::ffi::c_void,
    pub dwShareMode: u32,
    pub dwPreferredProtocols: u32,
    pub lpstrRdr: ::windows::core::PWSTR,
    pub nMaxRdr: u32,
    pub lpstrCard: ::windows::core::PWSTR,
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::fmt::Debug for OPENCARDNAME_EXW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OPENCARDNAME_EXW")
            .field("dwStructSize", &self.dwStructSize)
            .field("hSCardContext", &self.hSCardContext)
            .field("hwndOwner", &self.hwndOwner)
            .field("dwFlags", &self.dwFlags)
            .field("lpstrTitle", &self.lpstrTitle)
            .field("lpstrSearchDesc", &self.lpstrSearchDesc)
            .field("hIcon", &self.hIcon)
            .field("pOpenCardSearchCriteria", &self.pOpenCardSearchCriteria)
            .field("pvUserData", &self.pvUserData)
            .field("dwShareMode", &self.dwShareMode)
            .field("dwPreferredProtocols", &self.dwPreferredProtocols)
            .field("lpstrRdr", &self.lpstrRdr)
            .field("nMaxRdr", &self.nMaxRdr)
            .field("lpstrCard", &self.lpstrCard)
            .field("nMaxCard", &self.nMaxCard)
            .field("dwActiveProtocol", &self.dwActiveProtocol)
            .field("hCardHandle", &self.hCardHandle)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::windows::core::TypeKind for OPENCARDNAME_EXW {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for OPENCARDNAME_EXW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct OPENCARD_SEARCH_CRITERIAA {
    pub dwStructSize: u32,
    pub lpstrGroupNames: ::windows::core::PSTR,
    pub nMaxGroupNames: u32,
    pub rgguidInterfaces: *const ::windows::core::GUID,
    pub cguidInterfaces: u32,
    pub lpstrCardNames: ::windows::core::PSTR,
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for OPENCARD_SEARCH_CRITERIAA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OPENCARD_SEARCH_CRITERIAA")
            .field("dwStructSize", &self.dwStructSize)
            .field("lpstrGroupNames", &self.lpstrGroupNames)
            .field("nMaxGroupNames", &self.nMaxGroupNames)
            .field("rgguidInterfaces", &self.rgguidInterfaces)
            .field("cguidInterfaces", &self.cguidInterfaces)
            .field("lpstrCardNames", &self.lpstrCardNames)
            .field("nMaxCardNames", &self.nMaxCardNames)
            .field("pvUserData", &self.pvUserData)
            .field("dwShareMode", &self.dwShareMode)
            .field("dwPreferredProtocols", &self.dwPreferredProtocols)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for OPENCARD_SEARCH_CRITERIAA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OPENCARD_SEARCH_CRITERIAA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct OPENCARD_SEARCH_CRITERIAW {
    pub dwStructSize: u32,
    pub lpstrGroupNames: ::windows::core::PWSTR,
    pub nMaxGroupNames: u32,
    pub rgguidInterfaces: *const ::windows::core::GUID,
    pub cguidInterfaces: u32,
    pub lpstrCardNames: ::windows::core::PWSTR,
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for OPENCARD_SEARCH_CRITERIAW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OPENCARD_SEARCH_CRITERIAW")
            .field("dwStructSize", &self.dwStructSize)
            .field("lpstrGroupNames", &self.lpstrGroupNames)
            .field("nMaxGroupNames", &self.nMaxGroupNames)
            .field("rgguidInterfaces", &self.rgguidInterfaces)
            .field("cguidInterfaces", &self.cguidInterfaces)
            .field("lpstrCardNames", &self.lpstrCardNames)
            .field("nMaxCardNames", &self.nMaxCardNames)
            .field("pvUserData", &self.pvUserData)
            .field("dwShareMode", &self.dwShareMode)
            .field("dwPreferredProtocols", &self.dwPreferredProtocols)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for OPENCARD_SEARCH_CRITERIAW {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OPENCARD_SEARCH_CRITERIAW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::windows::core::TypeKind for READER_SEL_REQUEST {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for READER_SEL_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::windows::core::TypeKind for READER_SEL_REQUEST_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for READER_SEL_REQUEST_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for READER_SEL_REQUEST_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("READER_SEL_REQUEST_0_0").field("cbReaderNameOffset", &self.cbReaderNameOffset).field("cchReaderNameLength", &self.cchReaderNameLength).field("cbContainerNameOffset", &self.cbContainerNameOffset).field("cchContainerNameLength", &self.cchContainerNameLength).field("dwDesiredCardModuleVersion", &self.dwDesiredCardModuleVersion).field("dwCspFlags", &self.dwCspFlags).finish()
    }
}
impl ::windows::core::TypeKind for READER_SEL_REQUEST_0_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for READER_SEL_REQUEST_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.cbReaderNameOffset == other.cbReaderNameOffset && self.cchReaderNameLength == other.cchReaderNameLength && self.cbContainerNameOffset == other.cbContainerNameOffset && self.cchContainerNameLength == other.cchContainerNameLength && self.dwDesiredCardModuleVersion == other.dwDesiredCardModuleVersion && self.dwCspFlags == other.dwCspFlags
    }
}
impl ::core::cmp::Eq for READER_SEL_REQUEST_0_0 {}
impl ::core::default::Default for READER_SEL_REQUEST_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for READER_SEL_REQUEST_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("READER_SEL_REQUEST_0_1").field("cbSerialNumberOffset", &self.cbSerialNumberOffset).field("cbSerialNumberLength", &self.cbSerialNumberLength).field("dwDesiredCardModuleVersion", &self.dwDesiredCardModuleVersion).finish()
    }
}
impl ::windows::core::TypeKind for READER_SEL_REQUEST_0_1 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for READER_SEL_REQUEST_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.cbSerialNumberOffset == other.cbSerialNumberOffset && self.cbSerialNumberLength == other.cbSerialNumberLength && self.dwDesiredCardModuleVersion == other.dwDesiredCardModuleVersion
    }
}
impl ::core::cmp::Eq for READER_SEL_REQUEST_0_1 {}
impl ::core::default::Default for READER_SEL_REQUEST_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
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
impl ::core::fmt::Debug for READER_SEL_RESPONSE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("READER_SEL_RESPONSE").field("cbReaderNameOffset", &self.cbReaderNameOffset).field("cchReaderNameLength", &self.cchReaderNameLength).field("cbCardNameOffset", &self.cbCardNameOffset).field("cchCardNameLength", &self.cchCardNameLength).finish()
    }
}
impl ::windows::core::TypeKind for READER_SEL_RESPONSE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for READER_SEL_RESPONSE {
    fn eq(&self, other: &Self) -> bool {
        self.cbReaderNameOffset == other.cbReaderNameOffset && self.cchReaderNameLength == other.cchReaderNameLength && self.cbCardNameOffset == other.cbCardNameOffset && self.cchCardNameLength == other.cchCardNameLength
    }
}
impl ::core::cmp::Eq for READER_SEL_RESPONSE {}
impl ::core::default::Default for READER_SEL_RESPONSE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
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
impl ::core::fmt::Debug for SCARD_ATRMASK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCARD_ATRMASK").field("cbAtr", &self.cbAtr).field("rgbAtr", &self.rgbAtr).field("rgbMask", &self.rgbMask).finish()
    }
}
impl ::windows::core::TypeKind for SCARD_ATRMASK {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SCARD_ATRMASK {
    fn eq(&self, other: &Self) -> bool {
        self.cbAtr == other.cbAtr && self.rgbAtr == other.rgbAtr && self.rgbMask == other.rgbMask
    }
}
impl ::core::cmp::Eq for SCARD_ATRMASK {}
impl ::core::default::Default for SCARD_ATRMASK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
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
impl ::core::fmt::Debug for SCARD_IO_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCARD_IO_REQUEST").field("dwProtocol", &self.dwProtocol).field("cbPciLength", &self.cbPciLength).finish()
    }
}
impl ::windows::core::TypeKind for SCARD_IO_REQUEST {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SCARD_IO_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.dwProtocol == other.dwProtocol && self.cbPciLength == other.cbPciLength
    }
}
impl ::core::cmp::Eq for SCARD_IO_REQUEST {}
impl ::core::default::Default for SCARD_IO_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub struct SCARD_READERSTATEA {
    pub szReader: ::windows::core::PCSTR,
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
impl ::core::fmt::Debug for SCARD_READERSTATEA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCARD_READERSTATEA").field("szReader", &self.szReader).field("pvUserData", &self.pvUserData).field("dwCurrentState", &self.dwCurrentState).field("dwEventState", &self.dwEventState).field("cbAtr", &self.cbAtr).field("rgbAtr", &self.rgbAtr).finish()
    }
}
impl ::windows::core::TypeKind for SCARD_READERSTATEA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SCARD_READERSTATEA {
    fn eq(&self, other: &Self) -> bool {
        self.szReader == other.szReader && self.pvUserData == other.pvUserData && self.dwCurrentState == other.dwCurrentState && self.dwEventState == other.dwEventState && self.cbAtr == other.cbAtr && self.rgbAtr == other.rgbAtr
    }
}
impl ::core::cmp::Eq for SCARD_READERSTATEA {}
impl ::core::default::Default for SCARD_READERSTATEA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub struct SCARD_READERSTATEW {
    pub szReader: ::windows::core::PCWSTR,
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
impl ::core::fmt::Debug for SCARD_READERSTATEW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCARD_READERSTATEW").field("szReader", &self.szReader).field("pvUserData", &self.pvUserData).field("dwCurrentState", &self.dwCurrentState).field("dwEventState", &self.dwEventState).field("cbAtr", &self.cbAtr).field("rgbAtr", &self.rgbAtr).finish()
    }
}
impl ::windows::core::TypeKind for SCARD_READERSTATEW {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SCARD_READERSTATEW {
    fn eq(&self, other: &Self) -> bool {
        self.szReader == other.szReader && self.pvUserData == other.pvUserData && self.dwCurrentState == other.dwCurrentState && self.dwEventState == other.dwEventState && self.cbAtr == other.cbAtr && self.rgbAtr == other.rgbAtr
    }
}
impl ::core::cmp::Eq for SCARD_READERSTATEW {}
impl ::core::default::Default for SCARD_READERSTATEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
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
impl ::core::fmt::Debug for SCARD_T0_COMMAND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCARD_T0_COMMAND").field("bCla", &self.bCla).field("bIns", &self.bIns).field("bP1", &self.bP1).field("bP2", &self.bP2).field("bP3", &self.bP3).finish()
    }
}
impl ::windows::core::TypeKind for SCARD_T0_COMMAND {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SCARD_T0_COMMAND {
    fn eq(&self, other: &Self) -> bool {
        self.bCla == other.bCla && self.bIns == other.bIns && self.bP1 == other.bP1 && self.bP2 == other.bP2 && self.bP3 == other.bP3
    }
}
impl ::core::cmp::Eq for SCARD_T0_COMMAND {}
impl ::core::default::Default for SCARD_T0_COMMAND {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
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
impl ::windows::core::TypeKind for SCARD_T0_REQUEST {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for SCARD_T0_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::windows::core::TypeKind for SCARD_T0_REQUEST_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for SCARD_T0_REQUEST_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
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
impl ::core::fmt::Debug for SCARD_T1_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCARD_T1_REQUEST").field("ioRequest", &self.ioRequest).finish()
    }
}
impl ::windows::core::TypeKind for SCARD_T1_REQUEST {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SCARD_T1_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.ioRequest == other.ioRequest
    }
}
impl ::core::cmp::Eq for SCARD_T1_REQUEST {}
impl ::core::default::Default for SCARD_T1_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
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
impl ::core::fmt::Debug for SecHandle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SecHandle").field("dwLower", &self.dwLower).field("dwUpper", &self.dwUpper).finish()
    }
}
impl ::windows::core::TypeKind for SecHandle {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SecHandle {
    fn eq(&self, other: &Self) -> bool {
        self.dwLower == other.dwLower && self.dwUpper == other.dwUpper
    }
}
impl ::core::cmp::Eq for SecHandle {}
impl ::core::default::Default for SecHandle {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for SecPkgContext_ClientCreds {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SecPkgContext_ClientCreds").field("AuthBufferLen", &self.AuthBufferLen).field("AuthBuffer", &self.AuthBuffer).finish()
    }
}
impl ::windows::core::TypeKind for SecPkgContext_ClientCreds {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SecPkgContext_ClientCreds {
    fn eq(&self, other: &Self) -> bool {
        self.AuthBufferLen == other.AuthBufferLen && self.AuthBuffer == other.AuthBuffer
    }
}
impl ::core::cmp::Eq for SecPkgContext_ClientCreds {}
impl ::core::default::Default for SecPkgContext_ClientCreds {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub struct USERNAME_TARGET_CREDENTIAL_INFO {
    pub UserName: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for USERNAME_TARGET_CREDENTIAL_INFO {}
impl ::core::clone::Clone for USERNAME_TARGET_CREDENTIAL_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for USERNAME_TARGET_CREDENTIAL_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USERNAME_TARGET_CREDENTIAL_INFO").field("UserName", &self.UserName).finish()
    }
}
impl ::windows::core::TypeKind for USERNAME_TARGET_CREDENTIAL_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for USERNAME_TARGET_CREDENTIAL_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.UserName == other.UserName
    }
}
impl ::core::cmp::Eq for USERNAME_TARGET_CREDENTIAL_INFO {}
impl ::core::default::Default for USERNAME_TARGET_CREDENTIAL_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Security_Credentials\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPOCNCHKPROC = ::core::option::Option<unsafe extern "system" fn(param0: usize, param1: usize, param2: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub type LPOCNCONNPROCA = ::core::option::Option<unsafe extern "system" fn(param0: usize, param1: ::windows::core::PCSTR, param2: ::windows::core::PCSTR, param3: *const ::core::ffi::c_void) -> usize>;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub type LPOCNCONNPROCW = ::core::option::Option<unsafe extern "system" fn(param0: usize, param1: ::windows::core::PCWSTR, param2: ::windows::core::PCWSTR, param3: *const ::core::ffi::c_void) -> usize>;
#[doc = "*Required features: `\"Win32_Security_Credentials\"`*"]
pub type LPOCNDSCPROC = ::core::option::Option<unsafe extern "system" fn(param0: usize, param1: usize, param2: *const ::core::ffi::c_void) -> ()>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
