#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn GetNetScheduleAccountInformation<P0>(pwszservername: P0, wszaccount: &mut [u16]) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "mstask.dll""system" fn GetNetScheduleAccountInformation ( pwszservername : :: windows::core::PCWSTR , ccaccount : u32 , wszaccount : :: windows::core::PWSTR ) -> :: windows::core::HRESULT );
    GetNetScheduleAccountInformation(pwszservername.into_param().abi(), wszaccount.len() as _, ::core::mem::transmute(wszaccount.as_ptr())).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn I_NetLogonControl2<P0>(servername: P0, functioncode: u32, querylevel: u32, data: *const u8, buffer: *mut *mut u8) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn I_NetLogonControl2 ( servername : :: windows::core::PCWSTR , functioncode : u32 , querylevel : u32 , data : *const u8 , buffer : *mut *mut u8 ) -> u32 );
    I_NetLogonControl2(servername.into_param().abi(), functioncode, querylevel, data, buffer)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn LogErrorA(dwmessageid: u32, plpwssubstrings: &[::windows::core::PCSTR], dwerrorcode: u32) {
    ::windows::imp::link ! ( "rtutils.dll""system" fn LogErrorA ( dwmessageid : u32 , cnumberofsubstrings : u32 , plpwssubstrings : *const :: windows::core::PCSTR , dwerrorcode : u32 ) -> ( ) );
    LogErrorA(dwmessageid, plpwssubstrings.len() as _, ::core::mem::transmute(plpwssubstrings.as_ptr()), dwerrorcode)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn LogErrorW(dwmessageid: u32, plpwssubstrings: &[::windows::core::PCWSTR], dwerrorcode: u32) {
    ::windows::imp::link ! ( "rtutils.dll""system" fn LogErrorW ( dwmessageid : u32 , cnumberofsubstrings : u32 , plpwssubstrings : *const :: windows::core::PCWSTR , dwerrorcode : u32 ) -> ( ) );
    LogErrorW(dwmessageid, plpwssubstrings.len() as _, ::core::mem::transmute(plpwssubstrings.as_ptr()), dwerrorcode)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn LogEventA(weventtype: u32, dwmessageid: u32, plpwssubstrings: &[::windows::core::PCSTR]) {
    ::windows::imp::link ! ( "rtutils.dll""system" fn LogEventA ( weventtype : u32 , dwmessageid : u32 , cnumberofsubstrings : u32 , plpwssubstrings : *const :: windows::core::PCSTR ) -> ( ) );
    LogEventA(weventtype, dwmessageid, plpwssubstrings.len() as _, ::core::mem::transmute(plpwssubstrings.as_ptr()))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn LogEventW(weventtype: u32, dwmessageid: u32, plpwssubstrings: &[::windows::core::PCWSTR]) {
    ::windows::imp::link ! ( "rtutils.dll""system" fn LogEventW ( weventtype : u32 , dwmessageid : u32 , cnumberofsubstrings : u32 , plpwssubstrings : *const :: windows::core::PCWSTR ) -> ( ) );
    LogEventW(weventtype, dwmessageid, plpwssubstrings.len() as _, ::core::mem::transmute(plpwssubstrings.as_ptr()))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn MprSetupProtocolEnum(dwtransportid: u32, lplpbuffer: *mut *mut u8, lpdwentriesread: *mut u32) -> u32 {
    ::windows::imp::link ! ( "rtutils.dll""system" fn MprSetupProtocolEnum ( dwtransportid : u32 , lplpbuffer : *mut *mut u8 , lpdwentriesread : *mut u32 ) -> u32 );
    MprSetupProtocolEnum(dwtransportid, lplpbuffer, lpdwentriesread)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn MprSetupProtocolFree(lpbuffer: *mut ::core::ffi::c_void) -> u32 {
    ::windows::imp::link ! ( "rtutils.dll""system" fn MprSetupProtocolFree ( lpbuffer : *mut ::core::ffi::c_void ) -> u32 );
    MprSetupProtocolFree(lpbuffer)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetAccessAdd<P0>(servername: P0, level: u32, buf: *const u8, parm_err: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetAccessAdd ( servername : :: windows::core::PCWSTR , level : u32 , buf : *const u8 , parm_err : *mut u32 ) -> u32 );
    NetAccessAdd(servername.into_param().abi(), level, buf, ::core::mem::transmute(parm_err.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetAccessDel<P0, P1>(servername: P0, resource: P1) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetAccessDel ( servername : :: windows::core::PCWSTR , resource : :: windows::core::PCWSTR ) -> u32 );
    NetAccessDel(servername.into_param().abi(), resource.into_param().abi())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetAccessEnum<P0, P1>(servername: P0, basepath: P1, recursive: u32, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resume_handle: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetAccessEnum ( servername : :: windows::core::PCWSTR , basepath : :: windows::core::PCWSTR , recursive : u32 , level : u32 , bufptr : *mut *mut u8 , prefmaxlen : u32 , entriesread : *mut u32 , totalentries : *mut u32 , resume_handle : *mut u32 ) -> u32 );
    NetAccessEnum(servername.into_param().abi(), basepath.into_param().abi(), recursive, level, bufptr, prefmaxlen, entriesread, totalentries, ::core::mem::transmute(resume_handle.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetAccessGetInfo<P0, P1>(servername: P0, resource: P1, level: u32, bufptr: *mut *mut u8) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetAccessGetInfo ( servername : :: windows::core::PCWSTR , resource : :: windows::core::PCWSTR , level : u32 , bufptr : *mut *mut u8 ) -> u32 );
    NetAccessGetInfo(servername.into_param().abi(), resource.into_param().abi(), level, bufptr)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetAccessGetUserPerms<P0, P1, P2>(servername: P0, ugname: P1, resource: P2, perms: *mut u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetAccessGetUserPerms ( servername : :: windows::core::PCWSTR , ugname : :: windows::core::PCWSTR , resource : :: windows::core::PCWSTR , perms : *mut u32 ) -> u32 );
    NetAccessGetUserPerms(servername.into_param().abi(), ugname.into_param().abi(), resource.into_param().abi(), perms)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetAccessSetInfo<P0, P1>(servername: P0, resource: P1, level: u32, buf: *const u8, parm_err: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetAccessSetInfo ( servername : :: windows::core::PCWSTR , resource : :: windows::core::PCWSTR , level : u32 , buf : *const u8 , parm_err : *mut u32 ) -> u32 );
    NetAccessSetInfo(servername.into_param().abi(), resource.into_param().abi(), level, buf, ::core::mem::transmute(parm_err.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetAddAlternateComputerName<P0, P1, P2, P3>(server: P0, alternatename: P1, domainaccount: P2, domainaccountpassword: P3, reserved: u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P3: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetAddAlternateComputerName ( server : :: windows::core::PCWSTR , alternatename : :: windows::core::PCWSTR , domainaccount : :: windows::core::PCWSTR , domainaccountpassword : :: windows::core::PCWSTR , reserved : u32 ) -> u32 );
    NetAddAlternateComputerName(server.into_param().abi(), alternatename.into_param().abi(), domainaccount.into_param().abi(), domainaccountpassword.into_param().abi(), reserved)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetAddServiceAccount<P0, P1, P2>(servername: P0, accountname: P1, password: P2, flags: u32) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetAddServiceAccount ( servername : :: windows::core::PCWSTR , accountname : :: windows::core::PCWSTR , password : :: windows::core::PCWSTR , flags : u32 ) -> super::super::Foundation:: NTSTATUS );
    NetAddServiceAccount(servername.into_param().abi(), accountname.into_param().abi(), password.into_param().abi(), flags).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetAlertRaise<P0>(alerttype: P0, buffer: *const ::core::ffi::c_void, buffersize: u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetAlertRaise ( alerttype : :: windows::core::PCWSTR , buffer : *const ::core::ffi::c_void , buffersize : u32 ) -> u32 );
    NetAlertRaise(alerttype.into_param().abi(), buffer, buffersize)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetAlertRaiseEx<P0, P1>(alerttype: P0, variableinfo: *const ::core::ffi::c_void, variableinfosize: u32, servicename: P1) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetAlertRaiseEx ( alerttype : :: windows::core::PCWSTR , variableinfo : *const ::core::ffi::c_void , variableinfosize : u32 , servicename : :: windows::core::PCWSTR ) -> u32 );
    NetAlertRaiseEx(alerttype.into_param().abi(), variableinfo, variableinfosize, servicename.into_param().abi())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetApiBufferAllocate(bytecount: u32, buffer: *mut *mut ::core::ffi::c_void) -> u32 {
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetApiBufferAllocate ( bytecount : u32 , buffer : *mut *mut ::core::ffi::c_void ) -> u32 );
    NetApiBufferAllocate(bytecount, buffer)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetApiBufferFree(buffer: ::core::option::Option<*const ::core::ffi::c_void>) -> u32 {
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetApiBufferFree ( buffer : *const ::core::ffi::c_void ) -> u32 );
    NetApiBufferFree(::core::mem::transmute(buffer.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetApiBufferReallocate(oldbuffer: ::core::option::Option<*const ::core::ffi::c_void>, newbytecount: u32, newbuffer: *mut *mut ::core::ffi::c_void) -> u32 {
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetApiBufferReallocate ( oldbuffer : *const ::core::ffi::c_void , newbytecount : u32 , newbuffer : *mut *mut ::core::ffi::c_void ) -> u32 );
    NetApiBufferReallocate(::core::mem::transmute(oldbuffer.unwrap_or(::std::ptr::null())), newbytecount, newbuffer)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetApiBufferSize(buffer: *const ::core::ffi::c_void, bytecount: *mut u32) -> u32 {
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetApiBufferSize ( buffer : *const ::core::ffi::c_void , bytecount : *mut u32 ) -> u32 );
    NetApiBufferSize(buffer, bytecount)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetAuditClear<P0, P1, P2>(server: P0, backupfile: P1, service: P2) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetAuditClear ( server : :: windows::core::PCWSTR , backupfile : :: windows::core::PCWSTR , service : :: windows::core::PCWSTR ) -> u32 );
    NetAuditClear(server.into_param().abi(), backupfile.into_param().abi(), service.into_param().abi())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetAuditRead<P0, P1>(server: P0, service: P1, auditloghandle: *mut HLOG, offset: u32, reserved1: *mut u32, reserved2: u32, offsetflag: u32, bufptr: *mut *mut u8, prefmaxlen: u32, bytesread: *mut u32, totalavailable: *mut u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetAuditRead ( server : :: windows::core::PCWSTR , service : :: windows::core::PCWSTR , auditloghandle : *mut HLOG , offset : u32 , reserved1 : *mut u32 , reserved2 : u32 , offsetflag : u32 , bufptr : *mut *mut u8 , prefmaxlen : u32 , bytesread : *mut u32 , totalavailable : *mut u32 ) -> u32 );
    NetAuditRead(server.into_param().abi(), service.into_param().abi(), auditloghandle, offset, reserved1, reserved2, offsetflag, bufptr, prefmaxlen, bytesread, totalavailable)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetAuditWrite<P0>(r#type: u32, buf: *mut u8, numbytes: u32, service: P0, reserved: *mut u8) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetAuditWrite ( r#type : u32 , buf : *mut u8 , numbytes : u32 , service : :: windows::core::PCWSTR , reserved : *mut u8 ) -> u32 );
    NetAuditWrite(r#type, buf, numbytes, service.into_param().abi(), reserved)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetConfigGet<P0, P1, P2>(server: P0, component: P1, parameter: P2, bufptr: *mut *mut u8) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetConfigGet ( server : :: windows::core::PCWSTR , component : :: windows::core::PCWSTR , parameter : :: windows::core::PCWSTR , bufptr : *mut *mut u8 ) -> u32 );
    NetConfigGet(server.into_param().abi(), component.into_param().abi(), parameter.into_param().abi(), bufptr)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetConfigGetAll<P0, P1>(server: P0, component: P1, bufptr: *mut *mut u8) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetConfigGetAll ( server : :: windows::core::PCWSTR , component : :: windows::core::PCWSTR , bufptr : *mut *mut u8 ) -> u32 );
    NetConfigGetAll(server.into_param().abi(), component.into_param().abi(), bufptr)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetConfigSet<P0, P1, P2>(server: P0, reserved1: P1, component: P2, level: u32, reserved2: u32, buf: *mut u8, reserved3: u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetConfigSet ( server : :: windows::core::PCWSTR , reserved1 : :: windows::core::PCWSTR , component : :: windows::core::PCWSTR , level : u32 , reserved2 : u32 , buf : *mut u8 , reserved3 : u32 ) -> u32 );
    NetConfigSet(server.into_param().abi(), reserved1.into_param().abi(), component.into_param().abi(), level, reserved2, buf, reserved3)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetCreateProvisioningPackage(pprovisioningparams: *const NETSETUP_PROVISIONING_PARAMS, pppackagebindata: ::core::option::Option<*mut *mut u8>, pdwpackagebindatasize: ::core::option::Option<*mut u32>, pppackagetextdata: ::core::option::Option<*mut ::windows::core::PWSTR>) -> u32 {
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetCreateProvisioningPackage ( pprovisioningparams : *const NETSETUP_PROVISIONING_PARAMS , pppackagebindata : *mut *mut u8 , pdwpackagebindatasize : *mut u32 , pppackagetextdata : *mut :: windows::core::PWSTR ) -> u32 );
    NetCreateProvisioningPackage(pprovisioningparams, ::core::mem::transmute(pppackagebindata.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pdwpackagebindatasize.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pppackagetextdata.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetEnumerateComputerNames<P0>(server: P0, nametype: NET_COMPUTER_NAME_TYPE, reserved: u32, entrycount: *mut u32, computernames: *mut *mut ::windows::core::PWSTR) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetEnumerateComputerNames ( server : :: windows::core::PCWSTR , nametype : NET_COMPUTER_NAME_TYPE , reserved : u32 , entrycount : *mut u32 , computernames : *mut *mut :: windows::core::PWSTR ) -> u32 );
    NetEnumerateComputerNames(server.into_param().abi(), nametype, reserved, entrycount, computernames)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetEnumerateServiceAccounts<P0>(servername: P0, flags: u32, accountscount: *mut u32, accounts: *mut *mut *mut u16) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetEnumerateServiceAccounts ( servername : :: windows::core::PCWSTR , flags : u32 , accountscount : *mut u32 , accounts : *mut *mut *mut u16 ) -> super::super::Foundation:: NTSTATUS );
    NetEnumerateServiceAccounts(servername.into_param().abi(), flags, accountscount, accounts).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetErrorLogClear<P0, P1>(uncservername: P0, backupfile: P1, reserved: ::core::option::Option<*const u8>) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetErrorLogClear ( uncservername : :: windows::core::PCWSTR , backupfile : :: windows::core::PCWSTR , reserved : *const u8 ) -> u32 );
    NetErrorLogClear(uncservername.into_param().abi(), backupfile.into_param().abi(), ::core::mem::transmute(reserved.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetErrorLogRead<P0, P1>(uncservername: P0, reserved1: P1, errorloghandle: *const HLOG, offset: u32, reserved2: ::core::option::Option<*const u32>, reserved3: u32, offsetflag: u32, bufptr: *mut *mut u8, prefmaxsize: u32, bytesread: *mut u32, totalavailable: *mut u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetErrorLogRead ( uncservername : :: windows::core::PCWSTR , reserved1 : :: windows::core::PCWSTR , errorloghandle : *const HLOG , offset : u32 , reserved2 : *const u32 , reserved3 : u32 , offsetflag : u32 , bufptr : *mut *mut u8 , prefmaxsize : u32 , bytesread : *mut u32 , totalavailable : *mut u32 ) -> u32 );
    NetErrorLogRead(uncservername.into_param().abi(), reserved1.into_param().abi(), errorloghandle, offset, ::core::mem::transmute(reserved2.unwrap_or(::std::ptr::null())), reserved3, offsetflag, bufptr, prefmaxsize, bytesread, totalavailable)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetErrorLogWrite<P0>(reserved1: ::core::option::Option<*const u8>, code: u32, component: P0, buffer: *const u8, numbytes: u32, msgbuf: *const u8, strcount: u32, reserved2: ::core::option::Option<*const u8>) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetErrorLogWrite ( reserved1 : *const u8 , code : u32 , component : :: windows::core::PCWSTR , buffer : *const u8 , numbytes : u32 , msgbuf : *const u8 , strcount : u32 , reserved2 : *const u8 ) -> u32 );
    NetErrorLogWrite(::core::mem::transmute(reserved1.unwrap_or(::std::ptr::null())), code, component.into_param().abi(), buffer, numbytes, msgbuf, strcount, ::core::mem::transmute(reserved2.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[inline]
pub unsafe fn NetFreeAadJoinInformation(pjoininfo: ::core::option::Option<*const DSREG_JOIN_INFO>) {
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetFreeAadJoinInformation ( pjoininfo : *const DSREG_JOIN_INFO ) -> ( ) );
    NetFreeAadJoinInformation(::core::mem::transmute(pjoininfo.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[inline]
pub unsafe fn NetGetAadJoinInformation<P0>(pcsztenantid: P0) -> ::windows::core::Result<*mut DSREG_JOIN_INFO>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetGetAadJoinInformation ( pcsztenantid : :: windows::core::PCWSTR , ppjoininfo : *mut *mut DSREG_JOIN_INFO ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<*mut DSREG_JOIN_INFO>();
    NetGetAadJoinInformation(pcsztenantid.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetGetAnyDCName<P0, P1>(servername: P0, domainname: P1, buffer: *mut *mut u8) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetGetAnyDCName ( servername : :: windows::core::PCWSTR , domainname : :: windows::core::PCWSTR , buffer : *mut *mut u8 ) -> u32 );
    NetGetAnyDCName(servername.into_param().abi(), domainname.into_param().abi(), buffer)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetGetDCName<P0, P1>(servername: P0, domainname: P1, buffer: *mut *mut u8) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetGetDCName ( servername : :: windows::core::PCWSTR , domainname : :: windows::core::PCWSTR , buffer : *mut *mut u8 ) -> u32 );
    NetGetDCName(servername.into_param().abi(), domainname.into_param().abi(), buffer)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetGetDisplayInformationIndex<P0, P1>(servername: P0, level: u32, prefix: P1, index: *mut u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetGetDisplayInformationIndex ( servername : :: windows::core::PCWSTR , level : u32 , prefix : :: windows::core::PCWSTR , index : *mut u32 ) -> u32 );
    NetGetDisplayInformationIndex(servername.into_param().abi(), level, prefix.into_param().abi(), index)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetGetJoinInformation<P0>(lpserver: P0, lpnamebuffer: *mut ::windows::core::PWSTR, buffertype: *mut NETSETUP_JOIN_STATUS) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetGetJoinInformation ( lpserver : :: windows::core::PCWSTR , lpnamebuffer : *mut :: windows::core::PWSTR , buffertype : *mut NETSETUP_JOIN_STATUS ) -> u32 );
    NetGetJoinInformation(lpserver.into_param().abi(), lpnamebuffer, buffertype)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetGetJoinableOUs<P0, P1, P2, P3>(lpserver: P0, lpdomain: P1, lpaccount: P2, lppassword: P3, oucount: *mut u32, ous: *mut *mut ::windows::core::PWSTR) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P3: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetGetJoinableOUs ( lpserver : :: windows::core::PCWSTR , lpdomain : :: windows::core::PCWSTR , lpaccount : :: windows::core::PCWSTR , lppassword : :: windows::core::PCWSTR , oucount : *mut u32 , ous : *mut *mut :: windows::core::PWSTR ) -> u32 );
    NetGetJoinableOUs(lpserver.into_param().abi(), lpdomain.into_param().abi(), lpaccount.into_param().abi(), lppassword.into_param().abi(), oucount, ous)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetGroupAdd<P0>(servername: P0, level: u32, buf: *const u8, parm_err: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetGroupAdd ( servername : :: windows::core::PCWSTR , level : u32 , buf : *const u8 , parm_err : *mut u32 ) -> u32 );
    NetGroupAdd(servername.into_param().abi(), level, buf, ::core::mem::transmute(parm_err.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetGroupAddUser<P0, P1, P2>(servername: P0, groupname: P1, username: P2) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetGroupAddUser ( servername : :: windows::core::PCWSTR , groupname : :: windows::core::PCWSTR , username : :: windows::core::PCWSTR ) -> u32 );
    NetGroupAddUser(servername.into_param().abi(), groupname.into_param().abi(), username.into_param().abi())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetGroupDel<P0, P1>(servername: P0, groupname: P1) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetGroupDel ( servername : :: windows::core::PCWSTR , groupname : :: windows::core::PCWSTR ) -> u32 );
    NetGroupDel(servername.into_param().abi(), groupname.into_param().abi())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetGroupDelUser<P0, P1, P2>(servername: P0, groupname: P1, username: P2) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetGroupDelUser ( servername : :: windows::core::PCWSTR , groupname : :: windows::core::PCWSTR , username : :: windows::core::PCWSTR ) -> u32 );
    NetGroupDelUser(servername.into_param().abi(), groupname.into_param().abi(), username.into_param().abi())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetGroupEnum<P0>(servername: P0, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resume_handle: ::core::option::Option<*mut usize>) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetGroupEnum ( servername : :: windows::core::PCWSTR , level : u32 , bufptr : *mut *mut u8 , prefmaxlen : u32 , entriesread : *mut u32 , totalentries : *mut u32 , resume_handle : *mut usize ) -> u32 );
    NetGroupEnum(servername.into_param().abi(), level, bufptr, prefmaxlen, entriesread, totalentries, ::core::mem::transmute(resume_handle.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetGroupGetInfo<P0, P1>(servername: P0, groupname: P1, level: u32, bufptr: *mut *mut u8) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetGroupGetInfo ( servername : :: windows::core::PCWSTR , groupname : :: windows::core::PCWSTR , level : u32 , bufptr : *mut *mut u8 ) -> u32 );
    NetGroupGetInfo(servername.into_param().abi(), groupname.into_param().abi(), level, bufptr)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetGroupGetUsers<P0, P1>(servername: P0, groupname: P1, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resumehandle: ::core::option::Option<*mut usize>) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetGroupGetUsers ( servername : :: windows::core::PCWSTR , groupname : :: windows::core::PCWSTR , level : u32 , bufptr : *mut *mut u8 , prefmaxlen : u32 , entriesread : *mut u32 , totalentries : *mut u32 , resumehandle : *mut usize ) -> u32 );
    NetGroupGetUsers(servername.into_param().abi(), groupname.into_param().abi(), level, bufptr, prefmaxlen, entriesread, totalentries, ::core::mem::transmute(resumehandle.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetGroupSetInfo<P0, P1>(servername: P0, groupname: P1, level: u32, buf: *const u8, parm_err: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetGroupSetInfo ( servername : :: windows::core::PCWSTR , groupname : :: windows::core::PCWSTR , level : u32 , buf : *const u8 , parm_err : *mut u32 ) -> u32 );
    NetGroupSetInfo(servername.into_param().abi(), groupname.into_param().abi(), level, buf, ::core::mem::transmute(parm_err.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetGroupSetUsers<P0, P1>(servername: P0, groupname: P1, level: u32, buf: *const u8, totalentries: u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetGroupSetUsers ( servername : :: windows::core::PCWSTR , groupname : :: windows::core::PCWSTR , level : u32 , buf : *const u8 , totalentries : u32 ) -> u32 );
    NetGroupSetUsers(servername.into_param().abi(), groupname.into_param().abi(), level, buf, totalentries)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetIsServiceAccount<P0, P1>(servername: P0, accountname: P1, isservice: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetIsServiceAccount ( servername : :: windows::core::PCWSTR , accountname : :: windows::core::PCWSTR , isservice : *mut super::super::Foundation:: BOOL ) -> super::super::Foundation:: NTSTATUS );
    NetIsServiceAccount(servername.into_param().abi(), accountname.into_param().abi(), isservice).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetJoinDomain<P0, P1, P2, P3, P4>(lpserver: P0, lpdomain: P1, lpmachineaccountou: P2, lpaccount: P3, lppassword: P4, fjoinoptions: NET_JOIN_DOMAIN_JOIN_OPTIONS) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P3: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P4: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetJoinDomain ( lpserver : :: windows::core::PCWSTR , lpdomain : :: windows::core::PCWSTR , lpmachineaccountou : :: windows::core::PCWSTR , lpaccount : :: windows::core::PCWSTR , lppassword : :: windows::core::PCWSTR , fjoinoptions : NET_JOIN_DOMAIN_JOIN_OPTIONS ) -> u32 );
    NetJoinDomain(lpserver.into_param().abi(), lpdomain.into_param().abi(), lpmachineaccountou.into_param().abi(), lpaccount.into_param().abi(), lppassword.into_param().abi(), fjoinoptions)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetLocalGroupAdd<P0>(servername: P0, level: u32, buf: *const u8, parm_err: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetLocalGroupAdd ( servername : :: windows::core::PCWSTR , level : u32 , buf : *const u8 , parm_err : *mut u32 ) -> u32 );
    NetLocalGroupAdd(servername.into_param().abi(), level, buf, ::core::mem::transmute(parm_err.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetLocalGroupAddMember<P0, P1, P2>(servername: P0, groupname: P1, membersid: P2) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<super::super::Foundation::PSID>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetLocalGroupAddMember ( servername : :: windows::core::PCWSTR , groupname : :: windows::core::PCWSTR , membersid : super::super::Foundation:: PSID ) -> u32 );
    NetLocalGroupAddMember(servername.into_param().abi(), groupname.into_param().abi(), membersid.into_param().abi())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetLocalGroupAddMembers<P0, P1>(servername: P0, groupname: P1, level: u32, buf: *const u8, totalentries: u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetLocalGroupAddMembers ( servername : :: windows::core::PCWSTR , groupname : :: windows::core::PCWSTR , level : u32 , buf : *const u8 , totalentries : u32 ) -> u32 );
    NetLocalGroupAddMembers(servername.into_param().abi(), groupname.into_param().abi(), level, buf, totalentries)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetLocalGroupDel<P0, P1>(servername: P0, groupname: P1) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetLocalGroupDel ( servername : :: windows::core::PCWSTR , groupname : :: windows::core::PCWSTR ) -> u32 );
    NetLocalGroupDel(servername.into_param().abi(), groupname.into_param().abi())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetLocalGroupDelMember<P0, P1, P2>(servername: P0, groupname: P1, membersid: P2) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<super::super::Foundation::PSID>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetLocalGroupDelMember ( servername : :: windows::core::PCWSTR , groupname : :: windows::core::PCWSTR , membersid : super::super::Foundation:: PSID ) -> u32 );
    NetLocalGroupDelMember(servername.into_param().abi(), groupname.into_param().abi(), membersid.into_param().abi())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetLocalGroupDelMembers<P0, P1>(servername: P0, groupname: P1, level: u32, buf: *const u8, totalentries: u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetLocalGroupDelMembers ( servername : :: windows::core::PCWSTR , groupname : :: windows::core::PCWSTR , level : u32 , buf : *const u8 , totalentries : u32 ) -> u32 );
    NetLocalGroupDelMembers(servername.into_param().abi(), groupname.into_param().abi(), level, buf, totalentries)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetLocalGroupEnum<P0>(servername: P0, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resumehandle: ::core::option::Option<*mut usize>) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetLocalGroupEnum ( servername : :: windows::core::PCWSTR , level : u32 , bufptr : *mut *mut u8 , prefmaxlen : u32 , entriesread : *mut u32 , totalentries : *mut u32 , resumehandle : *mut usize ) -> u32 );
    NetLocalGroupEnum(servername.into_param().abi(), level, bufptr, prefmaxlen, entriesread, totalentries, ::core::mem::transmute(resumehandle.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetLocalGroupGetInfo<P0, P1>(servername: P0, groupname: P1, level: u32, bufptr: *mut *mut u8) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetLocalGroupGetInfo ( servername : :: windows::core::PCWSTR , groupname : :: windows::core::PCWSTR , level : u32 , bufptr : *mut *mut u8 ) -> u32 );
    NetLocalGroupGetInfo(servername.into_param().abi(), groupname.into_param().abi(), level, bufptr)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetLocalGroupGetMembers<P0, P1>(servername: P0, localgroupname: P1, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resumehandle: ::core::option::Option<*mut usize>) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetLocalGroupGetMembers ( servername : :: windows::core::PCWSTR , localgroupname : :: windows::core::PCWSTR , level : u32 , bufptr : *mut *mut u8 , prefmaxlen : u32 , entriesread : *mut u32 , totalentries : *mut u32 , resumehandle : *mut usize ) -> u32 );
    NetLocalGroupGetMembers(servername.into_param().abi(), localgroupname.into_param().abi(), level, bufptr, prefmaxlen, entriesread, totalentries, ::core::mem::transmute(resumehandle.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetLocalGroupSetInfo<P0, P1>(servername: P0, groupname: P1, level: u32, buf: *const u8, parm_err: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetLocalGroupSetInfo ( servername : :: windows::core::PCWSTR , groupname : :: windows::core::PCWSTR , level : u32 , buf : *const u8 , parm_err : *mut u32 ) -> u32 );
    NetLocalGroupSetInfo(servername.into_param().abi(), groupname.into_param().abi(), level, buf, ::core::mem::transmute(parm_err.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetLocalGroupSetMembers<P0, P1>(servername: P0, groupname: P1, level: u32, buf: *const u8, totalentries: u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetLocalGroupSetMembers ( servername : :: windows::core::PCWSTR , groupname : :: windows::core::PCWSTR , level : u32 , buf : *const u8 , totalentries : u32 ) -> u32 );
    NetLocalGroupSetMembers(servername.into_param().abi(), groupname.into_param().abi(), level, buf, totalentries)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetMessageBufferSend<P0, P1, P2>(servername: P0, msgname: P1, fromname: P2, buf: *const u8, buflen: u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetMessageBufferSend ( servername : :: windows::core::PCWSTR , msgname : :: windows::core::PCWSTR , fromname : :: windows::core::PCWSTR , buf : *const u8 , buflen : u32 ) -> u32 );
    NetMessageBufferSend(servername.into_param().abi(), msgname.into_param().abi(), fromname.into_param().abi(), buf, buflen)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetMessageNameAdd<P0, P1>(servername: P0, msgname: P1) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetMessageNameAdd ( servername : :: windows::core::PCWSTR , msgname : :: windows::core::PCWSTR ) -> u32 );
    NetMessageNameAdd(servername.into_param().abi(), msgname.into_param().abi())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetMessageNameDel<P0, P1>(servername: P0, msgname: P1) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetMessageNameDel ( servername : :: windows::core::PCWSTR , msgname : :: windows::core::PCWSTR ) -> u32 );
    NetMessageNameDel(servername.into_param().abi(), msgname.into_param().abi())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetMessageNameEnum<P0>(servername: P0, level: u32, bufptr: *const *const u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resume_handle: *mut u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetMessageNameEnum ( servername : :: windows::core::PCWSTR , level : u32 , bufptr : *const *const u8 , prefmaxlen : u32 , entriesread : *mut u32 , totalentries : *mut u32 , resume_handle : *mut u32 ) -> u32 );
    NetMessageNameEnum(servername.into_param().abi(), level, bufptr, prefmaxlen, entriesread, totalentries, resume_handle)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetMessageNameGetInfo<P0, P1>(servername: P0, msgname: P1, level: u32, bufptr: *const *const u8) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetMessageNameGetInfo ( servername : :: windows::core::PCWSTR , msgname : :: windows::core::PCWSTR , level : u32 , bufptr : *const *const u8 ) -> u32 );
    NetMessageNameGetInfo(servername.into_param().abi(), msgname.into_param().abi(), level, bufptr)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetProvisionComputerAccount<P0, P1, P2, P3>(lpdomain: P0, lpmachinename: P1, lpmachineaccountou: P2, lpdcname: P3, dwoptions: NETSETUP_PROVISION, pprovisionbindata: ::core::option::Option<*mut *mut u8>, pdwprovisionbindatasize: ::core::option::Option<*mut u32>, pprovisiontextdata: ::core::option::Option<*mut ::windows::core::PWSTR>) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P3: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetProvisionComputerAccount ( lpdomain : :: windows::core::PCWSTR , lpmachinename : :: windows::core::PCWSTR , lpmachineaccountou : :: windows::core::PCWSTR , lpdcname : :: windows::core::PCWSTR , dwoptions : NETSETUP_PROVISION , pprovisionbindata : *mut *mut u8 , pdwprovisionbindatasize : *mut u32 , pprovisiontextdata : *mut :: windows::core::PWSTR ) -> u32 );
    NetProvisionComputerAccount(lpdomain.into_param().abi(), lpmachinename.into_param().abi(), lpmachineaccountou.into_param().abi(), lpdcname.into_param().abi(), dwoptions, ::core::mem::transmute(pprovisionbindata.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pdwprovisionbindatasize.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pprovisiontextdata.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetQueryDisplayInformation<P0>(servername: P0, level: u32, index: u32, entriesrequested: u32, preferredmaximumlength: u32, returnedentrycount: *mut u32, sortedbuffer: *mut *mut ::core::ffi::c_void) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetQueryDisplayInformation ( servername : :: windows::core::PCWSTR , level : u32 , index : u32 , entriesrequested : u32 , preferredmaximumlength : u32 , returnedentrycount : *mut u32 , sortedbuffer : *mut *mut ::core::ffi::c_void ) -> u32 );
    NetQueryDisplayInformation(servername.into_param().abi(), level, index, entriesrequested, preferredmaximumlength, returnedentrycount, sortedbuffer)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetQueryServiceAccount<P0, P1>(servername: P0, accountname: P1, infolevel: u32, buffer: *mut *mut u8) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetQueryServiceAccount ( servername : :: windows::core::PCWSTR , accountname : :: windows::core::PCWSTR , infolevel : u32 , buffer : *mut *mut u8 ) -> super::super::Foundation:: NTSTATUS );
    NetQueryServiceAccount(servername.into_param().abi(), accountname.into_param().abi(), infolevel, buffer).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetRemoteComputerSupports<P0>(uncservername: P0, optionswanted: NET_REMOTE_COMPUTER_SUPPORTS_OPTIONS, optionssupported: *mut u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetRemoteComputerSupports ( uncservername : :: windows::core::PCWSTR , optionswanted : NET_REMOTE_COMPUTER_SUPPORTS_OPTIONS , optionssupported : *mut u32 ) -> u32 );
    NetRemoteComputerSupports(uncservername.into_param().abi(), optionswanted, optionssupported)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetRemoteTOD<P0>(uncservername: P0, bufferptr: *mut *mut u8) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetRemoteTOD ( uncservername : :: windows::core::PCWSTR , bufferptr : *mut *mut u8 ) -> u32 );
    NetRemoteTOD(uncservername.into_param().abi(), bufferptr)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetRemoveAlternateComputerName<P0, P1, P2, P3>(server: P0, alternatename: P1, domainaccount: P2, domainaccountpassword: P3, reserved: u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P3: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetRemoveAlternateComputerName ( server : :: windows::core::PCWSTR , alternatename : :: windows::core::PCWSTR , domainaccount : :: windows::core::PCWSTR , domainaccountpassword : :: windows::core::PCWSTR , reserved : u32 ) -> u32 );
    NetRemoveAlternateComputerName(server.into_param().abi(), alternatename.into_param().abi(), domainaccount.into_param().abi(), domainaccountpassword.into_param().abi(), reserved)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetRemoveServiceAccount<P0, P1>(servername: P0, accountname: P1, flags: u32) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetRemoveServiceAccount ( servername : :: windows::core::PCWSTR , accountname : :: windows::core::PCWSTR , flags : u32 ) -> super::super::Foundation:: NTSTATUS );
    NetRemoveServiceAccount(servername.into_param().abi(), accountname.into_param().abi(), flags).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetRenameMachineInDomain<P0, P1, P2, P3>(lpserver: P0, lpnewmachinename: P1, lpaccount: P2, lppassword: P3, frenameoptions: u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P3: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetRenameMachineInDomain ( lpserver : :: windows::core::PCWSTR , lpnewmachinename : :: windows::core::PCWSTR , lpaccount : :: windows::core::PCWSTR , lppassword : :: windows::core::PCWSTR , frenameoptions : u32 ) -> u32 );
    NetRenameMachineInDomain(lpserver.into_param().abi(), lpnewmachinename.into_param().abi(), lpaccount.into_param().abi(), lppassword.into_param().abi(), frenameoptions)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetReplExportDirAdd<P0>(servername: P0, level: u32, buf: *const u8, parm_err: *mut u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetReplExportDirAdd ( servername : :: windows::core::PCWSTR , level : u32 , buf : *const u8 , parm_err : *mut u32 ) -> u32 );
    NetReplExportDirAdd(servername.into_param().abi(), level, buf, parm_err)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetReplExportDirDel<P0, P1>(servername: P0, dirname: P1) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetReplExportDirDel ( servername : :: windows::core::PCWSTR , dirname : :: windows::core::PCWSTR ) -> u32 );
    NetReplExportDirDel(servername.into_param().abi(), dirname.into_param().abi())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetReplExportDirEnum<P0>(servername: P0, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resumehandle: *mut u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetReplExportDirEnum ( servername : :: windows::core::PCWSTR , level : u32 , bufptr : *mut *mut u8 , prefmaxlen : u32 , entriesread : *mut u32 , totalentries : *mut u32 , resumehandle : *mut u32 ) -> u32 );
    NetReplExportDirEnum(servername.into_param().abi(), level, bufptr, prefmaxlen, entriesread, totalentries, resumehandle)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetReplExportDirGetInfo<P0, P1>(servername: P0, dirname: P1, level: u32, bufptr: *mut *mut u8) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetReplExportDirGetInfo ( servername : :: windows::core::PCWSTR , dirname : :: windows::core::PCWSTR , level : u32 , bufptr : *mut *mut u8 ) -> u32 );
    NetReplExportDirGetInfo(servername.into_param().abi(), dirname.into_param().abi(), level, bufptr)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetReplExportDirLock<P0, P1>(servername: P0, dirname: P1) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetReplExportDirLock ( servername : :: windows::core::PCWSTR , dirname : :: windows::core::PCWSTR ) -> u32 );
    NetReplExportDirLock(servername.into_param().abi(), dirname.into_param().abi())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetReplExportDirSetInfo<P0, P1>(servername: P0, dirname: P1, level: u32, buf: *const u8, parm_err: *mut u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetReplExportDirSetInfo ( servername : :: windows::core::PCWSTR , dirname : :: windows::core::PCWSTR , level : u32 , buf : *const u8 , parm_err : *mut u32 ) -> u32 );
    NetReplExportDirSetInfo(servername.into_param().abi(), dirname.into_param().abi(), level, buf, parm_err)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetReplExportDirUnlock<P0, P1>(servername: P0, dirname: P1, unlockforce: u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetReplExportDirUnlock ( servername : :: windows::core::PCWSTR , dirname : :: windows::core::PCWSTR , unlockforce : u32 ) -> u32 );
    NetReplExportDirUnlock(servername.into_param().abi(), dirname.into_param().abi(), unlockforce)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetReplGetInfo<P0>(servername: P0, level: u32, bufptr: *mut *mut u8) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetReplGetInfo ( servername : :: windows::core::PCWSTR , level : u32 , bufptr : *mut *mut u8 ) -> u32 );
    NetReplGetInfo(servername.into_param().abi(), level, bufptr)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetReplImportDirAdd<P0>(servername: P0, level: u32, buf: *const u8, parm_err: *mut u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetReplImportDirAdd ( servername : :: windows::core::PCWSTR , level : u32 , buf : *const u8 , parm_err : *mut u32 ) -> u32 );
    NetReplImportDirAdd(servername.into_param().abi(), level, buf, parm_err)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetReplImportDirDel<P0, P1>(servername: P0, dirname: P1) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetReplImportDirDel ( servername : :: windows::core::PCWSTR , dirname : :: windows::core::PCWSTR ) -> u32 );
    NetReplImportDirDel(servername.into_param().abi(), dirname.into_param().abi())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetReplImportDirEnum<P0>(servername: P0, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resumehandle: *mut u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetReplImportDirEnum ( servername : :: windows::core::PCWSTR , level : u32 , bufptr : *mut *mut u8 , prefmaxlen : u32 , entriesread : *mut u32 , totalentries : *mut u32 , resumehandle : *mut u32 ) -> u32 );
    NetReplImportDirEnum(servername.into_param().abi(), level, bufptr, prefmaxlen, entriesread, totalentries, resumehandle)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetReplImportDirGetInfo<P0, P1>(servername: P0, dirname: P1, level: u32, bufptr: *mut *mut u8) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetReplImportDirGetInfo ( servername : :: windows::core::PCWSTR , dirname : :: windows::core::PCWSTR , level : u32 , bufptr : *mut *mut u8 ) -> u32 );
    NetReplImportDirGetInfo(servername.into_param().abi(), dirname.into_param().abi(), level, bufptr)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetReplImportDirLock<P0, P1>(servername: P0, dirname: P1) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetReplImportDirLock ( servername : :: windows::core::PCWSTR , dirname : :: windows::core::PCWSTR ) -> u32 );
    NetReplImportDirLock(servername.into_param().abi(), dirname.into_param().abi())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetReplImportDirUnlock<P0, P1>(servername: P0, dirname: P1, unlockforce: u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetReplImportDirUnlock ( servername : :: windows::core::PCWSTR , dirname : :: windows::core::PCWSTR , unlockforce : u32 ) -> u32 );
    NetReplImportDirUnlock(servername.into_param().abi(), dirname.into_param().abi(), unlockforce)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetReplSetInfo<P0>(servername: P0, level: u32, buf: *const u8, parm_err: *mut u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetReplSetInfo ( servername : :: windows::core::PCWSTR , level : u32 , buf : *const u8 , parm_err : *mut u32 ) -> u32 );
    NetReplSetInfo(servername.into_param().abi(), level, buf, parm_err)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetRequestOfflineDomainJoin<P0>(pprovisionbindata: &[u8], dwoptions: NET_REQUEST_PROVISION_OPTIONS, lpwindowspath: P0) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetRequestOfflineDomainJoin ( pprovisionbindata : *const u8 , cbprovisionbindatasize : u32 , dwoptions : NET_REQUEST_PROVISION_OPTIONS , lpwindowspath : :: windows::core::PCWSTR ) -> u32 );
    NetRequestOfflineDomainJoin(::core::mem::transmute(pprovisionbindata.as_ptr()), pprovisionbindata.len() as _, dwoptions, lpwindowspath.into_param().abi())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetRequestProvisioningPackageInstall<P0>(ppackagebindata: &[u8], dwprovisionoptions: NET_REQUEST_PROVISION_OPTIONS, lpwindowspath: P0, pvreserved: ::core::option::Option<*const ::core::ffi::c_void>) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetRequestProvisioningPackageInstall ( ppackagebindata : *const u8 , dwpackagebindatasize : u32 , dwprovisionoptions : NET_REQUEST_PROVISION_OPTIONS , lpwindowspath : :: windows::core::PCWSTR , pvreserved : *const ::core::ffi::c_void ) -> u32 );
    NetRequestProvisioningPackageInstall(::core::mem::transmute(ppackagebindata.as_ptr()), ppackagebindata.len() as _, dwprovisionoptions, lpwindowspath.into_param().abi(), ::core::mem::transmute(pvreserved.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetScheduleJobAdd<P0>(servername: P0, buffer: *mut u8, jobid: *mut u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetScheduleJobAdd ( servername : :: windows::core::PCWSTR , buffer : *mut u8 , jobid : *mut u32 ) -> u32 );
    NetScheduleJobAdd(servername.into_param().abi(), buffer, jobid)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetScheduleJobDel<P0>(servername: P0, minjobid: u32, maxjobid: u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetScheduleJobDel ( servername : :: windows::core::PCWSTR , minjobid : u32 , maxjobid : u32 ) -> u32 );
    NetScheduleJobDel(servername.into_param().abi(), minjobid, maxjobid)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetScheduleJobEnum<P0>(servername: P0, pointertobuffer: *mut *mut u8, prefferedmaximumlength: u32, entriesread: *mut u32, totalentries: *mut u32, resumehandle: *mut u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetScheduleJobEnum ( servername : :: windows::core::PCWSTR , pointertobuffer : *mut *mut u8 , prefferedmaximumlength : u32 , entriesread : *mut u32 , totalentries : *mut u32 , resumehandle : *mut u32 ) -> u32 );
    NetScheduleJobEnum(servername.into_param().abi(), pointertobuffer, prefferedmaximumlength, entriesread, totalentries, resumehandle)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetScheduleJobGetInfo<P0>(servername: P0, jobid: u32, pointertobuffer: *mut *mut u8) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetScheduleJobGetInfo ( servername : :: windows::core::PCWSTR , jobid : u32 , pointertobuffer : *mut *mut u8 ) -> u32 );
    NetScheduleJobGetInfo(servername.into_param().abi(), jobid, pointertobuffer)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetServerComputerNameAdd<P0, P1, P2>(servername: P0, emulateddomainname: P1, emulatedservername: P2) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetServerComputerNameAdd ( servername : :: windows::core::PCWSTR , emulateddomainname : :: windows::core::PCWSTR , emulatedservername : :: windows::core::PCWSTR ) -> u32 );
    NetServerComputerNameAdd(servername.into_param().abi(), emulateddomainname.into_param().abi(), emulatedservername.into_param().abi())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetServerComputerNameDel<P0, P1>(servername: P0, emulatedservername: P1) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetServerComputerNameDel ( servername : :: windows::core::PCWSTR , emulatedservername : :: windows::core::PCWSTR ) -> u32 );
    NetServerComputerNameDel(servername.into_param().abi(), emulatedservername.into_param().abi())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetServerDiskEnum<P0>(servername: P0, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resume_handle: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetServerDiskEnum ( servername : :: windows::core::PCWSTR , level : u32 , bufptr : *mut *mut u8 , prefmaxlen : u32 , entriesread : *mut u32 , totalentries : *mut u32 , resume_handle : *mut u32 ) -> u32 );
    NetServerDiskEnum(servername.into_param().abi(), level, bufptr, prefmaxlen, entriesread, totalentries, ::core::mem::transmute(resume_handle.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetServerEnum<P0, P1>(servername: P0, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, servertype: NET_SERVER_TYPE, domain: P1, resume_handle: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetServerEnum ( servername : :: windows::core::PCWSTR , level : u32 , bufptr : *mut *mut u8 , prefmaxlen : u32 , entriesread : *mut u32 , totalentries : *mut u32 , servertype : NET_SERVER_TYPE , domain : :: windows::core::PCWSTR , resume_handle : *mut u32 ) -> u32 );
    NetServerEnum(servername.into_param().abi(), level, bufptr, prefmaxlen, entriesread, totalentries, servertype, domain.into_param().abi(), ::core::mem::transmute(resume_handle.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetServerGetInfo<P0>(servername: P0, level: u32, bufptr: *mut *mut u8) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetServerGetInfo ( servername : :: windows::core::PCWSTR , level : u32 , bufptr : *mut *mut u8 ) -> u32 );
    NetServerGetInfo(servername.into_param().abi(), level, bufptr)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetServerSetInfo<P0>(servername: P0, level: u32, buf: *const u8, parmerror: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetServerSetInfo ( servername : :: windows::core::PCWSTR , level : u32 , buf : *const u8 , parmerror : *mut u32 ) -> u32 );
    NetServerSetInfo(servername.into_param().abi(), level, buf, ::core::mem::transmute(parmerror.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetServerTransportAdd<P0>(servername: P0, level: u32, bufptr: *const u8) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetServerTransportAdd ( servername : :: windows::core::PCWSTR , level : u32 , bufptr : *const u8 ) -> u32 );
    NetServerTransportAdd(servername.into_param().abi(), level, bufptr)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetServerTransportAddEx<P0>(servername: P0, level: u32, bufptr: *const u8) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetServerTransportAddEx ( servername : :: windows::core::PCWSTR , level : u32 , bufptr : *const u8 ) -> u32 );
    NetServerTransportAddEx(servername.into_param().abi(), level, bufptr)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetServerTransportDel<P0>(servername: P0, level: u32, bufptr: *const u8) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetServerTransportDel ( servername : :: windows::core::PCWSTR , level : u32 , bufptr : *const u8 ) -> u32 );
    NetServerTransportDel(servername.into_param().abi(), level, bufptr)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetServerTransportEnum<P0>(servername: P0, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resume_handle: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetServerTransportEnum ( servername : :: windows::core::PCWSTR , level : u32 , bufptr : *mut *mut u8 , prefmaxlen : u32 , entriesread : *mut u32 , totalentries : *mut u32 , resume_handle : *mut u32 ) -> u32 );
    NetServerTransportEnum(servername.into_param().abi(), level, bufptr, prefmaxlen, entriesread, totalentries, ::core::mem::transmute(resume_handle.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetServiceControl<P0, P1>(servername: P0, service: P1, opcode: u32, arg: u32, bufptr: *mut *mut u8) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetServiceControl ( servername : :: windows::core::PCWSTR , service : :: windows::core::PCWSTR , opcode : u32 , arg : u32 , bufptr : *mut *mut u8 ) -> u32 );
    NetServiceControl(servername.into_param().abi(), service.into_param().abi(), opcode, arg, bufptr)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetServiceEnum<P0>(servername: P0, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resume_handle: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetServiceEnum ( servername : :: windows::core::PCWSTR , level : u32 , bufptr : *mut *mut u8 , prefmaxlen : u32 , entriesread : *mut u32 , totalentries : *mut u32 , resume_handle : *mut u32 ) -> u32 );
    NetServiceEnum(servername.into_param().abi(), level, bufptr, prefmaxlen, entriesread, totalentries, ::core::mem::transmute(resume_handle.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetServiceGetInfo<P0, P1>(servername: P0, service: P1, level: u32, bufptr: *mut *mut u8) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetServiceGetInfo ( servername : :: windows::core::PCWSTR , service : :: windows::core::PCWSTR , level : u32 , bufptr : *mut *mut u8 ) -> u32 );
    NetServiceGetInfo(servername.into_param().abi(), service.into_param().abi(), level, bufptr)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetServiceInstall<P0, P1>(servername: P0, service: P1, argv: &[::windows::core::PCWSTR], bufptr: *mut *mut u8) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetServiceInstall ( servername : :: windows::core::PCWSTR , service : :: windows::core::PCWSTR , argc : u32 , argv : *const :: windows::core::PCWSTR , bufptr : *mut *mut u8 ) -> u32 );
    NetServiceInstall(servername.into_param().abi(), service.into_param().abi(), argv.len() as _, ::core::mem::transmute(argv.as_ptr()), bufptr)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetSetPrimaryComputerName<P0, P1, P2, P3>(server: P0, primaryname: P1, domainaccount: P2, domainaccountpassword: P3, reserved: u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P3: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetSetPrimaryComputerName ( server : :: windows::core::PCWSTR , primaryname : :: windows::core::PCWSTR , domainaccount : :: windows::core::PCWSTR , domainaccountpassword : :: windows::core::PCWSTR , reserved : u32 ) -> u32 );
    NetSetPrimaryComputerName(server.into_param().abi(), primaryname.into_param().abi(), domainaccount.into_param().abi(), domainaccountpassword.into_param().abi(), reserved)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetUnjoinDomain<P0, P1, P2>(lpserver: P0, lpaccount: P1, lppassword: P2, funjoinoptions: u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetUnjoinDomain ( lpserver : :: windows::core::PCWSTR , lpaccount : :: windows::core::PCWSTR , lppassword : :: windows::core::PCWSTR , funjoinoptions : u32 ) -> u32 );
    NetUnjoinDomain(lpserver.into_param().abi(), lpaccount.into_param().abi(), lppassword.into_param().abi(), funjoinoptions)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetUseAdd(servername: ::core::option::Option<*const i8>, levelflags: u32, buf: *const u8, parm_err: ::core::option::Option<*mut u32>) -> u32 {
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetUseAdd ( servername : *const i8 , levelflags : u32 , buf : *const u8 , parm_err : *mut u32 ) -> u32 );
    NetUseAdd(::core::mem::transmute(servername.unwrap_or(::std::ptr::null())), levelflags, buf, ::core::mem::transmute(parm_err.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetUseDel<P0, P1>(uncservername: P0, usename: P1, forcelevelflags: FORCE_LEVEL_FLAGS) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetUseDel ( uncservername : :: windows::core::PCWSTR , usename : :: windows::core::PCWSTR , forcelevelflags : FORCE_LEVEL_FLAGS ) -> u32 );
    NetUseDel(uncservername.into_param().abi(), usename.into_param().abi(), forcelevelflags)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetUseEnum<P0>(uncservername: P0, levelflags: u32, bufptr: ::core::option::Option<*mut *mut u8>, preferedmaximumsize: u32, entriesread: ::core::option::Option<*mut u32>, totalentries: *mut u32, resumehandle: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetUseEnum ( uncservername : :: windows::core::PCWSTR , levelflags : u32 , bufptr : *mut *mut u8 , preferedmaximumsize : u32 , entriesread : *mut u32 , totalentries : *mut u32 , resumehandle : *mut u32 ) -> u32 );
    NetUseEnum(uncservername.into_param().abi(), levelflags, ::core::mem::transmute(bufptr.unwrap_or(::std::ptr::null_mut())), preferedmaximumsize, ::core::mem::transmute(entriesread.unwrap_or(::std::ptr::null_mut())), totalentries, ::core::mem::transmute(resumehandle.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetUseGetInfo<P0, P1>(uncservername: P0, usename: P1, levelflags: u32, bufptr: ::core::option::Option<*mut *mut u8>) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetUseGetInfo ( uncservername : :: windows::core::PCWSTR , usename : :: windows::core::PCWSTR , levelflags : u32 , bufptr : *mut *mut u8 ) -> u32 );
    NetUseGetInfo(uncservername.into_param().abi(), usename.into_param().abi(), levelflags, ::core::mem::transmute(bufptr.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetUserAdd<P0>(servername: P0, level: u32, buf: *const u8, parm_err: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetUserAdd ( servername : :: windows::core::PCWSTR , level : u32 , buf : *const u8 , parm_err : *mut u32 ) -> u32 );
    NetUserAdd(servername.into_param().abi(), level, buf, ::core::mem::transmute(parm_err.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetUserChangePassword<P0, P1, P2, P3>(domainname: P0, username: P1, oldpassword: P2, newpassword: P3) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P3: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetUserChangePassword ( domainname : :: windows::core::PCWSTR , username : :: windows::core::PCWSTR , oldpassword : :: windows::core::PCWSTR , newpassword : :: windows::core::PCWSTR ) -> u32 );
    NetUserChangePassword(domainname.into_param().abi(), username.into_param().abi(), oldpassword.into_param().abi(), newpassword.into_param().abi())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetUserDel<P0, P1>(servername: P0, username: P1) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetUserDel ( servername : :: windows::core::PCWSTR , username : :: windows::core::PCWSTR ) -> u32 );
    NetUserDel(servername.into_param().abi(), username.into_param().abi())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetUserEnum<P0>(servername: P0, level: u32, filter: NET_USER_ENUM_FILTER_FLAGS, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resume_handle: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetUserEnum ( servername : :: windows::core::PCWSTR , level : u32 , filter : NET_USER_ENUM_FILTER_FLAGS , bufptr : *mut *mut u8 , prefmaxlen : u32 , entriesread : *mut u32 , totalentries : *mut u32 , resume_handle : *mut u32 ) -> u32 );
    NetUserEnum(servername.into_param().abi(), level, filter, bufptr, prefmaxlen, entriesread, totalentries, ::core::mem::transmute(resume_handle.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetUserGetGroups<P0, P1>(servername: P0, username: P1, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetUserGetGroups ( servername : :: windows::core::PCWSTR , username : :: windows::core::PCWSTR , level : u32 , bufptr : *mut *mut u8 , prefmaxlen : u32 , entriesread : *mut u32 , totalentries : *mut u32 ) -> u32 );
    NetUserGetGroups(servername.into_param().abi(), username.into_param().abi(), level, bufptr, prefmaxlen, entriesread, totalentries)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetUserGetInfo<P0, P1>(servername: P0, username: P1, level: u32, bufptr: *mut *mut u8) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetUserGetInfo ( servername : :: windows::core::PCWSTR , username : :: windows::core::PCWSTR , level : u32 , bufptr : *mut *mut u8 ) -> u32 );
    NetUserGetInfo(servername.into_param().abi(), username.into_param().abi(), level, bufptr)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetUserGetLocalGroups<P0, P1>(servername: P0, username: P1, level: u32, flags: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetUserGetLocalGroups ( servername : :: windows::core::PCWSTR , username : :: windows::core::PCWSTR , level : u32 , flags : u32 , bufptr : *mut *mut u8 , prefmaxlen : u32 , entriesread : *mut u32 , totalentries : *mut u32 ) -> u32 );
    NetUserGetLocalGroups(servername.into_param().abi(), username.into_param().abi(), level, flags, bufptr, prefmaxlen, entriesread, totalentries)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetUserModalsGet<P0>(servername: P0, level: u32, bufptr: *mut *mut u8) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetUserModalsGet ( servername : :: windows::core::PCWSTR , level : u32 , bufptr : *mut *mut u8 ) -> u32 );
    NetUserModalsGet(servername.into_param().abi(), level, bufptr)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetUserModalsSet<P0>(servername: P0, level: u32, buf: *const u8, parm_err: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetUserModalsSet ( servername : :: windows::core::PCWSTR , level : u32 , buf : *const u8 , parm_err : *mut u32 ) -> u32 );
    NetUserModalsSet(servername.into_param().abi(), level, buf, ::core::mem::transmute(parm_err.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetUserSetGroups<P0, P1>(servername: P0, username: P1, level: u32, buf: *const u8, num_entries: u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetUserSetGroups ( servername : :: windows::core::PCWSTR , username : :: windows::core::PCWSTR , level : u32 , buf : *const u8 , num_entries : u32 ) -> u32 );
    NetUserSetGroups(servername.into_param().abi(), username.into_param().abi(), level, buf, num_entries)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetUserSetInfo<P0, P1>(servername: P0, username: P1, level: u32, buf: *const u8, parm_err: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetUserSetInfo ( servername : :: windows::core::PCWSTR , username : :: windows::core::PCWSTR , level : u32 , buf : *const u8 , parm_err : *mut u32 ) -> u32 );
    NetUserSetInfo(servername.into_param().abi(), username.into_param().abi(), level, buf, ::core::mem::transmute(parm_err.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetValidateName<P0, P1, P2, P3>(lpserver: P0, lpname: P1, lpaccount: P2, lppassword: P3, nametype: NETSETUP_NAME_TYPE) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P3: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetValidateName ( lpserver : :: windows::core::PCWSTR , lpname : :: windows::core::PCWSTR , lpaccount : :: windows::core::PCWSTR , lppassword : :: windows::core::PCWSTR , nametype : NETSETUP_NAME_TYPE ) -> u32 );
    NetValidateName(lpserver.into_param().abi(), lpname.into_param().abi(), lpaccount.into_param().abi(), lppassword.into_param().abi(), nametype)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetValidatePasswordPolicy<P0>(servername: P0, qualifier: *mut ::core::ffi::c_void, validationtype: NET_VALIDATE_PASSWORD_TYPE, inputarg: *mut ::core::ffi::c_void, outputarg: *mut *mut ::core::ffi::c_void) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetValidatePasswordPolicy ( servername : :: windows::core::PCWSTR , qualifier : *mut ::core::ffi::c_void , validationtype : NET_VALIDATE_PASSWORD_TYPE , inputarg : *mut ::core::ffi::c_void , outputarg : *mut *mut ::core::ffi::c_void ) -> u32 );
    NetValidatePasswordPolicy(servername.into_param().abi(), qualifier, validationtype, inputarg, outputarg)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetValidatePasswordPolicyFree(outputarg: *mut *mut ::core::ffi::c_void) -> u32 {
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetValidatePasswordPolicyFree ( outputarg : *mut *mut ::core::ffi::c_void ) -> u32 );
    NetValidatePasswordPolicyFree(outputarg)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetWkstaGetInfo<P0>(servername: P0, level: u32, bufptr: ::core::option::Option<*mut *mut u8>) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetWkstaGetInfo ( servername : :: windows::core::PCWSTR , level : u32 , bufptr : *mut *mut u8 ) -> u32 );
    NetWkstaGetInfo(servername.into_param().abi(), level, ::core::mem::transmute(bufptr.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetWkstaSetInfo<P0>(servername: P0, level: u32, buffer: *const u8, parm_err: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetWkstaSetInfo ( servername : :: windows::core::PCWSTR , level : u32 , buffer : *const u8 , parm_err : *mut u32 ) -> u32 );
    NetWkstaSetInfo(servername.into_param().abi(), level, buffer, ::core::mem::transmute(parm_err.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetWkstaTransportAdd(servername: ::core::option::Option<*const i8>, level: u32, buf: *const u8, parm_err: ::core::option::Option<*mut u32>) -> u32 {
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetWkstaTransportAdd ( servername : *const i8 , level : u32 , buf : *const u8 , parm_err : *mut u32 ) -> u32 );
    NetWkstaTransportAdd(::core::mem::transmute(servername.unwrap_or(::std::ptr::null())), level, buf, ::core::mem::transmute(parm_err.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetWkstaTransportDel<P0, P1>(servername: P0, transportname: P1, ucond: FORCE_LEVEL_FLAGS) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetWkstaTransportDel ( servername : :: windows::core::PCWSTR , transportname : :: windows::core::PCWSTR , ucond : FORCE_LEVEL_FLAGS ) -> u32 );
    NetWkstaTransportDel(servername.into_param().abi(), transportname.into_param().abi(), ucond)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetWkstaTransportEnum(servername: ::core::option::Option<*const i8>, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resume_handle: ::core::option::Option<*mut u32>) -> u32 {
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetWkstaTransportEnum ( servername : *const i8 , level : u32 , bufptr : *mut *mut u8 , prefmaxlen : u32 , entriesread : *mut u32 , totalentries : *mut u32 , resume_handle : *mut u32 ) -> u32 );
    NetWkstaTransportEnum(::core::mem::transmute(servername.unwrap_or(::std::ptr::null())), level, bufptr, prefmaxlen, entriesread, totalentries, ::core::mem::transmute(resume_handle.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetWkstaUserEnum<P0>(servername: P0, level: u32, bufptr: ::core::option::Option<*mut *mut u8>, prefmaxlen: u32, entriesread: ::core::option::Option<*mut u32>, totalentries: *mut u32, resumehandle: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetWkstaUserEnum ( servername : :: windows::core::PCWSTR , level : u32 , bufptr : *mut *mut u8 , prefmaxlen : u32 , entriesread : *mut u32 , totalentries : *mut u32 , resumehandle : *mut u32 ) -> u32 );
    NetWkstaUserEnum(servername.into_param().abi(), level, ::core::mem::transmute(bufptr.unwrap_or(::std::ptr::null_mut())), prefmaxlen, ::core::mem::transmute(entriesread.unwrap_or(::std::ptr::null_mut())), totalentries, ::core::mem::transmute(resumehandle.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetWkstaUserGetInfo<P0>(reserved: P0, level: u32, bufptr: *mut *mut u8) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetWkstaUserGetInfo ( reserved : :: windows::core::PCWSTR , level : u32 , bufptr : *mut *mut u8 ) -> u32 );
    NetWkstaUserGetInfo(reserved.into_param().abi(), level, bufptr)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn NetWkstaUserSetInfo<P0>(reserved: P0, level: u32, buf: *const u8, parm_err: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "netapi32.dll""system" fn NetWkstaUserSetInfo ( reserved : :: windows::core::PCWSTR , level : u32 , buf : *const u8 , parm_err : *mut u32 ) -> u32 );
    NetWkstaUserSetInfo(reserved.into_param().abi(), level, buf, ::core::mem::transmute(parm_err.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn RouterAssert<P0, P1, P2>(pszfailedassertion: P0, pszfilename: P1, dwlinenumber: u32, pszmessage: P2)
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "rtutils.dll""system" fn RouterAssert ( pszfailedassertion : :: windows::core::PCSTR , pszfilename : :: windows::core::PCSTR , dwlinenumber : u32 , pszmessage : :: windows::core::PCSTR ) -> ( ) );
    RouterAssert(pszfailedassertion.into_param().abi(), pszfilename.into_param().abi(), dwlinenumber, pszmessage.into_param().abi())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn RouterGetErrorStringA(dwerrorcode: u32, lplpszerrorstring: *mut ::windows::core::PSTR) -> u32 {
    ::windows::imp::link ! ( "rtutils.dll""system" fn RouterGetErrorStringA ( dwerrorcode : u32 , lplpszerrorstring : *mut :: windows::core::PSTR ) -> u32 );
    RouterGetErrorStringA(dwerrorcode, lplpszerrorstring)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn RouterGetErrorStringW(dwerrorcode: u32, lplpwszerrorstring: *mut ::windows::core::PWSTR) -> u32 {
    ::windows::imp::link ! ( "rtutils.dll""system" fn RouterGetErrorStringW ( dwerrorcode : u32 , lplpwszerrorstring : *mut :: windows::core::PWSTR ) -> u32 );
    RouterGetErrorStringW(dwerrorcode, lplpwszerrorstring)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RouterLogDeregisterA<P0>(hloghandle: P0)
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "rtutils.dll""system" fn RouterLogDeregisterA ( hloghandle : super::super::Foundation:: HANDLE ) -> ( ) );
    RouterLogDeregisterA(hloghandle.into_param().abi())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RouterLogDeregisterW<P0>(hloghandle: P0)
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "rtutils.dll""system" fn RouterLogDeregisterW ( hloghandle : super::super::Foundation:: HANDLE ) -> ( ) );
    RouterLogDeregisterW(hloghandle.into_param().abi())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RouterLogEventA<P0>(hloghandle: P0, dweventtype: u32, dwmessageid: u32, plpszsubstringarray: ::core::option::Option<&[::windows::core::PCSTR]>, dwerrorcode: u32)
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "rtutils.dll""system" fn RouterLogEventA ( hloghandle : super::super::Foundation:: HANDLE , dweventtype : u32 , dwmessageid : u32 , dwsubstringcount : u32 , plpszsubstringarray : *const :: windows::core::PCSTR , dwerrorcode : u32 ) -> ( ) );
    RouterLogEventA(hloghandle.into_param().abi(), dweventtype, dwmessageid, plpszsubstringarray.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(plpszsubstringarray.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), dwerrorcode)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RouterLogEventDataA<P0>(hloghandle: P0, dweventtype: u32, dwmessageid: u32, plpszsubstringarray: ::core::option::Option<&[::windows::core::PCSTR]>, dwdatabytes: u32, lpdatabytes: *mut u8)
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "rtutils.dll""system" fn RouterLogEventDataA ( hloghandle : super::super::Foundation:: HANDLE , dweventtype : u32 , dwmessageid : u32 , dwsubstringcount : u32 , plpszsubstringarray : *const :: windows::core::PCSTR , dwdatabytes : u32 , lpdatabytes : *mut u8 ) -> ( ) );
    RouterLogEventDataA(hloghandle.into_param().abi(), dweventtype, dwmessageid, plpszsubstringarray.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(plpszsubstringarray.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), dwdatabytes, lpdatabytes)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RouterLogEventDataW<P0>(hloghandle: P0, dweventtype: u32, dwmessageid: u32, plpszsubstringarray: ::core::option::Option<&[::windows::core::PCWSTR]>, dwdatabytes: u32, lpdatabytes: *mut u8)
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "rtutils.dll""system" fn RouterLogEventDataW ( hloghandle : super::super::Foundation:: HANDLE , dweventtype : u32 , dwmessageid : u32 , dwsubstringcount : u32 , plpszsubstringarray : *const :: windows::core::PCWSTR , dwdatabytes : u32 , lpdatabytes : *mut u8 ) -> ( ) );
    RouterLogEventDataW(hloghandle.into_param().abi(), dweventtype, dwmessageid, plpszsubstringarray.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(plpszsubstringarray.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), dwdatabytes, lpdatabytes)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RouterLogEventExA<P0, P1>(hloghandle: P0, dweventtype: u32, dwerrorcode: u32, dwmessageid: u32, ptszformat: P1)
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "rtutils.dll""cdecl" fn RouterLogEventExA ( hloghandle : super::super::Foundation:: HANDLE , dweventtype : u32 , dwerrorcode : u32 , dwmessageid : u32 , ptszformat : :: windows::core::PCSTR ) -> ( ) );
    RouterLogEventExA(hloghandle.into_param().abi(), dweventtype, dwerrorcode, dwmessageid, ptszformat.into_param().abi())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RouterLogEventExW<P0, P1>(hloghandle: P0, dweventtype: u32, dwerrorcode: u32, dwmessageid: u32, ptszformat: P1)
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "rtutils.dll""cdecl" fn RouterLogEventExW ( hloghandle : super::super::Foundation:: HANDLE , dweventtype : u32 , dwerrorcode : u32 , dwmessageid : u32 , ptszformat : :: windows::core::PCWSTR ) -> ( ) );
    RouterLogEventExW(hloghandle.into_param().abi(), dweventtype, dwerrorcode, dwmessageid, ptszformat.into_param().abi())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RouterLogEventStringA<P0>(hloghandle: P0, dweventtype: u32, dwmessageid: u32, plpszsubstringarray: &[::windows::core::PCSTR], dwerrorcode: u32, dwerrorindex: u32)
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "rtutils.dll""system" fn RouterLogEventStringA ( hloghandle : super::super::Foundation:: HANDLE , dweventtype : u32 , dwmessageid : u32 , dwsubstringcount : u32 , plpszsubstringarray : *const :: windows::core::PCSTR , dwerrorcode : u32 , dwerrorindex : u32 ) -> ( ) );
    RouterLogEventStringA(hloghandle.into_param().abi(), dweventtype, dwmessageid, plpszsubstringarray.len() as _, ::core::mem::transmute(plpszsubstringarray.as_ptr()), dwerrorcode, dwerrorindex)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RouterLogEventStringW<P0>(hloghandle: P0, dweventtype: u32, dwmessageid: u32, plpszsubstringarray: &[::windows::core::PCWSTR], dwerrorcode: u32, dwerrorindex: u32)
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "rtutils.dll""system" fn RouterLogEventStringW ( hloghandle : super::super::Foundation:: HANDLE , dweventtype : u32 , dwmessageid : u32 , dwsubstringcount : u32 , plpszsubstringarray : *const :: windows::core::PCWSTR , dwerrorcode : u32 , dwerrorindex : u32 ) -> ( ) );
    RouterLogEventStringW(hloghandle.into_param().abi(), dweventtype, dwmessageid, plpszsubstringarray.len() as _, ::core::mem::transmute(plpszsubstringarray.as_ptr()), dwerrorcode, dwerrorindex)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RouterLogEventValistExA<P0, P1>(hloghandle: P0, dweventtype: u32, dwerrorcode: u32, dwmessageid: u32, ptszformat: P1, arglist: *mut i8)
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "rtutils.dll""system" fn RouterLogEventValistExA ( hloghandle : super::super::Foundation:: HANDLE , dweventtype : u32 , dwerrorcode : u32 , dwmessageid : u32 , ptszformat : :: windows::core::PCSTR , arglist : *mut i8 ) -> ( ) );
    RouterLogEventValistExA(hloghandle.into_param().abi(), dweventtype, dwerrorcode, dwmessageid, ptszformat.into_param().abi(), arglist)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RouterLogEventValistExW<P0, P1>(hloghandle: P0, dweventtype: u32, dwerrorcode: u32, dwmessageid: u32, ptszformat: P1, arglist: *mut i8)
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "rtutils.dll""system" fn RouterLogEventValistExW ( hloghandle : super::super::Foundation:: HANDLE , dweventtype : u32 , dwerrorcode : u32 , dwmessageid : u32 , ptszformat : :: windows::core::PCWSTR , arglist : *mut i8 ) -> ( ) );
    RouterLogEventValistExW(hloghandle.into_param().abi(), dweventtype, dwerrorcode, dwmessageid, ptszformat.into_param().abi(), arglist)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RouterLogEventW<P0>(hloghandle: P0, dweventtype: u32, dwmessageid: u32, plpszsubstringarray: ::core::option::Option<&[::windows::core::PCWSTR]>, dwerrorcode: u32)
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "rtutils.dll""system" fn RouterLogEventW ( hloghandle : super::super::Foundation:: HANDLE , dweventtype : u32 , dwmessageid : u32 , dwsubstringcount : u32 , plpszsubstringarray : *const :: windows::core::PCWSTR , dwerrorcode : u32 ) -> ( ) );
    RouterLogEventW(hloghandle.into_param().abi(), dweventtype, dwmessageid, plpszsubstringarray.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(plpszsubstringarray.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), dwerrorcode)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RouterLogRegisterA<P0>(lpszsource: P0) -> super::super::Foundation::HANDLE
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "rtutils.dll""system" fn RouterLogRegisterA ( lpszsource : :: windows::core::PCSTR ) -> super::super::Foundation:: HANDLE );
    RouterLogRegisterA(lpszsource.into_param().abi())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RouterLogRegisterW<P0>(lpszsource: P0) -> super::super::Foundation::HANDLE
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "rtutils.dll""system" fn RouterLogRegisterW ( lpszsource : :: windows::core::PCWSTR ) -> super::super::Foundation:: HANDLE );
    RouterLogRegisterW(lpszsource.into_param().abi())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn SetNetScheduleAccountInformation<P0, P1, P2>(pwszservername: P0, pwszaccount: P1, pwszpassword: P2) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "mstask.dll""system" fn SetNetScheduleAccountInformation ( pwszservername : :: windows::core::PCWSTR , pwszaccount : :: windows::core::PCWSTR , pwszpassword : :: windows::core::PCWSTR ) -> :: windows::core::HRESULT );
    SetNetScheduleAccountInformation(pwszservername.into_param().abi(), pwszaccount.into_param().abi(), pwszpassword.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn TraceDeregisterA(dwtraceid: u32) -> u32 {
    ::windows::imp::link ! ( "rtutils.dll""system" fn TraceDeregisterA ( dwtraceid : u32 ) -> u32 );
    TraceDeregisterA(dwtraceid)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn TraceDeregisterExA(dwtraceid: u32, dwflags: u32) -> u32 {
    ::windows::imp::link ! ( "rtutils.dll""system" fn TraceDeregisterExA ( dwtraceid : u32 , dwflags : u32 ) -> u32 );
    TraceDeregisterExA(dwtraceid, dwflags)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn TraceDeregisterExW(dwtraceid: u32, dwflags: u32) -> u32 {
    ::windows::imp::link ! ( "rtutils.dll""system" fn TraceDeregisterExW ( dwtraceid : u32 , dwflags : u32 ) -> u32 );
    TraceDeregisterExW(dwtraceid, dwflags)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn TraceDeregisterW(dwtraceid: u32) -> u32 {
    ::windows::imp::link ! ( "rtutils.dll""system" fn TraceDeregisterW ( dwtraceid : u32 ) -> u32 );
    TraceDeregisterW(dwtraceid)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TraceDumpExA<P0, P1>(dwtraceid: u32, dwflags: u32, lpbbytes: *mut u8, dwbytecount: u32, dwgroupsize: u32, baddressprefix: P0, lpszprefix: P1) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "rtutils.dll""system" fn TraceDumpExA ( dwtraceid : u32 , dwflags : u32 , lpbbytes : *mut u8 , dwbytecount : u32 , dwgroupsize : u32 , baddressprefix : super::super::Foundation:: BOOL , lpszprefix : :: windows::core::PCSTR ) -> u32 );
    TraceDumpExA(dwtraceid, dwflags, lpbbytes, dwbytecount, dwgroupsize, baddressprefix.into_param().abi(), lpszprefix.into_param().abi())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TraceDumpExW<P0, P1>(dwtraceid: u32, dwflags: u32, lpbbytes: *mut u8, dwbytecount: u32, dwgroupsize: u32, baddressprefix: P0, lpszprefix: P1) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "rtutils.dll""system" fn TraceDumpExW ( dwtraceid : u32 , dwflags : u32 , lpbbytes : *mut u8 , dwbytecount : u32 , dwgroupsize : u32 , baddressprefix : super::super::Foundation:: BOOL , lpszprefix : :: windows::core::PCWSTR ) -> u32 );
    TraceDumpExW(dwtraceid, dwflags, lpbbytes, dwbytecount, dwgroupsize, baddressprefix.into_param().abi(), lpszprefix.into_param().abi())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TraceGetConsoleA(dwtraceid: u32, lphconsole: *mut super::super::Foundation::HANDLE) -> u32 {
    ::windows::imp::link ! ( "rtutils.dll""system" fn TraceGetConsoleA ( dwtraceid : u32 , lphconsole : *mut super::super::Foundation:: HANDLE ) -> u32 );
    TraceGetConsoleA(dwtraceid, lphconsole)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TraceGetConsoleW(dwtraceid: u32, lphconsole: *mut super::super::Foundation::HANDLE) -> u32 {
    ::windows::imp::link ! ( "rtutils.dll""system" fn TraceGetConsoleW ( dwtraceid : u32 , lphconsole : *mut super::super::Foundation:: HANDLE ) -> u32 );
    TraceGetConsoleW(dwtraceid, lphconsole)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn TracePrintfA<P0>(dwtraceid: u32, lpszformat: P0) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "rtutils.dll""cdecl" fn TracePrintfA ( dwtraceid : u32 , lpszformat : :: windows::core::PCSTR ) -> u32 );
    TracePrintfA(dwtraceid, lpszformat.into_param().abi())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn TracePrintfExA<P0>(dwtraceid: u32, dwflags: u32, lpszformat: P0) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "rtutils.dll""cdecl" fn TracePrintfExA ( dwtraceid : u32 , dwflags : u32 , lpszformat : :: windows::core::PCSTR ) -> u32 );
    TracePrintfExA(dwtraceid, dwflags, lpszformat.into_param().abi())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn TracePrintfExW<P0>(dwtraceid: u32, dwflags: u32, lpszformat: P0) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "rtutils.dll""cdecl" fn TracePrintfExW ( dwtraceid : u32 , dwflags : u32 , lpszformat : :: windows::core::PCWSTR ) -> u32 );
    TracePrintfExW(dwtraceid, dwflags, lpszformat.into_param().abi())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn TracePrintfW<P0>(dwtraceid: u32, lpszformat: P0) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "rtutils.dll""cdecl" fn TracePrintfW ( dwtraceid : u32 , lpszformat : :: windows::core::PCWSTR ) -> u32 );
    TracePrintfW(dwtraceid, lpszformat.into_param().abi())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn TracePutsExA<P0>(dwtraceid: u32, dwflags: u32, lpszstring: P0) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "rtutils.dll""system" fn TracePutsExA ( dwtraceid : u32 , dwflags : u32 , lpszstring : :: windows::core::PCSTR ) -> u32 );
    TracePutsExA(dwtraceid, dwflags, lpszstring.into_param().abi())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn TracePutsExW<P0>(dwtraceid: u32, dwflags: u32, lpszstring: P0) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "rtutils.dll""system" fn TracePutsExW ( dwtraceid : u32 , dwflags : u32 , lpszstring : :: windows::core::PCWSTR ) -> u32 );
    TracePutsExW(dwtraceid, dwflags, lpszstring.into_param().abi())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn TraceRegisterExA<P0>(lpszcallername: P0, dwflags: u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "rtutils.dll""system" fn TraceRegisterExA ( lpszcallername : :: windows::core::PCSTR , dwflags : u32 ) -> u32 );
    TraceRegisterExA(lpszcallername.into_param().abi(), dwflags)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn TraceRegisterExW<P0>(lpszcallername: P0, dwflags: u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "rtutils.dll""system" fn TraceRegisterExW ( lpszcallername : :: windows::core::PCWSTR , dwflags : u32 ) -> u32 );
    TraceRegisterExW(lpszcallername.into_param().abi(), dwflags)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn TraceVprintfExA<P0>(dwtraceid: u32, dwflags: u32, lpszformat: P0, arglist: *mut i8) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "rtutils.dll""system" fn TraceVprintfExA ( dwtraceid : u32 , dwflags : u32 , lpszformat : :: windows::core::PCSTR , arglist : *mut i8 ) -> u32 );
    TraceVprintfExA(dwtraceid, dwflags, lpszformat.into_param().abi(), arglist)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[inline]
pub unsafe fn TraceVprintfExW<P0>(dwtraceid: u32, dwflags: u32, lpszformat: P0, arglist: *mut i8) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "rtutils.dll""system" fn TraceVprintfExW ( dwtraceid : u32 , dwflags : u32 , lpszformat : :: windows::core::PCWSTR , arglist : *mut i8 ) -> u32 );
    TraceVprintfExW(dwtraceid, dwflags, lpszformat.into_param().abi(), arglist)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[repr(transparent)]
pub struct IEnumNetCfgBindingInterface(::windows::core::IUnknown);
impl IEnumNetCfgBindingInterface {
    pub unsafe fn Next(&self, rgelt: &mut [::core::option::Option<INetCfgBindingInterface>], pceltfetched: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), rgelt.len() as _, ::core::mem::transmute(rgelt.as_ptr()), ::core::mem::transmute(pceltfetched.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self, ppenum: ::core::option::Option<*const ::core::option::Option<IEnumNetCfgBindingInterface>>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppenum.unwrap_or(::std::ptr::null()))).ok()
    }
}
::windows::imp::interface_hierarchy!(IEnumNetCfgBindingInterface, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IEnumNetCfgBindingInterface {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumNetCfgBindingInterface {}
impl ::core::fmt::Debug for IEnumNetCfgBindingInterface {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumNetCfgBindingInterface").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumNetCfgBindingInterface {
    type Vtable = IEnumNetCfgBindingInterface_Vtbl;
}
impl ::core::clone::Clone for IEnumNetCfgBindingInterface {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEnumNetCfgBindingInterface {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc0e8ae90_306e_11d1_aacf_00805fc1270e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumNetCfgBindingInterface_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *const *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[repr(transparent)]
pub struct IEnumNetCfgBindingPath(::windows::core::IUnknown);
impl IEnumNetCfgBindingPath {
    pub unsafe fn Next(&self, rgelt: &mut [::core::option::Option<INetCfgBindingPath>], pceltfetched: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), rgelt.len() as _, ::core::mem::transmute(rgelt.as_ptr()), ::core::mem::transmute(pceltfetched.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self, ppenum: ::core::option::Option<*const ::core::option::Option<IEnumNetCfgBindingPath>>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppenum.unwrap_or(::std::ptr::null()))).ok()
    }
}
::windows::imp::interface_hierarchy!(IEnumNetCfgBindingPath, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IEnumNetCfgBindingPath {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumNetCfgBindingPath {}
impl ::core::fmt::Debug for IEnumNetCfgBindingPath {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumNetCfgBindingPath").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumNetCfgBindingPath {
    type Vtable = IEnumNetCfgBindingPath_Vtbl;
}
impl ::core::clone::Clone for IEnumNetCfgBindingPath {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEnumNetCfgBindingPath {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc0e8ae91_306e_11d1_aacf_00805fc1270e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumNetCfgBindingPath_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *const *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[repr(transparent)]
pub struct IEnumNetCfgComponent(::windows::core::IUnknown);
impl IEnumNetCfgComponent {
    pub unsafe fn Next(&self, rgelt: &mut [::core::option::Option<INetCfgComponent>], pceltfetched: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), rgelt.len() as _, ::core::mem::transmute(rgelt.as_ptr()), ::core::mem::transmute(pceltfetched.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self, ppenum: ::core::option::Option<*const ::core::option::Option<IEnumNetCfgComponent>>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppenum.unwrap_or(::std::ptr::null()))).ok()
    }
}
::windows::imp::interface_hierarchy!(IEnumNetCfgComponent, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IEnumNetCfgComponent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumNetCfgComponent {}
impl ::core::fmt::Debug for IEnumNetCfgComponent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumNetCfgComponent").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumNetCfgComponent {
    type Vtable = IEnumNetCfgComponent_Vtbl;
}
impl ::core::clone::Clone for IEnumNetCfgComponent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEnumNetCfgComponent {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc0e8ae92_306e_11d1_aacf_00805fc1270e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumNetCfgComponent_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *const *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[repr(transparent)]
pub struct INetCfg(::windows::core::IUnknown);
impl INetCfg {
    pub unsafe fn Initialize(&self, pvreserved: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Initialize)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pvreserved.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn Uninitialize(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Uninitialize)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Apply(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Apply)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Cancel)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn EnumComponents(&self, pguidclass: *const ::windows::core::GUID, ppenumcomponent: ::core::option::Option<*mut ::core::option::Option<IEnumNetCfgComponent>>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnumComponents)(::windows::core::Interface::as_raw(self), pguidclass, ::core::mem::transmute(ppenumcomponent.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn FindComponent<P0>(&self, pszwinfid: P0, pcomponent: ::core::option::Option<*mut ::core::option::Option<INetCfgComponent>>) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).FindComponent)(::windows::core::Interface::as_raw(self), pszwinfid.into_param().abi(), ::core::mem::transmute(pcomponent.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn QueryNetCfgClass(&self, pguidclass: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppvobject: ::core::option::Option<*mut *mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).QueryNetCfgClass)(::windows::core::Interface::as_raw(self), pguidclass, riid, ::core::mem::transmute(ppvobject.unwrap_or(::std::ptr::null_mut()))).ok()
    }
}
::windows::imp::interface_hierarchy!(INetCfg, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for INetCfg {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetCfg {}
impl ::core::fmt::Debug for INetCfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetCfg").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for INetCfg {
    type Vtable = INetCfg_Vtbl;
}
impl ::core::clone::Clone for INetCfg {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for INetCfg {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc0e8ae93_306e_11d1_aacf_00805fc1270e);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetCfg_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvreserved: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Uninitialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Apply: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EnumComponents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidclass: *const ::windows::core::GUID, ppenumcomponent: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FindComponent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszwinfid: ::windows::core::PCWSTR, pcomponent: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub QueryNetCfgClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidclass: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[repr(transparent)]
pub struct INetCfgBindingInterface(::windows::core::IUnknown);
impl INetCfgBindingInterface {
    pub unsafe fn GetName(&self, ppszwinterfacename: ::core::option::Option<*mut ::windows::core::PWSTR>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppszwinterfacename.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetUpperComponent(&self, ppnccitem: ::core::option::Option<*mut ::core::option::Option<INetCfgComponent>>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetUpperComponent)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppnccitem.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetLowerComponent(&self, ppnccitem: ::core::option::Option<*mut ::core::option::Option<INetCfgComponent>>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetLowerComponent)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppnccitem.unwrap_or(::std::ptr::null_mut()))).ok()
    }
}
::windows::imp::interface_hierarchy!(INetCfgBindingInterface, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for INetCfgBindingInterface {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetCfgBindingInterface {}
impl ::core::fmt::Debug for INetCfgBindingInterface {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetCfgBindingInterface").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for INetCfgBindingInterface {
    type Vtable = INetCfgBindingInterface_Vtbl;
}
impl ::core::clone::Clone for INetCfgBindingInterface {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for INetCfgBindingInterface {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc0e8ae94_306e_11d1_aacf_00805fc1270e);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetCfgBindingInterface_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszwinterfacename: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetUpperComponent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppnccitem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetLowerComponent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppnccitem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[repr(transparent)]
pub struct INetCfgBindingPath(::windows::core::IUnknown);
impl INetCfgBindingPath {
    pub unsafe fn IsSamePathAs<P0>(&self, ppath: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<INetCfgBindingPath>,
    {
        (::windows::core::Interface::vtable(self).IsSamePathAs)(::windows::core::Interface::as_raw(self), ppath.into_param().abi()).ok()
    }
    pub unsafe fn IsSubPathOf<P0>(&self, ppath: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<INetCfgBindingPath>,
    {
        (::windows::core::Interface::vtable(self).IsSubPathOf)(::windows::core::Interface::as_raw(self), ppath.into_param().abi()).ok()
    }
    pub unsafe fn IsEnabled(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).IsEnabled)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Enable<P0>(&self, fenable: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).Enable)(::windows::core::Interface::as_raw(self), fenable.into_param().abi()).ok()
    }
    pub unsafe fn GetPathToken(&self, ppszwpathtoken: ::core::option::Option<*mut ::windows::core::PWSTR>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPathToken)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppszwpathtoken.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetOwner(&self, ppcomponent: ::core::option::Option<*mut ::core::option::Option<INetCfgComponent>>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetOwner)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppcomponent.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetDepth(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetDepth)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumBindingInterfaces(&self, ppenuminterface: ::core::option::Option<*mut ::core::option::Option<IEnumNetCfgBindingInterface>>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnumBindingInterfaces)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppenuminterface.unwrap_or(::std::ptr::null_mut()))).ok()
    }
}
::windows::imp::interface_hierarchy!(INetCfgBindingPath, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for INetCfgBindingPath {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetCfgBindingPath {}
impl ::core::fmt::Debug for INetCfgBindingPath {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetCfgBindingPath").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for INetCfgBindingPath {
    type Vtable = INetCfgBindingPath_Vtbl;
}
impl ::core::clone::Clone for INetCfgBindingPath {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for INetCfgBindingPath {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc0e8ae96_306e_11d1_aacf_00805fc1270e);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetCfgBindingPath_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub IsSamePathAs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppath: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IsSubPathOf: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppath: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Enable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fenable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Enable: usize,
    pub GetPathToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszwpathtoken: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetOwner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcomponent: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDepth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcinterfaces: *mut u32) -> ::windows::core::HRESULT,
    pub EnumBindingInterfaces: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenuminterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[repr(transparent)]
pub struct INetCfgClass(::windows::core::IUnknown);
impl INetCfgClass {
    pub unsafe fn FindComponent<P0>(&self, pszwinfid: P0, ppnccitem: ::core::option::Option<*mut ::core::option::Option<INetCfgComponent>>) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).FindComponent)(::windows::core::Interface::as_raw(self), pszwinfid.into_param().abi(), ::core::mem::transmute(ppnccitem.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn EnumComponents(&self, ppenumcomponent: ::core::option::Option<*mut ::core::option::Option<IEnumNetCfgComponent>>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnumComponents)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppenumcomponent.unwrap_or(::std::ptr::null_mut()))).ok()
    }
}
::windows::imp::interface_hierarchy!(INetCfgClass, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for INetCfgClass {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetCfgClass {}
impl ::core::fmt::Debug for INetCfgClass {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetCfgClass").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for INetCfgClass {
    type Vtable = INetCfgClass_Vtbl;
}
impl ::core::clone::Clone for INetCfgClass {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for INetCfgClass {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc0e8ae97_306e_11d1_aacf_00805fc1270e);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetCfgClass_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub FindComponent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszwinfid: ::windows::core::PCWSTR, ppnccitem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EnumComponents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumcomponent: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[repr(transparent)]
pub struct INetCfgClassSetup(::windows::core::IUnknown);
impl INetCfgClassSetup {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SelectAndInstall<P0>(&self, hwndparent: P0, pobotoken: ::core::option::Option<*const OBO_TOKEN>, ppnccitem: ::core::option::Option<*mut ::core::option::Option<INetCfgComponent>>) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    {
        (::windows::core::Interface::vtable(self).SelectAndInstall)(::windows::core::Interface::as_raw(self), hwndparent.into_param().abi(), ::core::mem::transmute(pobotoken.unwrap_or(::std::ptr::null())), ::core::mem::transmute(ppnccitem.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Install<P0, P1, P2>(&self, pszwinfid: P0, pobotoken: ::core::option::Option<*const OBO_TOKEN>, dwsetupflags: u32, dwupgradefrombuildno: u32, pszwanswerfile: P1, pszwanswersections: P2, ppnccitem: ::core::option::Option<*mut ::core::option::Option<INetCfgComponent>>) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).Install)(::windows::core::Interface::as_raw(self), pszwinfid.into_param().abi(), ::core::mem::transmute(pobotoken.unwrap_or(::std::ptr::null())), dwsetupflags, dwupgradefrombuildno, pszwanswerfile.into_param().abi(), pszwanswersections.into_param().abi(), ::core::mem::transmute(ppnccitem.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeInstall<P0>(&self, pcomponent: P0, pobotoken: ::core::option::Option<*const OBO_TOKEN>, pmszwrefs: ::core::option::Option<*mut ::windows::core::PWSTR>) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<INetCfgComponent>,
    {
        (::windows::core::Interface::vtable(self).DeInstall)(::windows::core::Interface::as_raw(self), pcomponent.into_param().abi(), ::core::mem::transmute(pobotoken.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pmszwrefs.unwrap_or(::std::ptr::null_mut()))).ok()
    }
}
::windows::imp::interface_hierarchy!(INetCfgClassSetup, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for INetCfgClassSetup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetCfgClassSetup {}
impl ::core::fmt::Debug for INetCfgClassSetup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetCfgClassSetup").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for INetCfgClassSetup {
    type Vtable = INetCfgClassSetup_Vtbl;
}
impl ::core::clone::Clone for INetCfgClassSetup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for INetCfgClassSetup {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc0e8ae9d_306e_11d1_aacf_00805fc1270e);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetCfgClassSetup_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub SelectAndInstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, pobotoken: *const OBO_TOKEN, ppnccitem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SelectAndInstall: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Install: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszwinfid: ::windows::core::PCWSTR, pobotoken: *const OBO_TOKEN, dwsetupflags: u32, dwupgradefrombuildno: u32, pszwanswerfile: ::windows::core::PCWSTR, pszwanswersections: ::windows::core::PCWSTR, ppnccitem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Install: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DeInstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcomponent: *mut ::core::ffi::c_void, pobotoken: *const OBO_TOKEN, pmszwrefs: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DeInstall: usize,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[repr(transparent)]
pub struct INetCfgClassSetup2(::windows::core::IUnknown);
impl INetCfgClassSetup2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SelectAndInstall<P0>(&self, hwndparent: P0, pobotoken: ::core::option::Option<*const OBO_TOKEN>, ppnccitem: ::core::option::Option<*mut ::core::option::Option<INetCfgComponent>>) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    {
        (::windows::core::Interface::vtable(self).base__.SelectAndInstall)(::windows::core::Interface::as_raw(self), hwndparent.into_param().abi(), ::core::mem::transmute(pobotoken.unwrap_or(::std::ptr::null())), ::core::mem::transmute(ppnccitem.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Install<P0, P1, P2>(&self, pszwinfid: P0, pobotoken: ::core::option::Option<*const OBO_TOKEN>, dwsetupflags: u32, dwupgradefrombuildno: u32, pszwanswerfile: P1, pszwanswersections: P2, ppnccitem: ::core::option::Option<*mut ::core::option::Option<INetCfgComponent>>) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.Install)(::windows::core::Interface::as_raw(self), pszwinfid.into_param().abi(), ::core::mem::transmute(pobotoken.unwrap_or(::std::ptr::null())), dwsetupflags, dwupgradefrombuildno, pszwanswerfile.into_param().abi(), pszwanswersections.into_param().abi(), ::core::mem::transmute(ppnccitem.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeInstall<P0>(&self, pcomponent: P0, pobotoken: ::core::option::Option<*const OBO_TOKEN>, pmszwrefs: ::core::option::Option<*mut ::windows::core::PWSTR>) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<INetCfgComponent>,
    {
        (::windows::core::Interface::vtable(self).base__.DeInstall)(::windows::core::Interface::as_raw(self), pcomponent.into_param().abi(), ::core::mem::transmute(pobotoken.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pmszwrefs.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn UpdateNonEnumeratedComponent<P0>(&self, picomp: P0, dwsetupflags: u32, dwupgradefrombuildno: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<INetCfgComponent>,
    {
        (::windows::core::Interface::vtable(self).UpdateNonEnumeratedComponent)(::windows::core::Interface::as_raw(self), picomp.into_param().abi(), dwsetupflags, dwupgradefrombuildno).ok()
    }
}
::windows::imp::interface_hierarchy!(INetCfgClassSetup2, ::windows::core::IUnknown, INetCfgClassSetup);
impl ::core::cmp::PartialEq for INetCfgClassSetup2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetCfgClassSetup2 {}
impl ::core::fmt::Debug for INetCfgClassSetup2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetCfgClassSetup2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for INetCfgClassSetup2 {
    type Vtable = INetCfgClassSetup2_Vtbl;
}
impl ::core::clone::Clone for INetCfgClassSetup2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for INetCfgClassSetup2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc0e8aea0_306e_11d1_aacf_00805fc1270e);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetCfgClassSetup2_Vtbl {
    pub base__: INetCfgClassSetup_Vtbl,
    pub UpdateNonEnumeratedComponent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, picomp: *mut ::core::ffi::c_void, dwsetupflags: u32, dwupgradefrombuildno: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[repr(transparent)]
pub struct INetCfgComponent(::windows::core::IUnknown);
impl INetCfgComponent {
    pub unsafe fn GetDisplayName(&self, ppszwdisplayname: ::core::option::Option<*mut ::windows::core::PWSTR>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDisplayName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppszwdisplayname.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetDisplayName<P0>(&self, pszwdisplayname: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetDisplayName)(::windows::core::Interface::as_raw(self), pszwdisplayname.into_param().abi()).ok()
    }
    pub unsafe fn GetHelpText(&self, pszwhelptext: ::core::option::Option<*mut ::windows::core::PWSTR>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetHelpText)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pszwhelptext.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetId(&self, ppszwid: ::core::option::Option<*mut ::windows::core::PWSTR>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetId)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppszwid.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetCharacteristics(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetCharacteristics)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetInstanceGuid(&self, pguid: ::core::option::Option<*mut ::windows::core::GUID>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetInstanceGuid)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pguid.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetPnpDevNodeId(&self, ppszwdevnodeid: ::core::option::Option<*mut ::windows::core::PWSTR>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPnpDevNodeId)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppszwdevnodeid.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetClassGuid(&self, pguid: ::core::option::Option<*mut ::windows::core::GUID>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetClassGuid)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pguid.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetBindName(&self, ppszwbindname: ::core::option::Option<*mut ::windows::core::PWSTR>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetBindName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppszwbindname.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetDeviceStatus(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetDeviceStatus)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Registry\"`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub unsafe fn OpenParamKey(&self, phkey: ::core::option::Option<*mut super::super::System::Registry::HKEY>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OpenParamKey)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(phkey.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RaisePropertyUi<P0, P1>(&self, hwndparent: P0, dwflags: u32, punkcontext: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
        P1: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).RaisePropertyUi)(::windows::core::Interface::as_raw(self), hwndparent.into_param().abi(), dwflags, punkcontext.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(INetCfgComponent, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for INetCfgComponent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetCfgComponent {}
impl ::core::fmt::Debug for INetCfgComponent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetCfgComponent").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for INetCfgComponent {
    type Vtable = INetCfgComponent_Vtbl;
}
impl ::core::clone::Clone for INetCfgComponent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for INetCfgComponent {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc0e8ae99_306e_11d1_aacf_00805fc1270e);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetCfgComponent_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszwdisplayname: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszwdisplayname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetHelpText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszwhelptext: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszwid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetCharacteristics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwcharacteristics: *mut u32) -> ::windows::core::HRESULT,
    pub GetInstanceGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetPnpDevNodeId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszwdevnodeid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetClassGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetBindName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszwbindname: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetDeviceStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pulstatus: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Registry")]
    pub OpenParamKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phkey: *mut super::super::System::Registry::HKEY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Registry"))]
    OpenParamKey: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RaisePropertyUi: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, dwflags: u32, punkcontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RaisePropertyUi: usize,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[repr(transparent)]
pub struct INetCfgComponentBindings(::windows::core::IUnknown);
impl INetCfgComponentBindings {
    pub unsafe fn BindTo<P0>(&self, pnccitem: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<INetCfgComponent>,
    {
        (::windows::core::Interface::vtable(self).BindTo)(::windows::core::Interface::as_raw(self), pnccitem.into_param().abi()).ok()
    }
    pub unsafe fn UnbindFrom<P0>(&self, pnccitem: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<INetCfgComponent>,
    {
        (::windows::core::Interface::vtable(self).UnbindFrom)(::windows::core::Interface::as_raw(self), pnccitem.into_param().abi()).ok()
    }
    pub unsafe fn SupportsBindingInterface<P0>(&self, dwflags: u32, pszwinterfacename: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SupportsBindingInterface)(::windows::core::Interface::as_raw(self), dwflags, pszwinterfacename.into_param().abi()).ok()
    }
    pub unsafe fn IsBoundTo<P0>(&self, pnccitem: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<INetCfgComponent>,
    {
        (::windows::core::Interface::vtable(self).IsBoundTo)(::windows::core::Interface::as_raw(self), pnccitem.into_param().abi()).ok()
    }
    pub unsafe fn IsBindableTo<P0>(&self, pnccitem: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<INetCfgComponent>,
    {
        (::windows::core::Interface::vtable(self).IsBindableTo)(::windows::core::Interface::as_raw(self), pnccitem.into_param().abi()).ok()
    }
    pub unsafe fn EnumBindingPaths(&self, dwflags: u32, ppienum: ::core::option::Option<*mut ::core::option::Option<IEnumNetCfgBindingPath>>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnumBindingPaths)(::windows::core::Interface::as_raw(self), dwflags, ::core::mem::transmute(ppienum.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn MoveBefore<P0, P1>(&self, pncbitemsrc: P0, pncbitemdest: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<INetCfgBindingPath>,
        P1: ::windows::core::IntoParam<INetCfgBindingPath>,
    {
        (::windows::core::Interface::vtable(self).MoveBefore)(::windows::core::Interface::as_raw(self), pncbitemsrc.into_param().abi(), pncbitemdest.into_param().abi()).ok()
    }
    pub unsafe fn MoveAfter<P0, P1>(&self, pncbitemsrc: P0, pncbitemdest: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<INetCfgBindingPath>,
        P1: ::windows::core::IntoParam<INetCfgBindingPath>,
    {
        (::windows::core::Interface::vtable(self).MoveAfter)(::windows::core::Interface::as_raw(self), pncbitemsrc.into_param().abi(), pncbitemdest.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(INetCfgComponentBindings, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for INetCfgComponentBindings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetCfgComponentBindings {}
impl ::core::fmt::Debug for INetCfgComponentBindings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetCfgComponentBindings").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for INetCfgComponentBindings {
    type Vtable = INetCfgComponentBindings_Vtbl;
}
impl ::core::clone::Clone for INetCfgComponentBindings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for INetCfgComponentBindings {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc0e8ae9e_306e_11d1_aacf_00805fc1270e);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetCfgComponentBindings_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub BindTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnccitem: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub UnbindFrom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnccitem: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SupportsBindingInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, pszwinterfacename: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub IsBoundTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnccitem: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IsBindableTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnccitem: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EnumBindingPaths: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, ppienum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub MoveBefore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pncbitemsrc: *mut ::core::ffi::c_void, pncbitemdest: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub MoveAfter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pncbitemsrc: *mut ::core::ffi::c_void, pncbitemdest: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[repr(transparent)]
pub struct INetCfgComponentControl(::windows::core::IUnknown);
impl INetCfgComponentControl {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Initialize<P0, P1, P2>(&self, picomp: P0, pinetcfg: P1, finstalling: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<INetCfgComponent>,
        P1: ::windows::core::IntoParam<INetCfg>,
        P2: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).Initialize)(::windows::core::Interface::as_raw(self), picomp.into_param().abi(), pinetcfg.into_param().abi(), finstalling.into_param().abi()).ok()
    }
    pub unsafe fn ApplyRegistryChanges(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ApplyRegistryChanges)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ApplyPnpChanges<P0>(&self, picallback: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<INetCfgPnpReconfigCallback>,
    {
        (::windows::core::Interface::vtable(self).ApplyPnpChanges)(::windows::core::Interface::as_raw(self), picallback.into_param().abi()).ok()
    }
    pub unsafe fn CancelChanges(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CancelChanges)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(INetCfgComponentControl, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for INetCfgComponentControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetCfgComponentControl {}
impl ::core::fmt::Debug for INetCfgComponentControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetCfgComponentControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for INetCfgComponentControl {
    type Vtable = INetCfgComponentControl_Vtbl;
}
impl ::core::clone::Clone for INetCfgComponentControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for INetCfgComponentControl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x932238df_bea1_11d0_9298_00c04fc99dcf);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetCfgComponentControl_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, picomp: *mut ::core::ffi::c_void, pinetcfg: *mut ::core::ffi::c_void, finstalling: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Initialize: usize,
    pub ApplyRegistryChanges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ApplyPnpChanges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, picallback: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CancelChanges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[repr(transparent)]
pub struct INetCfgComponentNotifyBinding(::windows::core::IUnknown);
impl INetCfgComponentNotifyBinding {
    pub unsafe fn QueryBindingPath<P0>(&self, dwchangeflag: u32, pipath: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<INetCfgBindingPath>,
    {
        (::windows::core::Interface::vtable(self).QueryBindingPath)(::windows::core::Interface::as_raw(self), dwchangeflag, pipath.into_param().abi()).ok()
    }
    pub unsafe fn NotifyBindingPath<P0>(&self, dwchangeflag: u32, pipath: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<INetCfgBindingPath>,
    {
        (::windows::core::Interface::vtable(self).NotifyBindingPath)(::windows::core::Interface::as_raw(self), dwchangeflag, pipath.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(INetCfgComponentNotifyBinding, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for INetCfgComponentNotifyBinding {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetCfgComponentNotifyBinding {}
impl ::core::fmt::Debug for INetCfgComponentNotifyBinding {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetCfgComponentNotifyBinding").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for INetCfgComponentNotifyBinding {
    type Vtable = INetCfgComponentNotifyBinding_Vtbl;
}
impl ::core::clone::Clone for INetCfgComponentNotifyBinding {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for INetCfgComponentNotifyBinding {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x932238e1_bea1_11d0_9298_00c04fc99dcf);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetCfgComponentNotifyBinding_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub QueryBindingPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwchangeflag: u32, pipath: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub NotifyBindingPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwchangeflag: u32, pipath: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[repr(transparent)]
pub struct INetCfgComponentNotifyGlobal(::windows::core::IUnknown);
impl INetCfgComponentNotifyGlobal {
    pub unsafe fn GetSupportedNotifications(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetSupportedNotifications)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SysQueryBindingPath<P0>(&self, dwchangeflag: u32, pipath: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<INetCfgBindingPath>,
    {
        (::windows::core::Interface::vtable(self).SysQueryBindingPath)(::windows::core::Interface::as_raw(self), dwchangeflag, pipath.into_param().abi()).ok()
    }
    pub unsafe fn SysNotifyBindingPath<P0>(&self, dwchangeflag: u32, pipath: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<INetCfgBindingPath>,
    {
        (::windows::core::Interface::vtable(self).SysNotifyBindingPath)(::windows::core::Interface::as_raw(self), dwchangeflag, pipath.into_param().abi()).ok()
    }
    pub unsafe fn SysNotifyComponent<P0>(&self, dwchangeflag: u32, picomp: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<INetCfgComponent>,
    {
        (::windows::core::Interface::vtable(self).SysNotifyComponent)(::windows::core::Interface::as_raw(self), dwchangeflag, picomp.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(INetCfgComponentNotifyGlobal, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for INetCfgComponentNotifyGlobal {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetCfgComponentNotifyGlobal {}
impl ::core::fmt::Debug for INetCfgComponentNotifyGlobal {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetCfgComponentNotifyGlobal").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for INetCfgComponentNotifyGlobal {
    type Vtable = INetCfgComponentNotifyGlobal_Vtbl;
}
impl ::core::clone::Clone for INetCfgComponentNotifyGlobal {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for INetCfgComponentNotifyGlobal {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x932238e2_bea1_11d0_9298_00c04fc99dcf);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetCfgComponentNotifyGlobal_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetSupportedNotifications: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwnotifications: *mut u32) -> ::windows::core::HRESULT,
    pub SysQueryBindingPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwchangeflag: u32, pipath: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SysNotifyBindingPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwchangeflag: u32, pipath: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SysNotifyComponent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwchangeflag: u32, picomp: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[repr(transparent)]
pub struct INetCfgComponentPropertyUi(::windows::core::IUnknown);
impl INetCfgComponentPropertyUi {
    pub unsafe fn QueryPropertyUi<P0>(&self, punkreserved: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).QueryPropertyUi)(::windows::core::Interface::as_raw(self), punkreserved.into_param().abi()).ok()
    }
    pub unsafe fn SetContext<P0>(&self, punkreserved: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).SetContext)(::windows::core::Interface::as_raw(self), punkreserved.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MergePropPages<P0>(&self, pdwdefpages: *mut u32, pahpspprivate: *mut *mut u8, pcpages: *mut u32, hwndparent: P0, pszstartpage: ::core::option::Option<*const ::windows::core::PCWSTR>) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    {
        (::windows::core::Interface::vtable(self).MergePropPages)(::windows::core::Interface::as_raw(self), pdwdefpages, pahpspprivate, pcpages, hwndparent.into_param().abi(), ::core::mem::transmute(pszstartpage.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ValidateProperties<P0>(&self, hwndsheet: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    {
        (::windows::core::Interface::vtable(self).ValidateProperties)(::windows::core::Interface::as_raw(self), hwndsheet.into_param().abi()).ok()
    }
    pub unsafe fn ApplyProperties(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ApplyProperties)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn CancelProperties(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CancelProperties)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(INetCfgComponentPropertyUi, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for INetCfgComponentPropertyUi {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetCfgComponentPropertyUi {}
impl ::core::fmt::Debug for INetCfgComponentPropertyUi {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetCfgComponentPropertyUi").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for INetCfgComponentPropertyUi {
    type Vtable = INetCfgComponentPropertyUi_Vtbl;
}
impl ::core::clone::Clone for INetCfgComponentPropertyUi {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for INetCfgComponentPropertyUi {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x932238e0_bea1_11d0_9298_00c04fc99dcf);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetCfgComponentPropertyUi_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub QueryPropertyUi: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punkreserved: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punkreserved: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub MergePropPages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwdefpages: *mut u32, pahpspprivate: *mut *mut u8, pcpages: *mut u32, hwndparent: super::super::Foundation::HWND, pszstartpage: *const ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MergePropPages: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ValidateProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndsheet: super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ValidateProperties: usize,
    pub ApplyProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CancelProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[repr(transparent)]
pub struct INetCfgComponentSetup(::windows::core::IUnknown);
impl INetCfgComponentSetup {
    pub unsafe fn Install(&self, dwsetupflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Install)(::windows::core::Interface::as_raw(self), dwsetupflags).ok()
    }
    pub unsafe fn Upgrade(&self, dwsetupflags: u32, dwupgradefombuildno: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Upgrade)(::windows::core::Interface::as_raw(self), dwsetupflags, dwupgradefombuildno).ok()
    }
    pub unsafe fn ReadAnswerFile<P0, P1>(&self, pszwanswerfile: P0, pszwanswersections: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).ReadAnswerFile)(::windows::core::Interface::as_raw(self), pszwanswerfile.into_param().abi(), pszwanswersections.into_param().abi()).ok()
    }
    pub unsafe fn Removing(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Removing)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(INetCfgComponentSetup, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for INetCfgComponentSetup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetCfgComponentSetup {}
impl ::core::fmt::Debug for INetCfgComponentSetup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetCfgComponentSetup").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for INetCfgComponentSetup {
    type Vtable = INetCfgComponentSetup_Vtbl;
}
impl ::core::clone::Clone for INetCfgComponentSetup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for INetCfgComponentSetup {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x932238e3_bea1_11d0_9298_00c04fc99dcf);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetCfgComponentSetup_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Install: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwsetupflags: u32) -> ::windows::core::HRESULT,
    pub Upgrade: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwsetupflags: u32, dwupgradefombuildno: u32) -> ::windows::core::HRESULT,
    pub ReadAnswerFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszwanswerfile: ::windows::core::PCWSTR, pszwanswersections: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub Removing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[repr(transparent)]
pub struct INetCfgComponentSysPrep(::windows::core::IUnknown);
impl INetCfgComponentSysPrep {
    pub unsafe fn SaveAdapterParameters<P0, P1>(&self, pncsp: P0, pszwanswersections: P1, padapterinstanceguid: *const ::windows::core::GUID) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<INetCfgSysPrep>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SaveAdapterParameters)(::windows::core::Interface::as_raw(self), pncsp.into_param().abi(), pszwanswersections.into_param().abi(), padapterinstanceguid).ok()
    }
    pub unsafe fn RestoreAdapterParameters<P0, P1>(&self, pszwanswerfile: P0, pszwanswersection: P1, padapterinstanceguid: *const ::windows::core::GUID) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).RestoreAdapterParameters)(::windows::core::Interface::as_raw(self), pszwanswerfile.into_param().abi(), pszwanswersection.into_param().abi(), padapterinstanceguid).ok()
    }
}
::windows::imp::interface_hierarchy!(INetCfgComponentSysPrep, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for INetCfgComponentSysPrep {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetCfgComponentSysPrep {}
impl ::core::fmt::Debug for INetCfgComponentSysPrep {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetCfgComponentSysPrep").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for INetCfgComponentSysPrep {
    type Vtable = INetCfgComponentSysPrep_Vtbl;
}
impl ::core::clone::Clone for INetCfgComponentSysPrep {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for INetCfgComponentSysPrep {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc0e8ae9a_306e_11d1_aacf_00805fc1270e);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetCfgComponentSysPrep_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SaveAdapterParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pncsp: *mut ::core::ffi::c_void, pszwanswersections: ::windows::core::PCWSTR, padapterinstanceguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub RestoreAdapterParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszwanswerfile: ::windows::core::PCWSTR, pszwanswersection: ::windows::core::PCWSTR, padapterinstanceguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[repr(transparent)]
pub struct INetCfgComponentUpperEdge(::windows::core::IUnknown);
impl INetCfgComponentUpperEdge {
    pub unsafe fn GetInterfaceIdsForAdapter<P0>(&self, padapter: P0, pdwnuminterfaces: *mut u32, ppguidinterfaceids: ::core::option::Option<*mut *mut ::windows::core::GUID>) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<INetCfgComponent>,
    {
        (::windows::core::Interface::vtable(self).GetInterfaceIdsForAdapter)(::windows::core::Interface::as_raw(self), padapter.into_param().abi(), pdwnuminterfaces, ::core::mem::transmute(ppguidinterfaceids.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn AddInterfacesToAdapter<P0>(&self, padapter: P0, dwnuminterfaces: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<INetCfgComponent>,
    {
        (::windows::core::Interface::vtable(self).AddInterfacesToAdapter)(::windows::core::Interface::as_raw(self), padapter.into_param().abi(), dwnuminterfaces).ok()
    }
    pub unsafe fn RemoveInterfacesFromAdapter<P0>(&self, padapter: P0, pguidinterfaceids: &[::windows::core::GUID]) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<INetCfgComponent>,
    {
        (::windows::core::Interface::vtable(self).RemoveInterfacesFromAdapter)(::windows::core::Interface::as_raw(self), padapter.into_param().abi(), pguidinterfaceids.len() as _, ::core::mem::transmute(pguidinterfaceids.as_ptr())).ok()
    }
}
::windows::imp::interface_hierarchy!(INetCfgComponentUpperEdge, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for INetCfgComponentUpperEdge {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetCfgComponentUpperEdge {}
impl ::core::fmt::Debug for INetCfgComponentUpperEdge {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetCfgComponentUpperEdge").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for INetCfgComponentUpperEdge {
    type Vtable = INetCfgComponentUpperEdge_Vtbl;
}
impl ::core::clone::Clone for INetCfgComponentUpperEdge {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for INetCfgComponentUpperEdge {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x932238e4_bea1_11d0_9298_00c04fc99dcf);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetCfgComponentUpperEdge_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetInterfaceIdsForAdapter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, padapter: *mut ::core::ffi::c_void, pdwnuminterfaces: *mut u32, ppguidinterfaceids: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub AddInterfacesToAdapter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, padapter: *mut ::core::ffi::c_void, dwnuminterfaces: u32) -> ::windows::core::HRESULT,
    pub RemoveInterfacesFromAdapter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, padapter: *mut ::core::ffi::c_void, dwnuminterfaces: u32, pguidinterfaceids: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[repr(transparent)]
pub struct INetCfgLock(::windows::core::IUnknown);
impl INetCfgLock {
    pub unsafe fn AcquireWriteLock<P0>(&self, cmstimeout: u32, pszwclientdescription: P0, ppszwclientdescription: ::core::option::Option<*mut ::windows::core::PWSTR>) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).AcquireWriteLock)(::windows::core::Interface::as_raw(self), cmstimeout, pszwclientdescription.into_param().abi(), ::core::mem::transmute(ppszwclientdescription.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn ReleaseWriteLock(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReleaseWriteLock)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn IsWriteLocked(&self, ppszwclientdescription: ::core::option::Option<*mut ::windows::core::PWSTR>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).IsWriteLocked)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppszwclientdescription.unwrap_or(::std::ptr::null_mut()))).ok()
    }
}
::windows::imp::interface_hierarchy!(INetCfgLock, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for INetCfgLock {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetCfgLock {}
impl ::core::fmt::Debug for INetCfgLock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetCfgLock").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for INetCfgLock {
    type Vtable = INetCfgLock_Vtbl;
}
impl ::core::clone::Clone for INetCfgLock {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for INetCfgLock {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc0e8ae9f_306e_11d1_aacf_00805fc1270e);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetCfgLock_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub AcquireWriteLock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cmstimeout: u32, pszwclientdescription: ::windows::core::PCWSTR, ppszwclientdescription: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub ReleaseWriteLock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IsWriteLocked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszwclientdescription: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[repr(transparent)]
pub struct INetCfgPnpReconfigCallback(::windows::core::IUnknown);
impl INetCfgPnpReconfigCallback {
    pub unsafe fn SendPnpReconfig<P0, P1>(&self, layer: NCPNP_RECONFIG_LAYER, pszwupper: P0, pszwlower: P1, pvdata: *const ::core::ffi::c_void, dwsizeofdata: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SendPnpReconfig)(::windows::core::Interface::as_raw(self), layer, pszwupper.into_param().abi(), pszwlower.into_param().abi(), pvdata, dwsizeofdata).ok()
    }
}
::windows::imp::interface_hierarchy!(INetCfgPnpReconfigCallback, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for INetCfgPnpReconfigCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetCfgPnpReconfigCallback {}
impl ::core::fmt::Debug for INetCfgPnpReconfigCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetCfgPnpReconfigCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for INetCfgPnpReconfigCallback {
    type Vtable = INetCfgPnpReconfigCallback_Vtbl;
}
impl ::core::clone::Clone for INetCfgPnpReconfigCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for INetCfgPnpReconfigCallback {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8d84bd35_e227_11d2_b700_00a0c98a6a85);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetCfgPnpReconfigCallback_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SendPnpReconfig: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, layer: NCPNP_RECONFIG_LAYER, pszwupper: ::windows::core::PCWSTR, pszwlower: ::windows::core::PCWSTR, pvdata: *const ::core::ffi::c_void, dwsizeofdata: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[repr(transparent)]
pub struct INetCfgSysPrep(::windows::core::IUnknown);
impl INetCfgSysPrep {
    pub unsafe fn HrSetupSetFirstDword<P0, P1>(&self, pwszsection: P0, pwszkey: P1, dwvalue: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).HrSetupSetFirstDword)(::windows::core::Interface::as_raw(self), pwszsection.into_param().abi(), pwszkey.into_param().abi(), dwvalue).ok()
    }
    pub unsafe fn HrSetupSetFirstString<P0, P1, P2>(&self, pwszsection: P0, pwszkey: P1, pwszvalue: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).HrSetupSetFirstString)(::windows::core::Interface::as_raw(self), pwszsection.into_param().abi(), pwszkey.into_param().abi(), pwszvalue.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HrSetupSetFirstStringAsBool<P0, P1, P2>(&self, pwszsection: P0, pwszkey: P1, fvalue: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).HrSetupSetFirstStringAsBool)(::windows::core::Interface::as_raw(self), pwszsection.into_param().abi(), pwszkey.into_param().abi(), fvalue.into_param().abi()).ok()
    }
    pub unsafe fn HrSetupSetFirstMultiSzField<P0, P1, P2>(&self, pwszsection: P0, pwszkey: P1, pmszvalue: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).HrSetupSetFirstMultiSzField)(::windows::core::Interface::as_raw(self), pwszsection.into_param().abi(), pwszkey.into_param().abi(), pmszvalue.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(INetCfgSysPrep, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for INetCfgSysPrep {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetCfgSysPrep {}
impl ::core::fmt::Debug for INetCfgSysPrep {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetCfgSysPrep").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for INetCfgSysPrep {
    type Vtable = INetCfgSysPrep_Vtbl;
}
impl ::core::clone::Clone for INetCfgSysPrep {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for INetCfgSysPrep {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc0e8ae98_306e_11d1_aacf_00805fc1270e);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetCfgSysPrep_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub HrSetupSetFirstDword: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszsection: ::windows::core::PCWSTR, pwszkey: ::windows::core::PCWSTR, dwvalue: u32) -> ::windows::core::HRESULT,
    pub HrSetupSetFirstString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszsection: ::windows::core::PCWSTR, pwszkey: ::windows::core::PCWSTR, pwszvalue: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub HrSetupSetFirstStringAsBool: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszsection: ::windows::core::PCWSTR, pwszkey: ::windows::core::PCWSTR, fvalue: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    HrSetupSetFirstStringAsBool: usize,
    pub HrSetupSetFirstMultiSzField: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszsection: ::windows::core::PCWSTR, pwszkey: ::windows::core::PCWSTR, pmszvalue: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[repr(transparent)]
pub struct INetLanConnectionUiInfo(::windows::core::IUnknown);
impl INetLanConnectionUiInfo {
    pub unsafe fn GetDeviceGuid(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).GetDeviceGuid)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(INetLanConnectionUiInfo, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for INetLanConnectionUiInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetLanConnectionUiInfo {}
impl ::core::fmt::Debug for INetLanConnectionUiInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetLanConnectionUiInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for INetLanConnectionUiInfo {
    type Vtable = INetLanConnectionUiInfo_Vtbl;
}
impl ::core::clone::Clone for INetLanConnectionUiInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for INetLanConnectionUiInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc08956a6_1cd3_11d1_b1c5_00805fc1270e);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetLanConnectionUiInfo_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetDeviceGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[repr(transparent)]
pub struct INetRasConnectionIpUiInfo(::windows::core::IUnknown);
impl INetRasConnectionIpUiInfo {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetUiInfo(&self, pinfo: *mut RASCON_IPUI) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetUiInfo)(::windows::core::Interface::as_raw(self), pinfo).ok()
    }
}
::windows::imp::interface_hierarchy!(INetRasConnectionIpUiInfo, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for INetRasConnectionIpUiInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetRasConnectionIpUiInfo {}
impl ::core::fmt::Debug for INetRasConnectionIpUiInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetRasConnectionIpUiInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for INetRasConnectionIpUiInfo {
    type Vtable = INetRasConnectionIpUiInfo_Vtbl;
}
impl ::core::clone::Clone for INetRasConnectionIpUiInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for INetRasConnectionIpUiInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfaedcf58_31fe_11d1_aad2_00805fc1270e);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetRasConnectionIpUiInfo_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetUiInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *mut RASCON_IPUI) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetUiInfo: usize,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[repr(transparent)]
pub struct IProvisioningDomain(::windows::core::IUnknown);
impl IProvisioningDomain {
    pub unsafe fn Add<P0>(&self, pszwpathtofolder: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).Add)(::windows::core::Interface::as_raw(self), pszwpathtofolder.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
    pub unsafe fn Query<P0, P1, P2>(&self, pszwdomain: P0, pszwlanguage: P1, pszwxpathquery: P2) -> ::windows::core::Result<super::super::Data::Xml::MsXml::IXMLDOMNodeList>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::Data::Xml::MsXml::IXMLDOMNodeList>();
        (::windows::core::Interface::vtable(self).Query)(::windows::core::Interface::as_raw(self), pszwdomain.into_param().abi(), pszwlanguage.into_param().abi(), pszwxpathquery.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IProvisioningDomain, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IProvisioningDomain {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProvisioningDomain {}
impl ::core::fmt::Debug for IProvisioningDomain {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProvisioningDomain").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IProvisioningDomain {
    type Vtable = IProvisioningDomain_Vtbl;
}
impl ::core::clone::Clone for IProvisioningDomain {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IProvisioningDomain {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc96fbd50_24dd_11d8_89fb_00904b2ea9c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProvisioningDomain_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszwpathtofolder: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
    pub Query: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszwdomain: ::windows::core::PCWSTR, pszwlanguage: ::windows::core::PCWSTR, pszwxpathquery: ::windows::core::PCWSTR, nodes: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com")))]
    Query: usize,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[repr(transparent)]
pub struct IProvisioningProfileWireless(::windows::core::IUnknown);
impl IProvisioningProfileWireless {
    pub unsafe fn CreateProfile<P0, P1>(&self, bstrxmlwirelessconfigprofile: P0, bstrxmlconnectionconfigprofile: P1, padapterinstanceguid: *const ::windows::core::GUID) -> ::windows::core::Result<u32>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).CreateProfile)(::windows::core::Interface::as_raw(self), bstrxmlwirelessconfigprofile.into_param().abi(), bstrxmlconnectionconfigprofile.into_param().abi(), padapterinstanceguid, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IProvisioningProfileWireless, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IProvisioningProfileWireless {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProvisioningProfileWireless {}
impl ::core::fmt::Debug for IProvisioningProfileWireless {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProvisioningProfileWireless").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IProvisioningProfileWireless {
    type Vtable = IProvisioningProfileWireless_Vtbl;
}
impl ::core::clone::Clone for IProvisioningProfileWireless {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IProvisioningProfileWireless {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc96fbd51_24dd_11d8_89fb_00904b2ea9c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProvisioningProfileWireless_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub CreateProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrxmlwirelessconfigprofile: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrxmlconnectionconfigprofile: ::std::mem::MaybeUninit<::windows::core::BSTR>, padapterinstanceguid: *const ::windows::core::GUID, pulstatus: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const AA_AUDIT_ALL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const AA_A_ACL: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const AA_A_CREATE: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const AA_A_DELETE: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const AA_A_OPEN: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const AA_A_OWNER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const AA_A_WRITE: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const AA_CLOSE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const AA_F_ACL: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const AA_F_CREATE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const AA_F_DELETE: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const AA_F_OPEN: u32 = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const AA_F_WRITE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const AA_S_ACL: u32 = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const AA_S_CREATE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const AA_S_DELETE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const AA_S_OPEN: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const AA_S_WRITE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const ACCESS_ACCESS_LIST_PARMNUM: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const ACCESS_ATTR_PARMNUM: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const ACCESS_AUDIT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const ACCESS_COUNT_PARMNUM: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const ACCESS_FAIL_ACL: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const ACCESS_FAIL_DELETE: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const ACCESS_FAIL_MASK: u32 = 3840u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const ACCESS_FAIL_OPEN: u32 = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const ACCESS_FAIL_SHIFT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const ACCESS_FAIL_WRITE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const ACCESS_GROUP: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const ACCESS_LETTERS: ::windows::core::PCSTR = ::windows::s!("RWCXDAP         ");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const ACCESS_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const ACCESS_RESOURCE_NAME_PARMNUM: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const ACCESS_SUCCESS_ACL: u32 = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const ACCESS_SUCCESS_DELETE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const ACCESS_SUCCESS_MASK: u32 = 240u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const ACCESS_SUCCESS_OPEN: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const ACCESS_SUCCESS_WRITE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const ACTION_ADMINUNLOCK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const ACTION_LOCKOUT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const AE_ACCLIMITEXCD: u32 = 17u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const AE_ACCRESTRICT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const AE_ACLMOD: u32 = 12u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const AE_ACLMODFAIL: u32 = 19u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const AE_ADD: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const AE_ADMIN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const AE_ADMINDIS: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const AE_ADMINPRIVREQD: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const AE_ADMIN_CLOSE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const AE_AUTODIS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const AE_BADPW: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const AE_CLOSEFILE: u32 = 9u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const AE_CONNREJ: u32 = 6u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const AE_CONNSTART: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const AE_CONNSTOP: u32 = 5u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const AE_DELETE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const AE_ERROR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const AE_GENERAL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const AE_GENERIC_TYPE: u32 = 21u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const AE_GUEST: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const AE_LIM_DELETED: u32 = 5u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const AE_LIM_DISABLED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const AE_LIM_EXPIRED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const AE_LIM_INVAL_WKSTA: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const AE_LIM_LOGONHOURS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const AE_LIM_UNKNOWN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const AE_LOCKOUT: u32 = 20u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const AE_MOD: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const AE_NETLOGDENIED: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const AE_NETLOGOFF: u32 = 15u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const AE_NETLOGON: u32 = 14u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const AE_NOACCESSPERM: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const AE_NORMAL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const AE_NORMAL_CLOSE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const AE_RESACCESS: u32 = 7u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const AE_RESACCESS2: u32 = 18u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const AE_RESACCESSREJ: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const AE_SERVICESTAT: u32 = 11u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const AE_SESSDIS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const AE_SESSLOGOFF: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const AE_SESSLOGON: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const AE_SESSPWERR: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const AE_SES_CLOSE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const AE_SRVCONT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const AE_SRVPAUSED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const AE_SRVSTART: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const AE_SRVSTATUS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const AE_SRVSTOP: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const AE_UASMOD: u32 = 13u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const AE_UAS_GROUP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const AE_UAS_MODALS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const AE_UAS_USER: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const AE_UNSHARE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const AE_USER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const AE_USERLIMIT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const ALERTER_MAILSLOT: ::windows::core::PCWSTR = ::windows::w!("\\\\.\\MAILSLOT\\Alerter");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const ALERTSZ: u32 = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const ALERT_ADMIN_EVENT: ::windows::core::PCWSTR = ::windows::w!("ADMIN");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const ALERT_ERRORLOG_EVENT: ::windows::core::PCWSTR = ::windows::w!("ERRORLOG");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const ALERT_MESSAGE_EVENT: ::windows::core::PCWSTR = ::windows::w!("MESSAGE");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const ALERT_PRINT_EVENT: ::windows::core::PCWSTR = ::windows::w!("PRINTING");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const ALERT_USER_EVENT: ::windows::core::PCWSTR = ::windows::w!("USER");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const ALIGN_SIZE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const ALLOCATE_RESPONSE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const BACKUP_MSG_FILENAME: ::windows::core::PCWSTR = ::windows::w!("BAK.MSG");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const CLTYPE_LEN: u32 = 12u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const CNLEN: u32 = 15u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const COULD_NOT_VERIFY_VOLUMES: i32 = -1073727512i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const CREATE_BYPASS_CSC: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const CREATE_CRED_RESET: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const CREATE_GLOBAL_MAPPING: u32 = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const CREATE_NO_CONNECT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const CREATE_PERSIST_MAPPING: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const CREATE_REQUIRE_CONNECTION_INTEGRITY: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const CREATE_REQUIRE_CONNECTION_PRIVACY: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const CREATE_WRITE_THROUGH_SEMANTICS: u32 = 64u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const CRYPT_KEY_LEN: u32 = 7u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const CRYPT_TXT_LEN: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const DEF_MAX_BADPW: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const DEF_MAX_PWHIST: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const DEF_MIN_PWLEN: u32 = 6u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const DEF_PWUNIQUENESS: u32 = 5u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const DEVLEN: u32 = 80u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const DFS_CONNECTION_FAILURE: i32 = 1073756226i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const DFS_ERROR_ACTIVEDIRECTORY_OFFLINE: i32 = -1073727301i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const DFS_ERROR_CLUSTERINFO_FAILED: i32 = -1073727307i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const DFS_ERROR_COMPUTERINFO_FAILED: i32 = -1073727308i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const DFS_ERROR_CREATEEVENT_FAILED: i32 = -1073727309i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const DFS_ERROR_CREATE_REPARSEPOINT_FAILURE: i32 = -1073727321i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const DFS_ERROR_CREATE_REPARSEPOINT_SUCCESS: i32 = 1073756370i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const DFS_ERROR_CROSS_FOREST_TRUST_INFO_FAILED: i32 = -1073727274i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const DFS_ERROR_DCINFO_FAILED: i32 = -1073727306i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const DFS_ERROR_DSCONNECT_FAILED: i32 = -2147469122i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const DFS_ERROR_DUPLICATE_LINK: i32 = -1073727277i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const DFS_ERROR_HANDLENAMESPACE_FAILED: i32 = -1073727304i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const DFS_ERROR_LINKS_OVERLAP: i32 = -1073727280i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const DFS_ERROR_LINK_OVERLAP: i32 = -1073727279i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const DFS_ERROR_MUTLIPLE_ROOTS_NOT_SUPPORTED: i32 = -1073727289i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const DFS_ERROR_NO_DFS_DATA: i32 = -1073727294i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const DFS_ERROR_ON_ROOT: i32 = -2147469114i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const DFS_ERROR_OVERLAPPING_DIRECTORIES: i32 = -1073727319i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const DFS_ERROR_PREFIXTABLE_FAILED: i32 = -1073727305i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const DFS_ERROR_REFLECTIONENGINE_FAILED: i32 = -1073727302i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const DFS_ERROR_REGISTERSTORE_FAILED: i32 = -1073727303i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const DFS_ERROR_REMOVE_LINK_FAILED: i32 = -1073727284i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const DFS_ERROR_RESYNCHRONIZE_FAILED: i32 = -1073727285i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const DFS_ERROR_ROOTSYNCINIT_FAILED: i32 = -1073727310i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const DFS_ERROR_SECURITYINIT_FAILED: i32 = -1073727313i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const DFS_ERROR_SITECACHEINIT_FAILED: i32 = -1073727311i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const DFS_ERROR_SITESUPPOR_FAILED: i32 = -1073727300i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const DFS_ERROR_TARGET_LIST_INCORRECT: i32 = -1073727281i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const DFS_ERROR_THREADINIT_FAILED: i32 = -1073727312i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const DFS_ERROR_TOO_MANY_ERRORS: i32 = -1073727315i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const DFS_ERROR_TRUSTED_DOMAIN_INFO_FAILED: i32 = -1073727276i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const DFS_ERROR_UNSUPPORTED_FILESYSTEM: i32 = -1073727320i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const DFS_ERROR_WINSOCKINIT_FAILED: i32 = -1073727314i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const DFS_INFO_ACTIVEDIRECTORY_ONLINE: i32 = 1073756332i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const DFS_INFO_CROSS_FOREST_TRUST_INFO_SUCCESS: i32 = 1073756375i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const DFS_INFO_DOMAIN_REFERRAL_MIN_OVERFLOW: i32 = 1073756361i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const DFS_INFO_DS_RECONNECTED: i32 = 1073756353i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const DFS_INFO_FINISH_BUILDING_NAMESPACE: i32 = 1073756357i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const DFS_INFO_FINISH_INIT: i32 = 1073756355i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const DFS_INFO_RECONNECT_DATA: i32 = 1073756356i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const DFS_INFO_TRUSTED_DOMAIN_INFO_SUCCESS: i32 = 1073756373i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const DFS_INIT_SUCCESS: i32 = 1073756376i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const DFS_MAX_DNR_ATTEMPTS: i32 = 1073756229i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const DFS_OPEN_FAILURE: i32 = 1073756231i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const DFS_REFERRAL_FAILURE: i32 = 1073756227i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const DFS_REFERRAL_REQUEST: i32 = 1073756142i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const DFS_REFERRAL_SUCCESS: i32 = 1073756228i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const DFS_ROOT_SHARE_ACQUIRE_FAILED: i32 = -2147469095i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const DFS_ROOT_SHARE_ACQUIRE_SUCCESS: i32 = 1073756378i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const DFS_SPECIAL_REFERRAL_FAILURE: i32 = 1073756230i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const DFS_WARN_DOMAIN_REFERRAL_OVERFLOW: i32 = -2147469112i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const DFS_WARN_INCOMPLETE_MOVE: i32 = -2147469110i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const DFS_WARN_METADATA_LINK_INFO_INVALID: i32 = -2147469106i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const DFS_WARN_METADATA_LINK_TYPE_INCORRECT: i32 = -2147469107i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const DNLEN: u32 = 15u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const ENCRYPTED_PWLEN: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const ERRLOG2_BASE: u32 = 5700u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const ERRLOG_BASE: u32 = 3100u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_BAD_ACCOUNT_NAME: i32 = -1073734816i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_BAD_SERVICE_STATE: i32 = -1073734808i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_BOOT_SYSTEM_DRIVERS_FAILED: i32 = -1073734798i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_BOWSER_CANT_READ_REGISTRY: i32 = 1073749853i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_BOWSER_ELECTION_RECEIVED: i32 = 8012i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_BOWSER_ELECTION_SENT_FIND_MASTER_FAILED: i32 = 1073749838i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_BOWSER_ELECTION_SENT_GETBLIST_FAILED: i32 = 1073749837i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_BOWSER_GETBROWSERLIST_THRESHOLD_EXCEEDED: i32 = 1073749855i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_BOWSER_ILLEGAL_DATAGRAM: i32 = -2147475642i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_BOWSER_ILLEGAL_DATAGRAM_THRESHOLD: i32 = -1073733808i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_BOWSER_MAILSLOT_DATAGRAM_THRESHOLD_EXCEEDED: i32 = 1073749854i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_BOWSER_NAME_CONVERSION_FAILED: i32 = -1073733814i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_BOWSER_NON_MASTER_MASTER_ANNOUNCE: i32 = -2147475643i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_BOWSER_NON_PDC_WON_ELECTION: i32 = 1073749852i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_BOWSER_OLD_BACKUP_FOUND: i32 = 1073749848i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_BOWSER_OTHER_MASTER_ON_NET: i32 = -1073733821i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_BOWSER_PDC_LOST_ELECTION: i32 = 1073749851i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_BOWSER_PROMOTED_WHILE_ALREADY_MASTER: i32 = -2147475644i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_BRIDGE_ADAPTER_BIND_FAILED: i32 = -1073727120i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_BRIDGE_ADAPTER_FILTER_FAILED: i32 = -1073727122i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_BRIDGE_ADAPTER_LINK_SPEED_QUERY_FAILED: i32 = -1073727124i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_BRIDGE_ADAPTER_MAC_ADDR_QUERY_FAILED: i32 = -1073727123i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_BRIDGE_ADAPTER_NAME_QUERY_FAILED: i32 = -1073727121i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_BRIDGE_BUFFER_POOL_CREATION_FAILED: i32 = -1073727214i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_BRIDGE_DEVICE_CREATION_FAILED: i32 = -1073727221i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_BRIDGE_ETHERNET_NOT_OFFERED: i32 = -1073727218i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_BRIDGE_INIT_MALLOC_FAILED: i32 = -1073727213i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_BRIDGE_MINIPORT_INIT_FAILED: i32 = -1073727219i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_BRIDGE_MINIPORT_REGISTER_FAILED: i32 = -1073727222i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_BRIDGE_MINIPROT_DEVNAME_MISSING: i32 = -1073727223i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_BRIDGE_NO_BRIDGE_MAC_ADDR: i32 = -1073727220i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_BRIDGE_PACKET_POOL_CREATION_FAILED: i32 = -1073727215i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_BRIDGE_PROTOCOL_REGISTER_FAILED: i32 = -1073727224i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_BRIDGE_THREAD_CREATION_FAILED: i32 = -1073727217i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_BRIDGE_THREAD_REF_FAILED: i32 = -1073727216i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_BROWSER_BACKUP_STOPPED: i32 = -1073733792i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_BROWSER_DEPENDANT_SERVICE_FAILED: i32 = -1073733807i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_BROWSER_DOMAIN_LIST_FAILED: i32 = -2147475626i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_BROWSER_DOMAIN_LIST_RETRIEVED: i32 = 8026i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_BROWSER_ELECTION_SENT_LANMAN_NT_STARTED: i32 = 1073749839i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_BROWSER_ELECTION_SENT_LANMAN_NT_STOPPED: i32 = 1073749857i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_BROWSER_ELECTION_SENT_ROLE_CHANGED: i32 = 1073749859i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_BROWSER_GETBLIST_RECEIVED_NOT_MASTER: i32 = -1073733790i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_BROWSER_ILLEGAL_CONFIG: i32 = -2147475625i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_BROWSER_MASTER_PROMOTION_FAILED: i32 = -1073733815i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_BROWSER_MASTER_PROMOTION_FAILED_NO_MASTER: i32 = -1073733804i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_BROWSER_MASTER_PROMOTION_FAILED_STOPPING: i32 = -1073733805i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_BROWSER_NOT_STARTED_IPX_CONFIG_MISMATCH: i32 = -1073733788i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_BROWSER_OTHERDOMAIN_ADD_FAILED: i32 = -1073733813i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_BROWSER_ROLE_CHANGE_FAILED: i32 = -1073733816i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_BROWSER_SERVER_LIST_FAILED: i32 = -2147475627i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_BROWSER_SERVER_LIST_RETRIEVED: i32 = 8025i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_BROWSER_STATUS_BITS_UPDATE_FAILED: i32 = -1073733817i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_CALL_TO_FUNCTION_FAILED: i32 = -1073734819i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_CALL_TO_FUNCTION_FAILED_II: i32 = -1073734818i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_CIRCULAR_DEPENDENCY_AUTO: i32 = -1073734806i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_CIRCULAR_DEPENDENCY_DEMAND: i32 = -1073734807i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_COMMAND_NOT_INTERACTIVE: i32 = -1073733924i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_COMMAND_START_FAILED: i32 = -1073733923i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_CONNECTION_TIMEOUT: i32 = -1073734815i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_ComputerNameChange: i32 = -2147477637i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_DAV_REDIR_DELAYED_WRITE_FAILED: i32 = -2147468848i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_DCOM_ASSERTION_FAILURE: i32 = -1073731812i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_DCOM_COMPLUS_DISABLED: i32 = -1073731810i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_DCOM_INVALID_ENDPOINT_DATA: i32 = -1073731811i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_DEPEND_ON_LATER_GROUP: i32 = -1073734804i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_DEPEND_ON_LATER_SERVICE: i32 = -1073734805i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_DNSAPI_DEREGISTRATION_FAILED_NOTSUPP: i32 = -2147472466i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_DNSAPI_DEREGISTRATION_FAILED_NOTSUPP_PRIMARY_DN: i32 = -2147472454i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_DNSAPI_DEREGISTRATION_FAILED_OTHER: i32 = -2147472463i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_DNSAPI_DEREGISTRATION_FAILED_OTHER_PRIMARY_DN: i32 = -2147472451i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_DNSAPI_DEREGISTRATION_FAILED_REFUSED: i32 = -2147472465i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_DNSAPI_DEREGISTRATION_FAILED_REFUSED_PRIMARY_DN: i32 = -2147472453i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_DNSAPI_DEREGISTRATION_FAILED_SECURITY: i32 = -2147472464i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_DNSAPI_DEREGISTRATION_FAILED_SECURITY_PRIMARY_DN: i32 = -2147472452i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_DNSAPI_DEREGISTRATION_FAILED_SERVERFAIL: i32 = -2147472467i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_DNSAPI_DEREGISTRATION_FAILED_SERVERFAIL_PRIMARY_DN: i32 = -2147472455i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_DNSAPI_DEREGISTRATION_FAILED_TIMEOUT: i32 = -2147472468i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_DNSAPI_DEREGISTRATION_FAILED_TIMEOUT_PRIMARY_DN: i32 = -2147472456i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_DNSAPI_PTR_DEREGISTRATION_FAILED_NOTSUPP: i32 = -2147472460i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_DNSAPI_PTR_DEREGISTRATION_FAILED_OTHER: i32 = -2147472457i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_DNSAPI_PTR_DEREGISTRATION_FAILED_REFUSED: i32 = -2147472459i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_DNSAPI_PTR_DEREGISTRATION_FAILED_SECURITY: i32 = -2147472458i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_DNSAPI_PTR_DEREGISTRATION_FAILED_SERVERFAIL: i32 = -2147472461i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_DNSAPI_PTR_DEREGISTRATION_FAILED_TIMEOUT: i32 = -2147472462i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_DNSAPI_PTR_REGISTRATION_FAILED_NOTSUPP: i32 = -2147472490i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_DNSAPI_PTR_REGISTRATION_FAILED_OTHER: i32 = -2147472487i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_DNSAPI_PTR_REGISTRATION_FAILED_REFUSED: i32 = -2147472489i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_DNSAPI_PTR_REGISTRATION_FAILED_SECURITY: i32 = -2147472488i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_DNSAPI_PTR_REGISTRATION_FAILED_SERVERFAIL: i32 = -2147472491i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_DNSAPI_PTR_REGISTRATION_FAILED_TIMEOUT: i32 = -2147472492i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_DNSAPI_REGISTERED_ADAPTER: i32 = 1073753024i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_DNSAPI_REGISTERED_ADAPTER_PRIMARY_DN: i32 = 1073753026i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_DNSAPI_REGISTERED_PTR: i32 = 1073753025i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_DNSAPI_REGISTRATION_FAILED_NOTSUPP: i32 = -2147472496i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_DNSAPI_REGISTRATION_FAILED_NOTSUPP_PRIMARY_DN: i32 = -2147472484i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_DNSAPI_REGISTRATION_FAILED_OTHER: i32 = -2147472493i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_DNSAPI_REGISTRATION_FAILED_OTHER_PRIMARY_DN: i32 = -2147472481i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_DNSAPI_REGISTRATION_FAILED_REFUSED: i32 = -2147472495i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_DNSAPI_REGISTRATION_FAILED_REFUSED_PRIMARY_DN: i32 = -2147472483i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_DNSAPI_REGISTRATION_FAILED_SECURITY: i32 = -2147472494i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_DNSAPI_REGISTRATION_FAILED_SECURITY_PRIMARY_DN: i32 = -2147472482i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_DNSAPI_REGISTRATION_FAILED_SERVERFAIL: i32 = -2147472497i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_DNSAPI_REGISTRATION_FAILED_SERVERFAIL_PRIMARY_DN: i32 = -2147472485i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_DNSAPI_REGISTRATION_FAILED_TIMEOUT: i32 = -2147472498i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_DNSAPI_REGISTRATION_FAILED_TIMEOUT_PRIMARY_DN: i32 = -2147472486i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_DNSDomainNameChange: i32 = -2147477636i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_DNS_CACHE_NETWORK_PERF_WARNING: i32 = -2147472598i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_DNS_CACHE_START_FAILURE_LOW_MEMORY: i32 = -1073730817i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_DNS_CACHE_START_FAILURE_NO_CONTROL: i32 = -1073730822i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_DNS_CACHE_START_FAILURE_NO_DLL: i32 = -1073730824i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_DNS_CACHE_START_FAILURE_NO_DONE_EVENT: i32 = -1073730821i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_DNS_CACHE_START_FAILURE_NO_ENTRY: i32 = -1073730823i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_DNS_CACHE_START_FAILURE_NO_RPC: i32 = -1073730820i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_DNS_CACHE_START_FAILURE_NO_SHUTDOWN_NOTIFY: i32 = -1073730819i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_DNS_CACHE_START_FAILURE_NO_UPDATE: i32 = -1073730818i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_DNS_CACHE_UNABLE_TO_REACH_SERVER_WARNING: i32 = -2147472597i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_EQOS_ERROR_MACHINE_POLICY_KEYNAME_SIZE_ZERO: i32 = -1073725118i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_EQOS_ERROR_MACHINE_POLICY_KEYNAME_TOO_LONG: i32 = -1073725120i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_EQOS_ERROR_MACHINE_POLICY_REFERESH: i32 = -1073725124i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_EQOS_ERROR_OPENING_MACHINE_POLICY_ROOT_KEY: i32 = -1073725122i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_EQOS_ERROR_OPENING_MACHINE_POLICY_SUBKEY: i32 = -1073725116i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_EQOS_ERROR_OPENING_USER_POLICY_ROOT_KEY: i32 = -1073725121i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_EQOS_ERROR_OPENING_USER_POLICY_SUBKEY: i32 = -1073725115i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_EQOS_ERROR_PROCESSING_MACHINE_POLICY_FIELD: i32 = -1073725114i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_EQOS_ERROR_PROCESSING_USER_POLICY_FIELD: i32 = -1073725113i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_EQOS_ERROR_SETTING_APP_MARKING: i32 = -1073725111i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_EQOS_ERROR_SETTING_TCP_AUTOTUNING: i32 = -1073725112i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_EQOS_ERROR_USER_POLICY_KEYNAME_SIZE_ZERO: i32 = -1073725117i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_EQOS_ERROR_USER_POLICY_KEYNAME_TOO_LONG: i32 = -1073725119i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_EQOS_ERROR_USER_POLICY_REFERESH: i32 = -1073725123i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_EQOS_INFO_APP_MARKING_ALLOWED: i32 = 1073758335i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_EQOS_INFO_APP_MARKING_IGNORED: i32 = 1073758334i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_EQOS_INFO_APP_MARKING_NOT_CONFIGURED: i32 = 1073758333i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_EQOS_INFO_LOCAL_SETTING_DONT_USE_NLA: i32 = 1073758336i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_EQOS_INFO_MACHINE_POLICY_REFRESH_NO_CHANGE: i32 = 1073758324i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_EQOS_INFO_MACHINE_POLICY_REFRESH_WITH_CHANGE: i32 = 1073758325i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_EQOS_INFO_TCP_AUTOTUNING_HIGHLY_RESTRICTED: i32 = 1073758330i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_EQOS_INFO_TCP_AUTOTUNING_NORMAL: i32 = 1073758332i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_EQOS_INFO_TCP_AUTOTUNING_NOT_CONFIGURED: i32 = 1073758328i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_EQOS_INFO_TCP_AUTOTUNING_OFF: i32 = 1073758329i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_EQOS_INFO_TCP_AUTOTUNING_RESTRICTED: i32 = 1073758331i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_EQOS_INFO_USER_POLICY_REFRESH_NO_CHANGE: i32 = 1073758326i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_EQOS_INFO_USER_POLICY_REFRESH_WITH_CHANGE: i32 = 1073758327i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_EQOS_URL_QOS_APPLICATION_CONFLICT: i32 = 1073758337i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_EQOS_WARNING_MACHINE_POLICY_CONFLICT: i32 = -2147467040i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_EQOS_WARNING_MACHINE_POLICY_NO_FULLPATH_APPNAME: i32 = -2147467038i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_EQOS_WARNING_MACHINE_POLICY_PROFILE_NOT_SPECIFIED: i32 = -2147467044i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_EQOS_WARNING_MACHINE_POLICY_QUOTA_EXCEEDED: i32 = -2147467042i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_EQOS_WARNING_MACHINE_POLICY_VERSION: i32 = -2147467046i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_EQOS_WARNING_TEST_1: i32 = -2147467048i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_EQOS_WARNING_TEST_2: i32 = -2147467047i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_EQOS_WARNING_USER_POLICY_CONFLICT: i32 = -2147467039i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_EQOS_WARNING_USER_POLICY_NO_FULLPATH_APPNAME: i32 = -2147467037i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_EQOS_WARNING_USER_POLICY_PROFILE_NOT_SPECIFIED: i32 = -2147467043i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_EQOS_WARNING_USER_POLICY_QUOTA_EXCEEDED: i32 = -2147467041i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_EQOS_WARNING_USER_POLICY_VERSION: i32 = -2147467045i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_EventLogProductInfo: i32 = -2147477639i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_EventlogAbnormalShutdown: i32 = -2147477640i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_EventlogStarted: i32 = -2147477643i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_EventlogStopped: i32 = -2147477642i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_EventlogUptime: i32 = -2147477635i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_FIRST_LOGON_FAILED: i32 = -1073734811i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_FIRST_LOGON_FAILED_II: i32 = -1073734786i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_FRS_ACCESS_CHECKS_DISABLED: i32 = -2147470131i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_FRS_ACCESS_CHECKS_FAILED_UNKNOWN: i32 = -1073728305i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_FRS_ACCESS_CHECKS_FAILED_USER: i32 = -2147470130i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_FRS_ASSERT: i32 = -1073728318i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_FRS_BAD_REG_DATA: i32 = -2147470101i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_FRS_CANNOT_COMMUNICATE: i32 = -1073728314i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_FRS_CANNOT_CREATE_UUID: i32 = -1073728300i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_FRS_CANNOT_START_BACKUP_RESTORE_IN_PROGRESS: i32 = -1073728303i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_FRS_CANT_OPEN_PREINSTALL: i32 = -1073728273i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_FRS_CANT_OPEN_STAGE: i32 = -1073728274i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_FRS_DATABASE_SPACE: i32 = -1073728313i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_FRS_DISK_WRITE_CACHE_ENABLED: i32 = -2147470136i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_FRS_DS_POLL_ERROR_SUMMARY: i32 = -2147470086i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_FRS_DUPLICATE_IN_CXTION: i32 = -1073728266i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_FRS_DUPLICATE_IN_CXTION_SYSVOL: i32 = -1073728267i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_FRS_ERROR: i32 = -1073728324i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_FRS_ERROR_REPLICA_SET_DELETED: i32 = -2147470088i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_FRS_HUGE_FILE: i32 = -2147470125i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_FRS_IN_ERROR_STATE: i32 = -1073728269i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_FRS_JET_1414: i32 = -1073728311i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_FRS_JOIN_FAIL_TIME_SKEW: i32 = -1073728276i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_FRS_LONG_JOIN: i32 = -2147470140i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_FRS_LONG_JOIN_DONE: i32 = -2147470139i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_FRS_MOVED_PREEXISTING: i32 = -2147470128i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_FRS_NO_DNS_ATTRIBUTE: i32 = -2147470123i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_FRS_NO_SID: i32 = -1073728298i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_FRS_OVERLAPS_LOGGING: i32 = -1073728283i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_FRS_OVERLAPS_OTHER_STAGE: i32 = -1073728279i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_FRS_OVERLAPS_ROOT: i32 = -1073728280i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_FRS_OVERLAPS_STAGE: i32 = -1073728281i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_FRS_OVERLAPS_WORKING: i32 = -1073728282i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_FRS_PREPARE_ROOT_FAILED: i32 = -1073728278i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_FRS_REPLICA_IN_JRNL_WRAP_ERROR: i32 = -1073728263i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_FRS_REPLICA_NO_ROOT_CHANGE: i32 = -1073728268i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_FRS_REPLICA_SET_CREATE_FAIL: i32 = -1073728272i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_FRS_REPLICA_SET_CREATE_OK: i32 = 1073755377i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_FRS_REPLICA_SET_CXTIONS: i32 = 1073755378i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_FRS_RMTCO_TIME_SKEW: i32 = -1073728275i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_FRS_ROOT_HAS_MOVED: i32 = -1073728265i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_FRS_ROOT_NOT_VALID: i32 = -1073728285i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_FRS_STAGE_NOT_VALID: i32 = -1073728284i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_FRS_STAGING_AREA_FULL: i32 = -2147470126i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_FRS_STARTING: i32 = 1073755325i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_FRS_STOPPED: i32 = 1073755327i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_FRS_STOPPED_ASSERT: i32 = -1073728319i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_FRS_STOPPED_FORCE: i32 = -1073728320i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_FRS_STOPPING: i32 = 1073755326i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_FRS_SYSVOL_NOT_READY: i32 = -2147470134i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_FRS_SYSVOL_NOT_READY_PRIMARY: i32 = -2147470133i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_FRS_SYSVOL_READY: i32 = 1073755340i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_FRS_VOLUME_NOT_SUPPORTED: i32 = -1073728317i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_INVALID_DRIVER_DEPENDENCY: i32 = -1073734809i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_IPX_CREATE_DEVICE: i32 = -1073732318i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_IPX_ILLEGAL_CONFIG: i32 = -2147474145i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_IPX_INTERNAL_NET_INVALID: i32 = -1073732320i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_IPX_NEW_DEFAULT_TYPE: i32 = 1073751325i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_IPX_NO_ADAPTERS: i32 = -1073732317i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_IPX_NO_FRAME_TYPES: i32 = -1073732319i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_IPX_SAP_ANNOUNCE: i32 = -2147474146i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_NBT_BAD_BACKUP_WINS_ADDR: i32 = -2147479344i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_NBT_BAD_PRIMARY_WINS_ADDR: i32 = -2147479343i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_NBT_CREATE_ADDRESS: i32 = -1073737517i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_NBT_CREATE_CONNECTION: i32 = -1073737516i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_NBT_CREATE_DEVICE: i32 = -1073737513i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_NBT_CREATE_DRIVER: i32 = -1073737524i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_NBT_DUPLICATE_NAME: i32 = -1073737505i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_NBT_DUPLICATE_NAME_ERROR: i32 = -1073737503i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_NBT_NAME_RELEASE: i32 = -1073737504i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_NBT_NAME_SERVER_ADDRS: i32 = -1073737518i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_NBT_NON_OS_INIT: i32 = -1073737515i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_NBT_NO_BACKUP_WINS: i32 = -2147479346i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_NBT_NO_DEVICES: i32 = -2147479336i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_NBT_NO_RESOURCES: i32 = -1073737502i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_NBT_NO_WINS: i32 = -2147479345i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_NBT_OPEN_REG_LINKAGE: i32 = -1073737511i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_NBT_OPEN_REG_NAMESERVER: i32 = -2147479332i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_NBT_OPEN_REG_PARAMS: i32 = -1073737523i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_NBT_READ_BIND: i32 = -1073737510i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_NBT_READ_EXPORT: i32 = -1073737509i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_NBT_TIMERS: i32 = -1073737514i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_NDIS_ADAPTER_CHECK_ERROR: i32 = -1073736793i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_NDIS_ADAPTER_DISABLED: i32 = -2147478634i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_NDIS_ADAPTER_NOT_FOUND: i32 = -1073736821i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_NDIS_BAD_IO_BASE_ADDRESS: i32 = -1073736812i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_NDIS_BAD_VERSION: i32 = -1073736818i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_NDIS_CABLE_DISCONNECTED_ERROR: i32 = -2147478615i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_NDIS_DMA_CONFLICT: i32 = -2147478629i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_NDIS_DRIVER_FAILURE: i32 = -1073736819i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_NDIS_HARDWARE_FAILURE: i32 = -1073736822i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_NDIS_INTERRUPT_CONFLICT: i32 = -2147478630i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_NDIS_INTERRUPT_CONNECT: i32 = -1073736820i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_NDIS_INVALID_DOWNLOAD_FILE_ERROR: i32 = -1073736804i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_NDIS_INVALID_VALUE_FROM_ADAPTER: i32 = -1073736814i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_NDIS_IO_PORT_CONFLICT: i32 = -2147478633i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_NDIS_LOBE_FAILUE_ERROR: i32 = -2147478621i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_NDIS_MAXFRAMESIZE_ERROR: i32 = -2147478625i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_NDIS_MAXINTERNALBUFS_ERROR: i32 = -2147478624i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_NDIS_MAXMULTICAST_ERROR: i32 = -2147478623i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_NDIS_MAXRECEIVES_ERROR: i32 = -2147478627i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_NDIS_MAXTRANSMITS_ERROR: i32 = -2147478626i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_NDIS_MEMORY_CONFLICT: i32 = -2147478631i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_NDIS_MISSING_CONFIGURATION_PARAMETER: i32 = -1073736813i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_NDIS_NETWORK_ADDRESS: i32 = -1073736816i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_NDIS_OUT_OF_RESOURCE: i32 = -1073736823i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_NDIS_PORT_OR_DMA_CONFLICT: i32 = -2147478632i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_NDIS_PRODUCTID_ERROR: i32 = -2147478622i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_NDIS_RECEIVE_SPACE_SMALL: i32 = 1073746837i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_NDIS_REMOVE_RECEIVED_ERROR: i32 = -2147478619i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_NDIS_RESET_FAILURE_CORRECTION: i32 = -2147478614i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_NDIS_RESET_FAILURE_ERROR: i32 = -2147478616i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_NDIS_RESOURCE_CONFLICT: i32 = -1073736824i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_NDIS_SIGNAL_LOSS_ERROR: i32 = -2147478620i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_NDIS_TIMEOUT: i32 = -2147478641i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_NDIS_TOKEN_RING_CORRECTION: i32 = 1073746854i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_NDIS_UNSUPPORTED_CONFIGURATION: i32 = -1073736815i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_PS_ADMISSIONCONTROL_OVERFLOW: i32 = -2147469537i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_PS_BAD_BESTEFFORT_LIMIT: i32 = -2147469548i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_PS_BINDING_FAILED: i32 = -1073727720i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_PS_GPC_REGISTER_FAILED: i32 = -1073727824i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_PS_INIT_DEVICE_FAILED: i32 = -1073727717i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_PS_MISSING_ADAPTER_REGISTRY_DATA: i32 = -1073727719i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_PS_NETWORK_ADDRESS_FAIL: i32 = -1073727712i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_PS_NO_RESOURCES_FOR_INIT: i32 = -1073727823i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_PS_QUERY_OID_GEN_LINK_SPEED: i32 = -1073727721i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_PS_QUERY_OID_GEN_MAXIMUM_FRAME_SIZE: i32 = -1073727723i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_PS_QUERY_OID_GEN_MAXIMUM_TOTAL_SIZE: i32 = -1073727722i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_PS_REGISTER_ADDRESS_FAMILY_FAILED: i32 = -1073727718i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_PS_REGISTER_MINIPORT_FAILED: i32 = -1073727821i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_PS_REGISTER_PROTOCOL_FAILED: i32 = -1073727822i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_PS_RESOURCE_POOL: i32 = -1073727714i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_PS_WAN_LIMITED_BESTEFFORT: i32 = -2147469539i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_PS_WMI_INSTANCE_NAME_FAILED: i32 = -1073727716i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_RDR_AT_THREAD_MAX: i32 = -2147480622i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_RDR_CANT_BIND_TRANSPORT: i32 = -2147480616i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_RDR_CANT_BUILD_SMB_HEADER: i32 = -2147480613i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_RDR_CANT_CREATE_DEVICE: i32 = -2147480646i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_RDR_CANT_CREATE_THREAD: i32 = -2147480645i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_RDR_CANT_GET_SECURITY_CONTEXT: i32 = -2147480614i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_RDR_CANT_READ_REGISTRY: i32 = -2147480621i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_RDR_CANT_REGISTER_ADDRESS: i32 = -2147480615i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_RDR_CANT_SET_THREAD: i32 = -2147480644i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_RDR_CLOSE_BEHIND: i32 = -2147480637i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_RDR_CONNECTION: i32 = -2147480629i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_RDR_CONNECTION_REFERENCE: i32 = -2147480633i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_RDR_CONTEXTS: i32 = -2147480624i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_RDR_DELAYED_SET_ATTRIBUTES_FAILED: i32 = -2147480618i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_RDR_DELETEONCLOSE_FAILED: i32 = -2147480617i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_RDR_DISPOSITION: i32 = -2147480625i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_RDR_ENCRYPT: i32 = -2147480630i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_RDR_FAILED_UNLOCK: i32 = -2147480639i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_RDR_INVALID_LOCK_REPLY: i32 = -2147480641i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_RDR_INVALID_OPLOCK: i32 = -2147480634i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_RDR_INVALID_REPLY: i32 = -2147480643i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_RDR_INVALID_SMB: i32 = -2147480642i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_RDR_MAXCMDS: i32 = -2147480627i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_RDR_OPLOCK_SMB: i32 = -2147480626i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_RDR_PRIMARY_TRANSPORT_CONNECT_FAILED: i32 = -2147480619i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_RDR_RESOURCE_SHORTAGE: i32 = -2147480647i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_RDR_SECURITY_SIGNATURE_MISMATCH: i32 = -2147480612i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_RDR_SERVER_REFERENCE: i32 = -2147480632i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_RDR_SMB_REFERENCE: i32 = -2147480631i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_RDR_TIMEOUT: i32 = -2147480635i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_RDR_TIMEZONE_BIAS_TOO_LARGE: i32 = -2147480620i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_RDR_UNEXPECTED_ERROR: i32 = -2147480636i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_RDR_WRITE_BEHIND_FLUSH_FAILED: i32 = -2147480623i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_READFILE_TIMEOUT: i32 = -1073734814i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_REVERTED_TO_LASTKNOWNGOOD: i32 = -1073734817i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_RPCSS_ACTIVATION_ERROR: i32 = -1073731817i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_RPCSS_CREATEDEBUGGERPROCESS_FAILURE: i32 = -1073731794i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_RPCSS_CREATEPROCESS_FAILURE: i32 = -1073731824i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_RPCSS_DEFAULT_LAUNCH_ACCESS_DENIED: i32 = -1073731821i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_RPCSS_LAUNCH_ACCESS_DENIED: i32 = -1073731822i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_RPCSS_REMOTE_SIDE_ERROR: i32 = -1073731818i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_RPCSS_REMOTE_SIDE_ERROR_WITH_FILE: i32 = -1073731816i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_RPCSS_REMOTE_SIDE_UNAVAILABLE: i32 = -1073731815i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_RPCSS_RUNAS_CANT_LOGIN: i32 = -1073731820i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_RPCSS_RUNAS_CREATEPROCESS_FAILURE: i32 = -1073731823i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_RPCSS_SERVER_NOT_RESPONDING: i32 = -1073731813i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_RPCSS_SERVER_START_TIMEOUT: i32 = -1073731814i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_RPCSS_START_SERVICE_FAILURE: i32 = -1073731819i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_RPCSS_STOP_SERVICE_FAILURE: i32 = -1073731795i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_RUNNING_LASTKNOWNGOOD: i32 = -1073734797i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_SCOPE_LABEL_TOO_LONG: i32 = -2147479331i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_SCOPE_TOO_LONG: i32 = -2147479330i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_SECOND_LOGON_FAILED: i32 = -1073734810i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_SERVICE_CONFIG_BACKOUT_FAILED: i32 = -1073734787i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_SERVICE_CONTROL_SUCCESS: i32 = 1073748859i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_SERVICE_CRASH: i32 = -1073734793i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_SERVICE_CRASH_NO_ACTION: i32 = -1073734790i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_SERVICE_DIFFERENT_PID_CONNECTED: i32 = -2147476609i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_SERVICE_EXIT_FAILED: i32 = -1073734801i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_SERVICE_EXIT_FAILED_SPECIFIC: i32 = -1073734800i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_SERVICE_LOGON_TYPE_NOT_GRANTED: i32 = -1073734783i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_SERVICE_NOT_INTERACTIVE: i32 = -1073734794i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_SERVICE_RECOVERY_FAILED: i32 = -1073734792i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_SERVICE_SCESRV_FAILED: i32 = -1073734791i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_SERVICE_SHUTDOWN_FAILED: i32 = -1073734781i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_SERVICE_START_AT_BOOT_FAILED: i32 = -1073734799i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_SERVICE_START_FAILED: i32 = -1073734824i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_SERVICE_START_FAILED_GROUP: i32 = -1073734822i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_SERVICE_START_FAILED_II: i32 = -1073734823i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_SERVICE_START_FAILED_NONE: i32 = -1073734821i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_SERVICE_START_HUNG: i32 = -1073734802i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_SERVICE_START_TYPE_CHANGED: i32 = 1073748864i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_SERVICE_STATUS_SUCCESS: i32 = 1073748860i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_SERVICE_STOP_SUCCESS_WITH_REASON: i32 = 1073748866i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_SEVERE_SERVICE_FAILED: i32 = -1073734803i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_SRV_CANT_BIND_DUP_NAME: i32 = -1073739319i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_SRV_CANT_BIND_TO_TRANSPORT: i32 = -2147481144i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_SRV_CANT_CHANGE_DOMAIN_NAME: i32 = -2147481136i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_SRV_CANT_CREATE_DEVICE: i32 = -1073739822i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_SRV_CANT_CREATE_PROCESS: i32 = -1073739821i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_SRV_CANT_CREATE_THREAD: i32 = -1073739820i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_SRV_CANT_GROW_TABLE: i32 = -2147481639i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_SRV_CANT_LOAD_DRIVER: i32 = -2147481140i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_SRV_CANT_MAP_ERROR: i32 = -2147481138i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_SRV_CANT_OPEN_NPFS: i32 = -1073739817i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_SRV_CANT_RECREATE_SHARE: i32 = -2147481137i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_SRV_CANT_START_SCAVENGER: i32 = -1073739814i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_SRV_CANT_UNLOAD_DRIVER: i32 = -2147481139i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_SRV_DISK_FULL: i32 = -2147481635i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_SRV_DOS_ATTACK_DETECTED: i32 = -2147481623i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_SRV_INVALID_REGISTRY_VALUE: i32 = -2147481142i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_SRV_INVALID_REQUEST: i32 = -1073739818i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_SRV_INVALID_SD: i32 = -2147481141i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_SRV_IRP_STACK_SIZE: i32 = -1073739813i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_SRV_KEY_NOT_CREATED: i32 = -1073739322i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_SRV_KEY_NOT_FOUND: i32 = -1073739323i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_SRV_NETWORK_ERROR: i32 = -2147481636i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_SRV_NONPAGED_POOL_LIMIT: i32 = -1073739807i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_SRV_NO_BLOCKING_IO: i32 = -2147481624i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_SRV_NO_FREE_CONNECTIONS: i32 = -2147481626i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_SRV_NO_FREE_RAW_WORK_ITEM: i32 = -2147481625i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_SRV_NO_NONPAGED_POOL: i32 = -1073739805i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_SRV_NO_PAGED_POOL: i32 = -1073739804i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_SRV_NO_TRANSPORTS_BOUND: i32 = -1073739321i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_SRV_NO_VIRTUAL_MEMORY: i32 = -1073739808i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_SRV_NO_WORK_ITEM: i32 = -2147481627i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_SRV_OUT_OF_WORK_ITEM_DOS: i32 = -2147481621i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_SRV_PAGED_POOL_LIMIT: i32 = -1073739806i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_SRV_RESOURCE_SHORTAGE: i32 = -1073739823i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_SRV_SERVICE_FAILED: i32 = -1073739824i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_SRV_TOO_MANY_DOS: i32 = -2147481622i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_SRV_TXF_INIT_FAILED: i32 = -2147481135i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_SRV_UNEXPECTED_DISC: i32 = -1073739819i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_STREAMS_ALLOCB_FAILURE: i32 = -2147479647i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_STREAMS_ALLOCB_FAILURE_CNT: i32 = -2147479646i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_STREAMS_ESBALLOC_FAILURE: i32 = -2147479645i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_STREAMS_ESBALLOC_FAILURE_CNT: i32 = -2147479644i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_STREAMS_STRLOG: i32 = -1073737824i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_TAKE_OWNERSHIP: i32 = -1073734796i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_TCPIP6_STARTED: i32 = 1073744924i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_TCPIP_ADAPTER_REG_FAILURE: i32 = -1073737633i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_TCPIP_ADDRESS_CONFLICT1: i32 = -1073737626i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_TCPIP_ADDRESS_CONFLICT2: i32 = -1073737625i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_TCPIP_AUTOCONFIGURED_ADDRESS_LIMIT_REACHED: i32 = -2147479444i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_TCPIP_AUTOCONFIGURED_ROUTE_LIMIT_REACHED: i32 = -2147479443i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_TCPIP_CREATE_DEVICE_FAILED: i32 = -1073737724i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_TCPIP_DHCP_INIT_FAILED: i32 = -2147479458i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_TCPIP_INTERFACE_BIND_FAILURE: i32 = -1073737617i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_TCPIP_INVALID_ADDRESS: i32 = -1073737637i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_TCPIP_INVALID_DEFAULT_GATEWAY: i32 = -2147479456i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_TCPIP_INVALID_MASK: i32 = -1073737636i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_TCPIP_IPV4_UNINSTALLED: i32 = 1073746027i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_TCPIP_IP_INIT_FAILED: i32 = -1073737628i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_TCPIP_MEDIA_CONNECT: i32 = 1073746025i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_TCPIP_MEDIA_DISCONNECT: i32 = 1073746026i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_TCPIP_NO_ADAPTER_RESOURCES: i32 = -1073737635i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_TCPIP_NO_ADDRESS_LIST: i32 = -1073737631i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_TCPIP_NO_BINDINGS: i32 = -1073737629i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_TCPIP_NO_MASK: i32 = -1073737638i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_TCPIP_NO_MASK_LIST: i32 = -1073737630i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_TCPIP_NO_RESOURCES_FOR_INIT: i32 = -1073737723i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_TCPIP_NTE_CONTEXT_LIST_FAILURE: i32 = -1073737624i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_TCPIP_OUT_OF_ORDER_FRAGMENTS_EXCEEDED: i32 = -2147479442i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_TCPIP_PCF_CLEAR_FILTER_FAILURE: i32 = -1073737530i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_TCPIP_PCF_MISSING_CAPABILITY: i32 = -2147479357i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_TCPIP_PCF_MULTICAST_OID_ISSUE: i32 = -2147479358i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_TCPIP_PCF_NO_ARP_FILTER: i32 = -2147479355i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_TCPIP_PCF_SET_FILTER_FAILURE: i32 = -2147479356i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_TCPIP_TCP_CONNECTIONS_PERF_IMPACTED: i32 = -2147479418i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_TCPIP_TCP_CONNECT_LIMIT_REACHED: i32 = -2147479422i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_TCPIP_TCP_GLOBAL_EPHEMERAL_PORT_SPACE_EXHAUSTED: i32 = -2147479417i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_TCPIP_TCP_INIT_FAILED: i32 = -1073737599i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_TCPIP_TCP_MPP_ATTACKS_DETECTED: i32 = -2147479419i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_TCPIP_TCP_TIME_WAIT_COLLISION: i32 = -2147479421i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_TCPIP_TCP_WSD_WS_RESTRICTED: i32 = -2147479420i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_TCPIP_TOO_MANY_GATEWAYS: i32 = -2147479451i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_TCPIP_TOO_MANY_NETS: i32 = -1073737639i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_TCPIP_UDP_GLOBAL_EPHEMERAL_PORT_SPACE_EXHAUSTED: i32 = -2147479382i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_TCPIP_UDP_LIMIT_REACHED: i32 = -2147479383i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_TRANSACT_INVALID: i32 = -1073734812i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_TRANSACT_TIMEOUT: i32 = -1073734813i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_TRANSPORT_ADAPTER_NOT_FOUND: i32 = -1073732818i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_TRANSPORT_BAD_PROTOCOL: i32 = 1073750835i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_TRANSPORT_BINDING_FAILED: i32 = -1073732819i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_TRANSPORT_QUERY_OID_FAILED: i32 = -1073732816i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_TRANSPORT_REGISTER_FAILED: i32 = -1073732820i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_TRANSPORT_RESOURCE_LIMIT: i32 = -2147474646i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_TRANSPORT_RESOURCE_POOL: i32 = -2147474647i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_TRANSPORT_RESOURCE_SPECIFIC: i32 = -2147474645i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_TRANSPORT_SET_OID_FAILED: i32 = -1073732817i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_TRANSPORT_TOO_MANY_LINKS: i32 = 1073750834i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_TRANSPORT_TRANSFER_DATA: i32 = 1073750833i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_TRK_INTERNAL_ERROR: i32 = -1073729324i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_TRK_SERVICE_CORRUPT_LOG: i32 = -1073729321i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_TRK_SERVICE_DUPLICATE_VOLIDS: i32 = 1073754331i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_TRK_SERVICE_MOVE_QUOTA_EXCEEDED: i32 = -2147471140i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_TRK_SERVICE_START_FAILURE: i32 = -1073729322i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_TRK_SERVICE_START_SUCCESS: i32 = 1073754325i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_TRK_SERVICE_VOLUME_CLAIM: i32 = 1073754330i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_TRK_SERVICE_VOLUME_CREATE: i32 = 1073754329i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_TRK_SERVICE_VOL_QUOTA_EXCEEDED: i32 = -2147471144i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_UP_DRIVER_ON_MP: i32 = -1073735724i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_WEBCLIENT_CLOSE_DELETE_FAILED: i32 = -2147468746i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_WEBCLIENT_CLOSE_PROPPATCH_FAILED: i32 = -2147468745i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_WEBCLIENT_CLOSE_PUT_FAILED: i32 = -2147468747i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_WEBCLIENT_SETINFO_PROPPATCH_FAILED: i32 = -2147468744i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_WINNAT_SESSION_LIMIT_REACHED: i32 = -2147466648i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_WINSOCK_CLOSESOCKET_STUCK: i32 = -2147467646i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_WINSOCK_TDI_FILTER_DETECTED: i32 = -2147467647i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVENT_WSK_OWNINGTHREAD_PARAMETER_IGNORED: i32 = -1073725824i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EVLEN: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EXTRA_EXIT_POINT: i32 = -1073727524i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EXTRA_EXIT_POINT_DELETED: i32 = -1073727520i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EXTRA_EXIT_POINT_NOT_DELETED: i32 = -1073727519i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EXTRA_VOLUME: i32 = -1073727521i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EXTRA_VOLUME_DELETED: i32 = -1073727514i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EXTRA_VOLUME_NOT_DELETED: i32 = -1073727513i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const GNLEN: u32 = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const GROUPIDMASK: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const GROUP_ALL_PARMNUM: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const GROUP_ATTRIBUTES_PARMNUM: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const GROUP_COMMENT_PARMNUM: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const GROUP_NAME_PARMNUM: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const GROUP_SPECIALGRP_ADMINS: ::windows::core::PCWSTR = ::windows::w!("ADMINS");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const GROUP_SPECIALGRP_GUESTS: ::windows::core::PCWSTR = ::windows::w!("GUESTS");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const GROUP_SPECIALGRP_LOCAL: ::windows::core::PCWSTR = ::windows::w!("LOCAL");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const GROUP_SPECIALGRP_USERS: ::windows::core::PCWSTR = ::windows::w!("USERS");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const HARDWARE_ADDRESS_LENGTH: u32 = 6u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const HELP_MSG_FILENAME: ::windows::core::PCWSTR = ::windows::w!("NETH");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const INTERFACE_INFO_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const INVALID_TRACEID: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const IPX_PROTOCOL_BASE: u32 = 131071u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const IPX_PROTOCOL_RIP: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const IR_PROMISCUOUS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const IR_PROMISCUOUS_MULTICAST: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const JOB_ADD_CURRENT_DATE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const JOB_EXEC_ERROR: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const JOB_NONINTERACTIVE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const JOB_RUNS_TODAY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const JOB_RUN_PERIODICALLY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const KNOWLEDGE_INCONSISTENCY_DETECTED: i32 = -1073727511i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const LG_INCLUDE_INDIRECT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const LM20_CNLEN: u32 = 15u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const LM20_DEVLEN: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const LM20_DNLEN: u32 = 15u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const LM20_GNLEN: u32 = 20u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const LM20_MAXCOMMENTSZ: u32 = 48u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const LM20_NNLEN: u32 = 12u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const LM20_PATHLEN: u32 = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const LM20_PWLEN: u32 = 14u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const LM20_QNLEN: u32 = 12u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const LM20_SERVICE_ACTIVE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const LM20_SERVICE_CONTINUE_PENDING: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const LM20_SERVICE_PAUSED: u32 = 12u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const LM20_SERVICE_PAUSE_PENDING: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const LM20_SNLEN: u32 = 15u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const LM20_STXTLEN: u32 = 63u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const LM20_UNCLEN: u32 = 17u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const LM20_UNLEN: u32 = 20u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const LM_REDIR_FAILURE: i32 = 1073756225i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const LOCALGROUP_COMMENT_PARMNUM: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const LOCALGROUP_NAME_PARMNUM: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const LOGFLAGS_BACKWARD: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const LOGFLAGS_FORWARD: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const LOGFLAGS_SEEK: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const LOWER_GET_HINT_MASK: u32 = 65280u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const LOWER_HINT_MASK: u32 = 255u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const MACHINE_UNJOINED: i32 = -1073727507i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const MAJOR_VERSION_MASK: u32 = 15u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const MAXCOMMENTSZ: u32 = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const MAXPERMENTRIES: u32 = 64u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const MAX_LANMAN_MESSAGE_ID: u32 = 5899u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const MAX_NERR: u32 = 2999u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const MAX_PASSWD_LEN: u32 = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const MAX_PREFERRED_LENGTH: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const MAX_PROTOCOL_DLL_LEN: u32 = 48u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const MAX_PROTOCOL_NAME_LEN: u32 = 40u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const MESSAGE_FILENAME: ::windows::core::PCWSTR = ::windows::w!("NETMSG");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const MFE_BOUNDARY_REACHED: u32 = 6u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const MFE_IIF: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const MFE_NOT_FORWARDING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const MFE_NOT_LAST_HOP: u32 = 10u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const MFE_NO_ERROR: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const MFE_NO_MULTICAST: u32 = 7u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const MFE_NO_ROUTE: u32 = 9u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const MFE_NO_SPACE: u32 = 13u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const MFE_OIF_PRUNED: u32 = 5u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const MFE_OLD_ROUTER: u32 = 11u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const MFE_PROHIBITED: u32 = 12u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const MFE_PRUNED_UPSTREAM: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const MFE_REACHED_CORE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const MFE_WRONG_IF: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const MIN_LANMAN_MESSAGE_ID: u32 = 2100u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const MISSING_EXIT_POINT: i32 = -1073727523i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const MISSING_EXIT_POINT_CREATED: i32 = -1073727518i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const MISSING_EXIT_POINT_NOT_CREATED: i32 = -1073727517i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const MISSING_VOLUME: i32 = -1073727522i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const MISSING_VOLUME_CREATED: i32 = -1073727516i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const MISSING_VOLUME_NOT_CREATED: i32 = -1073727515i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const MODALS_DOMAIN_ID_PARMNUM: u32 = 9u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const MODALS_DOMAIN_NAME_PARMNUM: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const MODALS_FORCE_LOGOFF_PARMNUM: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const MODALS_LOCKOUT_DURATION_PARMNUM: u32 = 10u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const MODALS_LOCKOUT_OBSERVATION_WINDOW_PARMNUM: u32 = 11u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const MODALS_LOCKOUT_THRESHOLD_PARMNUM: u32 = 12u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const MODALS_MAX_PASSWD_AGE_PARMNUM: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const MODALS_MIN_PASSWD_AGE_PARMNUM: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const MODALS_MIN_PASSWD_LEN_PARMNUM: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const MODALS_PASSWD_HIST_LEN_PARMNUM: u32 = 5u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const MODALS_PRIMARY_PARMNUM: u32 = 7u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const MODALS_ROLE_PARMNUM: u32 = 6u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const MRINFO_DISABLED_FLAG: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const MRINFO_DOWN_FLAG: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const MRINFO_LEAF_FLAG: u32 = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const MRINFO_PIM_FLAG: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const MRINFO_QUERIER_FLAG: u32 = 64u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const MRINFO_TUNNEL_FLAG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const MSGNAME_FORWARDED_FROM: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const MSGNAME_FORWARDED_TO: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const MSGNAME_NOT_FORWARDED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const MS_ROUTER_VERSION: u32 = 1536u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_AT_Exec_Err: u32 = 3178u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_AT_cannot_read: u32 = 3174u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_AT_cannot_write: u32 = 3129u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_AT_sched_err: u32 = 3175u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_AT_schedule_file_created: u32 = 3176u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_Access_File_Bad: u32 = 3122u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_Build_Name: u32 = 3170u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_Cant_Make_Msg_File: u32 = 3130u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_DiskFT: u32 = 3221u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_DriverNotLoaded: u32 = 5727u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_Entries_Lost: u32 = 3114u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_Error_in_DLL: u32 = 3256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_Exec_Netservr_NoMem: u32 = 3131u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_FT_ErrLog_Too_Large: u32 = 3258u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_FT_Update_In_Progress: u32 = 3259u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_FailedToGetComputerName: u32 = 5726u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_FailedToRegisterSC: u32 = 5724u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_FailedToSetServiceStatus: u32 = 5725u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_File_Changed: u32 = 3253u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_Files_Dont_Fit: u32 = 3254u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_HardErr_From_Server: u32 = 3182u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_HotFix: u32 = 3181u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_Init_Chardev_Err: u32 = 3124u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_Init_Exec_Fail: u32 = 3105u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_Init_OpenCreate_Err: u32 = 3110u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_Init_Seg_Overflow: u32 = 3120u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_Internal_Error: u32 = 3100u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_Invalid_Config_File: u32 = 3252u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_Invalid_Config_Line: u32 = 3251u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_Ioctl_Error: u32 = 3108u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_Joined_Domain: u32 = 3260u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_Joined_Workgroup: u32 = 3261u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_Lazy_Write_Err: u32 = 3180u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_LocalSecFail1: u32 = 3183u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_LocalSecFail2: u32 = 3184u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_LocalSecFail3: u32 = 3185u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_LocalSecGeneralFail: u32 = 3186u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_Mail_Slt_Err: u32 = 3173u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_Mailslot_err: u32 = 3127u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_Message_Send: u32 = 3172u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_Missing_Parameter: u32 = 3250u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_Msg_Log_Err: u32 = 3150u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_Msg_Sem_Shutdown: u32 = 3141u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_Msg_Shutdown: u32 = 3140u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_Msg_Unexpected_SMB_Type: u32 = 3152u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_Name_Expansion: u32 = 3171u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_Ncb_Error: u32 = 3106u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_Ncb_TooManyErr: u32 = 3126u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetBios: u32 = 3111u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetLogonFailedToInitializeAuthzRm: u32 = 5821u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetLogonFailedToInitializeRPCSD: u32 = 5822u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetWkSta_Internal_Error: u32 = 3190u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetWkSta_NCB_Err: u32 = 3195u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetWkSta_No_Resource: u32 = 3191u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetWkSta_Reset_Err: u32 = 3197u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetWkSta_SMB_Err: u32 = 3192u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetWkSta_Stuck_VC_Err: u32 = 3194u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetWkSta_Too_Many: u32 = 3198u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetWkSta_VC_Err: u32 = 3193u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetWkSta_Write_Behind_Err: u32 = 3196u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_Net_Not_Started: u32 = 3107u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonAddNameFailure: u32 = 5741u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonAuthDCFail: u32 = 3210u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonAuthDomainDowngraded: u32 = 5791u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonAuthNoDomainController: u32 = 5719u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonAuthNoTrustLsaSecret: u32 = 5720u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonAuthNoTrustSamAccount: u32 = 5721u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonAuthNoUplevelDomainController: u32 = 5790u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonBadSiteName: u32 = 5779u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonBadSubnetName: u32 = 5780u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonBrowserDriver: u32 = 5740u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonChangeLogCorrupt: u32 = 5705u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonDcOldSiteCovered: u32 = 5794u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonDcSiteCovered: u32 = 5784u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonDcSiteNotCovered: u32 = 5785u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonDcSiteNotCoveredAuto: u32 = 5795u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonDnsDeregAborted: u32 = 5808u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonDnsHostNameLowerCasingFailed: u32 = 5825u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonDownLevelLogoffFailed: u32 = 5708u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonDownLevelLogonFailed: u32 = 5707u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonDuplicateMachineAccounts: u32 = 5738u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonDynamicDnsDeregisterFailure: u32 = 5775u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonDynamicDnsFailure: u32 = 5782u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonDynamicDnsRegisterFailure: u32 = 5774u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonDynamicDnsServerFailure: u32 = 5781u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonFailedAccountDelta: u32 = 5735u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonFailedDnsHostNameUpdate: u32 = 5789u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonFailedDomainDelta: u32 = 5729u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonFailedFileCreate: u32 = 5776u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonFailedGlobalGroupDelta: u32 = 5730u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonFailedLocalGroupDelta: u32 = 5731u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonFailedPolicyDelta: u32 = 5733u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonFailedPrimary: u32 = 3223u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonFailedSecretDelta: u32 = 5736u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonFailedSpnUpdate: u32 = 5788u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonFailedToAddAuthzRpcInterface: u32 = 5820u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonFailedToAddRpcInterface: u32 = 5702u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonFailedToCreateShare: u32 = 5706u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonFailedToReadMailslot: u32 = 5703u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonFailedToRegisterSC: u32 = 5704u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonFailedToUpdateTrustList: u32 = 5701u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonFailedTrustedDomainDelta: u32 = 5734u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonFailedUserDelta: u32 = 5732u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonFullSyncCallFailed: u32 = 5714u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonFullSyncCallSuccess: u32 = 5713u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonFullSyncFailed: u32 = 5718u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonFullSyncSuccess: u32 = 5717u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonGcOldSiteCovered: u32 = 5796u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonGcSiteCovered: u32 = 5786u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonGcSiteNotCovered: u32 = 5787u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonGcSiteNotCoveredAuto: u32 = 5797u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonGetSubnetToSite: u32 = 5777u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonInvalidDwordParameterValue: u32 = 5804u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonInvalidGenericParameterValue: u32 = 5803u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonLanmanBdcsNotAllowed: u32 = 5772u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonMachinePasswdSetSucceeded: u32 = 5823u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonMsaPasswdSetSucceeded: u32 = 5824u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonNTLogoffFailed: u32 = 5710u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonNTLogonFailed: u32 = 5709u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonNdncOldSiteCovered: u32 = 5798u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonNdncSiteCovered: u32 = 5792u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonNdncSiteNotCovered: u32 = 5793u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonNdncSiteNotCoveredAuto: u32 = 5799u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonNoAddressToSiteMapping: u32 = 5802u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonNoDynamicDns: u32 = 5773u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonNoDynamicDnsManual: u32 = 5806u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonNoSiteForClient: u32 = 5778u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonNoSiteForClients: u32 = 5807u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonPartialSiteMappingForClients: u32 = 5810u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonPartialSyncCallFailed: u32 = 5712u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonPartialSyncCallSuccess: u32 = 5711u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonPartialSyncFailed: u32 = 5716u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonPartialSyncSuccess: u32 = 5715u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonPasswdSetFailed: u32 = 3224u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonRejectedRemoteDynamicDnsDeregister: u32 = 5814u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonRejectedRemoteDynamicDnsRegister: u32 = 5813u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonRemoteDynamicDnsDeregisterFailure: u32 = 5812u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonRemoteDynamicDnsRegisterFailure: u32 = 5811u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonRemoteDynamicDnsUpdateRequestFailure: u32 = 5815u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonRequireSignOrSealError: u32 = 3227u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonRpcCallCancelled: u32 = 5783u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonRpcPortRequestFailure: u32 = 5809u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonSSIInitError: u32 = 5700u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonServerAuthFailed: u32 = 5722u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonServerAuthFailedNoAccount: u32 = 5805u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonServerAuthNoTrustSamAccount: u32 = 5723u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonSessionTypeWrong: u32 = 5770u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonSpnCrackNamesFailure: u32 = 5801u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonSpnMultipleSamAccountNames: u32 = 5800u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonSyncError: u32 = 3226u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonSystemError: u32 = 5737u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonTooManyGlobalGroups: u32 = 5739u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonTrackingError: u32 = 3225u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonUserValidationReqInitialTimeOut: u32 = 5816u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonUserValidationReqRecurringTimeOut: u32 = 5817u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonUserValidationReqWaitInitialWarning: u32 = 5818u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NetlogonUserValidationReqWaitRecurringWarning: u32 = 5819u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_NoTranportLoaded: u32 = 5728u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_OEM_Code: u32 = 3299u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_ReleaseMem_Alert: u32 = 3128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_Remote_API: u32 = 3125u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_ReplAccessDenied: u32 = 3222u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_ReplBadExport: u32 = 3219u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_ReplBadImport: u32 = 3218u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_ReplBadMsg: u32 = 3215u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_ReplCannotMasterDir: u32 = 3207u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_ReplLogonFailed: u32 = 3211u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_ReplLostMaster: u32 = 3209u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_ReplMaxFiles: u32 = 3213u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_ReplMaxTreeDepth: u32 = 3214u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_ReplNetErr: u32 = 3212u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_ReplSignalFileErr: u32 = 3220u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_ReplSysErr: u32 = 3216u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_ReplUpdateError: u32 = 3208u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_ReplUserCurDir: u32 = 3206u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_ReplUserLoged: u32 = 3217u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_Resource_Shortage: u32 = 3101u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_RplAdapterResource: u32 = 5756u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_RplBackupDatabase: u32 = 5765u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_RplCheckConfigs: u32 = 5760u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_RplCheckSecurity: u32 = 5764u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_RplCreateProfiles: u32 = 5761u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_RplFileCopy: u32 = 5757u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_RplFileDelete: u32 = 5758u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_RplFilePerms: u32 = 5759u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_RplInitDatabase: u32 = 5766u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_RplInitRestoredDatabase: u32 = 5769u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_RplMessages: u32 = 5742u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_RplRegistry: u32 = 5762u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_RplReplaceRPLDISK: u32 = 5763u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_RplRestoreDatabaseFailure: u32 = 5767u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_RplRestoreDatabaseSuccess: u32 = 5768u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_RplSystem: u32 = 5744u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_RplUpgradeDBTo40: u32 = 5771u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_RplWkstaBbcFile: u32 = 5751u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_RplWkstaFileChecksum: u32 = 5749u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_RplWkstaFileLineCount: u32 = 5750u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_RplWkstaFileOpen: u32 = 5746u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_RplWkstaFileRead: u32 = 5747u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_RplWkstaFileSize: u32 = 5752u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_RplWkstaInternal: u32 = 5753u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_RplWkstaMemory: u32 = 5748u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_RplWkstaNetwork: u32 = 5755u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_RplWkstaTimeout: u32 = 5745u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_RplWkstaWrongVersion: u32 = 5754u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_RplXnsBoot: u32 = 5743u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_SMB_Illegal: u32 = 3112u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_Server_Lock_Failure: u32 = 3132u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_Service_Fail: u32 = 3113u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_Srv_Close_Failure: u32 = 3205u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_Srv_No_Mem_Grow: u32 = 3121u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_Srv_Thread_Failure: u32 = 3204u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_Srvnet_NB_Open: u32 = 3177u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_Srvnet_Not_Started: u32 = 3123u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_System_Error: u32 = 3257u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_System_Semaphore: u32 = 3109u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_UPS_CannotOpenDriver: u32 = 3233u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_UPS_CmdFileConfig: u32 = 3235u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_UPS_CmdFileError: u32 = 3232u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_UPS_CmdFileExec: u32 = 3236u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_UPS_PowerBack: u32 = 3234u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_UPS_PowerOut: u32 = 3230u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_UPS_Shutdown: u32 = 3231u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_Unable_To_Lock_Segment: u32 = 3102u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_Unable_To_Unlock_Segment: u32 = 3103u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_Uninstall_Service: u32 = 3104u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_VIO_POPUP_ERR: u32 = 3151u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_Wksta_Bad_Mailslot_SMB: u32 = 3165u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_Wksta_BiosThreadFailure: u32 = 3162u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_Wksta_Compname: u32 = 3161u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_Wksta_HostTab_Full: u32 = 3164u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_Wksta_Infoseg: u32 = 3160u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_Wksta_IniSeg: u32 = 3163u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_Wksta_SSIRelogon: u32 = 3167u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_Wksta_UASInit: u32 = 3166u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NELOG_Wrong_DLL_Version: u32 = 3255u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_ACFFileIOFail: u32 = 2229u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_ACFNoParent: u32 = 2232u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_ACFNoRoom: u32 = 2228u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_ACFNotFound: u32 = 2219u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_ACFNotLoaded: u32 = 2227u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_ACFTooManyLists: u32 = 2230u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_AccountExpired: u32 = 2239u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_AccountLockedOut: u32 = 2702u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_AccountUndefined: u32 = 2238u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_AcctLimitExceeded: u32 = 2434u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_ActiveConns: u32 = 2402u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_AddForwarded: u32 = 2275u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_AlertExists: u32 = 2430u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_AlreadyCloudDomainJoined: u32 = 2700u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_AlreadyExists: u32 = 2276u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_AlreadyForwarded: u32 = 2274u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_AlreadyLoggedOn: u32 = 2200u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_BASE: u32 = 2100u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_BadAsgType: u32 = 2251u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_BadComponent: u32 = 2356u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_BadControlRecv: u32 = 2193u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_BadDest: u32 = 2382u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_BadDev: u32 = 2341u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_BadDevString: u32 = 2340u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_BadDomainJoinInfo: u32 = 2712u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_BadDosFunction: u32 = 2502u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_BadDosRetCode: u32 = 2500u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_BadEventName: u32 = 2143u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_BadFileCheckSum: u32 = 2504u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_BadOfflineJoinInfo: u32 = 2710u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_BadPassword: u32 = 2203u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_BadPasswordCore: u32 = 2403u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_BadQueueDevString: u32 = 2334u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_BadQueuePriority: u32 = 2335u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_BadReceive: u32 = 2282u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_BadRecipient: u32 = 2433u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_BadServiceName: u32 = 2185u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_BadServiceProgName: u32 = 2188u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_BadSource: u32 = 2381u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_BadTransactConfig: u32 = 2141u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_BadUasConfig: u32 = 2450u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_BadUsername: u32 = 2202u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_BrowserConfiguredToNotRun: u32 = 2550u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_BrowserNotStarted: u32 = 2139u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_BrowserTableIncomplete: u32 = 2319u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_BufTooSmall: u32 = 2123u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_CallingRplSrvr: u32 = 2515u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_CanNotGrowSegment: u32 = 2233u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_CanNotGrowUASFile: u32 = 2456u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_CannotUnjoinAadDomain: u32 = 2727u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_CantConnectRplSrvr: u32 = 2513u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_CantCreateJoinInfo: u32 = 2711u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_CantLoadOfflineHive: u32 = 2717u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_CantOpenImageFile: u32 = 2514u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_CantType: u32 = 2357u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_CantVerifyHostname: u32 = 2716u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_CfgCompNotFound: u32 = 2146u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_CfgParamNotFound: u32 = 2147u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_ClientNameNotFound: u32 = 2312u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_CommDevInUse: u32 = 2343u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_ComputerAccountNotFound: u32 = 2697u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_ConnectionInsecure: u32 = 2718u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_DCNotFound: u32 = 2453u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_DS8DCNotFound: u32 = 2722u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_DS8DCRequired: u32 = 2720u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_DS9DCNotFound: u32 = 2725u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_DataTypeInvalid: u32 = 2167u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_DatabaseUpToDate: u32 = 2248u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_DefaultJoinRequired: u32 = 2694u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_DelComputerName: u32 = 2278u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_DeleteLater: u32 = 2298u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_DestExists: u32 = 2153u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_DestIdle: u32 = 2158u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_DestInvalidOp: u32 = 2159u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_DestInvalidState: u32 = 2162u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_DestNoRoom: u32 = 2157u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_DestNotFound: u32 = 2152u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_DevInUse: u32 = 2404u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_DevInvalidOpCode: u32 = 2331u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_DevNotFound: u32 = 2332u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_DevNotOpen: u32 = 2333u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_DevNotRedirected: u32 = 2107u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_DeviceIsShared: u32 = 2252u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_DeviceNotShared: u32 = 2311u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_DeviceShareConflict: u32 = 2318u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_DfsAlreadyShared: u32 = 2664u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_DfsBadRenamePath: u32 = 2671u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_DfsCantCreateJunctionPoint: u32 = 2669u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_DfsCantRemoveDfsRoot: u32 = 2682u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_DfsCantRemoveLastServerShare: u32 = 2677u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_DfsChildOrParentInDfs: u32 = 2683u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_DfsCyclicalName: u32 = 2674u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_DfsDataIsIdentical: u32 = 2681u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_DfsDuplicateService: u32 = 2676u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_DfsInconsistent: u32 = 2679u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_DfsInternalCorruption: u32 = 2660u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_DfsInternalError: u32 = 2690u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_DfsLeafVolume: u32 = 2667u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_DfsNoSuchServer: u32 = 2673u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_DfsNoSuchShare: u32 = 2665u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_DfsNoSuchVolume: u32 = 2662u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_DfsNotALeafVolume: u32 = 2666u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_DfsNotSupportedInServerDfs: u32 = 2675u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_DfsServerNotDfsAware: u32 = 2670u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_DfsServerUpgraded: u32 = 2680u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_DfsVolumeAlreadyExists: u32 = 2663u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_DfsVolumeDataCorrupt: u32 = 2661u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_DfsVolumeHasMultipleServers: u32 = 2668u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_DfsVolumeIsInterDfs: u32 = 2678u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_DfsVolumeIsOffline: u32 = 2672u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_DifferentServers: u32 = 2383u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_DriverNotFound: u32 = 2166u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_DupNameReboot: u32 = 2144u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_DuplicateName: u32 = 2297u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_DuplicateShare: u32 = 2118u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_ErrCommRunSrv: u32 = 2389u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_ErrorExecingGhost: u32 = 2391u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_ExecFailure: u32 = 2315u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_FileIdNotFound: u32 = 2314u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_GroupExists: u32 = 2223u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_GroupNotFound: u32 = 2220u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_GrpMsgProcessor: u32 = 2280u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_ImageParamErr: u32 = 2508u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_InUseBySpooler: u32 = 2342u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_IncompleteDel: u32 = 2299u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_InternalError: u32 = 2140u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_InvalidAPI: u32 = 2142u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_InvalidComputer: u32 = 2351u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_InvalidDatabase: u32 = 2247u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_InvalidDevice: u32 = 2294u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_InvalidLana: u32 = 2400u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_InvalidLogSeek: u32 = 2440u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_InvalidLogonHours: u32 = 2241u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_InvalidMachineNameForJoin: u32 = 2724u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_InvalidMaxUsers: u32 = 2122u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_InvalidUASOp: u32 = 2451u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_InvalidWorkgroupName: u32 = 2695u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_InvalidWorkstation: u32 = 2240u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_IsDfsShare: u32 = 2321u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_ItemNotFound: u32 = 2115u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_JobInvalidState: u32 = 2164u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_JobNoRoom: u32 = 2156u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_JobNotFound: u32 = 2151u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_JoinPerformedMustRestart: u32 = 2713u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_LDAPCapableDCRequired: u32 = 2721u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_LanmanIniError: u32 = 2131u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_LastAdmin: u32 = 2452u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_LineTooLong: u32 = 2149u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_LocalDrive: u32 = 2405u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_LocalForward: u32 = 2279u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_LogFileChanged: u32 = 2378u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_LogFileCorrupt: u32 = 2379u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_LogOverflow: u32 = 2377u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_LogonDomainExists: u32 = 2216u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_LogonNoUserPath: u32 = 2211u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_LogonScriptError: u32 = 2212u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_LogonServerConflict: u32 = 2210u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_LogonServerNotFound: u32 = 2215u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_LogonTrackingError: u32 = 2454u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_LogonsPaused: u32 = 2209u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_MaxLenExceeded: u32 = 2354u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_MsgAlreadyStarted: u32 = 2271u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_MsgInitFailed: u32 = 2272u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_MsgNotStarted: u32 = 2284u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_MultipleNets: u32 = 2300u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_NameInUse: u32 = 2283u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_NameNotForwarded: u32 = 2288u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_NameNotFound: u32 = 2273u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_NameUsesIncompatibleCodePage: u32 = 2696u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_NetNameNotFound: u32 = 2310u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_NetNotStarted: u32 = 2102u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_NetlogonNotStarted: u32 = 2455u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_NetworkError: u32 = 2136u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_NoAlternateServers: u32 = 2467u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_NoCommDevs: u32 = 2337u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_NoComputerName: u32 = 2270u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_NoForwardName: u32 = 2286u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_NoJoinPending: u32 = 2714u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_NoNetworkResource: u32 = 2105u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_NoOfflineJoinInfo: u32 = 2709u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_NoRoom: u32 = 2119u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_NoRplBootSystem: u32 = 2505u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_NoSuchAlert: u32 = 2432u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_NoSuchConnection: u32 = 2462u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_NoSuchServer: u32 = 2460u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_NoSuchSession: u32 = 2461u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_NonDosFloppyUsed: u32 = 2510u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_NonValidatedLogon: u32 = 2217u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_NotInCache: u32 = 2235u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_NotInDispatchTbl: u32 = 2192u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_NotLocalDomain: u32 = 2320u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_NotLocalName: u32 = 2285u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_NotLoggedOn: u32 = 2201u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_NotPrimary: u32 = 2226u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_OpenFiles: u32 = 2401u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_PasswordCantChange: u32 = 2243u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_PasswordExpired: u32 = 2242u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_PasswordFilterError: u32 = 2705u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_PasswordHistConflict: u32 = 2244u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_PasswordMismatch: u32 = 2458u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_PasswordMustChange: u32 = 2701u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_PasswordNotComplexEnough: u32 = 2704u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_PasswordTooLong: u32 = 2703u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_PasswordTooRecent: u32 = 2246u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_PasswordTooShort: u32 = 2245u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_PausedRemote: u32 = 2281u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_PersonalSku: u32 = 2698u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_PlainTextSecretsRequired: u32 = 2726u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_ProcNoRespond: u32 = 2160u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_ProcNotFound: u32 = 2168u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_ProfileCleanup: u32 = 2372u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_ProfileFileTooBig: u32 = 2370u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_ProfileLoadErr: u32 = 2374u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_ProfileOffset: u32 = 2371u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_ProfileSaveErr: u32 = 2375u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_ProfileUnknownCmd: u32 = 2373u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_ProgNeedsExtraMem: u32 = 2501u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_ProvisioningBlobUnsupported: u32 = 2719u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_QExists: u32 = 2154u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_QInvalidState: u32 = 2163u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_QNoRoom: u32 = 2155u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_QNotFound: u32 = 2150u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_QueueNotFound: u32 = 2338u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_RPL_CONNECTED: u32 = 2519u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_RedirectedPath: u32 = 2117u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_RemoteBootFailed: u32 = 2503u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_RemoteErr: u32 = 2127u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_RemoteFull: u32 = 2287u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_RemoteOnly: u32 = 2106u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_ResourceExists: u32 = 2225u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_ResourceNotFound: u32 = 2222u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_RplAdapterInfoCorrupted: u32 = 2625u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_RplAdapterNameUnavailable: u32 = 2633u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_RplAdapterNotFound: u32 = 2637u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_RplBackupDatabase: u32 = 2636u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_RplBadDatabase: u32 = 2612u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_RplBadRegistry: u32 = 2611u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_RplBootInUse: u32 = 2635u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_RplBootInfoCorrupted: u32 = 2628u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_RplBootNameUnavailable: u32 = 2640u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_RplBootNotFound: u32 = 2631u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_RplBootRestart: u32 = 2511u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_RplBootServiceTerm: u32 = 2517u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_RplBootStartFailed: u32 = 2518u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_RplCannotEnum: u32 = 2615u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_RplConfigInfoCorrupted: u32 = 2623u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_RplConfigNameUnavailable: u32 = 2641u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_RplConfigNotEmpty: u32 = 2634u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_RplConfigNotFound: u32 = 2624u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_RplIncompatibleProfile: u32 = 2632u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_RplInternal: u32 = 2626u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_RplLoadrDiskErr: u32 = 2507u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_RplLoadrNetBiosErr: u32 = 2506u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_RplNeedsRPLUSERAcct: u32 = 2630u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_RplNoAdaptersStarted: u32 = 2610u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_RplNotRplServer: u32 = 2614u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_RplProfileInfoCorrupted: u32 = 2619u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_RplProfileNameUnavailable: u32 = 2621u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_RplProfileNotEmpty: u32 = 2622u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_RplProfileNotFound: u32 = 2620u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_RplRplfilesShare: u32 = 2613u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_RplSrvrCallFailed: u32 = 2512u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_RplVendorInfoCorrupted: u32 = 2627u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_RplVendorNameUnavailable: u32 = 2639u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_RplVendorNotFound: u32 = 2638u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_RplWkstaInfoCorrupted: u32 = 2616u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_RplWkstaNameUnavailable: u32 = 2618u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_RplWkstaNeedsUserAcct: u32 = 2629u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_RplWkstaNotFound: u32 = 2617u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_RunSrvPaused: u32 = 2385u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_SameAsComputerName: u32 = 2253u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_ServerNotStarted: u32 = 2114u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_ServiceCtlBusy: u32 = 2187u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_ServiceCtlNotValid: u32 = 2191u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_ServiceCtlTimeout: u32 = 2186u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_ServiceEntryLocked: u32 = 2183u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_ServiceInstalled: u32 = 2182u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_ServiceKillProc: u32 = 2190u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_ServiceNotCtrl: u32 = 2189u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_ServiceNotInstalled: u32 = 2184u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_ServiceNotStarting: u32 = 2194u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_ServiceTableFull: u32 = 2181u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_ServiceTableLocked: u32 = 2180u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_SetupAlreadyJoined: u32 = 2691u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_SetupCheckDNSConfig: u32 = 2699u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_SetupDomainController: u32 = 2693u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_SetupNotJoined: u32 = 2692u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_ShareMem: u32 = 2104u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_ShareNotFound: u32 = 2392u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_SourceIsDir: u32 = 2380u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_SpeGroupOp: u32 = 2234u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_SpoolNoMemory: u32 = 2165u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_SpoolerNotLoaded: u32 = 2161u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_StandaloneLogon: u32 = 2214u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_StartingRplBoot: u32 = 2516u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_Success: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_SyncRequired: u32 = 2249u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_TargetVersionUnsupported: u32 = 2723u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_TimeDiffAtDC: u32 = 2457u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_TmpFile: u32 = 2316u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_TooManyAlerts: u32 = 2431u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_TooManyConnections: u32 = 2465u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_TooManyEntries: u32 = 2362u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_TooManyFiles: u32 = 2466u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_TooManyImageParams: u32 = 2509u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_TooManyItems: u32 = 2121u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_TooManyNames: u32 = 2277u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_TooManyServers: u32 = 2463u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_TooManySessions: u32 = 2464u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_TooMuchData: u32 = 2317u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_TruncatedBroadcast: u32 = 2289u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_TryDownLevel: u32 = 2470u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_UPSDriverNotStarted: u32 = 2480u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_UPSInvalidCommPort: u32 = 2482u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_UPSInvalidConfig: u32 = 2481u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_UPSShutdownFailed: u32 = 2484u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_UPSSignalAsserted: u32 = 2483u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_UnableToAddName_F: u32 = 2205u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_UnableToAddName_W: u32 = 2204u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_UnableToDelName_F: u32 = 2207u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_UnableToDelName_W: u32 = 2206u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_UnknownDevDir: u32 = 2116u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_UnknownServer: u32 = 2103u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_UseNotFound: u32 = 2250u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_UserExists: u32 = 2224u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_UserInGroup: u32 = 2236u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_UserLogon: u32 = 2231u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_UserNotFound: u32 = 2221u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_UserNotInGroup: u32 = 2237u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_ValuesNotSet: u32 = 2715u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_WkstaInconsistentState: u32 = 2137u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_WkstaNotStarted: u32 = 2138u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NERR_WriteFault: u32 = 2295u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETBIOS_NAME_LEN: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETCFG_CLIENT_CID_MS_MSClient: ::windows::core::PCWSTR = ::windows::w!("ms_msclient");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETCFG_E_ACTIVE_RAS_CONNECTIONS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147180506i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETCFG_E_ADAPTER_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147180505i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETCFG_E_ALREADY_INITIALIZED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147180512i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETCFG_E_COMPONENT_REMOVED_PENDING_REBOOT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147180504i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETCFG_E_DUPLICATE_INSTANCEID: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147180501i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETCFG_E_IN_USE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147180510i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETCFG_E_MAX_FILTER_LIMIT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147180503i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETCFG_E_NEED_REBOOT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147180507i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETCFG_E_NOT_INITIALIZED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147180511i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETCFG_E_NO_WRITE_LOCK: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147180508i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETCFG_E_VMSWITCH_ACTIVE_OVER_ADAPTER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147180502i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETCFG_SERVICE_CID_MS_NETBIOS: ::windows::core::PCWSTR = ::windows::w!("ms_netbios");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETCFG_SERVICE_CID_MS_PSCHED: ::windows::core::PCWSTR = ::windows::w!("ms_pschedpc");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETCFG_SERVICE_CID_MS_SERVER: ::windows::core::PCWSTR = ::windows::w!("ms_server");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETCFG_SERVICE_CID_MS_WLBS: ::windows::core::PCWSTR = ::windows::w!("ms_wlbs");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETCFG_S_CAUSED_SETUP_CHANGE: ::windows::core::HRESULT = ::windows::core::HRESULT(303140i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETCFG_S_COMMIT_NOW: ::windows::core::HRESULT = ::windows::core::HRESULT(303141i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETCFG_S_DISABLE_QUERY: ::windows::core::HRESULT = ::windows::core::HRESULT(303138i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETCFG_S_REBOOT: ::windows::core::HRESULT = ::windows::core::HRESULT(303136i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETCFG_S_STILL_REFERENCED: ::windows::core::HRESULT = ::windows::core::HRESULT(303139i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETCFG_TRANS_CID_MS_APPLETALK: ::windows::core::PCWSTR = ::windows::w!("ms_appletalk");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETCFG_TRANS_CID_MS_NETBEUI: ::windows::core::PCWSTR = ::windows::w!("ms_netbeui");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETCFG_TRANS_CID_MS_NETMON: ::windows::core::PCWSTR = ::windows::w!("ms_netmon");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETCFG_TRANS_CID_MS_NWIPX: ::windows::core::PCWSTR = ::windows::w!("ms_nwipx");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETCFG_TRANS_CID_MS_NWSPX: ::windows::core::PCWSTR = ::windows::w!("ms_nwspx");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETCFG_TRANS_CID_MS_TCPIP: ::windows::core::PCWSTR = ::windows::w!("ms_tcpip");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETLOGON_CONTROL_BACKUP_CHANGE_LOG: u32 = 65532u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETLOGON_CONTROL_BREAKPOINT: u32 = 65535u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETLOGON_CONTROL_CHANGE_PASSWORD: u32 = 9u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETLOGON_CONTROL_FIND_USER: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETLOGON_CONTROL_FORCE_DNS_REG: u32 = 11u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETLOGON_CONTROL_PDC_REPLICATE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETLOGON_CONTROL_QUERY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETLOGON_CONTROL_QUERY_DNS_REG: u32 = 12u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETLOGON_CONTROL_QUERY_ENC_TYPES: u32 = 13u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETLOGON_CONTROL_REDISCOVER: u32 = 5u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETLOGON_CONTROL_REPLICATE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETLOGON_CONTROL_SET_DBFLAG: u32 = 65534u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETLOGON_CONTROL_SYNCHRONIZE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETLOGON_CONTROL_TC_QUERY: u32 = 6u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETLOGON_CONTROL_TC_VERIFY: u32 = 10u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETLOGON_CONTROL_TRANSPORT_NOTIFY: u32 = 7u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETLOGON_CONTROL_TRUNCATE_LOG: u32 = 65533u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETLOGON_CONTROL_UNLOAD_NETLOGON_DLL: u32 = 65531u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETLOGON_DNS_UPDATE_FAILURE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETLOGON_FULL_SYNC_REPLICATION: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETLOGON_HAS_IP: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETLOGON_HAS_TIMESERV: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETLOGON_REDO_NEEDED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETLOGON_REPLICATION_IN_PROGRESS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETLOGON_REPLICATION_NEEDED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETLOGON_VERIFY_STATUS_RETURNED: u32 = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETLOG_NetlogonNonWindowsSupportsSecureRpc: u32 = 5826u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETLOG_NetlogonUnsecureRpcClient: u32 = 5827u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETLOG_NetlogonUnsecureRpcMachineAllowedBySsdl: u32 = 5830u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETLOG_NetlogonUnsecureRpcTrust: u32 = 5828u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETLOG_NetlogonUnsecureRpcTrustAllowedBySsdl: u32 = 5831u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETLOG_NetlogonUnsecuredRpcMachineTemporarilyAllowed: u32 = 5829u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETMAN_VARTYPE_HARDWARE_ADDRESS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETMAN_VARTYPE_STRING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETMAN_VARTYPE_ULONG: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETSETUP_ACCT_DELETE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETSETUP_ALT_SAMACCOUNTNAME: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETSETUP_DNS_NAME_CHANGES_ONLY: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETSETUP_INSTALL_INVOCATION: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETSETUP_PROVISIONING_PARAMS_CURRENT_VERSION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETSETUP_PROVISIONING_PARAMS_WIN8_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETSETUP_PROVISION_CHECK_PWD_ONLY: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETSETUP_PROVISION_PERSISTENTSITE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NET_DFS_ENUM: i32 = 1073756324i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NET_DFS_ENUMEX: i32 = 1073756325i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NET_IGNORE_UNSUPPORTED_FLAGS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NET_VALIDATE_BAD_PASSWORD_COUNT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NET_VALIDATE_BAD_PASSWORD_TIME: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NET_VALIDATE_LOCKOUT_TIME: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NET_VALIDATE_PASSWORD_HISTORY: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NET_VALIDATE_PASSWORD_HISTORY_LENGTH: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NET_VALIDATE_PASSWORD_LAST_SET: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NON_VALIDATED_LOGON: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NOT_A_DFS_PATH: i32 = 1073756224i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NO_PERMISSION_REQUIRED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NTFRSPRF_COLLECT_RPC_BINDING_ERROR_CONN: i32 = -1073728292i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NTFRSPRF_COLLECT_RPC_BINDING_ERROR_SET: i32 = -1073728293i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NTFRSPRF_COLLECT_RPC_CALL_ERROR_CONN: i32 = -1073728290i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NTFRSPRF_COLLECT_RPC_CALL_ERROR_SET: i32 = -1073728291i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NTFRSPRF_OPEN_RPC_BINDING_ERROR_CONN: i32 = -1073728296i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NTFRSPRF_OPEN_RPC_BINDING_ERROR_SET: i32 = -1073728297i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NTFRSPRF_OPEN_RPC_CALL_ERROR_CONN: i32 = -1073728294i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NTFRSPRF_OPEN_RPC_CALL_ERROR_SET: i32 = -1073728295i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NTFRSPRF_REGISTRY_ERROR_CONN: i32 = -1073728286i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NTFRSPRF_REGISTRY_ERROR_SET: i32 = -1073728287i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NTFRSPRF_VIRTUALALLOC_ERROR_CONN: i32 = -1073728288i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NTFRSPRF_VIRTUALALLOC_ERROR_SET: i32 = -1073728289i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NULL_USERSETINFO_PASSWD: ::windows::core::PCSTR = ::windows::s!("              ");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NWSAP_DISPLAY_NAME: ::windows::core::PCWSTR = ::windows::w!("NW Sap Agent");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NWSAP_EVENT_BADWANFILTER_VALUE: i32 = -1073733302i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NWSAP_EVENT_BIND_FAILED: i32 = -1073733320i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NWSAP_EVENT_CARDLISTEVENT_FAIL: i32 = -1073733301i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NWSAP_EVENT_CARDMALLOC_FAILED: i32 = -1073733316i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NWSAP_EVENT_CREATELPCEVENT_ERROR: i32 = -1073733305i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NWSAP_EVENT_CREATELPCPORT_ERROR: i32 = -1073733306i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NWSAP_EVENT_GETSOCKNAME_FAILED: i32 = -1073733319i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NWSAP_EVENT_HASHTABLE_MALLOC_FAILED: i32 = -1073733308i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NWSAP_EVENT_INVALID_FILTERNAME: i32 = -2147475123i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NWSAP_EVENT_KEY_NOT_FOUND: i32 = -1073733324i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NWSAP_EVENT_LPCHANDLEMEMORY_ERROR: i32 = -1073733303i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NWSAP_EVENT_LPCLISTENMEMORY_ERROR: i32 = -1073733304i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NWSAP_EVENT_NOCARDS: i32 = -1073733315i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NWSAP_EVENT_OPTBCASTINADDR_FAILED: i32 = -1073733317i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NWSAP_EVENT_OPTEXTENDEDADDR_FAILED: i32 = -1073733318i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NWSAP_EVENT_OPTMAXADAPTERNUM_ERROR: i32 = -1073733293i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NWSAP_EVENT_RECVSEM_FAIL: i32 = -1073733313i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NWSAP_EVENT_SDMDEVENT_FAIL: i32 = -1073733300i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NWSAP_EVENT_SENDEVENT_FAIL: i32 = -1073733312i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NWSAP_EVENT_SETOPTBCAST_FAILED: i32 = -1073733321i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NWSAP_EVENT_SOCKET_FAILED: i32 = -1073733322i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NWSAP_EVENT_STARTLPCWORKER_ERROR: i32 = -1073733307i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NWSAP_EVENT_STARTRECEIVE_ERROR: i32 = -1073733311i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NWSAP_EVENT_STARTWANCHECK_ERROR: i32 = -1073733294i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NWSAP_EVENT_STARTWANWORKER_ERROR: i32 = -1073733295i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NWSAP_EVENT_STARTWORKER_ERROR: i32 = -1073733310i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NWSAP_EVENT_TABLE_MALLOC_FAILED: i32 = -1073733309i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NWSAP_EVENT_THREADEVENT_FAIL: i32 = -1073733314i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NWSAP_EVENT_WANBIND_FAILED: i32 = -1073733296i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NWSAP_EVENT_WANEVENT_ERROR: i32 = -1073733291i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NWSAP_EVENT_WANHANDLEMEMORY_ERROR: i32 = -1073733292i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NWSAP_EVENT_WANSEM_FAIL: i32 = -1073733298i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NWSAP_EVENT_WANSOCKET_FAILED: i32 = -1073733297i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NWSAP_EVENT_WSASTARTUP_FAILED: i32 = -1073733323i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NetProvisioning: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2aa2b5fe_b846_4d07_810c_b21ee45320e3);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const OS2MSG_FILENAME: ::windows::core::PCWSTR = ::windows::w!("BASE");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const PARMNUM_ALL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const PARMNUM_BASE_INFOLEVEL: u32 = 1000u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const PARM_ERROR_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const PARM_ERROR_UNKNOWN: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const PASSWORD_EXPIRED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const PATHLEN: u32 = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const PLATFORM_ID_DOS: u32 = 300u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const PLATFORM_ID_NT: u32 = 500u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const PLATFORM_ID_OS2: u32 = 400u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const PLATFORM_ID_OSF: u32 = 600u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const PLATFORM_ID_VMS: u32 = 700u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const PREFIX_MISMATCH: i32 = -1073727510i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const PREFIX_MISMATCH_FIXED: i32 = -1073727509i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const PREFIX_MISMATCH_NOT_FIXED: i32 = -1073727508i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const PRJOB_COMPLETE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const PRJOB_DELETED: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const PRJOB_DESTNOPAPER: u32 = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const PRJOB_DESTOFFLINE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const PRJOB_DESTPAUSED: u32 = 64u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const PRJOB_DEVSTATUS: u32 = 508u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const PRJOB_ERROR: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const PRJOB_INTERV: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const PRJOB_NOTIFY: u32 = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const PRJOB_QSTATUS: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const PRJOB_QS_PAUSED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const PRJOB_QS_PRINTING: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const PRJOB_QS_QUEUED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const PRJOB_QS_SPOOLING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const PROTO_IPV6_DHCP: u32 = 999u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const PROTO_IP_ALG: u32 = 10010u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const PROTO_IP_BGMP: u32 = 11u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const PROTO_IP_BOOTP: u32 = 9999u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const PROTO_IP_DHCP_ALLOCATOR: u32 = 10004u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const PROTO_IP_DIFFSERV: u32 = 10008u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const PROTO_IP_DNS_PROXY: u32 = 10003u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const PROTO_IP_DTP: u32 = 10013u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const PROTO_IP_FTP: u32 = 10012u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const PROTO_IP_H323: u32 = 10011u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const PROTO_IP_IGMP: u32 = 10u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const PROTO_IP_MGM: u32 = 10009u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const PROTO_IP_MSDP: u32 = 9u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const PROTO_IP_NAT: u32 = 10005u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const PROTO_IP_VRRP: u32 = 112u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const PROTO_TYPE_MCAST: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const PROTO_TYPE_MS0: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const PROTO_TYPE_MS1: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const PROTO_TYPE_UCAST: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const PROTO_VENDOR_MS0: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const PROTO_VENDOR_MS1: u32 = 311u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const PROTO_VENDOR_MS2: u32 = 16383u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const PWLEN: u32 = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const QNLEN: u32 = 80u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const REGISTER_PROTOCOL_ENTRY_POINT_STRING: ::windows::core::PCSTR = ::windows::s!("RegisterProtocol");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const REPL_EXPORT_EXTENT_INFOLEVEL: u32 = 1001u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const REPL_EXPORT_INTEGRITY_INFOLEVEL: u32 = 1000u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const REPL_EXTENT_FILE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const REPL_EXTENT_TREE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const REPL_GUARDTIME_INFOLEVEL: u32 = 1002u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const REPL_INTEGRITY_FILE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const REPL_INTEGRITY_TREE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const REPL_INTERVAL_INFOLEVEL: u32 = 1000u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const REPL_PULSE_INFOLEVEL: u32 = 1001u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const REPL_RANDOM_INFOLEVEL: u32 = 1003u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const REPL_ROLE_BOTH: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const REPL_ROLE_EXPORT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const REPL_ROLE_IMPORT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const REPL_STATE_NEVER_REPLICATED: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const REPL_STATE_NO_MASTER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const REPL_STATE_NO_SYNC: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const REPL_STATE_OK: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const REPL_UNLOCK_FORCE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const REPL_UNLOCK_NOFORCE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const RF_ADD_ALL_INTERFACES: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const RF_DEMAND_UPDATE_ROUTES: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const RF_MULTICAST: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const RF_POWER: u32 = 64u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const RF_ROUTING: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const RF_ROUTINGV6: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const RIS_INTERFACE_ADDRESS_CHANGE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const RIS_INTERFACE_DISABLED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const RIS_INTERFACE_ENABLED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const RIS_INTERFACE_MEDIA_ABSENT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const RIS_INTERFACE_MEDIA_PRESENT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const ROUTING_DOMAIN_INFO_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const RTR_INFO_BLOCK_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const RTUTILS_MAX_PROTOCOL_DLL_LEN: u32 = 48u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const RTUTILS_MAX_PROTOCOL_NAME_LEN: u32 = 40u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVCE_LM20_W32TIME: ::windows::core::PCWSTR = ::windows::w!("w32time");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVER_DISPLAY_NAME: ::windows::core::PCWSTR = ::windows::w!("Server");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE2_BASE: u32 = 5600u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_ACCOUNT_FLAG_ADD_AGAINST_RODC: i32 = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_ACCOUNT_FLAG_LINK_TO_HOST_ONLY: i32 = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_ACCOUNT_FLAG_REMOVE_OFFLINE: i32 = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_ACCOUNT_FLAG_UNLINK_FROM_HOST_ONLY: i32 = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_ACCOUNT_PASSWORD: ::windows::core::PCWSTR = ::windows::w!("_SA_{262E99C9-6160-4871-ACEC-4E61736B6F21}");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_ACCOUNT_SECRET_PREFIX: ::windows::core::PCWSTR = ::windows::w!("_SC_{262E99C9-6160-4871-ACEC-4E61736B6F21}_");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_ADWS: ::windows::core::PCWSTR = ::windows::w!("ADWS");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_AFP: ::windows::core::PCWSTR = ::windows::w!("AFP");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_ALERTER: ::windows::core::PCWSTR = ::windows::w!("ALERTER");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_BASE: u32 = 3050u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_BROWSER: ::windows::core::PCWSTR = ::windows::w!("BROWSER");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_CCP_CHKPT_NUM: u32 = 255u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_CCP_NO_HINT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_CCP_QUERY_HINT: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_CCP_WAIT_TIME: u32 = 65280u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_CTRL_CONTINUE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_CTRL_INTERROGATE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_CTRL_PAUSE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_CTRL_REDIR_COMM: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_CTRL_REDIR_DISK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_CTRL_REDIR_PRINT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_CTRL_UNINSTALL: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_DHCP: ::windows::core::PCWSTR = ::windows::w!("DHCP");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_DNS_CACHE: ::windows::core::PCWSTR = ::windows::w!("DnsCache");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_DOS_ENCRYPTION: ::windows::core::PCWSTR = ::windows::w!("ENCRYPT");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_DSROLE: ::windows::core::PCWSTR = ::windows::w!("DsRoleSvc");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_INSTALLED: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_INSTALL_PENDING: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_INSTALL_STATE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_IP_CHKPT_NUM: u32 = 255u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_IP_NO_HINT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_IP_QUERY_HINT: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_IP_WAITTIME_SHIFT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_IP_WAIT_TIME: u32 = 65280u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_ISMSERV: ::windows::core::PCWSTR = ::windows::w!("IsmServ");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_KDC: ::windows::core::PCWSTR = ::windows::w!("kdc");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_LM20_AFP: ::windows::core::PCWSTR = ::windows::w!("AFP");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_LM20_ALERTER: ::windows::core::PCWSTR = ::windows::w!("ALERTER");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_LM20_BROWSER: ::windows::core::PCWSTR = ::windows::w!("BROWSER");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_LM20_DHCP: ::windows::core::PCWSTR = ::windows::w!("DHCP");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_LM20_DSROLE: ::windows::core::PCWSTR = ::windows::w!("DsRoleSvc");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_LM20_ISMSERV: ::windows::core::PCWSTR = ::windows::w!("IsmServ");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_LM20_KDC: ::windows::core::PCWSTR = ::windows::w!("kdc");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_LM20_LMHOSTS: ::windows::core::PCWSTR = ::windows::w!("LMHOSTS");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_LM20_MESSENGER: ::windows::core::PCWSTR = ::windows::w!("MESSENGER");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_LM20_NBT: ::windows::core::PCWSTR = ::windows::w!("NBT");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_LM20_NETLOGON: ::windows::core::PCWSTR = ::windows::w!("NETLOGON");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_LM20_NETPOPUP: ::windows::core::PCWSTR = ::windows::w!("NETPOPUP");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_LM20_NETRUN: ::windows::core::PCWSTR = ::windows::w!("NETRUN");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_LM20_NTDS: ::windows::core::PCWSTR = ::windows::w!("NTDS");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_LM20_NTFRS: ::windows::core::PCWSTR = ::windows::w!("NtFrs");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_LM20_NWSAP: ::windows::core::PCWSTR = ::windows::w!("NwSapAgent");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_LM20_REPL: ::windows::core::PCWSTR = ::windows::w!("REPLICATOR");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_LM20_RIPL: ::windows::core::PCWSTR = ::windows::w!("REMOTEBOOT");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_LM20_RPCLOCATOR: ::windows::core::PCWSTR = ::windows::w!("RPCLOCATOR");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_LM20_SCHEDULE: ::windows::core::PCWSTR = ::windows::w!("Schedule");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_LM20_SERVER: ::windows::core::PCWSTR = ::windows::w!("SERVER");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_LM20_SPOOLER: ::windows::core::PCWSTR = ::windows::w!("SPOOLER");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_LM20_SQLSERVER: ::windows::core::PCWSTR = ::windows::w!("SQLSERVER");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_LM20_TCPIP: ::windows::core::PCWSTR = ::windows::w!("TCPIP");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_LM20_TELNET: ::windows::core::PCWSTR = ::windows::w!("Telnet");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_LM20_TIMESOURCE: ::windows::core::PCWSTR = ::windows::w!("TIMESOURCE");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_LM20_TRKSVR: ::windows::core::PCWSTR = ::windows::w!("TrkSvr");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_LM20_TRKWKS: ::windows::core::PCWSTR = ::windows::w!("TrkWks");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_LM20_UPS: ::windows::core::PCWSTR = ::windows::w!("UPS");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_LM20_WORKSTATION: ::windows::core::PCWSTR = ::windows::w!("WORKSTATION");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_LM20_XACTSRV: ::windows::core::PCWSTR = ::windows::w!("XACTSRV");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_LMHOSTS: ::windows::core::PCWSTR = ::windows::w!("LMHOSTS");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_MAXTIME: u32 = 255u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_MESSENGER: ::windows::core::PCWSTR = ::windows::w!("MESSENGER");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_NBT: ::windows::core::PCWSTR = ::windows::w!("NBT");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_NETLOGON: ::windows::core::PCWSTR = ::windows::w!("NETLOGON");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_NETPOPUP: ::windows::core::PCWSTR = ::windows::w!("NETPOPUP");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_NETRUN: ::windows::core::PCWSTR = ::windows::w!("NETRUN");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_NOT_PAUSABLE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_NOT_UNINSTALLABLE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_NTDS: ::windows::core::PCWSTR = ::windows::w!("NTDS");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_NTFRS: ::windows::core::PCWSTR = ::windows::w!("NtFrs");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_NTIP_WAITTIME_SHIFT: u32 = 12u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_NTLMSSP: ::windows::core::PCWSTR = ::windows::w!("NtLmSsp");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_NT_MAXTIME: u32 = 65535u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_NWCS: ::windows::core::PCWSTR = ::windows::w!("NWCWorkstation");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_NWSAP: ::windows::core::PCWSTR = ::windows::w!("NwSapAgent");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_PAUSABLE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_PAUSE_STATE: u32 = 12u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_REDIR_COMM_PAUSED: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_REDIR_DISK_PAUSED: u32 = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_REDIR_PAUSED: u32 = 1792u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_REDIR_PRINT_PAUSED: u32 = 512u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_REPL: ::windows::core::PCWSTR = ::windows::w!("REPLICATOR");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_RESRV_MASK: u32 = 131071u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_RIPL: ::windows::core::PCWSTR = ::windows::w!("REMOTEBOOT");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_RPCLOCATOR: ::windows::core::PCWSTR = ::windows::w!("RPCLOCATOR");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_SCHEDULE: ::windows::core::PCWSTR = ::windows::w!("Schedule");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_SERVER: ::windows::core::PCWSTR = ::windows::w!("LanmanServer");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_SPOOLER: ::windows::core::PCWSTR = ::windows::w!("SPOOLER");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_SQLSERVER: ::windows::core::PCWSTR = ::windows::w!("SQLSERVER");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_TCPIP: ::windows::core::PCWSTR = ::windows::w!("TCPIP");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_TELNET: ::windows::core::PCWSTR = ::windows::w!("Telnet");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_TIMESOURCE: ::windows::core::PCWSTR = ::windows::w!("TIMESOURCE");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_TRKSVR: ::windows::core::PCWSTR = ::windows::w!("TrkSvr");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_TRKWKS: ::windows::core::PCWSTR = ::windows::w!("TrkWks");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_UIC_AMBIGPARM: u32 = 3058u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_UIC_BADPARMVAL: u32 = 3051u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_UIC_CONFIG: u32 = 3055u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_UIC_CONFLPARM: u32 = 3063u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_UIC_DUPPARM: u32 = 3059u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_UIC_EXEC: u32 = 3061u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_UIC_FILE: u32 = 3064u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_UIC_INTERNAL: u32 = 3057u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_UIC_KILL: u32 = 3060u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_UIC_MISSPARM: u32 = 3052u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_UIC_M_ADDPAK: u32 = 3090u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_UIC_M_ANNOUNCE: u32 = 3083u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_UIC_M_DATABASE_ERROR: u32 = 5602u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_UIC_M_DISK: u32 = 3071u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_UIC_M_ERRLOG: u32 = 3088u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_UIC_M_FILES: u32 = 3079u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_UIC_M_FILE_UW: u32 = 3089u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_UIC_M_LANGROUP: u32 = 3081u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_UIC_M_LANROOT: u32 = 3075u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_UIC_M_LAZY: u32 = 3091u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_UIC_M_LOGS: u32 = 3080u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_UIC_M_LSA_MACHINE_ACCT: u32 = 5601u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_UIC_M_MEMORY: u32 = 3070u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_UIC_M_MSGNAME: u32 = 3082u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_UIC_M_NETLOGON_AUTH: u32 = 3098u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_UIC_M_NETLOGON_DC_CFLCT: u32 = 3097u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_UIC_M_NETLOGON_MPATH: u32 = 5600u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_UIC_M_NETLOGON_NO_DC: u32 = 3096u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_UIC_M_NULL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_UIC_M_PROCESSES: u32 = 3073u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_UIC_M_REDIR: u32 = 3076u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_UIC_M_SECURITY: u32 = 3074u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_UIC_M_SEC_FILE_ERR: u32 = 3078u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_UIC_M_SERVER: u32 = 3077u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_UIC_M_SERVER_SEC_ERR: u32 = 3085u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_UIC_M_THREADS: u32 = 3072u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_UIC_M_UAS: u32 = 3084u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_UIC_M_UAS_INVALID_ROLE: u32 = 3095u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_UIC_M_UAS_MACHINE_ACCT: u32 = 3092u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_UIC_M_UAS_PROLOG: u32 = 3099u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_UIC_M_UAS_SERVERS_NMEMB: u32 = 3093u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_UIC_M_UAS_SERVERS_NOGRP: u32 = 3094u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_UIC_M_WKSTA: u32 = 3087u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_UIC_NORMAL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_UIC_RESOURCE: u32 = 3054u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_UIC_SUBSERV: u32 = 3062u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_UIC_SYSTEM: u32 = 3056u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_UIC_UNKPARM: u32 = 3053u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_UNINSTALLABLE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_UNINSTALLED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_UNINSTALL_PENDING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_UPS: ::windows::core::PCWSTR = ::windows::w!("UPS");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_W32TIME: ::windows::core::PCWSTR = ::windows::w!("w32time");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_WORKSTATION: ::windows::core::PCWSTR = ::windows::w!("LanmanWorkstation");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SERVICE_XACTSRV: ::windows::core::PCWSTR = ::windows::w!("XACTSRV");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SESSION_CRYPT_KLEN: u32 = 21u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SESSION_PWLEN: u32 = 24u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SHPWLEN: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SNLEN: u32 = 80u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SRV_HASH_GENERATION_ACTIVE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SRV_SUPPORT_HASH_GENERATION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const STXTLEN: u32 = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SUPPORTS_ANY: i32 = -1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SVAUD_BADNETLOGON: u32 = 384u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SVAUD_BADSESSLOGON: u32 = 24u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SVAUD_BADUSE: u32 = 6144u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SVAUD_GOODNETLOGON: u32 = 96u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SVAUD_GOODSESSLOGON: u32 = 6u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SVAUD_GOODUSE: u32 = 1536u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SVAUD_LOGONLIM: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SVAUD_PERMISSIONS: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SVAUD_RESOURCE: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SVAUD_SERVICE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SVAUD_USERLIST: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SVI1_NUM_ELEMENTS: u32 = 5u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SVI2_NUM_ELEMENTS: u32 = 40u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SVI3_NUM_ELEMENTS: u32 = 44u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SVTI2_CLUSTER_DNN_NAME: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SVTI2_CLUSTER_NAME: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SVTI2_REMAP_PIPE_NAMES: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SVTI2_RESERVED1: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SVTI2_RESERVED2: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SVTI2_RESERVED3: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SVTI2_SCOPED_NAME: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SVTI2_UNICODE_TRANSPORT_ADDRESS: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_ACCEPTDOWNLEVELAPIS_PARMNUM: u32 = 517u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_ACCESSALERT_PARMNUM: u32 = 40u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_ACTIVELOCKS_PARMNUM: u32 = 419u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_ALERTSCHEDULE_PARMNUM: u32 = 547u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_ALERTSCHED_PARMNUM: u32 = 37u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_ALERTS_PARMNUM: u32 = 11u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_ALIST_MTIME_PARMNUM: u32 = 403u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_ANNDELTA_PARMNUM: u32 = 18u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_ANNOUNCE_PARMNUM: u32 = 17u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_AUTOSHARESERVER_PARMNUM: u32 = 592u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_AUTOSHAREWKS_PARMNUM: u32 = 591u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_BALANCECOUNT_PARMNUM: u32 = 577u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_CACHEDDIRECTORYLIMIT_PARMNUM: u32 = 587u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_CACHEDOPENLIMIT_PARMNUM: u32 = 571u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_CHDEVJOBS_PARMNUM: u32 = 411u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_CHDEVQ_PARMNUM: u32 = 410u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_COMMENT_PARMNUM: u32 = 5u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_CONNECTIONLESSAUTODISC_PARMNUM: u32 = 562u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_CONNECTIONNOSESSIONSTIMEOUT_PARMNUM: u32 = 596u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_CONNECTIONS_PARMNUM: u32 = 412u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_CRITICALTHREADS_PARMNUM: u32 = 572u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_DISABLEDOS_PARMNUM: u32 = 600u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_DISABLESTRICTNAMECHECKING_PARMNUM: u32 = 602u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_DISC_PARMNUM: u32 = 10u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_DISKALERT_PARMNUM: u32 = 41u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_DISKSPACETHRESHOLD_PARMNUM: u32 = 550u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_DOMAIN_PARMNUM: u32 = 519u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_ENABLEAUTHENTICATEUSERSHARING_PARMNUM: u32 = 603u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_ENABLECOMPRESSION_PARMNUM: u32 = 590u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_ENABLEFCBOPENS_PARMNUM: u32 = 538u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_ENABLEFORCEDLOGOFF_PARMNUM: u32 = 515u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_ENABLEOPLOCKFORCECLOSE_PARMNUM: u32 = 537u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_ENABLEOPLOCKS_PARMNUM: u32 = 536u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_ENABLERAW_PARMNUM: u32 = 539u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_ENABLESECURITYSIGNATURE_PARMNUM: u32 = 593u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_ENABLESHAREDNETDRIVES_PARMNUM: u32 = 540u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_ENABLESOFTCOMPAT_PARMNUM: u32 = 514u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_ENABLEW9XSECURITYSIGNATURE_PARMNUM: u32 = 598u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_ENABLEWFW311DIRECTIPX_PARMNUM: u32 = 574u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_ENFORCEKERBEROSREAUTHENTICATION_PARMNUM: u32 = 599u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_ERRORALERT_PARMNUM: u32 = 38u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_ERRORTHRESHOLD_PARMNUM: u32 = 548u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_GLIST_MTIME_PARMNUM: u32 = 402u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_GUESTACC_PARMNUM: u32 = 408u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_HIDDEN_PARMNUM: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_IDLETHREADTIMEOUT_PARMNUM: u32 = 597u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_INITCONNTABLE_PARMNUM: u32 = 544u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_INITFILETABLE_PARMNUM: u32 = 545u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_INITSEARCHTABLE_PARMNUM: u32 = 546u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_INITSESSTABLE_PARMNUM: u32 = 543u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_INITWORKITEMS_PARMNUM: u32 = 505u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_IRPSTACKSIZE_PARMNUM: u32 = 508u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_LANMASK_PARMNUM: u32 = 407u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_LINKINFOVALIDTIME_PARMNUM: u32 = 554u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_LMANNOUNCE_PARMNUM: u32 = 518u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_LOCKVIOLATIONDELAY_PARMNUM: u32 = 569u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_LOCKVIOLATIONOFFSET_PARMNUM: u32 = 568u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_LOCKVIOLATIONRETRIES_PARMNUM: u32 = 567u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_LOGONALERT_PARMNUM: u32 = 39u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_LOWDISKSPACEMINIMUM_PARMNUM: u32 = 601u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_MAXAUDITSZ_PARMNUM: u32 = 43u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_MAXCOPYLENGTH_PARMNUM: u32 = 588u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_MAXCOPYREADLEN_PARMNUM: u32 = 520u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_MAXCOPYWRITELEN_PARMNUM: u32 = 521u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_MAXFREECONNECTIONS_PARMNUM: u32 = 542u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_MAXFREELFCBS_PARMNUM: u32 = 581u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_MAXFREEMFCBS_PARMNUM: u32 = 580u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_MAXFREEPAGEDPOOLCHUNKS_PARMNUM: u32 = 582u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_MAXFREERFCBS_PARMNUM: u32 = 579u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_MAXGLOBALOPENSEARCH_PARMNUM: u32 = 565u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_MAXKEEPCOMPLSEARCH_PARMNUM: u32 = 525u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_MAXKEEPSEARCH_PARMNUM: u32 = 523u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_MAXLINKDELAY_PARMNUM: u32 = 552u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_MAXMPXCT_PARMNUM: u32 = 533u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_MAXNONPAGEDMEMORYUSAGE_PARMNUM: u32 = 512u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_MAXPAGEDMEMORYUSAGE_PARMNUM: u32 = 513u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_MAXPAGEDPOOLCHUNKSIZE_PARMNUM: u32 = 584u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_MAXRAWBUFLEN_PARMNUM: u32 = 509u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_MAXRAWWORKITEMS_PARMNUM: u32 = 557u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_MAXTHREADSPERQUEUE_PARMNUM: u32 = 586u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_MAXWORKITEMIDLETIME_PARMNUM: u32 = 556u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_MAXWORKITEMS_PARMNUM: u32 = 506u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_MAX_CMD_LEN: u32 = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_MAX_SRV_HEUR_LEN: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_MDLREADSWITCHOVER_PARMNUM: u32 = 570u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_MINCLIENTBUFFERSIZE_PARMNUM: u32 = 595u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_MINFREECONNECTIONS_PARMNUM: u32 = 541u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_MINFREEWORKITEMS_PARMNUM: u32 = 530u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_MINKEEPCOMPLSEARCH_PARMNUM: u32 = 524u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_MINKEEPSEARCH_PARMNUM: u32 = 522u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_MINLINKTHROUGHPUT_PARMNUM: u32 = 553u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_MINPAGEDPOOLCHUNKSIZE_PARMNUM: u32 = 583u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_MINRCVQUEUE_PARMNUM: u32 = 529u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_NAME_PARMNUM: u32 = 102u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_NETIOALERT_PARMNUM: u32 = 42u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_NETWORKERRORTHRESHOLD_PARMNUM: u32 = 549u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_NODISC: i32 = -1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_NUMADMIN_PARMNUM: u32 = 406u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_NUMBIGBUF_PARMNUM: u32 = 422u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_NUMBLOCKTHREADS_PARMNUM: u32 = 527u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_NUMFILETASKS_PARMNUM: u32 = 423u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_NUMREQBUF_PARMNUM: u32 = 420u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_OPENFILES_PARMNUM: u32 = 414u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_OPENSEARCH_PARMNUM: u32 = 503u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_OPLOCKBREAKRESPONSEWAIT_PARMNUM: u32 = 535u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_OPLOCKBREAKWAIT_PARMNUM: u32 = 534u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_OTHERQUEUEAFFINITY_PARMNUM: u32 = 575u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_PLATFORM_ID_NT: u32 = 500u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_PLATFORM_ID_OS2: u32 = 400u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_PLATFORM_ID_PARMNUM: u32 = 101u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_PREFERREDAFFINITY_PARMNUM: u32 = 578u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_PRODUCTTYPE_PARMNUM: u32 = 560u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_QUEUESAMPLESECS_PARMNUM: u32 = 576u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_RAWWORKITEMS_PARMNUM: u32 = 507u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_REMOVEDUPLICATESEARCHES_PARMNUM: u32 = 566u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_REQUIRESECURITYSIGNATURE_PARMNUM: u32 = 594u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_RESTRICTNULLSESSACCESS_PARMNUM: u32 = 573u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_SCAVQOSINFOUPDATETIME_PARMNUM: u32 = 555u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_SCAVTIMEOUT_PARMNUM: u32 = 528u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_SECURITY_PARMNUM: u32 = 405u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_SENDSFROMPREFERREDPROCESSOR_PARMNUM: u32 = 585u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_SERVERSIZE_PARMNUM: u32 = 561u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_SESSCONNS_PARMNUM: u32 = 511u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_SESSOPENS_PARMNUM: u32 = 501u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_SESSREQS_PARMNUM: u32 = 417u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_SESSUSERS_PARMNUM: u32 = 510u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_SESSVCS_PARMNUM: u32 = 502u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_SHARES_PARMNUM: u32 = 413u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_SHARINGVIOLATIONDELAY_PARMNUM: u32 = 564u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_SHARINGVIOLATIONRETRIES_PARMNUM: u32 = 563u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_SIZREQBUF_PARMNUM: u32 = 504u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_SRVHEURISTICS_PARMNUM: u32 = 431u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_THREADCOUNTADD_PARMNUM: u32 = 526u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_THREADPRIORITY_PARMNUM: u32 = 532u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_TIMESOURCE_PARMNUM: u32 = 516u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_TYPE_PARMNUM: u32 = 105u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_ULIST_MTIME_PARMNUM: u32 = 401u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_USERPATH_PARMNUM: u32 = 112u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_USERS_PARMNUM: u32 = 107u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_USERS_PER_LICENSE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_VERSION_MAJOR_PARMNUM: u32 = 103u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_VERSION_MINOR_PARMNUM: u32 = 104u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_XACTMEMSIZE_PARMNUM: u32 = 531u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SW_AUTOPROF_LOAD_MASK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SW_AUTOPROF_SAVE_MASK: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const ServiceAccountPasswordGUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x262e99c9_6160_4871_acec_4e61736b6f21);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const TITLE_SC_MESSAGE_BOX: i32 = -1073734795i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const TRACE_NO_STDINFO: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const TRACE_NO_SYNCH: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const TRACE_USE_CONSOLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const TRACE_USE_DATE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const TRACE_USE_FILE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const TRACE_USE_MASK: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const TRACE_USE_MSEC: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const TRANSPORT_NAME_PARMNUM: u32 = 202u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const TRANSPORT_QUALITYOFSERVICE_PARMNUM: u32 = 201u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const UF_INTERDOMAIN_TRUST_ACCOUNT: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const UF_MNS_LOGON_ACCOUNT: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const UF_NORMAL_ACCOUNT: u32 = 512u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const UF_NO_AUTH_DATA_REQUIRED: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const UF_PARTIAL_SECRETS_ACCOUNT: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const UF_SERVER_TRUST_ACCOUNT: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const UF_TEMP_DUPLICATE_ACCOUNT: u32 = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const UF_USE_AES_KEYS: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const UF_WORKSTATION_TRUST_ACCOUNT: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const UNCLEN: u32 = 17u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const UNITS_PER_DAY: u32 = 24u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const UNLEN: u32 = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const UPPER_GET_HINT_MASK: u32 = 267386880u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const UPPER_HINT_MASK: u32 = 65280u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const USER_ACCT_EXPIRES_PARMNUM: u32 = 17u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const USER_AUTH_FLAGS_PARMNUM: u32 = 10u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const USER_CODE_PAGE_PARMNUM: u32 = 25u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const USER_COMMENT_PARMNUM: u32 = 7u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const USER_COUNTRY_CODE_PARMNUM: u32 = 24u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const USER_FLAGS_PARMNUM: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const USER_FULL_NAME_PARMNUM: u32 = 11u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const USER_HOME_DIR_DRIVE_PARMNUM: u32 = 53u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const USER_HOME_DIR_PARMNUM: u32 = 6u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const USER_LAST_LOGOFF_PARMNUM: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const USER_LAST_LOGON_PARMNUM: u32 = 15u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const USER_LOGON_HOURS_PARMNUM: u32 = 20u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const USER_LOGON_SERVER_PARMNUM: u32 = 23u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const USER_MAX_STORAGE_PARMNUM: u32 = 18u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const USER_NAME_PARMNUM: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const USER_NUM_LOGONS_PARMNUM: u32 = 22u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const USER_PAD_PW_COUNT_PARMNUM: u32 = 21u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const USER_PARMS_PARMNUM: u32 = 13u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const USER_PASSWORD_AGE_PARMNUM: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const USER_PASSWORD_PARMNUM: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const USER_PRIMARY_GROUP_PARMNUM: u32 = 51u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const USER_PRIV_MASK: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const USER_PRIV_PARMNUM: u32 = 5u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const USER_PROFILE: u32 = 52u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const USER_PROFILE_PARMNUM: u32 = 52u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const USER_SCRIPT_PATH_PARMNUM: u32 = 9u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const USER_UNITS_PER_WEEK_PARMNUM: u32 = 19u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const USER_USR_COMMENT_PARMNUM: u32 = 12u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const USER_WORKSTATIONS_PARMNUM: u32 = 14u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const USE_ASGTYPE_PARMNUM: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const USE_AUTHIDENTITY_PARMNUM: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const USE_CHARDEV: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const USE_CONN: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const USE_DEFAULT_CREDENTIALS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const USE_DISCONN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const USE_DOMAINNAME_PARMNUM: u32 = 6u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const USE_FLAGS_PARMNUM: u32 = 7u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const USE_FLAG_GLOBAL_MAPPING: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const USE_LOCAL_PARMNUM: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const USE_NETERR: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const USE_OK: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const USE_OPTIONS_PARMNUM: u32 = 10u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const USE_PASSWORD_PARMNUM: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const USE_PAUSED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const USE_RECONN: u32 = 5u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const USE_REMOTE_PARMNUM: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const USE_SD_PARMNUM: u32 = 9u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const USE_SESSLOST: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const USE_SPECIFIC_TRANSPORT: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const USE_USERNAME_PARMNUM: u32 = 5u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const VALIDATED_LOGON: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const VALID_LOGOFF: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WKSTA_BUFFERNAMEDPIPES_PARMNUM: u32 = 51u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WKSTA_BUFFERREADONLYFILES_PARMNUM: u32 = 59u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WKSTA_BUFFILESWITHDENYWRITE_PARMNUM: u32 = 58u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WKSTA_CACHEFILETIMEOUT_PARMNUM: u32 = 47u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WKSTA_CHARCOUNT_PARMNUM: u32 = 12u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WKSTA_CHARTIME_PARMNUM: u32 = 11u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WKSTA_CHARWAIT_PARMNUM: u32 = 10u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WKSTA_COMPUTERNAME_PARMNUM: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WKSTA_DORMANTFILELIMIT_PARMNUM: u32 = 46u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WKSTA_ERRLOGSZ_PARMNUM: u32 = 27u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WKSTA_FORCECORECREATEMODE_PARMNUM: u32 = 60u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WKSTA_KEEPCONN_PARMNUM: u32 = 13u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WKSTA_KEEPSEARCH_PARMNUM: u32 = 14u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WKSTA_LANGROUP_PARMNUM: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WKSTA_LANROOT_PARMNUM: u32 = 7u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WKSTA_LOCKINCREMENT_PARMNUM: u32 = 42u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WKSTA_LOCKMAXIMUM_PARMNUM: u32 = 43u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WKSTA_LOCKQUOTA_PARMNUM: u32 = 41u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WKSTA_LOGGED_ON_USERS_PARMNUM: u32 = 6u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WKSTA_LOGON_DOMAIN_PARMNUM: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WKSTA_LOGON_SERVER_PARMNUM: u32 = 9u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WKSTA_MAILSLOTS_PARMNUM: u32 = 30u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WKSTA_MAXCMDS_PARMNUM: u32 = 15u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WKSTA_MAXTHREADS_PARMNUM: u32 = 33u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WKSTA_MAXWRKCACHE_PARMNUM: u32 = 17u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WKSTA_NUMALERTS_PARMNUM: u32 = 20u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WKSTA_NUMCHARBUF_PARMNUM: u32 = 22u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WKSTA_NUMDGRAMBUF_PARMNUM: u32 = 31u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WKSTA_NUMSERVICES_PARMNUM: u32 = 21u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WKSTA_NUMWORKBUF_PARMNUM: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WKSTA_OTH_DOMAINS_PARMNUM: u32 = 101u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WKSTA_PIPEINCREMENT_PARMNUM: u32 = 44u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WKSTA_PIPEMAXIMUM_PARMNUM: u32 = 45u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WKSTA_PLATFORM_ID_PARMNUM: u32 = 100u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WKSTA_PRINTBUFTIME_PARMNUM: u32 = 28u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WKSTA_READAHEADTHRUPUT_PARMNUM: u32 = 62u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WKSTA_SESSTIMEOUT_PARMNUM: u32 = 18u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WKSTA_SIZCHARBUF_PARMNUM: u32 = 23u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WKSTA_SIZERROR_PARMNUM: u32 = 19u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WKSTA_SIZWORKBUF_PARMNUM: u32 = 29u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WKSTA_USE512BYTESMAXTRANSFER_PARMNUM: u32 = 61u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WKSTA_USECLOSEBEHIND_PARMNUM: u32 = 50u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WKSTA_USEENCRYPTION_PARMNUM: u32 = 57u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WKSTA_USELOCKANDREADANDUNLOCK_PARMNUM: u32 = 52u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WKSTA_USEOPPORTUNISTICLOCKING_PARMNUM: u32 = 48u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WKSTA_USERAWREAD_PARMNUM: u32 = 54u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WKSTA_USERAWWRITE_PARMNUM: u32 = 55u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WKSTA_USEUNLOCKBEHIND_PARMNUM: u32 = 49u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WKSTA_USEWRITERAWWITHDATA_PARMNUM: u32 = 56u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WKSTA_UTILIZENTCACHING_PARMNUM: u32 = 53u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WKSTA_VER_MAJOR_PARMNUM: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WKSTA_VER_MINOR_PARMNUM: u32 = 5u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WKSTA_WRKHEURISTICS_PARMNUM: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WORKSTATION_DISPLAY_NAME: ::windows::core::PCWSTR = ::windows::w!("Workstation");
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WZC_PROFILE_API_ERROR_FAILED_TO_LOAD_SCHEMA: u32 = 34u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WZC_PROFILE_API_ERROR_FAILED_TO_LOAD_XML: u32 = 33u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WZC_PROFILE_API_ERROR_INTERNAL: u32 = 36u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WZC_PROFILE_API_ERROR_NOT_SUPPORTED: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WZC_PROFILE_API_ERROR_XML_VALIDATION_FAILED: u32 = 35u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WZC_PROFILE_CONFIG_ERROR_1X_NOT_ALLOWED: u32 = 20u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WZC_PROFILE_CONFIG_ERROR_1X_NOT_ALLOWED_KEY_REQUIRED: u32 = 21u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WZC_PROFILE_CONFIG_ERROR_1X_NOT_ENABLED_KEY_PROVIDED: u32 = 22u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WZC_PROFILE_CONFIG_ERROR_EAP_METHOD_NOT_APPLICABLE: u32 = 24u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WZC_PROFILE_CONFIG_ERROR_EAP_METHOD_REQUIRED: u32 = 23u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WZC_PROFILE_CONFIG_ERROR_INVALID_AUTH_FOR_CONNECTION_TYPE: u32 = 15u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WZC_PROFILE_CONFIG_ERROR_INVALID_ENCRYPTION_FOR_AUTHMODE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WZC_PROFILE_CONFIG_ERROR_KEY_INDEX_NOT_APPLICABLE: u32 = 19u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WZC_PROFILE_CONFIG_ERROR_KEY_INDEX_REQUIRED: u32 = 18u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WZC_PROFILE_CONFIG_ERROR_KEY_REQUIRED: u32 = 17u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WZC_PROFILE_CONFIG_ERROR_WPA_ENCRYPTION_NOT_SUPPORTED: u32 = 26u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WZC_PROFILE_CONFIG_ERROR_WPA_NOT_SUPPORTED: u32 = 25u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WZC_PROFILE_SET_ERROR_DUPLICATE_NETWORK: u32 = 27u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WZC_PROFILE_SET_ERROR_MEMORY_ALLOCATION: u32 = 28u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WZC_PROFILE_SET_ERROR_READING_1X_CONFIG: u32 = 29u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WZC_PROFILE_SET_ERROR_WRITING_1X_CONFIG: u32 = 30u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WZC_PROFILE_SET_ERROR_WRITING_WZC_CFG: u32 = 31u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WZC_PROFILE_SUCCESS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WZC_PROFILE_XML_ERROR_1X_ENABLED: u32 = 10u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WZC_PROFILE_XML_ERROR_AUTHENTICATION: u32 = 7u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WZC_PROFILE_XML_ERROR_BAD_KEY_INDEX: u32 = 12u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WZC_PROFILE_XML_ERROR_BAD_NETWORK_KEY: u32 = 14u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WZC_PROFILE_XML_ERROR_BAD_SSID: u32 = 5u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WZC_PROFILE_XML_ERROR_BAD_VERSION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WZC_PROFILE_XML_ERROR_CONNECTION_TYPE: u32 = 6u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WZC_PROFILE_XML_ERROR_EAP_METHOD: u32 = 11u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WZC_PROFILE_XML_ERROR_ENCRYPTION: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WZC_PROFILE_XML_ERROR_KEY_INDEX_RANGE: u32 = 13u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WZC_PROFILE_XML_ERROR_KEY_PROVIDED_AUTOMATICALLY: u32 = 9u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WZC_PROFILE_XML_ERROR_NO_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WZC_PROFILE_XML_ERROR_SSID_NOT_FOUND: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const WZC_PROFILE_XML_ERROR_UNSUPPORTED_VERSION: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AF_OP(pub u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const AF_OP_PRINT: AF_OP = AF_OP(1u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const AF_OP_COMM: AF_OP = AF_OP(2u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const AF_OP_SERVER: AF_OP = AF_OP(4u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const AF_OP_ACCOUNTS: AF_OP = AF_OP(8u32);
impl ::core::marker::Copy for AF_OP {}
impl ::core::clone::Clone for AF_OP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AF_OP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for AF_OP {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for AF_OP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AF_OP").field(&self.0).finish()
    }
}
impl AF_OP {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for AF_OP {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for AF_OP {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for AF_OP {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for AF_OP {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for AF_OP {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct BIND_FLAGS1(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NCN_ADD: BIND_FLAGS1 = BIND_FLAGS1(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NCN_REMOVE: BIND_FLAGS1 = BIND_FLAGS1(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NCN_UPDATE: BIND_FLAGS1 = BIND_FLAGS1(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NCN_ENABLE: BIND_FLAGS1 = BIND_FLAGS1(16i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NCN_DISABLE: BIND_FLAGS1 = BIND_FLAGS1(32i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NCN_BINDING_PATH: BIND_FLAGS1 = BIND_FLAGS1(256i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NCN_PROPERTYCHANGE: BIND_FLAGS1 = BIND_FLAGS1(512i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NCN_NET: BIND_FLAGS1 = BIND_FLAGS1(65536i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NCN_NETTRANS: BIND_FLAGS1 = BIND_FLAGS1(131072i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NCN_NETCLIENT: BIND_FLAGS1 = BIND_FLAGS1(262144i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NCN_NETSERVICE: BIND_FLAGS1 = BIND_FLAGS1(524288i32);
impl ::core::marker::Copy for BIND_FLAGS1 {}
impl ::core::clone::Clone for BIND_FLAGS1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BIND_FLAGS1 {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for BIND_FLAGS1 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for BIND_FLAGS1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BIND_FLAGS1").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct COMPONENT_CHARACTERISTICS(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NCF_VIRTUAL: COMPONENT_CHARACTERISTICS = COMPONENT_CHARACTERISTICS(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NCF_SOFTWARE_ENUMERATED: COMPONENT_CHARACTERISTICS = COMPONENT_CHARACTERISTICS(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NCF_PHYSICAL: COMPONENT_CHARACTERISTICS = COMPONENT_CHARACTERISTICS(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NCF_HIDDEN: COMPONENT_CHARACTERISTICS = COMPONENT_CHARACTERISTICS(8i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NCF_NO_SERVICE: COMPONENT_CHARACTERISTICS = COMPONENT_CHARACTERISTICS(16i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NCF_NOT_USER_REMOVABLE: COMPONENT_CHARACTERISTICS = COMPONENT_CHARACTERISTICS(32i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NCF_MULTIPORT_INSTANCED_ADAPTER: COMPONENT_CHARACTERISTICS = COMPONENT_CHARACTERISTICS(64i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NCF_HAS_UI: COMPONENT_CHARACTERISTICS = COMPONENT_CHARACTERISTICS(128i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NCF_SINGLE_INSTANCE: COMPONENT_CHARACTERISTICS = COMPONENT_CHARACTERISTICS(256i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NCF_FILTER: COMPONENT_CHARACTERISTICS = COMPONENT_CHARACTERISTICS(1024i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NCF_DONTEXPOSELOWER: COMPONENT_CHARACTERISTICS = COMPONENT_CHARACTERISTICS(4096i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NCF_HIDE_BINDING: COMPONENT_CHARACTERISTICS = COMPONENT_CHARACTERISTICS(8192i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NCF_NDIS_PROTOCOL: COMPONENT_CHARACTERISTICS = COMPONENT_CHARACTERISTICS(16384i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NCF_FIXED_BINDING: COMPONENT_CHARACTERISTICS = COMPONENT_CHARACTERISTICS(131072i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NCF_LW_FILTER: COMPONENT_CHARACTERISTICS = COMPONENT_CHARACTERISTICS(262144i32);
impl ::core::marker::Copy for COMPONENT_CHARACTERISTICS {}
impl ::core::clone::Clone for COMPONENT_CHARACTERISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COMPONENT_CHARACTERISTICS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for COMPONENT_CHARACTERISTICS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for COMPONENT_CHARACTERISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COMPONENT_CHARACTERISTICS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DEFAULT_PAGES(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const DPP_ADVANCED: DEFAULT_PAGES = DEFAULT_PAGES(1i32);
impl ::core::marker::Copy for DEFAULT_PAGES {}
impl ::core::clone::Clone for DEFAULT_PAGES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DEFAULT_PAGES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DEFAULT_PAGES {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DEFAULT_PAGES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DEFAULT_PAGES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DSREG_JOIN_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const DSREG_UNKNOWN_JOIN: DSREG_JOIN_TYPE = DSREG_JOIN_TYPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const DSREG_DEVICE_JOIN: DSREG_JOIN_TYPE = DSREG_JOIN_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const DSREG_WORKPLACE_JOIN: DSREG_JOIN_TYPE = DSREG_JOIN_TYPE(2i32);
impl ::core::marker::Copy for DSREG_JOIN_TYPE {}
impl ::core::clone::Clone for DSREG_JOIN_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DSREG_JOIN_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DSREG_JOIN_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DSREG_JOIN_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DSREG_JOIN_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ENUM_BINDING_PATHS_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EBP_ABOVE: ENUM_BINDING_PATHS_FLAGS = ENUM_BINDING_PATHS_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const EBP_BELOW: ENUM_BINDING_PATHS_FLAGS = ENUM_BINDING_PATHS_FLAGS(2i32);
impl ::core::marker::Copy for ENUM_BINDING_PATHS_FLAGS {}
impl ::core::clone::Clone for ENUM_BINDING_PATHS_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ENUM_BINDING_PATHS_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for ENUM_BINDING_PATHS_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for ENUM_BINDING_PATHS_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ENUM_BINDING_PATHS_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FORCE_LEVEL_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const USE_NOFORCE: FORCE_LEVEL_FLAGS = FORCE_LEVEL_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const USE_FORCE: FORCE_LEVEL_FLAGS = FORCE_LEVEL_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const USE_LOTS_OF_FORCE: FORCE_LEVEL_FLAGS = FORCE_LEVEL_FLAGS(2u32);
impl ::core::marker::Copy for FORCE_LEVEL_FLAGS {}
impl ::core::clone::Clone for FORCE_LEVEL_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FORCE_LEVEL_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for FORCE_LEVEL_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for FORCE_LEVEL_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FORCE_LEVEL_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MSA_INFO_LEVEL(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const MsaInfoLevel0: MSA_INFO_LEVEL = MSA_INFO_LEVEL(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const MsaInfoLevelMax: MSA_INFO_LEVEL = MSA_INFO_LEVEL(1i32);
impl ::core::marker::Copy for MSA_INFO_LEVEL {}
impl ::core::clone::Clone for MSA_INFO_LEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MSA_INFO_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MSA_INFO_LEVEL {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MSA_INFO_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MSA_INFO_LEVEL").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MSA_INFO_STATE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const MsaInfoNotExist: MSA_INFO_STATE = MSA_INFO_STATE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const MsaInfoNotService: MSA_INFO_STATE = MSA_INFO_STATE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const MsaInfoCannotInstall: MSA_INFO_STATE = MSA_INFO_STATE(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const MsaInfoCanInstall: MSA_INFO_STATE = MSA_INFO_STATE(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const MsaInfoInstalled: MSA_INFO_STATE = MSA_INFO_STATE(5i32);
impl ::core::marker::Copy for MSA_INFO_STATE {}
impl ::core::clone::Clone for MSA_INFO_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MSA_INFO_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MSA_INFO_STATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MSA_INFO_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MSA_INFO_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NCPNP_RECONFIG_LAYER(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NCRL_NDIS: NCPNP_RECONFIG_LAYER = NCPNP_RECONFIG_LAYER(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NCRL_TDI: NCPNP_RECONFIG_LAYER = NCPNP_RECONFIG_LAYER(2i32);
impl ::core::marker::Copy for NCPNP_RECONFIG_LAYER {}
impl ::core::clone::Clone for NCPNP_RECONFIG_LAYER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NCPNP_RECONFIG_LAYER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for NCPNP_RECONFIG_LAYER {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for NCPNP_RECONFIG_LAYER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NCPNP_RECONFIG_LAYER").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NCRP_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NCRP_QUERY_PROPERTY_UI: NCRP_FLAGS = NCRP_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NCRP_SHOW_PROPERTY_UI: NCRP_FLAGS = NCRP_FLAGS(2i32);
impl ::core::marker::Copy for NCRP_FLAGS {}
impl ::core::clone::Clone for NCRP_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NCRP_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for NCRP_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for NCRP_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NCRP_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NETSETUP_JOIN_STATUS(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NetSetupUnknownStatus: NETSETUP_JOIN_STATUS = NETSETUP_JOIN_STATUS(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NetSetupUnjoined: NETSETUP_JOIN_STATUS = NETSETUP_JOIN_STATUS(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NetSetupWorkgroupName: NETSETUP_JOIN_STATUS = NETSETUP_JOIN_STATUS(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NetSetupDomainName: NETSETUP_JOIN_STATUS = NETSETUP_JOIN_STATUS(3i32);
impl ::core::marker::Copy for NETSETUP_JOIN_STATUS {}
impl ::core::clone::Clone for NETSETUP_JOIN_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NETSETUP_JOIN_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for NETSETUP_JOIN_STATUS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for NETSETUP_JOIN_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NETSETUP_JOIN_STATUS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NETSETUP_NAME_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NetSetupUnknown: NETSETUP_NAME_TYPE = NETSETUP_NAME_TYPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NetSetupMachine: NETSETUP_NAME_TYPE = NETSETUP_NAME_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NetSetupWorkgroup: NETSETUP_NAME_TYPE = NETSETUP_NAME_TYPE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NetSetupDomain: NETSETUP_NAME_TYPE = NETSETUP_NAME_TYPE(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NetSetupNonExistentDomain: NETSETUP_NAME_TYPE = NETSETUP_NAME_TYPE(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NetSetupDnsMachine: NETSETUP_NAME_TYPE = NETSETUP_NAME_TYPE(5i32);
impl ::core::marker::Copy for NETSETUP_NAME_TYPE {}
impl ::core::clone::Clone for NETSETUP_NAME_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NETSETUP_NAME_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for NETSETUP_NAME_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for NETSETUP_NAME_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NETSETUP_NAME_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NETSETUP_PROVISION(pub u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETSETUP_PROVISION_DOWNLEVEL_PRIV_SUPPORT: NETSETUP_PROVISION = NETSETUP_PROVISION(1u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETSETUP_PROVISION_REUSE_ACCOUNT: NETSETUP_PROVISION = NETSETUP_PROVISION(2u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETSETUP_PROVISION_USE_DEFAULT_PASSWORD: NETSETUP_PROVISION = NETSETUP_PROVISION(4u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETSETUP_PROVISION_SKIP_ACCOUNT_SEARCH: NETSETUP_PROVISION = NETSETUP_PROVISION(8u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETSETUP_PROVISION_ROOT_CA_CERTS: NETSETUP_PROVISION = NETSETUP_PROVISION(16u32);
impl ::core::marker::Copy for NETSETUP_PROVISION {}
impl ::core::clone::Clone for NETSETUP_PROVISION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NETSETUP_PROVISION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for NETSETUP_PROVISION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for NETSETUP_PROVISION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NETSETUP_PROVISION").field(&self.0).finish()
    }
}
impl NETSETUP_PROVISION {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for NETSETUP_PROVISION {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for NETSETUP_PROVISION {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for NETSETUP_PROVISION {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for NETSETUP_PROVISION {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for NETSETUP_PROVISION {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NETWORK_INSTALL_TIME(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NSF_PRIMARYINSTALL: NETWORK_INSTALL_TIME = NETWORK_INSTALL_TIME(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NSF_POSTSYSINSTALL: NETWORK_INSTALL_TIME = NETWORK_INSTALL_TIME(2i32);
impl ::core::marker::Copy for NETWORK_INSTALL_TIME {}
impl ::core::clone::Clone for NETWORK_INSTALL_TIME {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NETWORK_INSTALL_TIME {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for NETWORK_INSTALL_TIME {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for NETWORK_INSTALL_TIME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NETWORK_INSTALL_TIME").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NETWORK_UPGRADE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NSF_WIN16_UPGRADE: NETWORK_UPGRADE_TYPE = NETWORK_UPGRADE_TYPE(16i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NSF_WIN95_UPGRADE: NETWORK_UPGRADE_TYPE = NETWORK_UPGRADE_TYPE(32i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NSF_WINNT_WKS_UPGRADE: NETWORK_UPGRADE_TYPE = NETWORK_UPGRADE_TYPE(64i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NSF_WINNT_SVR_UPGRADE: NETWORK_UPGRADE_TYPE = NETWORK_UPGRADE_TYPE(128i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NSF_WINNT_SBS_UPGRADE: NETWORK_UPGRADE_TYPE = NETWORK_UPGRADE_TYPE(256i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NSF_COMPONENT_UPDATE: NETWORK_UPGRADE_TYPE = NETWORK_UPGRADE_TYPE(512i32);
impl ::core::marker::Copy for NETWORK_UPGRADE_TYPE {}
impl ::core::clone::Clone for NETWORK_UPGRADE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NETWORK_UPGRADE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for NETWORK_UPGRADE_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for NETWORK_UPGRADE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NETWORK_UPGRADE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NET_COMPUTER_NAME_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NetPrimaryComputerName: NET_COMPUTER_NAME_TYPE = NET_COMPUTER_NAME_TYPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NetAlternateComputerNames: NET_COMPUTER_NAME_TYPE = NET_COMPUTER_NAME_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NetAllComputerNames: NET_COMPUTER_NAME_TYPE = NET_COMPUTER_NAME_TYPE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NetComputerNameTypeMax: NET_COMPUTER_NAME_TYPE = NET_COMPUTER_NAME_TYPE(3i32);
impl ::core::marker::Copy for NET_COMPUTER_NAME_TYPE {}
impl ::core::clone::Clone for NET_COMPUTER_NAME_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NET_COMPUTER_NAME_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for NET_COMPUTER_NAME_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for NET_COMPUTER_NAME_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NET_COMPUTER_NAME_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NET_JOIN_DOMAIN_JOIN_OPTIONS(pub u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETSETUP_JOIN_DOMAIN: NET_JOIN_DOMAIN_JOIN_OPTIONS = NET_JOIN_DOMAIN_JOIN_OPTIONS(1u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETSETUP_ACCT_CREATE: NET_JOIN_DOMAIN_JOIN_OPTIONS = NET_JOIN_DOMAIN_JOIN_OPTIONS(2u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETSETUP_WIN9X_UPGRADE: NET_JOIN_DOMAIN_JOIN_OPTIONS = NET_JOIN_DOMAIN_JOIN_OPTIONS(16u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETSETUP_DOMAIN_JOIN_IF_JOINED: NET_JOIN_DOMAIN_JOIN_OPTIONS = NET_JOIN_DOMAIN_JOIN_OPTIONS(32u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETSETUP_JOIN_UNSECURE: NET_JOIN_DOMAIN_JOIN_OPTIONS = NET_JOIN_DOMAIN_JOIN_OPTIONS(64u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETSETUP_MACHINE_PWD_PASSED: NET_JOIN_DOMAIN_JOIN_OPTIONS = NET_JOIN_DOMAIN_JOIN_OPTIONS(128u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETSETUP_DEFER_SPN_SET: NET_JOIN_DOMAIN_JOIN_OPTIONS = NET_JOIN_DOMAIN_JOIN_OPTIONS(256u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETSETUP_JOIN_DC_ACCOUNT: NET_JOIN_DOMAIN_JOIN_OPTIONS = NET_JOIN_DOMAIN_JOIN_OPTIONS(512u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETSETUP_JOIN_WITH_NEW_NAME: NET_JOIN_DOMAIN_JOIN_OPTIONS = NET_JOIN_DOMAIN_JOIN_OPTIONS(1024u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETSETUP_JOIN_READONLY: NET_JOIN_DOMAIN_JOIN_OPTIONS = NET_JOIN_DOMAIN_JOIN_OPTIONS(2048u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETSETUP_AMBIGUOUS_DC: NET_JOIN_DOMAIN_JOIN_OPTIONS = NET_JOIN_DOMAIN_JOIN_OPTIONS(4096u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETSETUP_NO_NETLOGON_CACHE: NET_JOIN_DOMAIN_JOIN_OPTIONS = NET_JOIN_DOMAIN_JOIN_OPTIONS(8192u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETSETUP_DONT_CONTROL_SERVICES: NET_JOIN_DOMAIN_JOIN_OPTIONS = NET_JOIN_DOMAIN_JOIN_OPTIONS(16384u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETSETUP_SET_MACHINE_NAME: NET_JOIN_DOMAIN_JOIN_OPTIONS = NET_JOIN_DOMAIN_JOIN_OPTIONS(32768u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETSETUP_FORCE_SPN_SET: NET_JOIN_DOMAIN_JOIN_OPTIONS = NET_JOIN_DOMAIN_JOIN_OPTIONS(65536u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETSETUP_NO_ACCT_REUSE: NET_JOIN_DOMAIN_JOIN_OPTIONS = NET_JOIN_DOMAIN_JOIN_OPTIONS(131072u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETSETUP_IGNORE_UNSUPPORTED_FLAGS: NET_JOIN_DOMAIN_JOIN_OPTIONS = NET_JOIN_DOMAIN_JOIN_OPTIONS(268435456u32);
impl ::core::marker::Copy for NET_JOIN_DOMAIN_JOIN_OPTIONS {}
impl ::core::clone::Clone for NET_JOIN_DOMAIN_JOIN_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NET_JOIN_DOMAIN_JOIN_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for NET_JOIN_DOMAIN_JOIN_OPTIONS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for NET_JOIN_DOMAIN_JOIN_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NET_JOIN_DOMAIN_JOIN_OPTIONS").field(&self.0).finish()
    }
}
impl NET_JOIN_DOMAIN_JOIN_OPTIONS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for NET_JOIN_DOMAIN_JOIN_OPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for NET_JOIN_DOMAIN_JOIN_OPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for NET_JOIN_DOMAIN_JOIN_OPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for NET_JOIN_DOMAIN_JOIN_OPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for NET_JOIN_DOMAIN_JOIN_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NET_REMOTE_COMPUTER_SUPPORTS_OPTIONS(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SUPPORTS_REMOTE_ADMIN_PROTOCOL: NET_REMOTE_COMPUTER_SUPPORTS_OPTIONS = NET_REMOTE_COMPUTER_SUPPORTS_OPTIONS(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SUPPORTS_RPC: NET_REMOTE_COMPUTER_SUPPORTS_OPTIONS = NET_REMOTE_COMPUTER_SUPPORTS_OPTIONS(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SUPPORTS_SAM_PROTOCOL: NET_REMOTE_COMPUTER_SUPPORTS_OPTIONS = NET_REMOTE_COMPUTER_SUPPORTS_OPTIONS(8i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SUPPORTS_UNICODE: NET_REMOTE_COMPUTER_SUPPORTS_OPTIONS = NET_REMOTE_COMPUTER_SUPPORTS_OPTIONS(16i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SUPPORTS_LOCAL: NET_REMOTE_COMPUTER_SUPPORTS_OPTIONS = NET_REMOTE_COMPUTER_SUPPORTS_OPTIONS(32i32);
impl ::core::marker::Copy for NET_REMOTE_COMPUTER_SUPPORTS_OPTIONS {}
impl ::core::clone::Clone for NET_REMOTE_COMPUTER_SUPPORTS_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NET_REMOTE_COMPUTER_SUPPORTS_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for NET_REMOTE_COMPUTER_SUPPORTS_OPTIONS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for NET_REMOTE_COMPUTER_SUPPORTS_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NET_REMOTE_COMPUTER_SUPPORTS_OPTIONS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NET_REQUEST_PROVISION_OPTIONS(pub u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NETSETUP_PROVISION_ONLINE_CALLER: NET_REQUEST_PROVISION_OPTIONS = NET_REQUEST_PROVISION_OPTIONS(1073741824u32);
impl ::core::marker::Copy for NET_REQUEST_PROVISION_OPTIONS {}
impl ::core::clone::Clone for NET_REQUEST_PROVISION_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NET_REQUEST_PROVISION_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for NET_REQUEST_PROVISION_OPTIONS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for NET_REQUEST_PROVISION_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NET_REQUEST_PROVISION_OPTIONS").field(&self.0).finish()
    }
}
impl NET_REQUEST_PROVISION_OPTIONS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for NET_REQUEST_PROVISION_OPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for NET_REQUEST_PROVISION_OPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for NET_REQUEST_PROVISION_OPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for NET_REQUEST_PROVISION_OPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for NET_REQUEST_PROVISION_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NET_SERVER_TYPE(pub u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_TYPE_WORKSTATION: NET_SERVER_TYPE = NET_SERVER_TYPE(1u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_TYPE_SERVER: NET_SERVER_TYPE = NET_SERVER_TYPE(2u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_TYPE_SQLSERVER: NET_SERVER_TYPE = NET_SERVER_TYPE(4u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_TYPE_DOMAIN_CTRL: NET_SERVER_TYPE = NET_SERVER_TYPE(8u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_TYPE_DOMAIN_BAKCTRL: NET_SERVER_TYPE = NET_SERVER_TYPE(16u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_TYPE_TIME_SOURCE: NET_SERVER_TYPE = NET_SERVER_TYPE(32u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_TYPE_AFP: NET_SERVER_TYPE = NET_SERVER_TYPE(64u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_TYPE_NOVELL: NET_SERVER_TYPE = NET_SERVER_TYPE(128u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_TYPE_DOMAIN_MEMBER: NET_SERVER_TYPE = NET_SERVER_TYPE(256u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_TYPE_PRINTQ_SERVER: NET_SERVER_TYPE = NET_SERVER_TYPE(512u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_TYPE_DIALIN_SERVER: NET_SERVER_TYPE = NET_SERVER_TYPE(1024u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_TYPE_XENIX_SERVER: NET_SERVER_TYPE = NET_SERVER_TYPE(2048u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_TYPE_SERVER_UNIX: NET_SERVER_TYPE = NET_SERVER_TYPE(2048u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_TYPE_NT: NET_SERVER_TYPE = NET_SERVER_TYPE(4096u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_TYPE_WFW: NET_SERVER_TYPE = NET_SERVER_TYPE(8192u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_TYPE_SERVER_MFPN: NET_SERVER_TYPE = NET_SERVER_TYPE(16384u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_TYPE_SERVER_NT: NET_SERVER_TYPE = NET_SERVER_TYPE(32768u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_TYPE_POTENTIAL_BROWSER: NET_SERVER_TYPE = NET_SERVER_TYPE(65536u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_TYPE_BACKUP_BROWSER: NET_SERVER_TYPE = NET_SERVER_TYPE(131072u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_TYPE_MASTER_BROWSER: NET_SERVER_TYPE = NET_SERVER_TYPE(262144u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_TYPE_DOMAIN_MASTER: NET_SERVER_TYPE = NET_SERVER_TYPE(524288u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_TYPE_SERVER_OSF: NET_SERVER_TYPE = NET_SERVER_TYPE(1048576u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_TYPE_SERVER_VMS: NET_SERVER_TYPE = NET_SERVER_TYPE(2097152u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_TYPE_WINDOWS: NET_SERVER_TYPE = NET_SERVER_TYPE(4194304u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_TYPE_DFS: NET_SERVER_TYPE = NET_SERVER_TYPE(8388608u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_TYPE_CLUSTER_NT: NET_SERVER_TYPE = NET_SERVER_TYPE(16777216u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_TYPE_TERMINALSERVER: NET_SERVER_TYPE = NET_SERVER_TYPE(33554432u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_TYPE_CLUSTER_VS_NT: NET_SERVER_TYPE = NET_SERVER_TYPE(67108864u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_TYPE_DCE: NET_SERVER_TYPE = NET_SERVER_TYPE(268435456u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_TYPE_ALTERNATE_XPORT: NET_SERVER_TYPE = NET_SERVER_TYPE(536870912u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_TYPE_LOCAL_LIST_ONLY: NET_SERVER_TYPE = NET_SERVER_TYPE(1073741824u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_TYPE_DOMAIN_ENUM: NET_SERVER_TYPE = NET_SERVER_TYPE(2147483648u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_TYPE_ALL: NET_SERVER_TYPE = NET_SERVER_TYPE(4294967295u32);
impl ::core::marker::Copy for NET_SERVER_TYPE {}
impl ::core::clone::Clone for NET_SERVER_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NET_SERVER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for NET_SERVER_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for NET_SERVER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NET_SERVER_TYPE").field(&self.0).finish()
    }
}
impl NET_SERVER_TYPE {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for NET_SERVER_TYPE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for NET_SERVER_TYPE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for NET_SERVER_TYPE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for NET_SERVER_TYPE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for NET_SERVER_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NET_USER_ENUM_FILTER_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const FILTER_TEMP_DUPLICATE_ACCOUNT: NET_USER_ENUM_FILTER_FLAGS = NET_USER_ENUM_FILTER_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const FILTER_NORMAL_ACCOUNT: NET_USER_ENUM_FILTER_FLAGS = NET_USER_ENUM_FILTER_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const FILTER_INTERDOMAIN_TRUST_ACCOUNT: NET_USER_ENUM_FILTER_FLAGS = NET_USER_ENUM_FILTER_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const FILTER_WORKSTATION_TRUST_ACCOUNT: NET_USER_ENUM_FILTER_FLAGS = NET_USER_ENUM_FILTER_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const FILTER_SERVER_TRUST_ACCOUNT: NET_USER_ENUM_FILTER_FLAGS = NET_USER_ENUM_FILTER_FLAGS(32u32);
impl ::core::marker::Copy for NET_USER_ENUM_FILTER_FLAGS {}
impl ::core::clone::Clone for NET_USER_ENUM_FILTER_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NET_USER_ENUM_FILTER_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for NET_USER_ENUM_FILTER_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for NET_USER_ENUM_FILTER_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NET_USER_ENUM_FILTER_FLAGS").field(&self.0).finish()
    }
}
impl NET_USER_ENUM_FILTER_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for NET_USER_ENUM_FILTER_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for NET_USER_ENUM_FILTER_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for NET_USER_ENUM_FILTER_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for NET_USER_ENUM_FILTER_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for NET_USER_ENUM_FILTER_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NET_VALIDATE_PASSWORD_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NetValidateAuthentication: NET_VALIDATE_PASSWORD_TYPE = NET_VALIDATE_PASSWORD_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NetValidatePasswordChange: NET_VALIDATE_PASSWORD_TYPE = NET_VALIDATE_PASSWORD_TYPE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NetValidatePasswordReset: NET_VALIDATE_PASSWORD_TYPE = NET_VALIDATE_PASSWORD_TYPE(3i32);
impl ::core::marker::Copy for NET_VALIDATE_PASSWORD_TYPE {}
impl ::core::clone::Clone for NET_VALIDATE_PASSWORD_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NET_VALIDATE_PASSWORD_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for NET_VALIDATE_PASSWORD_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for NET_VALIDATE_PASSWORD_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NET_VALIDATE_PASSWORD_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct OBO_TOKEN_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const OBO_USER: OBO_TOKEN_TYPE = OBO_TOKEN_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const OBO_COMPONENT: OBO_TOKEN_TYPE = OBO_TOKEN_TYPE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const OBO_SOFTWARE: OBO_TOKEN_TYPE = OBO_TOKEN_TYPE(3i32);
impl ::core::marker::Copy for OBO_TOKEN_TYPE {}
impl ::core::clone::Clone for OBO_TOKEN_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for OBO_TOKEN_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for OBO_TOKEN_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for OBO_TOKEN_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OBO_TOKEN_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RASCON_UIINFO_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const RCUIF_VPN: RASCON_UIINFO_FLAGS = RASCON_UIINFO_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const RCUIF_DEMAND_DIAL: RASCON_UIINFO_FLAGS = RASCON_UIINFO_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const RCUIF_NOT_ADMIN: RASCON_UIINFO_FLAGS = RASCON_UIINFO_FLAGS(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const RCUIF_USE_IPv4_STATICADDRESS: RASCON_UIINFO_FLAGS = RASCON_UIINFO_FLAGS(8i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const RCUIF_USE_IPv4_NAME_SERVERS: RASCON_UIINFO_FLAGS = RASCON_UIINFO_FLAGS(16i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const RCUIF_USE_IPv4_REMOTE_GATEWAY: RASCON_UIINFO_FLAGS = RASCON_UIINFO_FLAGS(32i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const RCUIF_USE_IPv4_EXPLICIT_METRIC: RASCON_UIINFO_FLAGS = RASCON_UIINFO_FLAGS(64i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const RCUIF_USE_HEADER_COMPRESSION: RASCON_UIINFO_FLAGS = RASCON_UIINFO_FLAGS(128i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const RCUIF_USE_DISABLE_REGISTER_DNS: RASCON_UIINFO_FLAGS = RASCON_UIINFO_FLAGS(256i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const RCUIF_USE_PRIVATE_DNS_SUFFIX: RASCON_UIINFO_FLAGS = RASCON_UIINFO_FLAGS(512i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const RCUIF_ENABLE_NBT: RASCON_UIINFO_FLAGS = RASCON_UIINFO_FLAGS(1024i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const RCUIF_USE_IPv6_STATICADDRESS: RASCON_UIINFO_FLAGS = RASCON_UIINFO_FLAGS(2048i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const RCUIF_USE_IPv6_NAME_SERVERS: RASCON_UIINFO_FLAGS = RASCON_UIINFO_FLAGS(4096i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const RCUIF_USE_IPv6_REMOTE_GATEWAY: RASCON_UIINFO_FLAGS = RASCON_UIINFO_FLAGS(8192i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const RCUIF_USE_IPv6_EXPLICIT_METRIC: RASCON_UIINFO_FLAGS = RASCON_UIINFO_FLAGS(16384i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const RCUIF_DISABLE_CLASS_BASED_ROUTE: RASCON_UIINFO_FLAGS = RASCON_UIINFO_FLAGS(32768i32);
impl ::core::marker::Copy for RASCON_UIINFO_FLAGS {}
impl ::core::clone::Clone for RASCON_UIINFO_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RASCON_UIINFO_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for RASCON_UIINFO_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for RASCON_UIINFO_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RASCON_UIINFO_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SERVER_INFO_HIDDEN(pub u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_VISIBLE: SERVER_INFO_HIDDEN = SERVER_INFO_HIDDEN(0u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_HIDDEN: SERVER_INFO_HIDDEN = SERVER_INFO_HIDDEN(1u32);
impl ::core::marker::Copy for SERVER_INFO_HIDDEN {}
impl ::core::clone::Clone for SERVER_INFO_HIDDEN {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SERVER_INFO_HIDDEN {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SERVER_INFO_HIDDEN {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SERVER_INFO_HIDDEN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SERVER_INFO_HIDDEN").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SERVER_INFO_SECURITY(pub u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_SHARESECURITY: SERVER_INFO_SECURITY = SERVER_INFO_SECURITY(0u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const SV_USERSECURITY: SERVER_INFO_SECURITY = SERVER_INFO_SECURITY(1u32);
impl ::core::marker::Copy for SERVER_INFO_SECURITY {}
impl ::core::clone::Clone for SERVER_INFO_SECURITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SERVER_INFO_SECURITY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SERVER_INFO_SECURITY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SERVER_INFO_SECURITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SERVER_INFO_SECURITY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SUPPORTS_BINDING_INTERFACE_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NCF_LOWER: SUPPORTS_BINDING_INTERFACE_FLAGS = SUPPORTS_BINDING_INTERFACE_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const NCF_UPPER: SUPPORTS_BINDING_INTERFACE_FLAGS = SUPPORTS_BINDING_INTERFACE_FLAGS(2i32);
impl ::core::marker::Copy for SUPPORTS_BINDING_INTERFACE_FLAGS {}
impl ::core::clone::Clone for SUPPORTS_BINDING_INTERFACE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SUPPORTS_BINDING_INTERFACE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SUPPORTS_BINDING_INTERFACE_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SUPPORTS_BINDING_INTERFACE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SUPPORTS_BINDING_INTERFACE_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TRANSPORT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const UseTransportType_None: TRANSPORT_TYPE = TRANSPORT_TYPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const UseTransportType_Wsk: TRANSPORT_TYPE = TRANSPORT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const UseTransportType_Quic: TRANSPORT_TYPE = TRANSPORT_TYPE(2i32);
impl ::core::marker::Copy for TRANSPORT_TYPE {}
impl ::core::clone::Clone for TRANSPORT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TRANSPORT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for TRANSPORT_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for TRANSPORT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TRANSPORT_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct USER_ACCOUNT_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const UF_SCRIPT: USER_ACCOUNT_FLAGS = USER_ACCOUNT_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const UF_ACCOUNTDISABLE: USER_ACCOUNT_FLAGS = USER_ACCOUNT_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const UF_HOMEDIR_REQUIRED: USER_ACCOUNT_FLAGS = USER_ACCOUNT_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const UF_PASSWD_NOTREQD: USER_ACCOUNT_FLAGS = USER_ACCOUNT_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const UF_PASSWD_CANT_CHANGE: USER_ACCOUNT_FLAGS = USER_ACCOUNT_FLAGS(64u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const UF_LOCKOUT: USER_ACCOUNT_FLAGS = USER_ACCOUNT_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const UF_DONT_EXPIRE_PASSWD: USER_ACCOUNT_FLAGS = USER_ACCOUNT_FLAGS(65536u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const UF_ENCRYPTED_TEXT_PASSWORD_ALLOWED: USER_ACCOUNT_FLAGS = USER_ACCOUNT_FLAGS(128u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const UF_NOT_DELEGATED: USER_ACCOUNT_FLAGS = USER_ACCOUNT_FLAGS(1048576u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const UF_SMARTCARD_REQUIRED: USER_ACCOUNT_FLAGS = USER_ACCOUNT_FLAGS(262144u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const UF_USE_DES_KEY_ONLY: USER_ACCOUNT_FLAGS = USER_ACCOUNT_FLAGS(2097152u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const UF_DONT_REQUIRE_PREAUTH: USER_ACCOUNT_FLAGS = USER_ACCOUNT_FLAGS(4194304u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const UF_TRUSTED_FOR_DELEGATION: USER_ACCOUNT_FLAGS = USER_ACCOUNT_FLAGS(524288u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const UF_PASSWORD_EXPIRED: USER_ACCOUNT_FLAGS = USER_ACCOUNT_FLAGS(8388608u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const UF_TRUSTED_TO_AUTHENTICATE_FOR_DELEGATION: USER_ACCOUNT_FLAGS = USER_ACCOUNT_FLAGS(16777216u32);
impl ::core::marker::Copy for USER_ACCOUNT_FLAGS {}
impl ::core::clone::Clone for USER_ACCOUNT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for USER_ACCOUNT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for USER_ACCOUNT_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for USER_ACCOUNT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("USER_ACCOUNT_FLAGS").field(&self.0).finish()
    }
}
impl USER_ACCOUNT_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for USER_ACCOUNT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for USER_ACCOUNT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for USER_ACCOUNT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for USER_ACCOUNT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for USER_ACCOUNT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct USER_MODALS_ROLES(pub u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const UAS_ROLE_STANDALONE: USER_MODALS_ROLES = USER_MODALS_ROLES(0u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const UAS_ROLE_MEMBER: USER_MODALS_ROLES = USER_MODALS_ROLES(1u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const UAS_ROLE_BACKUP: USER_MODALS_ROLES = USER_MODALS_ROLES(2u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const UAS_ROLE_PRIMARY: USER_MODALS_ROLES = USER_MODALS_ROLES(3u32);
impl ::core::marker::Copy for USER_MODALS_ROLES {}
impl ::core::clone::Clone for USER_MODALS_ROLES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for USER_MODALS_ROLES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for USER_MODALS_ROLES {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for USER_MODALS_ROLES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("USER_MODALS_ROLES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct USER_PRIV(pub u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const USER_PRIV_GUEST: USER_PRIV = USER_PRIV(0u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const USER_PRIV_USER: USER_PRIV = USER_PRIV(1u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const USER_PRIV_ADMIN: USER_PRIV = USER_PRIV(2u32);
impl ::core::marker::Copy for USER_PRIV {}
impl ::core::clone::Clone for USER_PRIV {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for USER_PRIV {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for USER_PRIV {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for USER_PRIV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("USER_PRIV").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct USE_INFO_ASG_TYPE(pub u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const USE_WILDCARD: USE_INFO_ASG_TYPE = USE_INFO_ASG_TYPE(4294967295u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const USE_DISKDEV: USE_INFO_ASG_TYPE = USE_INFO_ASG_TYPE(0u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const USE_SPOOLDEV: USE_INFO_ASG_TYPE = USE_INFO_ASG_TYPE(1u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub const USE_IPC: USE_INFO_ASG_TYPE = USE_INFO_ASG_TYPE(3u32);
impl ::core::marker::Copy for USE_INFO_ASG_TYPE {}
impl ::core::clone::Clone for USE_INFO_ASG_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for USE_INFO_ASG_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for USE_INFO_ASG_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for USE_INFO_ASG_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("USE_INFO_ASG_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct ACCESS_INFO_0 {
    pub acc0_resource_name: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for ACCESS_INFO_0 {}
impl ::core::clone::Clone for ACCESS_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACCESS_INFO_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACCESS_INFO_0").field("acc0_resource_name", &self.acc0_resource_name).finish()
    }
}
impl ::windows::core::TypeKind for ACCESS_INFO_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for ACCESS_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        self.acc0_resource_name == other.acc0_resource_name
    }
}
impl ::core::cmp::Eq for ACCESS_INFO_0 {}
impl ::core::default::Default for ACCESS_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct ACCESS_INFO_1 {
    pub acc1_resource_name: ::windows::core::PWSTR,
    pub acc1_attr: u32,
    pub acc1_count: u32,
}
impl ::core::marker::Copy for ACCESS_INFO_1 {}
impl ::core::clone::Clone for ACCESS_INFO_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACCESS_INFO_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACCESS_INFO_1").field("acc1_resource_name", &self.acc1_resource_name).field("acc1_attr", &self.acc1_attr).field("acc1_count", &self.acc1_count).finish()
    }
}
impl ::windows::core::TypeKind for ACCESS_INFO_1 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for ACCESS_INFO_1 {
    fn eq(&self, other: &Self) -> bool {
        self.acc1_resource_name == other.acc1_resource_name && self.acc1_attr == other.acc1_attr && self.acc1_count == other.acc1_count
    }
}
impl ::core::cmp::Eq for ACCESS_INFO_1 {}
impl ::core::default::Default for ACCESS_INFO_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct ACCESS_INFO_1002 {
    pub acc1002_attr: u32,
}
impl ::core::marker::Copy for ACCESS_INFO_1002 {}
impl ::core::clone::Clone for ACCESS_INFO_1002 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACCESS_INFO_1002 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACCESS_INFO_1002").field("acc1002_attr", &self.acc1002_attr).finish()
    }
}
impl ::windows::core::TypeKind for ACCESS_INFO_1002 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for ACCESS_INFO_1002 {
    fn eq(&self, other: &Self) -> bool {
        self.acc1002_attr == other.acc1002_attr
    }
}
impl ::core::cmp::Eq for ACCESS_INFO_1002 {}
impl ::core::default::Default for ACCESS_INFO_1002 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct ACCESS_LIST {
    pub acl_ugname: ::windows::core::PWSTR,
    pub acl_access: u32,
}
impl ::core::marker::Copy for ACCESS_LIST {}
impl ::core::clone::Clone for ACCESS_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACCESS_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACCESS_LIST").field("acl_ugname", &self.acl_ugname).field("acl_access", &self.acl_access).finish()
    }
}
impl ::windows::core::TypeKind for ACCESS_LIST {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for ACCESS_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.acl_ugname == other.acl_ugname && self.acl_access == other.acl_access
    }
}
impl ::core::cmp::Eq for ACCESS_LIST {}
impl ::core::default::Default for ACCESS_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct ADMIN_OTHER_INFO {
    pub alrtad_errcode: u32,
    pub alrtad_numstrings: u32,
}
impl ::core::marker::Copy for ADMIN_OTHER_INFO {}
impl ::core::clone::Clone for ADMIN_OTHER_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ADMIN_OTHER_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ADMIN_OTHER_INFO").field("alrtad_errcode", &self.alrtad_errcode).field("alrtad_numstrings", &self.alrtad_numstrings).finish()
    }
}
impl ::windows::core::TypeKind for ADMIN_OTHER_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for ADMIN_OTHER_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.alrtad_errcode == other.alrtad_errcode && self.alrtad_numstrings == other.alrtad_numstrings
    }
}
impl ::core::cmp::Eq for ADMIN_OTHER_INFO {}
impl ::core::default::Default for ADMIN_OTHER_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct AE_ACCLIM {
    pub ae_al_compname: u32,
    pub ae_al_username: u32,
    pub ae_al_resname: u32,
    pub ae_al_limit: u32,
}
impl ::core::marker::Copy for AE_ACCLIM {}
impl ::core::clone::Clone for AE_ACCLIM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AE_ACCLIM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AE_ACCLIM").field("ae_al_compname", &self.ae_al_compname).field("ae_al_username", &self.ae_al_username).field("ae_al_resname", &self.ae_al_resname).field("ae_al_limit", &self.ae_al_limit).finish()
    }
}
impl ::windows::core::TypeKind for AE_ACCLIM {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for AE_ACCLIM {
    fn eq(&self, other: &Self) -> bool {
        self.ae_al_compname == other.ae_al_compname && self.ae_al_username == other.ae_al_username && self.ae_al_resname == other.ae_al_resname && self.ae_al_limit == other.ae_al_limit
    }
}
impl ::core::cmp::Eq for AE_ACCLIM {}
impl ::core::default::Default for AE_ACCLIM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct AE_ACLMOD {
    pub ae_am_compname: u32,
    pub ae_am_username: u32,
    pub ae_am_resname: u32,
    pub ae_am_action: u32,
    pub ae_am_datalen: u32,
}
impl ::core::marker::Copy for AE_ACLMOD {}
impl ::core::clone::Clone for AE_ACLMOD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AE_ACLMOD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AE_ACLMOD").field("ae_am_compname", &self.ae_am_compname).field("ae_am_username", &self.ae_am_username).field("ae_am_resname", &self.ae_am_resname).field("ae_am_action", &self.ae_am_action).field("ae_am_datalen", &self.ae_am_datalen).finish()
    }
}
impl ::windows::core::TypeKind for AE_ACLMOD {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for AE_ACLMOD {
    fn eq(&self, other: &Self) -> bool {
        self.ae_am_compname == other.ae_am_compname && self.ae_am_username == other.ae_am_username && self.ae_am_resname == other.ae_am_resname && self.ae_am_action == other.ae_am_action && self.ae_am_datalen == other.ae_am_datalen
    }
}
impl ::core::cmp::Eq for AE_ACLMOD {}
impl ::core::default::Default for AE_ACLMOD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct AE_CLOSEFILE {
    pub ae_cf_compname: u32,
    pub ae_cf_username: u32,
    pub ae_cf_resname: u32,
    pub ae_cf_fileid: u32,
    pub ae_cf_duration: u32,
    pub ae_cf_reason: u32,
}
impl ::core::marker::Copy for AE_CLOSEFILE {}
impl ::core::clone::Clone for AE_CLOSEFILE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AE_CLOSEFILE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AE_CLOSEFILE").field("ae_cf_compname", &self.ae_cf_compname).field("ae_cf_username", &self.ae_cf_username).field("ae_cf_resname", &self.ae_cf_resname).field("ae_cf_fileid", &self.ae_cf_fileid).field("ae_cf_duration", &self.ae_cf_duration).field("ae_cf_reason", &self.ae_cf_reason).finish()
    }
}
impl ::windows::core::TypeKind for AE_CLOSEFILE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for AE_CLOSEFILE {
    fn eq(&self, other: &Self) -> bool {
        self.ae_cf_compname == other.ae_cf_compname && self.ae_cf_username == other.ae_cf_username && self.ae_cf_resname == other.ae_cf_resname && self.ae_cf_fileid == other.ae_cf_fileid && self.ae_cf_duration == other.ae_cf_duration && self.ae_cf_reason == other.ae_cf_reason
    }
}
impl ::core::cmp::Eq for AE_CLOSEFILE {}
impl ::core::default::Default for AE_CLOSEFILE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct AE_CONNREJ {
    pub ae_cr_compname: u32,
    pub ae_cr_username: u32,
    pub ae_cr_netname: u32,
    pub ae_cr_reason: u32,
}
impl ::core::marker::Copy for AE_CONNREJ {}
impl ::core::clone::Clone for AE_CONNREJ {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AE_CONNREJ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AE_CONNREJ").field("ae_cr_compname", &self.ae_cr_compname).field("ae_cr_username", &self.ae_cr_username).field("ae_cr_netname", &self.ae_cr_netname).field("ae_cr_reason", &self.ae_cr_reason).finish()
    }
}
impl ::windows::core::TypeKind for AE_CONNREJ {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for AE_CONNREJ {
    fn eq(&self, other: &Self) -> bool {
        self.ae_cr_compname == other.ae_cr_compname && self.ae_cr_username == other.ae_cr_username && self.ae_cr_netname == other.ae_cr_netname && self.ae_cr_reason == other.ae_cr_reason
    }
}
impl ::core::cmp::Eq for AE_CONNREJ {}
impl ::core::default::Default for AE_CONNREJ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct AE_CONNSTART {
    pub ae_ct_compname: u32,
    pub ae_ct_username: u32,
    pub ae_ct_netname: u32,
    pub ae_ct_connid: u32,
}
impl ::core::marker::Copy for AE_CONNSTART {}
impl ::core::clone::Clone for AE_CONNSTART {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AE_CONNSTART {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AE_CONNSTART").field("ae_ct_compname", &self.ae_ct_compname).field("ae_ct_username", &self.ae_ct_username).field("ae_ct_netname", &self.ae_ct_netname).field("ae_ct_connid", &self.ae_ct_connid).finish()
    }
}
impl ::windows::core::TypeKind for AE_CONNSTART {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for AE_CONNSTART {
    fn eq(&self, other: &Self) -> bool {
        self.ae_ct_compname == other.ae_ct_compname && self.ae_ct_username == other.ae_ct_username && self.ae_ct_netname == other.ae_ct_netname && self.ae_ct_connid == other.ae_ct_connid
    }
}
impl ::core::cmp::Eq for AE_CONNSTART {}
impl ::core::default::Default for AE_CONNSTART {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct AE_CONNSTOP {
    pub ae_cp_compname: u32,
    pub ae_cp_username: u32,
    pub ae_cp_netname: u32,
    pub ae_cp_connid: u32,
    pub ae_cp_reason: u32,
}
impl ::core::marker::Copy for AE_CONNSTOP {}
impl ::core::clone::Clone for AE_CONNSTOP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AE_CONNSTOP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AE_CONNSTOP").field("ae_cp_compname", &self.ae_cp_compname).field("ae_cp_username", &self.ae_cp_username).field("ae_cp_netname", &self.ae_cp_netname).field("ae_cp_connid", &self.ae_cp_connid).field("ae_cp_reason", &self.ae_cp_reason).finish()
    }
}
impl ::windows::core::TypeKind for AE_CONNSTOP {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for AE_CONNSTOP {
    fn eq(&self, other: &Self) -> bool {
        self.ae_cp_compname == other.ae_cp_compname && self.ae_cp_username == other.ae_cp_username && self.ae_cp_netname == other.ae_cp_netname && self.ae_cp_connid == other.ae_cp_connid && self.ae_cp_reason == other.ae_cp_reason
    }
}
impl ::core::cmp::Eq for AE_CONNSTOP {}
impl ::core::default::Default for AE_CONNSTOP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct AE_GENERIC {
    pub ae_ge_msgfile: u32,
    pub ae_ge_msgnum: u32,
    pub ae_ge_params: u32,
    pub ae_ge_param1: u32,
    pub ae_ge_param2: u32,
    pub ae_ge_param3: u32,
    pub ae_ge_param4: u32,
    pub ae_ge_param5: u32,
    pub ae_ge_param6: u32,
    pub ae_ge_param7: u32,
    pub ae_ge_param8: u32,
    pub ae_ge_param9: u32,
}
impl ::core::marker::Copy for AE_GENERIC {}
impl ::core::clone::Clone for AE_GENERIC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AE_GENERIC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AE_GENERIC")
            .field("ae_ge_msgfile", &self.ae_ge_msgfile)
            .field("ae_ge_msgnum", &self.ae_ge_msgnum)
            .field("ae_ge_params", &self.ae_ge_params)
            .field("ae_ge_param1", &self.ae_ge_param1)
            .field("ae_ge_param2", &self.ae_ge_param2)
            .field("ae_ge_param3", &self.ae_ge_param3)
            .field("ae_ge_param4", &self.ae_ge_param4)
            .field("ae_ge_param5", &self.ae_ge_param5)
            .field("ae_ge_param6", &self.ae_ge_param6)
            .field("ae_ge_param7", &self.ae_ge_param7)
            .field("ae_ge_param8", &self.ae_ge_param8)
            .field("ae_ge_param9", &self.ae_ge_param9)
            .finish()
    }
}
impl ::windows::core::TypeKind for AE_GENERIC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for AE_GENERIC {
    fn eq(&self, other: &Self) -> bool {
        self.ae_ge_msgfile == other.ae_ge_msgfile && self.ae_ge_msgnum == other.ae_ge_msgnum && self.ae_ge_params == other.ae_ge_params && self.ae_ge_param1 == other.ae_ge_param1 && self.ae_ge_param2 == other.ae_ge_param2 && self.ae_ge_param3 == other.ae_ge_param3 && self.ae_ge_param4 == other.ae_ge_param4 && self.ae_ge_param5 == other.ae_ge_param5 && self.ae_ge_param6 == other.ae_ge_param6 && self.ae_ge_param7 == other.ae_ge_param7 && self.ae_ge_param8 == other.ae_ge_param8 && self.ae_ge_param9 == other.ae_ge_param9
    }
}
impl ::core::cmp::Eq for AE_GENERIC {}
impl ::core::default::Default for AE_GENERIC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct AE_LOCKOUT {
    pub ae_lk_compname: u32,
    pub ae_lk_username: u32,
    pub ae_lk_action: u32,
    pub ae_lk_bad_pw_count: u32,
}
impl ::core::marker::Copy for AE_LOCKOUT {}
impl ::core::clone::Clone for AE_LOCKOUT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AE_LOCKOUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AE_LOCKOUT").field("ae_lk_compname", &self.ae_lk_compname).field("ae_lk_username", &self.ae_lk_username).field("ae_lk_action", &self.ae_lk_action).field("ae_lk_bad_pw_count", &self.ae_lk_bad_pw_count).finish()
    }
}
impl ::windows::core::TypeKind for AE_LOCKOUT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for AE_LOCKOUT {
    fn eq(&self, other: &Self) -> bool {
        self.ae_lk_compname == other.ae_lk_compname && self.ae_lk_username == other.ae_lk_username && self.ae_lk_action == other.ae_lk_action && self.ae_lk_bad_pw_count == other.ae_lk_bad_pw_count
    }
}
impl ::core::cmp::Eq for AE_LOCKOUT {}
impl ::core::default::Default for AE_LOCKOUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct AE_NETLOGOFF {
    pub ae_nf_compname: u32,
    pub ae_nf_username: u32,
    pub ae_nf_reserved1: u32,
    pub ae_nf_reserved2: u32,
}
impl ::core::marker::Copy for AE_NETLOGOFF {}
impl ::core::clone::Clone for AE_NETLOGOFF {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AE_NETLOGOFF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AE_NETLOGOFF").field("ae_nf_compname", &self.ae_nf_compname).field("ae_nf_username", &self.ae_nf_username).field("ae_nf_reserved1", &self.ae_nf_reserved1).field("ae_nf_reserved2", &self.ae_nf_reserved2).finish()
    }
}
impl ::windows::core::TypeKind for AE_NETLOGOFF {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for AE_NETLOGOFF {
    fn eq(&self, other: &Self) -> bool {
        self.ae_nf_compname == other.ae_nf_compname && self.ae_nf_username == other.ae_nf_username && self.ae_nf_reserved1 == other.ae_nf_reserved1 && self.ae_nf_reserved2 == other.ae_nf_reserved2
    }
}
impl ::core::cmp::Eq for AE_NETLOGOFF {}
impl ::core::default::Default for AE_NETLOGOFF {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct AE_NETLOGON {
    pub ae_no_compname: u32,
    pub ae_no_username: u32,
    pub ae_no_privilege: u32,
    pub ae_no_authflags: u32,
}
impl ::core::marker::Copy for AE_NETLOGON {}
impl ::core::clone::Clone for AE_NETLOGON {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AE_NETLOGON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AE_NETLOGON").field("ae_no_compname", &self.ae_no_compname).field("ae_no_username", &self.ae_no_username).field("ae_no_privilege", &self.ae_no_privilege).field("ae_no_authflags", &self.ae_no_authflags).finish()
    }
}
impl ::windows::core::TypeKind for AE_NETLOGON {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for AE_NETLOGON {
    fn eq(&self, other: &Self) -> bool {
        self.ae_no_compname == other.ae_no_compname && self.ae_no_username == other.ae_no_username && self.ae_no_privilege == other.ae_no_privilege && self.ae_no_authflags == other.ae_no_authflags
    }
}
impl ::core::cmp::Eq for AE_NETLOGON {}
impl ::core::default::Default for AE_NETLOGON {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct AE_RESACCESS {
    pub ae_ra_compname: u32,
    pub ae_ra_username: u32,
    pub ae_ra_resname: u32,
    pub ae_ra_operation: u32,
    pub ae_ra_returncode: u32,
    pub ae_ra_restype: u32,
    pub ae_ra_fileid: u32,
}
impl ::core::marker::Copy for AE_RESACCESS {}
impl ::core::clone::Clone for AE_RESACCESS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AE_RESACCESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AE_RESACCESS").field("ae_ra_compname", &self.ae_ra_compname).field("ae_ra_username", &self.ae_ra_username).field("ae_ra_resname", &self.ae_ra_resname).field("ae_ra_operation", &self.ae_ra_operation).field("ae_ra_returncode", &self.ae_ra_returncode).field("ae_ra_restype", &self.ae_ra_restype).field("ae_ra_fileid", &self.ae_ra_fileid).finish()
    }
}
impl ::windows::core::TypeKind for AE_RESACCESS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for AE_RESACCESS {
    fn eq(&self, other: &Self) -> bool {
        self.ae_ra_compname == other.ae_ra_compname && self.ae_ra_username == other.ae_ra_username && self.ae_ra_resname == other.ae_ra_resname && self.ae_ra_operation == other.ae_ra_operation && self.ae_ra_returncode == other.ae_ra_returncode && self.ae_ra_restype == other.ae_ra_restype && self.ae_ra_fileid == other.ae_ra_fileid
    }
}
impl ::core::cmp::Eq for AE_RESACCESS {}
impl ::core::default::Default for AE_RESACCESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct AE_RESACCESSREJ {
    pub ae_rr_compname: u32,
    pub ae_rr_username: u32,
    pub ae_rr_resname: u32,
    pub ae_rr_operation: u32,
}
impl ::core::marker::Copy for AE_RESACCESSREJ {}
impl ::core::clone::Clone for AE_RESACCESSREJ {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AE_RESACCESSREJ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AE_RESACCESSREJ").field("ae_rr_compname", &self.ae_rr_compname).field("ae_rr_username", &self.ae_rr_username).field("ae_rr_resname", &self.ae_rr_resname).field("ae_rr_operation", &self.ae_rr_operation).finish()
    }
}
impl ::windows::core::TypeKind for AE_RESACCESSREJ {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for AE_RESACCESSREJ {
    fn eq(&self, other: &Self) -> bool {
        self.ae_rr_compname == other.ae_rr_compname && self.ae_rr_username == other.ae_rr_username && self.ae_rr_resname == other.ae_rr_resname && self.ae_rr_operation == other.ae_rr_operation
    }
}
impl ::core::cmp::Eq for AE_RESACCESSREJ {}
impl ::core::default::Default for AE_RESACCESSREJ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct AE_SERVICESTAT {
    pub ae_ss_compname: u32,
    pub ae_ss_username: u32,
    pub ae_ss_svcname: u32,
    pub ae_ss_status: u32,
    pub ae_ss_code: u32,
    pub ae_ss_text: u32,
    pub ae_ss_returnval: u32,
}
impl ::core::marker::Copy for AE_SERVICESTAT {}
impl ::core::clone::Clone for AE_SERVICESTAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AE_SERVICESTAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AE_SERVICESTAT").field("ae_ss_compname", &self.ae_ss_compname).field("ae_ss_username", &self.ae_ss_username).field("ae_ss_svcname", &self.ae_ss_svcname).field("ae_ss_status", &self.ae_ss_status).field("ae_ss_code", &self.ae_ss_code).field("ae_ss_text", &self.ae_ss_text).field("ae_ss_returnval", &self.ae_ss_returnval).finish()
    }
}
impl ::windows::core::TypeKind for AE_SERVICESTAT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for AE_SERVICESTAT {
    fn eq(&self, other: &Self) -> bool {
        self.ae_ss_compname == other.ae_ss_compname && self.ae_ss_username == other.ae_ss_username && self.ae_ss_svcname == other.ae_ss_svcname && self.ae_ss_status == other.ae_ss_status && self.ae_ss_code == other.ae_ss_code && self.ae_ss_text == other.ae_ss_text && self.ae_ss_returnval == other.ae_ss_returnval
    }
}
impl ::core::cmp::Eq for AE_SERVICESTAT {}
impl ::core::default::Default for AE_SERVICESTAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct AE_SESSLOGOFF {
    pub ae_sf_compname: u32,
    pub ae_sf_username: u32,
    pub ae_sf_reason: u32,
}
impl ::core::marker::Copy for AE_SESSLOGOFF {}
impl ::core::clone::Clone for AE_SESSLOGOFF {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AE_SESSLOGOFF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AE_SESSLOGOFF").field("ae_sf_compname", &self.ae_sf_compname).field("ae_sf_username", &self.ae_sf_username).field("ae_sf_reason", &self.ae_sf_reason).finish()
    }
}
impl ::windows::core::TypeKind for AE_SESSLOGOFF {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for AE_SESSLOGOFF {
    fn eq(&self, other: &Self) -> bool {
        self.ae_sf_compname == other.ae_sf_compname && self.ae_sf_username == other.ae_sf_username && self.ae_sf_reason == other.ae_sf_reason
    }
}
impl ::core::cmp::Eq for AE_SESSLOGOFF {}
impl ::core::default::Default for AE_SESSLOGOFF {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct AE_SESSLOGON {
    pub ae_so_compname: u32,
    pub ae_so_username: u32,
    pub ae_so_privilege: u32,
}
impl ::core::marker::Copy for AE_SESSLOGON {}
impl ::core::clone::Clone for AE_SESSLOGON {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AE_SESSLOGON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AE_SESSLOGON").field("ae_so_compname", &self.ae_so_compname).field("ae_so_username", &self.ae_so_username).field("ae_so_privilege", &self.ae_so_privilege).finish()
    }
}
impl ::windows::core::TypeKind for AE_SESSLOGON {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for AE_SESSLOGON {
    fn eq(&self, other: &Self) -> bool {
        self.ae_so_compname == other.ae_so_compname && self.ae_so_username == other.ae_so_username && self.ae_so_privilege == other.ae_so_privilege
    }
}
impl ::core::cmp::Eq for AE_SESSLOGON {}
impl ::core::default::Default for AE_SESSLOGON {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct AE_SESSPWERR {
    pub ae_sp_compname: u32,
    pub ae_sp_username: u32,
}
impl ::core::marker::Copy for AE_SESSPWERR {}
impl ::core::clone::Clone for AE_SESSPWERR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AE_SESSPWERR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AE_SESSPWERR").field("ae_sp_compname", &self.ae_sp_compname).field("ae_sp_username", &self.ae_sp_username).finish()
    }
}
impl ::windows::core::TypeKind for AE_SESSPWERR {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for AE_SESSPWERR {
    fn eq(&self, other: &Self) -> bool {
        self.ae_sp_compname == other.ae_sp_compname && self.ae_sp_username == other.ae_sp_username
    }
}
impl ::core::cmp::Eq for AE_SESSPWERR {}
impl ::core::default::Default for AE_SESSPWERR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct AE_SRVSTATUS {
    pub ae_sv_status: u32,
}
impl ::core::marker::Copy for AE_SRVSTATUS {}
impl ::core::clone::Clone for AE_SRVSTATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AE_SRVSTATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AE_SRVSTATUS").field("ae_sv_status", &self.ae_sv_status).finish()
    }
}
impl ::windows::core::TypeKind for AE_SRVSTATUS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for AE_SRVSTATUS {
    fn eq(&self, other: &Self) -> bool {
        self.ae_sv_status == other.ae_sv_status
    }
}
impl ::core::cmp::Eq for AE_SRVSTATUS {}
impl ::core::default::Default for AE_SRVSTATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct AE_UASMOD {
    pub ae_um_compname: u32,
    pub ae_um_username: u32,
    pub ae_um_resname: u32,
    pub ae_um_rectype: u32,
    pub ae_um_action: u32,
    pub ae_um_datalen: u32,
}
impl ::core::marker::Copy for AE_UASMOD {}
impl ::core::clone::Clone for AE_UASMOD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AE_UASMOD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AE_UASMOD").field("ae_um_compname", &self.ae_um_compname).field("ae_um_username", &self.ae_um_username).field("ae_um_resname", &self.ae_um_resname).field("ae_um_rectype", &self.ae_um_rectype).field("ae_um_action", &self.ae_um_action).field("ae_um_datalen", &self.ae_um_datalen).finish()
    }
}
impl ::windows::core::TypeKind for AE_UASMOD {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for AE_UASMOD {
    fn eq(&self, other: &Self) -> bool {
        self.ae_um_compname == other.ae_um_compname && self.ae_um_username == other.ae_um_username && self.ae_um_resname == other.ae_um_resname && self.ae_um_rectype == other.ae_um_rectype && self.ae_um_action == other.ae_um_action && self.ae_um_datalen == other.ae_um_datalen
    }
}
impl ::core::cmp::Eq for AE_UASMOD {}
impl ::core::default::Default for AE_UASMOD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct AT_ENUM {
    pub JobId: u32,
    pub JobTime: usize,
    pub DaysOfMonth: u32,
    pub DaysOfWeek: u8,
    pub Flags: u8,
    pub Command: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for AT_ENUM {}
impl ::core::clone::Clone for AT_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AT_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AT_ENUM").field("JobId", &self.JobId).field("JobTime", &self.JobTime).field("DaysOfMonth", &self.DaysOfMonth).field("DaysOfWeek", &self.DaysOfWeek).field("Flags", &self.Flags).field("Command", &self.Command).finish()
    }
}
impl ::windows::core::TypeKind for AT_ENUM {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for AT_ENUM {
    fn eq(&self, other: &Self) -> bool {
        self.JobId == other.JobId && self.JobTime == other.JobTime && self.DaysOfMonth == other.DaysOfMonth && self.DaysOfWeek == other.DaysOfWeek && self.Flags == other.Flags && self.Command == other.Command
    }
}
impl ::core::cmp::Eq for AT_ENUM {}
impl ::core::default::Default for AT_ENUM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct AT_INFO {
    pub JobTime: usize,
    pub DaysOfMonth: u32,
    pub DaysOfWeek: u8,
    pub Flags: u8,
    pub Command: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for AT_INFO {}
impl ::core::clone::Clone for AT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AT_INFO").field("JobTime", &self.JobTime).field("DaysOfMonth", &self.DaysOfMonth).field("DaysOfWeek", &self.DaysOfWeek).field("Flags", &self.Flags).field("Command", &self.Command).finish()
    }
}
impl ::windows::core::TypeKind for AT_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for AT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.JobTime == other.JobTime && self.DaysOfMonth == other.DaysOfMonth && self.DaysOfWeek == other.DaysOfWeek && self.Flags == other.Flags && self.Command == other.Command
    }
}
impl ::core::cmp::Eq for AT_INFO {}
impl ::core::default::Default for AT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct AUDIT_ENTRY {
    pub ae_len: u32,
    pub ae_reserved: u32,
    pub ae_time: u32,
    pub ae_type: u32,
    pub ae_data_offset: u32,
    pub ae_data_size: u32,
}
impl ::core::marker::Copy for AUDIT_ENTRY {}
impl ::core::clone::Clone for AUDIT_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AUDIT_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUDIT_ENTRY").field("ae_len", &self.ae_len).field("ae_reserved", &self.ae_reserved).field("ae_time", &self.ae_time).field("ae_type", &self.ae_type).field("ae_data_offset", &self.ae_data_offset).field("ae_data_size", &self.ae_data_size).finish()
    }
}
impl ::windows::core::TypeKind for AUDIT_ENTRY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for AUDIT_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.ae_len == other.ae_len && self.ae_reserved == other.ae_reserved && self.ae_time == other.ae_time && self.ae_type == other.ae_type && self.ae_data_offset == other.ae_data_offset && self.ae_data_size == other.ae_data_size
    }
}
impl ::core::cmp::Eq for AUDIT_ENTRY {}
impl ::core::default::Default for AUDIT_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct CONFIG_INFO_0 {
    pub cfgi0_key: ::windows::core::PWSTR,
    pub cfgi0_data: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for CONFIG_INFO_0 {}
impl ::core::clone::Clone for CONFIG_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CONFIG_INFO_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONFIG_INFO_0").field("cfgi0_key", &self.cfgi0_key).field("cfgi0_data", &self.cfgi0_data).finish()
    }
}
impl ::windows::core::TypeKind for CONFIG_INFO_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for CONFIG_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        self.cfgi0_key == other.cfgi0_key && self.cfgi0_data == other.cfgi0_data
    }
}
impl ::core::cmp::Eq for CONFIG_INFO_0 {}
impl ::core::default::Default for CONFIG_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub struct DSREG_JOIN_INFO {
    pub joinType: DSREG_JOIN_TYPE,
    pub pJoinCertificate: *const super::super::Security::Cryptography::CERT_CONTEXT,
    pub pszDeviceId: ::windows::core::PWSTR,
    pub pszIdpDomain: ::windows::core::PWSTR,
    pub pszTenantId: ::windows::core::PWSTR,
    pub pszJoinUserEmail: ::windows::core::PWSTR,
    pub pszTenantDisplayName: ::windows::core::PWSTR,
    pub pszMdmEnrollmentUrl: ::windows::core::PWSTR,
    pub pszMdmTermsOfUseUrl: ::windows::core::PWSTR,
    pub pszMdmComplianceUrl: ::windows::core::PWSTR,
    pub pszUserSettingSyncUrl: ::windows::core::PWSTR,
    pub pUserInfo: *mut DSREG_USER_INFO,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::marker::Copy for DSREG_JOIN_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::clone::Clone for DSREG_JOIN_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::fmt::Debug for DSREG_JOIN_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSREG_JOIN_INFO")
            .field("joinType", &self.joinType)
            .field("pJoinCertificate", &self.pJoinCertificate)
            .field("pszDeviceId", &self.pszDeviceId)
            .field("pszIdpDomain", &self.pszIdpDomain)
            .field("pszTenantId", &self.pszTenantId)
            .field("pszJoinUserEmail", &self.pszJoinUserEmail)
            .field("pszTenantDisplayName", &self.pszTenantDisplayName)
            .field("pszMdmEnrollmentUrl", &self.pszMdmEnrollmentUrl)
            .field("pszMdmTermsOfUseUrl", &self.pszMdmTermsOfUseUrl)
            .field("pszMdmComplianceUrl", &self.pszMdmComplianceUrl)
            .field("pszUserSettingSyncUrl", &self.pszUserSettingSyncUrl)
            .field("pUserInfo", &self.pUserInfo)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::windows::core::TypeKind for DSREG_JOIN_INFO {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::PartialEq for DSREG_JOIN_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.joinType == other.joinType && self.pJoinCertificate == other.pJoinCertificate && self.pszDeviceId == other.pszDeviceId && self.pszIdpDomain == other.pszIdpDomain && self.pszTenantId == other.pszTenantId && self.pszJoinUserEmail == other.pszJoinUserEmail && self.pszTenantDisplayName == other.pszTenantDisplayName && self.pszMdmEnrollmentUrl == other.pszMdmEnrollmentUrl && self.pszMdmTermsOfUseUrl == other.pszMdmTermsOfUseUrl && self.pszMdmComplianceUrl == other.pszMdmComplianceUrl && self.pszUserSettingSyncUrl == other.pszUserSettingSyncUrl && self.pUserInfo == other.pUserInfo
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::Eq for DSREG_JOIN_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::default::Default for DSREG_JOIN_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct DSREG_USER_INFO {
    pub pszUserEmail: ::windows::core::PWSTR,
    pub pszUserKeyId: ::windows::core::PWSTR,
    pub pszUserKeyName: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for DSREG_USER_INFO {}
impl ::core::clone::Clone for DSREG_USER_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DSREG_USER_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSREG_USER_INFO").field("pszUserEmail", &self.pszUserEmail).field("pszUserKeyId", &self.pszUserKeyId).field("pszUserKeyName", &self.pszUserKeyName).finish()
    }
}
impl ::windows::core::TypeKind for DSREG_USER_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DSREG_USER_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.pszUserEmail == other.pszUserEmail && self.pszUserKeyId == other.pszUserKeyId && self.pszUserKeyName == other.pszUserKeyName
    }
}
impl ::core::cmp::Eq for DSREG_USER_INFO {}
impl ::core::default::Default for DSREG_USER_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct ERRLOG_OTHER_INFO {
    pub alrter_errcode: u32,
    pub alrter_offset: u32,
}
impl ::core::marker::Copy for ERRLOG_OTHER_INFO {}
impl ::core::clone::Clone for ERRLOG_OTHER_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ERRLOG_OTHER_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ERRLOG_OTHER_INFO").field("alrter_errcode", &self.alrter_errcode).field("alrter_offset", &self.alrter_offset).finish()
    }
}
impl ::windows::core::TypeKind for ERRLOG_OTHER_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for ERRLOG_OTHER_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.alrter_errcode == other.alrter_errcode && self.alrter_offset == other.alrter_offset
    }
}
impl ::core::cmp::Eq for ERRLOG_OTHER_INFO {}
impl ::core::default::Default for ERRLOG_OTHER_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct ERROR_LOG {
    pub el_len: u32,
    pub el_reserved: u32,
    pub el_time: u32,
    pub el_error: u32,
    pub el_name: ::windows::core::PWSTR,
    pub el_text: ::windows::core::PWSTR,
    pub el_data: *mut u8,
    pub el_data_size: u32,
    pub el_nstrings: u32,
}
impl ::core::marker::Copy for ERROR_LOG {}
impl ::core::clone::Clone for ERROR_LOG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ERROR_LOG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ERROR_LOG").field("el_len", &self.el_len).field("el_reserved", &self.el_reserved).field("el_time", &self.el_time).field("el_error", &self.el_error).field("el_name", &self.el_name).field("el_text", &self.el_text).field("el_data", &self.el_data).field("el_data_size", &self.el_data_size).field("el_nstrings", &self.el_nstrings).finish()
    }
}
impl ::windows::core::TypeKind for ERROR_LOG {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for ERROR_LOG {
    fn eq(&self, other: &Self) -> bool {
        self.el_len == other.el_len && self.el_reserved == other.el_reserved && self.el_time == other.el_time && self.el_error == other.el_error && self.el_name == other.el_name && self.el_text == other.el_text && self.el_data == other.el_data && self.el_data_size == other.el_data_size && self.el_nstrings == other.el_nstrings
    }
}
impl ::core::cmp::Eq for ERROR_LOG {}
impl ::core::default::Default for ERROR_LOG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct FLAT_STRING {
    pub MaximumLength: i16,
    pub Length: i16,
    pub Buffer: [u8; 1],
}
impl ::core::marker::Copy for FLAT_STRING {}
impl ::core::clone::Clone for FLAT_STRING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FLAT_STRING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FLAT_STRING").field("MaximumLength", &self.MaximumLength).field("Length", &self.Length).field("Buffer", &self.Buffer).finish()
    }
}
impl ::windows::core::TypeKind for FLAT_STRING {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for FLAT_STRING {
    fn eq(&self, other: &Self) -> bool {
        self.MaximumLength == other.MaximumLength && self.Length == other.Length && self.Buffer == other.Buffer
    }
}
impl ::core::cmp::Eq for FLAT_STRING {}
impl ::core::default::Default for FLAT_STRING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct GROUP_INFO_0 {
    pub grpi0_name: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for GROUP_INFO_0 {}
impl ::core::clone::Clone for GROUP_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GROUP_INFO_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GROUP_INFO_0").field("grpi0_name", &self.grpi0_name).finish()
    }
}
impl ::windows::core::TypeKind for GROUP_INFO_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for GROUP_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        self.grpi0_name == other.grpi0_name
    }
}
impl ::core::cmp::Eq for GROUP_INFO_0 {}
impl ::core::default::Default for GROUP_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct GROUP_INFO_1 {
    pub grpi1_name: ::windows::core::PWSTR,
    pub grpi1_comment: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for GROUP_INFO_1 {}
impl ::core::clone::Clone for GROUP_INFO_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GROUP_INFO_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GROUP_INFO_1").field("grpi1_name", &self.grpi1_name).field("grpi1_comment", &self.grpi1_comment).finish()
    }
}
impl ::windows::core::TypeKind for GROUP_INFO_1 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for GROUP_INFO_1 {
    fn eq(&self, other: &Self) -> bool {
        self.grpi1_name == other.grpi1_name && self.grpi1_comment == other.grpi1_comment
    }
}
impl ::core::cmp::Eq for GROUP_INFO_1 {}
impl ::core::default::Default for GROUP_INFO_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct GROUP_INFO_1002 {
    pub grpi1002_comment: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for GROUP_INFO_1002 {}
impl ::core::clone::Clone for GROUP_INFO_1002 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GROUP_INFO_1002 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GROUP_INFO_1002").field("grpi1002_comment", &self.grpi1002_comment).finish()
    }
}
impl ::windows::core::TypeKind for GROUP_INFO_1002 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for GROUP_INFO_1002 {
    fn eq(&self, other: &Self) -> bool {
        self.grpi1002_comment == other.grpi1002_comment
    }
}
impl ::core::cmp::Eq for GROUP_INFO_1002 {}
impl ::core::default::Default for GROUP_INFO_1002 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct GROUP_INFO_1005 {
    pub grpi1005_attributes: u32,
}
impl ::core::marker::Copy for GROUP_INFO_1005 {}
impl ::core::clone::Clone for GROUP_INFO_1005 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GROUP_INFO_1005 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GROUP_INFO_1005").field("grpi1005_attributes", &self.grpi1005_attributes).finish()
    }
}
impl ::windows::core::TypeKind for GROUP_INFO_1005 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for GROUP_INFO_1005 {
    fn eq(&self, other: &Self) -> bool {
        self.grpi1005_attributes == other.grpi1005_attributes
    }
}
impl ::core::cmp::Eq for GROUP_INFO_1005 {}
impl ::core::default::Default for GROUP_INFO_1005 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct GROUP_INFO_2 {
    pub grpi2_name: ::windows::core::PWSTR,
    pub grpi2_comment: ::windows::core::PWSTR,
    pub grpi2_group_id: u32,
    pub grpi2_attributes: u32,
}
impl ::core::marker::Copy for GROUP_INFO_2 {}
impl ::core::clone::Clone for GROUP_INFO_2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GROUP_INFO_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GROUP_INFO_2").field("grpi2_name", &self.grpi2_name).field("grpi2_comment", &self.grpi2_comment).field("grpi2_group_id", &self.grpi2_group_id).field("grpi2_attributes", &self.grpi2_attributes).finish()
    }
}
impl ::windows::core::TypeKind for GROUP_INFO_2 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for GROUP_INFO_2 {
    fn eq(&self, other: &Self) -> bool {
        self.grpi2_name == other.grpi2_name && self.grpi2_comment == other.grpi2_comment && self.grpi2_group_id == other.grpi2_group_id && self.grpi2_attributes == other.grpi2_attributes
    }
}
impl ::core::cmp::Eq for GROUP_INFO_2 {}
impl ::core::default::Default for GROUP_INFO_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct GROUP_INFO_3 {
    pub grpi3_name: ::windows::core::PWSTR,
    pub grpi3_comment: ::windows::core::PWSTR,
    pub grpi3_group_sid: super::super::Foundation::PSID,
    pub grpi3_attributes: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GROUP_INFO_3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GROUP_INFO_3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for GROUP_INFO_3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GROUP_INFO_3").field("grpi3_name", &self.grpi3_name).field("grpi3_comment", &self.grpi3_comment).field("grpi3_group_sid", &self.grpi3_group_sid).field("grpi3_attributes", &self.grpi3_attributes).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for GROUP_INFO_3 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GROUP_INFO_3 {
    fn eq(&self, other: &Self) -> bool {
        self.grpi3_name == other.grpi3_name && self.grpi3_comment == other.grpi3_comment && self.grpi3_group_sid == other.grpi3_group_sid && self.grpi3_attributes == other.grpi3_attributes
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GROUP_INFO_3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GROUP_INFO_3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct GROUP_USERS_INFO_0 {
    pub grui0_name: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for GROUP_USERS_INFO_0 {}
impl ::core::clone::Clone for GROUP_USERS_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GROUP_USERS_INFO_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GROUP_USERS_INFO_0").field("grui0_name", &self.grui0_name).finish()
    }
}
impl ::windows::core::TypeKind for GROUP_USERS_INFO_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for GROUP_USERS_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        self.grui0_name == other.grui0_name
    }
}
impl ::core::cmp::Eq for GROUP_USERS_INFO_0 {}
impl ::core::default::Default for GROUP_USERS_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct GROUP_USERS_INFO_1 {
    pub grui1_name: ::windows::core::PWSTR,
    pub grui1_attributes: u32,
}
impl ::core::marker::Copy for GROUP_USERS_INFO_1 {}
impl ::core::clone::Clone for GROUP_USERS_INFO_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GROUP_USERS_INFO_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GROUP_USERS_INFO_1").field("grui1_name", &self.grui1_name).field("grui1_attributes", &self.grui1_attributes).finish()
    }
}
impl ::windows::core::TypeKind for GROUP_USERS_INFO_1 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for GROUP_USERS_INFO_1 {
    fn eq(&self, other: &Self) -> bool {
        self.grui1_name == other.grui1_name && self.grui1_attributes == other.grui1_attributes
    }
}
impl ::core::cmp::Eq for GROUP_USERS_INFO_1 {}
impl ::core::default::Default for GROUP_USERS_INFO_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct HARDWARE_ADDRESS {
    pub Address: [u8; 6],
}
impl ::core::marker::Copy for HARDWARE_ADDRESS {}
impl ::core::clone::Clone for HARDWARE_ADDRESS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HARDWARE_ADDRESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HARDWARE_ADDRESS").field("Address", &self.Address).finish()
    }
}
impl ::windows::core::TypeKind for HARDWARE_ADDRESS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for HARDWARE_ADDRESS {
    fn eq(&self, other: &Self) -> bool {
        self.Address == other.Address
    }
}
impl ::core::cmp::Eq for HARDWARE_ADDRESS {}
impl ::core::default::Default for HARDWARE_ADDRESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct HLOG {
    pub time: u32,
    pub last_flags: u32,
    pub offset: u32,
    pub rec_offset: u32,
}
impl ::core::marker::Copy for HLOG {}
impl ::core::clone::Clone for HLOG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HLOG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HLOG").field("time", &self.time).field("last_flags", &self.last_flags).field("offset", &self.offset).field("rec_offset", &self.rec_offset).finish()
    }
}
impl ::windows::core::TypeKind for HLOG {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for HLOG {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time && self.last_flags == other.last_flags && self.offset == other.offset && self.rec_offset == other.rec_offset
    }
}
impl ::core::cmp::Eq for HLOG {}
impl ::core::default::Default for HLOG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct LOCALGROUP_INFO_0 {
    pub lgrpi0_name: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for LOCALGROUP_INFO_0 {}
impl ::core::clone::Clone for LOCALGROUP_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for LOCALGROUP_INFO_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LOCALGROUP_INFO_0").field("lgrpi0_name", &self.lgrpi0_name).finish()
    }
}
impl ::windows::core::TypeKind for LOCALGROUP_INFO_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for LOCALGROUP_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        self.lgrpi0_name == other.lgrpi0_name
    }
}
impl ::core::cmp::Eq for LOCALGROUP_INFO_0 {}
impl ::core::default::Default for LOCALGROUP_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct LOCALGROUP_INFO_1 {
    pub lgrpi1_name: ::windows::core::PWSTR,
    pub lgrpi1_comment: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for LOCALGROUP_INFO_1 {}
impl ::core::clone::Clone for LOCALGROUP_INFO_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for LOCALGROUP_INFO_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LOCALGROUP_INFO_1").field("lgrpi1_name", &self.lgrpi1_name).field("lgrpi1_comment", &self.lgrpi1_comment).finish()
    }
}
impl ::windows::core::TypeKind for LOCALGROUP_INFO_1 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for LOCALGROUP_INFO_1 {
    fn eq(&self, other: &Self) -> bool {
        self.lgrpi1_name == other.lgrpi1_name && self.lgrpi1_comment == other.lgrpi1_comment
    }
}
impl ::core::cmp::Eq for LOCALGROUP_INFO_1 {}
impl ::core::default::Default for LOCALGROUP_INFO_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct LOCALGROUP_INFO_1002 {
    pub lgrpi1002_comment: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for LOCALGROUP_INFO_1002 {}
impl ::core::clone::Clone for LOCALGROUP_INFO_1002 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for LOCALGROUP_INFO_1002 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LOCALGROUP_INFO_1002").field("lgrpi1002_comment", &self.lgrpi1002_comment).finish()
    }
}
impl ::windows::core::TypeKind for LOCALGROUP_INFO_1002 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for LOCALGROUP_INFO_1002 {
    fn eq(&self, other: &Self) -> bool {
        self.lgrpi1002_comment == other.lgrpi1002_comment
    }
}
impl ::core::cmp::Eq for LOCALGROUP_INFO_1002 {}
impl ::core::default::Default for LOCALGROUP_INFO_1002 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct LOCALGROUP_MEMBERS_INFO_0 {
    pub lgrmi0_sid: super::super::Foundation::PSID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for LOCALGROUP_MEMBERS_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for LOCALGROUP_MEMBERS_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for LOCALGROUP_MEMBERS_INFO_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LOCALGROUP_MEMBERS_INFO_0").field("lgrmi0_sid", &self.lgrmi0_sid).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for LOCALGROUP_MEMBERS_INFO_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for LOCALGROUP_MEMBERS_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        self.lgrmi0_sid == other.lgrmi0_sid
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for LOCALGROUP_MEMBERS_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for LOCALGROUP_MEMBERS_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct LOCALGROUP_MEMBERS_INFO_1 {
    pub lgrmi1_sid: super::super::Foundation::PSID,
    pub lgrmi1_sidusage: super::super::Security::SID_NAME_USE,
    pub lgrmi1_name: ::windows::core::PWSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for LOCALGROUP_MEMBERS_INFO_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for LOCALGROUP_MEMBERS_INFO_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::fmt::Debug for LOCALGROUP_MEMBERS_INFO_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LOCALGROUP_MEMBERS_INFO_1").field("lgrmi1_sid", &self.lgrmi1_sid).field("lgrmi1_sidusage", &self.lgrmi1_sidusage).field("lgrmi1_name", &self.lgrmi1_name).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::windows::core::TypeKind for LOCALGROUP_MEMBERS_INFO_1 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for LOCALGROUP_MEMBERS_INFO_1 {
    fn eq(&self, other: &Self) -> bool {
        self.lgrmi1_sid == other.lgrmi1_sid && self.lgrmi1_sidusage == other.lgrmi1_sidusage && self.lgrmi1_name == other.lgrmi1_name
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for LOCALGROUP_MEMBERS_INFO_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for LOCALGROUP_MEMBERS_INFO_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct LOCALGROUP_MEMBERS_INFO_2 {
    pub lgrmi2_sid: super::super::Foundation::PSID,
    pub lgrmi2_sidusage: super::super::Security::SID_NAME_USE,
    pub lgrmi2_domainandname: ::windows::core::PWSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for LOCALGROUP_MEMBERS_INFO_2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for LOCALGROUP_MEMBERS_INFO_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::fmt::Debug for LOCALGROUP_MEMBERS_INFO_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LOCALGROUP_MEMBERS_INFO_2").field("lgrmi2_sid", &self.lgrmi2_sid).field("lgrmi2_sidusage", &self.lgrmi2_sidusage).field("lgrmi2_domainandname", &self.lgrmi2_domainandname).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::windows::core::TypeKind for LOCALGROUP_MEMBERS_INFO_2 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for LOCALGROUP_MEMBERS_INFO_2 {
    fn eq(&self, other: &Self) -> bool {
        self.lgrmi2_sid == other.lgrmi2_sid && self.lgrmi2_sidusage == other.lgrmi2_sidusage && self.lgrmi2_domainandname == other.lgrmi2_domainandname
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for LOCALGROUP_MEMBERS_INFO_2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for LOCALGROUP_MEMBERS_INFO_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct LOCALGROUP_MEMBERS_INFO_3 {
    pub lgrmi3_domainandname: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for LOCALGROUP_MEMBERS_INFO_3 {}
impl ::core::clone::Clone for LOCALGROUP_MEMBERS_INFO_3 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for LOCALGROUP_MEMBERS_INFO_3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LOCALGROUP_MEMBERS_INFO_3").field("lgrmi3_domainandname", &self.lgrmi3_domainandname).finish()
    }
}
impl ::windows::core::TypeKind for LOCALGROUP_MEMBERS_INFO_3 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for LOCALGROUP_MEMBERS_INFO_3 {
    fn eq(&self, other: &Self) -> bool {
        self.lgrmi3_domainandname == other.lgrmi3_domainandname
    }
}
impl ::core::cmp::Eq for LOCALGROUP_MEMBERS_INFO_3 {}
impl ::core::default::Default for LOCALGROUP_MEMBERS_INFO_3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct LOCALGROUP_USERS_INFO_0 {
    pub lgrui0_name: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for LOCALGROUP_USERS_INFO_0 {}
impl ::core::clone::Clone for LOCALGROUP_USERS_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for LOCALGROUP_USERS_INFO_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LOCALGROUP_USERS_INFO_0").field("lgrui0_name", &self.lgrui0_name).finish()
    }
}
impl ::windows::core::TypeKind for LOCALGROUP_USERS_INFO_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for LOCALGROUP_USERS_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        self.lgrui0_name == other.lgrui0_name
    }
}
impl ::core::cmp::Eq for LOCALGROUP_USERS_INFO_0 {}
impl ::core::default::Default for LOCALGROUP_USERS_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct MPR_PROTOCOL_0 {
    pub dwProtocolId: u32,
    pub wszProtocol: [u16; 41],
    pub wszDLLName: [u16; 49],
}
impl ::core::marker::Copy for MPR_PROTOCOL_0 {}
impl ::core::clone::Clone for MPR_PROTOCOL_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MPR_PROTOCOL_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MPR_PROTOCOL_0").field("dwProtocolId", &self.dwProtocolId).field("wszProtocol", &self.wszProtocol).field("wszDLLName", &self.wszDLLName).finish()
    }
}
impl ::windows::core::TypeKind for MPR_PROTOCOL_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MPR_PROTOCOL_0 {
    fn eq(&self, other: &Self) -> bool {
        self.dwProtocolId == other.dwProtocolId && self.wszProtocol == other.wszProtocol && self.wszDLLName == other.wszDLLName
    }
}
impl ::core::cmp::Eq for MPR_PROTOCOL_0 {}
impl ::core::default::Default for MPR_PROTOCOL_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct MSA_INFO_0 {
    pub State: MSA_INFO_STATE,
}
impl ::core::marker::Copy for MSA_INFO_0 {}
impl ::core::clone::Clone for MSA_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MSA_INFO_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MSA_INFO_0").field("State", &self.State).finish()
    }
}
impl ::windows::core::TypeKind for MSA_INFO_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MSA_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        self.State == other.State
    }
}
impl ::core::cmp::Eq for MSA_INFO_0 {}
impl ::core::default::Default for MSA_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct MSG_INFO_0 {
    pub msgi0_name: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for MSG_INFO_0 {}
impl ::core::clone::Clone for MSG_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MSG_INFO_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MSG_INFO_0").field("msgi0_name", &self.msgi0_name).finish()
    }
}
impl ::windows::core::TypeKind for MSG_INFO_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MSG_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        self.msgi0_name == other.msgi0_name
    }
}
impl ::core::cmp::Eq for MSG_INFO_0 {}
impl ::core::default::Default for MSG_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct MSG_INFO_1 {
    pub msgi1_name: ::windows::core::PWSTR,
    pub msgi1_forward_flag: u32,
    pub msgi1_forward: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for MSG_INFO_1 {}
impl ::core::clone::Clone for MSG_INFO_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MSG_INFO_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MSG_INFO_1").field("msgi1_name", &self.msgi1_name).field("msgi1_forward_flag", &self.msgi1_forward_flag).field("msgi1_forward", &self.msgi1_forward).finish()
    }
}
impl ::windows::core::TypeKind for MSG_INFO_1 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MSG_INFO_1 {
    fn eq(&self, other: &Self) -> bool {
        self.msgi1_name == other.msgi1_name && self.msgi1_forward_flag == other.msgi1_forward_flag && self.msgi1_forward == other.msgi1_forward
    }
}
impl ::core::cmp::Eq for MSG_INFO_1 {}
impl ::core::default::Default for MSG_INFO_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct NETLOGON_INFO_1 {
    pub netlog1_flags: u32,
    pub netlog1_pdc_connection_status: u32,
}
impl ::core::marker::Copy for NETLOGON_INFO_1 {}
impl ::core::clone::Clone for NETLOGON_INFO_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NETLOGON_INFO_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NETLOGON_INFO_1").field("netlog1_flags", &self.netlog1_flags).field("netlog1_pdc_connection_status", &self.netlog1_pdc_connection_status).finish()
    }
}
impl ::windows::core::TypeKind for NETLOGON_INFO_1 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NETLOGON_INFO_1 {
    fn eq(&self, other: &Self) -> bool {
        self.netlog1_flags == other.netlog1_flags && self.netlog1_pdc_connection_status == other.netlog1_pdc_connection_status
    }
}
impl ::core::cmp::Eq for NETLOGON_INFO_1 {}
impl ::core::default::Default for NETLOGON_INFO_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct NETLOGON_INFO_2 {
    pub netlog2_flags: u32,
    pub netlog2_pdc_connection_status: u32,
    pub netlog2_trusted_dc_name: ::windows::core::PWSTR,
    pub netlog2_tc_connection_status: u32,
}
impl ::core::marker::Copy for NETLOGON_INFO_2 {}
impl ::core::clone::Clone for NETLOGON_INFO_2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NETLOGON_INFO_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NETLOGON_INFO_2").field("netlog2_flags", &self.netlog2_flags).field("netlog2_pdc_connection_status", &self.netlog2_pdc_connection_status).field("netlog2_trusted_dc_name", &self.netlog2_trusted_dc_name).field("netlog2_tc_connection_status", &self.netlog2_tc_connection_status).finish()
    }
}
impl ::windows::core::TypeKind for NETLOGON_INFO_2 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NETLOGON_INFO_2 {
    fn eq(&self, other: &Self) -> bool {
        self.netlog2_flags == other.netlog2_flags && self.netlog2_pdc_connection_status == other.netlog2_pdc_connection_status && self.netlog2_trusted_dc_name == other.netlog2_trusted_dc_name && self.netlog2_tc_connection_status == other.netlog2_tc_connection_status
    }
}
impl ::core::cmp::Eq for NETLOGON_INFO_2 {}
impl ::core::default::Default for NETLOGON_INFO_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct NETLOGON_INFO_3 {
    pub netlog3_flags: u32,
    pub netlog3_logon_attempts: u32,
    pub netlog3_reserved1: u32,
    pub netlog3_reserved2: u32,
    pub netlog3_reserved3: u32,
    pub netlog3_reserved4: u32,
    pub netlog3_reserved5: u32,
}
impl ::core::marker::Copy for NETLOGON_INFO_3 {}
impl ::core::clone::Clone for NETLOGON_INFO_3 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NETLOGON_INFO_3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NETLOGON_INFO_3").field("netlog3_flags", &self.netlog3_flags).field("netlog3_logon_attempts", &self.netlog3_logon_attempts).field("netlog3_reserved1", &self.netlog3_reserved1).field("netlog3_reserved2", &self.netlog3_reserved2).field("netlog3_reserved3", &self.netlog3_reserved3).field("netlog3_reserved4", &self.netlog3_reserved4).field("netlog3_reserved5", &self.netlog3_reserved5).finish()
    }
}
impl ::windows::core::TypeKind for NETLOGON_INFO_3 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NETLOGON_INFO_3 {
    fn eq(&self, other: &Self) -> bool {
        self.netlog3_flags == other.netlog3_flags && self.netlog3_logon_attempts == other.netlog3_logon_attempts && self.netlog3_reserved1 == other.netlog3_reserved1 && self.netlog3_reserved2 == other.netlog3_reserved2 && self.netlog3_reserved3 == other.netlog3_reserved3 && self.netlog3_reserved4 == other.netlog3_reserved4 && self.netlog3_reserved5 == other.netlog3_reserved5
    }
}
impl ::core::cmp::Eq for NETLOGON_INFO_3 {}
impl ::core::default::Default for NETLOGON_INFO_3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct NETLOGON_INFO_4 {
    pub netlog4_trusted_dc_name: ::windows::core::PWSTR,
    pub netlog4_trusted_domain_name: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for NETLOGON_INFO_4 {}
impl ::core::clone::Clone for NETLOGON_INFO_4 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NETLOGON_INFO_4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NETLOGON_INFO_4").field("netlog4_trusted_dc_name", &self.netlog4_trusted_dc_name).field("netlog4_trusted_domain_name", &self.netlog4_trusted_domain_name).finish()
    }
}
impl ::windows::core::TypeKind for NETLOGON_INFO_4 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NETLOGON_INFO_4 {
    fn eq(&self, other: &Self) -> bool {
        self.netlog4_trusted_dc_name == other.netlog4_trusted_dc_name && self.netlog4_trusted_domain_name == other.netlog4_trusted_domain_name
    }
}
impl ::core::cmp::Eq for NETLOGON_INFO_4 {}
impl ::core::default::Default for NETLOGON_INFO_4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct NETSETUP_PROVISIONING_PARAMS {
    pub dwVersion: u32,
    pub lpDomain: ::windows::core::PCWSTR,
    pub lpHostName: ::windows::core::PCWSTR,
    pub lpMachineAccountOU: ::windows::core::PCWSTR,
    pub lpDcName: ::windows::core::PCWSTR,
    pub dwProvisionOptions: NETSETUP_PROVISION,
    pub aCertTemplateNames: *const ::windows::core::PCWSTR,
    pub cCertTemplateNames: u32,
    pub aMachinePolicyNames: *const ::windows::core::PCWSTR,
    pub cMachinePolicyNames: u32,
    pub aMachinePolicyPaths: *const ::windows::core::PCWSTR,
    pub cMachinePolicyPaths: u32,
    pub lpNetbiosName: ::windows::core::PWSTR,
    pub lpSiteName: ::windows::core::PWSTR,
    pub lpPrimaryDNSDomain: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for NETSETUP_PROVISIONING_PARAMS {}
impl ::core::clone::Clone for NETSETUP_PROVISIONING_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NETSETUP_PROVISIONING_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NETSETUP_PROVISIONING_PARAMS")
            .field("dwVersion", &self.dwVersion)
            .field("lpDomain", &self.lpDomain)
            .field("lpHostName", &self.lpHostName)
            .field("lpMachineAccountOU", &self.lpMachineAccountOU)
            .field("lpDcName", &self.lpDcName)
            .field("dwProvisionOptions", &self.dwProvisionOptions)
            .field("aCertTemplateNames", &self.aCertTemplateNames)
            .field("cCertTemplateNames", &self.cCertTemplateNames)
            .field("aMachinePolicyNames", &self.aMachinePolicyNames)
            .field("cMachinePolicyNames", &self.cMachinePolicyNames)
            .field("aMachinePolicyPaths", &self.aMachinePolicyPaths)
            .field("cMachinePolicyPaths", &self.cMachinePolicyPaths)
            .field("lpNetbiosName", &self.lpNetbiosName)
            .field("lpSiteName", &self.lpSiteName)
            .field("lpPrimaryDNSDomain", &self.lpPrimaryDNSDomain)
            .finish()
    }
}
impl ::windows::core::TypeKind for NETSETUP_PROVISIONING_PARAMS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NETSETUP_PROVISIONING_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.lpDomain == other.lpDomain && self.lpHostName == other.lpHostName && self.lpMachineAccountOU == other.lpMachineAccountOU && self.lpDcName == other.lpDcName && self.dwProvisionOptions == other.dwProvisionOptions && self.aCertTemplateNames == other.aCertTemplateNames && self.cCertTemplateNames == other.cCertTemplateNames && self.aMachinePolicyNames == other.aMachinePolicyNames && self.cMachinePolicyNames == other.cMachinePolicyNames && self.aMachinePolicyPaths == other.aMachinePolicyPaths && self.cMachinePolicyPaths == other.cMachinePolicyPaths && self.lpNetbiosName == other.lpNetbiosName && self.lpSiteName == other.lpSiteName && self.lpPrimaryDNSDomain == other.lpPrimaryDNSDomain
    }
}
impl ::core::cmp::Eq for NETSETUP_PROVISIONING_PARAMS {}
impl ::core::default::Default for NETSETUP_PROVISIONING_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct NETWORK_NAME {
    pub Name: FLAT_STRING,
}
impl ::core::marker::Copy for NETWORK_NAME {}
impl ::core::clone::Clone for NETWORK_NAME {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NETWORK_NAME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NETWORK_NAME").field("Name", &self.Name).finish()
    }
}
impl ::windows::core::TypeKind for NETWORK_NAME {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NETWORK_NAME {
    fn eq(&self, other: &Self) -> bool {
        self.Name == other.Name
    }
}
impl ::core::cmp::Eq for NETWORK_NAME {}
impl ::core::default::Default for NETWORK_NAME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct NET_DISPLAY_GROUP {
    pub grpi3_name: ::windows::core::PWSTR,
    pub grpi3_comment: ::windows::core::PWSTR,
    pub grpi3_group_id: u32,
    pub grpi3_attributes: u32,
    pub grpi3_next_index: u32,
}
impl ::core::marker::Copy for NET_DISPLAY_GROUP {}
impl ::core::clone::Clone for NET_DISPLAY_GROUP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NET_DISPLAY_GROUP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NET_DISPLAY_GROUP").field("grpi3_name", &self.grpi3_name).field("grpi3_comment", &self.grpi3_comment).field("grpi3_group_id", &self.grpi3_group_id).field("grpi3_attributes", &self.grpi3_attributes).field("grpi3_next_index", &self.grpi3_next_index).finish()
    }
}
impl ::windows::core::TypeKind for NET_DISPLAY_GROUP {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NET_DISPLAY_GROUP {
    fn eq(&self, other: &Self) -> bool {
        self.grpi3_name == other.grpi3_name && self.grpi3_comment == other.grpi3_comment && self.grpi3_group_id == other.grpi3_group_id && self.grpi3_attributes == other.grpi3_attributes && self.grpi3_next_index == other.grpi3_next_index
    }
}
impl ::core::cmp::Eq for NET_DISPLAY_GROUP {}
impl ::core::default::Default for NET_DISPLAY_GROUP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct NET_DISPLAY_MACHINE {
    pub usri2_name: ::windows::core::PWSTR,
    pub usri2_comment: ::windows::core::PWSTR,
    pub usri2_flags: USER_ACCOUNT_FLAGS,
    pub usri2_user_id: u32,
    pub usri2_next_index: u32,
}
impl ::core::marker::Copy for NET_DISPLAY_MACHINE {}
impl ::core::clone::Clone for NET_DISPLAY_MACHINE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NET_DISPLAY_MACHINE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NET_DISPLAY_MACHINE").field("usri2_name", &self.usri2_name).field("usri2_comment", &self.usri2_comment).field("usri2_flags", &self.usri2_flags).field("usri2_user_id", &self.usri2_user_id).field("usri2_next_index", &self.usri2_next_index).finish()
    }
}
impl ::windows::core::TypeKind for NET_DISPLAY_MACHINE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NET_DISPLAY_MACHINE {
    fn eq(&self, other: &Self) -> bool {
        self.usri2_name == other.usri2_name && self.usri2_comment == other.usri2_comment && self.usri2_flags == other.usri2_flags && self.usri2_user_id == other.usri2_user_id && self.usri2_next_index == other.usri2_next_index
    }
}
impl ::core::cmp::Eq for NET_DISPLAY_MACHINE {}
impl ::core::default::Default for NET_DISPLAY_MACHINE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct NET_DISPLAY_USER {
    pub usri1_name: ::windows::core::PWSTR,
    pub usri1_comment: ::windows::core::PWSTR,
    pub usri1_flags: USER_ACCOUNT_FLAGS,
    pub usri1_full_name: ::windows::core::PWSTR,
    pub usri1_user_id: u32,
    pub usri1_next_index: u32,
}
impl ::core::marker::Copy for NET_DISPLAY_USER {}
impl ::core::clone::Clone for NET_DISPLAY_USER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NET_DISPLAY_USER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NET_DISPLAY_USER").field("usri1_name", &self.usri1_name).field("usri1_comment", &self.usri1_comment).field("usri1_flags", &self.usri1_flags).field("usri1_full_name", &self.usri1_full_name).field("usri1_user_id", &self.usri1_user_id).field("usri1_next_index", &self.usri1_next_index).finish()
    }
}
impl ::windows::core::TypeKind for NET_DISPLAY_USER {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NET_DISPLAY_USER {
    fn eq(&self, other: &Self) -> bool {
        self.usri1_name == other.usri1_name && self.usri1_comment == other.usri1_comment && self.usri1_flags == other.usri1_flags && self.usri1_full_name == other.usri1_full_name && self.usri1_user_id == other.usri1_user_id && self.usri1_next_index == other.usri1_next_index
    }
}
impl ::core::cmp::Eq for NET_DISPLAY_USER {}
impl ::core::default::Default for NET_DISPLAY_USER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NET_VALIDATE_AUTHENTICATION_INPUT_ARG {
    pub InputPersistedFields: NET_VALIDATE_PERSISTED_FIELDS,
    pub PasswordMatched: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NET_VALIDATE_AUTHENTICATION_INPUT_ARG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NET_VALIDATE_AUTHENTICATION_INPUT_ARG {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NET_VALIDATE_AUTHENTICATION_INPUT_ARG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NET_VALIDATE_AUTHENTICATION_INPUT_ARG").field("InputPersistedFields", &self.InputPersistedFields).field("PasswordMatched", &self.PasswordMatched).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for NET_VALIDATE_AUTHENTICATION_INPUT_ARG {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NET_VALIDATE_AUTHENTICATION_INPUT_ARG {
    fn eq(&self, other: &Self) -> bool {
        self.InputPersistedFields == other.InputPersistedFields && self.PasswordMatched == other.PasswordMatched
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NET_VALIDATE_AUTHENTICATION_INPUT_ARG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NET_VALIDATE_AUTHENTICATION_INPUT_ARG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NET_VALIDATE_OUTPUT_ARG {
    pub ChangedPersistedFields: NET_VALIDATE_PERSISTED_FIELDS,
    pub ValidationStatus: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NET_VALIDATE_OUTPUT_ARG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NET_VALIDATE_OUTPUT_ARG {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NET_VALIDATE_OUTPUT_ARG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NET_VALIDATE_OUTPUT_ARG").field("ChangedPersistedFields", &self.ChangedPersistedFields).field("ValidationStatus", &self.ValidationStatus).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for NET_VALIDATE_OUTPUT_ARG {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NET_VALIDATE_OUTPUT_ARG {
    fn eq(&self, other: &Self) -> bool {
        self.ChangedPersistedFields == other.ChangedPersistedFields && self.ValidationStatus == other.ValidationStatus
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NET_VALIDATE_OUTPUT_ARG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NET_VALIDATE_OUTPUT_ARG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NET_VALIDATE_PASSWORD_CHANGE_INPUT_ARG {
    pub InputPersistedFields: NET_VALIDATE_PERSISTED_FIELDS,
    pub ClearPassword: ::windows::core::PWSTR,
    pub UserAccountName: ::windows::core::PWSTR,
    pub HashedPassword: NET_VALIDATE_PASSWORD_HASH,
    pub PasswordMatch: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NET_VALIDATE_PASSWORD_CHANGE_INPUT_ARG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NET_VALIDATE_PASSWORD_CHANGE_INPUT_ARG {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NET_VALIDATE_PASSWORD_CHANGE_INPUT_ARG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NET_VALIDATE_PASSWORD_CHANGE_INPUT_ARG").field("InputPersistedFields", &self.InputPersistedFields).field("ClearPassword", &self.ClearPassword).field("UserAccountName", &self.UserAccountName).field("HashedPassword", &self.HashedPassword).field("PasswordMatch", &self.PasswordMatch).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for NET_VALIDATE_PASSWORD_CHANGE_INPUT_ARG {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NET_VALIDATE_PASSWORD_CHANGE_INPUT_ARG {
    fn eq(&self, other: &Self) -> bool {
        self.InputPersistedFields == other.InputPersistedFields && self.ClearPassword == other.ClearPassword && self.UserAccountName == other.UserAccountName && self.HashedPassword == other.HashedPassword && self.PasswordMatch == other.PasswordMatch
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NET_VALIDATE_PASSWORD_CHANGE_INPUT_ARG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NET_VALIDATE_PASSWORD_CHANGE_INPUT_ARG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct NET_VALIDATE_PASSWORD_HASH {
    pub Length: u32,
    pub Hash: *mut u8,
}
impl ::core::marker::Copy for NET_VALIDATE_PASSWORD_HASH {}
impl ::core::clone::Clone for NET_VALIDATE_PASSWORD_HASH {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NET_VALIDATE_PASSWORD_HASH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NET_VALIDATE_PASSWORD_HASH").field("Length", &self.Length).field("Hash", &self.Hash).finish()
    }
}
impl ::windows::core::TypeKind for NET_VALIDATE_PASSWORD_HASH {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NET_VALIDATE_PASSWORD_HASH {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.Hash == other.Hash
    }
}
impl ::core::cmp::Eq for NET_VALIDATE_PASSWORD_HASH {}
impl ::core::default::Default for NET_VALIDATE_PASSWORD_HASH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NET_VALIDATE_PASSWORD_RESET_INPUT_ARG {
    pub InputPersistedFields: NET_VALIDATE_PERSISTED_FIELDS,
    pub ClearPassword: ::windows::core::PWSTR,
    pub UserAccountName: ::windows::core::PWSTR,
    pub HashedPassword: NET_VALIDATE_PASSWORD_HASH,
    pub PasswordMustChangeAtNextLogon: super::super::Foundation::BOOLEAN,
    pub ClearLockout: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NET_VALIDATE_PASSWORD_RESET_INPUT_ARG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NET_VALIDATE_PASSWORD_RESET_INPUT_ARG {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NET_VALIDATE_PASSWORD_RESET_INPUT_ARG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NET_VALIDATE_PASSWORD_RESET_INPUT_ARG").field("InputPersistedFields", &self.InputPersistedFields).field("ClearPassword", &self.ClearPassword).field("UserAccountName", &self.UserAccountName).field("HashedPassword", &self.HashedPassword).field("PasswordMustChangeAtNextLogon", &self.PasswordMustChangeAtNextLogon).field("ClearLockout", &self.ClearLockout).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for NET_VALIDATE_PASSWORD_RESET_INPUT_ARG {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NET_VALIDATE_PASSWORD_RESET_INPUT_ARG {
    fn eq(&self, other: &Self) -> bool {
        self.InputPersistedFields == other.InputPersistedFields && self.ClearPassword == other.ClearPassword && self.UserAccountName == other.UserAccountName && self.HashedPassword == other.HashedPassword && self.PasswordMustChangeAtNextLogon == other.PasswordMustChangeAtNextLogon && self.ClearLockout == other.ClearLockout
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NET_VALIDATE_PASSWORD_RESET_INPUT_ARG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NET_VALIDATE_PASSWORD_RESET_INPUT_ARG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NET_VALIDATE_PERSISTED_FIELDS {
    pub PresentFields: u32,
    pub PasswordLastSet: super::super::Foundation::FILETIME,
    pub BadPasswordTime: super::super::Foundation::FILETIME,
    pub LockoutTime: super::super::Foundation::FILETIME,
    pub BadPasswordCount: u32,
    pub PasswordHistoryLength: u32,
    pub PasswordHistory: *mut NET_VALIDATE_PASSWORD_HASH,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NET_VALIDATE_PERSISTED_FIELDS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NET_VALIDATE_PERSISTED_FIELDS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NET_VALIDATE_PERSISTED_FIELDS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NET_VALIDATE_PERSISTED_FIELDS").field("PresentFields", &self.PresentFields).field("PasswordLastSet", &self.PasswordLastSet).field("BadPasswordTime", &self.BadPasswordTime).field("LockoutTime", &self.LockoutTime).field("BadPasswordCount", &self.BadPasswordCount).field("PasswordHistoryLength", &self.PasswordHistoryLength).field("PasswordHistory", &self.PasswordHistory).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for NET_VALIDATE_PERSISTED_FIELDS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NET_VALIDATE_PERSISTED_FIELDS {
    fn eq(&self, other: &Self) -> bool {
        self.PresentFields == other.PresentFields && self.PasswordLastSet == other.PasswordLastSet && self.BadPasswordTime == other.BadPasswordTime && self.LockoutTime == other.LockoutTime && self.BadPasswordCount == other.BadPasswordCount && self.PasswordHistoryLength == other.PasswordHistoryLength && self.PasswordHistory == other.PasswordHistory
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NET_VALIDATE_PERSISTED_FIELDS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NET_VALIDATE_PERSISTED_FIELDS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct OBO_TOKEN {
    pub Type: OBO_TOKEN_TYPE,
    pub pncc: ::std::mem::ManuallyDrop<::core::option::Option<INetCfgComponent>>,
    pub pszwManufacturer: ::windows::core::PCWSTR,
    pub pszwProduct: ::windows::core::PCWSTR,
    pub pszwDisplayName: ::windows::core::PCWSTR,
    pub fRegistered: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OBO_TOKEN {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for OBO_TOKEN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OBO_TOKEN").field("Type", &self.Type).field("pncc", &self.pncc).field("pszwManufacturer", &self.pszwManufacturer).field("pszwProduct", &self.pszwProduct).field("pszwDisplayName", &self.pszwDisplayName).field("fRegistered", &self.fRegistered).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for OBO_TOKEN {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for OBO_TOKEN {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.pncc == other.pncc && self.pszwManufacturer == other.pszwManufacturer && self.pszwProduct == other.pszwProduct && self.pszwDisplayName == other.pszwDisplayName && self.fRegistered == other.fRegistered
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for OBO_TOKEN {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OBO_TOKEN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct PRINT_OTHER_INFO {
    pub alrtpr_jobid: u32,
    pub alrtpr_status: u32,
    pub alrtpr_submitted: u32,
    pub alrtpr_size: u32,
}
impl ::core::marker::Copy for PRINT_OTHER_INFO {}
impl ::core::clone::Clone for PRINT_OTHER_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PRINT_OTHER_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PRINT_OTHER_INFO").field("alrtpr_jobid", &self.alrtpr_jobid).field("alrtpr_status", &self.alrtpr_status).field("alrtpr_submitted", &self.alrtpr_submitted).field("alrtpr_size", &self.alrtpr_size).finish()
    }
}
impl ::windows::core::TypeKind for PRINT_OTHER_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for PRINT_OTHER_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.alrtpr_jobid == other.alrtpr_jobid && self.alrtpr_status == other.alrtpr_status && self.alrtpr_submitted == other.alrtpr_submitted && self.alrtpr_size == other.alrtpr_size
    }
}
impl ::core::cmp::Eq for PRINT_OTHER_INFO {}
impl ::core::default::Default for PRINT_OTHER_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RASCON_IPUI {
    pub guidConnection: ::windows::core::GUID,
    pub fIPv6Cfg: super::super::Foundation::BOOL,
    pub dwFlags: u32,
    pub pszwIpAddr: [u16; 16],
    pub pszwDnsAddr: [u16; 16],
    pub pszwDns2Addr: [u16; 16],
    pub pszwWinsAddr: [u16; 16],
    pub pszwWins2Addr: [u16; 16],
    pub pszwDnsSuffix: [u16; 256],
    pub pszwIpv6Addr: [u16; 65],
    pub dwIpv6PrefixLength: u32,
    pub pszwIpv6DnsAddr: [u16; 65],
    pub pszwIpv6Dns2Addr: [u16; 65],
    pub dwIPv4InfMetric: u32,
    pub dwIPv6InfMetric: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RASCON_IPUI {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RASCON_IPUI {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RASCON_IPUI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RASCON_IPUI")
            .field("guidConnection", &self.guidConnection)
            .field("fIPv6Cfg", &self.fIPv6Cfg)
            .field("dwFlags", &self.dwFlags)
            .field("pszwIpAddr", &self.pszwIpAddr)
            .field("pszwDnsAddr", &self.pszwDnsAddr)
            .field("pszwDns2Addr", &self.pszwDns2Addr)
            .field("pszwWinsAddr", &self.pszwWinsAddr)
            .field("pszwWins2Addr", &self.pszwWins2Addr)
            .field("pszwDnsSuffix", &self.pszwDnsSuffix)
            .field("pszwIpv6Addr", &self.pszwIpv6Addr)
            .field("dwIpv6PrefixLength", &self.dwIpv6PrefixLength)
            .field("pszwIpv6DnsAddr", &self.pszwIpv6DnsAddr)
            .field("pszwIpv6Dns2Addr", &self.pszwIpv6Dns2Addr)
            .field("dwIPv4InfMetric", &self.dwIPv4InfMetric)
            .field("dwIPv6InfMetric", &self.dwIPv6InfMetric)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for RASCON_IPUI {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RASCON_IPUI {
    fn eq(&self, other: &Self) -> bool {
        self.guidConnection == other.guidConnection && self.fIPv6Cfg == other.fIPv6Cfg && self.dwFlags == other.dwFlags && self.pszwIpAddr == other.pszwIpAddr && self.pszwDnsAddr == other.pszwDnsAddr && self.pszwDns2Addr == other.pszwDns2Addr && self.pszwWinsAddr == other.pszwWinsAddr && self.pszwWins2Addr == other.pszwWins2Addr && self.pszwDnsSuffix == other.pszwDnsSuffix && self.pszwIpv6Addr == other.pszwIpv6Addr && self.dwIpv6PrefixLength == other.dwIpv6PrefixLength && self.pszwIpv6DnsAddr == other.pszwIpv6DnsAddr && self.pszwIpv6Dns2Addr == other.pszwIpv6Dns2Addr && self.dwIPv4InfMetric == other.dwIPv4InfMetric && self.dwIPv6InfMetric == other.dwIPv6InfMetric
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RASCON_IPUI {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RASCON_IPUI {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct REPL_EDIR_INFO_0 {
    pub rped0_dirname: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for REPL_EDIR_INFO_0 {}
impl ::core::clone::Clone for REPL_EDIR_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for REPL_EDIR_INFO_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REPL_EDIR_INFO_0").field("rped0_dirname", &self.rped0_dirname).finish()
    }
}
impl ::windows::core::TypeKind for REPL_EDIR_INFO_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for REPL_EDIR_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        self.rped0_dirname == other.rped0_dirname
    }
}
impl ::core::cmp::Eq for REPL_EDIR_INFO_0 {}
impl ::core::default::Default for REPL_EDIR_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct REPL_EDIR_INFO_1 {
    pub rped1_dirname: ::windows::core::PWSTR,
    pub rped1_integrity: u32,
    pub rped1_extent: u32,
}
impl ::core::marker::Copy for REPL_EDIR_INFO_1 {}
impl ::core::clone::Clone for REPL_EDIR_INFO_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for REPL_EDIR_INFO_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REPL_EDIR_INFO_1").field("rped1_dirname", &self.rped1_dirname).field("rped1_integrity", &self.rped1_integrity).field("rped1_extent", &self.rped1_extent).finish()
    }
}
impl ::windows::core::TypeKind for REPL_EDIR_INFO_1 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for REPL_EDIR_INFO_1 {
    fn eq(&self, other: &Self) -> bool {
        self.rped1_dirname == other.rped1_dirname && self.rped1_integrity == other.rped1_integrity && self.rped1_extent == other.rped1_extent
    }
}
impl ::core::cmp::Eq for REPL_EDIR_INFO_1 {}
impl ::core::default::Default for REPL_EDIR_INFO_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct REPL_EDIR_INFO_1000 {
    pub rped1000_integrity: u32,
}
impl ::core::marker::Copy for REPL_EDIR_INFO_1000 {}
impl ::core::clone::Clone for REPL_EDIR_INFO_1000 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for REPL_EDIR_INFO_1000 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REPL_EDIR_INFO_1000").field("rped1000_integrity", &self.rped1000_integrity).finish()
    }
}
impl ::windows::core::TypeKind for REPL_EDIR_INFO_1000 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for REPL_EDIR_INFO_1000 {
    fn eq(&self, other: &Self) -> bool {
        self.rped1000_integrity == other.rped1000_integrity
    }
}
impl ::core::cmp::Eq for REPL_EDIR_INFO_1000 {}
impl ::core::default::Default for REPL_EDIR_INFO_1000 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct REPL_EDIR_INFO_1001 {
    pub rped1001_extent: u32,
}
impl ::core::marker::Copy for REPL_EDIR_INFO_1001 {}
impl ::core::clone::Clone for REPL_EDIR_INFO_1001 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for REPL_EDIR_INFO_1001 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REPL_EDIR_INFO_1001").field("rped1001_extent", &self.rped1001_extent).finish()
    }
}
impl ::windows::core::TypeKind for REPL_EDIR_INFO_1001 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for REPL_EDIR_INFO_1001 {
    fn eq(&self, other: &Self) -> bool {
        self.rped1001_extent == other.rped1001_extent
    }
}
impl ::core::cmp::Eq for REPL_EDIR_INFO_1001 {}
impl ::core::default::Default for REPL_EDIR_INFO_1001 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct REPL_EDIR_INFO_2 {
    pub rped2_dirname: ::windows::core::PWSTR,
    pub rped2_integrity: u32,
    pub rped2_extent: u32,
    pub rped2_lockcount: u32,
    pub rped2_locktime: u32,
}
impl ::core::marker::Copy for REPL_EDIR_INFO_2 {}
impl ::core::clone::Clone for REPL_EDIR_INFO_2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for REPL_EDIR_INFO_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REPL_EDIR_INFO_2").field("rped2_dirname", &self.rped2_dirname).field("rped2_integrity", &self.rped2_integrity).field("rped2_extent", &self.rped2_extent).field("rped2_lockcount", &self.rped2_lockcount).field("rped2_locktime", &self.rped2_locktime).finish()
    }
}
impl ::windows::core::TypeKind for REPL_EDIR_INFO_2 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for REPL_EDIR_INFO_2 {
    fn eq(&self, other: &Self) -> bool {
        self.rped2_dirname == other.rped2_dirname && self.rped2_integrity == other.rped2_integrity && self.rped2_extent == other.rped2_extent && self.rped2_lockcount == other.rped2_lockcount && self.rped2_locktime == other.rped2_locktime
    }
}
impl ::core::cmp::Eq for REPL_EDIR_INFO_2 {}
impl ::core::default::Default for REPL_EDIR_INFO_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct REPL_IDIR_INFO_0 {
    pub rpid0_dirname: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for REPL_IDIR_INFO_0 {}
impl ::core::clone::Clone for REPL_IDIR_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for REPL_IDIR_INFO_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REPL_IDIR_INFO_0").field("rpid0_dirname", &self.rpid0_dirname).finish()
    }
}
impl ::windows::core::TypeKind for REPL_IDIR_INFO_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for REPL_IDIR_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        self.rpid0_dirname == other.rpid0_dirname
    }
}
impl ::core::cmp::Eq for REPL_IDIR_INFO_0 {}
impl ::core::default::Default for REPL_IDIR_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct REPL_IDIR_INFO_1 {
    pub rpid1_dirname: ::windows::core::PWSTR,
    pub rpid1_state: u32,
    pub rpid1_mastername: ::windows::core::PWSTR,
    pub rpid1_last_update_time: u32,
    pub rpid1_lockcount: u32,
    pub rpid1_locktime: u32,
}
impl ::core::marker::Copy for REPL_IDIR_INFO_1 {}
impl ::core::clone::Clone for REPL_IDIR_INFO_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for REPL_IDIR_INFO_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REPL_IDIR_INFO_1").field("rpid1_dirname", &self.rpid1_dirname).field("rpid1_state", &self.rpid1_state).field("rpid1_mastername", &self.rpid1_mastername).field("rpid1_last_update_time", &self.rpid1_last_update_time).field("rpid1_lockcount", &self.rpid1_lockcount).field("rpid1_locktime", &self.rpid1_locktime).finish()
    }
}
impl ::windows::core::TypeKind for REPL_IDIR_INFO_1 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for REPL_IDIR_INFO_1 {
    fn eq(&self, other: &Self) -> bool {
        self.rpid1_dirname == other.rpid1_dirname && self.rpid1_state == other.rpid1_state && self.rpid1_mastername == other.rpid1_mastername && self.rpid1_last_update_time == other.rpid1_last_update_time && self.rpid1_lockcount == other.rpid1_lockcount && self.rpid1_locktime == other.rpid1_locktime
    }
}
impl ::core::cmp::Eq for REPL_IDIR_INFO_1 {}
impl ::core::default::Default for REPL_IDIR_INFO_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct REPL_INFO_0 {
    pub rp0_role: u32,
    pub rp0_exportpath: ::windows::core::PWSTR,
    pub rp0_exportlist: ::windows::core::PWSTR,
    pub rp0_importpath: ::windows::core::PWSTR,
    pub rp0_importlist: ::windows::core::PWSTR,
    pub rp0_logonusername: ::windows::core::PWSTR,
    pub rp0_interval: u32,
    pub rp0_pulse: u32,
    pub rp0_guardtime: u32,
    pub rp0_random: u32,
}
impl ::core::marker::Copy for REPL_INFO_0 {}
impl ::core::clone::Clone for REPL_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for REPL_INFO_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REPL_INFO_0").field("rp0_role", &self.rp0_role).field("rp0_exportpath", &self.rp0_exportpath).field("rp0_exportlist", &self.rp0_exportlist).field("rp0_importpath", &self.rp0_importpath).field("rp0_importlist", &self.rp0_importlist).field("rp0_logonusername", &self.rp0_logonusername).field("rp0_interval", &self.rp0_interval).field("rp0_pulse", &self.rp0_pulse).field("rp0_guardtime", &self.rp0_guardtime).field("rp0_random", &self.rp0_random).finish()
    }
}
impl ::windows::core::TypeKind for REPL_INFO_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for REPL_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        self.rp0_role == other.rp0_role && self.rp0_exportpath == other.rp0_exportpath && self.rp0_exportlist == other.rp0_exportlist && self.rp0_importpath == other.rp0_importpath && self.rp0_importlist == other.rp0_importlist && self.rp0_logonusername == other.rp0_logonusername && self.rp0_interval == other.rp0_interval && self.rp0_pulse == other.rp0_pulse && self.rp0_guardtime == other.rp0_guardtime && self.rp0_random == other.rp0_random
    }
}
impl ::core::cmp::Eq for REPL_INFO_0 {}
impl ::core::default::Default for REPL_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct REPL_INFO_1000 {
    pub rp1000_interval: u32,
}
impl ::core::marker::Copy for REPL_INFO_1000 {}
impl ::core::clone::Clone for REPL_INFO_1000 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for REPL_INFO_1000 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REPL_INFO_1000").field("rp1000_interval", &self.rp1000_interval).finish()
    }
}
impl ::windows::core::TypeKind for REPL_INFO_1000 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for REPL_INFO_1000 {
    fn eq(&self, other: &Self) -> bool {
        self.rp1000_interval == other.rp1000_interval
    }
}
impl ::core::cmp::Eq for REPL_INFO_1000 {}
impl ::core::default::Default for REPL_INFO_1000 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct REPL_INFO_1001 {
    pub rp1001_pulse: u32,
}
impl ::core::marker::Copy for REPL_INFO_1001 {}
impl ::core::clone::Clone for REPL_INFO_1001 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for REPL_INFO_1001 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REPL_INFO_1001").field("rp1001_pulse", &self.rp1001_pulse).finish()
    }
}
impl ::windows::core::TypeKind for REPL_INFO_1001 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for REPL_INFO_1001 {
    fn eq(&self, other: &Self) -> bool {
        self.rp1001_pulse == other.rp1001_pulse
    }
}
impl ::core::cmp::Eq for REPL_INFO_1001 {}
impl ::core::default::Default for REPL_INFO_1001 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct REPL_INFO_1002 {
    pub rp1002_guardtime: u32,
}
impl ::core::marker::Copy for REPL_INFO_1002 {}
impl ::core::clone::Clone for REPL_INFO_1002 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for REPL_INFO_1002 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REPL_INFO_1002").field("rp1002_guardtime", &self.rp1002_guardtime).finish()
    }
}
impl ::windows::core::TypeKind for REPL_INFO_1002 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for REPL_INFO_1002 {
    fn eq(&self, other: &Self) -> bool {
        self.rp1002_guardtime == other.rp1002_guardtime
    }
}
impl ::core::cmp::Eq for REPL_INFO_1002 {}
impl ::core::default::Default for REPL_INFO_1002 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct REPL_INFO_1003 {
    pub rp1003_random: u32,
}
impl ::core::marker::Copy for REPL_INFO_1003 {}
impl ::core::clone::Clone for REPL_INFO_1003 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for REPL_INFO_1003 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REPL_INFO_1003").field("rp1003_random", &self.rp1003_random).finish()
    }
}
impl ::windows::core::TypeKind for REPL_INFO_1003 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for REPL_INFO_1003 {
    fn eq(&self, other: &Self) -> bool {
        self.rp1003_random == other.rp1003_random
    }
}
impl ::core::cmp::Eq for REPL_INFO_1003 {}
impl ::core::default::Default for REPL_INFO_1003 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct RTR_INFO_BLOCK_HEADER {
    pub Version: u32,
    pub Size: u32,
    pub TocEntriesCount: u32,
    pub TocEntry: [RTR_TOC_ENTRY; 1],
}
impl ::core::marker::Copy for RTR_INFO_BLOCK_HEADER {}
impl ::core::clone::Clone for RTR_INFO_BLOCK_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RTR_INFO_BLOCK_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RTR_INFO_BLOCK_HEADER").field("Version", &self.Version).field("Size", &self.Size).field("TocEntriesCount", &self.TocEntriesCount).field("TocEntry", &self.TocEntry).finish()
    }
}
impl ::windows::core::TypeKind for RTR_INFO_BLOCK_HEADER {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for RTR_INFO_BLOCK_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.TocEntriesCount == other.TocEntriesCount && self.TocEntry == other.TocEntry
    }
}
impl ::core::cmp::Eq for RTR_INFO_BLOCK_HEADER {}
impl ::core::default::Default for RTR_INFO_BLOCK_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct RTR_TOC_ENTRY {
    pub InfoType: u32,
    pub InfoSize: u32,
    pub Count: u32,
    pub Offset: u32,
}
impl ::core::marker::Copy for RTR_TOC_ENTRY {}
impl ::core::clone::Clone for RTR_TOC_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RTR_TOC_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RTR_TOC_ENTRY").field("InfoType", &self.InfoType).field("InfoSize", &self.InfoSize).field("Count", &self.Count).field("Offset", &self.Offset).finish()
    }
}
impl ::windows::core::TypeKind for RTR_TOC_ENTRY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for RTR_TOC_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.InfoType == other.InfoType && self.InfoSize == other.InfoSize && self.Count == other.Count && self.Offset == other.Offset
    }
}
impl ::core::cmp::Eq for RTR_TOC_ENTRY {}
impl ::core::default::Default for RTR_TOC_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SERVER_INFO_100 {
    pub sv100_platform_id: u32,
    pub sv100_name: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for SERVER_INFO_100 {}
impl ::core::clone::Clone for SERVER_INFO_100 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVER_INFO_100 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_100").field("sv100_platform_id", &self.sv100_platform_id).field("sv100_name", &self.sv100_name).finish()
    }
}
impl ::windows::core::TypeKind for SERVER_INFO_100 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVER_INFO_100 {
    fn eq(&self, other: &Self) -> bool {
        self.sv100_platform_id == other.sv100_platform_id && self.sv100_name == other.sv100_name
    }
}
impl ::core::cmp::Eq for SERVER_INFO_100 {}
impl ::core::default::Default for SERVER_INFO_100 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SERVER_INFO_1005 {
    pub sv1005_comment: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for SERVER_INFO_1005 {}
impl ::core::clone::Clone for SERVER_INFO_1005 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVER_INFO_1005 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1005").field("sv1005_comment", &self.sv1005_comment).finish()
    }
}
impl ::windows::core::TypeKind for SERVER_INFO_1005 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1005 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1005_comment == other.sv1005_comment
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1005 {}
impl ::core::default::Default for SERVER_INFO_1005 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SERVER_INFO_101 {
    pub sv101_platform_id: u32,
    pub sv101_name: ::windows::core::PWSTR,
    pub sv101_version_major: u32,
    pub sv101_version_minor: u32,
    pub sv101_type: NET_SERVER_TYPE,
    pub sv101_comment: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for SERVER_INFO_101 {}
impl ::core::clone::Clone for SERVER_INFO_101 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVER_INFO_101 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_101").field("sv101_platform_id", &self.sv101_platform_id).field("sv101_name", &self.sv101_name).field("sv101_version_major", &self.sv101_version_major).field("sv101_version_minor", &self.sv101_version_minor).field("sv101_type", &self.sv101_type).field("sv101_comment", &self.sv101_comment).finish()
    }
}
impl ::windows::core::TypeKind for SERVER_INFO_101 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVER_INFO_101 {
    fn eq(&self, other: &Self) -> bool {
        self.sv101_platform_id == other.sv101_platform_id && self.sv101_name == other.sv101_name && self.sv101_version_major == other.sv101_version_major && self.sv101_version_minor == other.sv101_version_minor && self.sv101_type == other.sv101_type && self.sv101_comment == other.sv101_comment
    }
}
impl ::core::cmp::Eq for SERVER_INFO_101 {}
impl ::core::default::Default for SERVER_INFO_101 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SERVER_INFO_1010 {
    pub sv1010_disc: i32,
}
impl ::core::marker::Copy for SERVER_INFO_1010 {}
impl ::core::clone::Clone for SERVER_INFO_1010 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVER_INFO_1010 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1010").field("sv1010_disc", &self.sv1010_disc).finish()
    }
}
impl ::windows::core::TypeKind for SERVER_INFO_1010 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1010 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1010_disc == other.sv1010_disc
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1010 {}
impl ::core::default::Default for SERVER_INFO_1010 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SERVER_INFO_1016 {
    pub sv1016_hidden: SERVER_INFO_HIDDEN,
}
impl ::core::marker::Copy for SERVER_INFO_1016 {}
impl ::core::clone::Clone for SERVER_INFO_1016 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVER_INFO_1016 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1016").field("sv1016_hidden", &self.sv1016_hidden).finish()
    }
}
impl ::windows::core::TypeKind for SERVER_INFO_1016 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1016 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1016_hidden == other.sv1016_hidden
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1016 {}
impl ::core::default::Default for SERVER_INFO_1016 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SERVER_INFO_1017 {
    pub sv1017_announce: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1017 {}
impl ::core::clone::Clone for SERVER_INFO_1017 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVER_INFO_1017 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1017").field("sv1017_announce", &self.sv1017_announce).finish()
    }
}
impl ::windows::core::TypeKind for SERVER_INFO_1017 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1017 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1017_announce == other.sv1017_announce
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1017 {}
impl ::core::default::Default for SERVER_INFO_1017 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SERVER_INFO_1018 {
    pub sv1018_anndelta: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1018 {}
impl ::core::clone::Clone for SERVER_INFO_1018 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVER_INFO_1018 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1018").field("sv1018_anndelta", &self.sv1018_anndelta).finish()
    }
}
impl ::windows::core::TypeKind for SERVER_INFO_1018 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1018 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1018_anndelta == other.sv1018_anndelta
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1018 {}
impl ::core::default::Default for SERVER_INFO_1018 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SERVER_INFO_102 {
    pub sv102_platform_id: u32,
    pub sv102_name: ::windows::core::PWSTR,
    pub sv102_version_major: u32,
    pub sv102_version_minor: u32,
    pub sv102_type: NET_SERVER_TYPE,
    pub sv102_comment: ::windows::core::PWSTR,
    pub sv102_users: u32,
    pub sv102_disc: i32,
    pub sv102_hidden: SERVER_INFO_HIDDEN,
    pub sv102_announce: u32,
    pub sv102_anndelta: u32,
    pub sv102_licenses: u32,
    pub sv102_userpath: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for SERVER_INFO_102 {}
impl ::core::clone::Clone for SERVER_INFO_102 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVER_INFO_102 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_102")
            .field("sv102_platform_id", &self.sv102_platform_id)
            .field("sv102_name", &self.sv102_name)
            .field("sv102_version_major", &self.sv102_version_major)
            .field("sv102_version_minor", &self.sv102_version_minor)
            .field("sv102_type", &self.sv102_type)
            .field("sv102_comment", &self.sv102_comment)
            .field("sv102_users", &self.sv102_users)
            .field("sv102_disc", &self.sv102_disc)
            .field("sv102_hidden", &self.sv102_hidden)
            .field("sv102_announce", &self.sv102_announce)
            .field("sv102_anndelta", &self.sv102_anndelta)
            .field("sv102_licenses", &self.sv102_licenses)
            .field("sv102_userpath", &self.sv102_userpath)
            .finish()
    }
}
impl ::windows::core::TypeKind for SERVER_INFO_102 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVER_INFO_102 {
    fn eq(&self, other: &Self) -> bool {
        self.sv102_platform_id == other.sv102_platform_id && self.sv102_name == other.sv102_name && self.sv102_version_major == other.sv102_version_major && self.sv102_version_minor == other.sv102_version_minor && self.sv102_type == other.sv102_type && self.sv102_comment == other.sv102_comment && self.sv102_users == other.sv102_users && self.sv102_disc == other.sv102_disc && self.sv102_hidden == other.sv102_hidden && self.sv102_announce == other.sv102_announce && self.sv102_anndelta == other.sv102_anndelta && self.sv102_licenses == other.sv102_licenses && self.sv102_userpath == other.sv102_userpath
    }
}
impl ::core::cmp::Eq for SERVER_INFO_102 {}
impl ::core::default::Default for SERVER_INFO_102 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SERVER_INFO_103 {
    pub sv103_platform_id: u32,
    pub sv103_name: ::windows::core::PWSTR,
    pub sv103_version_major: u32,
    pub sv103_version_minor: u32,
    pub sv103_type: u32,
    pub sv103_comment: ::windows::core::PWSTR,
    pub sv103_users: u32,
    pub sv103_disc: i32,
    pub sv103_hidden: super::super::Foundation::BOOL,
    pub sv103_announce: u32,
    pub sv103_anndelta: u32,
    pub sv103_licenses: u32,
    pub sv103_userpath: ::windows::core::PWSTR,
    pub sv103_capabilities: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SERVER_INFO_103 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SERVER_INFO_103 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SERVER_INFO_103 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_103")
            .field("sv103_platform_id", &self.sv103_platform_id)
            .field("sv103_name", &self.sv103_name)
            .field("sv103_version_major", &self.sv103_version_major)
            .field("sv103_version_minor", &self.sv103_version_minor)
            .field("sv103_type", &self.sv103_type)
            .field("sv103_comment", &self.sv103_comment)
            .field("sv103_users", &self.sv103_users)
            .field("sv103_disc", &self.sv103_disc)
            .field("sv103_hidden", &self.sv103_hidden)
            .field("sv103_announce", &self.sv103_announce)
            .field("sv103_anndelta", &self.sv103_anndelta)
            .field("sv103_licenses", &self.sv103_licenses)
            .field("sv103_userpath", &self.sv103_userpath)
            .field("sv103_capabilities", &self.sv103_capabilities)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for SERVER_INFO_103 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVER_INFO_103 {
    fn eq(&self, other: &Self) -> bool {
        self.sv103_platform_id == other.sv103_platform_id && self.sv103_name == other.sv103_name && self.sv103_version_major == other.sv103_version_major && self.sv103_version_minor == other.sv103_version_minor && self.sv103_type == other.sv103_type && self.sv103_comment == other.sv103_comment && self.sv103_users == other.sv103_users && self.sv103_disc == other.sv103_disc && self.sv103_hidden == other.sv103_hidden && self.sv103_announce == other.sv103_announce && self.sv103_anndelta == other.sv103_anndelta && self.sv103_licenses == other.sv103_licenses && self.sv103_userpath == other.sv103_userpath && self.sv103_capabilities == other.sv103_capabilities
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVER_INFO_103 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVER_INFO_103 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SERVER_INFO_1107 {
    pub sv1107_users: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1107 {}
impl ::core::clone::Clone for SERVER_INFO_1107 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVER_INFO_1107 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1107").field("sv1107_users", &self.sv1107_users).finish()
    }
}
impl ::windows::core::TypeKind for SERVER_INFO_1107 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1107 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1107_users == other.sv1107_users
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1107 {}
impl ::core::default::Default for SERVER_INFO_1107 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SERVER_INFO_1501 {
    pub sv1501_sessopens: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1501 {}
impl ::core::clone::Clone for SERVER_INFO_1501 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVER_INFO_1501 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1501").field("sv1501_sessopens", &self.sv1501_sessopens).finish()
    }
}
impl ::windows::core::TypeKind for SERVER_INFO_1501 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1501 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1501_sessopens == other.sv1501_sessopens
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1501 {}
impl ::core::default::Default for SERVER_INFO_1501 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SERVER_INFO_1502 {
    pub sv1502_sessvcs: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1502 {}
impl ::core::clone::Clone for SERVER_INFO_1502 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVER_INFO_1502 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1502").field("sv1502_sessvcs", &self.sv1502_sessvcs).finish()
    }
}
impl ::windows::core::TypeKind for SERVER_INFO_1502 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1502 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1502_sessvcs == other.sv1502_sessvcs
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1502 {}
impl ::core::default::Default for SERVER_INFO_1502 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SERVER_INFO_1503 {
    pub sv1503_opensearch: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1503 {}
impl ::core::clone::Clone for SERVER_INFO_1503 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVER_INFO_1503 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1503").field("sv1503_opensearch", &self.sv1503_opensearch).finish()
    }
}
impl ::windows::core::TypeKind for SERVER_INFO_1503 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1503 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1503_opensearch == other.sv1503_opensearch
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1503 {}
impl ::core::default::Default for SERVER_INFO_1503 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SERVER_INFO_1506 {
    pub sv1506_maxworkitems: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1506 {}
impl ::core::clone::Clone for SERVER_INFO_1506 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVER_INFO_1506 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1506").field("sv1506_maxworkitems", &self.sv1506_maxworkitems).finish()
    }
}
impl ::windows::core::TypeKind for SERVER_INFO_1506 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1506 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1506_maxworkitems == other.sv1506_maxworkitems
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1506 {}
impl ::core::default::Default for SERVER_INFO_1506 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SERVER_INFO_1509 {
    pub sv1509_maxrawbuflen: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1509 {}
impl ::core::clone::Clone for SERVER_INFO_1509 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVER_INFO_1509 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1509").field("sv1509_maxrawbuflen", &self.sv1509_maxrawbuflen).finish()
    }
}
impl ::windows::core::TypeKind for SERVER_INFO_1509 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1509 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1509_maxrawbuflen == other.sv1509_maxrawbuflen
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1509 {}
impl ::core::default::Default for SERVER_INFO_1509 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SERVER_INFO_1510 {
    pub sv1510_sessusers: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1510 {}
impl ::core::clone::Clone for SERVER_INFO_1510 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVER_INFO_1510 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1510").field("sv1510_sessusers", &self.sv1510_sessusers).finish()
    }
}
impl ::windows::core::TypeKind for SERVER_INFO_1510 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1510 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1510_sessusers == other.sv1510_sessusers
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1510 {}
impl ::core::default::Default for SERVER_INFO_1510 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SERVER_INFO_1511 {
    pub sv1511_sessconns: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1511 {}
impl ::core::clone::Clone for SERVER_INFO_1511 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVER_INFO_1511 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1511").field("sv1511_sessconns", &self.sv1511_sessconns).finish()
    }
}
impl ::windows::core::TypeKind for SERVER_INFO_1511 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1511 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1511_sessconns == other.sv1511_sessconns
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1511 {}
impl ::core::default::Default for SERVER_INFO_1511 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SERVER_INFO_1512 {
    pub sv1512_maxnonpagedmemoryusage: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1512 {}
impl ::core::clone::Clone for SERVER_INFO_1512 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVER_INFO_1512 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1512").field("sv1512_maxnonpagedmemoryusage", &self.sv1512_maxnonpagedmemoryusage).finish()
    }
}
impl ::windows::core::TypeKind for SERVER_INFO_1512 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1512 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1512_maxnonpagedmemoryusage == other.sv1512_maxnonpagedmemoryusage
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1512 {}
impl ::core::default::Default for SERVER_INFO_1512 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SERVER_INFO_1513 {
    pub sv1513_maxpagedmemoryusage: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1513 {}
impl ::core::clone::Clone for SERVER_INFO_1513 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVER_INFO_1513 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1513").field("sv1513_maxpagedmemoryusage", &self.sv1513_maxpagedmemoryusage).finish()
    }
}
impl ::windows::core::TypeKind for SERVER_INFO_1513 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1513 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1513_maxpagedmemoryusage == other.sv1513_maxpagedmemoryusage
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1513 {}
impl ::core::default::Default for SERVER_INFO_1513 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SERVER_INFO_1514 {
    pub sv1514_enablesoftcompat: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SERVER_INFO_1514 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SERVER_INFO_1514 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SERVER_INFO_1514 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1514").field("sv1514_enablesoftcompat", &self.sv1514_enablesoftcompat).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for SERVER_INFO_1514 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVER_INFO_1514 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1514_enablesoftcompat == other.sv1514_enablesoftcompat
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVER_INFO_1514 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVER_INFO_1514 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SERVER_INFO_1515 {
    pub sv1515_enableforcedlogoff: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SERVER_INFO_1515 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SERVER_INFO_1515 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SERVER_INFO_1515 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1515").field("sv1515_enableforcedlogoff", &self.sv1515_enableforcedlogoff).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for SERVER_INFO_1515 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVER_INFO_1515 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1515_enableforcedlogoff == other.sv1515_enableforcedlogoff
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVER_INFO_1515 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVER_INFO_1515 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SERVER_INFO_1516 {
    pub sv1516_timesource: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SERVER_INFO_1516 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SERVER_INFO_1516 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SERVER_INFO_1516 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1516").field("sv1516_timesource", &self.sv1516_timesource).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for SERVER_INFO_1516 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVER_INFO_1516 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1516_timesource == other.sv1516_timesource
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVER_INFO_1516 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVER_INFO_1516 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SERVER_INFO_1518 {
    pub sv1518_lmannounce: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SERVER_INFO_1518 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SERVER_INFO_1518 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SERVER_INFO_1518 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1518").field("sv1518_lmannounce", &self.sv1518_lmannounce).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for SERVER_INFO_1518 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVER_INFO_1518 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1518_lmannounce == other.sv1518_lmannounce
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVER_INFO_1518 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVER_INFO_1518 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SERVER_INFO_1520 {
    pub sv1520_maxcopyreadlen: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1520 {}
impl ::core::clone::Clone for SERVER_INFO_1520 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVER_INFO_1520 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1520").field("sv1520_maxcopyreadlen", &self.sv1520_maxcopyreadlen).finish()
    }
}
impl ::windows::core::TypeKind for SERVER_INFO_1520 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1520 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1520_maxcopyreadlen == other.sv1520_maxcopyreadlen
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1520 {}
impl ::core::default::Default for SERVER_INFO_1520 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SERVER_INFO_1521 {
    pub sv1521_maxcopywritelen: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1521 {}
impl ::core::clone::Clone for SERVER_INFO_1521 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVER_INFO_1521 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1521").field("sv1521_maxcopywritelen", &self.sv1521_maxcopywritelen).finish()
    }
}
impl ::windows::core::TypeKind for SERVER_INFO_1521 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1521 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1521_maxcopywritelen == other.sv1521_maxcopywritelen
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1521 {}
impl ::core::default::Default for SERVER_INFO_1521 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SERVER_INFO_1522 {
    pub sv1522_minkeepsearch: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1522 {}
impl ::core::clone::Clone for SERVER_INFO_1522 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVER_INFO_1522 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1522").field("sv1522_minkeepsearch", &self.sv1522_minkeepsearch).finish()
    }
}
impl ::windows::core::TypeKind for SERVER_INFO_1522 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1522 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1522_minkeepsearch == other.sv1522_minkeepsearch
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1522 {}
impl ::core::default::Default for SERVER_INFO_1522 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SERVER_INFO_1523 {
    pub sv1523_maxkeepsearch: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1523 {}
impl ::core::clone::Clone for SERVER_INFO_1523 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVER_INFO_1523 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1523").field("sv1523_maxkeepsearch", &self.sv1523_maxkeepsearch).finish()
    }
}
impl ::windows::core::TypeKind for SERVER_INFO_1523 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1523 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1523_maxkeepsearch == other.sv1523_maxkeepsearch
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1523 {}
impl ::core::default::Default for SERVER_INFO_1523 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SERVER_INFO_1524 {
    pub sv1524_minkeepcomplsearch: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1524 {}
impl ::core::clone::Clone for SERVER_INFO_1524 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVER_INFO_1524 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1524").field("sv1524_minkeepcomplsearch", &self.sv1524_minkeepcomplsearch).finish()
    }
}
impl ::windows::core::TypeKind for SERVER_INFO_1524 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1524 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1524_minkeepcomplsearch == other.sv1524_minkeepcomplsearch
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1524 {}
impl ::core::default::Default for SERVER_INFO_1524 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SERVER_INFO_1525 {
    pub sv1525_maxkeepcomplsearch: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1525 {}
impl ::core::clone::Clone for SERVER_INFO_1525 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVER_INFO_1525 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1525").field("sv1525_maxkeepcomplsearch", &self.sv1525_maxkeepcomplsearch).finish()
    }
}
impl ::windows::core::TypeKind for SERVER_INFO_1525 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1525 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1525_maxkeepcomplsearch == other.sv1525_maxkeepcomplsearch
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1525 {}
impl ::core::default::Default for SERVER_INFO_1525 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SERVER_INFO_1528 {
    pub sv1528_scavtimeout: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1528 {}
impl ::core::clone::Clone for SERVER_INFO_1528 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVER_INFO_1528 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1528").field("sv1528_scavtimeout", &self.sv1528_scavtimeout).finish()
    }
}
impl ::windows::core::TypeKind for SERVER_INFO_1528 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1528 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1528_scavtimeout == other.sv1528_scavtimeout
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1528 {}
impl ::core::default::Default for SERVER_INFO_1528 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SERVER_INFO_1529 {
    pub sv1529_minrcvqueue: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1529 {}
impl ::core::clone::Clone for SERVER_INFO_1529 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVER_INFO_1529 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1529").field("sv1529_minrcvqueue", &self.sv1529_minrcvqueue).finish()
    }
}
impl ::windows::core::TypeKind for SERVER_INFO_1529 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1529 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1529_minrcvqueue == other.sv1529_minrcvqueue
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1529 {}
impl ::core::default::Default for SERVER_INFO_1529 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SERVER_INFO_1530 {
    pub sv1530_minfreeworkitems: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1530 {}
impl ::core::clone::Clone for SERVER_INFO_1530 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVER_INFO_1530 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1530").field("sv1530_minfreeworkitems", &self.sv1530_minfreeworkitems).finish()
    }
}
impl ::windows::core::TypeKind for SERVER_INFO_1530 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1530 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1530_minfreeworkitems == other.sv1530_minfreeworkitems
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1530 {}
impl ::core::default::Default for SERVER_INFO_1530 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SERVER_INFO_1533 {
    pub sv1533_maxmpxct: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1533 {}
impl ::core::clone::Clone for SERVER_INFO_1533 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVER_INFO_1533 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1533").field("sv1533_maxmpxct", &self.sv1533_maxmpxct).finish()
    }
}
impl ::windows::core::TypeKind for SERVER_INFO_1533 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1533 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1533_maxmpxct == other.sv1533_maxmpxct
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1533 {}
impl ::core::default::Default for SERVER_INFO_1533 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SERVER_INFO_1534 {
    pub sv1534_oplockbreakwait: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1534 {}
impl ::core::clone::Clone for SERVER_INFO_1534 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVER_INFO_1534 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1534").field("sv1534_oplockbreakwait", &self.sv1534_oplockbreakwait).finish()
    }
}
impl ::windows::core::TypeKind for SERVER_INFO_1534 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1534 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1534_oplockbreakwait == other.sv1534_oplockbreakwait
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1534 {}
impl ::core::default::Default for SERVER_INFO_1534 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SERVER_INFO_1535 {
    pub sv1535_oplockbreakresponsewait: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1535 {}
impl ::core::clone::Clone for SERVER_INFO_1535 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVER_INFO_1535 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1535").field("sv1535_oplockbreakresponsewait", &self.sv1535_oplockbreakresponsewait).finish()
    }
}
impl ::windows::core::TypeKind for SERVER_INFO_1535 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1535 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1535_oplockbreakresponsewait == other.sv1535_oplockbreakresponsewait
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1535 {}
impl ::core::default::Default for SERVER_INFO_1535 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SERVER_INFO_1536 {
    pub sv1536_enableoplocks: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SERVER_INFO_1536 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SERVER_INFO_1536 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SERVER_INFO_1536 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1536").field("sv1536_enableoplocks", &self.sv1536_enableoplocks).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for SERVER_INFO_1536 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVER_INFO_1536 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1536_enableoplocks == other.sv1536_enableoplocks
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVER_INFO_1536 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVER_INFO_1536 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SERVER_INFO_1537 {
    pub sv1537_enableoplockforceclose: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SERVER_INFO_1537 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SERVER_INFO_1537 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SERVER_INFO_1537 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1537").field("sv1537_enableoplockforceclose", &self.sv1537_enableoplockforceclose).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for SERVER_INFO_1537 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVER_INFO_1537 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1537_enableoplockforceclose == other.sv1537_enableoplockforceclose
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVER_INFO_1537 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVER_INFO_1537 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SERVER_INFO_1538 {
    pub sv1538_enablefcbopens: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SERVER_INFO_1538 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SERVER_INFO_1538 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SERVER_INFO_1538 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1538").field("sv1538_enablefcbopens", &self.sv1538_enablefcbopens).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for SERVER_INFO_1538 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVER_INFO_1538 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1538_enablefcbopens == other.sv1538_enablefcbopens
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVER_INFO_1538 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVER_INFO_1538 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SERVER_INFO_1539 {
    pub sv1539_enableraw: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SERVER_INFO_1539 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SERVER_INFO_1539 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SERVER_INFO_1539 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1539").field("sv1539_enableraw", &self.sv1539_enableraw).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for SERVER_INFO_1539 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVER_INFO_1539 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1539_enableraw == other.sv1539_enableraw
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVER_INFO_1539 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVER_INFO_1539 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SERVER_INFO_1540 {
    pub sv1540_enablesharednetdrives: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SERVER_INFO_1540 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SERVER_INFO_1540 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SERVER_INFO_1540 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1540").field("sv1540_enablesharednetdrives", &self.sv1540_enablesharednetdrives).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for SERVER_INFO_1540 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVER_INFO_1540 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1540_enablesharednetdrives == other.sv1540_enablesharednetdrives
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVER_INFO_1540 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVER_INFO_1540 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SERVER_INFO_1541 {
    pub sv1541_minfreeconnections: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SERVER_INFO_1541 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SERVER_INFO_1541 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SERVER_INFO_1541 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1541").field("sv1541_minfreeconnections", &self.sv1541_minfreeconnections).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for SERVER_INFO_1541 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVER_INFO_1541 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1541_minfreeconnections == other.sv1541_minfreeconnections
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVER_INFO_1541 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVER_INFO_1541 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SERVER_INFO_1542 {
    pub sv1542_maxfreeconnections: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SERVER_INFO_1542 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SERVER_INFO_1542 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SERVER_INFO_1542 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1542").field("sv1542_maxfreeconnections", &self.sv1542_maxfreeconnections).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for SERVER_INFO_1542 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVER_INFO_1542 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1542_maxfreeconnections == other.sv1542_maxfreeconnections
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVER_INFO_1542 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVER_INFO_1542 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SERVER_INFO_1543 {
    pub sv1543_initsesstable: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1543 {}
impl ::core::clone::Clone for SERVER_INFO_1543 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVER_INFO_1543 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1543").field("sv1543_initsesstable", &self.sv1543_initsesstable).finish()
    }
}
impl ::windows::core::TypeKind for SERVER_INFO_1543 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1543 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1543_initsesstable == other.sv1543_initsesstable
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1543 {}
impl ::core::default::Default for SERVER_INFO_1543 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SERVER_INFO_1544 {
    pub sv1544_initconntable: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1544 {}
impl ::core::clone::Clone for SERVER_INFO_1544 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVER_INFO_1544 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1544").field("sv1544_initconntable", &self.sv1544_initconntable).finish()
    }
}
impl ::windows::core::TypeKind for SERVER_INFO_1544 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1544 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1544_initconntable == other.sv1544_initconntable
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1544 {}
impl ::core::default::Default for SERVER_INFO_1544 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SERVER_INFO_1545 {
    pub sv1545_initfiletable: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1545 {}
impl ::core::clone::Clone for SERVER_INFO_1545 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVER_INFO_1545 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1545").field("sv1545_initfiletable", &self.sv1545_initfiletable).finish()
    }
}
impl ::windows::core::TypeKind for SERVER_INFO_1545 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1545 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1545_initfiletable == other.sv1545_initfiletable
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1545 {}
impl ::core::default::Default for SERVER_INFO_1545 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SERVER_INFO_1546 {
    pub sv1546_initsearchtable: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1546 {}
impl ::core::clone::Clone for SERVER_INFO_1546 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVER_INFO_1546 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1546").field("sv1546_initsearchtable", &self.sv1546_initsearchtable).finish()
    }
}
impl ::windows::core::TypeKind for SERVER_INFO_1546 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1546 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1546_initsearchtable == other.sv1546_initsearchtable
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1546 {}
impl ::core::default::Default for SERVER_INFO_1546 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SERVER_INFO_1547 {
    pub sv1547_alertschedule: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1547 {}
impl ::core::clone::Clone for SERVER_INFO_1547 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVER_INFO_1547 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1547").field("sv1547_alertschedule", &self.sv1547_alertschedule).finish()
    }
}
impl ::windows::core::TypeKind for SERVER_INFO_1547 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1547 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1547_alertschedule == other.sv1547_alertschedule
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1547 {}
impl ::core::default::Default for SERVER_INFO_1547 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SERVER_INFO_1548 {
    pub sv1548_errorthreshold: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1548 {}
impl ::core::clone::Clone for SERVER_INFO_1548 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVER_INFO_1548 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1548").field("sv1548_errorthreshold", &self.sv1548_errorthreshold).finish()
    }
}
impl ::windows::core::TypeKind for SERVER_INFO_1548 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1548 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1548_errorthreshold == other.sv1548_errorthreshold
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1548 {}
impl ::core::default::Default for SERVER_INFO_1548 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SERVER_INFO_1549 {
    pub sv1549_networkerrorthreshold: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1549 {}
impl ::core::clone::Clone for SERVER_INFO_1549 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVER_INFO_1549 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1549").field("sv1549_networkerrorthreshold", &self.sv1549_networkerrorthreshold).finish()
    }
}
impl ::windows::core::TypeKind for SERVER_INFO_1549 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1549 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1549_networkerrorthreshold == other.sv1549_networkerrorthreshold
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1549 {}
impl ::core::default::Default for SERVER_INFO_1549 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SERVER_INFO_1550 {
    pub sv1550_diskspacethreshold: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1550 {}
impl ::core::clone::Clone for SERVER_INFO_1550 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVER_INFO_1550 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1550").field("sv1550_diskspacethreshold", &self.sv1550_diskspacethreshold).finish()
    }
}
impl ::windows::core::TypeKind for SERVER_INFO_1550 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1550 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1550_diskspacethreshold == other.sv1550_diskspacethreshold
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1550 {}
impl ::core::default::Default for SERVER_INFO_1550 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SERVER_INFO_1552 {
    pub sv1552_maxlinkdelay: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1552 {}
impl ::core::clone::Clone for SERVER_INFO_1552 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVER_INFO_1552 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1552").field("sv1552_maxlinkdelay", &self.sv1552_maxlinkdelay).finish()
    }
}
impl ::windows::core::TypeKind for SERVER_INFO_1552 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1552 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1552_maxlinkdelay == other.sv1552_maxlinkdelay
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1552 {}
impl ::core::default::Default for SERVER_INFO_1552 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SERVER_INFO_1553 {
    pub sv1553_minlinkthroughput: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1553 {}
impl ::core::clone::Clone for SERVER_INFO_1553 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVER_INFO_1553 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1553").field("sv1553_minlinkthroughput", &self.sv1553_minlinkthroughput).finish()
    }
}
impl ::windows::core::TypeKind for SERVER_INFO_1553 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1553 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1553_minlinkthroughput == other.sv1553_minlinkthroughput
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1553 {}
impl ::core::default::Default for SERVER_INFO_1553 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SERVER_INFO_1554 {
    pub sv1554_linkinfovalidtime: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1554 {}
impl ::core::clone::Clone for SERVER_INFO_1554 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVER_INFO_1554 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1554").field("sv1554_linkinfovalidtime", &self.sv1554_linkinfovalidtime).finish()
    }
}
impl ::windows::core::TypeKind for SERVER_INFO_1554 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1554 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1554_linkinfovalidtime == other.sv1554_linkinfovalidtime
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1554 {}
impl ::core::default::Default for SERVER_INFO_1554 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SERVER_INFO_1555 {
    pub sv1555_scavqosinfoupdatetime: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1555 {}
impl ::core::clone::Clone for SERVER_INFO_1555 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVER_INFO_1555 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1555").field("sv1555_scavqosinfoupdatetime", &self.sv1555_scavqosinfoupdatetime).finish()
    }
}
impl ::windows::core::TypeKind for SERVER_INFO_1555 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1555 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1555_scavqosinfoupdatetime == other.sv1555_scavqosinfoupdatetime
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1555 {}
impl ::core::default::Default for SERVER_INFO_1555 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SERVER_INFO_1556 {
    pub sv1556_maxworkitemidletime: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1556 {}
impl ::core::clone::Clone for SERVER_INFO_1556 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVER_INFO_1556 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1556").field("sv1556_maxworkitemidletime", &self.sv1556_maxworkitemidletime).finish()
    }
}
impl ::windows::core::TypeKind for SERVER_INFO_1556 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1556 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1556_maxworkitemidletime == other.sv1556_maxworkitemidletime
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1556 {}
impl ::core::default::Default for SERVER_INFO_1556 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SERVER_INFO_1557 {
    pub sv1557_maxrawworkitems: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1557 {}
impl ::core::clone::Clone for SERVER_INFO_1557 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVER_INFO_1557 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1557").field("sv1557_maxrawworkitems", &self.sv1557_maxrawworkitems).finish()
    }
}
impl ::windows::core::TypeKind for SERVER_INFO_1557 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1557 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1557_maxrawworkitems == other.sv1557_maxrawworkitems
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1557 {}
impl ::core::default::Default for SERVER_INFO_1557 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SERVER_INFO_1560 {
    pub sv1560_producttype: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1560 {}
impl ::core::clone::Clone for SERVER_INFO_1560 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVER_INFO_1560 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1560").field("sv1560_producttype", &self.sv1560_producttype).finish()
    }
}
impl ::windows::core::TypeKind for SERVER_INFO_1560 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1560 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1560_producttype == other.sv1560_producttype
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1560 {}
impl ::core::default::Default for SERVER_INFO_1560 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SERVER_INFO_1561 {
    pub sv1561_serversize: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1561 {}
impl ::core::clone::Clone for SERVER_INFO_1561 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVER_INFO_1561 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1561").field("sv1561_serversize", &self.sv1561_serversize).finish()
    }
}
impl ::windows::core::TypeKind for SERVER_INFO_1561 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1561 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1561_serversize == other.sv1561_serversize
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1561 {}
impl ::core::default::Default for SERVER_INFO_1561 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SERVER_INFO_1562 {
    pub sv1562_connectionlessautodisc: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1562 {}
impl ::core::clone::Clone for SERVER_INFO_1562 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVER_INFO_1562 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1562").field("sv1562_connectionlessautodisc", &self.sv1562_connectionlessautodisc).finish()
    }
}
impl ::windows::core::TypeKind for SERVER_INFO_1562 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1562 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1562_connectionlessautodisc == other.sv1562_connectionlessautodisc
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1562 {}
impl ::core::default::Default for SERVER_INFO_1562 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SERVER_INFO_1563 {
    pub sv1563_sharingviolationretries: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1563 {}
impl ::core::clone::Clone for SERVER_INFO_1563 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVER_INFO_1563 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1563").field("sv1563_sharingviolationretries", &self.sv1563_sharingviolationretries).finish()
    }
}
impl ::windows::core::TypeKind for SERVER_INFO_1563 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1563 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1563_sharingviolationretries == other.sv1563_sharingviolationretries
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1563 {}
impl ::core::default::Default for SERVER_INFO_1563 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SERVER_INFO_1564 {
    pub sv1564_sharingviolationdelay: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1564 {}
impl ::core::clone::Clone for SERVER_INFO_1564 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVER_INFO_1564 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1564").field("sv1564_sharingviolationdelay", &self.sv1564_sharingviolationdelay).finish()
    }
}
impl ::windows::core::TypeKind for SERVER_INFO_1564 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1564 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1564_sharingviolationdelay == other.sv1564_sharingviolationdelay
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1564 {}
impl ::core::default::Default for SERVER_INFO_1564 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SERVER_INFO_1565 {
    pub sv1565_maxglobalopensearch: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1565 {}
impl ::core::clone::Clone for SERVER_INFO_1565 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVER_INFO_1565 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1565").field("sv1565_maxglobalopensearch", &self.sv1565_maxglobalopensearch).finish()
    }
}
impl ::windows::core::TypeKind for SERVER_INFO_1565 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1565 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1565_maxglobalopensearch == other.sv1565_maxglobalopensearch
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1565 {}
impl ::core::default::Default for SERVER_INFO_1565 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SERVER_INFO_1566 {
    pub sv1566_removeduplicatesearches: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SERVER_INFO_1566 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SERVER_INFO_1566 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SERVER_INFO_1566 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1566").field("sv1566_removeduplicatesearches", &self.sv1566_removeduplicatesearches).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for SERVER_INFO_1566 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVER_INFO_1566 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1566_removeduplicatesearches == other.sv1566_removeduplicatesearches
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVER_INFO_1566 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVER_INFO_1566 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SERVER_INFO_1567 {
    pub sv1567_lockviolationretries: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1567 {}
impl ::core::clone::Clone for SERVER_INFO_1567 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVER_INFO_1567 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1567").field("sv1567_lockviolationretries", &self.sv1567_lockviolationretries).finish()
    }
}
impl ::windows::core::TypeKind for SERVER_INFO_1567 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1567 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1567_lockviolationretries == other.sv1567_lockviolationretries
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1567 {}
impl ::core::default::Default for SERVER_INFO_1567 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SERVER_INFO_1568 {
    pub sv1568_lockviolationoffset: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1568 {}
impl ::core::clone::Clone for SERVER_INFO_1568 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVER_INFO_1568 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1568").field("sv1568_lockviolationoffset", &self.sv1568_lockviolationoffset).finish()
    }
}
impl ::windows::core::TypeKind for SERVER_INFO_1568 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1568 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1568_lockviolationoffset == other.sv1568_lockviolationoffset
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1568 {}
impl ::core::default::Default for SERVER_INFO_1568 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SERVER_INFO_1569 {
    pub sv1569_lockviolationdelay: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1569 {}
impl ::core::clone::Clone for SERVER_INFO_1569 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVER_INFO_1569 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1569").field("sv1569_lockviolationdelay", &self.sv1569_lockviolationdelay).finish()
    }
}
impl ::windows::core::TypeKind for SERVER_INFO_1569 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1569 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1569_lockviolationdelay == other.sv1569_lockviolationdelay
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1569 {}
impl ::core::default::Default for SERVER_INFO_1569 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SERVER_INFO_1570 {
    pub sv1570_mdlreadswitchover: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1570 {}
impl ::core::clone::Clone for SERVER_INFO_1570 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVER_INFO_1570 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1570").field("sv1570_mdlreadswitchover", &self.sv1570_mdlreadswitchover).finish()
    }
}
impl ::windows::core::TypeKind for SERVER_INFO_1570 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1570 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1570_mdlreadswitchover == other.sv1570_mdlreadswitchover
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1570 {}
impl ::core::default::Default for SERVER_INFO_1570 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SERVER_INFO_1571 {
    pub sv1571_cachedopenlimit: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1571 {}
impl ::core::clone::Clone for SERVER_INFO_1571 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVER_INFO_1571 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1571").field("sv1571_cachedopenlimit", &self.sv1571_cachedopenlimit).finish()
    }
}
impl ::windows::core::TypeKind for SERVER_INFO_1571 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1571 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1571_cachedopenlimit == other.sv1571_cachedopenlimit
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1571 {}
impl ::core::default::Default for SERVER_INFO_1571 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SERVER_INFO_1572 {
    pub sv1572_criticalthreads: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1572 {}
impl ::core::clone::Clone for SERVER_INFO_1572 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVER_INFO_1572 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1572").field("sv1572_criticalthreads", &self.sv1572_criticalthreads).finish()
    }
}
impl ::windows::core::TypeKind for SERVER_INFO_1572 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1572 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1572_criticalthreads == other.sv1572_criticalthreads
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1572 {}
impl ::core::default::Default for SERVER_INFO_1572 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SERVER_INFO_1573 {
    pub sv1573_restrictnullsessaccess: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1573 {}
impl ::core::clone::Clone for SERVER_INFO_1573 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVER_INFO_1573 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1573").field("sv1573_restrictnullsessaccess", &self.sv1573_restrictnullsessaccess).finish()
    }
}
impl ::windows::core::TypeKind for SERVER_INFO_1573 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1573 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1573_restrictnullsessaccess == other.sv1573_restrictnullsessaccess
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1573 {}
impl ::core::default::Default for SERVER_INFO_1573 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SERVER_INFO_1574 {
    pub sv1574_enablewfw311directipx: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1574 {}
impl ::core::clone::Clone for SERVER_INFO_1574 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVER_INFO_1574 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1574").field("sv1574_enablewfw311directipx", &self.sv1574_enablewfw311directipx).finish()
    }
}
impl ::windows::core::TypeKind for SERVER_INFO_1574 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1574 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1574_enablewfw311directipx == other.sv1574_enablewfw311directipx
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1574 {}
impl ::core::default::Default for SERVER_INFO_1574 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SERVER_INFO_1575 {
    pub sv1575_otherqueueaffinity: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1575 {}
impl ::core::clone::Clone for SERVER_INFO_1575 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVER_INFO_1575 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1575").field("sv1575_otherqueueaffinity", &self.sv1575_otherqueueaffinity).finish()
    }
}
impl ::windows::core::TypeKind for SERVER_INFO_1575 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1575 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1575_otherqueueaffinity == other.sv1575_otherqueueaffinity
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1575 {}
impl ::core::default::Default for SERVER_INFO_1575 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SERVER_INFO_1576 {
    pub sv1576_queuesamplesecs: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1576 {}
impl ::core::clone::Clone for SERVER_INFO_1576 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVER_INFO_1576 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1576").field("sv1576_queuesamplesecs", &self.sv1576_queuesamplesecs).finish()
    }
}
impl ::windows::core::TypeKind for SERVER_INFO_1576 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1576 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1576_queuesamplesecs == other.sv1576_queuesamplesecs
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1576 {}
impl ::core::default::Default for SERVER_INFO_1576 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SERVER_INFO_1577 {
    pub sv1577_balancecount: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1577 {}
impl ::core::clone::Clone for SERVER_INFO_1577 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVER_INFO_1577 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1577").field("sv1577_balancecount", &self.sv1577_balancecount).finish()
    }
}
impl ::windows::core::TypeKind for SERVER_INFO_1577 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1577 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1577_balancecount == other.sv1577_balancecount
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1577 {}
impl ::core::default::Default for SERVER_INFO_1577 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SERVER_INFO_1578 {
    pub sv1578_preferredaffinity: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1578 {}
impl ::core::clone::Clone for SERVER_INFO_1578 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVER_INFO_1578 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1578").field("sv1578_preferredaffinity", &self.sv1578_preferredaffinity).finish()
    }
}
impl ::windows::core::TypeKind for SERVER_INFO_1578 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1578 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1578_preferredaffinity == other.sv1578_preferredaffinity
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1578 {}
impl ::core::default::Default for SERVER_INFO_1578 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SERVER_INFO_1579 {
    pub sv1579_maxfreerfcbs: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1579 {}
impl ::core::clone::Clone for SERVER_INFO_1579 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVER_INFO_1579 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1579").field("sv1579_maxfreerfcbs", &self.sv1579_maxfreerfcbs).finish()
    }
}
impl ::windows::core::TypeKind for SERVER_INFO_1579 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1579 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1579_maxfreerfcbs == other.sv1579_maxfreerfcbs
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1579 {}
impl ::core::default::Default for SERVER_INFO_1579 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SERVER_INFO_1580 {
    pub sv1580_maxfreemfcbs: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1580 {}
impl ::core::clone::Clone for SERVER_INFO_1580 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVER_INFO_1580 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1580").field("sv1580_maxfreemfcbs", &self.sv1580_maxfreemfcbs).finish()
    }
}
impl ::windows::core::TypeKind for SERVER_INFO_1580 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1580 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1580_maxfreemfcbs == other.sv1580_maxfreemfcbs
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1580 {}
impl ::core::default::Default for SERVER_INFO_1580 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SERVER_INFO_1581 {
    pub sv1581_maxfreemlcbs: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1581 {}
impl ::core::clone::Clone for SERVER_INFO_1581 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVER_INFO_1581 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1581").field("sv1581_maxfreemlcbs", &self.sv1581_maxfreemlcbs).finish()
    }
}
impl ::windows::core::TypeKind for SERVER_INFO_1581 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1581 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1581_maxfreemlcbs == other.sv1581_maxfreemlcbs
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1581 {}
impl ::core::default::Default for SERVER_INFO_1581 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SERVER_INFO_1582 {
    pub sv1582_maxfreepagedpoolchunks: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1582 {}
impl ::core::clone::Clone for SERVER_INFO_1582 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVER_INFO_1582 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1582").field("sv1582_maxfreepagedpoolchunks", &self.sv1582_maxfreepagedpoolchunks).finish()
    }
}
impl ::windows::core::TypeKind for SERVER_INFO_1582 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1582 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1582_maxfreepagedpoolchunks == other.sv1582_maxfreepagedpoolchunks
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1582 {}
impl ::core::default::Default for SERVER_INFO_1582 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SERVER_INFO_1583 {
    pub sv1583_minpagedpoolchunksize: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1583 {}
impl ::core::clone::Clone for SERVER_INFO_1583 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVER_INFO_1583 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1583").field("sv1583_minpagedpoolchunksize", &self.sv1583_minpagedpoolchunksize).finish()
    }
}
impl ::windows::core::TypeKind for SERVER_INFO_1583 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1583 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1583_minpagedpoolchunksize == other.sv1583_minpagedpoolchunksize
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1583 {}
impl ::core::default::Default for SERVER_INFO_1583 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SERVER_INFO_1584 {
    pub sv1584_maxpagedpoolchunksize: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1584 {}
impl ::core::clone::Clone for SERVER_INFO_1584 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVER_INFO_1584 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1584").field("sv1584_maxpagedpoolchunksize", &self.sv1584_maxpagedpoolchunksize).finish()
    }
}
impl ::windows::core::TypeKind for SERVER_INFO_1584 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1584 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1584_maxpagedpoolchunksize == other.sv1584_maxpagedpoolchunksize
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1584 {}
impl ::core::default::Default for SERVER_INFO_1584 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SERVER_INFO_1585 {
    pub sv1585_sendsfrompreferredprocessor: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SERVER_INFO_1585 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SERVER_INFO_1585 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SERVER_INFO_1585 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1585").field("sv1585_sendsfrompreferredprocessor", &self.sv1585_sendsfrompreferredprocessor).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for SERVER_INFO_1585 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVER_INFO_1585 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1585_sendsfrompreferredprocessor == other.sv1585_sendsfrompreferredprocessor
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVER_INFO_1585 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVER_INFO_1585 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SERVER_INFO_1586 {
    pub sv1586_maxthreadsperqueue: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1586 {}
impl ::core::clone::Clone for SERVER_INFO_1586 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVER_INFO_1586 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1586").field("sv1586_maxthreadsperqueue", &self.sv1586_maxthreadsperqueue).finish()
    }
}
impl ::windows::core::TypeKind for SERVER_INFO_1586 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1586 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1586_maxthreadsperqueue == other.sv1586_maxthreadsperqueue
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1586 {}
impl ::core::default::Default for SERVER_INFO_1586 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SERVER_INFO_1587 {
    pub sv1587_cacheddirectorylimit: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1587 {}
impl ::core::clone::Clone for SERVER_INFO_1587 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVER_INFO_1587 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1587").field("sv1587_cacheddirectorylimit", &self.sv1587_cacheddirectorylimit).finish()
    }
}
impl ::windows::core::TypeKind for SERVER_INFO_1587 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1587 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1587_cacheddirectorylimit == other.sv1587_cacheddirectorylimit
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1587 {}
impl ::core::default::Default for SERVER_INFO_1587 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SERVER_INFO_1588 {
    pub sv1588_maxcopylength: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1588 {}
impl ::core::clone::Clone for SERVER_INFO_1588 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVER_INFO_1588 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1588").field("sv1588_maxcopylength", &self.sv1588_maxcopylength).finish()
    }
}
impl ::windows::core::TypeKind for SERVER_INFO_1588 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1588 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1588_maxcopylength == other.sv1588_maxcopylength
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1588 {}
impl ::core::default::Default for SERVER_INFO_1588 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SERVER_INFO_1590 {
    pub sv1590_enablecompression: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1590 {}
impl ::core::clone::Clone for SERVER_INFO_1590 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVER_INFO_1590 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1590").field("sv1590_enablecompression", &self.sv1590_enablecompression).finish()
    }
}
impl ::windows::core::TypeKind for SERVER_INFO_1590 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1590 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1590_enablecompression == other.sv1590_enablecompression
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1590 {}
impl ::core::default::Default for SERVER_INFO_1590 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SERVER_INFO_1591 {
    pub sv1591_autosharewks: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1591 {}
impl ::core::clone::Clone for SERVER_INFO_1591 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVER_INFO_1591 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1591").field("sv1591_autosharewks", &self.sv1591_autosharewks).finish()
    }
}
impl ::windows::core::TypeKind for SERVER_INFO_1591 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1591 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1591_autosharewks == other.sv1591_autosharewks
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1591 {}
impl ::core::default::Default for SERVER_INFO_1591 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SERVER_INFO_1592 {
    pub sv1592_autosharewks: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1592 {}
impl ::core::clone::Clone for SERVER_INFO_1592 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVER_INFO_1592 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1592").field("sv1592_autosharewks", &self.sv1592_autosharewks).finish()
    }
}
impl ::windows::core::TypeKind for SERVER_INFO_1592 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1592 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1592_autosharewks == other.sv1592_autosharewks
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1592 {}
impl ::core::default::Default for SERVER_INFO_1592 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SERVER_INFO_1593 {
    pub sv1593_enablesecuritysignature: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1593 {}
impl ::core::clone::Clone for SERVER_INFO_1593 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVER_INFO_1593 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1593").field("sv1593_enablesecuritysignature", &self.sv1593_enablesecuritysignature).finish()
    }
}
impl ::windows::core::TypeKind for SERVER_INFO_1593 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1593 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1593_enablesecuritysignature == other.sv1593_enablesecuritysignature
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1593 {}
impl ::core::default::Default for SERVER_INFO_1593 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SERVER_INFO_1594 {
    pub sv1594_requiresecuritysignature: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1594 {}
impl ::core::clone::Clone for SERVER_INFO_1594 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVER_INFO_1594 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1594").field("sv1594_requiresecuritysignature", &self.sv1594_requiresecuritysignature).finish()
    }
}
impl ::windows::core::TypeKind for SERVER_INFO_1594 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1594 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1594_requiresecuritysignature == other.sv1594_requiresecuritysignature
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1594 {}
impl ::core::default::Default for SERVER_INFO_1594 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SERVER_INFO_1595 {
    pub sv1595_minclientbuffersize: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1595 {}
impl ::core::clone::Clone for SERVER_INFO_1595 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVER_INFO_1595 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1595").field("sv1595_minclientbuffersize", &self.sv1595_minclientbuffersize).finish()
    }
}
impl ::windows::core::TypeKind for SERVER_INFO_1595 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1595 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1595_minclientbuffersize == other.sv1595_minclientbuffersize
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1595 {}
impl ::core::default::Default for SERVER_INFO_1595 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SERVER_INFO_1596 {
    pub sv1596_ConnectionNoSessionsTimeout: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1596 {}
impl ::core::clone::Clone for SERVER_INFO_1596 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVER_INFO_1596 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1596").field("sv1596_ConnectionNoSessionsTimeout", &self.sv1596_ConnectionNoSessionsTimeout).finish()
    }
}
impl ::windows::core::TypeKind for SERVER_INFO_1596 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1596 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1596_ConnectionNoSessionsTimeout == other.sv1596_ConnectionNoSessionsTimeout
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1596 {}
impl ::core::default::Default for SERVER_INFO_1596 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SERVER_INFO_1597 {
    pub sv1597_IdleThreadTimeOut: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1597 {}
impl ::core::clone::Clone for SERVER_INFO_1597 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVER_INFO_1597 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1597").field("sv1597_IdleThreadTimeOut", &self.sv1597_IdleThreadTimeOut).finish()
    }
}
impl ::windows::core::TypeKind for SERVER_INFO_1597 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1597 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1597_IdleThreadTimeOut == other.sv1597_IdleThreadTimeOut
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1597 {}
impl ::core::default::Default for SERVER_INFO_1597 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SERVER_INFO_1598 {
    pub sv1598_enableW9xsecuritysignature: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1598 {}
impl ::core::clone::Clone for SERVER_INFO_1598 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVER_INFO_1598 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1598").field("sv1598_enableW9xsecuritysignature", &self.sv1598_enableW9xsecuritysignature).finish()
    }
}
impl ::windows::core::TypeKind for SERVER_INFO_1598 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1598 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1598_enableW9xsecuritysignature == other.sv1598_enableW9xsecuritysignature
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1598 {}
impl ::core::default::Default for SERVER_INFO_1598 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SERVER_INFO_1599 {
    pub sv1598_enforcekerberosreauthentication: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SERVER_INFO_1599 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SERVER_INFO_1599 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SERVER_INFO_1599 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1599").field("sv1598_enforcekerberosreauthentication", &self.sv1598_enforcekerberosreauthentication).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for SERVER_INFO_1599 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVER_INFO_1599 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1598_enforcekerberosreauthentication == other.sv1598_enforcekerberosreauthentication
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVER_INFO_1599 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVER_INFO_1599 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SERVER_INFO_1600 {
    pub sv1598_disabledos: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SERVER_INFO_1600 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SERVER_INFO_1600 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SERVER_INFO_1600 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1600").field("sv1598_disabledos", &self.sv1598_disabledos).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for SERVER_INFO_1600 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVER_INFO_1600 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1598_disabledos == other.sv1598_disabledos
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVER_INFO_1600 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVER_INFO_1600 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SERVER_INFO_1601 {
    pub sv1598_lowdiskspaceminimum: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1601 {}
impl ::core::clone::Clone for SERVER_INFO_1601 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVER_INFO_1601 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1601").field("sv1598_lowdiskspaceminimum", &self.sv1598_lowdiskspaceminimum).finish()
    }
}
impl ::windows::core::TypeKind for SERVER_INFO_1601 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1601 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1598_lowdiskspaceminimum == other.sv1598_lowdiskspaceminimum
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1601 {}
impl ::core::default::Default for SERVER_INFO_1601 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SERVER_INFO_1602 {
    pub sv_1598_disablestrictnamechecking: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SERVER_INFO_1602 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SERVER_INFO_1602 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SERVER_INFO_1602 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1602").field("sv_1598_disablestrictnamechecking", &self.sv_1598_disablestrictnamechecking).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for SERVER_INFO_1602 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVER_INFO_1602 {
    fn eq(&self, other: &Self) -> bool {
        self.sv_1598_disablestrictnamechecking == other.sv_1598_disablestrictnamechecking
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVER_INFO_1602 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVER_INFO_1602 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SERVER_INFO_402 {
    pub sv402_ulist_mtime: u32,
    pub sv402_glist_mtime: u32,
    pub sv402_alist_mtime: u32,
    pub sv402_alerts: ::windows::core::PWSTR,
    pub sv402_security: SERVER_INFO_SECURITY,
    pub sv402_numadmin: u32,
    pub sv402_lanmask: u32,
    pub sv402_guestacct: ::windows::core::PWSTR,
    pub sv402_chdevs: u32,
    pub sv402_chdevq: u32,
    pub sv402_chdevjobs: u32,
    pub sv402_connections: u32,
    pub sv402_shares: u32,
    pub sv402_openfiles: u32,
    pub sv402_sessopens: u32,
    pub sv402_sessvcs: u32,
    pub sv402_sessreqs: u32,
    pub sv402_opensearch: u32,
    pub sv402_activelocks: u32,
    pub sv402_numreqbuf: u32,
    pub sv402_sizreqbuf: u32,
    pub sv402_numbigbuf: u32,
    pub sv402_numfiletasks: u32,
    pub sv402_alertsched: u32,
    pub sv402_erroralert: u32,
    pub sv402_logonalert: u32,
    pub sv402_accessalert: u32,
    pub sv402_diskalert: u32,
    pub sv402_netioalert: u32,
    pub sv402_maxauditsz: u32,
    pub sv402_srvheuristics: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for SERVER_INFO_402 {}
impl ::core::clone::Clone for SERVER_INFO_402 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVER_INFO_402 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_402")
            .field("sv402_ulist_mtime", &self.sv402_ulist_mtime)
            .field("sv402_glist_mtime", &self.sv402_glist_mtime)
            .field("sv402_alist_mtime", &self.sv402_alist_mtime)
            .field("sv402_alerts", &self.sv402_alerts)
            .field("sv402_security", &self.sv402_security)
            .field("sv402_numadmin", &self.sv402_numadmin)
            .field("sv402_lanmask", &self.sv402_lanmask)
            .field("sv402_guestacct", &self.sv402_guestacct)
            .field("sv402_chdevs", &self.sv402_chdevs)
            .field("sv402_chdevq", &self.sv402_chdevq)
            .field("sv402_chdevjobs", &self.sv402_chdevjobs)
            .field("sv402_connections", &self.sv402_connections)
            .field("sv402_shares", &self.sv402_shares)
            .field("sv402_openfiles", &self.sv402_openfiles)
            .field("sv402_sessopens", &self.sv402_sessopens)
            .field("sv402_sessvcs", &self.sv402_sessvcs)
            .field("sv402_sessreqs", &self.sv402_sessreqs)
            .field("sv402_opensearch", &self.sv402_opensearch)
            .field("sv402_activelocks", &self.sv402_activelocks)
            .field("sv402_numreqbuf", &self.sv402_numreqbuf)
            .field("sv402_sizreqbuf", &self.sv402_sizreqbuf)
            .field("sv402_numbigbuf", &self.sv402_numbigbuf)
            .field("sv402_numfiletasks", &self.sv402_numfiletasks)
            .field("sv402_alertsched", &self.sv402_alertsched)
            .field("sv402_erroralert", &self.sv402_erroralert)
            .field("sv402_logonalert", &self.sv402_logonalert)
            .field("sv402_accessalert", &self.sv402_accessalert)
            .field("sv402_diskalert", &self.sv402_diskalert)
            .field("sv402_netioalert", &self.sv402_netioalert)
            .field("sv402_maxauditsz", &self.sv402_maxauditsz)
            .field("sv402_srvheuristics", &self.sv402_srvheuristics)
            .finish()
    }
}
impl ::windows::core::TypeKind for SERVER_INFO_402 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVER_INFO_402 {
    fn eq(&self, other: &Self) -> bool {
        self.sv402_ulist_mtime == other.sv402_ulist_mtime
            && self.sv402_glist_mtime == other.sv402_glist_mtime
            && self.sv402_alist_mtime == other.sv402_alist_mtime
            && self.sv402_alerts == other.sv402_alerts
            && self.sv402_security == other.sv402_security
            && self.sv402_numadmin == other.sv402_numadmin
            && self.sv402_lanmask == other.sv402_lanmask
            && self.sv402_guestacct == other.sv402_guestacct
            && self.sv402_chdevs == other.sv402_chdevs
            && self.sv402_chdevq == other.sv402_chdevq
            && self.sv402_chdevjobs == other.sv402_chdevjobs
            && self.sv402_connections == other.sv402_connections
            && self.sv402_shares == other.sv402_shares
            && self.sv402_openfiles == other.sv402_openfiles
            && self.sv402_sessopens == other.sv402_sessopens
            && self.sv402_sessvcs == other.sv402_sessvcs
            && self.sv402_sessreqs == other.sv402_sessreqs
            && self.sv402_opensearch == other.sv402_opensearch
            && self.sv402_activelocks == other.sv402_activelocks
            && self.sv402_numreqbuf == other.sv402_numreqbuf
            && self.sv402_sizreqbuf == other.sv402_sizreqbuf
            && self.sv402_numbigbuf == other.sv402_numbigbuf
            && self.sv402_numfiletasks == other.sv402_numfiletasks
            && self.sv402_alertsched == other.sv402_alertsched
            && self.sv402_erroralert == other.sv402_erroralert
            && self.sv402_logonalert == other.sv402_logonalert
            && self.sv402_accessalert == other.sv402_accessalert
            && self.sv402_diskalert == other.sv402_diskalert
            && self.sv402_netioalert == other.sv402_netioalert
            && self.sv402_maxauditsz == other.sv402_maxauditsz
            && self.sv402_srvheuristics == other.sv402_srvheuristics
    }
}
impl ::core::cmp::Eq for SERVER_INFO_402 {}
impl ::core::default::Default for SERVER_INFO_402 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SERVER_INFO_403 {
    pub sv403_ulist_mtime: u32,
    pub sv403_glist_mtime: u32,
    pub sv403_alist_mtime: u32,
    pub sv403_alerts: ::windows::core::PWSTR,
    pub sv403_security: SERVER_INFO_SECURITY,
    pub sv403_numadmin: u32,
    pub sv403_lanmask: u32,
    pub sv403_guestacct: ::windows::core::PWSTR,
    pub sv403_chdevs: u32,
    pub sv403_chdevq: u32,
    pub sv403_chdevjobs: u32,
    pub sv403_connections: u32,
    pub sv403_shares: u32,
    pub sv403_openfiles: u32,
    pub sv403_sessopens: u32,
    pub sv403_sessvcs: u32,
    pub sv403_sessreqs: u32,
    pub sv403_opensearch: u32,
    pub sv403_activelocks: u32,
    pub sv403_numreqbuf: u32,
    pub sv403_sizreqbuf: u32,
    pub sv403_numbigbuf: u32,
    pub sv403_numfiletasks: u32,
    pub sv403_alertsched: u32,
    pub sv403_erroralert: u32,
    pub sv403_logonalert: u32,
    pub sv403_accessalert: u32,
    pub sv403_diskalert: u32,
    pub sv403_netioalert: u32,
    pub sv403_maxauditsz: u32,
    pub sv403_srvheuristics: ::windows::core::PWSTR,
    pub sv403_auditedevents: u32,
    pub sv403_autoprofile: u32,
    pub sv403_autopath: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for SERVER_INFO_403 {}
impl ::core::clone::Clone for SERVER_INFO_403 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVER_INFO_403 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_403")
            .field("sv403_ulist_mtime", &self.sv403_ulist_mtime)
            .field("sv403_glist_mtime", &self.sv403_glist_mtime)
            .field("sv403_alist_mtime", &self.sv403_alist_mtime)
            .field("sv403_alerts", &self.sv403_alerts)
            .field("sv403_security", &self.sv403_security)
            .field("sv403_numadmin", &self.sv403_numadmin)
            .field("sv403_lanmask", &self.sv403_lanmask)
            .field("sv403_guestacct", &self.sv403_guestacct)
            .field("sv403_chdevs", &self.sv403_chdevs)
            .field("sv403_chdevq", &self.sv403_chdevq)
            .field("sv403_chdevjobs", &self.sv403_chdevjobs)
            .field("sv403_connections", &self.sv403_connections)
            .field("sv403_shares", &self.sv403_shares)
            .field("sv403_openfiles", &self.sv403_openfiles)
            .field("sv403_sessopens", &self.sv403_sessopens)
            .field("sv403_sessvcs", &self.sv403_sessvcs)
            .field("sv403_sessreqs", &self.sv403_sessreqs)
            .field("sv403_opensearch", &self.sv403_opensearch)
            .field("sv403_activelocks", &self.sv403_activelocks)
            .field("sv403_numreqbuf", &self.sv403_numreqbuf)
            .field("sv403_sizreqbuf", &self.sv403_sizreqbuf)
            .field("sv403_numbigbuf", &self.sv403_numbigbuf)
            .field("sv403_numfiletasks", &self.sv403_numfiletasks)
            .field("sv403_alertsched", &self.sv403_alertsched)
            .field("sv403_erroralert", &self.sv403_erroralert)
            .field("sv403_logonalert", &self.sv403_logonalert)
            .field("sv403_accessalert", &self.sv403_accessalert)
            .field("sv403_diskalert", &self.sv403_diskalert)
            .field("sv403_netioalert", &self.sv403_netioalert)
            .field("sv403_maxauditsz", &self.sv403_maxauditsz)
            .field("sv403_srvheuristics", &self.sv403_srvheuristics)
            .field("sv403_auditedevents", &self.sv403_auditedevents)
            .field("sv403_autoprofile", &self.sv403_autoprofile)
            .field("sv403_autopath", &self.sv403_autopath)
            .finish()
    }
}
impl ::windows::core::TypeKind for SERVER_INFO_403 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVER_INFO_403 {
    fn eq(&self, other: &Self) -> bool {
        self.sv403_ulist_mtime == other.sv403_ulist_mtime
            && self.sv403_glist_mtime == other.sv403_glist_mtime
            && self.sv403_alist_mtime == other.sv403_alist_mtime
            && self.sv403_alerts == other.sv403_alerts
            && self.sv403_security == other.sv403_security
            && self.sv403_numadmin == other.sv403_numadmin
            && self.sv403_lanmask == other.sv403_lanmask
            && self.sv403_guestacct == other.sv403_guestacct
            && self.sv403_chdevs == other.sv403_chdevs
            && self.sv403_chdevq == other.sv403_chdevq
            && self.sv403_chdevjobs == other.sv403_chdevjobs
            && self.sv403_connections == other.sv403_connections
            && self.sv403_shares == other.sv403_shares
            && self.sv403_openfiles == other.sv403_openfiles
            && self.sv403_sessopens == other.sv403_sessopens
            && self.sv403_sessvcs == other.sv403_sessvcs
            && self.sv403_sessreqs == other.sv403_sessreqs
            && self.sv403_opensearch == other.sv403_opensearch
            && self.sv403_activelocks == other.sv403_activelocks
            && self.sv403_numreqbuf == other.sv403_numreqbuf
            && self.sv403_sizreqbuf == other.sv403_sizreqbuf
            && self.sv403_numbigbuf == other.sv403_numbigbuf
            && self.sv403_numfiletasks == other.sv403_numfiletasks
            && self.sv403_alertsched == other.sv403_alertsched
            && self.sv403_erroralert == other.sv403_erroralert
            && self.sv403_logonalert == other.sv403_logonalert
            && self.sv403_accessalert == other.sv403_accessalert
            && self.sv403_diskalert == other.sv403_diskalert
            && self.sv403_netioalert == other.sv403_netioalert
            && self.sv403_maxauditsz == other.sv403_maxauditsz
            && self.sv403_srvheuristics == other.sv403_srvheuristics
            && self.sv403_auditedevents == other.sv403_auditedevents
            && self.sv403_autoprofile == other.sv403_autoprofile
            && self.sv403_autopath == other.sv403_autopath
    }
}
impl ::core::cmp::Eq for SERVER_INFO_403 {}
impl ::core::default::Default for SERVER_INFO_403 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SERVER_INFO_502 {
    pub sv502_sessopens: u32,
    pub sv502_sessvcs: u32,
    pub sv502_opensearch: u32,
    pub sv502_sizreqbuf: u32,
    pub sv502_initworkitems: u32,
    pub sv502_maxworkitems: u32,
    pub sv502_rawworkitems: u32,
    pub sv502_irpstacksize: u32,
    pub sv502_maxrawbuflen: u32,
    pub sv502_sessusers: u32,
    pub sv502_sessconns: u32,
    pub sv502_maxpagedmemoryusage: u32,
    pub sv502_maxnonpagedmemoryusage: u32,
    pub sv502_enablesoftcompat: super::super::Foundation::BOOL,
    pub sv502_enableforcedlogoff: super::super::Foundation::BOOL,
    pub sv502_timesource: super::super::Foundation::BOOL,
    pub sv502_acceptdownlevelapis: super::super::Foundation::BOOL,
    pub sv502_lmannounce: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SERVER_INFO_502 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SERVER_INFO_502 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SERVER_INFO_502 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_502")
            .field("sv502_sessopens", &self.sv502_sessopens)
            .field("sv502_sessvcs", &self.sv502_sessvcs)
            .field("sv502_opensearch", &self.sv502_opensearch)
            .field("sv502_sizreqbuf", &self.sv502_sizreqbuf)
            .field("sv502_initworkitems", &self.sv502_initworkitems)
            .field("sv502_maxworkitems", &self.sv502_maxworkitems)
            .field("sv502_rawworkitems", &self.sv502_rawworkitems)
            .field("sv502_irpstacksize", &self.sv502_irpstacksize)
            .field("sv502_maxrawbuflen", &self.sv502_maxrawbuflen)
            .field("sv502_sessusers", &self.sv502_sessusers)
            .field("sv502_sessconns", &self.sv502_sessconns)
            .field("sv502_maxpagedmemoryusage", &self.sv502_maxpagedmemoryusage)
            .field("sv502_maxnonpagedmemoryusage", &self.sv502_maxnonpagedmemoryusage)
            .field("sv502_enablesoftcompat", &self.sv502_enablesoftcompat)
            .field("sv502_enableforcedlogoff", &self.sv502_enableforcedlogoff)
            .field("sv502_timesource", &self.sv502_timesource)
            .field("sv502_acceptdownlevelapis", &self.sv502_acceptdownlevelapis)
            .field("sv502_lmannounce", &self.sv502_lmannounce)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for SERVER_INFO_502 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVER_INFO_502 {
    fn eq(&self, other: &Self) -> bool {
        self.sv502_sessopens == other.sv502_sessopens
            && self.sv502_sessvcs == other.sv502_sessvcs
            && self.sv502_opensearch == other.sv502_opensearch
            && self.sv502_sizreqbuf == other.sv502_sizreqbuf
            && self.sv502_initworkitems == other.sv502_initworkitems
            && self.sv502_maxworkitems == other.sv502_maxworkitems
            && self.sv502_rawworkitems == other.sv502_rawworkitems
            && self.sv502_irpstacksize == other.sv502_irpstacksize
            && self.sv502_maxrawbuflen == other.sv502_maxrawbuflen
            && self.sv502_sessusers == other.sv502_sessusers
            && self.sv502_sessconns == other.sv502_sessconns
            && self.sv502_maxpagedmemoryusage == other.sv502_maxpagedmemoryusage
            && self.sv502_maxnonpagedmemoryusage == other.sv502_maxnonpagedmemoryusage
            && self.sv502_enablesoftcompat == other.sv502_enablesoftcompat
            && self.sv502_enableforcedlogoff == other.sv502_enableforcedlogoff
            && self.sv502_timesource == other.sv502_timesource
            && self.sv502_acceptdownlevelapis == other.sv502_acceptdownlevelapis
            && self.sv502_lmannounce == other.sv502_lmannounce
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVER_INFO_502 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVER_INFO_502 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SERVER_INFO_503 {
    pub sv503_sessopens: u32,
    pub sv503_sessvcs: u32,
    pub sv503_opensearch: u32,
    pub sv503_sizreqbuf: u32,
    pub sv503_initworkitems: u32,
    pub sv503_maxworkitems: u32,
    pub sv503_rawworkitems: u32,
    pub sv503_irpstacksize: u32,
    pub sv503_maxrawbuflen: u32,
    pub sv503_sessusers: u32,
    pub sv503_sessconns: u32,
    pub sv503_maxpagedmemoryusage: u32,
    pub sv503_maxnonpagedmemoryusage: u32,
    pub sv503_enablesoftcompat: super::super::Foundation::BOOL,
    pub sv503_enableforcedlogoff: super::super::Foundation::BOOL,
    pub sv503_timesource: super::super::Foundation::BOOL,
    pub sv503_acceptdownlevelapis: super::super::Foundation::BOOL,
    pub sv503_lmannounce: super::super::Foundation::BOOL,
    pub sv503_domain: ::windows::core::PWSTR,
    pub sv503_maxcopyreadlen: u32,
    pub sv503_maxcopywritelen: u32,
    pub sv503_minkeepsearch: u32,
    pub sv503_maxkeepsearch: u32,
    pub sv503_minkeepcomplsearch: u32,
    pub sv503_maxkeepcomplsearch: u32,
    pub sv503_threadcountadd: u32,
    pub sv503_numblockthreads: u32,
    pub sv503_scavtimeout: u32,
    pub sv503_minrcvqueue: u32,
    pub sv503_minfreeworkitems: u32,
    pub sv503_xactmemsize: u32,
    pub sv503_threadpriority: u32,
    pub sv503_maxmpxct: u32,
    pub sv503_oplockbreakwait: u32,
    pub sv503_oplockbreakresponsewait: u32,
    pub sv503_enableoplocks: super::super::Foundation::BOOL,
    pub sv503_enableoplockforceclose: super::super::Foundation::BOOL,
    pub sv503_enablefcbopens: super::super::Foundation::BOOL,
    pub sv503_enableraw: super::super::Foundation::BOOL,
    pub sv503_enablesharednetdrives: super::super::Foundation::BOOL,
    pub sv503_minfreeconnections: u32,
    pub sv503_maxfreeconnections: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SERVER_INFO_503 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SERVER_INFO_503 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SERVER_INFO_503 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_503")
            .field("sv503_sessopens", &self.sv503_sessopens)
            .field("sv503_sessvcs", &self.sv503_sessvcs)
            .field("sv503_opensearch", &self.sv503_opensearch)
            .field("sv503_sizreqbuf", &self.sv503_sizreqbuf)
            .field("sv503_initworkitems", &self.sv503_initworkitems)
            .field("sv503_maxworkitems", &self.sv503_maxworkitems)
            .field("sv503_rawworkitems", &self.sv503_rawworkitems)
            .field("sv503_irpstacksize", &self.sv503_irpstacksize)
            .field("sv503_maxrawbuflen", &self.sv503_maxrawbuflen)
            .field("sv503_sessusers", &self.sv503_sessusers)
            .field("sv503_sessconns", &self.sv503_sessconns)
            .field("sv503_maxpagedmemoryusage", &self.sv503_maxpagedmemoryusage)
            .field("sv503_maxnonpagedmemoryusage", &self.sv503_maxnonpagedmemoryusage)
            .field("sv503_enablesoftcompat", &self.sv503_enablesoftcompat)
            .field("sv503_enableforcedlogoff", &self.sv503_enableforcedlogoff)
            .field("sv503_timesource", &self.sv503_timesource)
            .field("sv503_acceptdownlevelapis", &self.sv503_acceptdownlevelapis)
            .field("sv503_lmannounce", &self.sv503_lmannounce)
            .field("sv503_domain", &self.sv503_domain)
            .field("sv503_maxcopyreadlen", &self.sv503_maxcopyreadlen)
            .field("sv503_maxcopywritelen", &self.sv503_maxcopywritelen)
            .field("sv503_minkeepsearch", &self.sv503_minkeepsearch)
            .field("sv503_maxkeepsearch", &self.sv503_maxkeepsearch)
            .field("sv503_minkeepcomplsearch", &self.sv503_minkeepcomplsearch)
            .field("sv503_maxkeepcomplsearch", &self.sv503_maxkeepcomplsearch)
            .field("sv503_threadcountadd", &self.sv503_threadcountadd)
            .field("sv503_numblockthreads", &self.sv503_numblockthreads)
            .field("sv503_scavtimeout", &self.sv503_scavtimeout)
            .field("sv503_minrcvqueue", &self.sv503_minrcvqueue)
            .field("sv503_minfreeworkitems", &self.sv503_minfreeworkitems)
            .field("sv503_xactmemsize", &self.sv503_xactmemsize)
            .field("sv503_threadpriority", &self.sv503_threadpriority)
            .field("sv503_maxmpxct", &self.sv503_maxmpxct)
            .field("sv503_oplockbreakwait", &self.sv503_oplockbreakwait)
            .field("sv503_oplockbreakresponsewait", &self.sv503_oplockbreakresponsewait)
            .field("sv503_enableoplocks", &self.sv503_enableoplocks)
            .field("sv503_enableoplockforceclose", &self.sv503_enableoplockforceclose)
            .field("sv503_enablefcbopens", &self.sv503_enablefcbopens)
            .field("sv503_enableraw", &self.sv503_enableraw)
            .field("sv503_enablesharednetdrives", &self.sv503_enablesharednetdrives)
            .field("sv503_minfreeconnections", &self.sv503_minfreeconnections)
            .field("sv503_maxfreeconnections", &self.sv503_maxfreeconnections)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for SERVER_INFO_503 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVER_INFO_503 {
    fn eq(&self, other: &Self) -> bool {
        self.sv503_sessopens == other.sv503_sessopens
            && self.sv503_sessvcs == other.sv503_sessvcs
            && self.sv503_opensearch == other.sv503_opensearch
            && self.sv503_sizreqbuf == other.sv503_sizreqbuf
            && self.sv503_initworkitems == other.sv503_initworkitems
            && self.sv503_maxworkitems == other.sv503_maxworkitems
            && self.sv503_rawworkitems == other.sv503_rawworkitems
            && self.sv503_irpstacksize == other.sv503_irpstacksize
            && self.sv503_maxrawbuflen == other.sv503_maxrawbuflen
            && self.sv503_sessusers == other.sv503_sessusers
            && self.sv503_sessconns == other.sv503_sessconns
            && self.sv503_maxpagedmemoryusage == other.sv503_maxpagedmemoryusage
            && self.sv503_maxnonpagedmemoryusage == other.sv503_maxnonpagedmemoryusage
            && self.sv503_enablesoftcompat == other.sv503_enablesoftcompat
            && self.sv503_enableforcedlogoff == other.sv503_enableforcedlogoff
            && self.sv503_timesource == other.sv503_timesource
            && self.sv503_acceptdownlevelapis == other.sv503_acceptdownlevelapis
            && self.sv503_lmannounce == other.sv503_lmannounce
            && self.sv503_domain == other.sv503_domain
            && self.sv503_maxcopyreadlen == other.sv503_maxcopyreadlen
            && self.sv503_maxcopywritelen == other.sv503_maxcopywritelen
            && self.sv503_minkeepsearch == other.sv503_minkeepsearch
            && self.sv503_maxkeepsearch == other.sv503_maxkeepsearch
            && self.sv503_minkeepcomplsearch == other.sv503_minkeepcomplsearch
            && self.sv503_maxkeepcomplsearch == other.sv503_maxkeepcomplsearch
            && self.sv503_threadcountadd == other.sv503_threadcountadd
            && self.sv503_numblockthreads == other.sv503_numblockthreads
            && self.sv503_scavtimeout == other.sv503_scavtimeout
            && self.sv503_minrcvqueue == other.sv503_minrcvqueue
            && self.sv503_minfreeworkitems == other.sv503_minfreeworkitems
            && self.sv503_xactmemsize == other.sv503_xactmemsize
            && self.sv503_threadpriority == other.sv503_threadpriority
            && self.sv503_maxmpxct == other.sv503_maxmpxct
            && self.sv503_oplockbreakwait == other.sv503_oplockbreakwait
            && self.sv503_oplockbreakresponsewait == other.sv503_oplockbreakresponsewait
            && self.sv503_enableoplocks == other.sv503_enableoplocks
            && self.sv503_enableoplockforceclose == other.sv503_enableoplockforceclose
            && self.sv503_enablefcbopens == other.sv503_enablefcbopens
            && self.sv503_enableraw == other.sv503_enableraw
            && self.sv503_enablesharednetdrives == other.sv503_enablesharednetdrives
            && self.sv503_minfreeconnections == other.sv503_minfreeconnections
            && self.sv503_maxfreeconnections == other.sv503_maxfreeconnections
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVER_INFO_503 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVER_INFO_503 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SERVER_INFO_598 {
    pub sv598_maxrawworkitems: u32,
    pub sv598_maxthreadsperqueue: u32,
    pub sv598_producttype: u32,
    pub sv598_serversize: u32,
    pub sv598_connectionlessautodisc: u32,
    pub sv598_sharingviolationretries: u32,
    pub sv598_sharingviolationdelay: u32,
    pub sv598_maxglobalopensearch: u32,
    pub sv598_removeduplicatesearches: u32,
    pub sv598_lockviolationoffset: u32,
    pub sv598_lockviolationdelay: u32,
    pub sv598_mdlreadswitchover: u32,
    pub sv598_cachedopenlimit: u32,
    pub sv598_otherqueueaffinity: u32,
    pub sv598_restrictnullsessaccess: super::super::Foundation::BOOL,
    pub sv598_enablewfw311directipx: super::super::Foundation::BOOL,
    pub sv598_queuesamplesecs: u32,
    pub sv598_balancecount: u32,
    pub sv598_preferredaffinity: u32,
    pub sv598_maxfreerfcbs: u32,
    pub sv598_maxfreemfcbs: u32,
    pub sv598_maxfreelfcbs: u32,
    pub sv598_maxfreepagedpoolchunks: u32,
    pub sv598_minpagedpoolchunksize: u32,
    pub sv598_maxpagedpoolchunksize: u32,
    pub sv598_sendsfrompreferredprocessor: super::super::Foundation::BOOL,
    pub sv598_cacheddirectorylimit: u32,
    pub sv598_maxcopylength: u32,
    pub sv598_enablecompression: super::super::Foundation::BOOL,
    pub sv598_autosharewks: super::super::Foundation::BOOL,
    pub sv598_autoshareserver: super::super::Foundation::BOOL,
    pub sv598_enablesecuritysignature: super::super::Foundation::BOOL,
    pub sv598_requiresecuritysignature: super::super::Foundation::BOOL,
    pub sv598_minclientbuffersize: u32,
    pub sv598_serverguid: ::windows::core::GUID,
    pub sv598_ConnectionNoSessionsTimeout: u32,
    pub sv598_IdleThreadTimeOut: u32,
    pub sv598_enableW9xsecuritysignature: super::super::Foundation::BOOL,
    pub sv598_enforcekerberosreauthentication: super::super::Foundation::BOOL,
    pub sv598_disabledos: super::super::Foundation::BOOL,
    pub sv598_lowdiskspaceminimum: u32,
    pub sv598_disablestrictnamechecking: super::super::Foundation::BOOL,
    pub sv598_enableauthenticateusersharing: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SERVER_INFO_598 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SERVER_INFO_598 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SERVER_INFO_598 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_598")
            .field("sv598_maxrawworkitems", &self.sv598_maxrawworkitems)
            .field("sv598_maxthreadsperqueue", &self.sv598_maxthreadsperqueue)
            .field("sv598_producttype", &self.sv598_producttype)
            .field("sv598_serversize", &self.sv598_serversize)
            .field("sv598_connectionlessautodisc", &self.sv598_connectionlessautodisc)
            .field("sv598_sharingviolationretries", &self.sv598_sharingviolationretries)
            .field("sv598_sharingviolationdelay", &self.sv598_sharingviolationdelay)
            .field("sv598_maxglobalopensearch", &self.sv598_maxglobalopensearch)
            .field("sv598_removeduplicatesearches", &self.sv598_removeduplicatesearches)
            .field("sv598_lockviolationoffset", &self.sv598_lockviolationoffset)
            .field("sv598_lockviolationdelay", &self.sv598_lockviolationdelay)
            .field("sv598_mdlreadswitchover", &self.sv598_mdlreadswitchover)
            .field("sv598_cachedopenlimit", &self.sv598_cachedopenlimit)
            .field("sv598_otherqueueaffinity", &self.sv598_otherqueueaffinity)
            .field("sv598_restrictnullsessaccess", &self.sv598_restrictnullsessaccess)
            .field("sv598_enablewfw311directipx", &self.sv598_enablewfw311directipx)
            .field("sv598_queuesamplesecs", &self.sv598_queuesamplesecs)
            .field("sv598_balancecount", &self.sv598_balancecount)
            .field("sv598_preferredaffinity", &self.sv598_preferredaffinity)
            .field("sv598_maxfreerfcbs", &self.sv598_maxfreerfcbs)
            .field("sv598_maxfreemfcbs", &self.sv598_maxfreemfcbs)
            .field("sv598_maxfreelfcbs", &self.sv598_maxfreelfcbs)
            .field("sv598_maxfreepagedpoolchunks", &self.sv598_maxfreepagedpoolchunks)
            .field("sv598_minpagedpoolchunksize", &self.sv598_minpagedpoolchunksize)
            .field("sv598_maxpagedpoolchunksize", &self.sv598_maxpagedpoolchunksize)
            .field("sv598_sendsfrompreferredprocessor", &self.sv598_sendsfrompreferredprocessor)
            .field("sv598_cacheddirectorylimit", &self.sv598_cacheddirectorylimit)
            .field("sv598_maxcopylength", &self.sv598_maxcopylength)
            .field("sv598_enablecompression", &self.sv598_enablecompression)
            .field("sv598_autosharewks", &self.sv598_autosharewks)
            .field("sv598_autoshareserver", &self.sv598_autoshareserver)
            .field("sv598_enablesecuritysignature", &self.sv598_enablesecuritysignature)
            .field("sv598_requiresecuritysignature", &self.sv598_requiresecuritysignature)
            .field("sv598_minclientbuffersize", &self.sv598_minclientbuffersize)
            .field("sv598_serverguid", &self.sv598_serverguid)
            .field("sv598_ConnectionNoSessionsTimeout", &self.sv598_ConnectionNoSessionsTimeout)
            .field("sv598_IdleThreadTimeOut", &self.sv598_IdleThreadTimeOut)
            .field("sv598_enableW9xsecuritysignature", &self.sv598_enableW9xsecuritysignature)
            .field("sv598_enforcekerberosreauthentication", &self.sv598_enforcekerberosreauthentication)
            .field("sv598_disabledos", &self.sv598_disabledos)
            .field("sv598_lowdiskspaceminimum", &self.sv598_lowdiskspaceminimum)
            .field("sv598_disablestrictnamechecking", &self.sv598_disablestrictnamechecking)
            .field("sv598_enableauthenticateusersharing", &self.sv598_enableauthenticateusersharing)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for SERVER_INFO_598 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVER_INFO_598 {
    fn eq(&self, other: &Self) -> bool {
        self.sv598_maxrawworkitems == other.sv598_maxrawworkitems
            && self.sv598_maxthreadsperqueue == other.sv598_maxthreadsperqueue
            && self.sv598_producttype == other.sv598_producttype
            && self.sv598_serversize == other.sv598_serversize
            && self.sv598_connectionlessautodisc == other.sv598_connectionlessautodisc
            && self.sv598_sharingviolationretries == other.sv598_sharingviolationretries
            && self.sv598_sharingviolationdelay == other.sv598_sharingviolationdelay
            && self.sv598_maxglobalopensearch == other.sv598_maxglobalopensearch
            && self.sv598_removeduplicatesearches == other.sv598_removeduplicatesearches
            && self.sv598_lockviolationoffset == other.sv598_lockviolationoffset
            && self.sv598_lockviolationdelay == other.sv598_lockviolationdelay
            && self.sv598_mdlreadswitchover == other.sv598_mdlreadswitchover
            && self.sv598_cachedopenlimit == other.sv598_cachedopenlimit
            && self.sv598_otherqueueaffinity == other.sv598_otherqueueaffinity
            && self.sv598_restrictnullsessaccess == other.sv598_restrictnullsessaccess
            && self.sv598_enablewfw311directipx == other.sv598_enablewfw311directipx
            && self.sv598_queuesamplesecs == other.sv598_queuesamplesecs
            && self.sv598_balancecount == other.sv598_balancecount
            && self.sv598_preferredaffinity == other.sv598_preferredaffinity
            && self.sv598_maxfreerfcbs == other.sv598_maxfreerfcbs
            && self.sv598_maxfreemfcbs == other.sv598_maxfreemfcbs
            && self.sv598_maxfreelfcbs == other.sv598_maxfreelfcbs
            && self.sv598_maxfreepagedpoolchunks == other.sv598_maxfreepagedpoolchunks
            && self.sv598_minpagedpoolchunksize == other.sv598_minpagedpoolchunksize
            && self.sv598_maxpagedpoolchunksize == other.sv598_maxpagedpoolchunksize
            && self.sv598_sendsfrompreferredprocessor == other.sv598_sendsfrompreferredprocessor
            && self.sv598_cacheddirectorylimit == other.sv598_cacheddirectorylimit
            && self.sv598_maxcopylength == other.sv598_maxcopylength
            && self.sv598_enablecompression == other.sv598_enablecompression
            && self.sv598_autosharewks == other.sv598_autosharewks
            && self.sv598_autoshareserver == other.sv598_autoshareserver
            && self.sv598_enablesecuritysignature == other.sv598_enablesecuritysignature
            && self.sv598_requiresecuritysignature == other.sv598_requiresecuritysignature
            && self.sv598_minclientbuffersize == other.sv598_minclientbuffersize
            && self.sv598_serverguid == other.sv598_serverguid
            && self.sv598_ConnectionNoSessionsTimeout == other.sv598_ConnectionNoSessionsTimeout
            && self.sv598_IdleThreadTimeOut == other.sv598_IdleThreadTimeOut
            && self.sv598_enableW9xsecuritysignature == other.sv598_enableW9xsecuritysignature
            && self.sv598_enforcekerberosreauthentication == other.sv598_enforcekerberosreauthentication
            && self.sv598_disabledos == other.sv598_disabledos
            && self.sv598_lowdiskspaceminimum == other.sv598_lowdiskspaceminimum
            && self.sv598_disablestrictnamechecking == other.sv598_disablestrictnamechecking
            && self.sv598_enableauthenticateusersharing == other.sv598_enableauthenticateusersharing
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVER_INFO_598 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVER_INFO_598 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SERVER_INFO_599 {
    pub sv599_sessopens: u32,
    pub sv599_sessvcs: u32,
    pub sv599_opensearch: u32,
    pub sv599_sizreqbuf: u32,
    pub sv599_initworkitems: u32,
    pub sv599_maxworkitems: u32,
    pub sv599_rawworkitems: u32,
    pub sv599_irpstacksize: u32,
    pub sv599_maxrawbuflen: u32,
    pub sv599_sessusers: u32,
    pub sv599_sessconns: u32,
    pub sv599_maxpagedmemoryusage: u32,
    pub sv599_maxnonpagedmemoryusage: u32,
    pub sv599_enablesoftcompat: super::super::Foundation::BOOL,
    pub sv599_enableforcedlogoff: super::super::Foundation::BOOL,
    pub sv599_timesource: super::super::Foundation::BOOL,
    pub sv599_acceptdownlevelapis: super::super::Foundation::BOOL,
    pub sv599_lmannounce: super::super::Foundation::BOOL,
    pub sv599_domain: ::windows::core::PWSTR,
    pub sv599_maxcopyreadlen: u32,
    pub sv599_maxcopywritelen: u32,
    pub sv599_minkeepsearch: u32,
    pub sv599_maxkeepsearch: u32,
    pub sv599_minkeepcomplsearch: u32,
    pub sv599_maxkeepcomplsearch: u32,
    pub sv599_threadcountadd: u32,
    pub sv599_numblockthreads: u32,
    pub sv599_scavtimeout: u32,
    pub sv599_minrcvqueue: u32,
    pub sv599_minfreeworkitems: u32,
    pub sv599_xactmemsize: u32,
    pub sv599_threadpriority: u32,
    pub sv599_maxmpxct: u32,
    pub sv599_oplockbreakwait: u32,
    pub sv599_oplockbreakresponsewait: u32,
    pub sv599_enableoplocks: super::super::Foundation::BOOL,
    pub sv599_enableoplockforceclose: super::super::Foundation::BOOL,
    pub sv599_enablefcbopens: super::super::Foundation::BOOL,
    pub sv599_enableraw: super::super::Foundation::BOOL,
    pub sv599_enablesharednetdrives: super::super::Foundation::BOOL,
    pub sv599_minfreeconnections: u32,
    pub sv599_maxfreeconnections: u32,
    pub sv599_initsesstable: u32,
    pub sv599_initconntable: u32,
    pub sv599_initfiletable: u32,
    pub sv599_initsearchtable: u32,
    pub sv599_alertschedule: u32,
    pub sv599_errorthreshold: u32,
    pub sv599_networkerrorthreshold: u32,
    pub sv599_diskspacethreshold: u32,
    pub sv599_reserved: u32,
    pub sv599_maxlinkdelay: u32,
    pub sv599_minlinkthroughput: u32,
    pub sv599_linkinfovalidtime: u32,
    pub sv599_scavqosinfoupdatetime: u32,
    pub sv599_maxworkitemidletime: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SERVER_INFO_599 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SERVER_INFO_599 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SERVER_INFO_599 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_599")
            .field("sv599_sessopens", &self.sv599_sessopens)
            .field("sv599_sessvcs", &self.sv599_sessvcs)
            .field("sv599_opensearch", &self.sv599_opensearch)
            .field("sv599_sizreqbuf", &self.sv599_sizreqbuf)
            .field("sv599_initworkitems", &self.sv599_initworkitems)
            .field("sv599_maxworkitems", &self.sv599_maxworkitems)
            .field("sv599_rawworkitems", &self.sv599_rawworkitems)
            .field("sv599_irpstacksize", &self.sv599_irpstacksize)
            .field("sv599_maxrawbuflen", &self.sv599_maxrawbuflen)
            .field("sv599_sessusers", &self.sv599_sessusers)
            .field("sv599_sessconns", &self.sv599_sessconns)
            .field("sv599_maxpagedmemoryusage", &self.sv599_maxpagedmemoryusage)
            .field("sv599_maxnonpagedmemoryusage", &self.sv599_maxnonpagedmemoryusage)
            .field("sv599_enablesoftcompat", &self.sv599_enablesoftcompat)
            .field("sv599_enableforcedlogoff", &self.sv599_enableforcedlogoff)
            .field("sv599_timesource", &self.sv599_timesource)
            .field("sv599_acceptdownlevelapis", &self.sv599_acceptdownlevelapis)
            .field("sv599_lmannounce", &self.sv599_lmannounce)
            .field("sv599_domain", &self.sv599_domain)
            .field("sv599_maxcopyreadlen", &self.sv599_maxcopyreadlen)
            .field("sv599_maxcopywritelen", &self.sv599_maxcopywritelen)
            .field("sv599_minkeepsearch", &self.sv599_minkeepsearch)
            .field("sv599_maxkeepsearch", &self.sv599_maxkeepsearch)
            .field("sv599_minkeepcomplsearch", &self.sv599_minkeepcomplsearch)
            .field("sv599_maxkeepcomplsearch", &self.sv599_maxkeepcomplsearch)
            .field("sv599_threadcountadd", &self.sv599_threadcountadd)
            .field("sv599_numblockthreads", &self.sv599_numblockthreads)
            .field("sv599_scavtimeout", &self.sv599_scavtimeout)
            .field("sv599_minrcvqueue", &self.sv599_minrcvqueue)
            .field("sv599_minfreeworkitems", &self.sv599_minfreeworkitems)
            .field("sv599_xactmemsize", &self.sv599_xactmemsize)
            .field("sv599_threadpriority", &self.sv599_threadpriority)
            .field("sv599_maxmpxct", &self.sv599_maxmpxct)
            .field("sv599_oplockbreakwait", &self.sv599_oplockbreakwait)
            .field("sv599_oplockbreakresponsewait", &self.sv599_oplockbreakresponsewait)
            .field("sv599_enableoplocks", &self.sv599_enableoplocks)
            .field("sv599_enableoplockforceclose", &self.sv599_enableoplockforceclose)
            .field("sv599_enablefcbopens", &self.sv599_enablefcbopens)
            .field("sv599_enableraw", &self.sv599_enableraw)
            .field("sv599_enablesharednetdrives", &self.sv599_enablesharednetdrives)
            .field("sv599_minfreeconnections", &self.sv599_minfreeconnections)
            .field("sv599_maxfreeconnections", &self.sv599_maxfreeconnections)
            .field("sv599_initsesstable", &self.sv599_initsesstable)
            .field("sv599_initconntable", &self.sv599_initconntable)
            .field("sv599_initfiletable", &self.sv599_initfiletable)
            .field("sv599_initsearchtable", &self.sv599_initsearchtable)
            .field("sv599_alertschedule", &self.sv599_alertschedule)
            .field("sv599_errorthreshold", &self.sv599_errorthreshold)
            .field("sv599_networkerrorthreshold", &self.sv599_networkerrorthreshold)
            .field("sv599_diskspacethreshold", &self.sv599_diskspacethreshold)
            .field("sv599_reserved", &self.sv599_reserved)
            .field("sv599_maxlinkdelay", &self.sv599_maxlinkdelay)
            .field("sv599_minlinkthroughput", &self.sv599_minlinkthroughput)
            .field("sv599_linkinfovalidtime", &self.sv599_linkinfovalidtime)
            .field("sv599_scavqosinfoupdatetime", &self.sv599_scavqosinfoupdatetime)
            .field("sv599_maxworkitemidletime", &self.sv599_maxworkitemidletime)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for SERVER_INFO_599 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVER_INFO_599 {
    fn eq(&self, other: &Self) -> bool {
        self.sv599_sessopens == other.sv599_sessopens
            && self.sv599_sessvcs == other.sv599_sessvcs
            && self.sv599_opensearch == other.sv599_opensearch
            && self.sv599_sizreqbuf == other.sv599_sizreqbuf
            && self.sv599_initworkitems == other.sv599_initworkitems
            && self.sv599_maxworkitems == other.sv599_maxworkitems
            && self.sv599_rawworkitems == other.sv599_rawworkitems
            && self.sv599_irpstacksize == other.sv599_irpstacksize
            && self.sv599_maxrawbuflen == other.sv599_maxrawbuflen
            && self.sv599_sessusers == other.sv599_sessusers
            && self.sv599_sessconns == other.sv599_sessconns
            && self.sv599_maxpagedmemoryusage == other.sv599_maxpagedmemoryusage
            && self.sv599_maxnonpagedmemoryusage == other.sv599_maxnonpagedmemoryusage
            && self.sv599_enablesoftcompat == other.sv599_enablesoftcompat
            && self.sv599_enableforcedlogoff == other.sv599_enableforcedlogoff
            && self.sv599_timesource == other.sv599_timesource
            && self.sv599_acceptdownlevelapis == other.sv599_acceptdownlevelapis
            && self.sv599_lmannounce == other.sv599_lmannounce
            && self.sv599_domain == other.sv599_domain
            && self.sv599_maxcopyreadlen == other.sv599_maxcopyreadlen
            && self.sv599_maxcopywritelen == other.sv599_maxcopywritelen
            && self.sv599_minkeepsearch == other.sv599_minkeepsearch
            && self.sv599_maxkeepsearch == other.sv599_maxkeepsearch
            && self.sv599_minkeepcomplsearch == other.sv599_minkeepcomplsearch
            && self.sv599_maxkeepcomplsearch == other.sv599_maxkeepcomplsearch
            && self.sv599_threadcountadd == other.sv599_threadcountadd
            && self.sv599_numblockthreads == other.sv599_numblockthreads
            && self.sv599_scavtimeout == other.sv599_scavtimeout
            && self.sv599_minrcvqueue == other.sv599_minrcvqueue
            && self.sv599_minfreeworkitems == other.sv599_minfreeworkitems
            && self.sv599_xactmemsize == other.sv599_xactmemsize
            && self.sv599_threadpriority == other.sv599_threadpriority
            && self.sv599_maxmpxct == other.sv599_maxmpxct
            && self.sv599_oplockbreakwait == other.sv599_oplockbreakwait
            && self.sv599_oplockbreakresponsewait == other.sv599_oplockbreakresponsewait
            && self.sv599_enableoplocks == other.sv599_enableoplocks
            && self.sv599_enableoplockforceclose == other.sv599_enableoplockforceclose
            && self.sv599_enablefcbopens == other.sv599_enablefcbopens
            && self.sv599_enableraw == other.sv599_enableraw
            && self.sv599_enablesharednetdrives == other.sv599_enablesharednetdrives
            && self.sv599_minfreeconnections == other.sv599_minfreeconnections
            && self.sv599_maxfreeconnections == other.sv599_maxfreeconnections
            && self.sv599_initsesstable == other.sv599_initsesstable
            && self.sv599_initconntable == other.sv599_initconntable
            && self.sv599_initfiletable == other.sv599_initfiletable
            && self.sv599_initsearchtable == other.sv599_initsearchtable
            && self.sv599_alertschedule == other.sv599_alertschedule
            && self.sv599_errorthreshold == other.sv599_errorthreshold
            && self.sv599_networkerrorthreshold == other.sv599_networkerrorthreshold
            && self.sv599_diskspacethreshold == other.sv599_diskspacethreshold
            && self.sv599_reserved == other.sv599_reserved
            && self.sv599_maxlinkdelay == other.sv599_maxlinkdelay
            && self.sv599_minlinkthroughput == other.sv599_minlinkthroughput
            && self.sv599_linkinfovalidtime == other.sv599_linkinfovalidtime
            && self.sv599_scavqosinfoupdatetime == other.sv599_scavqosinfoupdatetime
            && self.sv599_maxworkitemidletime == other.sv599_maxworkitemidletime
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVER_INFO_599 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVER_INFO_599 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SERVER_TRANSPORT_INFO_0 {
    pub svti0_numberofvcs: u32,
    pub svti0_transportname: ::windows::core::PWSTR,
    pub svti0_transportaddress: *mut u8,
    pub svti0_transportaddresslength: u32,
    pub svti0_networkaddress: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for SERVER_TRANSPORT_INFO_0 {}
impl ::core::clone::Clone for SERVER_TRANSPORT_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVER_TRANSPORT_INFO_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_TRANSPORT_INFO_0").field("svti0_numberofvcs", &self.svti0_numberofvcs).field("svti0_transportname", &self.svti0_transportname).field("svti0_transportaddress", &self.svti0_transportaddress).field("svti0_transportaddresslength", &self.svti0_transportaddresslength).field("svti0_networkaddress", &self.svti0_networkaddress).finish()
    }
}
impl ::windows::core::TypeKind for SERVER_TRANSPORT_INFO_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVER_TRANSPORT_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        self.svti0_numberofvcs == other.svti0_numberofvcs && self.svti0_transportname == other.svti0_transportname && self.svti0_transportaddress == other.svti0_transportaddress && self.svti0_transportaddresslength == other.svti0_transportaddresslength && self.svti0_networkaddress == other.svti0_networkaddress
    }
}
impl ::core::cmp::Eq for SERVER_TRANSPORT_INFO_0 {}
impl ::core::default::Default for SERVER_TRANSPORT_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SERVER_TRANSPORT_INFO_1 {
    pub svti1_numberofvcs: u32,
    pub svti1_transportname: ::windows::core::PWSTR,
    pub svti1_transportaddress: *mut u8,
    pub svti1_transportaddresslength: u32,
    pub svti1_networkaddress: ::windows::core::PWSTR,
    pub svti1_domain: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for SERVER_TRANSPORT_INFO_1 {}
impl ::core::clone::Clone for SERVER_TRANSPORT_INFO_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVER_TRANSPORT_INFO_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_TRANSPORT_INFO_1").field("svti1_numberofvcs", &self.svti1_numberofvcs).field("svti1_transportname", &self.svti1_transportname).field("svti1_transportaddress", &self.svti1_transportaddress).field("svti1_transportaddresslength", &self.svti1_transportaddresslength).field("svti1_networkaddress", &self.svti1_networkaddress).field("svti1_domain", &self.svti1_domain).finish()
    }
}
impl ::windows::core::TypeKind for SERVER_TRANSPORT_INFO_1 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVER_TRANSPORT_INFO_1 {
    fn eq(&self, other: &Self) -> bool {
        self.svti1_numberofvcs == other.svti1_numberofvcs && self.svti1_transportname == other.svti1_transportname && self.svti1_transportaddress == other.svti1_transportaddress && self.svti1_transportaddresslength == other.svti1_transportaddresslength && self.svti1_networkaddress == other.svti1_networkaddress && self.svti1_domain == other.svti1_domain
    }
}
impl ::core::cmp::Eq for SERVER_TRANSPORT_INFO_1 {}
impl ::core::default::Default for SERVER_TRANSPORT_INFO_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SERVER_TRANSPORT_INFO_2 {
    pub svti2_numberofvcs: u32,
    pub svti2_transportname: ::windows::core::PWSTR,
    pub svti2_transportaddress: *mut u8,
    pub svti2_transportaddresslength: u32,
    pub svti2_networkaddress: ::windows::core::PWSTR,
    pub svti2_domain: ::windows::core::PWSTR,
    pub svti2_flags: u32,
}
impl ::core::marker::Copy for SERVER_TRANSPORT_INFO_2 {}
impl ::core::clone::Clone for SERVER_TRANSPORT_INFO_2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVER_TRANSPORT_INFO_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_TRANSPORT_INFO_2").field("svti2_numberofvcs", &self.svti2_numberofvcs).field("svti2_transportname", &self.svti2_transportname).field("svti2_transportaddress", &self.svti2_transportaddress).field("svti2_transportaddresslength", &self.svti2_transportaddresslength).field("svti2_networkaddress", &self.svti2_networkaddress).field("svti2_domain", &self.svti2_domain).field("svti2_flags", &self.svti2_flags).finish()
    }
}
impl ::windows::core::TypeKind for SERVER_TRANSPORT_INFO_2 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVER_TRANSPORT_INFO_2 {
    fn eq(&self, other: &Self) -> bool {
        self.svti2_numberofvcs == other.svti2_numberofvcs && self.svti2_transportname == other.svti2_transportname && self.svti2_transportaddress == other.svti2_transportaddress && self.svti2_transportaddresslength == other.svti2_transportaddresslength && self.svti2_networkaddress == other.svti2_networkaddress && self.svti2_domain == other.svti2_domain && self.svti2_flags == other.svti2_flags
    }
}
impl ::core::cmp::Eq for SERVER_TRANSPORT_INFO_2 {}
impl ::core::default::Default for SERVER_TRANSPORT_INFO_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SERVER_TRANSPORT_INFO_3 {
    pub svti3_numberofvcs: u32,
    pub svti3_transportname: ::windows::core::PWSTR,
    pub svti3_transportaddress: *mut u8,
    pub svti3_transportaddresslength: u32,
    pub svti3_networkaddress: ::windows::core::PWSTR,
    pub svti3_domain: ::windows::core::PWSTR,
    pub svti3_flags: u32,
    pub svti3_passwordlength: u32,
    pub svti3_password: [u8; 256],
}
impl ::core::marker::Copy for SERVER_TRANSPORT_INFO_3 {}
impl ::core::clone::Clone for SERVER_TRANSPORT_INFO_3 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVER_TRANSPORT_INFO_3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_TRANSPORT_INFO_3")
            .field("svti3_numberofvcs", &self.svti3_numberofvcs)
            .field("svti3_transportname", &self.svti3_transportname)
            .field("svti3_transportaddress", &self.svti3_transportaddress)
            .field("svti3_transportaddresslength", &self.svti3_transportaddresslength)
            .field("svti3_networkaddress", &self.svti3_networkaddress)
            .field("svti3_domain", &self.svti3_domain)
            .field("svti3_flags", &self.svti3_flags)
            .field("svti3_passwordlength", &self.svti3_passwordlength)
            .field("svti3_password", &self.svti3_password)
            .finish()
    }
}
impl ::windows::core::TypeKind for SERVER_TRANSPORT_INFO_3 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVER_TRANSPORT_INFO_3 {
    fn eq(&self, other: &Self) -> bool {
        self.svti3_numberofvcs == other.svti3_numberofvcs && self.svti3_transportname == other.svti3_transportname && self.svti3_transportaddress == other.svti3_transportaddress && self.svti3_transportaddresslength == other.svti3_transportaddresslength && self.svti3_networkaddress == other.svti3_networkaddress && self.svti3_domain == other.svti3_domain && self.svti3_flags == other.svti3_flags && self.svti3_passwordlength == other.svti3_passwordlength && self.svti3_password == other.svti3_password
    }
}
impl ::core::cmp::Eq for SERVER_TRANSPORT_INFO_3 {}
impl ::core::default::Default for SERVER_TRANSPORT_INFO_3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SERVICE_INFO_0 {
    pub svci0_name: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for SERVICE_INFO_0 {}
impl ::core::clone::Clone for SERVICE_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVICE_INFO_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVICE_INFO_0").field("svci0_name", &self.svci0_name).finish()
    }
}
impl ::windows::core::TypeKind for SERVICE_INFO_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVICE_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        self.svci0_name == other.svci0_name
    }
}
impl ::core::cmp::Eq for SERVICE_INFO_0 {}
impl ::core::default::Default for SERVICE_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SERVICE_INFO_1 {
    pub svci1_name: ::windows::core::PWSTR,
    pub svci1_status: u32,
    pub svci1_code: u32,
    pub svci1_pid: u32,
}
impl ::core::marker::Copy for SERVICE_INFO_1 {}
impl ::core::clone::Clone for SERVICE_INFO_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVICE_INFO_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVICE_INFO_1").field("svci1_name", &self.svci1_name).field("svci1_status", &self.svci1_status).field("svci1_code", &self.svci1_code).field("svci1_pid", &self.svci1_pid).finish()
    }
}
impl ::windows::core::TypeKind for SERVICE_INFO_1 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVICE_INFO_1 {
    fn eq(&self, other: &Self) -> bool {
        self.svci1_name == other.svci1_name && self.svci1_status == other.svci1_status && self.svci1_code == other.svci1_code && self.svci1_pid == other.svci1_pid
    }
}
impl ::core::cmp::Eq for SERVICE_INFO_1 {}
impl ::core::default::Default for SERVICE_INFO_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SERVICE_INFO_2 {
    pub svci2_name: ::windows::core::PWSTR,
    pub svci2_status: u32,
    pub svci2_code: u32,
    pub svci2_pid: u32,
    pub svci2_text: ::windows::core::PWSTR,
    pub svci2_specific_error: u32,
    pub svci2_display_name: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for SERVICE_INFO_2 {}
impl ::core::clone::Clone for SERVICE_INFO_2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVICE_INFO_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVICE_INFO_2").field("svci2_name", &self.svci2_name).field("svci2_status", &self.svci2_status).field("svci2_code", &self.svci2_code).field("svci2_pid", &self.svci2_pid).field("svci2_text", &self.svci2_text).field("svci2_specific_error", &self.svci2_specific_error).field("svci2_display_name", &self.svci2_display_name).finish()
    }
}
impl ::windows::core::TypeKind for SERVICE_INFO_2 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVICE_INFO_2 {
    fn eq(&self, other: &Self) -> bool {
        self.svci2_name == other.svci2_name && self.svci2_status == other.svci2_status && self.svci2_code == other.svci2_code && self.svci2_pid == other.svci2_pid && self.svci2_text == other.svci2_text && self.svci2_specific_error == other.svci2_specific_error && self.svci2_display_name == other.svci2_display_name
    }
}
impl ::core::cmp::Eq for SERVICE_INFO_2 {}
impl ::core::default::Default for SERVICE_INFO_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SMB_COMPRESSION_INFO {
    pub Switch: super::super::Foundation::BOOLEAN,
    pub Reserved1: u8,
    pub Reserved2: u16,
    pub Reserved3: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SMB_COMPRESSION_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SMB_COMPRESSION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SMB_COMPRESSION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SMB_COMPRESSION_INFO").field("Switch", &self.Switch).field("Reserved1", &self.Reserved1).field("Reserved2", &self.Reserved2).field("Reserved3", &self.Reserved3).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for SMB_COMPRESSION_INFO {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SMB_COMPRESSION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Switch == other.Switch && self.Reserved1 == other.Reserved1 && self.Reserved2 == other.Reserved2 && self.Reserved3 == other.Reserved3
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SMB_COMPRESSION_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SMB_COMPRESSION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SMB_TREE_CONNECT_PARAMETERS {
    pub EABufferOffset: u32,
    pub EABufferLen: u32,
    pub CreateOptions: u32,
    pub TreeConnectAttributes: u32,
}
impl ::core::marker::Copy for SMB_TREE_CONNECT_PARAMETERS {}
impl ::core::clone::Clone for SMB_TREE_CONNECT_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SMB_TREE_CONNECT_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SMB_TREE_CONNECT_PARAMETERS").field("EABufferOffset", &self.EABufferOffset).field("EABufferLen", &self.EABufferLen).field("CreateOptions", &self.CreateOptions).field("TreeConnectAttributes", &self.TreeConnectAttributes).finish()
    }
}
impl ::windows::core::TypeKind for SMB_TREE_CONNECT_PARAMETERS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SMB_TREE_CONNECT_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.EABufferOffset == other.EABufferOffset && self.EABufferLen == other.EABufferLen && self.CreateOptions == other.CreateOptions && self.TreeConnectAttributes == other.TreeConnectAttributes
    }
}
impl ::core::cmp::Eq for SMB_TREE_CONNECT_PARAMETERS {}
impl ::core::default::Default for SMB_TREE_CONNECT_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct SMB_USE_OPTION_COMPRESSION_PARAMETERS {
    pub Tag: u32,
    pub Length: u16,
    pub Reserved: u16,
}
impl ::core::marker::Copy for SMB_USE_OPTION_COMPRESSION_PARAMETERS {}
impl ::core::clone::Clone for SMB_USE_OPTION_COMPRESSION_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SMB_USE_OPTION_COMPRESSION_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SMB_USE_OPTION_COMPRESSION_PARAMETERS").field("Tag", &self.Tag).field("Length", &self.Length).field("Reserved", &self.Reserved).finish()
    }
}
impl ::windows::core::TypeKind for SMB_USE_OPTION_COMPRESSION_PARAMETERS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SMB_USE_OPTION_COMPRESSION_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Tag == other.Tag && self.Length == other.Length && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for SMB_USE_OPTION_COMPRESSION_PARAMETERS {}
impl ::core::default::Default for SMB_USE_OPTION_COMPRESSION_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct STD_ALERT {
    pub alrt_timestamp: u32,
    pub alrt_eventname: [u16; 17],
    pub alrt_servicename: [u16; 81],
}
impl ::core::marker::Copy for STD_ALERT {}
impl ::core::clone::Clone for STD_ALERT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for STD_ALERT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STD_ALERT").field("alrt_timestamp", &self.alrt_timestamp).field("alrt_eventname", &self.alrt_eventname).field("alrt_servicename", &self.alrt_servicename).finish()
    }
}
impl ::windows::core::TypeKind for STD_ALERT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for STD_ALERT {
    fn eq(&self, other: &Self) -> bool {
        self.alrt_timestamp == other.alrt_timestamp && self.alrt_eventname == other.alrt_eventname && self.alrt_servicename == other.alrt_servicename
    }
}
impl ::core::cmp::Eq for STD_ALERT {}
impl ::core::default::Default for STD_ALERT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct TIME_OF_DAY_INFO {
    pub tod_elapsedt: u32,
    pub tod_msecs: u32,
    pub tod_hours: u32,
    pub tod_mins: u32,
    pub tod_secs: u32,
    pub tod_hunds: u32,
    pub tod_timezone: i32,
    pub tod_tinterval: u32,
    pub tod_day: u32,
    pub tod_month: u32,
    pub tod_year: u32,
    pub tod_weekday: u32,
}
impl ::core::marker::Copy for TIME_OF_DAY_INFO {}
impl ::core::clone::Clone for TIME_OF_DAY_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TIME_OF_DAY_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TIME_OF_DAY_INFO")
            .field("tod_elapsedt", &self.tod_elapsedt)
            .field("tod_msecs", &self.tod_msecs)
            .field("tod_hours", &self.tod_hours)
            .field("tod_mins", &self.tod_mins)
            .field("tod_secs", &self.tod_secs)
            .field("tod_hunds", &self.tod_hunds)
            .field("tod_timezone", &self.tod_timezone)
            .field("tod_tinterval", &self.tod_tinterval)
            .field("tod_day", &self.tod_day)
            .field("tod_month", &self.tod_month)
            .field("tod_year", &self.tod_year)
            .field("tod_weekday", &self.tod_weekday)
            .finish()
    }
}
impl ::windows::core::TypeKind for TIME_OF_DAY_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for TIME_OF_DAY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.tod_elapsedt == other.tod_elapsedt && self.tod_msecs == other.tod_msecs && self.tod_hours == other.tod_hours && self.tod_mins == other.tod_mins && self.tod_secs == other.tod_secs && self.tod_hunds == other.tod_hunds && self.tod_timezone == other.tod_timezone && self.tod_tinterval == other.tod_tinterval && self.tod_day == other.tod_day && self.tod_month == other.tod_month && self.tod_year == other.tod_year && self.tod_weekday == other.tod_weekday
    }
}
impl ::core::cmp::Eq for TIME_OF_DAY_INFO {}
impl ::core::default::Default for TIME_OF_DAY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TRANSPORT_INFO {
    pub Type: TRANSPORT_TYPE,
    pub SkipCertificateCheck: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TRANSPORT_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TRANSPORT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TRANSPORT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRANSPORT_INFO").field("Type", &self.Type).field("SkipCertificateCheck", &self.SkipCertificateCheck).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for TRANSPORT_INFO {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TRANSPORT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.SkipCertificateCheck == other.SkipCertificateCheck
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TRANSPORT_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TRANSPORT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct USER_INFO_0 {
    pub usri0_name: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for USER_INFO_0 {}
impl ::core::clone::Clone for USER_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for USER_INFO_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_INFO_0").field("usri0_name", &self.usri0_name).finish()
    }
}
impl ::windows::core::TypeKind for USER_INFO_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for USER_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        self.usri0_name == other.usri0_name
    }
}
impl ::core::cmp::Eq for USER_INFO_0 {}
impl ::core::default::Default for USER_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct USER_INFO_1 {
    pub usri1_name: ::windows::core::PWSTR,
    pub usri1_password: ::windows::core::PWSTR,
    pub usri1_password_age: u32,
    pub usri1_priv: USER_PRIV,
    pub usri1_home_dir: ::windows::core::PWSTR,
    pub usri1_comment: ::windows::core::PWSTR,
    pub usri1_flags: USER_ACCOUNT_FLAGS,
    pub usri1_script_path: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for USER_INFO_1 {}
impl ::core::clone::Clone for USER_INFO_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for USER_INFO_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_INFO_1").field("usri1_name", &self.usri1_name).field("usri1_password", &self.usri1_password).field("usri1_password_age", &self.usri1_password_age).field("usri1_priv", &self.usri1_priv).field("usri1_home_dir", &self.usri1_home_dir).field("usri1_comment", &self.usri1_comment).field("usri1_flags", &self.usri1_flags).field("usri1_script_path", &self.usri1_script_path).finish()
    }
}
impl ::windows::core::TypeKind for USER_INFO_1 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for USER_INFO_1 {
    fn eq(&self, other: &Self) -> bool {
        self.usri1_name == other.usri1_name && self.usri1_password == other.usri1_password && self.usri1_password_age == other.usri1_password_age && self.usri1_priv == other.usri1_priv && self.usri1_home_dir == other.usri1_home_dir && self.usri1_comment == other.usri1_comment && self.usri1_flags == other.usri1_flags && self.usri1_script_path == other.usri1_script_path
    }
}
impl ::core::cmp::Eq for USER_INFO_1 {}
impl ::core::default::Default for USER_INFO_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct USER_INFO_10 {
    pub usri10_name: ::windows::core::PWSTR,
    pub usri10_comment: ::windows::core::PWSTR,
    pub usri10_usr_comment: ::windows::core::PWSTR,
    pub usri10_full_name: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for USER_INFO_10 {}
impl ::core::clone::Clone for USER_INFO_10 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for USER_INFO_10 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_INFO_10").field("usri10_name", &self.usri10_name).field("usri10_comment", &self.usri10_comment).field("usri10_usr_comment", &self.usri10_usr_comment).field("usri10_full_name", &self.usri10_full_name).finish()
    }
}
impl ::windows::core::TypeKind for USER_INFO_10 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for USER_INFO_10 {
    fn eq(&self, other: &Self) -> bool {
        self.usri10_name == other.usri10_name && self.usri10_comment == other.usri10_comment && self.usri10_usr_comment == other.usri10_usr_comment && self.usri10_full_name == other.usri10_full_name
    }
}
impl ::core::cmp::Eq for USER_INFO_10 {}
impl ::core::default::Default for USER_INFO_10 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct USER_INFO_1003 {
    pub usri1003_password: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for USER_INFO_1003 {}
impl ::core::clone::Clone for USER_INFO_1003 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for USER_INFO_1003 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_INFO_1003").field("usri1003_password", &self.usri1003_password).finish()
    }
}
impl ::windows::core::TypeKind for USER_INFO_1003 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for USER_INFO_1003 {
    fn eq(&self, other: &Self) -> bool {
        self.usri1003_password == other.usri1003_password
    }
}
impl ::core::cmp::Eq for USER_INFO_1003 {}
impl ::core::default::Default for USER_INFO_1003 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct USER_INFO_1005 {
    pub usri1005_priv: USER_PRIV,
}
impl ::core::marker::Copy for USER_INFO_1005 {}
impl ::core::clone::Clone for USER_INFO_1005 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for USER_INFO_1005 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_INFO_1005").field("usri1005_priv", &self.usri1005_priv).finish()
    }
}
impl ::windows::core::TypeKind for USER_INFO_1005 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for USER_INFO_1005 {
    fn eq(&self, other: &Self) -> bool {
        self.usri1005_priv == other.usri1005_priv
    }
}
impl ::core::cmp::Eq for USER_INFO_1005 {}
impl ::core::default::Default for USER_INFO_1005 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct USER_INFO_1006 {
    pub usri1006_home_dir: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for USER_INFO_1006 {}
impl ::core::clone::Clone for USER_INFO_1006 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for USER_INFO_1006 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_INFO_1006").field("usri1006_home_dir", &self.usri1006_home_dir).finish()
    }
}
impl ::windows::core::TypeKind for USER_INFO_1006 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for USER_INFO_1006 {
    fn eq(&self, other: &Self) -> bool {
        self.usri1006_home_dir == other.usri1006_home_dir
    }
}
impl ::core::cmp::Eq for USER_INFO_1006 {}
impl ::core::default::Default for USER_INFO_1006 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct USER_INFO_1007 {
    pub usri1007_comment: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for USER_INFO_1007 {}
impl ::core::clone::Clone for USER_INFO_1007 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for USER_INFO_1007 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_INFO_1007").field("usri1007_comment", &self.usri1007_comment).finish()
    }
}
impl ::windows::core::TypeKind for USER_INFO_1007 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for USER_INFO_1007 {
    fn eq(&self, other: &Self) -> bool {
        self.usri1007_comment == other.usri1007_comment
    }
}
impl ::core::cmp::Eq for USER_INFO_1007 {}
impl ::core::default::Default for USER_INFO_1007 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct USER_INFO_1008 {
    pub usri1008_flags: USER_ACCOUNT_FLAGS,
}
impl ::core::marker::Copy for USER_INFO_1008 {}
impl ::core::clone::Clone for USER_INFO_1008 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for USER_INFO_1008 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_INFO_1008").field("usri1008_flags", &self.usri1008_flags).finish()
    }
}
impl ::windows::core::TypeKind for USER_INFO_1008 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for USER_INFO_1008 {
    fn eq(&self, other: &Self) -> bool {
        self.usri1008_flags == other.usri1008_flags
    }
}
impl ::core::cmp::Eq for USER_INFO_1008 {}
impl ::core::default::Default for USER_INFO_1008 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct USER_INFO_1009 {
    pub usri1009_script_path: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for USER_INFO_1009 {}
impl ::core::clone::Clone for USER_INFO_1009 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for USER_INFO_1009 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_INFO_1009").field("usri1009_script_path", &self.usri1009_script_path).finish()
    }
}
impl ::windows::core::TypeKind for USER_INFO_1009 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for USER_INFO_1009 {
    fn eq(&self, other: &Self) -> bool {
        self.usri1009_script_path == other.usri1009_script_path
    }
}
impl ::core::cmp::Eq for USER_INFO_1009 {}
impl ::core::default::Default for USER_INFO_1009 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct USER_INFO_1010 {
    pub usri1010_auth_flags: AF_OP,
}
impl ::core::marker::Copy for USER_INFO_1010 {}
impl ::core::clone::Clone for USER_INFO_1010 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for USER_INFO_1010 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_INFO_1010").field("usri1010_auth_flags", &self.usri1010_auth_flags).finish()
    }
}
impl ::windows::core::TypeKind for USER_INFO_1010 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for USER_INFO_1010 {
    fn eq(&self, other: &Self) -> bool {
        self.usri1010_auth_flags == other.usri1010_auth_flags
    }
}
impl ::core::cmp::Eq for USER_INFO_1010 {}
impl ::core::default::Default for USER_INFO_1010 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct USER_INFO_1011 {
    pub usri1011_full_name: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for USER_INFO_1011 {}
impl ::core::clone::Clone for USER_INFO_1011 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for USER_INFO_1011 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_INFO_1011").field("usri1011_full_name", &self.usri1011_full_name).finish()
    }
}
impl ::windows::core::TypeKind for USER_INFO_1011 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for USER_INFO_1011 {
    fn eq(&self, other: &Self) -> bool {
        self.usri1011_full_name == other.usri1011_full_name
    }
}
impl ::core::cmp::Eq for USER_INFO_1011 {}
impl ::core::default::Default for USER_INFO_1011 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct USER_INFO_1012 {
    pub usri1012_usr_comment: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for USER_INFO_1012 {}
impl ::core::clone::Clone for USER_INFO_1012 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for USER_INFO_1012 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_INFO_1012").field("usri1012_usr_comment", &self.usri1012_usr_comment).finish()
    }
}
impl ::windows::core::TypeKind for USER_INFO_1012 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for USER_INFO_1012 {
    fn eq(&self, other: &Self) -> bool {
        self.usri1012_usr_comment == other.usri1012_usr_comment
    }
}
impl ::core::cmp::Eq for USER_INFO_1012 {}
impl ::core::default::Default for USER_INFO_1012 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct USER_INFO_1013 {
    pub usri1013_parms: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for USER_INFO_1013 {}
impl ::core::clone::Clone for USER_INFO_1013 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for USER_INFO_1013 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_INFO_1013").field("usri1013_parms", &self.usri1013_parms).finish()
    }
}
impl ::windows::core::TypeKind for USER_INFO_1013 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for USER_INFO_1013 {
    fn eq(&self, other: &Self) -> bool {
        self.usri1013_parms == other.usri1013_parms
    }
}
impl ::core::cmp::Eq for USER_INFO_1013 {}
impl ::core::default::Default for USER_INFO_1013 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct USER_INFO_1014 {
    pub usri1014_workstations: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for USER_INFO_1014 {}
impl ::core::clone::Clone for USER_INFO_1014 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for USER_INFO_1014 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_INFO_1014").field("usri1014_workstations", &self.usri1014_workstations).finish()
    }
}
impl ::windows::core::TypeKind for USER_INFO_1014 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for USER_INFO_1014 {
    fn eq(&self, other: &Self) -> bool {
        self.usri1014_workstations == other.usri1014_workstations
    }
}
impl ::core::cmp::Eq for USER_INFO_1014 {}
impl ::core::default::Default for USER_INFO_1014 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct USER_INFO_1017 {
    pub usri1017_acct_expires: u32,
}
impl ::core::marker::Copy for USER_INFO_1017 {}
impl ::core::clone::Clone for USER_INFO_1017 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for USER_INFO_1017 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_INFO_1017").field("usri1017_acct_expires", &self.usri1017_acct_expires).finish()
    }
}
impl ::windows::core::TypeKind for USER_INFO_1017 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for USER_INFO_1017 {
    fn eq(&self, other: &Self) -> bool {
        self.usri1017_acct_expires == other.usri1017_acct_expires
    }
}
impl ::core::cmp::Eq for USER_INFO_1017 {}
impl ::core::default::Default for USER_INFO_1017 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct USER_INFO_1018 {
    pub usri1018_max_storage: u32,
}
impl ::core::marker::Copy for USER_INFO_1018 {}
impl ::core::clone::Clone for USER_INFO_1018 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for USER_INFO_1018 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_INFO_1018").field("usri1018_max_storage", &self.usri1018_max_storage).finish()
    }
}
impl ::windows::core::TypeKind for USER_INFO_1018 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for USER_INFO_1018 {
    fn eq(&self, other: &Self) -> bool {
        self.usri1018_max_storage == other.usri1018_max_storage
    }
}
impl ::core::cmp::Eq for USER_INFO_1018 {}
impl ::core::default::Default for USER_INFO_1018 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct USER_INFO_1020 {
    pub usri1020_units_per_week: u32,
    pub usri1020_logon_hours: *mut u8,
}
impl ::core::marker::Copy for USER_INFO_1020 {}
impl ::core::clone::Clone for USER_INFO_1020 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for USER_INFO_1020 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_INFO_1020").field("usri1020_units_per_week", &self.usri1020_units_per_week).field("usri1020_logon_hours", &self.usri1020_logon_hours).finish()
    }
}
impl ::windows::core::TypeKind for USER_INFO_1020 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for USER_INFO_1020 {
    fn eq(&self, other: &Self) -> bool {
        self.usri1020_units_per_week == other.usri1020_units_per_week && self.usri1020_logon_hours == other.usri1020_logon_hours
    }
}
impl ::core::cmp::Eq for USER_INFO_1020 {}
impl ::core::default::Default for USER_INFO_1020 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct USER_INFO_1023 {
    pub usri1023_logon_server: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for USER_INFO_1023 {}
impl ::core::clone::Clone for USER_INFO_1023 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for USER_INFO_1023 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_INFO_1023").field("usri1023_logon_server", &self.usri1023_logon_server).finish()
    }
}
impl ::windows::core::TypeKind for USER_INFO_1023 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for USER_INFO_1023 {
    fn eq(&self, other: &Self) -> bool {
        self.usri1023_logon_server == other.usri1023_logon_server
    }
}
impl ::core::cmp::Eq for USER_INFO_1023 {}
impl ::core::default::Default for USER_INFO_1023 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct USER_INFO_1024 {
    pub usri1024_country_code: u32,
}
impl ::core::marker::Copy for USER_INFO_1024 {}
impl ::core::clone::Clone for USER_INFO_1024 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for USER_INFO_1024 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_INFO_1024").field("usri1024_country_code", &self.usri1024_country_code).finish()
    }
}
impl ::windows::core::TypeKind for USER_INFO_1024 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for USER_INFO_1024 {
    fn eq(&self, other: &Self) -> bool {
        self.usri1024_country_code == other.usri1024_country_code
    }
}
impl ::core::cmp::Eq for USER_INFO_1024 {}
impl ::core::default::Default for USER_INFO_1024 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct USER_INFO_1025 {
    pub usri1025_code_page: u32,
}
impl ::core::marker::Copy for USER_INFO_1025 {}
impl ::core::clone::Clone for USER_INFO_1025 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for USER_INFO_1025 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_INFO_1025").field("usri1025_code_page", &self.usri1025_code_page).finish()
    }
}
impl ::windows::core::TypeKind for USER_INFO_1025 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for USER_INFO_1025 {
    fn eq(&self, other: &Self) -> bool {
        self.usri1025_code_page == other.usri1025_code_page
    }
}
impl ::core::cmp::Eq for USER_INFO_1025 {}
impl ::core::default::Default for USER_INFO_1025 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct USER_INFO_1051 {
    pub usri1051_primary_group_id: u32,
}
impl ::core::marker::Copy for USER_INFO_1051 {}
impl ::core::clone::Clone for USER_INFO_1051 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for USER_INFO_1051 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_INFO_1051").field("usri1051_primary_group_id", &self.usri1051_primary_group_id).finish()
    }
}
impl ::windows::core::TypeKind for USER_INFO_1051 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for USER_INFO_1051 {
    fn eq(&self, other: &Self) -> bool {
        self.usri1051_primary_group_id == other.usri1051_primary_group_id
    }
}
impl ::core::cmp::Eq for USER_INFO_1051 {}
impl ::core::default::Default for USER_INFO_1051 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct USER_INFO_1052 {
    pub usri1052_profile: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for USER_INFO_1052 {}
impl ::core::clone::Clone for USER_INFO_1052 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for USER_INFO_1052 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_INFO_1052").field("usri1052_profile", &self.usri1052_profile).finish()
    }
}
impl ::windows::core::TypeKind for USER_INFO_1052 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for USER_INFO_1052 {
    fn eq(&self, other: &Self) -> bool {
        self.usri1052_profile == other.usri1052_profile
    }
}
impl ::core::cmp::Eq for USER_INFO_1052 {}
impl ::core::default::Default for USER_INFO_1052 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct USER_INFO_1053 {
    pub usri1053_home_dir_drive: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for USER_INFO_1053 {}
impl ::core::clone::Clone for USER_INFO_1053 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for USER_INFO_1053 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_INFO_1053").field("usri1053_home_dir_drive", &self.usri1053_home_dir_drive).finish()
    }
}
impl ::windows::core::TypeKind for USER_INFO_1053 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for USER_INFO_1053 {
    fn eq(&self, other: &Self) -> bool {
        self.usri1053_home_dir_drive == other.usri1053_home_dir_drive
    }
}
impl ::core::cmp::Eq for USER_INFO_1053 {}
impl ::core::default::Default for USER_INFO_1053 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct USER_INFO_11 {
    pub usri11_name: ::windows::core::PWSTR,
    pub usri11_comment: ::windows::core::PWSTR,
    pub usri11_usr_comment: ::windows::core::PWSTR,
    pub usri11_full_name: ::windows::core::PWSTR,
    pub usri11_priv: USER_PRIV,
    pub usri11_auth_flags: AF_OP,
    pub usri11_password_age: u32,
    pub usri11_home_dir: ::windows::core::PWSTR,
    pub usri11_parms: ::windows::core::PWSTR,
    pub usri11_last_logon: u32,
    pub usri11_last_logoff: u32,
    pub usri11_bad_pw_count: u32,
    pub usri11_num_logons: u32,
    pub usri11_logon_server: ::windows::core::PWSTR,
    pub usri11_country_code: u32,
    pub usri11_workstations: ::windows::core::PWSTR,
    pub usri11_max_storage: u32,
    pub usri11_units_per_week: u32,
    pub usri11_logon_hours: *mut u8,
    pub usri11_code_page: u32,
}
impl ::core::marker::Copy for USER_INFO_11 {}
impl ::core::clone::Clone for USER_INFO_11 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for USER_INFO_11 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_INFO_11")
            .field("usri11_name", &self.usri11_name)
            .field("usri11_comment", &self.usri11_comment)
            .field("usri11_usr_comment", &self.usri11_usr_comment)
            .field("usri11_full_name", &self.usri11_full_name)
            .field("usri11_priv", &self.usri11_priv)
            .field("usri11_auth_flags", &self.usri11_auth_flags)
            .field("usri11_password_age", &self.usri11_password_age)
            .field("usri11_home_dir", &self.usri11_home_dir)
            .field("usri11_parms", &self.usri11_parms)
            .field("usri11_last_logon", &self.usri11_last_logon)
            .field("usri11_last_logoff", &self.usri11_last_logoff)
            .field("usri11_bad_pw_count", &self.usri11_bad_pw_count)
            .field("usri11_num_logons", &self.usri11_num_logons)
            .field("usri11_logon_server", &self.usri11_logon_server)
            .field("usri11_country_code", &self.usri11_country_code)
            .field("usri11_workstations", &self.usri11_workstations)
            .field("usri11_max_storage", &self.usri11_max_storage)
            .field("usri11_units_per_week", &self.usri11_units_per_week)
            .field("usri11_logon_hours", &self.usri11_logon_hours)
            .field("usri11_code_page", &self.usri11_code_page)
            .finish()
    }
}
impl ::windows::core::TypeKind for USER_INFO_11 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for USER_INFO_11 {
    fn eq(&self, other: &Self) -> bool {
        self.usri11_name == other.usri11_name
            && self.usri11_comment == other.usri11_comment
            && self.usri11_usr_comment == other.usri11_usr_comment
            && self.usri11_full_name == other.usri11_full_name
            && self.usri11_priv == other.usri11_priv
            && self.usri11_auth_flags == other.usri11_auth_flags
            && self.usri11_password_age == other.usri11_password_age
            && self.usri11_home_dir == other.usri11_home_dir
            && self.usri11_parms == other.usri11_parms
            && self.usri11_last_logon == other.usri11_last_logon
            && self.usri11_last_logoff == other.usri11_last_logoff
            && self.usri11_bad_pw_count == other.usri11_bad_pw_count
            && self.usri11_num_logons == other.usri11_num_logons
            && self.usri11_logon_server == other.usri11_logon_server
            && self.usri11_country_code == other.usri11_country_code
            && self.usri11_workstations == other.usri11_workstations
            && self.usri11_max_storage == other.usri11_max_storage
            && self.usri11_units_per_week == other.usri11_units_per_week
            && self.usri11_logon_hours == other.usri11_logon_hours
            && self.usri11_code_page == other.usri11_code_page
    }
}
impl ::core::cmp::Eq for USER_INFO_11 {}
impl ::core::default::Default for USER_INFO_11 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct USER_INFO_2 {
    pub usri2_name: ::windows::core::PWSTR,
    pub usri2_password: ::windows::core::PWSTR,
    pub usri2_password_age: u32,
    pub usri2_priv: USER_PRIV,
    pub usri2_home_dir: ::windows::core::PWSTR,
    pub usri2_comment: ::windows::core::PWSTR,
    pub usri2_flags: USER_ACCOUNT_FLAGS,
    pub usri2_script_path: ::windows::core::PWSTR,
    pub usri2_auth_flags: AF_OP,
    pub usri2_full_name: ::windows::core::PWSTR,
    pub usri2_usr_comment: ::windows::core::PWSTR,
    pub usri2_parms: ::windows::core::PWSTR,
    pub usri2_workstations: ::windows::core::PWSTR,
    pub usri2_last_logon: u32,
    pub usri2_last_logoff: u32,
    pub usri2_acct_expires: u32,
    pub usri2_max_storage: u32,
    pub usri2_units_per_week: u32,
    pub usri2_logon_hours: *mut u8,
    pub usri2_bad_pw_count: u32,
    pub usri2_num_logons: u32,
    pub usri2_logon_server: ::windows::core::PWSTR,
    pub usri2_country_code: u32,
    pub usri2_code_page: u32,
}
impl ::core::marker::Copy for USER_INFO_2 {}
impl ::core::clone::Clone for USER_INFO_2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for USER_INFO_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_INFO_2")
            .field("usri2_name", &self.usri2_name)
            .field("usri2_password", &self.usri2_password)
            .field("usri2_password_age", &self.usri2_password_age)
            .field("usri2_priv", &self.usri2_priv)
            .field("usri2_home_dir", &self.usri2_home_dir)
            .field("usri2_comment", &self.usri2_comment)
            .field("usri2_flags", &self.usri2_flags)
            .field("usri2_script_path", &self.usri2_script_path)
            .field("usri2_auth_flags", &self.usri2_auth_flags)
            .field("usri2_full_name", &self.usri2_full_name)
            .field("usri2_usr_comment", &self.usri2_usr_comment)
            .field("usri2_parms", &self.usri2_parms)
            .field("usri2_workstations", &self.usri2_workstations)
            .field("usri2_last_logon", &self.usri2_last_logon)
            .field("usri2_last_logoff", &self.usri2_last_logoff)
            .field("usri2_acct_expires", &self.usri2_acct_expires)
            .field("usri2_max_storage", &self.usri2_max_storage)
            .field("usri2_units_per_week", &self.usri2_units_per_week)
            .field("usri2_logon_hours", &self.usri2_logon_hours)
            .field("usri2_bad_pw_count", &self.usri2_bad_pw_count)
            .field("usri2_num_logons", &self.usri2_num_logons)
            .field("usri2_logon_server", &self.usri2_logon_server)
            .field("usri2_country_code", &self.usri2_country_code)
            .field("usri2_code_page", &self.usri2_code_page)
            .finish()
    }
}
impl ::windows::core::TypeKind for USER_INFO_2 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for USER_INFO_2 {
    fn eq(&self, other: &Self) -> bool {
        self.usri2_name == other.usri2_name
            && self.usri2_password == other.usri2_password
            && self.usri2_password_age == other.usri2_password_age
            && self.usri2_priv == other.usri2_priv
            && self.usri2_home_dir == other.usri2_home_dir
            && self.usri2_comment == other.usri2_comment
            && self.usri2_flags == other.usri2_flags
            && self.usri2_script_path == other.usri2_script_path
            && self.usri2_auth_flags == other.usri2_auth_flags
            && self.usri2_full_name == other.usri2_full_name
            && self.usri2_usr_comment == other.usri2_usr_comment
            && self.usri2_parms == other.usri2_parms
            && self.usri2_workstations == other.usri2_workstations
            && self.usri2_last_logon == other.usri2_last_logon
            && self.usri2_last_logoff == other.usri2_last_logoff
            && self.usri2_acct_expires == other.usri2_acct_expires
            && self.usri2_max_storage == other.usri2_max_storage
            && self.usri2_units_per_week == other.usri2_units_per_week
            && self.usri2_logon_hours == other.usri2_logon_hours
            && self.usri2_bad_pw_count == other.usri2_bad_pw_count
            && self.usri2_num_logons == other.usri2_num_logons
            && self.usri2_logon_server == other.usri2_logon_server
            && self.usri2_country_code == other.usri2_country_code
            && self.usri2_code_page == other.usri2_code_page
    }
}
impl ::core::cmp::Eq for USER_INFO_2 {}
impl ::core::default::Default for USER_INFO_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct USER_INFO_20 {
    pub usri20_name: ::windows::core::PWSTR,
    pub usri20_full_name: ::windows::core::PWSTR,
    pub usri20_comment: ::windows::core::PWSTR,
    pub usri20_flags: USER_ACCOUNT_FLAGS,
    pub usri20_user_id: u32,
}
impl ::core::marker::Copy for USER_INFO_20 {}
impl ::core::clone::Clone for USER_INFO_20 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for USER_INFO_20 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_INFO_20").field("usri20_name", &self.usri20_name).field("usri20_full_name", &self.usri20_full_name).field("usri20_comment", &self.usri20_comment).field("usri20_flags", &self.usri20_flags).field("usri20_user_id", &self.usri20_user_id).finish()
    }
}
impl ::windows::core::TypeKind for USER_INFO_20 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for USER_INFO_20 {
    fn eq(&self, other: &Self) -> bool {
        self.usri20_name == other.usri20_name && self.usri20_full_name == other.usri20_full_name && self.usri20_comment == other.usri20_comment && self.usri20_flags == other.usri20_flags && self.usri20_user_id == other.usri20_user_id
    }
}
impl ::core::cmp::Eq for USER_INFO_20 {}
impl ::core::default::Default for USER_INFO_20 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct USER_INFO_21 {
    pub usri21_password: [u8; 16],
}
impl ::core::marker::Copy for USER_INFO_21 {}
impl ::core::clone::Clone for USER_INFO_21 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for USER_INFO_21 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_INFO_21").field("usri21_password", &self.usri21_password).finish()
    }
}
impl ::windows::core::TypeKind for USER_INFO_21 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for USER_INFO_21 {
    fn eq(&self, other: &Self) -> bool {
        self.usri21_password == other.usri21_password
    }
}
impl ::core::cmp::Eq for USER_INFO_21 {}
impl ::core::default::Default for USER_INFO_21 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct USER_INFO_22 {
    pub usri22_name: ::windows::core::PWSTR,
    pub usri22_password: [u8; 16],
    pub usri22_password_age: u32,
    pub usri22_priv: USER_PRIV,
    pub usri22_home_dir: ::windows::core::PWSTR,
    pub usri22_comment: ::windows::core::PWSTR,
    pub usri22_flags: USER_ACCOUNT_FLAGS,
    pub usri22_script_path: ::windows::core::PWSTR,
    pub usri22_auth_flags: AF_OP,
    pub usri22_full_name: ::windows::core::PWSTR,
    pub usri22_usr_comment: ::windows::core::PWSTR,
    pub usri22_parms: ::windows::core::PWSTR,
    pub usri22_workstations: ::windows::core::PWSTR,
    pub usri22_last_logon: u32,
    pub usri22_last_logoff: u32,
    pub usri22_acct_expires: u32,
    pub usri22_max_storage: u32,
    pub usri22_units_per_week: u32,
    pub usri22_logon_hours: *mut u8,
    pub usri22_bad_pw_count: u32,
    pub usri22_num_logons: u32,
    pub usri22_logon_server: ::windows::core::PWSTR,
    pub usri22_country_code: u32,
    pub usri22_code_page: u32,
}
impl ::core::marker::Copy for USER_INFO_22 {}
impl ::core::clone::Clone for USER_INFO_22 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for USER_INFO_22 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_INFO_22")
            .field("usri22_name", &self.usri22_name)
            .field("usri22_password", &self.usri22_password)
            .field("usri22_password_age", &self.usri22_password_age)
            .field("usri22_priv", &self.usri22_priv)
            .field("usri22_home_dir", &self.usri22_home_dir)
            .field("usri22_comment", &self.usri22_comment)
            .field("usri22_flags", &self.usri22_flags)
            .field("usri22_script_path", &self.usri22_script_path)
            .field("usri22_auth_flags", &self.usri22_auth_flags)
            .field("usri22_full_name", &self.usri22_full_name)
            .field("usri22_usr_comment", &self.usri22_usr_comment)
            .field("usri22_parms", &self.usri22_parms)
            .field("usri22_workstations", &self.usri22_workstations)
            .field("usri22_last_logon", &self.usri22_last_logon)
            .field("usri22_last_logoff", &self.usri22_last_logoff)
            .field("usri22_acct_expires", &self.usri22_acct_expires)
            .field("usri22_max_storage", &self.usri22_max_storage)
            .field("usri22_units_per_week", &self.usri22_units_per_week)
            .field("usri22_logon_hours", &self.usri22_logon_hours)
            .field("usri22_bad_pw_count", &self.usri22_bad_pw_count)
            .field("usri22_num_logons", &self.usri22_num_logons)
            .field("usri22_logon_server", &self.usri22_logon_server)
            .field("usri22_country_code", &self.usri22_country_code)
            .field("usri22_code_page", &self.usri22_code_page)
            .finish()
    }
}
impl ::windows::core::TypeKind for USER_INFO_22 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for USER_INFO_22 {
    fn eq(&self, other: &Self) -> bool {
        self.usri22_name == other.usri22_name
            && self.usri22_password == other.usri22_password
            && self.usri22_password_age == other.usri22_password_age
            && self.usri22_priv == other.usri22_priv
            && self.usri22_home_dir == other.usri22_home_dir
            && self.usri22_comment == other.usri22_comment
            && self.usri22_flags == other.usri22_flags
            && self.usri22_script_path == other.usri22_script_path
            && self.usri22_auth_flags == other.usri22_auth_flags
            && self.usri22_full_name == other.usri22_full_name
            && self.usri22_usr_comment == other.usri22_usr_comment
            && self.usri22_parms == other.usri22_parms
            && self.usri22_workstations == other.usri22_workstations
            && self.usri22_last_logon == other.usri22_last_logon
            && self.usri22_last_logoff == other.usri22_last_logoff
            && self.usri22_acct_expires == other.usri22_acct_expires
            && self.usri22_max_storage == other.usri22_max_storage
            && self.usri22_units_per_week == other.usri22_units_per_week
            && self.usri22_logon_hours == other.usri22_logon_hours
            && self.usri22_bad_pw_count == other.usri22_bad_pw_count
            && self.usri22_num_logons == other.usri22_num_logons
            && self.usri22_logon_server == other.usri22_logon_server
            && self.usri22_country_code == other.usri22_country_code
            && self.usri22_code_page == other.usri22_code_page
    }
}
impl ::core::cmp::Eq for USER_INFO_22 {}
impl ::core::default::Default for USER_INFO_22 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct USER_INFO_23 {
    pub usri23_name: ::windows::core::PWSTR,
    pub usri23_full_name: ::windows::core::PWSTR,
    pub usri23_comment: ::windows::core::PWSTR,
    pub usri23_flags: USER_ACCOUNT_FLAGS,
    pub usri23_user_sid: super::super::Foundation::PSID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for USER_INFO_23 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for USER_INFO_23 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for USER_INFO_23 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_INFO_23").field("usri23_name", &self.usri23_name).field("usri23_full_name", &self.usri23_full_name).field("usri23_comment", &self.usri23_comment).field("usri23_flags", &self.usri23_flags).field("usri23_user_sid", &self.usri23_user_sid).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for USER_INFO_23 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for USER_INFO_23 {
    fn eq(&self, other: &Self) -> bool {
        self.usri23_name == other.usri23_name && self.usri23_full_name == other.usri23_full_name && self.usri23_comment == other.usri23_comment && self.usri23_flags == other.usri23_flags && self.usri23_user_sid == other.usri23_user_sid
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for USER_INFO_23 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for USER_INFO_23 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct USER_INFO_24 {
    pub usri24_internet_identity: super::super::Foundation::BOOL,
    pub usri24_flags: u32,
    pub usri24_internet_provider_name: ::windows::core::PWSTR,
    pub usri24_internet_principal_name: ::windows::core::PWSTR,
    pub usri24_user_sid: super::super::Foundation::PSID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for USER_INFO_24 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for USER_INFO_24 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for USER_INFO_24 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_INFO_24").field("usri24_internet_identity", &self.usri24_internet_identity).field("usri24_flags", &self.usri24_flags).field("usri24_internet_provider_name", &self.usri24_internet_provider_name).field("usri24_internet_principal_name", &self.usri24_internet_principal_name).field("usri24_user_sid", &self.usri24_user_sid).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for USER_INFO_24 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for USER_INFO_24 {
    fn eq(&self, other: &Self) -> bool {
        self.usri24_internet_identity == other.usri24_internet_identity && self.usri24_flags == other.usri24_flags && self.usri24_internet_provider_name == other.usri24_internet_provider_name && self.usri24_internet_principal_name == other.usri24_internet_principal_name && self.usri24_user_sid == other.usri24_user_sid
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for USER_INFO_24 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for USER_INFO_24 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct USER_INFO_3 {
    pub usri3_name: ::windows::core::PWSTR,
    pub usri3_password: ::windows::core::PWSTR,
    pub usri3_password_age: u32,
    pub usri3_priv: USER_PRIV,
    pub usri3_home_dir: ::windows::core::PWSTR,
    pub usri3_comment: ::windows::core::PWSTR,
    pub usri3_flags: USER_ACCOUNT_FLAGS,
    pub usri3_script_path: ::windows::core::PWSTR,
    pub usri3_auth_flags: AF_OP,
    pub usri3_full_name: ::windows::core::PWSTR,
    pub usri3_usr_comment: ::windows::core::PWSTR,
    pub usri3_parms: ::windows::core::PWSTR,
    pub usri3_workstations: ::windows::core::PWSTR,
    pub usri3_last_logon: u32,
    pub usri3_last_logoff: u32,
    pub usri3_acct_expires: u32,
    pub usri3_max_storage: u32,
    pub usri3_units_per_week: u32,
    pub usri3_logon_hours: *mut u8,
    pub usri3_bad_pw_count: u32,
    pub usri3_num_logons: u32,
    pub usri3_logon_server: ::windows::core::PWSTR,
    pub usri3_country_code: u32,
    pub usri3_code_page: u32,
    pub usri3_user_id: u32,
    pub usri3_primary_group_id: u32,
    pub usri3_profile: ::windows::core::PWSTR,
    pub usri3_home_dir_drive: ::windows::core::PWSTR,
    pub usri3_password_expired: u32,
}
impl ::core::marker::Copy for USER_INFO_3 {}
impl ::core::clone::Clone for USER_INFO_3 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for USER_INFO_3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_INFO_3")
            .field("usri3_name", &self.usri3_name)
            .field("usri3_password", &self.usri3_password)
            .field("usri3_password_age", &self.usri3_password_age)
            .field("usri3_priv", &self.usri3_priv)
            .field("usri3_home_dir", &self.usri3_home_dir)
            .field("usri3_comment", &self.usri3_comment)
            .field("usri3_flags", &self.usri3_flags)
            .field("usri3_script_path", &self.usri3_script_path)
            .field("usri3_auth_flags", &self.usri3_auth_flags)
            .field("usri3_full_name", &self.usri3_full_name)
            .field("usri3_usr_comment", &self.usri3_usr_comment)
            .field("usri3_parms", &self.usri3_parms)
            .field("usri3_workstations", &self.usri3_workstations)
            .field("usri3_last_logon", &self.usri3_last_logon)
            .field("usri3_last_logoff", &self.usri3_last_logoff)
            .field("usri3_acct_expires", &self.usri3_acct_expires)
            .field("usri3_max_storage", &self.usri3_max_storage)
            .field("usri3_units_per_week", &self.usri3_units_per_week)
            .field("usri3_logon_hours", &self.usri3_logon_hours)
            .field("usri3_bad_pw_count", &self.usri3_bad_pw_count)
            .field("usri3_num_logons", &self.usri3_num_logons)
            .field("usri3_logon_server", &self.usri3_logon_server)
            .field("usri3_country_code", &self.usri3_country_code)
            .field("usri3_code_page", &self.usri3_code_page)
            .field("usri3_user_id", &self.usri3_user_id)
            .field("usri3_primary_group_id", &self.usri3_primary_group_id)
            .field("usri3_profile", &self.usri3_profile)
            .field("usri3_home_dir_drive", &self.usri3_home_dir_drive)
            .field("usri3_password_expired", &self.usri3_password_expired)
            .finish()
    }
}
impl ::windows::core::TypeKind for USER_INFO_3 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for USER_INFO_3 {
    fn eq(&self, other: &Self) -> bool {
        self.usri3_name == other.usri3_name
            && self.usri3_password == other.usri3_password
            && self.usri3_password_age == other.usri3_password_age
            && self.usri3_priv == other.usri3_priv
            && self.usri3_home_dir == other.usri3_home_dir
            && self.usri3_comment == other.usri3_comment
            && self.usri3_flags == other.usri3_flags
            && self.usri3_script_path == other.usri3_script_path
            && self.usri3_auth_flags == other.usri3_auth_flags
            && self.usri3_full_name == other.usri3_full_name
            && self.usri3_usr_comment == other.usri3_usr_comment
            && self.usri3_parms == other.usri3_parms
            && self.usri3_workstations == other.usri3_workstations
            && self.usri3_last_logon == other.usri3_last_logon
            && self.usri3_last_logoff == other.usri3_last_logoff
            && self.usri3_acct_expires == other.usri3_acct_expires
            && self.usri3_max_storage == other.usri3_max_storage
            && self.usri3_units_per_week == other.usri3_units_per_week
            && self.usri3_logon_hours == other.usri3_logon_hours
            && self.usri3_bad_pw_count == other.usri3_bad_pw_count
            && self.usri3_num_logons == other.usri3_num_logons
            && self.usri3_logon_server == other.usri3_logon_server
            && self.usri3_country_code == other.usri3_country_code
            && self.usri3_code_page == other.usri3_code_page
            && self.usri3_user_id == other.usri3_user_id
            && self.usri3_primary_group_id == other.usri3_primary_group_id
            && self.usri3_profile == other.usri3_profile
            && self.usri3_home_dir_drive == other.usri3_home_dir_drive
            && self.usri3_password_expired == other.usri3_password_expired
    }
}
impl ::core::cmp::Eq for USER_INFO_3 {}
impl ::core::default::Default for USER_INFO_3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct USER_INFO_4 {
    pub usri4_name: ::windows::core::PWSTR,
    pub usri4_password: ::windows::core::PWSTR,
    pub usri4_password_age: u32,
    pub usri4_priv: USER_PRIV,
    pub usri4_home_dir: ::windows::core::PWSTR,
    pub usri4_comment: ::windows::core::PWSTR,
    pub usri4_flags: USER_ACCOUNT_FLAGS,
    pub usri4_script_path: ::windows::core::PWSTR,
    pub usri4_auth_flags: AF_OP,
    pub usri4_full_name: ::windows::core::PWSTR,
    pub usri4_usr_comment: ::windows::core::PWSTR,
    pub usri4_parms: ::windows::core::PWSTR,
    pub usri4_workstations: ::windows::core::PWSTR,
    pub usri4_last_logon: u32,
    pub usri4_last_logoff: u32,
    pub usri4_acct_expires: u32,
    pub usri4_max_storage: u32,
    pub usri4_units_per_week: u32,
    pub usri4_logon_hours: *mut u8,
    pub usri4_bad_pw_count: u32,
    pub usri4_num_logons: u32,
    pub usri4_logon_server: ::windows::core::PWSTR,
    pub usri4_country_code: u32,
    pub usri4_code_page: u32,
    pub usri4_user_sid: super::super::Foundation::PSID,
    pub usri4_primary_group_id: u32,
    pub usri4_profile: ::windows::core::PWSTR,
    pub usri4_home_dir_drive: ::windows::core::PWSTR,
    pub usri4_password_expired: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for USER_INFO_4 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for USER_INFO_4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for USER_INFO_4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_INFO_4")
            .field("usri4_name", &self.usri4_name)
            .field("usri4_password", &self.usri4_password)
            .field("usri4_password_age", &self.usri4_password_age)
            .field("usri4_priv", &self.usri4_priv)
            .field("usri4_home_dir", &self.usri4_home_dir)
            .field("usri4_comment", &self.usri4_comment)
            .field("usri4_flags", &self.usri4_flags)
            .field("usri4_script_path", &self.usri4_script_path)
            .field("usri4_auth_flags", &self.usri4_auth_flags)
            .field("usri4_full_name", &self.usri4_full_name)
            .field("usri4_usr_comment", &self.usri4_usr_comment)
            .field("usri4_parms", &self.usri4_parms)
            .field("usri4_workstations", &self.usri4_workstations)
            .field("usri4_last_logon", &self.usri4_last_logon)
            .field("usri4_last_logoff", &self.usri4_last_logoff)
            .field("usri4_acct_expires", &self.usri4_acct_expires)
            .field("usri4_max_storage", &self.usri4_max_storage)
            .field("usri4_units_per_week", &self.usri4_units_per_week)
            .field("usri4_logon_hours", &self.usri4_logon_hours)
            .field("usri4_bad_pw_count", &self.usri4_bad_pw_count)
            .field("usri4_num_logons", &self.usri4_num_logons)
            .field("usri4_logon_server", &self.usri4_logon_server)
            .field("usri4_country_code", &self.usri4_country_code)
            .field("usri4_code_page", &self.usri4_code_page)
            .field("usri4_user_sid", &self.usri4_user_sid)
            .field("usri4_primary_group_id", &self.usri4_primary_group_id)
            .field("usri4_profile", &self.usri4_profile)
            .field("usri4_home_dir_drive", &self.usri4_home_dir_drive)
            .field("usri4_password_expired", &self.usri4_password_expired)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for USER_INFO_4 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for USER_INFO_4 {
    fn eq(&self, other: &Self) -> bool {
        self.usri4_name == other.usri4_name
            && self.usri4_password == other.usri4_password
            && self.usri4_password_age == other.usri4_password_age
            && self.usri4_priv == other.usri4_priv
            && self.usri4_home_dir == other.usri4_home_dir
            && self.usri4_comment == other.usri4_comment
            && self.usri4_flags == other.usri4_flags
            && self.usri4_script_path == other.usri4_script_path
            && self.usri4_auth_flags == other.usri4_auth_flags
            && self.usri4_full_name == other.usri4_full_name
            && self.usri4_usr_comment == other.usri4_usr_comment
            && self.usri4_parms == other.usri4_parms
            && self.usri4_workstations == other.usri4_workstations
            && self.usri4_last_logon == other.usri4_last_logon
            && self.usri4_last_logoff == other.usri4_last_logoff
            && self.usri4_acct_expires == other.usri4_acct_expires
            && self.usri4_max_storage == other.usri4_max_storage
            && self.usri4_units_per_week == other.usri4_units_per_week
            && self.usri4_logon_hours == other.usri4_logon_hours
            && self.usri4_bad_pw_count == other.usri4_bad_pw_count
            && self.usri4_num_logons == other.usri4_num_logons
            && self.usri4_logon_server == other.usri4_logon_server
            && self.usri4_country_code == other.usri4_country_code
            && self.usri4_code_page == other.usri4_code_page
            && self.usri4_user_sid == other.usri4_user_sid
            && self.usri4_primary_group_id == other.usri4_primary_group_id
            && self.usri4_profile == other.usri4_profile
            && self.usri4_home_dir_drive == other.usri4_home_dir_drive
            && self.usri4_password_expired == other.usri4_password_expired
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for USER_INFO_4 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for USER_INFO_4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct USER_MODALS_INFO_0 {
    pub usrmod0_min_passwd_len: u32,
    pub usrmod0_max_passwd_age: u32,
    pub usrmod0_min_passwd_age: u32,
    pub usrmod0_force_logoff: u32,
    pub usrmod0_password_hist_len: u32,
}
impl ::core::marker::Copy for USER_MODALS_INFO_0 {}
impl ::core::clone::Clone for USER_MODALS_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for USER_MODALS_INFO_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_MODALS_INFO_0").field("usrmod0_min_passwd_len", &self.usrmod0_min_passwd_len).field("usrmod0_max_passwd_age", &self.usrmod0_max_passwd_age).field("usrmod0_min_passwd_age", &self.usrmod0_min_passwd_age).field("usrmod0_force_logoff", &self.usrmod0_force_logoff).field("usrmod0_password_hist_len", &self.usrmod0_password_hist_len).finish()
    }
}
impl ::windows::core::TypeKind for USER_MODALS_INFO_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for USER_MODALS_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        self.usrmod0_min_passwd_len == other.usrmod0_min_passwd_len && self.usrmod0_max_passwd_age == other.usrmod0_max_passwd_age && self.usrmod0_min_passwd_age == other.usrmod0_min_passwd_age && self.usrmod0_force_logoff == other.usrmod0_force_logoff && self.usrmod0_password_hist_len == other.usrmod0_password_hist_len
    }
}
impl ::core::cmp::Eq for USER_MODALS_INFO_0 {}
impl ::core::default::Default for USER_MODALS_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct USER_MODALS_INFO_1 {
    pub usrmod1_role: u32,
    pub usrmod1_primary: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for USER_MODALS_INFO_1 {}
impl ::core::clone::Clone for USER_MODALS_INFO_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for USER_MODALS_INFO_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_MODALS_INFO_1").field("usrmod1_role", &self.usrmod1_role).field("usrmod1_primary", &self.usrmod1_primary).finish()
    }
}
impl ::windows::core::TypeKind for USER_MODALS_INFO_1 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for USER_MODALS_INFO_1 {
    fn eq(&self, other: &Self) -> bool {
        self.usrmod1_role == other.usrmod1_role && self.usrmod1_primary == other.usrmod1_primary
    }
}
impl ::core::cmp::Eq for USER_MODALS_INFO_1 {}
impl ::core::default::Default for USER_MODALS_INFO_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct USER_MODALS_INFO_1001 {
    pub usrmod1001_min_passwd_len: u32,
}
impl ::core::marker::Copy for USER_MODALS_INFO_1001 {}
impl ::core::clone::Clone for USER_MODALS_INFO_1001 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for USER_MODALS_INFO_1001 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_MODALS_INFO_1001").field("usrmod1001_min_passwd_len", &self.usrmod1001_min_passwd_len).finish()
    }
}
impl ::windows::core::TypeKind for USER_MODALS_INFO_1001 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for USER_MODALS_INFO_1001 {
    fn eq(&self, other: &Self) -> bool {
        self.usrmod1001_min_passwd_len == other.usrmod1001_min_passwd_len
    }
}
impl ::core::cmp::Eq for USER_MODALS_INFO_1001 {}
impl ::core::default::Default for USER_MODALS_INFO_1001 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct USER_MODALS_INFO_1002 {
    pub usrmod1002_max_passwd_age: u32,
}
impl ::core::marker::Copy for USER_MODALS_INFO_1002 {}
impl ::core::clone::Clone for USER_MODALS_INFO_1002 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for USER_MODALS_INFO_1002 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_MODALS_INFO_1002").field("usrmod1002_max_passwd_age", &self.usrmod1002_max_passwd_age).finish()
    }
}
impl ::windows::core::TypeKind for USER_MODALS_INFO_1002 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for USER_MODALS_INFO_1002 {
    fn eq(&self, other: &Self) -> bool {
        self.usrmod1002_max_passwd_age == other.usrmod1002_max_passwd_age
    }
}
impl ::core::cmp::Eq for USER_MODALS_INFO_1002 {}
impl ::core::default::Default for USER_MODALS_INFO_1002 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct USER_MODALS_INFO_1003 {
    pub usrmod1003_min_passwd_age: u32,
}
impl ::core::marker::Copy for USER_MODALS_INFO_1003 {}
impl ::core::clone::Clone for USER_MODALS_INFO_1003 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for USER_MODALS_INFO_1003 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_MODALS_INFO_1003").field("usrmod1003_min_passwd_age", &self.usrmod1003_min_passwd_age).finish()
    }
}
impl ::windows::core::TypeKind for USER_MODALS_INFO_1003 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for USER_MODALS_INFO_1003 {
    fn eq(&self, other: &Self) -> bool {
        self.usrmod1003_min_passwd_age == other.usrmod1003_min_passwd_age
    }
}
impl ::core::cmp::Eq for USER_MODALS_INFO_1003 {}
impl ::core::default::Default for USER_MODALS_INFO_1003 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct USER_MODALS_INFO_1004 {
    pub usrmod1004_force_logoff: u32,
}
impl ::core::marker::Copy for USER_MODALS_INFO_1004 {}
impl ::core::clone::Clone for USER_MODALS_INFO_1004 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for USER_MODALS_INFO_1004 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_MODALS_INFO_1004").field("usrmod1004_force_logoff", &self.usrmod1004_force_logoff).finish()
    }
}
impl ::windows::core::TypeKind for USER_MODALS_INFO_1004 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for USER_MODALS_INFO_1004 {
    fn eq(&self, other: &Self) -> bool {
        self.usrmod1004_force_logoff == other.usrmod1004_force_logoff
    }
}
impl ::core::cmp::Eq for USER_MODALS_INFO_1004 {}
impl ::core::default::Default for USER_MODALS_INFO_1004 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct USER_MODALS_INFO_1005 {
    pub usrmod1005_password_hist_len: u32,
}
impl ::core::marker::Copy for USER_MODALS_INFO_1005 {}
impl ::core::clone::Clone for USER_MODALS_INFO_1005 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for USER_MODALS_INFO_1005 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_MODALS_INFO_1005").field("usrmod1005_password_hist_len", &self.usrmod1005_password_hist_len).finish()
    }
}
impl ::windows::core::TypeKind for USER_MODALS_INFO_1005 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for USER_MODALS_INFO_1005 {
    fn eq(&self, other: &Self) -> bool {
        self.usrmod1005_password_hist_len == other.usrmod1005_password_hist_len
    }
}
impl ::core::cmp::Eq for USER_MODALS_INFO_1005 {}
impl ::core::default::Default for USER_MODALS_INFO_1005 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct USER_MODALS_INFO_1006 {
    pub usrmod1006_role: USER_MODALS_ROLES,
}
impl ::core::marker::Copy for USER_MODALS_INFO_1006 {}
impl ::core::clone::Clone for USER_MODALS_INFO_1006 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for USER_MODALS_INFO_1006 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_MODALS_INFO_1006").field("usrmod1006_role", &self.usrmod1006_role).finish()
    }
}
impl ::windows::core::TypeKind for USER_MODALS_INFO_1006 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for USER_MODALS_INFO_1006 {
    fn eq(&self, other: &Self) -> bool {
        self.usrmod1006_role == other.usrmod1006_role
    }
}
impl ::core::cmp::Eq for USER_MODALS_INFO_1006 {}
impl ::core::default::Default for USER_MODALS_INFO_1006 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct USER_MODALS_INFO_1007 {
    pub usrmod1007_primary: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for USER_MODALS_INFO_1007 {}
impl ::core::clone::Clone for USER_MODALS_INFO_1007 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for USER_MODALS_INFO_1007 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_MODALS_INFO_1007").field("usrmod1007_primary", &self.usrmod1007_primary).finish()
    }
}
impl ::windows::core::TypeKind for USER_MODALS_INFO_1007 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for USER_MODALS_INFO_1007 {
    fn eq(&self, other: &Self) -> bool {
        self.usrmod1007_primary == other.usrmod1007_primary
    }
}
impl ::core::cmp::Eq for USER_MODALS_INFO_1007 {}
impl ::core::default::Default for USER_MODALS_INFO_1007 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct USER_MODALS_INFO_2 {
    pub usrmod2_domain_name: ::windows::core::PWSTR,
    pub usrmod2_domain_id: super::super::Foundation::PSID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for USER_MODALS_INFO_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for USER_MODALS_INFO_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for USER_MODALS_INFO_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_MODALS_INFO_2").field("usrmod2_domain_name", &self.usrmod2_domain_name).field("usrmod2_domain_id", &self.usrmod2_domain_id).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for USER_MODALS_INFO_2 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for USER_MODALS_INFO_2 {
    fn eq(&self, other: &Self) -> bool {
        self.usrmod2_domain_name == other.usrmod2_domain_name && self.usrmod2_domain_id == other.usrmod2_domain_id
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for USER_MODALS_INFO_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for USER_MODALS_INFO_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct USER_MODALS_INFO_3 {
    pub usrmod3_lockout_duration: u32,
    pub usrmod3_lockout_observation_window: u32,
    pub usrmod3_lockout_threshold: u32,
}
impl ::core::marker::Copy for USER_MODALS_INFO_3 {}
impl ::core::clone::Clone for USER_MODALS_INFO_3 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for USER_MODALS_INFO_3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_MODALS_INFO_3").field("usrmod3_lockout_duration", &self.usrmod3_lockout_duration).field("usrmod3_lockout_observation_window", &self.usrmod3_lockout_observation_window).field("usrmod3_lockout_threshold", &self.usrmod3_lockout_threshold).finish()
    }
}
impl ::windows::core::TypeKind for USER_MODALS_INFO_3 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for USER_MODALS_INFO_3 {
    fn eq(&self, other: &Self) -> bool {
        self.usrmod3_lockout_duration == other.usrmod3_lockout_duration && self.usrmod3_lockout_observation_window == other.usrmod3_lockout_observation_window && self.usrmod3_lockout_threshold == other.usrmod3_lockout_threshold
    }
}
impl ::core::cmp::Eq for USER_MODALS_INFO_3 {}
impl ::core::default::Default for USER_MODALS_INFO_3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct USER_OTHER_INFO {
    pub alrtus_errcode: u32,
    pub alrtus_numstrings: u32,
}
impl ::core::marker::Copy for USER_OTHER_INFO {}
impl ::core::clone::Clone for USER_OTHER_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for USER_OTHER_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_OTHER_INFO").field("alrtus_errcode", &self.alrtus_errcode).field("alrtus_numstrings", &self.alrtus_numstrings).finish()
    }
}
impl ::windows::core::TypeKind for USER_OTHER_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for USER_OTHER_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.alrtus_errcode == other.alrtus_errcode && self.alrtus_numstrings == other.alrtus_numstrings
    }
}
impl ::core::cmp::Eq for USER_OTHER_INFO {}
impl ::core::default::Default for USER_OTHER_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct USE_INFO_0 {
    pub ui0_local: ::windows::core::PWSTR,
    pub ui0_remote: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for USE_INFO_0 {}
impl ::core::clone::Clone for USE_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for USE_INFO_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USE_INFO_0").field("ui0_local", &self.ui0_local).field("ui0_remote", &self.ui0_remote).finish()
    }
}
impl ::windows::core::TypeKind for USE_INFO_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for USE_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        self.ui0_local == other.ui0_local && self.ui0_remote == other.ui0_remote
    }
}
impl ::core::cmp::Eq for USE_INFO_0 {}
impl ::core::default::Default for USE_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct USE_INFO_1 {
    pub ui1_local: ::windows::core::PWSTR,
    pub ui1_remote: ::windows::core::PWSTR,
    pub ui1_password: ::windows::core::PWSTR,
    pub ui1_status: u32,
    pub ui1_asg_type: USE_INFO_ASG_TYPE,
    pub ui1_refcount: u32,
    pub ui1_usecount: u32,
}
impl ::core::marker::Copy for USE_INFO_1 {}
impl ::core::clone::Clone for USE_INFO_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for USE_INFO_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USE_INFO_1").field("ui1_local", &self.ui1_local).field("ui1_remote", &self.ui1_remote).field("ui1_password", &self.ui1_password).field("ui1_status", &self.ui1_status).field("ui1_asg_type", &self.ui1_asg_type).field("ui1_refcount", &self.ui1_refcount).field("ui1_usecount", &self.ui1_usecount).finish()
    }
}
impl ::windows::core::TypeKind for USE_INFO_1 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for USE_INFO_1 {
    fn eq(&self, other: &Self) -> bool {
        self.ui1_local == other.ui1_local && self.ui1_remote == other.ui1_remote && self.ui1_password == other.ui1_password && self.ui1_status == other.ui1_status && self.ui1_asg_type == other.ui1_asg_type && self.ui1_refcount == other.ui1_refcount && self.ui1_usecount == other.ui1_usecount
    }
}
impl ::core::cmp::Eq for USE_INFO_1 {}
impl ::core::default::Default for USE_INFO_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct USE_INFO_2 {
    pub ui2_local: ::windows::core::PWSTR,
    pub ui2_remote: ::windows::core::PWSTR,
    pub ui2_password: ::windows::core::PWSTR,
    pub ui2_status: u32,
    pub ui2_asg_type: USE_INFO_ASG_TYPE,
    pub ui2_refcount: u32,
    pub ui2_usecount: u32,
    pub ui2_username: ::windows::core::PWSTR,
    pub ui2_domainname: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for USE_INFO_2 {}
impl ::core::clone::Clone for USE_INFO_2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for USE_INFO_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USE_INFO_2").field("ui2_local", &self.ui2_local).field("ui2_remote", &self.ui2_remote).field("ui2_password", &self.ui2_password).field("ui2_status", &self.ui2_status).field("ui2_asg_type", &self.ui2_asg_type).field("ui2_refcount", &self.ui2_refcount).field("ui2_usecount", &self.ui2_usecount).field("ui2_username", &self.ui2_username).field("ui2_domainname", &self.ui2_domainname).finish()
    }
}
impl ::windows::core::TypeKind for USE_INFO_2 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for USE_INFO_2 {
    fn eq(&self, other: &Self) -> bool {
        self.ui2_local == other.ui2_local && self.ui2_remote == other.ui2_remote && self.ui2_password == other.ui2_password && self.ui2_status == other.ui2_status && self.ui2_asg_type == other.ui2_asg_type && self.ui2_refcount == other.ui2_refcount && self.ui2_usecount == other.ui2_usecount && self.ui2_username == other.ui2_username && self.ui2_domainname == other.ui2_domainname
    }
}
impl ::core::cmp::Eq for USE_INFO_2 {}
impl ::core::default::Default for USE_INFO_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct USE_INFO_3 {
    pub ui3_ui2: USE_INFO_2,
    pub ui3_flags: u32,
}
impl ::core::marker::Copy for USE_INFO_3 {}
impl ::core::clone::Clone for USE_INFO_3 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for USE_INFO_3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USE_INFO_3").field("ui3_ui2", &self.ui3_ui2).field("ui3_flags", &self.ui3_flags).finish()
    }
}
impl ::windows::core::TypeKind for USE_INFO_3 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for USE_INFO_3 {
    fn eq(&self, other: &Self) -> bool {
        self.ui3_ui2 == other.ui3_ui2 && self.ui3_flags == other.ui3_flags
    }
}
impl ::core::cmp::Eq for USE_INFO_3 {}
impl ::core::default::Default for USE_INFO_3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct USE_INFO_4 {
    pub ui4_ui3: USE_INFO_3,
    pub ui4_auth_identity_length: u32,
    pub ui4_auth_identity: *mut u8,
}
impl ::core::marker::Copy for USE_INFO_4 {}
impl ::core::clone::Clone for USE_INFO_4 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for USE_INFO_4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USE_INFO_4").field("ui4_ui3", &self.ui4_ui3).field("ui4_auth_identity_length", &self.ui4_auth_identity_length).field("ui4_auth_identity", &self.ui4_auth_identity).finish()
    }
}
impl ::windows::core::TypeKind for USE_INFO_4 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for USE_INFO_4 {
    fn eq(&self, other: &Self) -> bool {
        self.ui4_ui3 == other.ui4_ui3 && self.ui4_auth_identity_length == other.ui4_auth_identity_length && self.ui4_auth_identity == other.ui4_auth_identity
    }
}
impl ::core::cmp::Eq for USE_INFO_4 {}
impl ::core::default::Default for USE_INFO_4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct USE_INFO_5 {
    pub ui4_ui3: USE_INFO_3,
    pub ui4_auth_identity_length: u32,
    pub ui4_auth_identity: *mut u8,
    pub ui5_security_descriptor_length: u32,
    pub ui5_security_descriptor: *mut u8,
    pub ui5_use_options_length: u32,
    pub ui5_use_options: *mut u8,
}
impl ::core::marker::Copy for USE_INFO_5 {}
impl ::core::clone::Clone for USE_INFO_5 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for USE_INFO_5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USE_INFO_5").field("ui4_ui3", &self.ui4_ui3).field("ui4_auth_identity_length", &self.ui4_auth_identity_length).field("ui4_auth_identity", &self.ui4_auth_identity).field("ui5_security_descriptor_length", &self.ui5_security_descriptor_length).field("ui5_security_descriptor", &self.ui5_security_descriptor).field("ui5_use_options_length", &self.ui5_use_options_length).field("ui5_use_options", &self.ui5_use_options).finish()
    }
}
impl ::windows::core::TypeKind for USE_INFO_5 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for USE_INFO_5 {
    fn eq(&self, other: &Self) -> bool {
        self.ui4_ui3 == other.ui4_ui3 && self.ui4_auth_identity_length == other.ui4_auth_identity_length && self.ui4_auth_identity == other.ui4_auth_identity && self.ui5_security_descriptor_length == other.ui5_security_descriptor_length && self.ui5_security_descriptor == other.ui5_security_descriptor && self.ui5_use_options_length == other.ui5_use_options_length && self.ui5_use_options == other.ui5_use_options
    }
}
impl ::core::cmp::Eq for USE_INFO_5 {}
impl ::core::default::Default for USE_INFO_5 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct USE_OPTION_DEFERRED_CONNECTION_PARAMETERS {
    pub Tag: u32,
    pub Length: u16,
    pub Reserved: u16,
}
impl ::core::marker::Copy for USE_OPTION_DEFERRED_CONNECTION_PARAMETERS {}
impl ::core::clone::Clone for USE_OPTION_DEFERRED_CONNECTION_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for USE_OPTION_DEFERRED_CONNECTION_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USE_OPTION_DEFERRED_CONNECTION_PARAMETERS").field("Tag", &self.Tag).field("Length", &self.Length).field("Reserved", &self.Reserved).finish()
    }
}
impl ::windows::core::TypeKind for USE_OPTION_DEFERRED_CONNECTION_PARAMETERS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for USE_OPTION_DEFERRED_CONNECTION_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Tag == other.Tag && self.Length == other.Length && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for USE_OPTION_DEFERRED_CONNECTION_PARAMETERS {}
impl ::core::default::Default for USE_OPTION_DEFERRED_CONNECTION_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct USE_OPTION_GENERIC {
    pub Tag: u32,
    pub Length: u16,
    pub Reserved: u16,
}
impl ::core::marker::Copy for USE_OPTION_GENERIC {}
impl ::core::clone::Clone for USE_OPTION_GENERIC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for USE_OPTION_GENERIC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USE_OPTION_GENERIC").field("Tag", &self.Tag).field("Length", &self.Length).field("Reserved", &self.Reserved).finish()
    }
}
impl ::windows::core::TypeKind for USE_OPTION_GENERIC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for USE_OPTION_GENERIC {
    fn eq(&self, other: &Self) -> bool {
        self.Tag == other.Tag && self.Length == other.Length && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for USE_OPTION_GENERIC {}
impl ::core::default::Default for USE_OPTION_GENERIC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct USE_OPTION_PROPERTIES {
    pub Tag: u32,
    pub pInfo: *mut ::core::ffi::c_void,
    pub Length: usize,
}
impl ::core::marker::Copy for USE_OPTION_PROPERTIES {}
impl ::core::clone::Clone for USE_OPTION_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for USE_OPTION_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USE_OPTION_PROPERTIES").field("Tag", &self.Tag).field("pInfo", &self.pInfo).field("Length", &self.Length).finish()
    }
}
impl ::windows::core::TypeKind for USE_OPTION_PROPERTIES {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for USE_OPTION_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        self.Tag == other.Tag && self.pInfo == other.pInfo && self.Length == other.Length
    }
}
impl ::core::cmp::Eq for USE_OPTION_PROPERTIES {}
impl ::core::default::Default for USE_OPTION_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct USE_OPTION_TRANSPORT_PARAMETERS {
    pub Tag: u32,
    pub Length: u16,
    pub Reserved: u16,
}
impl ::core::marker::Copy for USE_OPTION_TRANSPORT_PARAMETERS {}
impl ::core::clone::Clone for USE_OPTION_TRANSPORT_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for USE_OPTION_TRANSPORT_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USE_OPTION_TRANSPORT_PARAMETERS").field("Tag", &self.Tag).field("Length", &self.Length).field("Reserved", &self.Reserved).finish()
    }
}
impl ::windows::core::TypeKind for USE_OPTION_TRANSPORT_PARAMETERS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for USE_OPTION_TRANSPORT_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Tag == other.Tag && self.Length == other.Length && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for USE_OPTION_TRANSPORT_PARAMETERS {}
impl ::core::default::Default for USE_OPTION_TRANSPORT_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct WKSTA_INFO_100 {
    pub wki100_platform_id: u32,
    pub wki100_computername: ::windows::core::PWSTR,
    pub wki100_langroup: ::windows::core::PWSTR,
    pub wki100_ver_major: u32,
    pub wki100_ver_minor: u32,
}
impl ::core::marker::Copy for WKSTA_INFO_100 {}
impl ::core::clone::Clone for WKSTA_INFO_100 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WKSTA_INFO_100 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WKSTA_INFO_100").field("wki100_platform_id", &self.wki100_platform_id).field("wki100_computername", &self.wki100_computername).field("wki100_langroup", &self.wki100_langroup).field("wki100_ver_major", &self.wki100_ver_major).field("wki100_ver_minor", &self.wki100_ver_minor).finish()
    }
}
impl ::windows::core::TypeKind for WKSTA_INFO_100 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WKSTA_INFO_100 {
    fn eq(&self, other: &Self) -> bool {
        self.wki100_platform_id == other.wki100_platform_id && self.wki100_computername == other.wki100_computername && self.wki100_langroup == other.wki100_langroup && self.wki100_ver_major == other.wki100_ver_major && self.wki100_ver_minor == other.wki100_ver_minor
    }
}
impl ::core::cmp::Eq for WKSTA_INFO_100 {}
impl ::core::default::Default for WKSTA_INFO_100 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct WKSTA_INFO_101 {
    pub wki101_platform_id: u32,
    pub wki101_computername: ::windows::core::PWSTR,
    pub wki101_langroup: ::windows::core::PWSTR,
    pub wki101_ver_major: u32,
    pub wki101_ver_minor: u32,
    pub wki101_lanroot: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for WKSTA_INFO_101 {}
impl ::core::clone::Clone for WKSTA_INFO_101 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WKSTA_INFO_101 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WKSTA_INFO_101").field("wki101_platform_id", &self.wki101_platform_id).field("wki101_computername", &self.wki101_computername).field("wki101_langroup", &self.wki101_langroup).field("wki101_ver_major", &self.wki101_ver_major).field("wki101_ver_minor", &self.wki101_ver_minor).field("wki101_lanroot", &self.wki101_lanroot).finish()
    }
}
impl ::windows::core::TypeKind for WKSTA_INFO_101 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WKSTA_INFO_101 {
    fn eq(&self, other: &Self) -> bool {
        self.wki101_platform_id == other.wki101_platform_id && self.wki101_computername == other.wki101_computername && self.wki101_langroup == other.wki101_langroup && self.wki101_ver_major == other.wki101_ver_major && self.wki101_ver_minor == other.wki101_ver_minor && self.wki101_lanroot == other.wki101_lanroot
    }
}
impl ::core::cmp::Eq for WKSTA_INFO_101 {}
impl ::core::default::Default for WKSTA_INFO_101 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct WKSTA_INFO_1010 {
    pub wki1010_char_wait: u32,
}
impl ::core::marker::Copy for WKSTA_INFO_1010 {}
impl ::core::clone::Clone for WKSTA_INFO_1010 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WKSTA_INFO_1010 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WKSTA_INFO_1010").field("wki1010_char_wait", &self.wki1010_char_wait).finish()
    }
}
impl ::windows::core::TypeKind for WKSTA_INFO_1010 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WKSTA_INFO_1010 {
    fn eq(&self, other: &Self) -> bool {
        self.wki1010_char_wait == other.wki1010_char_wait
    }
}
impl ::core::cmp::Eq for WKSTA_INFO_1010 {}
impl ::core::default::Default for WKSTA_INFO_1010 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct WKSTA_INFO_1011 {
    pub wki1011_collection_time: u32,
}
impl ::core::marker::Copy for WKSTA_INFO_1011 {}
impl ::core::clone::Clone for WKSTA_INFO_1011 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WKSTA_INFO_1011 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WKSTA_INFO_1011").field("wki1011_collection_time", &self.wki1011_collection_time).finish()
    }
}
impl ::windows::core::TypeKind for WKSTA_INFO_1011 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WKSTA_INFO_1011 {
    fn eq(&self, other: &Self) -> bool {
        self.wki1011_collection_time == other.wki1011_collection_time
    }
}
impl ::core::cmp::Eq for WKSTA_INFO_1011 {}
impl ::core::default::Default for WKSTA_INFO_1011 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct WKSTA_INFO_1012 {
    pub wki1012_maximum_collection_count: u32,
}
impl ::core::marker::Copy for WKSTA_INFO_1012 {}
impl ::core::clone::Clone for WKSTA_INFO_1012 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WKSTA_INFO_1012 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WKSTA_INFO_1012").field("wki1012_maximum_collection_count", &self.wki1012_maximum_collection_count).finish()
    }
}
impl ::windows::core::TypeKind for WKSTA_INFO_1012 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WKSTA_INFO_1012 {
    fn eq(&self, other: &Self) -> bool {
        self.wki1012_maximum_collection_count == other.wki1012_maximum_collection_count
    }
}
impl ::core::cmp::Eq for WKSTA_INFO_1012 {}
impl ::core::default::Default for WKSTA_INFO_1012 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct WKSTA_INFO_1013 {
    pub wki1013_keep_conn: u32,
}
impl ::core::marker::Copy for WKSTA_INFO_1013 {}
impl ::core::clone::Clone for WKSTA_INFO_1013 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WKSTA_INFO_1013 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WKSTA_INFO_1013").field("wki1013_keep_conn", &self.wki1013_keep_conn).finish()
    }
}
impl ::windows::core::TypeKind for WKSTA_INFO_1013 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WKSTA_INFO_1013 {
    fn eq(&self, other: &Self) -> bool {
        self.wki1013_keep_conn == other.wki1013_keep_conn
    }
}
impl ::core::cmp::Eq for WKSTA_INFO_1013 {}
impl ::core::default::Default for WKSTA_INFO_1013 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct WKSTA_INFO_1018 {
    pub wki1018_sess_timeout: u32,
}
impl ::core::marker::Copy for WKSTA_INFO_1018 {}
impl ::core::clone::Clone for WKSTA_INFO_1018 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WKSTA_INFO_1018 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WKSTA_INFO_1018").field("wki1018_sess_timeout", &self.wki1018_sess_timeout).finish()
    }
}
impl ::windows::core::TypeKind for WKSTA_INFO_1018 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WKSTA_INFO_1018 {
    fn eq(&self, other: &Self) -> bool {
        self.wki1018_sess_timeout == other.wki1018_sess_timeout
    }
}
impl ::core::cmp::Eq for WKSTA_INFO_1018 {}
impl ::core::default::Default for WKSTA_INFO_1018 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct WKSTA_INFO_102 {
    pub wki102_platform_id: u32,
    pub wki102_computername: ::windows::core::PWSTR,
    pub wki102_langroup: ::windows::core::PWSTR,
    pub wki102_ver_major: u32,
    pub wki102_ver_minor: u32,
    pub wki102_lanroot: ::windows::core::PWSTR,
    pub wki102_logged_on_users: u32,
}
impl ::core::marker::Copy for WKSTA_INFO_102 {}
impl ::core::clone::Clone for WKSTA_INFO_102 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WKSTA_INFO_102 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WKSTA_INFO_102").field("wki102_platform_id", &self.wki102_platform_id).field("wki102_computername", &self.wki102_computername).field("wki102_langroup", &self.wki102_langroup).field("wki102_ver_major", &self.wki102_ver_major).field("wki102_ver_minor", &self.wki102_ver_minor).field("wki102_lanroot", &self.wki102_lanroot).field("wki102_logged_on_users", &self.wki102_logged_on_users).finish()
    }
}
impl ::windows::core::TypeKind for WKSTA_INFO_102 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WKSTA_INFO_102 {
    fn eq(&self, other: &Self) -> bool {
        self.wki102_platform_id == other.wki102_platform_id && self.wki102_computername == other.wki102_computername && self.wki102_langroup == other.wki102_langroup && self.wki102_ver_major == other.wki102_ver_major && self.wki102_ver_minor == other.wki102_ver_minor && self.wki102_lanroot == other.wki102_lanroot && self.wki102_logged_on_users == other.wki102_logged_on_users
    }
}
impl ::core::cmp::Eq for WKSTA_INFO_102 {}
impl ::core::default::Default for WKSTA_INFO_102 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct WKSTA_INFO_1023 {
    pub wki1023_siz_char_buf: u32,
}
impl ::core::marker::Copy for WKSTA_INFO_1023 {}
impl ::core::clone::Clone for WKSTA_INFO_1023 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WKSTA_INFO_1023 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WKSTA_INFO_1023").field("wki1023_siz_char_buf", &self.wki1023_siz_char_buf).finish()
    }
}
impl ::windows::core::TypeKind for WKSTA_INFO_1023 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WKSTA_INFO_1023 {
    fn eq(&self, other: &Self) -> bool {
        self.wki1023_siz_char_buf == other.wki1023_siz_char_buf
    }
}
impl ::core::cmp::Eq for WKSTA_INFO_1023 {}
impl ::core::default::Default for WKSTA_INFO_1023 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct WKSTA_INFO_1027 {
    pub wki1027_errlog_sz: u32,
}
impl ::core::marker::Copy for WKSTA_INFO_1027 {}
impl ::core::clone::Clone for WKSTA_INFO_1027 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WKSTA_INFO_1027 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WKSTA_INFO_1027").field("wki1027_errlog_sz", &self.wki1027_errlog_sz).finish()
    }
}
impl ::windows::core::TypeKind for WKSTA_INFO_1027 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WKSTA_INFO_1027 {
    fn eq(&self, other: &Self) -> bool {
        self.wki1027_errlog_sz == other.wki1027_errlog_sz
    }
}
impl ::core::cmp::Eq for WKSTA_INFO_1027 {}
impl ::core::default::Default for WKSTA_INFO_1027 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct WKSTA_INFO_1028 {
    pub wki1028_print_buf_time: u32,
}
impl ::core::marker::Copy for WKSTA_INFO_1028 {}
impl ::core::clone::Clone for WKSTA_INFO_1028 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WKSTA_INFO_1028 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WKSTA_INFO_1028").field("wki1028_print_buf_time", &self.wki1028_print_buf_time).finish()
    }
}
impl ::windows::core::TypeKind for WKSTA_INFO_1028 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WKSTA_INFO_1028 {
    fn eq(&self, other: &Self) -> bool {
        self.wki1028_print_buf_time == other.wki1028_print_buf_time
    }
}
impl ::core::cmp::Eq for WKSTA_INFO_1028 {}
impl ::core::default::Default for WKSTA_INFO_1028 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct WKSTA_INFO_1032 {
    pub wki1032_wrk_heuristics: u32,
}
impl ::core::marker::Copy for WKSTA_INFO_1032 {}
impl ::core::clone::Clone for WKSTA_INFO_1032 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WKSTA_INFO_1032 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WKSTA_INFO_1032").field("wki1032_wrk_heuristics", &self.wki1032_wrk_heuristics).finish()
    }
}
impl ::windows::core::TypeKind for WKSTA_INFO_1032 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WKSTA_INFO_1032 {
    fn eq(&self, other: &Self) -> bool {
        self.wki1032_wrk_heuristics == other.wki1032_wrk_heuristics
    }
}
impl ::core::cmp::Eq for WKSTA_INFO_1032 {}
impl ::core::default::Default for WKSTA_INFO_1032 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct WKSTA_INFO_1033 {
    pub wki1033_max_threads: u32,
}
impl ::core::marker::Copy for WKSTA_INFO_1033 {}
impl ::core::clone::Clone for WKSTA_INFO_1033 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WKSTA_INFO_1033 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WKSTA_INFO_1033").field("wki1033_max_threads", &self.wki1033_max_threads).finish()
    }
}
impl ::windows::core::TypeKind for WKSTA_INFO_1033 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WKSTA_INFO_1033 {
    fn eq(&self, other: &Self) -> bool {
        self.wki1033_max_threads == other.wki1033_max_threads
    }
}
impl ::core::cmp::Eq for WKSTA_INFO_1033 {}
impl ::core::default::Default for WKSTA_INFO_1033 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct WKSTA_INFO_1041 {
    pub wki1041_lock_quota: u32,
}
impl ::core::marker::Copy for WKSTA_INFO_1041 {}
impl ::core::clone::Clone for WKSTA_INFO_1041 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WKSTA_INFO_1041 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WKSTA_INFO_1041").field("wki1041_lock_quota", &self.wki1041_lock_quota).finish()
    }
}
impl ::windows::core::TypeKind for WKSTA_INFO_1041 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WKSTA_INFO_1041 {
    fn eq(&self, other: &Self) -> bool {
        self.wki1041_lock_quota == other.wki1041_lock_quota
    }
}
impl ::core::cmp::Eq for WKSTA_INFO_1041 {}
impl ::core::default::Default for WKSTA_INFO_1041 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct WKSTA_INFO_1042 {
    pub wki1042_lock_increment: u32,
}
impl ::core::marker::Copy for WKSTA_INFO_1042 {}
impl ::core::clone::Clone for WKSTA_INFO_1042 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WKSTA_INFO_1042 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WKSTA_INFO_1042").field("wki1042_lock_increment", &self.wki1042_lock_increment).finish()
    }
}
impl ::windows::core::TypeKind for WKSTA_INFO_1042 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WKSTA_INFO_1042 {
    fn eq(&self, other: &Self) -> bool {
        self.wki1042_lock_increment == other.wki1042_lock_increment
    }
}
impl ::core::cmp::Eq for WKSTA_INFO_1042 {}
impl ::core::default::Default for WKSTA_INFO_1042 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct WKSTA_INFO_1043 {
    pub wki1043_lock_maximum: u32,
}
impl ::core::marker::Copy for WKSTA_INFO_1043 {}
impl ::core::clone::Clone for WKSTA_INFO_1043 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WKSTA_INFO_1043 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WKSTA_INFO_1043").field("wki1043_lock_maximum", &self.wki1043_lock_maximum).finish()
    }
}
impl ::windows::core::TypeKind for WKSTA_INFO_1043 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WKSTA_INFO_1043 {
    fn eq(&self, other: &Self) -> bool {
        self.wki1043_lock_maximum == other.wki1043_lock_maximum
    }
}
impl ::core::cmp::Eq for WKSTA_INFO_1043 {}
impl ::core::default::Default for WKSTA_INFO_1043 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct WKSTA_INFO_1044 {
    pub wki1044_pipe_increment: u32,
}
impl ::core::marker::Copy for WKSTA_INFO_1044 {}
impl ::core::clone::Clone for WKSTA_INFO_1044 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WKSTA_INFO_1044 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WKSTA_INFO_1044").field("wki1044_pipe_increment", &self.wki1044_pipe_increment).finish()
    }
}
impl ::windows::core::TypeKind for WKSTA_INFO_1044 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WKSTA_INFO_1044 {
    fn eq(&self, other: &Self) -> bool {
        self.wki1044_pipe_increment == other.wki1044_pipe_increment
    }
}
impl ::core::cmp::Eq for WKSTA_INFO_1044 {}
impl ::core::default::Default for WKSTA_INFO_1044 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct WKSTA_INFO_1045 {
    pub wki1045_pipe_maximum: u32,
}
impl ::core::marker::Copy for WKSTA_INFO_1045 {}
impl ::core::clone::Clone for WKSTA_INFO_1045 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WKSTA_INFO_1045 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WKSTA_INFO_1045").field("wki1045_pipe_maximum", &self.wki1045_pipe_maximum).finish()
    }
}
impl ::windows::core::TypeKind for WKSTA_INFO_1045 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WKSTA_INFO_1045 {
    fn eq(&self, other: &Self) -> bool {
        self.wki1045_pipe_maximum == other.wki1045_pipe_maximum
    }
}
impl ::core::cmp::Eq for WKSTA_INFO_1045 {}
impl ::core::default::Default for WKSTA_INFO_1045 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct WKSTA_INFO_1046 {
    pub wki1046_dormant_file_limit: u32,
}
impl ::core::marker::Copy for WKSTA_INFO_1046 {}
impl ::core::clone::Clone for WKSTA_INFO_1046 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WKSTA_INFO_1046 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WKSTA_INFO_1046").field("wki1046_dormant_file_limit", &self.wki1046_dormant_file_limit).finish()
    }
}
impl ::windows::core::TypeKind for WKSTA_INFO_1046 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WKSTA_INFO_1046 {
    fn eq(&self, other: &Self) -> bool {
        self.wki1046_dormant_file_limit == other.wki1046_dormant_file_limit
    }
}
impl ::core::cmp::Eq for WKSTA_INFO_1046 {}
impl ::core::default::Default for WKSTA_INFO_1046 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct WKSTA_INFO_1047 {
    pub wki1047_cache_file_timeout: u32,
}
impl ::core::marker::Copy for WKSTA_INFO_1047 {}
impl ::core::clone::Clone for WKSTA_INFO_1047 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WKSTA_INFO_1047 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WKSTA_INFO_1047").field("wki1047_cache_file_timeout", &self.wki1047_cache_file_timeout).finish()
    }
}
impl ::windows::core::TypeKind for WKSTA_INFO_1047 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WKSTA_INFO_1047 {
    fn eq(&self, other: &Self) -> bool {
        self.wki1047_cache_file_timeout == other.wki1047_cache_file_timeout
    }
}
impl ::core::cmp::Eq for WKSTA_INFO_1047 {}
impl ::core::default::Default for WKSTA_INFO_1047 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WKSTA_INFO_1048 {
    pub wki1048_use_opportunistic_locking: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WKSTA_INFO_1048 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WKSTA_INFO_1048 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WKSTA_INFO_1048 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WKSTA_INFO_1048").field("wki1048_use_opportunistic_locking", &self.wki1048_use_opportunistic_locking).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WKSTA_INFO_1048 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WKSTA_INFO_1048 {
    fn eq(&self, other: &Self) -> bool {
        self.wki1048_use_opportunistic_locking == other.wki1048_use_opportunistic_locking
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WKSTA_INFO_1048 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WKSTA_INFO_1048 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WKSTA_INFO_1049 {
    pub wki1049_use_unlock_behind: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WKSTA_INFO_1049 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WKSTA_INFO_1049 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WKSTA_INFO_1049 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WKSTA_INFO_1049").field("wki1049_use_unlock_behind", &self.wki1049_use_unlock_behind).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WKSTA_INFO_1049 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WKSTA_INFO_1049 {
    fn eq(&self, other: &Self) -> bool {
        self.wki1049_use_unlock_behind == other.wki1049_use_unlock_behind
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WKSTA_INFO_1049 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WKSTA_INFO_1049 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WKSTA_INFO_1050 {
    pub wki1050_use_close_behind: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WKSTA_INFO_1050 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WKSTA_INFO_1050 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WKSTA_INFO_1050 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WKSTA_INFO_1050").field("wki1050_use_close_behind", &self.wki1050_use_close_behind).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WKSTA_INFO_1050 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WKSTA_INFO_1050 {
    fn eq(&self, other: &Self) -> bool {
        self.wki1050_use_close_behind == other.wki1050_use_close_behind
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WKSTA_INFO_1050 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WKSTA_INFO_1050 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WKSTA_INFO_1051 {
    pub wki1051_buf_named_pipes: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WKSTA_INFO_1051 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WKSTA_INFO_1051 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WKSTA_INFO_1051 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WKSTA_INFO_1051").field("wki1051_buf_named_pipes", &self.wki1051_buf_named_pipes).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WKSTA_INFO_1051 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WKSTA_INFO_1051 {
    fn eq(&self, other: &Self) -> bool {
        self.wki1051_buf_named_pipes == other.wki1051_buf_named_pipes
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WKSTA_INFO_1051 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WKSTA_INFO_1051 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WKSTA_INFO_1052 {
    pub wki1052_use_lock_read_unlock: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WKSTA_INFO_1052 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WKSTA_INFO_1052 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WKSTA_INFO_1052 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WKSTA_INFO_1052").field("wki1052_use_lock_read_unlock", &self.wki1052_use_lock_read_unlock).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WKSTA_INFO_1052 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WKSTA_INFO_1052 {
    fn eq(&self, other: &Self) -> bool {
        self.wki1052_use_lock_read_unlock == other.wki1052_use_lock_read_unlock
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WKSTA_INFO_1052 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WKSTA_INFO_1052 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WKSTA_INFO_1053 {
    pub wki1053_utilize_nt_caching: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WKSTA_INFO_1053 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WKSTA_INFO_1053 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WKSTA_INFO_1053 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WKSTA_INFO_1053").field("wki1053_utilize_nt_caching", &self.wki1053_utilize_nt_caching).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WKSTA_INFO_1053 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WKSTA_INFO_1053 {
    fn eq(&self, other: &Self) -> bool {
        self.wki1053_utilize_nt_caching == other.wki1053_utilize_nt_caching
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WKSTA_INFO_1053 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WKSTA_INFO_1053 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WKSTA_INFO_1054 {
    pub wki1054_use_raw_read: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WKSTA_INFO_1054 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WKSTA_INFO_1054 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WKSTA_INFO_1054 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WKSTA_INFO_1054").field("wki1054_use_raw_read", &self.wki1054_use_raw_read).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WKSTA_INFO_1054 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WKSTA_INFO_1054 {
    fn eq(&self, other: &Self) -> bool {
        self.wki1054_use_raw_read == other.wki1054_use_raw_read
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WKSTA_INFO_1054 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WKSTA_INFO_1054 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WKSTA_INFO_1055 {
    pub wki1055_use_raw_write: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WKSTA_INFO_1055 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WKSTA_INFO_1055 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WKSTA_INFO_1055 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WKSTA_INFO_1055").field("wki1055_use_raw_write", &self.wki1055_use_raw_write).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WKSTA_INFO_1055 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WKSTA_INFO_1055 {
    fn eq(&self, other: &Self) -> bool {
        self.wki1055_use_raw_write == other.wki1055_use_raw_write
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WKSTA_INFO_1055 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WKSTA_INFO_1055 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WKSTA_INFO_1056 {
    pub wki1056_use_write_raw_data: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WKSTA_INFO_1056 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WKSTA_INFO_1056 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WKSTA_INFO_1056 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WKSTA_INFO_1056").field("wki1056_use_write_raw_data", &self.wki1056_use_write_raw_data).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WKSTA_INFO_1056 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WKSTA_INFO_1056 {
    fn eq(&self, other: &Self) -> bool {
        self.wki1056_use_write_raw_data == other.wki1056_use_write_raw_data
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WKSTA_INFO_1056 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WKSTA_INFO_1056 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WKSTA_INFO_1057 {
    pub wki1057_use_encryption: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WKSTA_INFO_1057 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WKSTA_INFO_1057 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WKSTA_INFO_1057 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WKSTA_INFO_1057").field("wki1057_use_encryption", &self.wki1057_use_encryption).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WKSTA_INFO_1057 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WKSTA_INFO_1057 {
    fn eq(&self, other: &Self) -> bool {
        self.wki1057_use_encryption == other.wki1057_use_encryption
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WKSTA_INFO_1057 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WKSTA_INFO_1057 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WKSTA_INFO_1058 {
    pub wki1058_buf_files_deny_write: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WKSTA_INFO_1058 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WKSTA_INFO_1058 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WKSTA_INFO_1058 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WKSTA_INFO_1058").field("wki1058_buf_files_deny_write", &self.wki1058_buf_files_deny_write).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WKSTA_INFO_1058 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WKSTA_INFO_1058 {
    fn eq(&self, other: &Self) -> bool {
        self.wki1058_buf_files_deny_write == other.wki1058_buf_files_deny_write
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WKSTA_INFO_1058 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WKSTA_INFO_1058 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WKSTA_INFO_1059 {
    pub wki1059_buf_read_only_files: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WKSTA_INFO_1059 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WKSTA_INFO_1059 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WKSTA_INFO_1059 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WKSTA_INFO_1059").field("wki1059_buf_read_only_files", &self.wki1059_buf_read_only_files).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WKSTA_INFO_1059 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WKSTA_INFO_1059 {
    fn eq(&self, other: &Self) -> bool {
        self.wki1059_buf_read_only_files == other.wki1059_buf_read_only_files
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WKSTA_INFO_1059 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WKSTA_INFO_1059 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WKSTA_INFO_1060 {
    pub wki1060_force_core_create_mode: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WKSTA_INFO_1060 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WKSTA_INFO_1060 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WKSTA_INFO_1060 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WKSTA_INFO_1060").field("wki1060_force_core_create_mode", &self.wki1060_force_core_create_mode).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WKSTA_INFO_1060 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WKSTA_INFO_1060 {
    fn eq(&self, other: &Self) -> bool {
        self.wki1060_force_core_create_mode == other.wki1060_force_core_create_mode
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WKSTA_INFO_1060 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WKSTA_INFO_1060 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WKSTA_INFO_1061 {
    pub wki1061_use_512_byte_max_transfer: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WKSTA_INFO_1061 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WKSTA_INFO_1061 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WKSTA_INFO_1061 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WKSTA_INFO_1061").field("wki1061_use_512_byte_max_transfer", &self.wki1061_use_512_byte_max_transfer).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WKSTA_INFO_1061 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WKSTA_INFO_1061 {
    fn eq(&self, other: &Self) -> bool {
        self.wki1061_use_512_byte_max_transfer == other.wki1061_use_512_byte_max_transfer
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WKSTA_INFO_1061 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WKSTA_INFO_1061 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct WKSTA_INFO_1062 {
    pub wki1062_read_ahead_throughput: u32,
}
impl ::core::marker::Copy for WKSTA_INFO_1062 {}
impl ::core::clone::Clone for WKSTA_INFO_1062 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WKSTA_INFO_1062 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WKSTA_INFO_1062").field("wki1062_read_ahead_throughput", &self.wki1062_read_ahead_throughput).finish()
    }
}
impl ::windows::core::TypeKind for WKSTA_INFO_1062 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WKSTA_INFO_1062 {
    fn eq(&self, other: &Self) -> bool {
        self.wki1062_read_ahead_throughput == other.wki1062_read_ahead_throughput
    }
}
impl ::core::cmp::Eq for WKSTA_INFO_1062 {}
impl ::core::default::Default for WKSTA_INFO_1062 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct WKSTA_INFO_302 {
    pub wki302_char_wait: u32,
    pub wki302_collection_time: u32,
    pub wki302_maximum_collection_count: u32,
    pub wki302_keep_conn: u32,
    pub wki302_keep_search: u32,
    pub wki302_max_cmds: u32,
    pub wki302_num_work_buf: u32,
    pub wki302_siz_work_buf: u32,
    pub wki302_max_wrk_cache: u32,
    pub wki302_sess_timeout: u32,
    pub wki302_siz_error: u32,
    pub wki302_num_alerts: u32,
    pub wki302_num_services: u32,
    pub wki302_errlog_sz: u32,
    pub wki302_print_buf_time: u32,
    pub wki302_num_char_buf: u32,
    pub wki302_siz_char_buf: u32,
    pub wki302_wrk_heuristics: ::windows::core::PWSTR,
    pub wki302_mailslots: u32,
    pub wki302_num_dgram_buf: u32,
}
impl ::core::marker::Copy for WKSTA_INFO_302 {}
impl ::core::clone::Clone for WKSTA_INFO_302 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WKSTA_INFO_302 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WKSTA_INFO_302")
            .field("wki302_char_wait", &self.wki302_char_wait)
            .field("wki302_collection_time", &self.wki302_collection_time)
            .field("wki302_maximum_collection_count", &self.wki302_maximum_collection_count)
            .field("wki302_keep_conn", &self.wki302_keep_conn)
            .field("wki302_keep_search", &self.wki302_keep_search)
            .field("wki302_max_cmds", &self.wki302_max_cmds)
            .field("wki302_num_work_buf", &self.wki302_num_work_buf)
            .field("wki302_siz_work_buf", &self.wki302_siz_work_buf)
            .field("wki302_max_wrk_cache", &self.wki302_max_wrk_cache)
            .field("wki302_sess_timeout", &self.wki302_sess_timeout)
            .field("wki302_siz_error", &self.wki302_siz_error)
            .field("wki302_num_alerts", &self.wki302_num_alerts)
            .field("wki302_num_services", &self.wki302_num_services)
            .field("wki302_errlog_sz", &self.wki302_errlog_sz)
            .field("wki302_print_buf_time", &self.wki302_print_buf_time)
            .field("wki302_num_char_buf", &self.wki302_num_char_buf)
            .field("wki302_siz_char_buf", &self.wki302_siz_char_buf)
            .field("wki302_wrk_heuristics", &self.wki302_wrk_heuristics)
            .field("wki302_mailslots", &self.wki302_mailslots)
            .field("wki302_num_dgram_buf", &self.wki302_num_dgram_buf)
            .finish()
    }
}
impl ::windows::core::TypeKind for WKSTA_INFO_302 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WKSTA_INFO_302 {
    fn eq(&self, other: &Self) -> bool {
        self.wki302_char_wait == other.wki302_char_wait
            && self.wki302_collection_time == other.wki302_collection_time
            && self.wki302_maximum_collection_count == other.wki302_maximum_collection_count
            && self.wki302_keep_conn == other.wki302_keep_conn
            && self.wki302_keep_search == other.wki302_keep_search
            && self.wki302_max_cmds == other.wki302_max_cmds
            && self.wki302_num_work_buf == other.wki302_num_work_buf
            && self.wki302_siz_work_buf == other.wki302_siz_work_buf
            && self.wki302_max_wrk_cache == other.wki302_max_wrk_cache
            && self.wki302_sess_timeout == other.wki302_sess_timeout
            && self.wki302_siz_error == other.wki302_siz_error
            && self.wki302_num_alerts == other.wki302_num_alerts
            && self.wki302_num_services == other.wki302_num_services
            && self.wki302_errlog_sz == other.wki302_errlog_sz
            && self.wki302_print_buf_time == other.wki302_print_buf_time
            && self.wki302_num_char_buf == other.wki302_num_char_buf
            && self.wki302_siz_char_buf == other.wki302_siz_char_buf
            && self.wki302_wrk_heuristics == other.wki302_wrk_heuristics
            && self.wki302_mailslots == other.wki302_mailslots
            && self.wki302_num_dgram_buf == other.wki302_num_dgram_buf
    }
}
impl ::core::cmp::Eq for WKSTA_INFO_302 {}
impl ::core::default::Default for WKSTA_INFO_302 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct WKSTA_INFO_402 {
    pub wki402_char_wait: u32,
    pub wki402_collection_time: u32,
    pub wki402_maximum_collection_count: u32,
    pub wki402_keep_conn: u32,
    pub wki402_keep_search: u32,
    pub wki402_max_cmds: u32,
    pub wki402_num_work_buf: u32,
    pub wki402_siz_work_buf: u32,
    pub wki402_max_wrk_cache: u32,
    pub wki402_sess_timeout: u32,
    pub wki402_siz_error: u32,
    pub wki402_num_alerts: u32,
    pub wki402_num_services: u32,
    pub wki402_errlog_sz: u32,
    pub wki402_print_buf_time: u32,
    pub wki402_num_char_buf: u32,
    pub wki402_siz_char_buf: u32,
    pub wki402_wrk_heuristics: ::windows::core::PWSTR,
    pub wki402_mailslots: u32,
    pub wki402_num_dgram_buf: u32,
    pub wki402_max_threads: u32,
}
impl ::core::marker::Copy for WKSTA_INFO_402 {}
impl ::core::clone::Clone for WKSTA_INFO_402 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WKSTA_INFO_402 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WKSTA_INFO_402")
            .field("wki402_char_wait", &self.wki402_char_wait)
            .field("wki402_collection_time", &self.wki402_collection_time)
            .field("wki402_maximum_collection_count", &self.wki402_maximum_collection_count)
            .field("wki402_keep_conn", &self.wki402_keep_conn)
            .field("wki402_keep_search", &self.wki402_keep_search)
            .field("wki402_max_cmds", &self.wki402_max_cmds)
            .field("wki402_num_work_buf", &self.wki402_num_work_buf)
            .field("wki402_siz_work_buf", &self.wki402_siz_work_buf)
            .field("wki402_max_wrk_cache", &self.wki402_max_wrk_cache)
            .field("wki402_sess_timeout", &self.wki402_sess_timeout)
            .field("wki402_siz_error", &self.wki402_siz_error)
            .field("wki402_num_alerts", &self.wki402_num_alerts)
            .field("wki402_num_services", &self.wki402_num_services)
            .field("wki402_errlog_sz", &self.wki402_errlog_sz)
            .field("wki402_print_buf_time", &self.wki402_print_buf_time)
            .field("wki402_num_char_buf", &self.wki402_num_char_buf)
            .field("wki402_siz_char_buf", &self.wki402_siz_char_buf)
            .field("wki402_wrk_heuristics", &self.wki402_wrk_heuristics)
            .field("wki402_mailslots", &self.wki402_mailslots)
            .field("wki402_num_dgram_buf", &self.wki402_num_dgram_buf)
            .field("wki402_max_threads", &self.wki402_max_threads)
            .finish()
    }
}
impl ::windows::core::TypeKind for WKSTA_INFO_402 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WKSTA_INFO_402 {
    fn eq(&self, other: &Self) -> bool {
        self.wki402_char_wait == other.wki402_char_wait
            && self.wki402_collection_time == other.wki402_collection_time
            && self.wki402_maximum_collection_count == other.wki402_maximum_collection_count
            && self.wki402_keep_conn == other.wki402_keep_conn
            && self.wki402_keep_search == other.wki402_keep_search
            && self.wki402_max_cmds == other.wki402_max_cmds
            && self.wki402_num_work_buf == other.wki402_num_work_buf
            && self.wki402_siz_work_buf == other.wki402_siz_work_buf
            && self.wki402_max_wrk_cache == other.wki402_max_wrk_cache
            && self.wki402_sess_timeout == other.wki402_sess_timeout
            && self.wki402_siz_error == other.wki402_siz_error
            && self.wki402_num_alerts == other.wki402_num_alerts
            && self.wki402_num_services == other.wki402_num_services
            && self.wki402_errlog_sz == other.wki402_errlog_sz
            && self.wki402_print_buf_time == other.wki402_print_buf_time
            && self.wki402_num_char_buf == other.wki402_num_char_buf
            && self.wki402_siz_char_buf == other.wki402_siz_char_buf
            && self.wki402_wrk_heuristics == other.wki402_wrk_heuristics
            && self.wki402_mailslots == other.wki402_mailslots
            && self.wki402_num_dgram_buf == other.wki402_num_dgram_buf
            && self.wki402_max_threads == other.wki402_max_threads
    }
}
impl ::core::cmp::Eq for WKSTA_INFO_402 {}
impl ::core::default::Default for WKSTA_INFO_402 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WKSTA_INFO_502 {
    pub wki502_char_wait: u32,
    pub wki502_collection_time: u32,
    pub wki502_maximum_collection_count: u32,
    pub wki502_keep_conn: u32,
    pub wki502_max_cmds: u32,
    pub wki502_sess_timeout: u32,
    pub wki502_siz_char_buf: u32,
    pub wki502_max_threads: u32,
    pub wki502_lock_quota: u32,
    pub wki502_lock_increment: u32,
    pub wki502_lock_maximum: u32,
    pub wki502_pipe_increment: u32,
    pub wki502_pipe_maximum: u32,
    pub wki502_cache_file_timeout: u32,
    pub wki502_dormant_file_limit: u32,
    pub wki502_read_ahead_throughput: u32,
    pub wki502_num_mailslot_buffers: u32,
    pub wki502_num_srv_announce_buffers: u32,
    pub wki502_max_illegal_datagram_events: u32,
    pub wki502_illegal_datagram_event_reset_frequency: u32,
    pub wki502_log_election_packets: super::super::Foundation::BOOL,
    pub wki502_use_opportunistic_locking: super::super::Foundation::BOOL,
    pub wki502_use_unlock_behind: super::super::Foundation::BOOL,
    pub wki502_use_close_behind: super::super::Foundation::BOOL,
    pub wki502_buf_named_pipes: super::super::Foundation::BOOL,
    pub wki502_use_lock_read_unlock: super::super::Foundation::BOOL,
    pub wki502_utilize_nt_caching: super::super::Foundation::BOOL,
    pub wki502_use_raw_read: super::super::Foundation::BOOL,
    pub wki502_use_raw_write: super::super::Foundation::BOOL,
    pub wki502_use_write_raw_data: super::super::Foundation::BOOL,
    pub wki502_use_encryption: super::super::Foundation::BOOL,
    pub wki502_buf_files_deny_write: super::super::Foundation::BOOL,
    pub wki502_buf_read_only_files: super::super::Foundation::BOOL,
    pub wki502_force_core_create_mode: super::super::Foundation::BOOL,
    pub wki502_use_512_byte_max_transfer: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WKSTA_INFO_502 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WKSTA_INFO_502 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WKSTA_INFO_502 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WKSTA_INFO_502")
            .field("wki502_char_wait", &self.wki502_char_wait)
            .field("wki502_collection_time", &self.wki502_collection_time)
            .field("wki502_maximum_collection_count", &self.wki502_maximum_collection_count)
            .field("wki502_keep_conn", &self.wki502_keep_conn)
            .field("wki502_max_cmds", &self.wki502_max_cmds)
            .field("wki502_sess_timeout", &self.wki502_sess_timeout)
            .field("wki502_siz_char_buf", &self.wki502_siz_char_buf)
            .field("wki502_max_threads", &self.wki502_max_threads)
            .field("wki502_lock_quota", &self.wki502_lock_quota)
            .field("wki502_lock_increment", &self.wki502_lock_increment)
            .field("wki502_lock_maximum", &self.wki502_lock_maximum)
            .field("wki502_pipe_increment", &self.wki502_pipe_increment)
            .field("wki502_pipe_maximum", &self.wki502_pipe_maximum)
            .field("wki502_cache_file_timeout", &self.wki502_cache_file_timeout)
            .field("wki502_dormant_file_limit", &self.wki502_dormant_file_limit)
            .field("wki502_read_ahead_throughput", &self.wki502_read_ahead_throughput)
            .field("wki502_num_mailslot_buffers", &self.wki502_num_mailslot_buffers)
            .field("wki502_num_srv_announce_buffers", &self.wki502_num_srv_announce_buffers)
            .field("wki502_max_illegal_datagram_events", &self.wki502_max_illegal_datagram_events)
            .field("wki502_illegal_datagram_event_reset_frequency", &self.wki502_illegal_datagram_event_reset_frequency)
            .field("wki502_log_election_packets", &self.wki502_log_election_packets)
            .field("wki502_use_opportunistic_locking", &self.wki502_use_opportunistic_locking)
            .field("wki502_use_unlock_behind", &self.wki502_use_unlock_behind)
            .field("wki502_use_close_behind", &self.wki502_use_close_behind)
            .field("wki502_buf_named_pipes", &self.wki502_buf_named_pipes)
            .field("wki502_use_lock_read_unlock", &self.wki502_use_lock_read_unlock)
            .field("wki502_utilize_nt_caching", &self.wki502_utilize_nt_caching)
            .field("wki502_use_raw_read", &self.wki502_use_raw_read)
            .field("wki502_use_raw_write", &self.wki502_use_raw_write)
            .field("wki502_use_write_raw_data", &self.wki502_use_write_raw_data)
            .field("wki502_use_encryption", &self.wki502_use_encryption)
            .field("wki502_buf_files_deny_write", &self.wki502_buf_files_deny_write)
            .field("wki502_buf_read_only_files", &self.wki502_buf_read_only_files)
            .field("wki502_force_core_create_mode", &self.wki502_force_core_create_mode)
            .field("wki502_use_512_byte_max_transfer", &self.wki502_use_512_byte_max_transfer)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WKSTA_INFO_502 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WKSTA_INFO_502 {
    fn eq(&self, other: &Self) -> bool {
        self.wki502_char_wait == other.wki502_char_wait
            && self.wki502_collection_time == other.wki502_collection_time
            && self.wki502_maximum_collection_count == other.wki502_maximum_collection_count
            && self.wki502_keep_conn == other.wki502_keep_conn
            && self.wki502_max_cmds == other.wki502_max_cmds
            && self.wki502_sess_timeout == other.wki502_sess_timeout
            && self.wki502_siz_char_buf == other.wki502_siz_char_buf
            && self.wki502_max_threads == other.wki502_max_threads
            && self.wki502_lock_quota == other.wki502_lock_quota
            && self.wki502_lock_increment == other.wki502_lock_increment
            && self.wki502_lock_maximum == other.wki502_lock_maximum
            && self.wki502_pipe_increment == other.wki502_pipe_increment
            && self.wki502_pipe_maximum == other.wki502_pipe_maximum
            && self.wki502_cache_file_timeout == other.wki502_cache_file_timeout
            && self.wki502_dormant_file_limit == other.wki502_dormant_file_limit
            && self.wki502_read_ahead_throughput == other.wki502_read_ahead_throughput
            && self.wki502_num_mailslot_buffers == other.wki502_num_mailslot_buffers
            && self.wki502_num_srv_announce_buffers == other.wki502_num_srv_announce_buffers
            && self.wki502_max_illegal_datagram_events == other.wki502_max_illegal_datagram_events
            && self.wki502_illegal_datagram_event_reset_frequency == other.wki502_illegal_datagram_event_reset_frequency
            && self.wki502_log_election_packets == other.wki502_log_election_packets
            && self.wki502_use_opportunistic_locking == other.wki502_use_opportunistic_locking
            && self.wki502_use_unlock_behind == other.wki502_use_unlock_behind
            && self.wki502_use_close_behind == other.wki502_use_close_behind
            && self.wki502_buf_named_pipes == other.wki502_buf_named_pipes
            && self.wki502_use_lock_read_unlock == other.wki502_use_lock_read_unlock
            && self.wki502_utilize_nt_caching == other.wki502_utilize_nt_caching
            && self.wki502_use_raw_read == other.wki502_use_raw_read
            && self.wki502_use_raw_write == other.wki502_use_raw_write
            && self.wki502_use_write_raw_data == other.wki502_use_write_raw_data
            && self.wki502_use_encryption == other.wki502_use_encryption
            && self.wki502_buf_files_deny_write == other.wki502_buf_files_deny_write
            && self.wki502_buf_read_only_files == other.wki502_buf_read_only_files
            && self.wki502_force_core_create_mode == other.wki502_force_core_create_mode
            && self.wki502_use_512_byte_max_transfer == other.wki502_use_512_byte_max_transfer
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WKSTA_INFO_502 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WKSTA_INFO_502 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WKSTA_TRANSPORT_INFO_0 {
    pub wkti0_quality_of_service: u32,
    pub wkti0_number_of_vcs: u32,
    pub wkti0_transport_name: ::windows::core::PWSTR,
    pub wkti0_transport_address: ::windows::core::PWSTR,
    pub wkti0_wan_ish: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WKSTA_TRANSPORT_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WKSTA_TRANSPORT_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WKSTA_TRANSPORT_INFO_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WKSTA_TRANSPORT_INFO_0").field("wkti0_quality_of_service", &self.wkti0_quality_of_service).field("wkti0_number_of_vcs", &self.wkti0_number_of_vcs).field("wkti0_transport_name", &self.wkti0_transport_name).field("wkti0_transport_address", &self.wkti0_transport_address).field("wkti0_wan_ish", &self.wkti0_wan_ish).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WKSTA_TRANSPORT_INFO_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WKSTA_TRANSPORT_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        self.wkti0_quality_of_service == other.wkti0_quality_of_service && self.wkti0_number_of_vcs == other.wkti0_number_of_vcs && self.wkti0_transport_name == other.wkti0_transport_name && self.wkti0_transport_address == other.wkti0_transport_address && self.wkti0_wan_ish == other.wkti0_wan_ish
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WKSTA_TRANSPORT_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WKSTA_TRANSPORT_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct WKSTA_USER_INFO_0 {
    pub wkui0_username: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for WKSTA_USER_INFO_0 {}
impl ::core::clone::Clone for WKSTA_USER_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WKSTA_USER_INFO_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WKSTA_USER_INFO_0").field("wkui0_username", &self.wkui0_username).finish()
    }
}
impl ::windows::core::TypeKind for WKSTA_USER_INFO_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WKSTA_USER_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        self.wkui0_username == other.wkui0_username
    }
}
impl ::core::cmp::Eq for WKSTA_USER_INFO_0 {}
impl ::core::default::Default for WKSTA_USER_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct WKSTA_USER_INFO_1 {
    pub wkui1_username: ::windows::core::PWSTR,
    pub wkui1_logon_domain: ::windows::core::PWSTR,
    pub wkui1_oth_domains: ::windows::core::PWSTR,
    pub wkui1_logon_server: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for WKSTA_USER_INFO_1 {}
impl ::core::clone::Clone for WKSTA_USER_INFO_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WKSTA_USER_INFO_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WKSTA_USER_INFO_1").field("wkui1_username", &self.wkui1_username).field("wkui1_logon_domain", &self.wkui1_logon_domain).field("wkui1_oth_domains", &self.wkui1_oth_domains).field("wkui1_logon_server", &self.wkui1_logon_server).finish()
    }
}
impl ::windows::core::TypeKind for WKSTA_USER_INFO_1 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WKSTA_USER_INFO_1 {
    fn eq(&self, other: &Self) -> bool {
        self.wkui1_username == other.wkui1_username && self.wkui1_logon_domain == other.wkui1_logon_domain && self.wkui1_oth_domains == other.wkui1_oth_domains && self.wkui1_logon_server == other.wkui1_logon_server
    }
}
impl ::core::cmp::Eq for WKSTA_USER_INFO_1 {}
impl ::core::default::Default for WKSTA_USER_INFO_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub struct WKSTA_USER_INFO_1101 {
    pub wkui1101_oth_domains: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for WKSTA_USER_INFO_1101 {}
impl ::core::clone::Clone for WKSTA_USER_INFO_1101 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WKSTA_USER_INFO_1101 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WKSTA_USER_INFO_1101").field("wkui1101_oth_domains", &self.wkui1101_oth_domains).finish()
    }
}
impl ::windows::core::TypeKind for WKSTA_USER_INFO_1101 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WKSTA_USER_INFO_1101 {
    fn eq(&self, other: &Self) -> bool {
        self.wkui1101_oth_domains == other.wkui1101_oth_domains
    }
}
impl ::core::cmp::Eq for WKSTA_USER_INFO_1101 {}
impl ::core::default::Default for WKSTA_USER_INFO_1101 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`*"]
pub type WORKERFUNCTION = ::core::option::Option<unsafe extern "system" fn(param0: *mut ::core::ffi::c_void) -> ()>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
