#[inline]
pub unsafe fn CredDeleteA<P0>(targetname: P0, r#type: u32, flags: Option<u32>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn CredDeleteA(targetname : windows_core::PCSTR, r#type : u32, flags : u32) -> windows_core::BOOL);
    unsafe { CredDeleteA(targetname.param().abi(), r#type, flags.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CredDeleteW<P0>(targetname: P0, r#type: u32, flags: Option<u32>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn CredDeleteW(targetname : windows_core::PCWSTR, r#type : u32, flags : u32) -> windows_core::BOOL);
    unsafe { CredDeleteW(targetname.param().abi(), r#type, flags.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn CredEnumerateA<P0>(filter: P0, flags: Option<u32>, count: *mut u32, credential: *mut *mut PCREDENTIALA) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn CredEnumerateA(filter : windows_core::PCSTR, flags : u32, count : *mut u32, credential : *mut *mut PCREDENTIALA) -> windows_core::BOOL);
    unsafe { CredEnumerateA(filter.param().abi(), flags.unwrap_or(core::mem::zeroed()) as _, count as _, credential as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn CredEnumerateW<P0>(filter: P0, flags: Option<u32>, count: *mut u32, credential: *mut *mut PCREDENTIALW) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn CredEnumerateW(filter : windows_core::PCWSTR, flags : u32, count : *mut u32, credential : *mut *mut PCREDENTIALW) -> windows_core::BOOL);
    unsafe { CredEnumerateW(filter.param().abi(), flags.unwrap_or(core::mem::zeroed()) as _, count as _, credential as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn CredFindBestCredentialA<P0>(targetname: P0, r#type: u32, flags: u32, credential: *mut PCREDENTIALA) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn CredFindBestCredentialA(targetname : windows_core::PCSTR, r#type : u32, flags : u32, credential : *mut PCREDENTIALA) -> windows_core::BOOL);
    unsafe { CredFindBestCredentialA(targetname.param().abi(), r#type, flags, credential as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn CredFindBestCredentialW<P0>(targetname: P0, r#type: u32, flags: u32, credential: *mut PCREDENTIALW) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn CredFindBestCredentialW(targetname : windows_core::PCWSTR, r#type : u32, flags : u32, credential : *mut PCREDENTIALW) -> windows_core::BOOL);
    unsafe { CredFindBestCredentialW(targetname.param().abi(), r#type, flags, credential as _) }
}
#[inline]
pub unsafe fn CredFree(buffer: *const core::ffi::c_void) {
    windows_core::link!("advapi32.dll" "system" fn CredFree(buffer : *const core::ffi::c_void));
    unsafe { CredFree(buffer) }
}
#[inline]
pub unsafe fn CredGetSessionTypes(maximumpersist: &mut [u32]) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn CredGetSessionTypes(maximumpersistcount : u32, maximumpersist : *mut u32) -> windows_core::BOOL);
    unsafe { CredGetSessionTypes(maximumpersist.len().try_into().unwrap(), maximumpersist.as_mut_ptr()) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn CredGetTargetInfoA<P0>(targetname: P0, flags: u32, targetinfo: *mut PCREDENTIAL_TARGET_INFORMATIONA) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn CredGetTargetInfoA(targetname : windows_core::PCSTR, flags : u32, targetinfo : *mut PCREDENTIAL_TARGET_INFORMATIONA) -> windows_core::BOOL);
    unsafe { CredGetTargetInfoA(targetname.param().abi(), flags, targetinfo as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn CredGetTargetInfoW<P0>(targetname: P0, flags: u32, targetinfo: *mut PCREDENTIAL_TARGET_INFORMATIONW) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn CredGetTargetInfoW(targetname : windows_core::PCWSTR, flags : u32, targetinfo : *mut PCREDENTIAL_TARGET_INFORMATIONW) -> windows_core::BOOL);
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
#[inline]
pub unsafe fn CredMarshalCredentialA(credtype: CRED_MARSHAL_TYPE, credential: *const core::ffi::c_void, marshaledcredential: *mut windows_core::PSTR) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn CredMarshalCredentialA(credtype : CRED_MARSHAL_TYPE, credential : *const core::ffi::c_void, marshaledcredential : *mut windows_core::PSTR) -> windows_core::BOOL);
    unsafe { CredMarshalCredentialA(credtype, credential, marshaledcredential as _) }
}
#[inline]
pub unsafe fn CredMarshalCredentialW(credtype: CRED_MARSHAL_TYPE, credential: *const core::ffi::c_void, marshaledcredential: *mut windows_core::PWSTR) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn CredMarshalCredentialW(credtype : CRED_MARSHAL_TYPE, credential : *const core::ffi::c_void, marshaledcredential : *mut windows_core::PWSTR) -> windows_core::BOOL);
    unsafe { CredMarshalCredentialW(credtype, credential, marshaledcredential as _) }
}
#[inline]
pub unsafe fn CredPackAuthenticationBufferA<P1, P2>(dwflags: u32, pszusername: P1, pszpassword: P2, ppackedcredentials: Option<*mut u8>, pcbpackedcredentials: *mut u32) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("credui.dll" "system" fn CredPackAuthenticationBufferA(dwflags : u32, pszusername : windows_core::PCSTR, pszpassword : windows_core::PCSTR, ppackedcredentials : *mut u8, pcbpackedcredentials : *mut u32) -> windows_core::BOOL);
    unsafe { CredPackAuthenticationBufferA(dwflags, pszusername.param().abi(), pszpassword.param().abi(), ppackedcredentials.unwrap_or(core::mem::zeroed()) as _, pcbpackedcredentials as _) }
}
#[inline]
pub unsafe fn CredPackAuthenticationBufferW<P1, P2>(dwflags: u32, pszusername: P1, pszpassword: P2, ppackedcredentials: Option<*mut u8>, pcbpackedcredentials: *mut u32) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("credui.dll" "system" fn CredPackAuthenticationBufferW(dwflags : u32, pszusername : windows_core::PCWSTR, pszpassword : windows_core::PCWSTR, ppackedcredentials : *mut u8, pcbpackedcredentials : *mut u32) -> windows_core::BOOL);
    unsafe { CredPackAuthenticationBufferW(dwflags, pszusername.param().abi(), pszpassword.param().abi(), ppackedcredentials.unwrap_or(core::mem::zeroed()) as _, pcbpackedcredentials as _) }
}
#[inline]
pub unsafe fn CredProtectA(fasself: bool, pszcredentials: &[u8], pszprotectedcredentials: windows_core::PSTR, pcchmaxchars: *mut u32, protectiontype: Option<*mut CRED_PROTECTION_TYPE>) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn CredProtectA(fasself : windows_core::BOOL, pszcredentials : windows_core::PCSTR, cchcredentials : u32, pszprotectedcredentials : windows_core::PSTR, pcchmaxchars : *mut u32, protectiontype : *mut CRED_PROTECTION_TYPE) -> windows_core::BOOL);
    unsafe { CredProtectA(fasself.into(), core::mem::transmute(pszcredentials.as_ptr()), pszcredentials.len().try_into().unwrap(), pszprotectedcredentials, pcchmaxchars as _, protectiontype.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CredProtectW(fasself: bool, pszcredentials: &[u16], pszprotectedcredentials: windows_core::PWSTR, pcchmaxchars: *mut u32, protectiontype: Option<*mut CRED_PROTECTION_TYPE>) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn CredProtectW(fasself : windows_core::BOOL, pszcredentials : windows_core::PCWSTR, cchcredentials : u32, pszprotectedcredentials : windows_core::PWSTR, pcchmaxchars : *mut u32, protectiontype : *mut CRED_PROTECTION_TYPE) -> windows_core::BOOL);
    unsafe { CredProtectW(fasself.into(), core::mem::transmute(pszcredentials.as_ptr()), pszcredentials.len().try_into().unwrap(), pszprotectedcredentials, pcchmaxchars as _, protectiontype.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn CredReadA<P0>(targetname: P0, r#type: u32, flags: Option<u32>, credential: *mut PCREDENTIALA) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn CredReadA(targetname : windows_core::PCSTR, r#type : u32, flags : u32, credential : *mut PCREDENTIALA) -> windows_core::BOOL);
    unsafe { CredReadA(targetname.param().abi(), r#type, flags.unwrap_or(core::mem::zeroed()) as _, credential as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn CredReadDomainCredentialsA(targetinfo: *const CREDENTIAL_TARGET_INFORMATIONA, flags: u32, count: *mut u32, credential: *mut *mut PCREDENTIALA) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn CredReadDomainCredentialsA(targetinfo : *const CREDENTIAL_TARGET_INFORMATIONA, flags : u32, count : *mut u32, credential : *mut *mut PCREDENTIALA) -> windows_core::BOOL);
    unsafe { CredReadDomainCredentialsA(targetinfo, flags, count as _, credential as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn CredReadDomainCredentialsW(targetinfo: *const CREDENTIAL_TARGET_INFORMATIONW, flags: u32, count: *mut u32, credential: *mut *mut PCREDENTIALW) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn CredReadDomainCredentialsW(targetinfo : *const CREDENTIAL_TARGET_INFORMATIONW, flags : u32, count : *mut u32, credential : *mut *mut PCREDENTIALW) -> windows_core::BOOL);
    unsafe { CredReadDomainCredentialsW(targetinfo, flags, count as _, credential as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn CredReadW<P0>(targetname: P0, r#type: u32, flags: Option<u32>, credential: *mut PCREDENTIALW) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn CredReadW(targetname : windows_core::PCWSTR, r#type : u32, flags : u32, credential : *mut PCREDENTIALW) -> windows_core::BOOL);
    unsafe { CredReadW(targetname.param().abi(), r#type, flags.unwrap_or(core::mem::zeroed()) as _, credential as _) }
}
#[inline]
pub unsafe fn CredRenameA<P0, P1>(oldtargetname: P0, newtargetname: P1, r#type: u32, flags: Option<u32>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn CredRenameA(oldtargetname : windows_core::PCSTR, newtargetname : windows_core::PCSTR, r#type : u32, flags : u32) -> windows_core::BOOL);
    unsafe { CredRenameA(oldtargetname.param().abi(), newtargetname.param().abi(), r#type, flags.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CredRenameW<P0, P1>(oldtargetname: P0, newtargetname: P1, r#type: u32, flags: Option<u32>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn CredRenameW(oldtargetname : windows_core::PCWSTR, newtargetname : windows_core::PCWSTR, r#type : u32, flags : u32) -> windows_core::BOOL);
    unsafe { CredRenameW(oldtargetname.param().abi(), newtargetname.param().abi(), r#type, flags.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "sspi")]
#[inline]
pub unsafe fn CredUICmdLinePromptForCredentialsA<P0>(psztargetname: P0, pcontext: Option<super::PCtxtHandle>, dwautherror: u32, username: &mut [u8], pszpassword: &mut [u8], pfsave: Option<*mut windows_core::BOOL>, dwflags: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("credui.dll" "system" fn CredUICmdLinePromptForCredentialsA(psztargetname : windows_core::PCSTR, pcontext : super::PCtxtHandle, dwautherror : u32, username : windows_core::PSTR, uluserbuffersize : u32, pszpassword : windows_core::PSTR, ulpasswordbuffersize : u32, pfsave : *mut windows_core::BOOL, dwflags : u32) -> u32);
    unsafe { CredUICmdLinePromptForCredentialsA(psztargetname.param().abi(), pcontext.unwrap_or(core::mem::zeroed()) as _, dwautherror, core::mem::transmute(username.as_mut_ptr()), username.len().try_into().unwrap(), core::mem::transmute(pszpassword.as_mut_ptr()), pszpassword.len().try_into().unwrap(), pfsave.unwrap_or(core::mem::zeroed()) as _, dwflags) }
}
#[cfg(feature = "sspi")]
#[inline]
pub unsafe fn CredUICmdLinePromptForCredentialsW<P0>(psztargetname: P0, pcontext: Option<super::PCtxtHandle>, dwautherror: u32, username: &mut [u16], pszpassword: &mut [u16], pfsave: Option<*mut windows_core::BOOL>, dwflags: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("credui.dll" "system" fn CredUICmdLinePromptForCredentialsW(psztargetname : windows_core::PCWSTR, pcontext : super::PCtxtHandle, dwautherror : u32, username : windows_core::PWSTR, uluserbuffersize : u32, pszpassword : windows_core::PWSTR, ulpasswordbuffersize : u32, pfsave : *mut windows_core::BOOL, dwflags : u32) -> u32);
    unsafe { CredUICmdLinePromptForCredentialsW(psztargetname.param().abi(), pcontext.unwrap_or(core::mem::zeroed()) as _, dwautherror, core::mem::transmute(username.as_mut_ptr()), username.len().try_into().unwrap(), core::mem::transmute(pszpassword.as_mut_ptr()), pszpassword.len().try_into().unwrap(), pfsave.unwrap_or(core::mem::zeroed()) as _, dwflags) }
}
#[inline]
pub unsafe fn CredUIConfirmCredentialsA<P0>(psztargetname: P0, bconfirm: bool) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("credui.dll" "system" fn CredUIConfirmCredentialsA(psztargetname : windows_core::PCSTR, bconfirm : windows_core::BOOL) -> u32);
    unsafe { CredUIConfirmCredentialsA(psztargetname.param().abi(), bconfirm.into()) }
}
#[inline]
pub unsafe fn CredUIConfirmCredentialsW<P0>(psztargetname: P0, bconfirm: bool) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("credui.dll" "system" fn CredUIConfirmCredentialsW(psztargetname : windows_core::PCWSTR, bconfirm : windows_core::BOOL) -> u32);
    unsafe { CredUIConfirmCredentialsW(psztargetname.param().abi(), bconfirm.into()) }
}
#[inline]
pub unsafe fn CredUIParseUserNameA<P0>(username: P0, user: &mut [i8], domain: &mut [i8]) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("credui.dll" "system" fn CredUIParseUserNameA(username : windows_core::PCSTR, user : *mut i8, userbuffersize : u32, domain : *mut i8, domainbuffersize : u32) -> u32);
    unsafe { CredUIParseUserNameA(username.param().abi(), user.as_mut_ptr(), user.len().try_into().unwrap(), domain.as_mut_ptr(), domain.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn CredUIParseUserNameW<P0>(username: P0, user: &mut [u16], domain: &mut [u16]) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("credui.dll" "system" fn CredUIParseUserNameW(username : windows_core::PCWSTR, user : *mut u16, userbuffersize : u32, domain : *mut u16, domainbuffersize : u32) -> u32);
    unsafe { CredUIParseUserNameW(username.param().abi(), user.as_mut_ptr(), user.len().try_into().unwrap(), domain.as_mut_ptr(), domain.len().try_into().unwrap()) }
}
#[cfg(all(feature = "sspi", feature = "windef"))]
#[inline]
pub unsafe fn CredUIPromptForCredentialsA<P1>(puiinfo: Option<*const CREDUI_INFOA>, psztargetname: P1, pcontext: Option<super::PCtxtHandle>, dwautherror: u32, pszusername: &mut [u8], pszpassword: &mut [u8], save: Option<*mut windows_core::BOOL>, dwflags: u32) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("credui.dll" "system" fn CredUIPromptForCredentialsA(puiinfo : *const CREDUI_INFOA, psztargetname : windows_core::PCSTR, pcontext : super::PCtxtHandle, dwautherror : u32, pszusername : windows_core::PSTR, ulusernamebuffersize : u32, pszpassword : windows_core::PSTR, ulpasswordbuffersize : u32, save : *mut windows_core::BOOL, dwflags : u32) -> u32);
    unsafe { CredUIPromptForCredentialsA(puiinfo.unwrap_or(core::mem::zeroed()) as _, psztargetname.param().abi(), pcontext.unwrap_or(core::mem::zeroed()) as _, dwautherror, core::mem::transmute(pszusername.as_mut_ptr()), pszusername.len().try_into().unwrap(), core::mem::transmute(pszpassword.as_mut_ptr()), pszpassword.len().try_into().unwrap(), save.unwrap_or(core::mem::zeroed()) as _, dwflags) }
}
#[cfg(all(feature = "sspi", feature = "windef"))]
#[inline]
pub unsafe fn CredUIPromptForCredentialsW<P1>(puiinfo: Option<*const CREDUI_INFOW>, psztargetname: P1, pcontext: Option<super::PCtxtHandle>, dwautherror: u32, pszusername: &mut [u16], pszpassword: &mut [u16], save: Option<*mut windows_core::BOOL>, dwflags: u32) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("credui.dll" "system" fn CredUIPromptForCredentialsW(puiinfo : *const CREDUI_INFOW, psztargetname : windows_core::PCWSTR, pcontext : super::PCtxtHandle, dwautherror : u32, pszusername : windows_core::PWSTR, ulusernamebuffersize : u32, pszpassword : windows_core::PWSTR, ulpasswordbuffersize : u32, save : *mut windows_core::BOOL, dwflags : u32) -> u32);
    unsafe { CredUIPromptForCredentialsW(puiinfo.unwrap_or(core::mem::zeroed()) as _, psztargetname.param().abi(), pcontext.unwrap_or(core::mem::zeroed()) as _, dwautherror, core::mem::transmute(pszusername.as_mut_ptr()), pszusername.len().try_into().unwrap(), core::mem::transmute(pszpassword.as_mut_ptr()), pszpassword.len().try_into().unwrap(), save.unwrap_or(core::mem::zeroed()) as _, dwflags) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn CredUIPromptForWindowsCredentialsA(puiinfo: Option<*const CREDUI_INFOA>, dwautherror: u32, pulauthpackage: *mut u32, pvinauthbuffer: Option<*const core::ffi::c_void>, ulinauthbuffersize: u32, ppvoutauthbuffer: *mut *mut core::ffi::c_void, puloutauthbuffersize: *mut u32, pfsave: Option<*mut windows_core::BOOL>, dwflags: u32) -> u32 {
    windows_core::link!("credui.dll" "system" fn CredUIPromptForWindowsCredentialsA(puiinfo : *const CREDUI_INFOA, dwautherror : u32, pulauthpackage : *mut u32, pvinauthbuffer : *const core::ffi::c_void, ulinauthbuffersize : u32, ppvoutauthbuffer : *mut *mut core::ffi::c_void, puloutauthbuffersize : *mut u32, pfsave : *mut windows_core::BOOL, dwflags : u32) -> u32);
    unsafe { CredUIPromptForWindowsCredentialsA(puiinfo.unwrap_or(core::mem::zeroed()) as _, dwautherror, pulauthpackage as _, pvinauthbuffer.unwrap_or(core::mem::zeroed()) as _, ulinauthbuffersize, ppvoutauthbuffer as _, puloutauthbuffersize as _, pfsave.unwrap_or(core::mem::zeroed()) as _, dwflags) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn CredUIPromptForWindowsCredentialsW(puiinfo: Option<*const CREDUI_INFOW>, dwautherror: u32, pulauthpackage: *mut u32, pvinauthbuffer: Option<*const core::ffi::c_void>, ulinauthbuffersize: u32, ppvoutauthbuffer: *mut *mut core::ffi::c_void, puloutauthbuffersize: *mut u32, pfsave: Option<*mut windows_core::BOOL>, dwflags: u32) -> u32 {
    windows_core::link!("credui.dll" "system" fn CredUIPromptForWindowsCredentialsW(puiinfo : *const CREDUI_INFOW, dwautherror : u32, pulauthpackage : *mut u32, pvinauthbuffer : *const core::ffi::c_void, ulinauthbuffersize : u32, ppvoutauthbuffer : *mut *mut core::ffi::c_void, puloutauthbuffersize : *mut u32, pfsave : *mut windows_core::BOOL, dwflags : u32) -> u32);
    unsafe { CredUIPromptForWindowsCredentialsW(puiinfo.unwrap_or(core::mem::zeroed()) as _, dwautherror, pulauthpackage as _, pvinauthbuffer.unwrap_or(core::mem::zeroed()) as _, ulinauthbuffersize, ppvoutauthbuffer as _, puloutauthbuffersize as _, pfsave.unwrap_or(core::mem::zeroed()) as _, dwflags) }
}
#[inline]
pub unsafe fn CredUIReadSSOCredW<P0>(pszrealm: P0, ppszusername: *mut windows_core::PWSTR) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("credui.dll" "system" fn CredUIReadSSOCredW(pszrealm : windows_core::PCWSTR, ppszusername : *mut windows_core::PWSTR) -> u32);
    unsafe { CredUIReadSSOCredW(pszrealm.param().abi(), ppszusername as _) }
}
#[inline]
pub unsafe fn CredUIStoreSSOCredW<P0, P1, P2>(pszrealm: P0, pszusername: P1, pszpassword: P2, bpersist: bool) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("credui.dll" "system" fn CredUIStoreSSOCredW(pszrealm : windows_core::PCWSTR, pszusername : windows_core::PCWSTR, pszpassword : windows_core::PCWSTR, bpersist : windows_core::BOOL) -> u32);
    unsafe { CredUIStoreSSOCredW(pszrealm.param().abi(), pszusername.param().abi(), pszpassword.param().abi(), bpersist.into()) }
}
#[inline]
pub unsafe fn CredUnPackAuthenticationBufferA(dwflags: u32, pauthbuffer: *const core::ffi::c_void, cbauthbuffer: u32, pszusername: Option<windows_core::PSTR>, pcchlmaxusername: *mut u32, pszdomainname: Option<windows_core::PSTR>, pcchmaxdomainname: Option<*mut u32>, pszpassword: Option<windows_core::PSTR>, pcchmaxpassword: *mut u32) -> windows_core::BOOL {
    windows_core::link!("credui.dll" "system" fn CredUnPackAuthenticationBufferA(dwflags : u32, pauthbuffer : *const core::ffi::c_void, cbauthbuffer : u32, pszusername : windows_core::PSTR, pcchlmaxusername : *mut u32, pszdomainname : windows_core::PSTR, pcchmaxdomainname : *mut u32, pszpassword : windows_core::PSTR, pcchmaxpassword : *mut u32) -> windows_core::BOOL);
    unsafe { CredUnPackAuthenticationBufferA(dwflags, pauthbuffer, cbauthbuffer, pszusername.unwrap_or(core::mem::zeroed()) as _, pcchlmaxusername as _, pszdomainname.unwrap_or(core::mem::zeroed()) as _, pcchmaxdomainname.unwrap_or(core::mem::zeroed()) as _, pszpassword.unwrap_or(core::mem::zeroed()) as _, pcchmaxpassword as _) }
}
#[inline]
pub unsafe fn CredUnPackAuthenticationBufferW(dwflags: u32, pauthbuffer: *const core::ffi::c_void, cbauthbuffer: u32, pszusername: Option<windows_core::PWSTR>, pcchmaxusername: *mut u32, pszdomainname: Option<windows_core::PWSTR>, pcchmaxdomainname: Option<*mut u32>, pszpassword: Option<windows_core::PWSTR>, pcchmaxpassword: *mut u32) -> windows_core::BOOL {
    windows_core::link!("credui.dll" "system" fn CredUnPackAuthenticationBufferW(dwflags : u32, pauthbuffer : *const core::ffi::c_void, cbauthbuffer : u32, pszusername : windows_core::PWSTR, pcchmaxusername : *mut u32, pszdomainname : windows_core::PWSTR, pcchmaxdomainname : *mut u32, pszpassword : windows_core::PWSTR, pcchmaxpassword : *mut u32) -> windows_core::BOOL);
    unsafe { CredUnPackAuthenticationBufferW(dwflags, pauthbuffer, cbauthbuffer, pszusername.unwrap_or(core::mem::zeroed()) as _, pcchmaxusername as _, pszdomainname.unwrap_or(core::mem::zeroed()) as _, pcchmaxdomainname.unwrap_or(core::mem::zeroed()) as _, pszpassword.unwrap_or(core::mem::zeroed()) as _, pcchmaxpassword as _) }
}
#[inline]
pub unsafe fn CredUnmarshalCredentialA<P0>(marshaledcredential: P0, credtype: *mut CRED_MARSHAL_TYPE, credential: *mut *mut core::ffi::c_void) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn CredUnmarshalCredentialA(marshaledcredential : windows_core::PCSTR, credtype : *mut CRED_MARSHAL_TYPE, credential : *mut *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { CredUnmarshalCredentialA(marshaledcredential.param().abi(), credtype as _, credential as _) }
}
#[inline]
pub unsafe fn CredUnmarshalCredentialW<P0>(marshaledcredential: P0, credtype: *mut CRED_MARSHAL_TYPE, credential: *mut *mut core::ffi::c_void) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn CredUnmarshalCredentialW(marshaledcredential : windows_core::PCWSTR, credtype : *mut CRED_MARSHAL_TYPE, credential : *mut *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { CredUnmarshalCredentialW(marshaledcredential.param().abi(), credtype as _, credential as _) }
}
#[inline]
pub unsafe fn CredUnprotectA(fasself: bool, pszprotectedcredentials: &[u8], pszcredentials: Option<windows_core::PSTR>, pcchmaxchars: *mut u32) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn CredUnprotectA(fasself : windows_core::BOOL, pszprotectedcredentials : windows_core::PCSTR, cchprotectedcredentials : u32, pszcredentials : windows_core::PSTR, pcchmaxchars : *mut u32) -> windows_core::BOOL);
    unsafe { CredUnprotectA(fasself.into(), core::mem::transmute(pszprotectedcredentials.as_ptr()), pszprotectedcredentials.len().try_into().unwrap(), pszcredentials.unwrap_or(core::mem::zeroed()) as _, pcchmaxchars as _) }
}
#[inline]
pub unsafe fn CredUnprotectW(fasself: bool, pszprotectedcredentials: &[u16], pszcredentials: Option<windows_core::PWSTR>, pcchmaxchars: *mut u32) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn CredUnprotectW(fasself : windows_core::BOOL, pszprotectedcredentials : windows_core::PCWSTR, cchprotectedcredentials : u32, pszcredentials : windows_core::PWSTR, pcchmaxchars : *mut u32) -> windows_core::BOOL);
    unsafe { CredUnprotectW(fasself.into(), core::mem::transmute(pszprotectedcredentials.as_ptr()), pszprotectedcredentials.len().try_into().unwrap(), pszcredentials.unwrap_or(core::mem::zeroed()) as _, pcchmaxchars as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn CredWriteA(credential: *const CREDENTIALA, flags: u32) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn CredWriteA(credential : *const CREDENTIALA, flags : u32) -> windows_core::BOOL);
    unsafe { CredWriteA(credential, flags) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn CredWriteDomainCredentialsA(targetinfo: *const CREDENTIAL_TARGET_INFORMATIONA, credential: *const CREDENTIALA, flags: u32) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn CredWriteDomainCredentialsA(targetinfo : *const CREDENTIAL_TARGET_INFORMATIONA, credential : *const CREDENTIALA, flags : u32) -> windows_core::BOOL);
    unsafe { CredWriteDomainCredentialsA(targetinfo, credential, flags) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn CredWriteDomainCredentialsW(targetinfo: *const CREDENTIAL_TARGET_INFORMATIONW, credential: *const CREDENTIALW, flags: u32) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn CredWriteDomainCredentialsW(targetinfo : *const CREDENTIAL_TARGET_INFORMATIONW, credential : *const CREDENTIALW, flags : u32) -> windows_core::BOOL);
    unsafe { CredWriteDomainCredentialsW(targetinfo, credential, flags) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn CredWriteW(credential: *const CREDENTIALW, flags: u32) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn CredWriteW(credential : *const CREDENTIALW, flags : u32) -> windows_core::BOOL);
    unsafe { CredWriteW(credential, flags) }
}
pub const BACK_BUTTON_IDENTIFY_AUTH_PACKAGE: u32 = 3402629121;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BINARY_BLOB_CREDENTIAL_INFO {
    pub cbBlob: u32,
    pub pbBlob: super::LPBYTE,
}
pub const BinaryBlobCredential: CRED_MARSHAL_TYPE = 3;
pub const BinaryBlobForSystem: CRED_MARSHAL_TYPE = 5;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CERT_CREDENTIAL_INFO {
    pub cbSize: u32,
    pub rgbHashOfCert: [u8; 20],
}
impl Default for CERT_CREDENTIAL_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_HASH_LENGTH: u32 = 20;
#[cfg(feature = "minwindef")]
pub type CREDENTIAL = CREDENTIALA;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CREDENTIALA {
    pub Flags: u32,
    pub Type: u32,
    pub TargetName: windows_core::PSTR,
    pub Comment: windows_core::PSTR,
    pub LastWritten: super::FILETIME,
    pub CredentialBlobSize: u32,
    pub CredentialBlob: super::LPBYTE,
    pub Persist: u32,
    pub AttributeCount: u32,
    pub Attributes: PCREDENTIAL_ATTRIBUTEA,
    pub TargetAlias: windows_core::PSTR,
    pub UserName: windows_core::PSTR,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CREDENTIALW {
    pub Flags: u32,
    pub Type: u32,
    pub TargetName: windows_core::PWSTR,
    pub Comment: windows_core::PWSTR,
    pub LastWritten: super::FILETIME,
    pub CredentialBlobSize: u32,
    pub CredentialBlob: super::LPBYTE,
    pub Persist: u32,
    pub AttributeCount: u32,
    pub Attributes: PCREDENTIAL_ATTRIBUTEW,
    pub TargetAlias: windows_core::PWSTR,
    pub UserName: windows_core::PWSTR,
}
#[cfg(feature = "minwindef")]
pub type CREDENTIAL_ATTRIBUTE = CREDENTIAL_ATTRIBUTEA;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CREDENTIAL_ATTRIBUTEA {
    pub Keyword: windows_core::PSTR,
    pub Flags: u32,
    pub ValueSize: u32,
    pub Value: super::LPBYTE,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CREDENTIAL_ATTRIBUTEW {
    pub Keyword: windows_core::PWSTR,
    pub Flags: u32,
    pub ValueSize: u32,
    pub Value: super::LPBYTE,
}
#[cfg(feature = "minwindef")]
pub type CREDENTIAL_TARGET_INFORMATION = CREDENTIAL_TARGET_INFORMATIONA;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CREDENTIAL_TARGET_INFORMATIONA {
    pub TargetName: windows_core::PSTR,
    pub NetbiosServerName: windows_core::PSTR,
    pub DnsServerName: windows_core::PSTR,
    pub NetbiosDomainName: windows_core::PSTR,
    pub DnsDomainName: windows_core::PSTR,
    pub DnsTreeName: windows_core::PSTR,
    pub PackageName: windows_core::PSTR,
    pub Flags: u32,
    pub CredTypeCount: u32,
    pub CredTypes: super::LPDWORD,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CREDENTIAL_TARGET_INFORMATIONW {
    pub TargetName: windows_core::PWSTR,
    pub NetbiosServerName: windows_core::PWSTR,
    pub DnsServerName: windows_core::PWSTR,
    pub NetbiosDomainName: windows_core::PWSTR,
    pub DnsDomainName: windows_core::PWSTR,
    pub DnsTreeName: windows_core::PWSTR,
    pub PackageName: windows_core::PWSTR,
    pub Flags: u32,
    pub CredTypeCount: u32,
    pub CredTypes: super::LPDWORD,
}
pub const CREDUIWIN_AUTHPACKAGE_ONLY: u32 = 16;
pub const CREDUIWIN_CHECKBOX: u32 = 2;
pub const CREDUIWIN_DOWNLEVEL_HELLO_AS_SMART_CARD: u32 = 2147483648;
pub const CREDUIWIN_ENUMERATE_ADMINS: u32 = 256;
pub const CREDUIWIN_ENUMERATE_CURRENT_USER: u32 = 512;
pub const CREDUIWIN_GENERIC: u32 = 1;
pub const CREDUIWIN_IGNORE_CLOUDAUTHORITY_NAME: u32 = 262144;
pub const CREDUIWIN_IN_CRED_ONLY: u32 = 32;
pub const CREDUIWIN_PACK_32_WOW: u32 = 268435456;
pub const CREDUIWIN_PREPROMPTING: u32 = 8192;
pub const CREDUIWIN_SECURE_PROMPT: u32 = 4096;
pub const CREDUIWIN_USE_V2: u32 = 64;
pub const CREDUIWIN_VALID_FLAGS: i32 = -1878772941;
pub const CREDUI_FLAGS_ALWAYS_SHOW_UI: u32 = 128;
pub const CREDUI_FLAGS_COMPLETE_USERNAME: u32 = 2048;
pub const CREDUI_FLAGS_DO_NOT_PERSIST: u32 = 2;
pub const CREDUI_FLAGS_EXCLUDE_CERTIFICATES: u32 = 8;
pub const CREDUI_FLAGS_EXPECT_CONFIRMATION: u32 = 131072;
pub const CREDUI_FLAGS_GENERIC_CREDENTIALS: u32 = 262144;
pub const CREDUI_FLAGS_INCORRECT_PASSWORD: u32 = 1;
pub const CREDUI_FLAGS_KEEP_USERNAME: u32 = 1048576;
pub const CREDUI_FLAGS_PASSWORD_ONLY_OK: u32 = 512;
pub const CREDUI_FLAGS_PERSIST: u32 = 4096;
pub const CREDUI_FLAGS_PROMPT_VALID: u32 = 1990623;
pub const CREDUI_FLAGS_REQUEST_ADMINISTRATOR: u32 = 4;
pub const CREDUI_FLAGS_REQUIRE_CERTIFICATE: u32 = 16;
pub const CREDUI_FLAGS_REQUIRE_SMARTCARD: u32 = 256;
pub const CREDUI_FLAGS_SERVER_CREDENTIAL: u32 = 16384;
pub const CREDUI_FLAGS_SHOW_SAVE_CHECK_BOX: u32 = 64;
pub const CREDUI_FLAGS_USERNAME_TARGET_CREDENTIALS: u32 = 524288;
pub const CREDUI_FLAGS_VALIDATE_USERNAME: u32 = 1024;
pub const CREDUI_FOOTER_LINK_AUTHPACKAGE_ID: u32 = 212664322;
#[cfg(feature = "windef")]
pub type CREDUI_INFO = CREDUI_INFOA;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CREDUI_INFOA {
    pub cbSize: u32,
    pub hwndParent: super::HWND,
    pub pszMessageText: windows_core::PCSTR,
    pub pszCaptionText: windows_core::PCSTR,
    pub hbmBanner: super::HBITMAP,
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CREDUI_INFOW {
    pub cbSize: u32,
    pub hwndParent: super::HWND,
    pub pszMessageText: windows_core::PCWSTR,
    pub pszCaptionText: windows_core::PCWSTR,
    pub hbmBanner: super::HBITMAP,
}
pub const CREDUI_MAX_CAPTION_LENGTH: u32 = 128;
pub const CREDUI_MAX_DOMAIN_TARGET_LENGTH: u32 = 337;
pub const CREDUI_MAX_GENERIC_TARGET_LENGTH: u32 = 32767;
pub const CREDUI_MAX_MESSAGE_LENGTH: u32 = 1024;
pub const CREDUI_MAX_PASSWORD_LENGTH: u32 = 256;
pub const CREDUI_MAX_USERNAME_LENGTH: u32 = 513;
pub const CREDUI_PICKERSCREEN_AUTHPACKAGE_ID: u32 = 212664323;
pub const CRED_ALLOW_NAME_RESOLUTION: u32 = 1;
pub const CRED_CACHE_TARGET_INFORMATION: u32 = 1;
pub const CRED_ENUMERATE_ALL_CREDENTIALS: u32 = 1;
pub const CRED_FLAGS_NGC_CERT: u32 = 128;
pub const CRED_FLAGS_OWF_CRED_BLOB: u32 = 8;
pub const CRED_FLAGS_PASSWORD_FOR_CERT: u32 = 1;
pub const CRED_FLAGS_PROMPT_NOW: u32 = 2;
pub const CRED_FLAGS_REQUIRE_CONFIRMATION: u32 = 16;
pub const CRED_FLAGS_USERNAME_TARGET: u32 = 4;
pub const CRED_FLAGS_VALID_FLAGS: u32 = 61695;
pub const CRED_FLAGS_VALID_INPUT_FLAGS: u32 = 61599;
pub const CRED_FLAGS_VSM_PROTECTED: u32 = 64;
pub const CRED_FLAGS_WILDCARD_MATCH: u32 = 32;
pub const CRED_LOGON_TYPES_MASK: u32 = 61440;
pub type CRED_MARSHAL_TYPE = i32;
pub const CRED_MAX_ATTRIBUTES: u32 = 64;
pub const CRED_MAX_CREDENTIAL_BLOB_SIZE: u32 = 2560;
pub const CRED_MAX_DOMAIN_TARGET_NAME_LENGTH: u32 = 337;
pub const CRED_MAX_GENERIC_TARGET_NAME_LENGTH: u32 = 32767;
pub const CRED_MAX_STRING_LENGTH: u32 = 256;
pub const CRED_MAX_TARGETNAME_ATTRIBUTE_LENGTH: u32 = 256;
pub const CRED_MAX_TARGETNAME_NAMESPACE_LENGTH: u32 = 256;
pub const CRED_MAX_USERNAME_LENGTH: u32 = 513;
pub const CRED_MAX_VALUE_SIZE: u32 = 256;
pub const CRED_PACK_GENERIC_CREDENTIALS: u32 = 4;
pub const CRED_PACK_ID_PROVIDER_CREDENTIALS: u32 = 8;
pub const CRED_PACK_PROTECTED_CREDENTIALS: u32 = 1;
pub const CRED_PACK_WOW_BUFFER: u32 = 2;
pub const CRED_PERSIST_ENTERPRISE: u32 = 3;
pub const CRED_PERSIST_LOCAL_MACHINE: u32 = 2;
pub const CRED_PERSIST_NONE: u32 = 0;
pub const CRED_PERSIST_SESSION: u32 = 1;
pub const CRED_PRESERVE_CREDENTIAL_BLOB: u32 = 1;
pub type CRED_PROTECTION_TYPE = i32;
pub const CRED_PROTECT_AS_SELF: u32 = 1;
pub const CRED_PROTECT_TO_SYSTEM: u32 = 2;
pub const CRED_PROTECT_VALID_FLAGS: u32 = 3;
pub const CRED_SESSION_WILDCARD_NAME_A: windows_core::PCSTR = windows_core::s!("*Session");
pub const CRED_SESSION_WILDCARD_NAME_W: windows_core::PCWSTR = windows_core::w!("*Session");
pub const CRED_TARGETNAME_ATTRIBUTE_BATCH_A: windows_core::PCSTR = windows_core::s!("batch");
pub const CRED_TARGETNAME_ATTRIBUTE_BATCH_W: windows_core::PCWSTR = windows_core::w!("batch");
pub const CRED_TARGETNAME_ATTRIBUTE_CACHEDINTERACTIVE_A: windows_core::PCSTR = windows_core::s!("cachedinteractive");
pub const CRED_TARGETNAME_ATTRIBUTE_CACHEDINTERACTIVE_W: windows_core::PCWSTR = windows_core::w!("cachedinteractive");
pub const CRED_TARGETNAME_ATTRIBUTE_INTERACTIVE_A: windows_core::PCSTR = windows_core::s!("interactive");
pub const CRED_TARGETNAME_ATTRIBUTE_INTERACTIVE_W: windows_core::PCWSTR = windows_core::w!("interactive");
pub const CRED_TARGETNAME_ATTRIBUTE_NAME_A: windows_core::PCSTR = windows_core::s!("name");
pub const CRED_TARGETNAME_ATTRIBUTE_NAME_W: windows_core::PCWSTR = windows_core::w!("name");
pub const CRED_TARGETNAME_ATTRIBUTE_NETWORKCLEARTEXT_A: windows_core::PCSTR = windows_core::s!("networkcleartext");
pub const CRED_TARGETNAME_ATTRIBUTE_NETWORKCLEARTEXT_W: windows_core::PCWSTR = windows_core::w!("networkcleartext");
pub const CRED_TARGETNAME_ATTRIBUTE_NETWORK_A: windows_core::PCSTR = windows_core::s!("network");
pub const CRED_TARGETNAME_ATTRIBUTE_NETWORK_W: windows_core::PCWSTR = windows_core::w!("network");
pub const CRED_TARGETNAME_ATTRIBUTE_REMOTEINTERACTIVE_A: windows_core::PCSTR = windows_core::s!("remoteinteractive");
pub const CRED_TARGETNAME_ATTRIBUTE_REMOTEINTERACTIVE_W: windows_core::PCWSTR = windows_core::w!("remoteinteractive");
pub const CRED_TARGETNAME_ATTRIBUTE_SEPERATOR_A: u32 = 61;
pub const CRED_TARGETNAME_ATTRIBUTE_SEPERATOR_W: u32 = 61;
pub const CRED_TARGETNAME_ATTRIBUTE_SERVICE_A: windows_core::PCSTR = windows_core::s!("service");
pub const CRED_TARGETNAME_ATTRIBUTE_SERVICE_W: windows_core::PCWSTR = windows_core::w!("service");
pub const CRED_TARGETNAME_ATTRIBUTE_TARGET_A: windows_core::PCSTR = windows_core::s!("target");
pub const CRED_TARGETNAME_ATTRIBUTE_TARGET_W: windows_core::PCWSTR = windows_core::w!("target");
pub const CRED_TARGETNAME_DOMAIN_EXTENDED_USERNAME_SEPARATOR_A: u32 = 124;
pub const CRED_TARGETNAME_DOMAIN_EXTENDED_USERNAME_SEPARATOR_W: u32 = 124;
pub const CRED_TARGETNAME_DOMAIN_NAMESPACE_A: windows_core::PCSTR = windows_core::s!("Domain");
pub const CRED_TARGETNAME_DOMAIN_NAMESPACE_W: windows_core::PCWSTR = windows_core::w!("Domain");
pub const CRED_TARGETNAME_LEGACYGENERIC_NAMESPACE_A: windows_core::PCSTR = windows_core::s!("LegacyGeneric");
pub const CRED_TARGETNAME_LEGACYGENERIC_NAMESPACE_W: windows_core::PCWSTR = windows_core::w!("LegacyGeneric");
pub const CRED_TARGETNAME_NAMESPACE_SEPERATOR_A: u32 = 58;
pub const CRED_TARGETNAME_NAMESPACE_SEPERATOR_W: u32 = 58;
pub const CRED_TI_CREATE_EXPLICIT_CRED: u32 = 16;
pub const CRED_TI_DNSTREE_IS_DFS_SERVER: u32 = 64;
pub const CRED_TI_DOMAIN_FORMAT_UNKNOWN: u32 = 2;
pub const CRED_TI_ONLY_PASSWORD_REQUIRED: u32 = 4;
pub const CRED_TI_SERVER_FORMAT_UNKNOWN: u32 = 1;
pub const CRED_TI_USERNAME_TARGET: u32 = 8;
pub const CRED_TI_VALID_FLAGS: u32 = 61567;
pub const CRED_TI_WORKGROUP_MEMBER: u32 = 32;
pub const CRED_TYPE_DOMAIN_CERTIFICATE: u32 = 3;
pub const CRED_TYPE_DOMAIN_EXTENDED: u32 = 6;
pub const CRED_TYPE_DOMAIN_PASSWORD: u32 = 2;
pub const CRED_TYPE_DOMAIN_VISIBLE_PASSWORD: u32 = 4;
pub const CRED_TYPE_GENERIC: u32 = 1;
pub const CRED_TYPE_GENERIC_CERTIFICATE: u32 = 5;
pub const CRED_TYPE_MAXIMUM: u32 = 7;
pub const CRED_TYPE_MAXIMUM_EX: u32 = 1007;
pub const CRED_UNIVERSAL_WILDCARD_A: u32 = 42;
pub const CRED_UNIVERSAL_WILDCARD_W: u32 = 42;
pub const CRED_UNPROTECT_ALLOW_TO_SYSTEM: u32 = 2;
pub const CRED_UNPROTECT_AS_SELF: u32 = 1;
pub const CRED_UNPROTECT_VALID_FLAGS: u32 = 3;
pub const CertCredential: CRED_MARSHAL_TYPE = 1;
pub const CredForSystemProtection: CRED_PROTECTION_TYPE = 3;
pub const CredTrustedProtection: CRED_PROTECTION_TYPE = 2;
pub const CredUnprotected: CRED_PROTECTION_TYPE = 0;
pub const CredUserProtection: CRED_PROTECTION_TYPE = 1;
pub const NERR_BASE: u32 = 2100;
pub const NERR_PasswordExpired: u32 = 2242;
#[cfg(feature = "minwindef")]
pub type PBINARY_BLOB_CREDENTIAL_INFO = *mut BINARY_BLOB_CREDENTIAL_INFO;
pub type PCERT_CREDENTIAL_INFO = *mut CERT_CREDENTIAL_INFO;
#[cfg(feature = "minwindef")]
pub type PCREDENTIAL = PCREDENTIALA;
#[cfg(feature = "minwindef")]
pub type PCREDENTIALA = *mut CREDENTIALA;
#[cfg(feature = "minwindef")]
pub type PCREDENTIALW = *mut CREDENTIALW;
#[cfg(feature = "minwindef")]
pub type PCREDENTIAL_ATTRIBUTE = PCREDENTIAL_ATTRIBUTEA;
#[cfg(feature = "minwindef")]
pub type PCREDENTIAL_ATTRIBUTEA = *mut CREDENTIAL_ATTRIBUTEA;
#[cfg(feature = "minwindef")]
pub type PCREDENTIAL_ATTRIBUTEW = *mut CREDENTIAL_ATTRIBUTEW;
#[cfg(feature = "minwindef")]
pub type PCREDENTIAL_TARGET_INFORMATION = PCREDENTIAL_TARGET_INFORMATIONA;
#[cfg(feature = "minwindef")]
pub type PCREDENTIAL_TARGET_INFORMATIONA = *mut CREDENTIAL_TARGET_INFORMATIONA;
#[cfg(feature = "minwindef")]
pub type PCREDENTIAL_TARGET_INFORMATIONW = *mut CREDENTIAL_TARGET_INFORMATIONW;
#[cfg(feature = "windef")]
pub type PCREDUI_INFO = PCREDUI_INFOA;
#[cfg(feature = "windef")]
pub type PCREDUI_INFOA = *mut CREDUI_INFOA;
#[cfg(feature = "windef")]
pub type PCREDUI_INFOW = *mut CREDUI_INFOW;
pub type PCRED_MARSHAL_TYPE = *mut CRED_MARSHAL_TYPE;
pub type PCRED_PROTECTION_TYPE = *mut CRED_PROTECTION_TYPE;
pub type PUSERNAME_TARGET_CREDENTIAL_INFO = *mut USERNAME_TARGET_CREDENTIAL_INFO;
#[cfg(feature = "bcrypt")]
pub const STATUS_ACCESS_DENIED: super::NTSTATUS = super::NTSTATUS(0xC0000022_u32 as _);
#[cfg(feature = "bcrypt")]
pub const STATUS_ACCOUNT_DISABLED: super::NTSTATUS = super::NTSTATUS(0xC0000072_u32 as _);
#[cfg(feature = "bcrypt")]
pub const STATUS_ACCOUNT_EXPIRED: super::NTSTATUS = super::NTSTATUS(0xC0000193_u32 as _);
#[cfg(feature = "bcrypt")]
pub const STATUS_ACCOUNT_LOCKED_OUT: super::NTSTATUS = super::NTSTATUS(0xC0000234_u32 as _);
#[cfg(feature = "bcrypt")]
pub const STATUS_ACCOUNT_RESTRICTION: super::NTSTATUS = super::NTSTATUS(0xC000006E_u32 as _);
#[cfg(feature = "bcrypt")]
pub const STATUS_AUTHENTICATION_FIREWALL_FAILED: super::NTSTATUS = super::NTSTATUS(0xC0000413_u32 as _);
#[cfg(feature = "bcrypt")]
pub const STATUS_DOWNGRADE_DETECTED: super::NTSTATUS = super::NTSTATUS(0xC0000388_u32 as _);
#[cfg(feature = "bcrypt")]
pub const STATUS_LOGON_FAILURE: super::NTSTATUS = super::NTSTATUS(0xC000006D_u32 as _);
#[cfg(feature = "bcrypt")]
pub const STATUS_LOGON_TYPE_NOT_GRANTED: super::NTSTATUS = super::NTSTATUS(0xC000015B_u32 as _);
#[cfg(feature = "bcrypt")]
pub const STATUS_NO_SUCH_LOGON_SESSION: super::NTSTATUS = super::NTSTATUS(0xC000005F_u32 as _);
#[cfg(feature = "bcrypt")]
pub const STATUS_NO_SUCH_USER: super::NTSTATUS = super::NTSTATUS(0xC0000064_u32 as _);
#[cfg(feature = "bcrypt")]
pub const STATUS_PASSWORD_EXPIRED: super::NTSTATUS = super::NTSTATUS(0xC0000071_u32 as _);
#[cfg(feature = "bcrypt")]
pub const STATUS_PASSWORD_MUST_CHANGE: super::NTSTATUS = super::NTSTATUS(0xC0000224_u32 as _);
#[cfg(feature = "bcrypt")]
pub const STATUS_WRONG_PASSWORD: super::NTSTATUS = super::NTSTATUS(0xC000006A_u32 as _);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct USERNAME_TARGET_CREDENTIAL_INFO {
    pub UserName: windows_core::PWSTR,
}
pub const UsernameForPackedCredentials: CRED_MARSHAL_TYPE = 4;
pub const UsernameTargetCredential: CRED_MARSHAL_TYPE = 2;
