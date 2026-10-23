#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CredDeleteA<P0>(targetname: P0, r#type: super::DWORD, flags: super::DWORD) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn CredDeleteA(targetname : windows_core::PCSTR, r#type : super::DWORD, flags : super::DWORD) -> windows_core::BOOL);
    unsafe { CredDeleteA(targetname.param().abi(), r#type, flags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CredDeleteW<P0>(targetname: P0, r#type: super::DWORD, flags: super::DWORD) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn CredDeleteW(targetname : windows_core::PCWSTR, r#type : super::DWORD, flags : super::DWORD) -> windows_core::BOOL);
    unsafe { CredDeleteW(targetname.param().abi(), r#type, flags) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CredEnumerateA<P0>(filter: P0, flags: super::DWORD, count: *mut super::DWORD, credential: *mut *mut PCREDENTIALA) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn CredEnumerateA(filter : windows_core::PCSTR, flags : super::DWORD, count : *mut super::DWORD, credential : *mut *mut PCREDENTIALA) -> windows_core::BOOL);
    unsafe { CredEnumerateA(filter.param().abi(), flags, count as _, credential as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CredEnumerateW<P0>(filter: P0, flags: super::DWORD, count: *mut super::DWORD, credential: *mut *mut PCREDENTIALW) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn CredEnumerateW(filter : windows_core::PCWSTR, flags : super::DWORD, count : *mut super::DWORD, credential : *mut *mut PCREDENTIALW) -> windows_core::BOOL);
    unsafe { CredEnumerateW(filter.param().abi(), flags, count as _, credential as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CredFindBestCredentialA<P0>(targetname: P0, r#type: super::DWORD, flags: super::DWORD, credential: *mut PCREDENTIALA) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn CredFindBestCredentialA(targetname : windows_core::PCSTR, r#type : super::DWORD, flags : super::DWORD, credential : *mut PCREDENTIALA) -> windows_core::BOOL);
    unsafe { CredFindBestCredentialA(targetname.param().abi(), r#type, flags, credential as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CredFindBestCredentialW<P0>(targetname: P0, r#type: super::DWORD, flags: super::DWORD, credential: *mut PCREDENTIALW) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn CredFindBestCredentialW(targetname : windows_core::PCWSTR, r#type : super::DWORD, flags : super::DWORD, credential : *mut PCREDENTIALW) -> windows_core::BOOL);
    unsafe { CredFindBestCredentialW(targetname.param().abi(), r#type, flags, credential as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CredFree(buffer: super::PVOID) {
    windows_core::link!("advapi32.dll" "system" fn CredFree(buffer : super::PVOID));
    unsafe { CredFree(buffer) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CredGetSessionTypes(maximumpersistcount: super::DWORD, maximumpersist: super::LPDWORD) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn CredGetSessionTypes(maximumpersistcount : super::DWORD, maximumpersist : super::LPDWORD) -> windows_core::BOOL);
    unsafe { CredGetSessionTypes(maximumpersistcount, maximumpersist as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CredGetTargetInfoA<P0>(targetname: P0, flags: super::DWORD, targetinfo: *mut PCREDENTIAL_TARGET_INFORMATIONA) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn CredGetTargetInfoA(targetname : windows_core::PCSTR, flags : super::DWORD, targetinfo : *mut PCREDENTIAL_TARGET_INFORMATIONA) -> windows_core::BOOL);
    unsafe { CredGetTargetInfoA(targetname.param().abi(), flags, targetinfo as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CredGetTargetInfoW<P0>(targetname: P0, flags: super::DWORD, targetinfo: *mut PCREDENTIAL_TARGET_INFORMATIONW) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn CredGetTargetInfoW(targetname : windows_core::PCWSTR, flags : super::DWORD, targetinfo : *mut PCREDENTIAL_TARGET_INFORMATIONW) -> windows_core::BOOL);
    unsafe { CredGetTargetInfoW(targetname.param().abi(), flags, targetinfo as _) }
}
#[inline]
pub unsafe fn CredIsMarshaledCredentialA<P0>(marshaledcredential: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn CredIsMarshaledCredentialA(marshaledcredential : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { CredIsMarshaledCredentialA(marshaledcredential.param().abi()) }
}
#[inline]
pub unsafe fn CredIsMarshaledCredentialW<P0>(marshaledcredential: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn CredIsMarshaledCredentialW(marshaledcredential : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { CredIsMarshaledCredentialW(marshaledcredential.param().abi()) }
}
#[inline]
pub unsafe fn CredIsProtectedA<P0>(pszprotectedcredentials: P0, pprotectiontype: *mut CRED_PROTECTION_TYPE) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn CredIsProtectedA(pszprotectedcredentials : windows_core::PCSTR, pprotectiontype : *mut CRED_PROTECTION_TYPE) -> windows_core::BOOL);
    unsafe { CredIsProtectedA(pszprotectedcredentials.param().abi(), pprotectiontype as _) }
}
#[inline]
pub unsafe fn CredIsProtectedW<P0>(pszprotectedcredentials: P0, pprotectiontype: *mut CRED_PROTECTION_TYPE) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn CredIsProtectedW(pszprotectedcredentials : windows_core::PCWSTR, pprotectiontype : *mut CRED_PROTECTION_TYPE) -> windows_core::BOOL);
    unsafe { CredIsProtectedW(pszprotectedcredentials.param().abi(), pprotectiontype as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CredMarshalCredentialA(credtype: CRED_MARSHAL_TYPE, credential: super::PVOID, marshaledcredential: *mut windows_core::PSTR) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn CredMarshalCredentialA(credtype : CRED_MARSHAL_TYPE, credential : super::PVOID, marshaledcredential : *mut windows_core::PSTR) -> windows_core::BOOL);
    unsafe { CredMarshalCredentialA(credtype, credential, marshaledcredential as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CredMarshalCredentialW(credtype: CRED_MARSHAL_TYPE, credential: super::PVOID, marshaledcredential: *mut windows_core::PWSTR) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn CredMarshalCredentialW(credtype : CRED_MARSHAL_TYPE, credential : super::PVOID, marshaledcredential : *mut windows_core::PWSTR) -> windows_core::BOOL);
    unsafe { CredMarshalCredentialW(credtype, credential, marshaledcredential as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CredPackAuthenticationBufferA<P1, P2>(dwflags: super::DWORD, pszusername: P1, pszpassword: P2, ppackedcredentials: Option<super::PBYTE>, pcbpackedcredentials: *mut super::DWORD) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("credui.dll" "system" fn CredPackAuthenticationBufferA(dwflags : super::DWORD, pszusername : windows_core::PCSTR, pszpassword : windows_core::PCSTR, ppackedcredentials : super::PBYTE, pcbpackedcredentials : *mut super::DWORD) -> windows_core::BOOL);
    unsafe { CredPackAuthenticationBufferA(dwflags, pszusername.param().abi(), pszpassword.param().abi(), ppackedcredentials.unwrap_or(core::mem::zeroed()) as _, pcbpackedcredentials as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CredPackAuthenticationBufferW<P1, P2>(dwflags: super::DWORD, pszusername: P1, pszpassword: P2, ppackedcredentials: Option<super::PBYTE>, pcbpackedcredentials: *mut super::DWORD) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("credui.dll" "system" fn CredPackAuthenticationBufferW(dwflags : super::DWORD, pszusername : windows_core::PCWSTR, pszpassword : windows_core::PCWSTR, ppackedcredentials : super::PBYTE, pcbpackedcredentials : *mut super::DWORD) -> windows_core::BOOL);
    unsafe { CredPackAuthenticationBufferW(dwflags, pszusername.param().abi(), pszpassword.param().abi(), ppackedcredentials.unwrap_or(core::mem::zeroed()) as _, pcbpackedcredentials as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CredProtectA(fasself: bool, pszcredentials: &[u8], pszprotectedcredentials: windows_core::PSTR, pcchmaxchars: *mut super::DWORD, protectiontype: Option<*mut CRED_PROTECTION_TYPE>) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn CredProtectA(fasself : windows_core::BOOL, pszcredentials : windows_core::PCSTR, cchcredentials : super::DWORD, pszprotectedcredentials : windows_core::PSTR, pcchmaxchars : *mut super::DWORD, protectiontype : *mut CRED_PROTECTION_TYPE) -> windows_core::BOOL);
    unsafe { CredProtectA(fasself.into(), core::mem::transmute(pszcredentials.as_ptr()), pszcredentials.len().try_into().unwrap(), pszprotectedcredentials, pcchmaxchars as _, protectiontype.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CredProtectW(fasself: bool, pszcredentials: &[u16], pszprotectedcredentials: windows_core::PWSTR, pcchmaxchars: *mut super::DWORD, protectiontype: Option<*mut CRED_PROTECTION_TYPE>) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn CredProtectW(fasself : windows_core::BOOL, pszcredentials : windows_core::PCWSTR, cchcredentials : super::DWORD, pszprotectedcredentials : windows_core::PWSTR, pcchmaxchars : *mut super::DWORD, protectiontype : *mut CRED_PROTECTION_TYPE) -> windows_core::BOOL);
    unsafe { CredProtectW(fasself.into(), core::mem::transmute(pszcredentials.as_ptr()), pszcredentials.len().try_into().unwrap(), pszprotectedcredentials, pcchmaxchars as _, protectiontype.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CredReadA<P0>(targetname: P0, r#type: super::DWORD, flags: super::DWORD, credential: *mut PCREDENTIALA) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn CredReadA(targetname : windows_core::PCSTR, r#type : super::DWORD, flags : super::DWORD, credential : *mut PCREDENTIALA) -> windows_core::BOOL);
    unsafe { CredReadA(targetname.param().abi(), r#type, flags, credential as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CredReadDomainCredentialsA(targetinfo: PCREDENTIAL_TARGET_INFORMATIONA, flags: super::DWORD, count: *mut super::DWORD, credential: *mut *mut PCREDENTIALA) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn CredReadDomainCredentialsA(targetinfo : PCREDENTIAL_TARGET_INFORMATIONA, flags : super::DWORD, count : *mut super::DWORD, credential : *mut *mut PCREDENTIALA) -> windows_core::BOOL);
    unsafe { CredReadDomainCredentialsA(targetinfo, flags, count as _, credential as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CredReadDomainCredentialsW(targetinfo: PCREDENTIAL_TARGET_INFORMATIONW, flags: super::DWORD, count: *mut super::DWORD, credential: *mut *mut PCREDENTIALW) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn CredReadDomainCredentialsW(targetinfo : PCREDENTIAL_TARGET_INFORMATIONW, flags : super::DWORD, count : *mut super::DWORD, credential : *mut *mut PCREDENTIALW) -> windows_core::BOOL);
    unsafe { CredReadDomainCredentialsW(targetinfo, flags, count as _, credential as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CredReadW<P0>(targetname: P0, r#type: super::DWORD, flags: super::DWORD, credential: *mut PCREDENTIALW) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn CredReadW(targetname : windows_core::PCWSTR, r#type : super::DWORD, flags : super::DWORD, credential : *mut PCREDENTIALW) -> windows_core::BOOL);
    unsafe { CredReadW(targetname.param().abi(), r#type, flags, credential as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CredRenameA<P0, P1>(oldtargetname: P0, newtargetname: P1, r#type: super::DWORD, flags: super::DWORD) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn CredRenameA(oldtargetname : windows_core::PCSTR, newtargetname : windows_core::PCSTR, r#type : super::DWORD, flags : super::DWORD) -> windows_core::BOOL);
    unsafe { CredRenameA(oldtargetname.param().abi(), newtargetname.param().abi(), r#type, flags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CredRenameW<P0, P1>(oldtargetname: P0, newtargetname: P1, r#type: super::DWORD, flags: super::DWORD) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn CredRenameW(oldtargetname : windows_core::PCWSTR, newtargetname : windows_core::PCWSTR, r#type : super::DWORD, flags : super::DWORD) -> windows_core::BOOL);
    unsafe { CredRenameW(oldtargetname.param().abi(), newtargetname.param().abi(), r#type, flags) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "sspi"))]
#[inline]
pub unsafe fn CredUICmdLinePromptForCredentialsA<P0>(psztargetname: P0, pcontext: super::PCtxtHandle, dwautherror: super::DWORD, username: &mut [u8], pszpassword: &mut [u8], pfsave: Option<super::PBOOL>, dwflags: super::DWORD) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("credui.dll" "system" fn CredUICmdLinePromptForCredentialsA(psztargetname : windows_core::PCSTR, pcontext : super::PCtxtHandle, dwautherror : super::DWORD, username : windows_core::PSTR, uluserbuffersize : super::ULONG, pszpassword : windows_core::PSTR, ulpasswordbuffersize : super::ULONG, pfsave : super::PBOOL, dwflags : super::DWORD) -> super::DWORD);
    unsafe { CredUICmdLinePromptForCredentialsA(psztargetname.param().abi(), pcontext as _, dwautherror, core::mem::transmute(username.as_mut_ptr()), username.len().try_into().unwrap(), core::mem::transmute(pszpassword.as_mut_ptr()), pszpassword.len().try_into().unwrap(), pfsave.unwrap_or(core::mem::zeroed()) as _, dwflags) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "sspi"))]
#[inline]
pub unsafe fn CredUICmdLinePromptForCredentialsW<P0>(psztargetname: P0, pcontext: super::PCtxtHandle, dwautherror: super::DWORD, username: &mut [u16], pszpassword: &mut [u16], pfsave: Option<super::PBOOL>, dwflags: super::DWORD) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("credui.dll" "system" fn CredUICmdLinePromptForCredentialsW(psztargetname : windows_core::PCWSTR, pcontext : super::PCtxtHandle, dwautherror : super::DWORD, username : windows_core::PWSTR, uluserbuffersize : super::ULONG, pszpassword : windows_core::PWSTR, ulpasswordbuffersize : super::ULONG, pfsave : super::PBOOL, dwflags : super::DWORD) -> super::DWORD);
    unsafe { CredUICmdLinePromptForCredentialsW(psztargetname.param().abi(), pcontext as _, dwautherror, core::mem::transmute(username.as_mut_ptr()), username.len().try_into().unwrap(), core::mem::transmute(pszpassword.as_mut_ptr()), pszpassword.len().try_into().unwrap(), pfsave.unwrap_or(core::mem::zeroed()) as _, dwflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CredUIConfirmCredentialsA<P0>(psztargetname: P0, bconfirm: bool) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("credui.dll" "system" fn CredUIConfirmCredentialsA(psztargetname : windows_core::PCSTR, bconfirm : windows_core::BOOL) -> super::DWORD);
    unsafe { CredUIConfirmCredentialsA(psztargetname.param().abi(), bconfirm.into()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CredUIConfirmCredentialsW<P0>(psztargetname: P0, bconfirm: bool) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("credui.dll" "system" fn CredUIConfirmCredentialsW(psztargetname : windows_core::PCWSTR, bconfirm : windows_core::BOOL) -> super::DWORD);
    unsafe { CredUIConfirmCredentialsW(psztargetname.param().abi(), bconfirm.into()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CredUIParseUserNameA<P0>(username: P0, user: *mut super::CHAR, userbuffersize: super::ULONG, domain: *mut super::CHAR, domainbuffersize: super::ULONG) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("credui.dll" "system" fn CredUIParseUserNameA(username : windows_core::PCSTR, user : *mut super::CHAR, userbuffersize : super::ULONG, domain : *mut super::CHAR, domainbuffersize : super::ULONG) -> super::DWORD);
    unsafe { CredUIParseUserNameA(username.param().abi(), user as _, userbuffersize, domain as _, domainbuffersize) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CredUIParseUserNameW<P0>(username: P0, user: *mut super::WCHAR, userbuffersize: super::ULONG, domain: *mut super::WCHAR, domainbuffersize: super::ULONG) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("credui.dll" "system" fn CredUIParseUserNameW(username : windows_core::PCWSTR, user : *mut super::WCHAR, userbuffersize : super::ULONG, domain : *mut super::WCHAR, domainbuffersize : super::ULONG) -> super::DWORD);
    unsafe { CredUIParseUserNameW(username.param().abi(), user as _, userbuffersize, domain as _, domainbuffersize) }
}
#[cfg(all(feature = "shared", feature = "sspi", feature = "windef"))]
#[inline]
pub unsafe fn CredUIPromptForCredentialsA<P1>(puiinfo: Option<PCREDUI_INFOA>, psztargetname: P1, pcontext: super::PCtxtHandle, dwautherror: super::DWORD, pszusername: &mut [u8], pszpassword: &mut [u8], save: Option<*mut windows_core::BOOL>, dwflags: super::DWORD) -> super::DWORD
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("credui.dll" "system" fn CredUIPromptForCredentialsA(puiinfo : PCREDUI_INFOA, psztargetname : windows_core::PCSTR, pcontext : super::PCtxtHandle, dwautherror : super::DWORD, pszusername : windows_core::PSTR, ulusernamebuffersize : super::ULONG, pszpassword : windows_core::PSTR, ulpasswordbuffersize : super::ULONG, save : *mut windows_core::BOOL, dwflags : super::DWORD) -> super::DWORD);
    unsafe { CredUIPromptForCredentialsA(puiinfo.unwrap_or(core::mem::zeroed()) as _, psztargetname.param().abi(), pcontext as _, dwautherror, core::mem::transmute(pszusername.as_mut_ptr()), pszusername.len().try_into().unwrap(), core::mem::transmute(pszpassword.as_mut_ptr()), pszpassword.len().try_into().unwrap(), save.unwrap_or(core::mem::zeroed()) as _, dwflags) }
}
#[cfg(all(feature = "shared", feature = "sspi", feature = "windef"))]
#[inline]
pub unsafe fn CredUIPromptForCredentialsW<P1>(puiinfo: Option<PCREDUI_INFOW>, psztargetname: P1, pcontext: super::PCtxtHandle, dwautherror: super::DWORD, pszusername: &mut [u16], pszpassword: &mut [u16], save: Option<*mut windows_core::BOOL>, dwflags: super::DWORD) -> super::DWORD
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("credui.dll" "system" fn CredUIPromptForCredentialsW(puiinfo : PCREDUI_INFOW, psztargetname : windows_core::PCWSTR, pcontext : super::PCtxtHandle, dwautherror : super::DWORD, pszusername : windows_core::PWSTR, ulusernamebuffersize : super::ULONG, pszpassword : windows_core::PWSTR, ulpasswordbuffersize : super::ULONG, save : *mut windows_core::BOOL, dwflags : super::DWORD) -> super::DWORD);
    unsafe { CredUIPromptForCredentialsW(puiinfo.unwrap_or(core::mem::zeroed()) as _, psztargetname.param().abi(), pcontext as _, dwautherror, core::mem::transmute(pszusername.as_mut_ptr()), pszusername.len().try_into().unwrap(), core::mem::transmute(pszpassword.as_mut_ptr()), pszpassword.len().try_into().unwrap(), save.unwrap_or(core::mem::zeroed()) as _, dwflags) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
#[inline]
pub unsafe fn CredUIPromptForWindowsCredentialsA(puiinfo: Option<PCREDUI_INFOA>, dwautherror: super::DWORD, pulauthpackage: *mut super::ULONG, pvinauthbuffer: Option<super::LPCVOID>, ulinauthbuffersize: super::ULONG, ppvoutauthbuffer: *mut super::LPVOID, puloutauthbuffersize: *mut super::ULONG, pfsave: Option<*mut windows_core::BOOL>, dwflags: super::DWORD) -> super::DWORD {
    windows_core::link!("credui.dll" "system" fn CredUIPromptForWindowsCredentialsA(puiinfo : PCREDUI_INFOA, dwautherror : super::DWORD, pulauthpackage : *mut super::ULONG, pvinauthbuffer : super::LPCVOID, ulinauthbuffersize : super::ULONG, ppvoutauthbuffer : *mut super::LPVOID, puloutauthbuffersize : *mut super::ULONG, pfsave : *mut windows_core::BOOL, dwflags : super::DWORD) -> super::DWORD);
    unsafe { CredUIPromptForWindowsCredentialsA(puiinfo.unwrap_or(core::mem::zeroed()) as _, dwautherror, pulauthpackage as _, pvinauthbuffer.unwrap_or(core::mem::zeroed()) as _, ulinauthbuffersize, ppvoutauthbuffer as _, puloutauthbuffersize as _, pfsave.unwrap_or(core::mem::zeroed()) as _, dwflags) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
#[inline]
pub unsafe fn CredUIPromptForWindowsCredentialsW(puiinfo: Option<PCREDUI_INFOW>, dwautherror: super::DWORD, pulauthpackage: *mut super::ULONG, pvinauthbuffer: Option<super::LPCVOID>, ulinauthbuffersize: super::ULONG, ppvoutauthbuffer: *mut super::LPVOID, puloutauthbuffersize: *mut super::ULONG, pfsave: Option<*mut windows_core::BOOL>, dwflags: super::DWORD) -> super::DWORD {
    windows_core::link!("credui.dll" "system" fn CredUIPromptForWindowsCredentialsW(puiinfo : PCREDUI_INFOW, dwautherror : super::DWORD, pulauthpackage : *mut super::ULONG, pvinauthbuffer : super::LPCVOID, ulinauthbuffersize : super::ULONG, ppvoutauthbuffer : *mut super::LPVOID, puloutauthbuffersize : *mut super::ULONG, pfsave : *mut windows_core::BOOL, dwflags : super::DWORD) -> super::DWORD);
    unsafe { CredUIPromptForWindowsCredentialsW(puiinfo.unwrap_or(core::mem::zeroed()) as _, dwautherror, pulauthpackage as _, pvinauthbuffer.unwrap_or(core::mem::zeroed()) as _, ulinauthbuffersize, ppvoutauthbuffer as _, puloutauthbuffersize as _, pfsave.unwrap_or(core::mem::zeroed()) as _, dwflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CredUIReadSSOCredW<P0>(pszrealm: P0, ppszusername: *mut windows_core::PWSTR) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("credui.dll" "system" fn CredUIReadSSOCredW(pszrealm : windows_core::PCWSTR, ppszusername : *mut windows_core::PWSTR) -> super::DWORD);
    unsafe { CredUIReadSSOCredW(pszrealm.param().abi(), ppszusername as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CredUIStoreSSOCredW<P0, P1, P2>(pszrealm: P0, pszusername: P1, pszpassword: P2, bpersist: bool) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("credui.dll" "system" fn CredUIStoreSSOCredW(pszrealm : windows_core::PCWSTR, pszusername : windows_core::PCWSTR, pszpassword : windows_core::PCWSTR, bpersist : windows_core::BOOL) -> super::DWORD);
    unsafe { CredUIStoreSSOCredW(pszrealm.param().abi(), pszusername.param().abi(), pszpassword.param().abi(), bpersist.into()) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CredUnPackAuthenticationBufferA(dwflags: super::DWORD, pauthbuffer: super::PVOID, cbauthbuffer: super::DWORD, pszusername: Option<windows_core::PSTR>, pcchlmaxusername: *mut super::DWORD, pszdomainname: Option<windows_core::PSTR>, pcchmaxdomainname: Option<*mut super::DWORD>, pszpassword: Option<windows_core::PSTR>, pcchmaxpassword: *mut super::DWORD) -> windows_core::BOOL {
    windows_core::link!("credui.dll" "system" fn CredUnPackAuthenticationBufferA(dwflags : super::DWORD, pauthbuffer : super::PVOID, cbauthbuffer : super::DWORD, pszusername : windows_core::PSTR, pcchlmaxusername : *mut super::DWORD, pszdomainname : windows_core::PSTR, pcchmaxdomainname : *mut super::DWORD, pszpassword : windows_core::PSTR, pcchmaxpassword : *mut super::DWORD) -> windows_core::BOOL);
    unsafe { CredUnPackAuthenticationBufferA(dwflags, pauthbuffer, cbauthbuffer, pszusername.unwrap_or(core::mem::zeroed()) as _, pcchlmaxusername as _, pszdomainname.unwrap_or(core::mem::zeroed()) as _, pcchmaxdomainname.unwrap_or(core::mem::zeroed()) as _, pszpassword.unwrap_or(core::mem::zeroed()) as _, pcchmaxpassword as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CredUnPackAuthenticationBufferW(dwflags: super::DWORD, pauthbuffer: super::PVOID, cbauthbuffer: super::DWORD, pszusername: Option<windows_core::PWSTR>, pcchmaxusername: *mut super::DWORD, pszdomainname: Option<windows_core::PWSTR>, pcchmaxdomainname: Option<*mut super::DWORD>, pszpassword: Option<windows_core::PWSTR>, pcchmaxpassword: *mut super::DWORD) -> windows_core::BOOL {
    windows_core::link!("credui.dll" "system" fn CredUnPackAuthenticationBufferW(dwflags : super::DWORD, pauthbuffer : super::PVOID, cbauthbuffer : super::DWORD, pszusername : windows_core::PWSTR, pcchmaxusername : *mut super::DWORD, pszdomainname : windows_core::PWSTR, pcchmaxdomainname : *mut super::DWORD, pszpassword : windows_core::PWSTR, pcchmaxpassword : *mut super::DWORD) -> windows_core::BOOL);
    unsafe { CredUnPackAuthenticationBufferW(dwflags, pauthbuffer, cbauthbuffer, pszusername.unwrap_or(core::mem::zeroed()) as _, pcchmaxusername as _, pszdomainname.unwrap_or(core::mem::zeroed()) as _, pcchmaxdomainname.unwrap_or(core::mem::zeroed()) as _, pszpassword.unwrap_or(core::mem::zeroed()) as _, pcchmaxpassword as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CredUnmarshalCredentialA<P0>(marshaledcredential: P0, credtype: PCRED_MARSHAL_TYPE, credential: *mut super::PVOID) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn CredUnmarshalCredentialA(marshaledcredential : windows_core::PCSTR, credtype : PCRED_MARSHAL_TYPE, credential : *mut super::PVOID) -> windows_core::BOOL);
    unsafe { CredUnmarshalCredentialA(marshaledcredential.param().abi(), credtype as _, credential as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CredUnmarshalCredentialW<P0>(marshaledcredential: P0, credtype: PCRED_MARSHAL_TYPE, credential: *mut super::PVOID) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn CredUnmarshalCredentialW(marshaledcredential : windows_core::PCWSTR, credtype : PCRED_MARSHAL_TYPE, credential : *mut super::PVOID) -> windows_core::BOOL);
    unsafe { CredUnmarshalCredentialW(marshaledcredential.param().abi(), credtype as _, credential as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CredUnprotectA(fasself: bool, pszprotectedcredentials: &[u8], pszcredentials: Option<windows_core::PSTR>, pcchmaxchars: *mut super::DWORD) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn CredUnprotectA(fasself : windows_core::BOOL, pszprotectedcredentials : windows_core::PCSTR, cchprotectedcredentials : super::DWORD, pszcredentials : windows_core::PSTR, pcchmaxchars : *mut super::DWORD) -> windows_core::BOOL);
    unsafe { CredUnprotectA(fasself.into(), core::mem::transmute(pszprotectedcredentials.as_ptr()), pszprotectedcredentials.len().try_into().unwrap(), pszcredentials.unwrap_or(core::mem::zeroed()) as _, pcchmaxchars as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CredUnprotectW(fasself: bool, pszprotectedcredentials: &[u16], pszcredentials: Option<windows_core::PWSTR>, pcchmaxchars: *mut super::DWORD) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn CredUnprotectW(fasself : windows_core::BOOL, pszprotectedcredentials : windows_core::PCWSTR, cchprotectedcredentials : super::DWORD, pszcredentials : windows_core::PWSTR, pcchmaxchars : *mut super::DWORD) -> windows_core::BOOL);
    unsafe { CredUnprotectW(fasself.into(), core::mem::transmute(pszprotectedcredentials.as_ptr()), pszprotectedcredentials.len().try_into().unwrap(), pszcredentials.unwrap_or(core::mem::zeroed()) as _, pcchmaxchars as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CredWriteA(credential: PCREDENTIALA, flags: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn CredWriteA(credential : PCREDENTIALA, flags : super::DWORD) -> windows_core::BOOL);
    unsafe { CredWriteA(credential, flags) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CredWriteDomainCredentialsA(targetinfo: PCREDENTIAL_TARGET_INFORMATIONA, credential: PCREDENTIALA, flags: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn CredWriteDomainCredentialsA(targetinfo : PCREDENTIAL_TARGET_INFORMATIONA, credential : PCREDENTIALA, flags : super::DWORD) -> windows_core::BOOL);
    unsafe { CredWriteDomainCredentialsA(targetinfo, credential, flags) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CredWriteDomainCredentialsW(targetinfo: PCREDENTIAL_TARGET_INFORMATIONW, credential: PCREDENTIALW, flags: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn CredWriteDomainCredentialsW(targetinfo : PCREDENTIAL_TARGET_INFORMATIONW, credential : PCREDENTIALW, flags : super::DWORD) -> windows_core::BOOL);
    unsafe { CredWriteDomainCredentialsW(targetinfo, credential, flags) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CredWriteW(credential: PCREDENTIALW, flags: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn CredWriteW(credential : PCREDENTIALW, flags : super::DWORD) -> windows_core::BOOL);
    unsafe { CredWriteW(credential, flags) }
}
pub const BACK_BUTTON_IDENTIFY_AUTH_PACKAGE: u32 = 3402629121;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BINARY_BLOB_CREDENTIAL_INFO {
    pub cbBlob: super::ULONG,
    pub pbBlob: super::LPBYTE,
}
pub const BinaryBlobCredential: CRED_MARSHAL_TYPE = 3;
pub const BinaryBlobForSystem: CRED_MARSHAL_TYPE = 5;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CERT_CREDENTIAL_INFO {
    pub cbSize: super::ULONG,
    pub rgbHashOfCert: [super::UCHAR; 20],
}
#[cfg(feature = "shared")]
impl Default for CERT_CREDENTIAL_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_HASH_LENGTH: i32 = 20;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type CREDENTIAL = CREDENTIALA;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CREDENTIALA {
    pub Flags: super::DWORD,
    pub Type: super::DWORD,
    pub TargetName: windows_core::PSTR,
    pub Comment: windows_core::PSTR,
    pub LastWritten: super::FILETIME,
    pub CredentialBlobSize: super::DWORD,
    pub CredentialBlob: super::LPBYTE,
    pub Persist: super::DWORD,
    pub AttributeCount: super::DWORD,
    pub Attributes: PCREDENTIAL_ATTRIBUTEA,
    pub TargetAlias: windows_core::PSTR,
    pub UserName: windows_core::PSTR,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CREDENTIALW {
    pub Flags: super::DWORD,
    pub Type: super::DWORD,
    pub TargetName: windows_core::PWSTR,
    pub Comment: windows_core::PWSTR,
    pub LastWritten: super::FILETIME,
    pub CredentialBlobSize: super::DWORD,
    pub CredentialBlob: super::LPBYTE,
    pub Persist: super::DWORD,
    pub AttributeCount: super::DWORD,
    pub Attributes: PCREDENTIAL_ATTRIBUTEW,
    pub TargetAlias: windows_core::PWSTR,
    pub UserName: windows_core::PWSTR,
}
#[cfg(feature = "shared")]
pub type CREDENTIAL_ATTRIBUTE = CREDENTIAL_ATTRIBUTEA;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CREDENTIAL_ATTRIBUTEA {
    pub Keyword: windows_core::PSTR,
    pub Flags: super::DWORD,
    pub ValueSize: super::DWORD,
    pub Value: super::LPBYTE,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CREDENTIAL_ATTRIBUTEW {
    pub Keyword: windows_core::PWSTR,
    pub Flags: super::DWORD,
    pub ValueSize: super::DWORD,
    pub Value: super::LPBYTE,
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type CREDENTIAL_TARGET_INFORMATION = CREDENTIAL_TARGET_INFORMATIONA;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CREDENTIAL_TARGET_INFORMATIONA {
    pub TargetName: windows_core::PSTR,
    pub NetbiosServerName: windows_core::PSTR,
    pub DnsServerName: windows_core::PSTR,
    pub NetbiosDomainName: windows_core::PSTR,
    pub DnsDomainName: windows_core::PSTR,
    pub DnsTreeName: windows_core::PSTR,
    pub PackageName: windows_core::PSTR,
    pub Flags: super::ULONG,
    pub CredTypeCount: super::DWORD,
    pub CredTypes: super::LPDWORD,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CREDENTIAL_TARGET_INFORMATIONW {
    pub TargetName: windows_core::PWSTR,
    pub NetbiosServerName: windows_core::PWSTR,
    pub DnsServerName: windows_core::PWSTR,
    pub NetbiosDomainName: windows_core::PWSTR,
    pub DnsDomainName: windows_core::PWSTR,
    pub DnsTreeName: windows_core::PWSTR,
    pub PackageName: windows_core::PWSTR,
    pub Flags: super::ULONG,
    pub CredTypeCount: super::DWORD,
    pub CredTypes: super::LPDWORD,
}
pub const CREDUIWIN_AUTHPACKAGE_ONLY: i32 = 16;
pub const CREDUIWIN_CHECKBOX: i32 = 2;
pub const CREDUIWIN_DOWNLEVEL_HELLO_AS_SMART_CARD: u32 = 2147483648;
pub const CREDUIWIN_ENUMERATE_ADMINS: i32 = 256;
pub const CREDUIWIN_ENUMERATE_CURRENT_USER: i32 = 512;
pub const CREDUIWIN_GENERIC: i32 = 1;
pub const CREDUIWIN_IGNORE_CLOUDAUTHORITY_NAME: i32 = 262144;
pub const CREDUIWIN_IN_CRED_ONLY: i32 = 32;
pub const CREDUIWIN_PACK_32_WOW: i32 = 268435456;
pub const CREDUIWIN_PREPROMPTING: i32 = 8192;
pub const CREDUIWIN_SECURE_PROMPT: i32 = 4096;
pub const CREDUIWIN_USE_V2: i32 = 64;
pub const CREDUIWIN_VALID_FLAGS: u32 = 2416194355;
pub const CREDUI_FLAGS_ALWAYS_SHOW_UI: i32 = 128;
pub const CREDUI_FLAGS_COMPLETE_USERNAME: i32 = 2048;
pub const CREDUI_FLAGS_DO_NOT_PERSIST: i32 = 2;
pub const CREDUI_FLAGS_EXCLUDE_CERTIFICATES: i32 = 8;
pub const CREDUI_FLAGS_EXPECT_CONFIRMATION: i32 = 131072;
pub const CREDUI_FLAGS_GENERIC_CREDENTIALS: i32 = 262144;
pub const CREDUI_FLAGS_INCORRECT_PASSWORD: i32 = 1;
pub const CREDUI_FLAGS_KEEP_USERNAME: i32 = 1048576;
pub const CREDUI_FLAGS_PASSWORD_ONLY_OK: i32 = 512;
pub const CREDUI_FLAGS_PERSIST: i32 = 4096;
pub const CREDUI_FLAGS_PROMPT_VALID: i32 = 1990623;
pub const CREDUI_FLAGS_REQUEST_ADMINISTRATOR: i32 = 4;
pub const CREDUI_FLAGS_REQUIRE_CERTIFICATE: i32 = 16;
pub const CREDUI_FLAGS_REQUIRE_SMARTCARD: i32 = 256;
pub const CREDUI_FLAGS_SERVER_CREDENTIAL: i32 = 16384;
pub const CREDUI_FLAGS_SHOW_SAVE_CHECK_BOX: i32 = 64;
pub const CREDUI_FLAGS_USERNAME_TARGET_CREDENTIALS: i32 = 524288;
pub const CREDUI_FLAGS_VALIDATE_USERNAME: i32 = 1024;
pub const CREDUI_FOOTER_LINK_AUTHPACKAGE_ID: i32 = 212664322;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type CREDUI_INFO = CREDUI_INFOA;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CREDUI_INFOA {
    pub cbSize: super::DWORD,
    pub hwndParent: super::HWND,
    pub pszMessageText: windows_core::PCSTR,
    pub pszCaptionText: windows_core::PCSTR,
    pub hbmBanner: super::HBITMAP,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CREDUI_INFOW {
    pub cbSize: super::DWORD,
    pub hwndParent: super::HWND,
    pub pszMessageText: windows_core::PCWSTR,
    pub pszCaptionText: windows_core::PCWSTR,
    pub hbmBanner: super::HBITMAP,
}
pub const CREDUI_MAX_CAPTION_LENGTH: i32 = 128;
pub const CREDUI_MAX_DOMAIN_TARGET_LENGTH: i32 = 337;
pub const CREDUI_MAX_GENERIC_TARGET_LENGTH: i32 = 32767;
pub const CREDUI_MAX_MESSAGE_LENGTH: i32 = 1024;
pub const CREDUI_MAX_PASSWORD_LENGTH: i32 = 256;
pub const CREDUI_MAX_USERNAME_LENGTH: i32 = 513;
pub const CREDUI_PICKERSCREEN_AUTHPACKAGE_ID: i32 = 212664323;
pub const CRED_ALLOW_NAME_RESOLUTION: i32 = 1;
pub const CRED_CACHE_TARGET_INFORMATION: i32 = 1;
pub const CRED_ENUMERATE_ALL_CREDENTIALS: i32 = 1;
pub const CRED_FLAGS_NGC_CERT: i32 = 128;
pub const CRED_FLAGS_OWF_CRED_BLOB: i32 = 8;
pub const CRED_FLAGS_PASSWORD_FOR_CERT: i32 = 1;
pub const CRED_FLAGS_PROMPT_NOW: i32 = 2;
pub const CRED_FLAGS_REQUIRE_CONFIRMATION: i32 = 16;
pub const CRED_FLAGS_USERNAME_TARGET: i32 = 4;
pub const CRED_FLAGS_VALID_FLAGS: i32 = 61695;
pub const CRED_FLAGS_VALID_INPUT_FLAGS: i32 = 61599;
pub const CRED_FLAGS_VSM_PROTECTED: i32 = 64;
pub const CRED_FLAGS_WILDCARD_MATCH: i32 = 32;
pub const CRED_LOGON_TYPES_MASK: i32 = 61440;
pub type CRED_MARSHAL_TYPE = i32;
pub const CRED_MAX_ATTRIBUTES: i32 = 64;
pub const CRED_MAX_CREDENTIAL_BLOB_SIZE: i32 = 2560;
pub const CRED_MAX_DOMAIN_TARGET_NAME_LENGTH: i32 = 337;
pub const CRED_MAX_GENERIC_TARGET_NAME_LENGTH: i32 = 32767;
pub const CRED_MAX_STRING_LENGTH: i32 = 256;
pub const CRED_MAX_TARGETNAME_ATTRIBUTE_LENGTH: i32 = 256;
pub const CRED_MAX_TARGETNAME_NAMESPACE_LENGTH: i32 = 256;
pub const CRED_MAX_USERNAME_LENGTH: i32 = 513;
pub const CRED_MAX_VALUE_SIZE: i32 = 256;
pub const CRED_PACK_GENERIC_CREDENTIALS: i32 = 4;
pub const CRED_PACK_ID_PROVIDER_CREDENTIALS: i32 = 8;
pub const CRED_PACK_PROTECTED_CREDENTIALS: i32 = 1;
pub const CRED_PACK_WOW_BUFFER: i32 = 2;
pub const CRED_PERSIST_ENTERPRISE: i32 = 3;
pub const CRED_PERSIST_LOCAL_MACHINE: i32 = 2;
pub const CRED_PERSIST_NONE: i32 = 0;
pub const CRED_PERSIST_SESSION: i32 = 1;
pub const CRED_PRESERVE_CREDENTIAL_BLOB: i32 = 1;
pub type CRED_PROTECTION_TYPE = i32;
pub const CRED_PROTECT_AS_SELF: i32 = 1;
pub const CRED_PROTECT_TO_SYSTEM: i32 = 2;
pub const CRED_PROTECT_VALID_FLAGS: i32 = 3;
pub const CRED_SESSION_WILDCARD_NAME: windows_core::PCSTR = windows_core::s!("*Session");
pub const CRED_SESSION_WILDCARD_NAME_A: windows_core::PCSTR = windows_core::s!("*Session");
#[cfg(target_arch = "x86")]
pub const CRED_SESSION_WILDCARD_NAME_LENGTH: u32 = 8;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const CRED_SESSION_WILDCARD_NAME_LENGTH: u64 = 8;
pub const CRED_SESSION_WILDCARD_NAME_W: windows_core::PCWSTR = windows_core::w!("*Session");
pub const CRED_TARGETNAME_ATTRIBUTE_BATCH: windows_core::PCSTR = windows_core::s!("batch");
pub const CRED_TARGETNAME_ATTRIBUTE_BATCH_A: windows_core::PCSTR = windows_core::s!("batch");
#[cfg(target_arch = "x86")]
pub const CRED_TARGETNAME_ATTRIBUTE_BATCH_LENGTH: u32 = 5;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const CRED_TARGETNAME_ATTRIBUTE_BATCH_LENGTH: u64 = 5;
pub const CRED_TARGETNAME_ATTRIBUTE_BATCH_W: windows_core::PCWSTR = windows_core::w!("batch");
pub const CRED_TARGETNAME_ATTRIBUTE_CACHEDINTERACTIVE: windows_core::PCSTR = windows_core::s!("cachedinteractive");
pub const CRED_TARGETNAME_ATTRIBUTE_CACHEDINTERACTIVE_A: windows_core::PCSTR = windows_core::s!("cachedinteractive");
#[cfg(target_arch = "x86")]
pub const CRED_TARGETNAME_ATTRIBUTE_CACHEDINTERACTIVE_LENGTH: u32 = 17;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const CRED_TARGETNAME_ATTRIBUTE_CACHEDINTERACTIVE_LENGTH: u64 = 17;
pub const CRED_TARGETNAME_ATTRIBUTE_CACHEDINTERACTIVE_W: windows_core::PCWSTR = windows_core::w!("cachedinteractive");
pub const CRED_TARGETNAME_ATTRIBUTE_INTERACTIVE: windows_core::PCSTR = windows_core::s!("interactive");
pub const CRED_TARGETNAME_ATTRIBUTE_INTERACTIVE_A: windows_core::PCSTR = windows_core::s!("interactive");
#[cfg(target_arch = "x86")]
pub const CRED_TARGETNAME_ATTRIBUTE_INTERACTIVE_LENGTH: u32 = 11;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const CRED_TARGETNAME_ATTRIBUTE_INTERACTIVE_LENGTH: u64 = 11;
pub const CRED_TARGETNAME_ATTRIBUTE_INTERACTIVE_W: windows_core::PCWSTR = windows_core::w!("interactive");
pub const CRED_TARGETNAME_ATTRIBUTE_NAME: windows_core::PCSTR = windows_core::s!("name");
pub const CRED_TARGETNAME_ATTRIBUTE_NAME_A: windows_core::PCSTR = windows_core::s!("name");
#[cfg(target_arch = "x86")]
pub const CRED_TARGETNAME_ATTRIBUTE_NAME_LENGTH: u32 = 4;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const CRED_TARGETNAME_ATTRIBUTE_NAME_LENGTH: u64 = 4;
pub const CRED_TARGETNAME_ATTRIBUTE_NAME_W: windows_core::PCWSTR = windows_core::w!("name");
pub const CRED_TARGETNAME_ATTRIBUTE_NETWORK: windows_core::PCSTR = windows_core::s!("network");
pub const CRED_TARGETNAME_ATTRIBUTE_NETWORKCLEARTEXT: windows_core::PCSTR = windows_core::s!("networkcleartext");
pub const CRED_TARGETNAME_ATTRIBUTE_NETWORKCLEARTEXT_A: windows_core::PCSTR = windows_core::s!("networkcleartext");
#[cfg(target_arch = "x86")]
pub const CRED_TARGETNAME_ATTRIBUTE_NETWORKCLEARTEXT_LENGTH: u32 = 16;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const CRED_TARGETNAME_ATTRIBUTE_NETWORKCLEARTEXT_LENGTH: u64 = 16;
pub const CRED_TARGETNAME_ATTRIBUTE_NETWORKCLEARTEXT_W: windows_core::PCWSTR = windows_core::w!("networkcleartext");
pub const CRED_TARGETNAME_ATTRIBUTE_NETWORK_A: windows_core::PCSTR = windows_core::s!("network");
#[cfg(target_arch = "x86")]
pub const CRED_TARGETNAME_ATTRIBUTE_NETWORK_LENGTH: u32 = 7;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const CRED_TARGETNAME_ATTRIBUTE_NETWORK_LENGTH: u64 = 7;
pub const CRED_TARGETNAME_ATTRIBUTE_NETWORK_W: windows_core::PCWSTR = windows_core::w!("network");
pub const CRED_TARGETNAME_ATTRIBUTE_REMOTEINTERACTIVE: windows_core::PCSTR = windows_core::s!("remoteinteractive");
pub const CRED_TARGETNAME_ATTRIBUTE_REMOTEINTERACTIVE_A: windows_core::PCSTR = windows_core::s!("remoteinteractive");
#[cfg(target_arch = "x86")]
pub const CRED_TARGETNAME_ATTRIBUTE_REMOTEINTERACTIVE_LENGTH: u32 = 17;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const CRED_TARGETNAME_ATTRIBUTE_REMOTEINTERACTIVE_LENGTH: u64 = 17;
pub const CRED_TARGETNAME_ATTRIBUTE_REMOTEINTERACTIVE_W: windows_core::PCWSTR = windows_core::w!("remoteinteractive");
pub const CRED_TARGETNAME_ATTRIBUTE_SEPERATOR_A: i8 = 61;
pub const CRED_TARGETNAME_ATTRIBUTE_SEPERATOR_W: u16 = 61;
pub const CRED_TARGETNAME_ATTRIBUTE_SERVICE: windows_core::PCSTR = windows_core::s!("service");
pub const CRED_TARGETNAME_ATTRIBUTE_SERVICE_A: windows_core::PCSTR = windows_core::s!("service");
#[cfg(target_arch = "x86")]
pub const CRED_TARGETNAME_ATTRIBUTE_SERVICE_LENGTH: u32 = 7;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const CRED_TARGETNAME_ATTRIBUTE_SERVICE_LENGTH: u64 = 7;
pub const CRED_TARGETNAME_ATTRIBUTE_SERVICE_W: windows_core::PCWSTR = windows_core::w!("service");
pub const CRED_TARGETNAME_ATTRIBUTE_TARGET: windows_core::PCSTR = windows_core::s!("target");
pub const CRED_TARGETNAME_ATTRIBUTE_TARGET_A: windows_core::PCSTR = windows_core::s!("target");
#[cfg(target_arch = "x86")]
pub const CRED_TARGETNAME_ATTRIBUTE_TARGET_LENGTH: u32 = 6;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const CRED_TARGETNAME_ATTRIBUTE_TARGET_LENGTH: u64 = 6;
pub const CRED_TARGETNAME_ATTRIBUTE_TARGET_W: windows_core::PCWSTR = windows_core::w!("target");
pub const CRED_TARGETNAME_DOMAIN_EXTENDED_USERNAME_SEPARATOR_A: i8 = 124;
pub const CRED_TARGETNAME_DOMAIN_EXTENDED_USERNAME_SEPARATOR_W: u16 = 124;
pub const CRED_TARGETNAME_DOMAIN_NAMESPACE: windows_core::PCSTR = windows_core::s!("Domain");
pub const CRED_TARGETNAME_DOMAIN_NAMESPACE_A: windows_core::PCSTR = windows_core::s!("Domain");
#[cfg(target_arch = "x86")]
pub const CRED_TARGETNAME_DOMAIN_NAMESPACE_LENGTH: u32 = 6;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const CRED_TARGETNAME_DOMAIN_NAMESPACE_LENGTH: u64 = 6;
pub const CRED_TARGETNAME_DOMAIN_NAMESPACE_W: windows_core::PCWSTR = windows_core::w!("Domain");
pub const CRED_TARGETNAME_LEGACYGENERIC_NAMESPACE_A: windows_core::PCSTR = windows_core::s!("LegacyGeneric");
#[cfg(target_arch = "x86")]
pub const CRED_TARGETNAME_LEGACYGENERIC_NAMESPACE_LENGTH: u32 = 13;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const CRED_TARGETNAME_LEGACYGENERIC_NAMESPACE_LENGTH: u64 = 13;
pub const CRED_TARGETNAME_LEGACYGENERIC_NAMESPACE_W: windows_core::PCWSTR = windows_core::w!("LegacyGeneric");
pub const CRED_TARGETNAME_NAMESPACE_SEPERATOR_A: i8 = 58;
pub const CRED_TARGETNAME_NAMESPACE_SEPERATOR_W: u16 = 58;
pub const CRED_TI_CREATE_EXPLICIT_CRED: i32 = 16;
pub const CRED_TI_DNSTREE_IS_DFS_SERVER: i32 = 64;
pub const CRED_TI_DOMAIN_FORMAT_UNKNOWN: i32 = 2;
pub const CRED_TI_ONLY_PASSWORD_REQUIRED: i32 = 4;
pub const CRED_TI_SERVER_FORMAT_UNKNOWN: i32 = 1;
pub const CRED_TI_USERNAME_TARGET: i32 = 8;
pub const CRED_TI_VALID_FLAGS: i32 = 61567;
pub const CRED_TI_WORKGROUP_MEMBER: i32 = 32;
pub const CRED_TYPE_DOMAIN_CERTIFICATE: i32 = 3;
pub const CRED_TYPE_DOMAIN_EXTENDED: i32 = 6;
pub const CRED_TYPE_DOMAIN_PASSWORD: i32 = 2;
pub const CRED_TYPE_DOMAIN_VISIBLE_PASSWORD: i32 = 4;
pub const CRED_TYPE_GENERIC: i32 = 1;
pub const CRED_TYPE_GENERIC_CERTIFICATE: i32 = 5;
pub const CRED_TYPE_MAXIMUM: i32 = 7;
pub const CRED_TYPE_MAXIMUM_EX: i32 = 1007;
pub const CRED_UNIVERSAL_WILDCARD_A: i8 = 42;
pub const CRED_UNIVERSAL_WILDCARD_W: u16 = 42;
pub const CRED_UNPROTECT_ALLOW_TO_SYSTEM: i32 = 2;
pub const CRED_UNPROTECT_AS_SELF: i32 = 1;
pub const CRED_UNPROTECT_VALID_FLAGS: i32 = 3;
pub const CertCredential: CRED_MARSHAL_TYPE = 1;
pub const CredForSystemProtection: CRED_PROTECTION_TYPE = 3;
pub const CredTrustedProtection: CRED_PROTECTION_TYPE = 2;
pub const CredUnprotected: CRED_PROTECTION_TYPE = 0;
pub const CredUserProtection: CRED_PROTECTION_TYPE = 1;
pub const NERR_BASE: i32 = 2100;
pub const NERR_PasswordExpired: i32 = 2242;
#[cfg(feature = "shared")]
pub type PBINARY_BLOB_CREDENTIAL_INFO = *mut BINARY_BLOB_CREDENTIAL_INFO;
#[cfg(feature = "shared")]
pub type PCERT_CREDENTIAL_INFO = *mut CERT_CREDENTIAL_INFO;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PCREDENTIAL = *mut CREDENTIALA;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PCREDENTIALA = *mut CREDENTIALA;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PCREDENTIALW = *mut CREDENTIALW;
#[cfg(feature = "shared")]
pub type PCREDENTIAL_ATTRIBUTE = *mut CREDENTIAL_ATTRIBUTEA;
#[cfg(feature = "shared")]
pub type PCREDENTIAL_ATTRIBUTEA = *mut CREDENTIAL_ATTRIBUTEA;
#[cfg(feature = "shared")]
pub type PCREDENTIAL_ATTRIBUTEW = *mut CREDENTIAL_ATTRIBUTEW;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PCREDENTIAL_TARGET_INFORMATION = *mut CREDENTIAL_TARGET_INFORMATIONA;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PCREDENTIAL_TARGET_INFORMATIONA = *mut CREDENTIAL_TARGET_INFORMATIONA;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PCREDENTIAL_TARGET_INFORMATIONW = *mut CREDENTIAL_TARGET_INFORMATIONW;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PCREDUI_INFO = *mut CREDUI_INFOA;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PCREDUI_INFOA = *mut CREDUI_INFOA;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PCREDUI_INFOW = *mut CREDUI_INFOW;
pub type PCRED_MARSHAL_TYPE = *mut CRED_MARSHAL_TYPE;
pub type PCRED_PROTECTION_TYPE = *mut CRED_PROTECTION_TYPE;
pub type PUSERNAME_TARGET_CREDENTIAL_INFO = *mut USERNAME_TARGET_CREDENTIAL_INFO;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct USERNAME_TARGET_CREDENTIAL_INFO {
    pub UserName: windows_core::PWSTR,
}
pub const UsernameForPackedCredentials: CRED_MARSHAL_TYPE = 4;
pub const UsernameTargetCredential: CRED_MARSHAL_TYPE = 2;
