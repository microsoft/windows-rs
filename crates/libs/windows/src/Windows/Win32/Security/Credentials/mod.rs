#[inline]
pub unsafe fn CredDeleteA<P0>(targetname: P0, r#type: CRED_TYPE, flags: Option<u32>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("advapi32.dll" "system" fn CredDeleteA(targetname : windows_core::PCSTR, r#type : CRED_TYPE, flags : u32) -> windows_core::BOOL);
    unsafe { CredDeleteA(targetname.param().abi(), r#type, flags.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn CredDeleteW<P0>(targetname: P0, r#type: CRED_TYPE, flags: Option<u32>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("advapi32.dll" "system" fn CredDeleteW(targetname : windows_core::PCWSTR, r#type : CRED_TYPE, flags : u32) -> windows_core::BOOL);
    unsafe { CredDeleteW(targetname.param().abi(), r#type, flags.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn CredEnumerateA<P0>(filter: P0, flags: Option<CRED_ENUMERATE_FLAGS>, count: *mut u32, credential: *mut *mut *mut CREDENTIALA) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("advapi32.dll" "system" fn CredEnumerateA(filter : windows_core::PCSTR, flags : CRED_ENUMERATE_FLAGS, count : *mut u32, credential : *mut *mut *mut CREDENTIALA) -> windows_core::BOOL);
    unsafe { CredEnumerateA(filter.param().abi(), flags.unwrap_or(core::mem::zeroed()) as _, count as _, credential as _).ok() }
}
#[inline]
pub unsafe fn CredEnumerateW<P0>(filter: P0, flags: Option<CRED_ENUMERATE_FLAGS>, count: *mut u32, credential: *mut *mut *mut CREDENTIALW) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("advapi32.dll" "system" fn CredEnumerateW(filter : windows_core::PCWSTR, flags : CRED_ENUMERATE_FLAGS, count : *mut u32, credential : *mut *mut *mut CREDENTIALW) -> windows_core::BOOL);
    unsafe { CredEnumerateW(filter.param().abi(), flags.unwrap_or(core::mem::zeroed()) as _, count as _, credential as _).ok() }
}
#[inline]
pub unsafe fn CredFindBestCredentialA<P0>(targetname: P0, r#type: u32, flags: u32, credential: *mut *mut CREDENTIALA) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("advapi32.dll" "system" fn CredFindBestCredentialA(targetname : windows_core::PCSTR, r#type : u32, flags : u32, credential : *mut *mut CREDENTIALA) -> windows_core::BOOL);
    unsafe { CredFindBestCredentialA(targetname.param().abi(), r#type, flags, credential as _).ok() }
}
#[inline]
pub unsafe fn CredFindBestCredentialW<P0>(targetname: P0, r#type: u32, flags: u32, credential: *mut *mut CREDENTIALW) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("advapi32.dll" "system" fn CredFindBestCredentialW(targetname : windows_core::PCWSTR, r#type : u32, flags : u32, credential : *mut *mut CREDENTIALW) -> windows_core::BOOL);
    unsafe { CredFindBestCredentialW(targetname.param().abi(), r#type, flags, credential as _).ok() }
}
#[inline]
pub unsafe fn CredFree(buffer: *const core::ffi::c_void) {
    windows_link::link!("advapi32.dll" "system" fn CredFree(buffer : *const core::ffi::c_void));
    unsafe { CredFree(buffer) }
}
#[inline]
pub unsafe fn CredGetSessionTypes(maximumpersist: &mut [u32]) -> windows_core::Result<()> {
    windows_link::link!("advapi32.dll" "system" fn CredGetSessionTypes(maximumpersistcount : u32, maximumpersist : *mut u32) -> windows_core::BOOL);
    unsafe { CredGetSessionTypes(maximumpersist.len().try_into().unwrap(), core::mem::transmute(maximumpersist.as_ptr())).ok() }
}
#[inline]
pub unsafe fn CredGetTargetInfoA<P0>(targetname: P0, flags: u32, targetinfo: *mut *mut CREDENTIAL_TARGET_INFORMATIONA) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("advapi32.dll" "system" fn CredGetTargetInfoA(targetname : windows_core::PCSTR, flags : u32, targetinfo : *mut *mut CREDENTIAL_TARGET_INFORMATIONA) -> windows_core::BOOL);
    unsafe { CredGetTargetInfoA(targetname.param().abi(), flags, targetinfo as _).ok() }
}
#[inline]
pub unsafe fn CredGetTargetInfoW<P0>(targetname: P0, flags: u32, targetinfo: *mut *mut CREDENTIAL_TARGET_INFORMATIONW) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("advapi32.dll" "system" fn CredGetTargetInfoW(targetname : windows_core::PCWSTR, flags : u32, targetinfo : *mut *mut CREDENTIAL_TARGET_INFORMATIONW) -> windows_core::BOOL);
    unsafe { CredGetTargetInfoW(targetname.param().abi(), flags, targetinfo as _).ok() }
}
#[inline]
pub unsafe fn CredIsMarshaledCredentialA<P0>(marshaledcredential: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("advapi32.dll" "system" fn CredIsMarshaledCredentialA(marshaledcredential : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { CredIsMarshaledCredentialA(marshaledcredential.param().abi()) }
}
#[inline]
pub unsafe fn CredIsMarshaledCredentialW<P0>(marshaledcredential: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("advapi32.dll" "system" fn CredIsMarshaledCredentialW(marshaledcredential : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { CredIsMarshaledCredentialW(marshaledcredential.param().abi()) }
}
#[inline]
pub unsafe fn CredIsProtectedA<P0>(pszprotectedcredentials: P0, pprotectiontype: *mut CRED_PROTECTION_TYPE) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("advapi32.dll" "system" fn CredIsProtectedA(pszprotectedcredentials : windows_core::PCSTR, pprotectiontype : *mut CRED_PROTECTION_TYPE) -> windows_core::BOOL);
    unsafe { CredIsProtectedA(pszprotectedcredentials.param().abi(), pprotectiontype as _).ok() }
}
#[inline]
pub unsafe fn CredIsProtectedW<P0>(pszprotectedcredentials: P0, pprotectiontype: *mut CRED_PROTECTION_TYPE) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("advapi32.dll" "system" fn CredIsProtectedW(pszprotectedcredentials : windows_core::PCWSTR, pprotectiontype : *mut CRED_PROTECTION_TYPE) -> windows_core::BOOL);
    unsafe { CredIsProtectedW(pszprotectedcredentials.param().abi(), pprotectiontype as _).ok() }
}
#[inline]
pub unsafe fn CredMarshalCredentialA(credtype: CRED_MARSHAL_TYPE, credential: *const core::ffi::c_void, marshaledcredential: *mut windows_core::PSTR) -> windows_core::Result<()> {
    windows_link::link!("advapi32.dll" "system" fn CredMarshalCredentialA(credtype : CRED_MARSHAL_TYPE, credential : *const core::ffi::c_void, marshaledcredential : *mut windows_core::PSTR) -> windows_core::BOOL);
    unsafe { CredMarshalCredentialA(credtype, credential, marshaledcredential as _).ok() }
}
#[inline]
pub unsafe fn CredMarshalCredentialW(credtype: CRED_MARSHAL_TYPE, credential: *const core::ffi::c_void, marshaledcredential: *mut windows_core::PWSTR) -> windows_core::Result<()> {
    windows_link::link!("advapi32.dll" "system" fn CredMarshalCredentialW(credtype : CRED_MARSHAL_TYPE, credential : *const core::ffi::c_void, marshaledcredential : *mut windows_core::PWSTR) -> windows_core::BOOL);
    unsafe { CredMarshalCredentialW(credtype, credential, marshaledcredential as _).ok() }
}
#[inline]
pub unsafe fn CredPackAuthenticationBufferA<P1, P2>(dwflags: CRED_PACK_FLAGS, pszusername: P1, pszpassword: P2, ppackedcredentials: Option<*mut u8>, pcbpackedcredentials: *mut u32) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("credui.dll" "system" fn CredPackAuthenticationBufferA(dwflags : CRED_PACK_FLAGS, pszusername : windows_core::PCSTR, pszpassword : windows_core::PCSTR, ppackedcredentials : *mut u8, pcbpackedcredentials : *mut u32) -> windows_core::BOOL);
    unsafe { CredPackAuthenticationBufferA(dwflags, pszusername.param().abi(), pszpassword.param().abi(), ppackedcredentials.unwrap_or(core::mem::zeroed()) as _, pcbpackedcredentials as _).ok() }
}
#[inline]
pub unsafe fn CredPackAuthenticationBufferW<P1, P2>(dwflags: CRED_PACK_FLAGS, pszusername: P1, pszpassword: P2, ppackedcredentials: Option<*mut u8>, pcbpackedcredentials: *mut u32) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("credui.dll" "system" fn CredPackAuthenticationBufferW(dwflags : CRED_PACK_FLAGS, pszusername : windows_core::PCWSTR, pszpassword : windows_core::PCWSTR, ppackedcredentials : *mut u8, pcbpackedcredentials : *mut u32) -> windows_core::BOOL);
    unsafe { CredPackAuthenticationBufferW(dwflags, pszusername.param().abi(), pszpassword.param().abi(), ppackedcredentials.unwrap_or(core::mem::zeroed()) as _, pcbpackedcredentials as _).ok() }
}
#[inline]
pub unsafe fn CredProtectA(fasself: bool, pszcredentials: &[u8], pszprotectedcredentials: windows_core::PSTR, pcchmaxchars: *mut u32, protectiontype: Option<*mut CRED_PROTECTION_TYPE>) -> windows_core::Result<()> {
    windows_link::link!("advapi32.dll" "system" fn CredProtectA(fasself : windows_core::BOOL, pszcredentials : windows_core::PCSTR, cchcredentials : u32, pszprotectedcredentials : windows_core::PSTR, pcchmaxchars : *mut u32, protectiontype : *mut CRED_PROTECTION_TYPE) -> windows_core::BOOL);
    unsafe { CredProtectA(fasself.into(), core::mem::transmute(pszcredentials.as_ptr()), pszcredentials.len().try_into().unwrap(), core::mem::transmute(pszprotectedcredentials), pcchmaxchars as _, protectiontype.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn CredProtectW(fasself: bool, pszcredentials: &[u16], pszprotectedcredentials: windows_core::PWSTR, pcchmaxchars: *mut u32, protectiontype: Option<*mut CRED_PROTECTION_TYPE>) -> windows_core::Result<()> {
    windows_link::link!("advapi32.dll" "system" fn CredProtectW(fasself : windows_core::BOOL, pszcredentials : windows_core::PCWSTR, cchcredentials : u32, pszprotectedcredentials : windows_core::PWSTR, pcchmaxchars : *mut u32, protectiontype : *mut CRED_PROTECTION_TYPE) -> windows_core::BOOL);
    unsafe { CredProtectW(fasself.into(), core::mem::transmute(pszcredentials.as_ptr()), pszcredentials.len().try_into().unwrap(), core::mem::transmute(pszprotectedcredentials), pcchmaxchars as _, protectiontype.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn CredReadA<P0>(targetname: P0, r#type: CRED_TYPE, flags: Option<u32>, credential: *mut *mut CREDENTIALA) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("advapi32.dll" "system" fn CredReadA(targetname : windows_core::PCSTR, r#type : CRED_TYPE, flags : u32, credential : *mut *mut CREDENTIALA) -> windows_core::BOOL);
    unsafe { CredReadA(targetname.param().abi(), r#type, flags.unwrap_or(core::mem::zeroed()) as _, credential as _).ok() }
}
#[inline]
pub unsafe fn CredReadDomainCredentialsA(targetinfo: *const CREDENTIAL_TARGET_INFORMATIONA, flags: u32, count: *mut u32, credential: *mut *mut *mut CREDENTIALA) -> windows_core::Result<()> {
    windows_link::link!("advapi32.dll" "system" fn CredReadDomainCredentialsA(targetinfo : *const CREDENTIAL_TARGET_INFORMATIONA, flags : u32, count : *mut u32, credential : *mut *mut *mut CREDENTIALA) -> windows_core::BOOL);
    unsafe { CredReadDomainCredentialsA(targetinfo, flags, count as _, credential as _).ok() }
}
#[inline]
pub unsafe fn CredReadDomainCredentialsW(targetinfo: *const CREDENTIAL_TARGET_INFORMATIONW, flags: u32, count: *mut u32, credential: *mut *mut *mut CREDENTIALW) -> windows_core::Result<()> {
    windows_link::link!("advapi32.dll" "system" fn CredReadDomainCredentialsW(targetinfo : *const CREDENTIAL_TARGET_INFORMATIONW, flags : u32, count : *mut u32, credential : *mut *mut *mut CREDENTIALW) -> windows_core::BOOL);
    unsafe { CredReadDomainCredentialsW(targetinfo, flags, count as _, credential as _).ok() }
}
#[inline]
pub unsafe fn CredReadW<P0>(targetname: P0, r#type: CRED_TYPE, flags: Option<u32>, credential: *mut *mut CREDENTIALW) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("advapi32.dll" "system" fn CredReadW(targetname : windows_core::PCWSTR, r#type : CRED_TYPE, flags : u32, credential : *mut *mut CREDENTIALW) -> windows_core::BOOL);
    unsafe { CredReadW(targetname.param().abi(), r#type, flags.unwrap_or(core::mem::zeroed()) as _, credential as _).ok() }
}
#[inline]
pub unsafe fn CredRenameA<P0, P1>(oldtargetname: P0, newtargetname: P1, r#type: CRED_TYPE, flags: Option<u32>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("advapi32.dll" "system" fn CredRenameA(oldtargetname : windows_core::PCSTR, newtargetname : windows_core::PCSTR, r#type : CRED_TYPE, flags : u32) -> windows_core::BOOL);
    unsafe { CredRenameA(oldtargetname.param().abi(), newtargetname.param().abi(), r#type, flags.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn CredRenameW<P0, P1>(oldtargetname: P0, newtargetname: P1, r#type: CRED_TYPE, flags: Option<u32>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("advapi32.dll" "system" fn CredRenameW(oldtargetname : windows_core::PCWSTR, newtargetname : windows_core::PCWSTR, r#type : CRED_TYPE, flags : u32) -> windows_core::BOOL);
    unsafe { CredRenameW(oldtargetname.param().abi(), newtargetname.param().abi(), r#type, flags.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn CredUICmdLinePromptForCredentialsA<P0>(psztargetname: P0, pcontext: Option<*const SecHandle>, dwautherror: u32, username: &mut [u8], pszpassword: &mut [u8], pfsave: Option<*mut windows_core::BOOL>, dwflags: CREDUI_FLAGS) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("credui.dll" "system" fn CredUICmdLinePromptForCredentialsA(psztargetname : windows_core::PCSTR, pcontext : *const SecHandle, dwautherror : u32, username : windows_core::PSTR, uluserbuffersize : u32, pszpassword : windows_core::PSTR, ulpasswordbuffersize : u32, pfsave : *mut windows_core::BOOL, dwflags : CREDUI_FLAGS) -> u32);
    unsafe { CredUICmdLinePromptForCredentialsA(psztargetname.param().abi(), pcontext.unwrap_or(core::mem::zeroed()) as _, dwautherror, core::mem::transmute(username.as_ptr()), username.len().try_into().unwrap(), core::mem::transmute(pszpassword.as_ptr()), pszpassword.len().try_into().unwrap(), pfsave.unwrap_or(core::mem::zeroed()) as _, dwflags) }
}
#[inline]
pub unsafe fn CredUICmdLinePromptForCredentialsW<P0>(psztargetname: P0, pcontext: Option<*const SecHandle>, dwautherror: u32, username: &mut [u16], pszpassword: &mut [u16], pfsave: Option<*mut windows_core::BOOL>, dwflags: CREDUI_FLAGS) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("credui.dll" "system" fn CredUICmdLinePromptForCredentialsW(psztargetname : windows_core::PCWSTR, pcontext : *const SecHandle, dwautherror : u32, username : windows_core::PWSTR, uluserbuffersize : u32, pszpassword : windows_core::PWSTR, ulpasswordbuffersize : u32, pfsave : *mut windows_core::BOOL, dwflags : CREDUI_FLAGS) -> u32);
    unsafe { CredUICmdLinePromptForCredentialsW(psztargetname.param().abi(), pcontext.unwrap_or(core::mem::zeroed()) as _, dwautherror, core::mem::transmute(username.as_ptr()), username.len().try_into().unwrap(), core::mem::transmute(pszpassword.as_ptr()), pszpassword.len().try_into().unwrap(), pfsave.unwrap_or(core::mem::zeroed()) as _, dwflags) }
}
#[inline]
pub unsafe fn CredUIConfirmCredentialsA<P0>(psztargetname: P0, bconfirm: bool) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("credui.dll" "system" fn CredUIConfirmCredentialsA(psztargetname : windows_core::PCSTR, bconfirm : windows_core::BOOL) -> u32);
    unsafe { CredUIConfirmCredentialsA(psztargetname.param().abi(), bconfirm.into()) }
}
#[inline]
pub unsafe fn CredUIConfirmCredentialsW<P0>(psztargetname: P0, bconfirm: bool) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("credui.dll" "system" fn CredUIConfirmCredentialsW(psztargetname : windows_core::PCWSTR, bconfirm : windows_core::BOOL) -> u32);
    unsafe { CredUIConfirmCredentialsW(psztargetname.param().abi(), bconfirm.into()) }
}
#[inline]
pub unsafe fn CredUIParseUserNameA<P0>(username: P0, user: &mut [u8], domain: &mut [u8]) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("credui.dll" "system" fn CredUIParseUserNameA(username : windows_core::PCSTR, user : windows_core::PSTR, userbuffersize : u32, domain : windows_core::PSTR, domainbuffersize : u32) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { CredUIParseUserNameA(username.param().abi(), core::mem::transmute(user.as_ptr()), user.len().try_into().unwrap(), core::mem::transmute(domain.as_ptr()), domain.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn CredUIParseUserNameW<P0>(username: P0, user: &mut [u16], domain: &mut [u16]) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("credui.dll" "system" fn CredUIParseUserNameW(username : windows_core::PCWSTR, user : windows_core::PWSTR, userbuffersize : u32, domain : windows_core::PWSTR, domainbuffersize : u32) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { CredUIParseUserNameW(username.param().abi(), core::mem::transmute(user.as_ptr()), user.len().try_into().unwrap(), core::mem::transmute(domain.as_ptr()), domain.len().try_into().unwrap()) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn CredUIPromptForCredentialsA<P1>(puiinfo: Option<*const CREDUI_INFOA>, psztargetname: P1, pcontext: Option<*const SecHandle>, dwautherror: u32, pszusername: &mut [u8], pszpassword: &mut [u8], save: Option<*mut windows_core::BOOL>, dwflags: CREDUI_FLAGS) -> super::super::Foundation::WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("credui.dll" "system" fn CredUIPromptForCredentialsA(puiinfo : *const CREDUI_INFOA, psztargetname : windows_core::PCSTR, pcontext : *const SecHandle, dwautherror : u32, pszusername : windows_core::PSTR, ulusernamebuffersize : u32, pszpassword : windows_core::PSTR, ulpasswordbuffersize : u32, save : *mut windows_core::BOOL, dwflags : CREDUI_FLAGS) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { CredUIPromptForCredentialsA(puiinfo.unwrap_or(core::mem::zeroed()) as _, psztargetname.param().abi(), pcontext.unwrap_or(core::mem::zeroed()) as _, dwautherror, core::mem::transmute(pszusername.as_ptr()), pszusername.len().try_into().unwrap(), core::mem::transmute(pszpassword.as_ptr()), pszpassword.len().try_into().unwrap(), save.unwrap_or(core::mem::zeroed()) as _, dwflags) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn CredUIPromptForCredentialsW<P1>(puiinfo: Option<*const CREDUI_INFOW>, psztargetname: P1, pcontext: Option<*const SecHandle>, dwautherror: u32, pszusername: &mut [u16], pszpassword: &mut [u16], save: Option<*mut windows_core::BOOL>, dwflags: CREDUI_FLAGS) -> super::super::Foundation::WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("credui.dll" "system" fn CredUIPromptForCredentialsW(puiinfo : *const CREDUI_INFOW, psztargetname : windows_core::PCWSTR, pcontext : *const SecHandle, dwautherror : u32, pszusername : windows_core::PWSTR, ulusernamebuffersize : u32, pszpassword : windows_core::PWSTR, ulpasswordbuffersize : u32, save : *mut windows_core::BOOL, dwflags : CREDUI_FLAGS) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { CredUIPromptForCredentialsW(puiinfo.unwrap_or(core::mem::zeroed()) as _, psztargetname.param().abi(), pcontext.unwrap_or(core::mem::zeroed()) as _, dwautherror, core::mem::transmute(pszusername.as_ptr()), pszusername.len().try_into().unwrap(), core::mem::transmute(pszpassword.as_ptr()), pszpassword.len().try_into().unwrap(), save.unwrap_or(core::mem::zeroed()) as _, dwflags) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn CredUIPromptForWindowsCredentialsA(puiinfo: Option<*const CREDUI_INFOA>, dwautherror: u32, pulauthpackage: *mut u32, pvinauthbuffer: Option<*const core::ffi::c_void>, ulinauthbuffersize: u32, ppvoutauthbuffer: *mut *mut core::ffi::c_void, puloutauthbuffersize: *mut u32, pfsave: Option<*mut windows_core::BOOL>, dwflags: CREDUIWIN_FLAGS) -> u32 {
    windows_link::link!("credui.dll" "system" fn CredUIPromptForWindowsCredentialsA(puiinfo : *const CREDUI_INFOA, dwautherror : u32, pulauthpackage : *mut u32, pvinauthbuffer : *const core::ffi::c_void, ulinauthbuffersize : u32, ppvoutauthbuffer : *mut *mut core::ffi::c_void, puloutauthbuffersize : *mut u32, pfsave : *mut windows_core::BOOL, dwflags : CREDUIWIN_FLAGS) -> u32);
    unsafe { CredUIPromptForWindowsCredentialsA(puiinfo.unwrap_or(core::mem::zeroed()) as _, dwautherror, pulauthpackage as _, pvinauthbuffer.unwrap_or(core::mem::zeroed()) as _, ulinauthbuffersize, ppvoutauthbuffer as _, puloutauthbuffersize as _, pfsave.unwrap_or(core::mem::zeroed()) as _, dwflags) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn CredUIPromptForWindowsCredentialsW(puiinfo: Option<*const CREDUI_INFOW>, dwautherror: u32, pulauthpackage: *mut u32, pvinauthbuffer: Option<*const core::ffi::c_void>, ulinauthbuffersize: u32, ppvoutauthbuffer: *mut *mut core::ffi::c_void, puloutauthbuffersize: *mut u32, pfsave: Option<*mut windows_core::BOOL>, dwflags: CREDUIWIN_FLAGS) -> u32 {
    windows_link::link!("credui.dll" "system" fn CredUIPromptForWindowsCredentialsW(puiinfo : *const CREDUI_INFOW, dwautherror : u32, pulauthpackage : *mut u32, pvinauthbuffer : *const core::ffi::c_void, ulinauthbuffersize : u32, ppvoutauthbuffer : *mut *mut core::ffi::c_void, puloutauthbuffersize : *mut u32, pfsave : *mut windows_core::BOOL, dwflags : CREDUIWIN_FLAGS) -> u32);
    unsafe { CredUIPromptForWindowsCredentialsW(puiinfo.unwrap_or(core::mem::zeroed()) as _, dwautherror, pulauthpackage as _, pvinauthbuffer.unwrap_or(core::mem::zeroed()) as _, ulinauthbuffersize, ppvoutauthbuffer as _, puloutauthbuffersize as _, pfsave.unwrap_or(core::mem::zeroed()) as _, dwflags) }
}
#[inline]
pub unsafe fn CredUIReadSSOCredW<P0>(pszrealm: P0, ppszusername: *mut windows_core::PWSTR) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("credui.dll" "system" fn CredUIReadSSOCredW(pszrealm : windows_core::PCWSTR, ppszusername : *mut windows_core::PWSTR) -> u32);
    unsafe { CredUIReadSSOCredW(pszrealm.param().abi(), ppszusername as _) }
}
#[inline]
pub unsafe fn CredUIStoreSSOCredW<P0, P1, P2>(pszrealm: P0, pszusername: P1, pszpassword: P2, bpersist: bool) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("credui.dll" "system" fn CredUIStoreSSOCredW(pszrealm : windows_core::PCWSTR, pszusername : windows_core::PCWSTR, pszpassword : windows_core::PCWSTR, bpersist : windows_core::BOOL) -> u32);
    unsafe { CredUIStoreSSOCredW(pszrealm.param().abi(), pszusername.param().abi(), pszpassword.param().abi(), bpersist.into()) }
}
#[inline]
pub unsafe fn CredUnPackAuthenticationBufferA(dwflags: CRED_PACK_FLAGS, pauthbuffer: *const core::ffi::c_void, cbauthbuffer: u32, pszusername: Option<windows_core::PSTR>, pcchlmaxusername: *mut u32, pszdomainname: Option<windows_core::PSTR>, pcchmaxdomainname: Option<*mut u32>, pszpassword: Option<windows_core::PSTR>, pcchmaxpassword: *mut u32) -> windows_core::Result<()> {
    windows_link::link!("credui.dll" "system" fn CredUnPackAuthenticationBufferA(dwflags : CRED_PACK_FLAGS, pauthbuffer : *const core::ffi::c_void, cbauthbuffer : u32, pszusername : windows_core::PSTR, pcchlmaxusername : *mut u32, pszdomainname : windows_core::PSTR, pcchmaxdomainname : *mut u32, pszpassword : windows_core::PSTR, pcchmaxpassword : *mut u32) -> windows_core::BOOL);
    unsafe { CredUnPackAuthenticationBufferA(dwflags, pauthbuffer, cbauthbuffer, pszusername.unwrap_or(core::mem::zeroed()) as _, pcchlmaxusername as _, pszdomainname.unwrap_or(core::mem::zeroed()) as _, pcchmaxdomainname.unwrap_or(core::mem::zeroed()) as _, pszpassword.unwrap_or(core::mem::zeroed()) as _, pcchmaxpassword as _).ok() }
}
#[inline]
pub unsafe fn CredUnPackAuthenticationBufferW(dwflags: CRED_PACK_FLAGS, pauthbuffer: *const core::ffi::c_void, cbauthbuffer: u32, pszusername: Option<windows_core::PWSTR>, pcchmaxusername: *mut u32, pszdomainname: Option<windows_core::PWSTR>, pcchmaxdomainname: Option<*mut u32>, pszpassword: Option<windows_core::PWSTR>, pcchmaxpassword: *mut u32) -> windows_core::Result<()> {
    windows_link::link!("credui.dll" "system" fn CredUnPackAuthenticationBufferW(dwflags : CRED_PACK_FLAGS, pauthbuffer : *const core::ffi::c_void, cbauthbuffer : u32, pszusername : windows_core::PWSTR, pcchmaxusername : *mut u32, pszdomainname : windows_core::PWSTR, pcchmaxdomainname : *mut u32, pszpassword : windows_core::PWSTR, pcchmaxpassword : *mut u32) -> windows_core::BOOL);
    unsafe { CredUnPackAuthenticationBufferW(dwflags, pauthbuffer, cbauthbuffer, pszusername.unwrap_or(core::mem::zeroed()) as _, pcchmaxusername as _, pszdomainname.unwrap_or(core::mem::zeroed()) as _, pcchmaxdomainname.unwrap_or(core::mem::zeroed()) as _, pszpassword.unwrap_or(core::mem::zeroed()) as _, pcchmaxpassword as _).ok() }
}
#[inline]
pub unsafe fn CredUnmarshalCredentialA<P0>(marshaledcredential: P0, credtype: *mut CRED_MARSHAL_TYPE, credential: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("advapi32.dll" "system" fn CredUnmarshalCredentialA(marshaledcredential : windows_core::PCSTR, credtype : *mut CRED_MARSHAL_TYPE, credential : *mut *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { CredUnmarshalCredentialA(marshaledcredential.param().abi(), credtype as _, credential as _).ok() }
}
#[inline]
pub unsafe fn CredUnmarshalCredentialW<P0>(marshaledcredential: P0, credtype: *mut CRED_MARSHAL_TYPE, credential: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("advapi32.dll" "system" fn CredUnmarshalCredentialW(marshaledcredential : windows_core::PCWSTR, credtype : *mut CRED_MARSHAL_TYPE, credential : *mut *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { CredUnmarshalCredentialW(marshaledcredential.param().abi(), credtype as _, credential as _).ok() }
}
#[inline]
pub unsafe fn CredUnprotectA(fasself: bool, pszprotectedcredentials: &[u8], pszcredentials: Option<windows_core::PSTR>, pcchmaxchars: *mut u32) -> windows_core::Result<()> {
    windows_link::link!("advapi32.dll" "system" fn CredUnprotectA(fasself : windows_core::BOOL, pszprotectedcredentials : windows_core::PCSTR, cchprotectedcredentials : u32, pszcredentials : windows_core::PSTR, pcchmaxchars : *mut u32) -> windows_core::BOOL);
    unsafe { CredUnprotectA(fasself.into(), core::mem::transmute(pszprotectedcredentials.as_ptr()), pszprotectedcredentials.len().try_into().unwrap(), pszcredentials.unwrap_or(core::mem::zeroed()) as _, pcchmaxchars as _).ok() }
}
#[inline]
pub unsafe fn CredUnprotectW(fasself: bool, pszprotectedcredentials: &[u16], pszcredentials: Option<windows_core::PWSTR>, pcchmaxchars: *mut u32) -> windows_core::Result<()> {
    windows_link::link!("advapi32.dll" "system" fn CredUnprotectW(fasself : windows_core::BOOL, pszprotectedcredentials : windows_core::PCWSTR, cchprotectedcredentials : u32, pszcredentials : windows_core::PWSTR, pcchmaxchars : *mut u32) -> windows_core::BOOL);
    unsafe { CredUnprotectW(fasself.into(), core::mem::transmute(pszprotectedcredentials.as_ptr()), pszprotectedcredentials.len().try_into().unwrap(), pszcredentials.unwrap_or(core::mem::zeroed()) as _, pcchmaxchars as _).ok() }
}
#[inline]
pub unsafe fn CredWriteA(credential: *const CREDENTIALA, flags: u32) -> windows_core::Result<()> {
    windows_link::link!("advapi32.dll" "system" fn CredWriteA(credential : *const CREDENTIALA, flags : u32) -> windows_core::BOOL);
    unsafe { CredWriteA(credential, flags).ok() }
}
#[inline]
pub unsafe fn CredWriteDomainCredentialsA(targetinfo: *const CREDENTIAL_TARGET_INFORMATIONA, credential: *const CREDENTIALA, flags: u32) -> windows_core::Result<()> {
    windows_link::link!("advapi32.dll" "system" fn CredWriteDomainCredentialsA(targetinfo : *const CREDENTIAL_TARGET_INFORMATIONA, credential : *const CREDENTIALA, flags : u32) -> windows_core::BOOL);
    unsafe { CredWriteDomainCredentialsA(targetinfo, credential, flags).ok() }
}
#[inline]
pub unsafe fn CredWriteDomainCredentialsW(targetinfo: *const CREDENTIAL_TARGET_INFORMATIONW, credential: *const CREDENTIALW, flags: u32) -> windows_core::Result<()> {
    windows_link::link!("advapi32.dll" "system" fn CredWriteDomainCredentialsW(targetinfo : *const CREDENTIAL_TARGET_INFORMATIONW, credential : *const CREDENTIALW, flags : u32) -> windows_core::BOOL);
    unsafe { CredWriteDomainCredentialsW(targetinfo, credential, flags).ok() }
}
#[inline]
pub unsafe fn CredWriteW(credential: *const CREDENTIALW, flags: u32) -> windows_core::Result<()> {
    windows_link::link!("advapi32.dll" "system" fn CredWriteW(credential : *const CREDENTIALW, flags : u32) -> windows_core::BOOL);
    unsafe { CredWriteW(credential, flags).ok() }
}
#[inline]
pub unsafe fn GetOpenCardNameA(param0: *mut OPENCARDNAMEA) -> i32 {
    windows_link::link!("scarddlg.dll" "system" fn GetOpenCardNameA(param0 : *mut OPENCARDNAMEA) -> i32);
    unsafe { GetOpenCardNameA(param0 as _) }
}
#[inline]
pub unsafe fn GetOpenCardNameW(param0: *mut OPENCARDNAMEW) -> i32 {
    windows_link::link!("scarddlg.dll" "system" fn GetOpenCardNameW(param0 : *mut OPENCARDNAMEW) -> i32);
    unsafe { GetOpenCardNameW(param0 as _) }
}
#[inline]
pub unsafe fn KeyCredentialManagerFreeInformation(keycredentialmanagerinfo: *const KeyCredentialManagerInfo) {
    windows_link::link!("keycredmgr.dll" "system" fn KeyCredentialManagerFreeInformation(keycredentialmanagerinfo : *const KeyCredentialManagerInfo));
    unsafe { KeyCredentialManagerFreeInformation(keycredentialmanagerinfo) }
}
#[inline]
pub unsafe fn KeyCredentialManagerGetInformation() -> windows_core::Result<*mut KeyCredentialManagerInfo> {
    windows_link::link!("keycredmgr.dll" "system" fn KeyCredentialManagerGetInformation(keycredentialmanagerinfo : *mut *mut KeyCredentialManagerInfo) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        KeyCredentialManagerGetInformation(&mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn KeyCredentialManagerGetOperationErrorStates(keycredentialmanageroperationtype: KeyCredentialManagerOperationType, isready: *mut windows_core::BOOL, keycredentialmanageroperationerrorstates: *mut KeyCredentialManagerOperationErrorStates) -> windows_core::Result<()> {
    windows_link::link!("keycredmgr.dll" "system" fn KeyCredentialManagerGetOperationErrorStates(keycredentialmanageroperationtype : KeyCredentialManagerOperationType, isready : *mut windows_core::BOOL, keycredentialmanageroperationerrorstates : *mut KeyCredentialManagerOperationErrorStates) -> windows_core::HRESULT);
    unsafe { KeyCredentialManagerGetOperationErrorStates(keycredentialmanageroperationtype, isready as _, keycredentialmanageroperationerrorstates as _).ok() }
}
#[inline]
pub unsafe fn KeyCredentialManagerShowUIOperation(hwndowner: super::super::Foundation::HWND, keycredentialmanageroperationtype: KeyCredentialManagerOperationType) -> windows_core::Result<()> {
    windows_link::link!("keycredmgr.dll" "system" fn KeyCredentialManagerShowUIOperation(hwndowner : super::super::Foundation:: HWND, keycredentialmanageroperationtype : KeyCredentialManagerOperationType) -> windows_core::HRESULT);
    unsafe { KeyCredentialManagerShowUIOperation(hwndowner, keycredentialmanageroperationtype).ok() }
}
#[inline]
pub unsafe fn SCardAccessStartedEvent() -> windows_core::Result<super::super::Foundation::HANDLE> {
    windows_link::link!("winscard.dll" "system" fn SCardAccessStartedEvent() -> super::super::Foundation:: HANDLE);
    let result__ = unsafe { SCardAccessStartedEvent() };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn SCardAddReaderToGroupA<P1, P2>(hcontext: usize, szreadername: P1, szgroupname: P2) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("winscard.dll" "system" fn SCardAddReaderToGroupA(hcontext : usize, szreadername : windows_core::PCSTR, szgroupname : windows_core::PCSTR) -> i32);
    unsafe { SCardAddReaderToGroupA(hcontext, szreadername.param().abi(), szgroupname.param().abi()) }
}
#[inline]
pub unsafe fn SCardAddReaderToGroupW<P1, P2>(hcontext: usize, szreadername: P1, szgroupname: P2) -> i32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("winscard.dll" "system" fn SCardAddReaderToGroupW(hcontext : usize, szreadername : windows_core::PCWSTR, szgroupname : windows_core::PCWSTR) -> i32);
    unsafe { SCardAddReaderToGroupW(hcontext, szreadername.param().abi(), szgroupname.param().abi()) }
}
#[inline]
pub unsafe fn SCardAudit(hcontext: usize, dwevent: u32) -> i32 {
    windows_link::link!("winscard.dll" "system" fn SCardAudit(hcontext : usize, dwevent : u32) -> i32);
    unsafe { SCardAudit(hcontext, dwevent) }
}
#[inline]
pub unsafe fn SCardBeginTransaction(hcard: usize) -> i32 {
    windows_link::link!("winscard.dll" "system" fn SCardBeginTransaction(hcard : usize) -> i32);
    unsafe { SCardBeginTransaction(hcard) }
}
#[inline]
pub unsafe fn SCardCancel(hcontext: usize) -> i32 {
    windows_link::link!("winscard.dll" "system" fn SCardCancel(hcontext : usize) -> i32);
    unsafe { SCardCancel(hcontext) }
}
#[inline]
pub unsafe fn SCardConnectA<P1>(hcontext: usize, szreader: P1, dwsharemode: u32, dwpreferredprotocols: u32, phcard: *mut usize, pdwactiveprotocol: *mut u32) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("winscard.dll" "system" fn SCardConnectA(hcontext : usize, szreader : windows_core::PCSTR, dwsharemode : u32, dwpreferredprotocols : u32, phcard : *mut usize, pdwactiveprotocol : *mut u32) -> i32);
    unsafe { SCardConnectA(hcontext, szreader.param().abi(), dwsharemode, dwpreferredprotocols, phcard as _, pdwactiveprotocol as _) }
}
#[inline]
pub unsafe fn SCardConnectW<P1>(hcontext: usize, szreader: P1, dwsharemode: u32, dwpreferredprotocols: u32, phcard: *mut usize, pdwactiveprotocol: *mut u32) -> i32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("winscard.dll" "system" fn SCardConnectW(hcontext : usize, szreader : windows_core::PCWSTR, dwsharemode : u32, dwpreferredprotocols : u32, phcard : *mut usize, pdwactiveprotocol : *mut u32) -> i32);
    unsafe { SCardConnectW(hcontext, szreader.param().abi(), dwsharemode, dwpreferredprotocols, phcard as _, pdwactiveprotocol as _) }
}
#[inline]
pub unsafe fn SCardControl(hcard: usize, dwcontrolcode: u32, lpinbuffer: *const core::ffi::c_void, cbinbuffersize: u32, lpoutbuffer: *mut core::ffi::c_void, cboutbuffersize: u32, lpbytesreturned: *mut u32) -> i32 {
    windows_link::link!("winscard.dll" "system" fn SCardControl(hcard : usize, dwcontrolcode : u32, lpinbuffer : *const core::ffi::c_void, cbinbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, cboutbuffersize : u32, lpbytesreturned : *mut u32) -> i32);
    unsafe { SCardControl(hcard, dwcontrolcode, lpinbuffer, cbinbuffersize, lpoutbuffer as _, cboutbuffersize, lpbytesreturned as _) }
}
#[inline]
pub unsafe fn SCardDisconnect(hcard: usize, dwdisposition: u32) -> i32 {
    windows_link::link!("winscard.dll" "system" fn SCardDisconnect(hcard : usize, dwdisposition : u32) -> i32);
    unsafe { SCardDisconnect(hcard, dwdisposition) }
}
#[inline]
pub unsafe fn SCardDlgExtendedError() -> i32 {
    windows_link::link!("scarddlg.dll" "system" fn SCardDlgExtendedError() -> i32);
    unsafe { SCardDlgExtendedError() }
}
#[inline]
pub unsafe fn SCardEndTransaction(hcard: usize, dwdisposition: u32) -> i32 {
    windows_link::link!("winscard.dll" "system" fn SCardEndTransaction(hcard : usize, dwdisposition : u32) -> i32);
    unsafe { SCardEndTransaction(hcard, dwdisposition) }
}
#[inline]
pub unsafe fn SCardEstablishContext(dwscope: SCARD_SCOPE, pvreserved1: Option<*const core::ffi::c_void>, pvreserved2: Option<*const core::ffi::c_void>, phcontext: *mut usize) -> i32 {
    windows_link::link!("winscard.dll" "system" fn SCardEstablishContext(dwscope : SCARD_SCOPE, pvreserved1 : *const core::ffi::c_void, pvreserved2 : *const core::ffi::c_void, phcontext : *mut usize) -> i32);
    unsafe { SCardEstablishContext(dwscope, pvreserved1.unwrap_or(core::mem::zeroed()) as _, pvreserved2.unwrap_or(core::mem::zeroed()) as _, phcontext as _) }
}
#[inline]
pub unsafe fn SCardForgetCardTypeA<P1>(hcontext: usize, szcardname: P1) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("winscard.dll" "system" fn SCardForgetCardTypeA(hcontext : usize, szcardname : windows_core::PCSTR) -> i32);
    unsafe { SCardForgetCardTypeA(hcontext, szcardname.param().abi()) }
}
#[inline]
pub unsafe fn SCardForgetCardTypeW<P1>(hcontext: usize, szcardname: P1) -> i32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("winscard.dll" "system" fn SCardForgetCardTypeW(hcontext : usize, szcardname : windows_core::PCWSTR) -> i32);
    unsafe { SCardForgetCardTypeW(hcontext, szcardname.param().abi()) }
}
#[inline]
pub unsafe fn SCardForgetReaderA<P1>(hcontext: usize, szreadername: P1) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("winscard.dll" "system" fn SCardForgetReaderA(hcontext : usize, szreadername : windows_core::PCSTR) -> i32);
    unsafe { SCardForgetReaderA(hcontext, szreadername.param().abi()) }
}
#[inline]
pub unsafe fn SCardForgetReaderGroupA<P1>(hcontext: usize, szgroupname: P1) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("winscard.dll" "system" fn SCardForgetReaderGroupA(hcontext : usize, szgroupname : windows_core::PCSTR) -> i32);
    unsafe { SCardForgetReaderGroupA(hcontext, szgroupname.param().abi()) }
}
#[inline]
pub unsafe fn SCardForgetReaderGroupW<P1>(hcontext: usize, szgroupname: P1) -> i32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("winscard.dll" "system" fn SCardForgetReaderGroupW(hcontext : usize, szgroupname : windows_core::PCWSTR) -> i32);
    unsafe { SCardForgetReaderGroupW(hcontext, szgroupname.param().abi()) }
}
#[inline]
pub unsafe fn SCardForgetReaderW<P1>(hcontext: usize, szreadername: P1) -> i32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("winscard.dll" "system" fn SCardForgetReaderW(hcontext : usize, szreadername : windows_core::PCWSTR) -> i32);
    unsafe { SCardForgetReaderW(hcontext, szreadername.param().abi()) }
}
#[inline]
pub unsafe fn SCardFreeMemory(hcontext: usize, pvmem: *const core::ffi::c_void) -> i32 {
    windows_link::link!("winscard.dll" "system" fn SCardFreeMemory(hcontext : usize, pvmem : *const core::ffi::c_void) -> i32);
    unsafe { SCardFreeMemory(hcontext, pvmem) }
}
#[inline]
pub unsafe fn SCardGetAttrib(hcard: usize, dwattrid: u32, pbattr: Option<*mut u8>, pcbattrlen: *mut u32) -> i32 {
    windows_link::link!("winscard.dll" "system" fn SCardGetAttrib(hcard : usize, dwattrid : u32, pbattr : *mut u8, pcbattrlen : *mut u32) -> i32);
    unsafe { SCardGetAttrib(hcard, dwattrid, pbattr.unwrap_or(core::mem::zeroed()) as _, pcbattrlen as _) }
}
#[inline]
pub unsafe fn SCardGetCardTypeProviderNameA<P1>(hcontext: usize, szcardname: P1, dwproviderid: u32, szprovider: windows_core::PSTR, pcchprovider: *mut u32) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("winscard.dll" "system" fn SCardGetCardTypeProviderNameA(hcontext : usize, szcardname : windows_core::PCSTR, dwproviderid : u32, szprovider : windows_core::PSTR, pcchprovider : *mut u32) -> i32);
    unsafe { SCardGetCardTypeProviderNameA(hcontext, szcardname.param().abi(), dwproviderid, core::mem::transmute(szprovider), pcchprovider as _) }
}
#[inline]
pub unsafe fn SCardGetCardTypeProviderNameW<P1>(hcontext: usize, szcardname: P1, dwproviderid: u32, szprovider: windows_core::PWSTR, pcchprovider: *mut u32) -> i32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("winscard.dll" "system" fn SCardGetCardTypeProviderNameW(hcontext : usize, szcardname : windows_core::PCWSTR, dwproviderid : u32, szprovider : windows_core::PWSTR, pcchprovider : *mut u32) -> i32);
    unsafe { SCardGetCardTypeProviderNameW(hcontext, szcardname.param().abi(), dwproviderid, core::mem::transmute(szprovider), pcchprovider as _) }
}
#[inline]
pub unsafe fn SCardGetDeviceTypeIdA<P1>(hcontext: usize, szreadername: P1, pdwdevicetypeid: *mut u32) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("winscard.dll" "system" fn SCardGetDeviceTypeIdA(hcontext : usize, szreadername : windows_core::PCSTR, pdwdevicetypeid : *mut u32) -> i32);
    unsafe { SCardGetDeviceTypeIdA(hcontext, szreadername.param().abi(), pdwdevicetypeid as _) }
}
#[inline]
pub unsafe fn SCardGetDeviceTypeIdW<P1>(hcontext: usize, szreadername: P1, pdwdevicetypeid: *mut u32) -> i32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("winscard.dll" "system" fn SCardGetDeviceTypeIdW(hcontext : usize, szreadername : windows_core::PCWSTR, pdwdevicetypeid : *mut u32) -> i32);
    unsafe { SCardGetDeviceTypeIdW(hcontext, szreadername.param().abi(), pdwdevicetypeid as _) }
}
#[inline]
pub unsafe fn SCardGetProviderIdA<P1>(hcontext: usize, szcard: P1, pguidproviderid: *mut windows_core::GUID) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("winscard.dll" "system" fn SCardGetProviderIdA(hcontext : usize, szcard : windows_core::PCSTR, pguidproviderid : *mut windows_core::GUID) -> i32);
    unsafe { SCardGetProviderIdA(hcontext, szcard.param().abi(), pguidproviderid as _) }
}
#[inline]
pub unsafe fn SCardGetProviderIdW<P1>(hcontext: usize, szcard: P1, pguidproviderid: *mut windows_core::GUID) -> i32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("winscard.dll" "system" fn SCardGetProviderIdW(hcontext : usize, szcard : windows_core::PCWSTR, pguidproviderid : *mut windows_core::GUID) -> i32);
    unsafe { SCardGetProviderIdW(hcontext, szcard.param().abi(), pguidproviderid as _) }
}
#[inline]
pub unsafe fn SCardGetReaderDeviceInstanceIdA<P1>(hcontext: usize, szreadername: P1, szdeviceinstanceid: Option<windows_core::PSTR>, pcchdeviceinstanceid: *mut u32) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("winscard.dll" "system" fn SCardGetReaderDeviceInstanceIdA(hcontext : usize, szreadername : windows_core::PCSTR, szdeviceinstanceid : windows_core::PSTR, pcchdeviceinstanceid : *mut u32) -> i32);
    unsafe { SCardGetReaderDeviceInstanceIdA(hcontext, szreadername.param().abi(), szdeviceinstanceid.unwrap_or(core::mem::zeroed()) as _, pcchdeviceinstanceid as _) }
}
#[inline]
pub unsafe fn SCardGetReaderDeviceInstanceIdW<P1>(hcontext: usize, szreadername: P1, szdeviceinstanceid: Option<windows_core::PWSTR>, pcchdeviceinstanceid: *mut u32) -> i32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("winscard.dll" "system" fn SCardGetReaderDeviceInstanceIdW(hcontext : usize, szreadername : windows_core::PCWSTR, szdeviceinstanceid : windows_core::PWSTR, pcchdeviceinstanceid : *mut u32) -> i32);
    unsafe { SCardGetReaderDeviceInstanceIdW(hcontext, szreadername.param().abi(), szdeviceinstanceid.unwrap_or(core::mem::zeroed()) as _, pcchdeviceinstanceid as _) }
}
#[inline]
pub unsafe fn SCardGetReaderIconA<P1>(hcontext: usize, szreadername: P1, pbicon: *mut u8, pcbicon: *mut u32) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("winscard.dll" "system" fn SCardGetReaderIconA(hcontext : usize, szreadername : windows_core::PCSTR, pbicon : *mut u8, pcbicon : *mut u32) -> i32);
    unsafe { SCardGetReaderIconA(hcontext, szreadername.param().abi(), pbicon as _, pcbicon as _) }
}
#[inline]
pub unsafe fn SCardGetReaderIconW<P1>(hcontext: usize, szreadername: P1, pbicon: *mut u8, pcbicon: *mut u32) -> i32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("winscard.dll" "system" fn SCardGetReaderIconW(hcontext : usize, szreadername : windows_core::PCWSTR, pbicon : *mut u8, pcbicon : *mut u32) -> i32);
    unsafe { SCardGetReaderIconW(hcontext, szreadername.param().abi(), pbicon as _, pcbicon as _) }
}
#[inline]
pub unsafe fn SCardGetStatusChangeA(hcontext: usize, dwtimeout: u32, rgreaderstates: *mut SCARD_READERSTATEA, creaders: u32) -> i32 {
    windows_link::link!("winscard.dll" "system" fn SCardGetStatusChangeA(hcontext : usize, dwtimeout : u32, rgreaderstates : *mut SCARD_READERSTATEA, creaders : u32) -> i32);
    unsafe { SCardGetStatusChangeA(hcontext, dwtimeout, rgreaderstates as _, creaders) }
}
#[inline]
pub unsafe fn SCardGetStatusChangeW(hcontext: usize, dwtimeout: u32, rgreaderstates: *mut SCARD_READERSTATEW, creaders: u32) -> i32 {
    windows_link::link!("winscard.dll" "system" fn SCardGetStatusChangeW(hcontext : usize, dwtimeout : u32, rgreaderstates : *mut SCARD_READERSTATEW, creaders : u32) -> i32);
    unsafe { SCardGetStatusChangeW(hcontext, dwtimeout, rgreaderstates as _, creaders) }
}
#[inline]
pub unsafe fn SCardGetTransmitCount(hcard: usize, pctransmitcount: *mut u32) -> i32 {
    windows_link::link!("winscard.dll" "system" fn SCardGetTransmitCount(hcard : usize, pctransmitcount : *mut u32) -> i32);
    unsafe { SCardGetTransmitCount(hcard, pctransmitcount as _) }
}
#[inline]
pub unsafe fn SCardIntroduceCardTypeA<P1>(hcontext: usize, szcardname: P1, pguidprimaryprovider: Option<*const windows_core::GUID>, rgguidinterfaces: Option<*const windows_core::GUID>, dwinterfacecount: u32, pbatr: *const u8, pbatrmask: *const u8, cbatrlen: u32) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("winscard.dll" "system" fn SCardIntroduceCardTypeA(hcontext : usize, szcardname : windows_core::PCSTR, pguidprimaryprovider : *const windows_core::GUID, rgguidinterfaces : *const windows_core::GUID, dwinterfacecount : u32, pbatr : *const u8, pbatrmask : *const u8, cbatrlen : u32) -> i32);
    unsafe { SCardIntroduceCardTypeA(hcontext, szcardname.param().abi(), pguidprimaryprovider.unwrap_or(core::mem::zeroed()) as _, rgguidinterfaces.unwrap_or(core::mem::zeroed()) as _, dwinterfacecount, pbatr, pbatrmask, cbatrlen) }
}
#[inline]
pub unsafe fn SCardIntroduceCardTypeW<P1>(hcontext: usize, szcardname: P1, pguidprimaryprovider: Option<*const windows_core::GUID>, rgguidinterfaces: Option<*const windows_core::GUID>, dwinterfacecount: u32, pbatr: *const u8, pbatrmask: *const u8, cbatrlen: u32) -> i32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("winscard.dll" "system" fn SCardIntroduceCardTypeW(hcontext : usize, szcardname : windows_core::PCWSTR, pguidprimaryprovider : *const windows_core::GUID, rgguidinterfaces : *const windows_core::GUID, dwinterfacecount : u32, pbatr : *const u8, pbatrmask : *const u8, cbatrlen : u32) -> i32);
    unsafe { SCardIntroduceCardTypeW(hcontext, szcardname.param().abi(), pguidprimaryprovider.unwrap_or(core::mem::zeroed()) as _, rgguidinterfaces.unwrap_or(core::mem::zeroed()) as _, dwinterfacecount, pbatr, pbatrmask, cbatrlen) }
}
#[inline]
pub unsafe fn SCardIntroduceReaderA<P1, P2>(hcontext: usize, szreadername: P1, szdevicename: P2) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("winscard.dll" "system" fn SCardIntroduceReaderA(hcontext : usize, szreadername : windows_core::PCSTR, szdevicename : windows_core::PCSTR) -> i32);
    unsafe { SCardIntroduceReaderA(hcontext, szreadername.param().abi(), szdevicename.param().abi()) }
}
#[inline]
pub unsafe fn SCardIntroduceReaderGroupA<P1>(hcontext: usize, szgroupname: P1) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("winscard.dll" "system" fn SCardIntroduceReaderGroupA(hcontext : usize, szgroupname : windows_core::PCSTR) -> i32);
    unsafe { SCardIntroduceReaderGroupA(hcontext, szgroupname.param().abi()) }
}
#[inline]
pub unsafe fn SCardIntroduceReaderGroupW<P1>(hcontext: usize, szgroupname: P1) -> i32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("winscard.dll" "system" fn SCardIntroduceReaderGroupW(hcontext : usize, szgroupname : windows_core::PCWSTR) -> i32);
    unsafe { SCardIntroduceReaderGroupW(hcontext, szgroupname.param().abi()) }
}
#[inline]
pub unsafe fn SCardIntroduceReaderW<P1, P2>(hcontext: usize, szreadername: P1, szdevicename: P2) -> i32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("winscard.dll" "system" fn SCardIntroduceReaderW(hcontext : usize, szreadername : windows_core::PCWSTR, szdevicename : windows_core::PCWSTR) -> i32);
    unsafe { SCardIntroduceReaderW(hcontext, szreadername.param().abi(), szdevicename.param().abi()) }
}
#[inline]
pub unsafe fn SCardIsValidContext(hcontext: usize) -> i32 {
    windows_link::link!("winscard.dll" "system" fn SCardIsValidContext(hcontext : usize) -> i32);
    unsafe { SCardIsValidContext(hcontext) }
}
#[inline]
pub unsafe fn SCardListCardsA(hcontext: usize, pbatr: Option<*const u8>, rgquidinterfaces: Option<&[windows_core::GUID]>, mszcards: Option<windows_core::PSTR>, pcchcards: *mut u32) -> i32 {
    windows_link::link!("winscard.dll" "system" fn SCardListCardsA(hcontext : usize, pbatr : *const u8, rgquidinterfaces : *const windows_core::GUID, cguidinterfacecount : u32, mszcards : windows_core::PSTR, pcchcards : *mut u32) -> i32);
    unsafe { SCardListCardsA(hcontext, pbatr.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(rgquidinterfaces.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), rgquidinterfaces.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), mszcards.unwrap_or(core::mem::zeroed()) as _, pcchcards as _) }
}
#[inline]
pub unsafe fn SCardListCardsW(hcontext: usize, pbatr: Option<*const u8>, rgquidinterfaces: Option<&[windows_core::GUID]>, mszcards: Option<windows_core::PWSTR>, pcchcards: *mut u32) -> i32 {
    windows_link::link!("winscard.dll" "system" fn SCardListCardsW(hcontext : usize, pbatr : *const u8, rgquidinterfaces : *const windows_core::GUID, cguidinterfacecount : u32, mszcards : windows_core::PWSTR, pcchcards : *mut u32) -> i32);
    unsafe { SCardListCardsW(hcontext, pbatr.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(rgquidinterfaces.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), rgquidinterfaces.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), mszcards.unwrap_or(core::mem::zeroed()) as _, pcchcards as _) }
}
#[inline]
pub unsafe fn SCardListInterfacesA<P1>(hcontext: usize, szcard: P1, pguidinterfaces: *mut windows_core::GUID, pcguidinterfaces: *mut u32) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("winscard.dll" "system" fn SCardListInterfacesA(hcontext : usize, szcard : windows_core::PCSTR, pguidinterfaces : *mut windows_core::GUID, pcguidinterfaces : *mut u32) -> i32);
    unsafe { SCardListInterfacesA(hcontext, szcard.param().abi(), pguidinterfaces as _, pcguidinterfaces as _) }
}
#[inline]
pub unsafe fn SCardListInterfacesW<P1>(hcontext: usize, szcard: P1, pguidinterfaces: *mut windows_core::GUID, pcguidinterfaces: *mut u32) -> i32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("winscard.dll" "system" fn SCardListInterfacesW(hcontext : usize, szcard : windows_core::PCWSTR, pguidinterfaces : *mut windows_core::GUID, pcguidinterfaces : *mut u32) -> i32);
    unsafe { SCardListInterfacesW(hcontext, szcard.param().abi(), pguidinterfaces as _, pcguidinterfaces as _) }
}
#[inline]
pub unsafe fn SCardListReaderGroupsA(hcontext: usize, mszgroups: Option<windows_core::PSTR>, pcchgroups: *mut u32) -> i32 {
    windows_link::link!("winscard.dll" "system" fn SCardListReaderGroupsA(hcontext : usize, mszgroups : windows_core::PSTR, pcchgroups : *mut u32) -> i32);
    unsafe { SCardListReaderGroupsA(hcontext, mszgroups.unwrap_or(core::mem::zeroed()) as _, pcchgroups as _) }
}
#[inline]
pub unsafe fn SCardListReaderGroupsW(hcontext: usize, mszgroups: Option<windows_core::PWSTR>, pcchgroups: *mut u32) -> i32 {
    windows_link::link!("winscard.dll" "system" fn SCardListReaderGroupsW(hcontext : usize, mszgroups : windows_core::PWSTR, pcchgroups : *mut u32) -> i32);
    unsafe { SCardListReaderGroupsW(hcontext, mszgroups.unwrap_or(core::mem::zeroed()) as _, pcchgroups as _) }
}
#[inline]
pub unsafe fn SCardListReadersA<P1>(hcontext: usize, mszgroups: P1, mszreaders: Option<windows_core::PSTR>, pcchreaders: *mut u32) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("winscard.dll" "system" fn SCardListReadersA(hcontext : usize, mszgroups : windows_core::PCSTR, mszreaders : windows_core::PSTR, pcchreaders : *mut u32) -> i32);
    unsafe { SCardListReadersA(hcontext, mszgroups.param().abi(), mszreaders.unwrap_or(core::mem::zeroed()) as _, pcchreaders as _) }
}
#[inline]
pub unsafe fn SCardListReadersW<P1>(hcontext: usize, mszgroups: P1, mszreaders: Option<windows_core::PWSTR>, pcchreaders: *mut u32) -> i32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("winscard.dll" "system" fn SCardListReadersW(hcontext : usize, mszgroups : windows_core::PCWSTR, mszreaders : windows_core::PWSTR, pcchreaders : *mut u32) -> i32);
    unsafe { SCardListReadersW(hcontext, mszgroups.param().abi(), mszreaders.unwrap_or(core::mem::zeroed()) as _, pcchreaders as _) }
}
#[inline]
pub unsafe fn SCardListReadersWithDeviceInstanceIdA<P1>(hcontext: usize, szdeviceinstanceid: P1, mszreaders: Option<windows_core::PSTR>, pcchreaders: *mut u32) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("winscard.dll" "system" fn SCardListReadersWithDeviceInstanceIdA(hcontext : usize, szdeviceinstanceid : windows_core::PCSTR, mszreaders : windows_core::PSTR, pcchreaders : *mut u32) -> i32);
    unsafe { SCardListReadersWithDeviceInstanceIdA(hcontext, szdeviceinstanceid.param().abi(), mszreaders.unwrap_or(core::mem::zeroed()) as _, pcchreaders as _) }
}
#[inline]
pub unsafe fn SCardListReadersWithDeviceInstanceIdW<P1>(hcontext: usize, szdeviceinstanceid: P1, mszreaders: Option<windows_core::PWSTR>, pcchreaders: *mut u32) -> i32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("winscard.dll" "system" fn SCardListReadersWithDeviceInstanceIdW(hcontext : usize, szdeviceinstanceid : windows_core::PCWSTR, mszreaders : windows_core::PWSTR, pcchreaders : *mut u32) -> i32);
    unsafe { SCardListReadersWithDeviceInstanceIdW(hcontext, szdeviceinstanceid.param().abi(), mszreaders.unwrap_or(core::mem::zeroed()) as _, pcchreaders as _) }
}
#[inline]
pub unsafe fn SCardLocateCardsA<P1>(hcontext: usize, mszcards: P1, rgreaderstates: *mut SCARD_READERSTATEA, creaders: u32) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("winscard.dll" "system" fn SCardLocateCardsA(hcontext : usize, mszcards : windows_core::PCSTR, rgreaderstates : *mut SCARD_READERSTATEA, creaders : u32) -> i32);
    unsafe { SCardLocateCardsA(hcontext, mszcards.param().abi(), rgreaderstates as _, creaders) }
}
#[inline]
pub unsafe fn SCardLocateCardsByATRA(hcontext: usize, rgatrmasks: *const SCARD_ATRMASK, catrs: u32, rgreaderstates: *mut SCARD_READERSTATEA, creaders: u32) -> i32 {
    windows_link::link!("winscard.dll" "system" fn SCardLocateCardsByATRA(hcontext : usize, rgatrmasks : *const SCARD_ATRMASK, catrs : u32, rgreaderstates : *mut SCARD_READERSTATEA, creaders : u32) -> i32);
    unsafe { SCardLocateCardsByATRA(hcontext, rgatrmasks, catrs, rgreaderstates as _, creaders) }
}
#[inline]
pub unsafe fn SCardLocateCardsByATRW(hcontext: usize, rgatrmasks: *const SCARD_ATRMASK, catrs: u32, rgreaderstates: *mut SCARD_READERSTATEW, creaders: u32) -> i32 {
    windows_link::link!("winscard.dll" "system" fn SCardLocateCardsByATRW(hcontext : usize, rgatrmasks : *const SCARD_ATRMASK, catrs : u32, rgreaderstates : *mut SCARD_READERSTATEW, creaders : u32) -> i32);
    unsafe { SCardLocateCardsByATRW(hcontext, rgatrmasks, catrs, rgreaderstates as _, creaders) }
}
#[inline]
pub unsafe fn SCardLocateCardsW<P1>(hcontext: usize, mszcards: P1, rgreaderstates: *mut SCARD_READERSTATEW, creaders: u32) -> i32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("winscard.dll" "system" fn SCardLocateCardsW(hcontext : usize, mszcards : windows_core::PCWSTR, rgreaderstates : *mut SCARD_READERSTATEW, creaders : u32) -> i32);
    unsafe { SCardLocateCardsW(hcontext, mszcards.param().abi(), rgreaderstates as _, creaders) }
}
#[inline]
pub unsafe fn SCardReadCacheA<P3>(hcontext: usize, cardidentifier: *const windows_core::GUID, freshnesscounter: u32, lookupname: P3, data: *mut u8, datalen: *mut u32) -> i32
where
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("winscard.dll" "system" fn SCardReadCacheA(hcontext : usize, cardidentifier : *const windows_core::GUID, freshnesscounter : u32, lookupname : windows_core::PCSTR, data : *mut u8, datalen : *mut u32) -> i32);
    unsafe { SCardReadCacheA(hcontext, cardidentifier, freshnesscounter, lookupname.param().abi(), data as _, datalen as _) }
}
#[inline]
pub unsafe fn SCardReadCacheW<P3>(hcontext: usize, cardidentifier: *const windows_core::GUID, freshnesscounter: u32, lookupname: P3, data: *mut u8, datalen: *mut u32) -> i32
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("winscard.dll" "system" fn SCardReadCacheW(hcontext : usize, cardidentifier : *const windows_core::GUID, freshnesscounter : u32, lookupname : windows_core::PCWSTR, data : *mut u8, datalen : *mut u32) -> i32);
    unsafe { SCardReadCacheW(hcontext, cardidentifier, freshnesscounter, lookupname.param().abi(), data as _, datalen as _) }
}
#[inline]
pub unsafe fn SCardReconnect(hcard: usize, dwsharemode: u32, dwpreferredprotocols: u32, dwinitialization: u32, pdwactiveprotocol: Option<*mut u32>) -> i32 {
    windows_link::link!("winscard.dll" "system" fn SCardReconnect(hcard : usize, dwsharemode : u32, dwpreferredprotocols : u32, dwinitialization : u32, pdwactiveprotocol : *mut u32) -> i32);
    unsafe { SCardReconnect(hcard, dwsharemode, dwpreferredprotocols, dwinitialization, pdwactiveprotocol.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SCardReleaseContext(hcontext: usize) -> i32 {
    windows_link::link!("winscard.dll" "system" fn SCardReleaseContext(hcontext : usize) -> i32);
    unsafe { SCardReleaseContext(hcontext) }
}
#[inline]
pub unsafe fn SCardReleaseStartedEvent() {
    windows_link::link!("winscard.dll" "system" fn SCardReleaseStartedEvent());
    unsafe { SCardReleaseStartedEvent() }
}
#[inline]
pub unsafe fn SCardRemoveReaderFromGroupA<P1, P2>(hcontext: usize, szreadername: P1, szgroupname: P2) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("winscard.dll" "system" fn SCardRemoveReaderFromGroupA(hcontext : usize, szreadername : windows_core::PCSTR, szgroupname : windows_core::PCSTR) -> i32);
    unsafe { SCardRemoveReaderFromGroupA(hcontext, szreadername.param().abi(), szgroupname.param().abi()) }
}
#[inline]
pub unsafe fn SCardRemoveReaderFromGroupW<P1, P2>(hcontext: usize, szreadername: P1, szgroupname: P2) -> i32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("winscard.dll" "system" fn SCardRemoveReaderFromGroupW(hcontext : usize, szreadername : windows_core::PCWSTR, szgroupname : windows_core::PCWSTR) -> i32);
    unsafe { SCardRemoveReaderFromGroupW(hcontext, szreadername.param().abi(), szgroupname.param().abi()) }
}
#[inline]
pub unsafe fn SCardSetAttrib(hcard: usize, dwattrid: u32, pbattr: &[u8]) -> i32 {
    windows_link::link!("winscard.dll" "system" fn SCardSetAttrib(hcard : usize, dwattrid : u32, pbattr : *const u8, cbattrlen : u32) -> i32);
    unsafe { SCardSetAttrib(hcard, dwattrid, core::mem::transmute(pbattr.as_ptr()), pbattr.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn SCardSetCardTypeProviderNameA<P1, P3>(hcontext: usize, szcardname: P1, dwproviderid: u32, szprovider: P3) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("winscard.dll" "system" fn SCardSetCardTypeProviderNameA(hcontext : usize, szcardname : windows_core::PCSTR, dwproviderid : u32, szprovider : windows_core::PCSTR) -> i32);
    unsafe { SCardSetCardTypeProviderNameA(hcontext, szcardname.param().abi(), dwproviderid, szprovider.param().abi()) }
}
#[inline]
pub unsafe fn SCardSetCardTypeProviderNameW<P1, P3>(hcontext: usize, szcardname: P1, dwproviderid: u32, szprovider: P3) -> i32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("winscard.dll" "system" fn SCardSetCardTypeProviderNameW(hcontext : usize, szcardname : windows_core::PCWSTR, dwproviderid : u32, szprovider : windows_core::PCWSTR) -> i32);
    unsafe { SCardSetCardTypeProviderNameW(hcontext, szcardname.param().abi(), dwproviderid, szprovider.param().abi()) }
}
#[inline]
pub unsafe fn SCardState(hcard: usize, pdwstate: *mut u32, pdwprotocol: *mut u32, pbatr: *mut u8, pcbatrlen: *mut u32) -> i32 {
    windows_link::link!("winscard.dll" "system" fn SCardState(hcard : usize, pdwstate : *mut u32, pdwprotocol : *mut u32, pbatr : *mut u8, pcbatrlen : *mut u32) -> i32);
    unsafe { SCardState(hcard, pdwstate as _, pdwprotocol as _, pbatr as _, pcbatrlen as _) }
}
#[inline]
pub unsafe fn SCardStatusA(hcard: usize, mszreadernames: Option<windows_core::PSTR>, pcchreaderlen: Option<*mut u32>, pdwstate: Option<*mut u32>, pdwprotocol: Option<*mut u32>, pbatr: Option<*mut u8>, pcbatrlen: Option<*mut u32>) -> i32 {
    windows_link::link!("winscard.dll" "system" fn SCardStatusA(hcard : usize, mszreadernames : windows_core::PSTR, pcchreaderlen : *mut u32, pdwstate : *mut u32, pdwprotocol : *mut u32, pbatr : *mut u8, pcbatrlen : *mut u32) -> i32);
    unsafe { SCardStatusA(hcard, mszreadernames.unwrap_or(core::mem::zeroed()) as _, pcchreaderlen.unwrap_or(core::mem::zeroed()) as _, pdwstate.unwrap_or(core::mem::zeroed()) as _, pdwprotocol.unwrap_or(core::mem::zeroed()) as _, pbatr.unwrap_or(core::mem::zeroed()) as _, pcbatrlen.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SCardStatusW(hcard: usize, mszreadernames: Option<windows_core::PWSTR>, pcchreaderlen: Option<*mut u32>, pdwstate: Option<*mut u32>, pdwprotocol: Option<*mut u32>, pbatr: Option<*mut u8>, pcbatrlen: Option<*mut u32>) -> i32 {
    windows_link::link!("winscard.dll" "system" fn SCardStatusW(hcard : usize, mszreadernames : windows_core::PWSTR, pcchreaderlen : *mut u32, pdwstate : *mut u32, pdwprotocol : *mut u32, pbatr : *mut u8, pcbatrlen : *mut u32) -> i32);
    unsafe { SCardStatusW(hcard, mszreadernames.unwrap_or(core::mem::zeroed()) as _, pcchreaderlen.unwrap_or(core::mem::zeroed()) as _, pdwstate.unwrap_or(core::mem::zeroed()) as _, pdwprotocol.unwrap_or(core::mem::zeroed()) as _, pbatr.unwrap_or(core::mem::zeroed()) as _, pcbatrlen.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SCardTransmit(hcard: usize, piosendpci: *const SCARD_IO_REQUEST, pbsendbuffer: &[u8], piorecvpci: Option<*mut SCARD_IO_REQUEST>, pbrecvbuffer: *mut u8, pcbrecvlength: *mut u32) -> i32 {
    windows_link::link!("winscard.dll" "system" fn SCardTransmit(hcard : usize, piosendpci : *const SCARD_IO_REQUEST, pbsendbuffer : *const u8, cbsendlength : u32, piorecvpci : *mut SCARD_IO_REQUEST, pbrecvbuffer : *mut u8, pcbrecvlength : *mut u32) -> i32);
    unsafe { SCardTransmit(hcard, piosendpci, core::mem::transmute(pbsendbuffer.as_ptr()), pbsendbuffer.len().try_into().unwrap(), piorecvpci.unwrap_or(core::mem::zeroed()) as _, pbrecvbuffer as _, pcbrecvlength as _) }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn SCardUIDlgSelectCardA(param0: *mut OPENCARDNAME_EXA) -> i32 {
    windows_link::link!("scarddlg.dll" "system" fn SCardUIDlgSelectCardA(param0 : *mut OPENCARDNAME_EXA) -> i32);
    unsafe { SCardUIDlgSelectCardA(param0 as _) }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn SCardUIDlgSelectCardW(param0: *mut OPENCARDNAME_EXW) -> i32 {
    windows_link::link!("scarddlg.dll" "system" fn SCardUIDlgSelectCardW(param0 : *mut OPENCARDNAME_EXW) -> i32);
    unsafe { SCardUIDlgSelectCardW(param0 as _) }
}
#[inline]
pub unsafe fn SCardWriteCacheA<P3>(hcontext: usize, cardidentifier: *const windows_core::GUID, freshnesscounter: u32, lookupname: P3, data: &[u8]) -> i32
where
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("winscard.dll" "system" fn SCardWriteCacheA(hcontext : usize, cardidentifier : *const windows_core::GUID, freshnesscounter : u32, lookupname : windows_core::PCSTR, data : *const u8, datalen : u32) -> i32);
    unsafe { SCardWriteCacheA(hcontext, cardidentifier, freshnesscounter, lookupname.param().abi(), core::mem::transmute(data.as_ptr()), data.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn SCardWriteCacheW<P3>(hcontext: usize, cardidentifier: *const windows_core::GUID, freshnesscounter: u32, lookupname: P3, data: &[u8]) -> i32
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("winscard.dll" "system" fn SCardWriteCacheW(hcontext : usize, cardidentifier : *const windows_core::GUID, freshnesscounter : u32, lookupname : windows_core::PCWSTR, data : *const u8, datalen : u32) -> i32);
    unsafe { SCardWriteCacheW(hcontext, cardidentifier, freshnesscounter, lookupname.param().abi(), core::mem::transmute(data.as_ptr()), data.len().try_into().unwrap()) }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BINARY_BLOB_CREDENTIAL_INFO {
    pub cbBlob: u32,
    pub pbBlob: *mut u8,
}
impl Default for BINARY_BLOB_CREDENTIAL_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const BinaryBlobCredential: CRED_MARSHAL_TYPE = CRED_MARSHAL_TYPE(3i32);
pub const BinaryBlobForSystem: CRED_MARSHAL_TYPE = CRED_MARSHAL_TYPE(5i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CERT_CREDENTIAL_INFO {
    pub cbSize: u32,
    pub rgbHashOfCert: [u8; 20],
}
impl Default for CERT_CREDENTIAL_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_HASH_LENGTH: u32 = 20u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CREDENTIALA {
    pub Flags: CRED_FLAGS,
    pub Type: CRED_TYPE,
    pub TargetName: windows_core::PSTR,
    pub Comment: windows_core::PSTR,
    pub LastWritten: super::super::Foundation::FILETIME,
    pub CredentialBlobSize: u32,
    pub CredentialBlob: *mut u8,
    pub Persist: CRED_PERSIST,
    pub AttributeCount: u32,
    pub Attributes: *mut CREDENTIAL_ATTRIBUTEA,
    pub TargetAlias: windows_core::PSTR,
    pub UserName: windows_core::PSTR,
}
impl Default for CREDENTIALA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CREDENTIALW {
    pub Flags: CRED_FLAGS,
    pub Type: CRED_TYPE,
    pub TargetName: windows_core::PWSTR,
    pub Comment: windows_core::PWSTR,
    pub LastWritten: super::super::Foundation::FILETIME,
    pub CredentialBlobSize: u32,
    pub CredentialBlob: *mut u8,
    pub Persist: CRED_PERSIST,
    pub AttributeCount: u32,
    pub Attributes: *mut CREDENTIAL_ATTRIBUTEW,
    pub TargetAlias: windows_core::PWSTR,
    pub UserName: windows_core::PWSTR,
}
impl Default for CREDENTIALW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CREDENTIAL_ATTRIBUTEA {
    pub Keyword: windows_core::PSTR,
    pub Flags: u32,
    pub ValueSize: u32,
    pub Value: *mut u8,
}
impl Default for CREDENTIAL_ATTRIBUTEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CREDENTIAL_ATTRIBUTEW {
    pub Keyword: windows_core::PWSTR,
    pub Flags: u32,
    pub ValueSize: u32,
    pub Value: *mut u8,
}
impl Default for CREDENTIAL_ATTRIBUTEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
    pub CredTypes: *mut u32,
}
impl Default for CREDENTIAL_TARGET_INFORMATIONA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
    pub CredTypes: *mut u32,
}
impl Default for CREDENTIAL_TARGET_INFORMATIONW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CREDSPP_SUBMIT_TYPE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CREDSSP_CRED {
    pub Type: CREDSPP_SUBMIT_TYPE,
    pub pSchannelCred: *mut core::ffi::c_void,
    pub pSpnegoCred: *mut core::ffi::c_void,
}
impl Default for CREDSSP_CRED {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CREDSSP_CRED_EX {
    pub Type: CREDSPP_SUBMIT_TYPE,
    pub Version: u32,
    pub Flags: u32,
    pub Reserved: u32,
    pub Cred: CREDSSP_CRED,
}
pub const CREDSSP_CRED_EX_VERSION: u32 = 0u32;
pub const CREDSSP_FLAG_REDIRECT: u32 = 1u32;
pub const CREDSSP_NAME: windows_core::PCWSTR = windows_core::w!("CREDSSP");
pub const CREDSSP_SERVER_AUTH_CERTIFICATE: u32 = 2u32;
pub const CREDSSP_SERVER_AUTH_LOOPBACK: u32 = 4u32;
pub const CREDSSP_SERVER_AUTH_NEGOTIATE: u32 = 1u32;
pub const CREDUIWIN_AUTHPACKAGE_ONLY: CREDUIWIN_FLAGS = CREDUIWIN_FLAGS(16u32);
pub const CREDUIWIN_CHECKBOX: CREDUIWIN_FLAGS = CREDUIWIN_FLAGS(2u32);
pub const CREDUIWIN_DOWNLEVEL_HELLO_AS_SMART_CARD: u32 = 2147483648u32;
pub const CREDUIWIN_ENUMERATE_ADMINS: CREDUIWIN_FLAGS = CREDUIWIN_FLAGS(256u32);
pub const CREDUIWIN_ENUMERATE_CURRENT_USER: CREDUIWIN_FLAGS = CREDUIWIN_FLAGS(512u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CREDUIWIN_FLAGS(pub u32);
impl CREDUIWIN_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CREDUIWIN_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CREDUIWIN_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CREDUIWIN_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CREDUIWIN_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CREDUIWIN_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const CREDUIWIN_GENERIC: CREDUIWIN_FLAGS = CREDUIWIN_FLAGS(1u32);
pub const CREDUIWIN_IGNORE_CLOUDAUTHORITY_NAME: u32 = 262144u32;
pub const CREDUIWIN_IN_CRED_ONLY: CREDUIWIN_FLAGS = CREDUIWIN_FLAGS(32u32);
pub const CREDUIWIN_PACK_32_WOW: CREDUIWIN_FLAGS = CREDUIWIN_FLAGS(268435456u32);
pub const CREDUIWIN_PREPROMPTING: CREDUIWIN_FLAGS = CREDUIWIN_FLAGS(8192u32);
pub const CREDUIWIN_SECURE_PROMPT: CREDUIWIN_FLAGS = CREDUIWIN_FLAGS(4096u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CREDUI_FLAGS(pub u32);
impl CREDUI_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CREDUI_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CREDUI_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CREDUI_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CREDUI_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CREDUI_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const CREDUI_FLAGS_ALWAYS_SHOW_UI: CREDUI_FLAGS = CREDUI_FLAGS(128u32);
pub const CREDUI_FLAGS_COMPLETE_USERNAME: CREDUI_FLAGS = CREDUI_FLAGS(2048u32);
pub const CREDUI_FLAGS_DO_NOT_PERSIST: CREDUI_FLAGS = CREDUI_FLAGS(2u32);
pub const CREDUI_FLAGS_EXCLUDE_CERTIFICATES: CREDUI_FLAGS = CREDUI_FLAGS(8u32);
pub const CREDUI_FLAGS_EXPECT_CONFIRMATION: CREDUI_FLAGS = CREDUI_FLAGS(131072u32);
pub const CREDUI_FLAGS_GENERIC_CREDENTIALS: CREDUI_FLAGS = CREDUI_FLAGS(262144u32);
pub const CREDUI_FLAGS_INCORRECT_PASSWORD: CREDUI_FLAGS = CREDUI_FLAGS(1u32);
pub const CREDUI_FLAGS_KEEP_USERNAME: CREDUI_FLAGS = CREDUI_FLAGS(1048576u32);
pub const CREDUI_FLAGS_PASSWORD_ONLY_OK: CREDUI_FLAGS = CREDUI_FLAGS(512u32);
pub const CREDUI_FLAGS_PERSIST: CREDUI_FLAGS = CREDUI_FLAGS(4096u32);
pub const CREDUI_FLAGS_REQUEST_ADMINISTRATOR: CREDUI_FLAGS = CREDUI_FLAGS(4u32);
pub const CREDUI_FLAGS_REQUIRE_CERTIFICATE: CREDUI_FLAGS = CREDUI_FLAGS(16u32);
pub const CREDUI_FLAGS_REQUIRE_SMARTCARD: CREDUI_FLAGS = CREDUI_FLAGS(256u32);
pub const CREDUI_FLAGS_SERVER_CREDENTIAL: CREDUI_FLAGS = CREDUI_FLAGS(16384u32);
pub const CREDUI_FLAGS_SHOW_SAVE_CHECK_BOX: CREDUI_FLAGS = CREDUI_FLAGS(64u32);
pub const CREDUI_FLAGS_USERNAME_TARGET_CREDENTIALS: CREDUI_FLAGS = CREDUI_FLAGS(524288u32);
pub const CREDUI_FLAGS_VALIDATE_USERNAME: CREDUI_FLAGS = CREDUI_FLAGS(1024u32);
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CREDUI_INFOA {
    pub cbSize: u32,
    pub hwndParent: super::super::Foundation::HWND,
    pub pszMessageText: windows_core::PCSTR,
    pub pszCaptionText: windows_core::PCSTR,
    pub hbmBanner: super::super::Graphics::Gdi::HBITMAP,
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CREDUI_INFOW {
    pub cbSize: u32,
    pub hwndParent: super::super::Foundation::HWND,
    pub pszMessageText: windows_core::PCWSTR,
    pub pszCaptionText: windows_core::PCWSTR,
    pub hbmBanner: super::super::Graphics::Gdi::HBITMAP,
}
pub const CREDUI_MAX_CAPTION_LENGTH: u32 = 128u32;
pub const CREDUI_MAX_DOMAIN_TARGET_LENGTH: u32 = 337u32;
pub const CREDUI_MAX_GENERIC_TARGET_LENGTH: u32 = 32767u32;
pub const CREDUI_MAX_MESSAGE_LENGTH: u32 = 1024u32;
pub const CREDUI_MAX_USERNAME_LENGTH: u32 = 513u32;
pub const CRED_ALLOW_NAME_RESOLUTION: u32 = 1u32;
pub const CRED_CACHE_TARGET_INFORMATION: u32 = 1u32;
pub const CRED_ENUMERATE_ALL_CREDENTIALS: CRED_ENUMERATE_FLAGS = CRED_ENUMERATE_FLAGS(1u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CRED_ENUMERATE_FLAGS(pub u32);
impl CRED_ENUMERATE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CRED_ENUMERATE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CRED_ENUMERATE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CRED_ENUMERATE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CRED_ENUMERATE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CRED_ENUMERATE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CRED_FLAGS(pub u32);
impl CRED_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CRED_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CRED_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CRED_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CRED_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CRED_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const CRED_FLAGS_NGC_CERT: CRED_FLAGS = CRED_FLAGS(128u32);
pub const CRED_FLAGS_OWF_CRED_BLOB: CRED_FLAGS = CRED_FLAGS(8u32);
pub const CRED_FLAGS_PASSWORD_FOR_CERT: CRED_FLAGS = CRED_FLAGS(1u32);
pub const CRED_FLAGS_PROMPT_NOW: CRED_FLAGS = CRED_FLAGS(2u32);
pub const CRED_FLAGS_REQUIRE_CONFIRMATION: CRED_FLAGS = CRED_FLAGS(16u32);
pub const CRED_FLAGS_USERNAME_TARGET: CRED_FLAGS = CRED_FLAGS(4u32);
pub const CRED_FLAGS_VALID_FLAGS: CRED_FLAGS = CRED_FLAGS(61695u32);
pub const CRED_FLAGS_VALID_INPUT_FLAGS: CRED_FLAGS = CRED_FLAGS(61599u32);
pub const CRED_FLAGS_VSM_PROTECTED: CRED_FLAGS = CRED_FLAGS(64u32);
pub const CRED_FLAGS_WILDCARD_MATCH: CRED_FLAGS = CRED_FLAGS(32u32);
pub const CRED_LOGON_TYPES_MASK: u32 = 61440u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CRED_MARSHAL_TYPE(pub i32);
pub const CRED_MAX_ATTRIBUTES: u32 = 64u32;
pub const CRED_MAX_CREDENTIAL_BLOB_SIZE: u32 = 2560u32;
pub const CRED_MAX_DOMAIN_TARGET_NAME_LENGTH: u32 = 337u32;
pub const CRED_MAX_GENERIC_TARGET_NAME_LENGTH: u32 = 32767u32;
pub const CRED_MAX_STRING_LENGTH: u32 = 256u32;
pub const CRED_MAX_TARGETNAME_ATTRIBUTE_LENGTH: u32 = 256u32;
pub const CRED_MAX_TARGETNAME_NAMESPACE_LENGTH: u32 = 256u32;
pub const CRED_MAX_USERNAME_LENGTH: u32 = 513u32;
pub const CRED_MAX_VALUE_SIZE: u32 = 256u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CRED_PACK_FLAGS(pub u32);
impl CRED_PACK_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CRED_PACK_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CRED_PACK_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CRED_PACK_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CRED_PACK_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CRED_PACK_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const CRED_PACK_GENERIC_CREDENTIALS: CRED_PACK_FLAGS = CRED_PACK_FLAGS(4u32);
pub const CRED_PACK_ID_PROVIDER_CREDENTIALS: CRED_PACK_FLAGS = CRED_PACK_FLAGS(8u32);
pub const CRED_PACK_PROTECTED_CREDENTIALS: CRED_PACK_FLAGS = CRED_PACK_FLAGS(1u32);
pub const CRED_PACK_WOW_BUFFER: CRED_PACK_FLAGS = CRED_PACK_FLAGS(2u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CRED_PERSIST(pub u32);
pub const CRED_PERSIST_ENTERPRISE: CRED_PERSIST = CRED_PERSIST(3u32);
pub const CRED_PERSIST_LOCAL_MACHINE: CRED_PERSIST = CRED_PERSIST(2u32);
pub const CRED_PERSIST_NONE: CRED_PERSIST = CRED_PERSIST(0u32);
pub const CRED_PERSIST_SESSION: CRED_PERSIST = CRED_PERSIST(1u32);
pub const CRED_PRESERVE_CREDENTIAL_BLOB: u32 = 1u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CRED_PROTECTION_TYPE(pub i32);
pub const CRED_PROTECT_AS_SELF: u32 = 1u32;
pub const CRED_PROTECT_TO_SYSTEM: u32 = 2u32;
pub const CRED_SESSION_WILDCARD_NAME: windows_core::PCWSTR = windows_core::w!("*Session");
pub const CRED_SESSION_WILDCARD_NAME_A: windows_core::PCSTR = windows_core::s!("*Session");
pub const CRED_SESSION_WILDCARD_NAME_W: windows_core::PCWSTR = windows_core::w!("*Session");
pub const CRED_TARGETNAME_ATTRIBUTE_BATCH: windows_core::PCWSTR = windows_core::w!("batch");
pub const CRED_TARGETNAME_ATTRIBUTE_BATCH_A: windows_core::PCSTR = windows_core::s!("batch");
pub const CRED_TARGETNAME_ATTRIBUTE_BATCH_W: windows_core::PCWSTR = windows_core::w!("batch");
pub const CRED_TARGETNAME_ATTRIBUTE_CACHEDINTERACTIVE: windows_core::PCWSTR = windows_core::w!("cachedinteractive");
pub const CRED_TARGETNAME_ATTRIBUTE_CACHEDINTERACTIVE_A: windows_core::PCSTR = windows_core::s!("cachedinteractive");
pub const CRED_TARGETNAME_ATTRIBUTE_CACHEDINTERACTIVE_W: windows_core::PCWSTR = windows_core::w!("cachedinteractive");
pub const CRED_TARGETNAME_ATTRIBUTE_INTERACTIVE: windows_core::PCWSTR = windows_core::w!("interactive");
pub const CRED_TARGETNAME_ATTRIBUTE_INTERACTIVE_A: windows_core::PCSTR = windows_core::s!("interactive");
pub const CRED_TARGETNAME_ATTRIBUTE_INTERACTIVE_W: windows_core::PCWSTR = windows_core::w!("interactive");
pub const CRED_TARGETNAME_ATTRIBUTE_NAME: windows_core::PCWSTR = windows_core::w!("name");
pub const CRED_TARGETNAME_ATTRIBUTE_NAME_A: windows_core::PCSTR = windows_core::s!("name");
pub const CRED_TARGETNAME_ATTRIBUTE_NAME_W: windows_core::PCWSTR = windows_core::w!("name");
pub const CRED_TARGETNAME_ATTRIBUTE_NETWORK: windows_core::PCWSTR = windows_core::w!("network");
pub const CRED_TARGETNAME_ATTRIBUTE_NETWORKCLEARTEXT: windows_core::PCWSTR = windows_core::w!("networkcleartext");
pub const CRED_TARGETNAME_ATTRIBUTE_NETWORKCLEARTEXT_A: windows_core::PCSTR = windows_core::s!("networkcleartext");
pub const CRED_TARGETNAME_ATTRIBUTE_NETWORKCLEARTEXT_W: windows_core::PCWSTR = windows_core::w!("networkcleartext");
pub const CRED_TARGETNAME_ATTRIBUTE_NETWORK_A: windows_core::PCSTR = windows_core::s!("network");
pub const CRED_TARGETNAME_ATTRIBUTE_NETWORK_W: windows_core::PCWSTR = windows_core::w!("network");
pub const CRED_TARGETNAME_ATTRIBUTE_REMOTEINTERACTIVE: windows_core::PCWSTR = windows_core::w!("remoteinteractive");
pub const CRED_TARGETNAME_ATTRIBUTE_REMOTEINTERACTIVE_A: windows_core::PCSTR = windows_core::s!("remoteinteractive");
pub const CRED_TARGETNAME_ATTRIBUTE_REMOTEINTERACTIVE_W: windows_core::PCWSTR = windows_core::w!("remoteinteractive");
pub const CRED_TARGETNAME_ATTRIBUTE_SERVICE: windows_core::PCWSTR = windows_core::w!("service");
pub const CRED_TARGETNAME_ATTRIBUTE_SERVICE_A: windows_core::PCSTR = windows_core::s!("service");
pub const CRED_TARGETNAME_ATTRIBUTE_SERVICE_W: windows_core::PCWSTR = windows_core::w!("service");
pub const CRED_TARGETNAME_ATTRIBUTE_TARGET: windows_core::PCWSTR = windows_core::w!("target");
pub const CRED_TARGETNAME_ATTRIBUTE_TARGET_A: windows_core::PCSTR = windows_core::s!("target");
pub const CRED_TARGETNAME_ATTRIBUTE_TARGET_W: windows_core::PCWSTR = windows_core::w!("target");
pub const CRED_TARGETNAME_DOMAIN_NAMESPACE: windows_core::PCWSTR = windows_core::w!("Domain");
pub const CRED_TARGETNAME_DOMAIN_NAMESPACE_A: windows_core::PCSTR = windows_core::s!("Domain");
pub const CRED_TARGETNAME_DOMAIN_NAMESPACE_W: windows_core::PCWSTR = windows_core::w!("Domain");
pub const CRED_TARGETNAME_LEGACYGENERIC_NAMESPACE_A: windows_core::PCSTR = windows_core::s!("LegacyGeneric");
pub const CRED_TARGETNAME_LEGACYGENERIC_NAMESPACE_W: windows_core::PCWSTR = windows_core::w!("LegacyGeneric");
pub const CRED_TI_CREATE_EXPLICIT_CRED: u32 = 16u32;
pub const CRED_TI_DNSTREE_IS_DFS_SERVER: u32 = 64u32;
pub const CRED_TI_DOMAIN_FORMAT_UNKNOWN: u32 = 2u32;
pub const CRED_TI_ONLY_PASSWORD_REQUIRED: u32 = 4u32;
pub const CRED_TI_SERVER_FORMAT_UNKNOWN: u32 = 1u32;
pub const CRED_TI_USERNAME_TARGET: u32 = 8u32;
pub const CRED_TI_VALID_FLAGS: u32 = 61567u32;
pub const CRED_TI_WORKGROUP_MEMBER: u32 = 32u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CRED_TYPE(pub u32);
pub const CRED_TYPE_DOMAIN_CERTIFICATE: CRED_TYPE = CRED_TYPE(3u32);
pub const CRED_TYPE_DOMAIN_EXTENDED: CRED_TYPE = CRED_TYPE(6u32);
pub const CRED_TYPE_DOMAIN_PASSWORD: CRED_TYPE = CRED_TYPE(2u32);
pub const CRED_TYPE_DOMAIN_VISIBLE_PASSWORD: CRED_TYPE = CRED_TYPE(4u32);
pub const CRED_TYPE_GENERIC: CRED_TYPE = CRED_TYPE(1u32);
pub const CRED_TYPE_GENERIC_CERTIFICATE: CRED_TYPE = CRED_TYPE(5u32);
pub const CRED_TYPE_MAXIMUM: CRED_TYPE = CRED_TYPE(7u32);
pub const CRED_TYPE_MAXIMUM_EX: CRED_TYPE = CRED_TYPE(1007u32);
pub const CRED_UNPROTECT_ALLOW_TO_SYSTEM: u32 = 2u32;
pub const CRED_UNPROTECT_AS_SELF: u32 = 1u32;
pub const CertCredential: CRED_MARSHAL_TYPE = CRED_MARSHAL_TYPE(1i32);
pub const CredForSystemProtection: CRED_PROTECTION_TYPE = CRED_PROTECTION_TYPE(3i32);
pub const CredTrustedProtection: CRED_PROTECTION_TYPE = CRED_PROTECTION_TYPE(2i32);
pub const CredUnprotected: CRED_PROTECTION_TYPE = CRED_PROTECTION_TYPE(0i32);
pub const CredUserProtection: CRED_PROTECTION_TYPE = CRED_PROTECTION_TYPE(1i32);
pub const CredsspCertificateCreds: CREDSPP_SUBMIT_TYPE = CREDSPP_SUBMIT_TYPE(13i32);
pub const CredsspCredEx: CREDSPP_SUBMIT_TYPE = CREDSPP_SUBMIT_TYPE(100i32);
pub const CredsspPasswordCreds: CREDSPP_SUBMIT_TYPE = CREDSPP_SUBMIT_TYPE(2i32);
pub const CredsspSchannelCreds: CREDSPP_SUBMIT_TYPE = CREDSPP_SUBMIT_TYPE(4i32);
pub const CredsspSubmitBufferBoth: CREDSPP_SUBMIT_TYPE = CREDSPP_SUBMIT_TYPE(50i32);
pub const CredsspSubmitBufferBothOld: CREDSPP_SUBMIT_TYPE = CREDSPP_SUBMIT_TYPE(51i32);
pub const FILE_DEVICE_SMARTCARD: u32 = 49u32;
pub const GUID_DEVINTERFACE_SMARTCARD_READER: windows_core::GUID = windows_core::GUID::from_u128(0x50dd5230_ba8a_11d1_bf5d_0000f805f530);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KeyCredentialManagerInfo {
    pub containerId: windows_core::GUID,
}
pub const KeyCredentialManagerOperationErrorStateCertificateFailure: KeyCredentialManagerOperationErrorStates = KeyCredentialManagerOperationErrorStates(4i32);
pub const KeyCredentialManagerOperationErrorStateDeviceJoinFailure: KeyCredentialManagerOperationErrorStates = KeyCredentialManagerOperationErrorStates(1i32);
pub const KeyCredentialManagerOperationErrorStateHardwareFailure: KeyCredentialManagerOperationErrorStates = KeyCredentialManagerOperationErrorStates(32i32);
pub const KeyCredentialManagerOperationErrorStateNone: KeyCredentialManagerOperationErrorStates = KeyCredentialManagerOperationErrorStates(0i32);
pub const KeyCredentialManagerOperationErrorStatePinExistsFailure: KeyCredentialManagerOperationErrorStates = KeyCredentialManagerOperationErrorStates(64i32);
pub const KeyCredentialManagerOperationErrorStatePolicyFailure: KeyCredentialManagerOperationErrorStates = KeyCredentialManagerOperationErrorStates(16i32);
pub const KeyCredentialManagerOperationErrorStateRemoteSessionFailure: KeyCredentialManagerOperationErrorStates = KeyCredentialManagerOperationErrorStates(8i32);
pub const KeyCredentialManagerOperationErrorStateTokenFailure: KeyCredentialManagerOperationErrorStates = KeyCredentialManagerOperationErrorStates(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KeyCredentialManagerOperationErrorStates(pub i32);
impl KeyCredentialManagerOperationErrorStates {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for KeyCredentialManagerOperationErrorStates {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for KeyCredentialManagerOperationErrorStates {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for KeyCredentialManagerOperationErrorStates {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for KeyCredentialManagerOperationErrorStates {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for KeyCredentialManagerOperationErrorStates {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KeyCredentialManagerOperationType(pub i32);
pub const KeyCredentialManagerPinChange: KeyCredentialManagerOperationType = KeyCredentialManagerOperationType(1i32);
pub const KeyCredentialManagerPinReset: KeyCredentialManagerOperationType = KeyCredentialManagerOperationType(2i32);
pub const KeyCredentialManagerProvisioning: KeyCredentialManagerOperationType = KeyCredentialManagerOperationType(0i32);
pub type LPOCNCHKPROC = Option<unsafe extern "system" fn(param0: usize, param1: usize, param2: *const core::ffi::c_void) -> windows_core::BOOL>;
pub type LPOCNCONNPROCA = Option<unsafe extern "system" fn(param0: usize, param1: windows_core::PCSTR, param2: windows_core::PCSTR, param3: *const core::ffi::c_void) -> usize>;
pub type LPOCNCONNPROCW = Option<unsafe extern "system" fn(param0: usize, param1: windows_core::PCWSTR, param2: windows_core::PCWSTR, param3: *const core::ffi::c_void) -> usize>;
pub type LPOCNDSCPROC = Option<unsafe extern "system" fn(param0: usize, param1: usize, param2: *const core::ffi::c_void)>;
pub const MAXIMUM_ATTR_STRING_LENGTH: u32 = 32u32;
pub const MAXIMUM_SMARTCARD_READERS: u32 = 10u32;
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct OPENCARDNAMEA {
    pub dwStructSize: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub hSCardContext: usize,
    pub lpstrGroupNames: windows_core::PSTR,
    pub nMaxGroupNames: u32,
    pub lpstrCardNames: windows_core::PSTR,
    pub nMaxCardNames: u32,
    pub rgguidInterfaces: *const windows_core::GUID,
    pub cguidInterfaces: u32,
    pub lpstrRdr: windows_core::PSTR,
    pub nMaxRdr: u32,
    pub lpstrCard: windows_core::PSTR,
    pub nMaxCard: u32,
    pub lpstrTitle: windows_core::PCSTR,
    pub dwFlags: u32,
    pub pvUserData: *mut core::ffi::c_void,
    pub dwShareMode: u32,
    pub dwPreferredProtocols: u32,
    pub dwActiveProtocol: u32,
    pub lpfnConnect: LPOCNCONNPROCA,
    pub lpfnCheck: LPOCNCHKPROC,
    pub lpfnDisconnect: LPOCNDSCPROC,
    pub hCardHandle: usize,
}
impl Default for OPENCARDNAMEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct OPENCARDNAMEW {
    pub dwStructSize: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub hSCardContext: usize,
    pub lpstrGroupNames: windows_core::PWSTR,
    pub nMaxGroupNames: u32,
    pub lpstrCardNames: windows_core::PWSTR,
    pub nMaxCardNames: u32,
    pub rgguidInterfaces: *const windows_core::GUID,
    pub cguidInterfaces: u32,
    pub lpstrRdr: windows_core::PWSTR,
    pub nMaxRdr: u32,
    pub lpstrCard: windows_core::PWSTR,
    pub nMaxCard: u32,
    pub lpstrTitle: windows_core::PCWSTR,
    pub dwFlags: u32,
    pub pvUserData: *mut core::ffi::c_void,
    pub dwShareMode: u32,
    pub dwPreferredProtocols: u32,
    pub dwActiveProtocol: u32,
    pub lpfnConnect: LPOCNCONNPROCW,
    pub lpfnCheck: LPOCNCHKPROC,
    pub lpfnDisconnect: LPOCNDSCPROC,
    pub hCardHandle: usize,
}
impl Default for OPENCARDNAMEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[derive(Clone, Copy, Debug)]
pub struct OPENCARDNAME_EXA {
    pub dwStructSize: u32,
    pub hSCardContext: usize,
    pub hwndOwner: super::super::Foundation::HWND,
    pub dwFlags: u32,
    pub lpstrTitle: windows_core::PCSTR,
    pub lpstrSearchDesc: windows_core::PCSTR,
    pub hIcon: super::super::UI::WindowsAndMessaging::HICON,
    pub pOpenCardSearchCriteria: *mut OPENCARD_SEARCH_CRITERIAA,
    pub lpfnConnect: LPOCNCONNPROCA,
    pub pvUserData: *mut core::ffi::c_void,
    pub dwShareMode: u32,
    pub dwPreferredProtocols: u32,
    pub lpstrRdr: windows_core::PSTR,
    pub nMaxRdr: u32,
    pub lpstrCard: windows_core::PSTR,
    pub nMaxCard: u32,
    pub dwActiveProtocol: u32,
    pub hCardHandle: usize,
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl Default for OPENCARDNAME_EXA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[derive(Clone, Copy, Debug)]
pub struct OPENCARDNAME_EXW {
    pub dwStructSize: u32,
    pub hSCardContext: usize,
    pub hwndOwner: super::super::Foundation::HWND,
    pub dwFlags: u32,
    pub lpstrTitle: windows_core::PCWSTR,
    pub lpstrSearchDesc: windows_core::PCWSTR,
    pub hIcon: super::super::UI::WindowsAndMessaging::HICON,
    pub pOpenCardSearchCriteria: *mut OPENCARD_SEARCH_CRITERIAW,
    pub lpfnConnect: LPOCNCONNPROCW,
    pub pvUserData: *mut core::ffi::c_void,
    pub dwShareMode: u32,
    pub dwPreferredProtocols: u32,
    pub lpstrRdr: windows_core::PWSTR,
    pub nMaxRdr: u32,
    pub lpstrCard: windows_core::PWSTR,
    pub nMaxCard: u32,
    pub dwActiveProtocol: u32,
    pub hCardHandle: usize,
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl Default for OPENCARDNAME_EXW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct OPENCARD_SEARCH_CRITERIAA {
    pub dwStructSize: u32,
    pub lpstrGroupNames: windows_core::PSTR,
    pub nMaxGroupNames: u32,
    pub rgguidInterfaces: *const windows_core::GUID,
    pub cguidInterfaces: u32,
    pub lpstrCardNames: windows_core::PSTR,
    pub nMaxCardNames: u32,
    pub lpfnCheck: LPOCNCHKPROC,
    pub lpfnConnect: LPOCNCONNPROCA,
    pub lpfnDisconnect: LPOCNDSCPROC,
    pub pvUserData: *mut core::ffi::c_void,
    pub dwShareMode: u32,
    pub dwPreferredProtocols: u32,
}
impl Default for OPENCARD_SEARCH_CRITERIAA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct OPENCARD_SEARCH_CRITERIAW {
    pub dwStructSize: u32,
    pub lpstrGroupNames: windows_core::PWSTR,
    pub nMaxGroupNames: u32,
    pub rgguidInterfaces: *const windows_core::GUID,
    pub cguidInterfaces: u32,
    pub lpstrCardNames: windows_core::PWSTR,
    pub nMaxCardNames: u32,
    pub lpfnCheck: LPOCNCHKPROC,
    pub lpfnConnect: LPOCNCONNPROCW,
    pub lpfnDisconnect: LPOCNDSCPROC,
    pub pvUserData: *mut core::ffi::c_void,
    pub dwShareMode: u32,
    pub dwPreferredProtocols: u32,
}
impl Default for OPENCARD_SEARCH_CRITERIAW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct READER_SEL_REQUEST {
    pub dwShareMode: u32,
    pub dwPreferredProtocols: u32,
    pub MatchType: READER_SEL_REQUEST_MATCH_TYPE,
    pub Anonymous: READER_SEL_REQUEST_0,
}
impl Default for READER_SEL_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union READER_SEL_REQUEST_0 {
    pub ReaderAndContainerParameter: READER_SEL_REQUEST_0_0,
    pub SerialNumberParameter: READER_SEL_REQUEST_0_1,
}
impl Default for READER_SEL_REQUEST_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct READER_SEL_REQUEST_0_0 {
    pub cbReaderNameOffset: u32,
    pub cchReaderNameLength: u32,
    pub cbContainerNameOffset: u32,
    pub cchContainerNameLength: u32,
    pub dwDesiredCardModuleVersion: u32,
    pub dwCspFlags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct READER_SEL_REQUEST_0_1 {
    pub cbSerialNumberOffset: u32,
    pub cbSerialNumberLength: u32,
    pub dwDesiredCardModuleVersion: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct READER_SEL_REQUEST_MATCH_TYPE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct READER_SEL_RESPONSE {
    pub cbReaderNameOffset: u32,
    pub cchReaderNameLength: u32,
    pub cbCardNameOffset: u32,
    pub cchCardNameLength: u32,
}
pub const RSR_MATCH_TYPE_ALL_CARDS: READER_SEL_REQUEST_MATCH_TYPE = READER_SEL_REQUEST_MATCH_TYPE(3i32);
pub const RSR_MATCH_TYPE_READER_AND_CONTAINER: READER_SEL_REQUEST_MATCH_TYPE = READER_SEL_REQUEST_MATCH_TYPE(1i32);
pub const RSR_MATCH_TYPE_SERIAL_NUMBER: READER_SEL_REQUEST_MATCH_TYPE = READER_SEL_REQUEST_MATCH_TYPE(2i32);
pub const SCARD_ABSENT: u32 = 1u32;
pub const SCARD_ALL_READERS: windows_core::PCWSTR = windows_core::w!("SCard$AllReaders\u{0}00");
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SCARD_ATRMASK {
    pub cbAtr: u32,
    pub rgbAtr: [u8; 36],
    pub rgbMask: [u8; 36],
}
impl Default for SCARD_ATRMASK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SCARD_ATR_LENGTH: u32 = 33u32;
pub const SCARD_AUDIT_CHV_FAILURE: u32 = 0u32;
pub const SCARD_AUDIT_CHV_SUCCESS: u32 = 1u32;
pub const SCARD_CLASS_COMMUNICATIONS: u32 = 2u32;
pub const SCARD_CLASS_ICC_STATE: u32 = 9u32;
pub const SCARD_CLASS_IFD_PROTOCOL: u32 = 8u32;
pub const SCARD_CLASS_MECHANICAL: u32 = 6u32;
pub const SCARD_CLASS_PERF: u32 = 32766u32;
pub const SCARD_CLASS_POWER_MGMT: u32 = 4u32;
pub const SCARD_CLASS_PROTOCOL: u32 = 3u32;
pub const SCARD_CLASS_SECURITY: u32 = 5u32;
pub const SCARD_CLASS_SYSTEM: u32 = 32767u32;
pub const SCARD_CLASS_VENDOR_DEFINED: u32 = 7u32;
pub const SCARD_CLASS_VENDOR_INFO: u32 = 1u32;
pub const SCARD_COLD_RESET: u32 = 1u32;
pub const SCARD_DEFAULT_READERS: windows_core::PCWSTR = windows_core::w!("SCard$DefaultReaders\u{0}00");
pub const SCARD_EJECT_CARD: u32 = 3u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SCARD_IO_REQUEST {
    pub dwProtocol: u32,
    pub cbPciLength: u32,
}
pub const SCARD_LEAVE_CARD: u32 = 0u32;
pub const SCARD_LOCAL_READERS: windows_core::PCWSTR = windows_core::w!("SCard$LocalReaders\u{0}00");
pub const SCARD_NEGOTIABLE: u32 = 5u32;
pub const SCARD_POWERED: u32 = 4u32;
pub const SCARD_POWER_DOWN: u32 = 0u32;
pub const SCARD_PRESENT: u32 = 2u32;
pub const SCARD_PROTOCOL_DEFAULT: u32 = 2147483648u32;
pub const SCARD_PROTOCOL_OPTIMAL: u32 = 0u32;
pub const SCARD_PROTOCOL_RAW: u32 = 65536u32;
pub const SCARD_PROTOCOL_T0: u32 = 1u32;
pub const SCARD_PROTOCOL_T1: u32 = 2u32;
pub const SCARD_PROTOCOL_UNDEFINED: u32 = 0u32;
pub const SCARD_PROVIDER_CSP: u32 = 2u32;
pub const SCARD_PROVIDER_KSP: u32 = 3u32;
pub const SCARD_PROVIDER_PRIMARY: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SCARD_READERSTATEA {
    pub szReader: windows_core::PCSTR,
    pub pvUserData: *mut core::ffi::c_void,
    pub dwCurrentState: SCARD_STATE,
    pub dwEventState: SCARD_STATE,
    pub cbAtr: u32,
    pub rgbAtr: [u8; 36],
}
impl Default for SCARD_READERSTATEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SCARD_READERSTATEW {
    pub szReader: windows_core::PCWSTR,
    pub pvUserData: *mut core::ffi::c_void,
    pub dwCurrentState: SCARD_STATE,
    pub dwEventState: SCARD_STATE,
    pub cbAtr: u32,
    pub rgbAtr: [u8; 36],
}
impl Default for SCARD_READERSTATEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SCARD_READER_CONFISCATES: u32 = 4u32;
pub const SCARD_READER_CONTACTLESS: u32 = 8u32;
pub const SCARD_READER_EJECTS: u32 = 2u32;
pub const SCARD_READER_SWALLOWS: u32 = 1u32;
pub const SCARD_READER_TYPE_EMBEDDEDSE: u32 = 2048u32;
pub const SCARD_READER_TYPE_IDE: u32 = 16u32;
pub const SCARD_READER_TYPE_KEYBOARD: u32 = 4u32;
pub const SCARD_READER_TYPE_NFC: u32 = 256u32;
pub const SCARD_READER_TYPE_NGC: u32 = 1024u32;
pub const SCARD_READER_TYPE_PARALELL: u32 = 2u32;
pub const SCARD_READER_TYPE_PCMCIA: u32 = 64u32;
pub const SCARD_READER_TYPE_SCSI: u32 = 8u32;
pub const SCARD_READER_TYPE_SERIAL: u32 = 1u32;
pub const SCARD_READER_TYPE_TPM: u32 = 128u32;
pub const SCARD_READER_TYPE_UICC: u32 = 512u32;
pub const SCARD_READER_TYPE_USB: u32 = 32u32;
pub const SCARD_READER_TYPE_VENDOR: u32 = 240u32;
pub const SCARD_RESET_CARD: u32 = 1u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SCARD_SCOPE(pub u32);
pub const SCARD_SCOPE_SYSTEM: SCARD_SCOPE = SCARD_SCOPE(2u32);
pub const SCARD_SCOPE_TERMINAL: u32 = 1u32;
pub const SCARD_SCOPE_USER: SCARD_SCOPE = SCARD_SCOPE(0u32);
pub const SCARD_SHARE_DIRECT: u32 = 3u32;
pub const SCARD_SHARE_EXCLUSIVE: u32 = 1u32;
pub const SCARD_SHARE_SHARED: u32 = 2u32;
pub const SCARD_SPECIFIC: u32 = 6u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SCARD_STATE(pub u32);
pub const SCARD_STATE_ATRMATCH: SCARD_STATE = SCARD_STATE(64u32);
pub const SCARD_STATE_CHANGED: SCARD_STATE = SCARD_STATE(2u32);
pub const SCARD_STATE_EMPTY: SCARD_STATE = SCARD_STATE(16u32);
pub const SCARD_STATE_EXCLUSIVE: SCARD_STATE = SCARD_STATE(128u32);
pub const SCARD_STATE_IGNORE: SCARD_STATE = SCARD_STATE(1u32);
pub const SCARD_STATE_INUSE: SCARD_STATE = SCARD_STATE(256u32);
pub const SCARD_STATE_MUTE: SCARD_STATE = SCARD_STATE(512u32);
pub const SCARD_STATE_PRESENT: SCARD_STATE = SCARD_STATE(32u32);
pub const SCARD_STATE_UNAVAILABLE: SCARD_STATE = SCARD_STATE(8u32);
pub const SCARD_STATE_UNAWARE: SCARD_STATE = SCARD_STATE(0u32);
pub const SCARD_STATE_UNKNOWN: SCARD_STATE = SCARD_STATE(4u32);
pub const SCARD_STATE_UNPOWERED: u32 = 1024u32;
pub const SCARD_SWALLOWED: u32 = 3u32;
pub const SCARD_SYSTEM_READERS: windows_core::PCWSTR = windows_core::w!("SCard$SystemReaders\u{0}00");
pub const SCARD_T0_CMD_LENGTH: u32 = 5u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SCARD_T0_COMMAND {
    pub bCla: u8,
    pub bIns: u8,
    pub bP1: u8,
    pub bP2: u8,
    pub bP3: u8,
}
pub const SCARD_T0_HEADER_LENGTH: u32 = 7u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SCARD_T0_REQUEST {
    pub ioRequest: SCARD_IO_REQUEST,
    pub bSw1: u8,
    pub bSw2: u8,
    pub Anonymous: SCARD_T0_REQUEST_0,
}
impl Default for SCARD_T0_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union SCARD_T0_REQUEST_0 {
    pub CmdBytes: SCARD_T0_COMMAND,
    pub rgbHeader: [u8; 5],
}
impl Default for SCARD_T0_REQUEST_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SCARD_T1_EPILOGUE_LENGTH: u32 = 2u32;
pub const SCARD_T1_EPILOGUE_LENGTH_LRC: u32 = 1u32;
pub const SCARD_T1_MAX_IFS: u32 = 254u32;
pub const SCARD_T1_PROLOGUE_LENGTH: u32 = 3u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SCARD_T1_REQUEST {
    pub ioRequest: SCARD_IO_REQUEST,
}
pub const SCARD_UNKNOWN: u32 = 0u32;
pub const SCARD_UNPOWER_CARD: u32 = 2u32;
pub const SCARD_WARM_RESET: u32 = 2u32;
pub const SCERR_NOCARDNAME: u32 = 16384u32;
pub const SCERR_NOGUIDS: u32 = 32768u32;
pub const SC_DLG_FORCE_UI: u32 = 4u32;
pub const SC_DLG_MINIMAL_UI: u32 = 1u32;
pub const SC_DLG_NO_UI: u32 = 2u32;
pub const SECPKG_ALT_ATTR: u32 = 2147483648u32;
pub const SECPKG_ATTR_C_FULL_IDENT_TOKEN: u32 = 2147483781u32;
pub const STATUS_ACCOUNT_DISABLED: super::super::Foundation::NTSTATUS = super::super::Foundation::NTSTATUS(0xC0000072_u32 as _);
pub const STATUS_ACCOUNT_EXPIRED: super::super::Foundation::NTSTATUS = super::super::Foundation::NTSTATUS(0xC0000193_u32 as _);
pub const STATUS_ACCOUNT_LOCKED_OUT: super::super::Foundation::NTSTATUS = super::super::Foundation::NTSTATUS(0xC0000234_u32 as _);
pub const STATUS_ACCOUNT_RESTRICTION: super::super::Foundation::NTSTATUS = super::super::Foundation::NTSTATUS(0xC000006E_u32 as _);
pub const STATUS_AUTHENTICATION_FIREWALL_FAILED: super::super::Foundation::NTSTATUS = super::super::Foundation::NTSTATUS(0xC0000413_u32 as _);
pub const STATUS_DOWNGRADE_DETECTED: super::super::Foundation::NTSTATUS = super::super::Foundation::NTSTATUS(0xC0000388_u32 as _);
pub const STATUS_LOGON_FAILURE: super::super::Foundation::NTSTATUS = super::super::Foundation::NTSTATUS(0xC000006D_u32 as _);
pub const STATUS_LOGON_TYPE_NOT_GRANTED: super::super::Foundation::NTSTATUS = super::super::Foundation::NTSTATUS(0xC000015B_u32 as _);
pub const STATUS_NO_SUCH_LOGON_SESSION: super::super::Foundation::NTSTATUS = super::super::Foundation::NTSTATUS(0xC000005F_u32 as _);
pub const STATUS_NO_SUCH_USER: super::super::Foundation::NTSTATUS = super::super::Foundation::NTSTATUS(0xC0000064_u32 as _);
pub const STATUS_PASSWORD_EXPIRED: super::super::Foundation::NTSTATUS = super::super::Foundation::NTSTATUS(0xC0000071_u32 as _);
pub const STATUS_PASSWORD_MUST_CHANGE: super::super::Foundation::NTSTATUS = super::super::Foundation::NTSTATUS(0xC0000224_u32 as _);
pub const STATUS_WRONG_PASSWORD: super::super::Foundation::NTSTATUS = super::super::Foundation::NTSTATUS(0xC000006A_u32 as _);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SecHandle {
    pub dwLower: usize,
    pub dwUpper: usize,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SecPkgContext_ClientCreds {
    pub AuthBufferLen: u32,
    pub AuthBuffer: *mut u8,
}
impl Default for SecPkgContext_ClientCreds {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const TS_SSP_NAME: windows_core::PCWSTR = windows_core::w!("TSSSP");
pub const TS_SSP_NAME_A: windows_core::PCSTR = windows_core::s!("TSSSP");
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct USERNAME_TARGET_CREDENTIAL_INFO {
    pub UserName: windows_core::PWSTR,
}
pub const UsernameForPackedCredentials: CRED_MARSHAL_TYPE = CRED_MARSHAL_TYPE(4i32);
pub const UsernameTargetCredential: CRED_MARSHAL_TYPE = CRED_MARSHAL_TYPE(2i32);
pub const szOID_TS_KP_TS_SERVER_AUTH: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.54.1.2");
