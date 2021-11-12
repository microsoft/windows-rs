#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNetScheduleAccountInformation(pwszservername: super::super::Foundation::PWSTR, ccaccount: u32, wszaccount: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn I_NetLogonControl2(servername: super::super::Foundation::PWSTR, functioncode: u32, querylevel: u32, data: *const u8, buffer: *mut *mut u8) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn LogErrorA(dwmessageid: u32, cnumberofsubstrings: u32, plpwssubstrings: *const super::super::Foundation::PSTR, dwerrorcode: u32);
    #[cfg(feature = "Win32_Foundation")]
    pub fn LogErrorW(dwmessageid: u32, cnumberofsubstrings: u32, plpwssubstrings: *const super::super::Foundation::PWSTR, dwerrorcode: u32);
    #[cfg(feature = "Win32_Foundation")]
    pub fn LogEventA(weventtype: u32, dwmessageid: u32, cnumberofsubstrings: u32, plpwssubstrings: *const super::super::Foundation::PSTR);
    #[cfg(feature = "Win32_Foundation")]
    pub fn LogEventW(weventtype: u32, dwmessageid: u32, cnumberofsubstrings: u32, plpwssubstrings: *const super::super::Foundation::PWSTR);
    pub fn MprSetupProtocolEnum(dwtransportid: u32, lplpbuffer: *mut *mut u8, lpdwentriesread: *mut u32) -> u32;
    pub fn MprSetupProtocolFree(lpbuffer: *mut ::core::ffi::c_void) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetAccessAdd(servername: super::super::Foundation::PWSTR, level: u32, buf: *const u8, parm_err: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetAccessDel(servername: super::super::Foundation::PWSTR, resource: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetAccessEnum(servername: super::super::Foundation::PWSTR, basepath: super::super::Foundation::PWSTR, recursive: u32, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resume_handle: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetAccessGetInfo(servername: super::super::Foundation::PWSTR, resource: super::super::Foundation::PWSTR, level: u32, bufptr: *mut *mut u8) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetAccessGetUserPerms(servername: super::super::Foundation::PWSTR, ugname: super::super::Foundation::PWSTR, resource: super::super::Foundation::PWSTR, perms: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetAccessSetInfo(servername: super::super::Foundation::PWSTR, resource: super::super::Foundation::PWSTR, level: u32, buf: *const u8, parm_err: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetAddAlternateComputerName(server: super::super::Foundation::PWSTR, alternatename: super::super::Foundation::PWSTR, domainaccount: super::super::Foundation::PWSTR, domainaccountpassword: super::super::Foundation::PWSTR, reserved: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetAddServiceAccount(servername: super::super::Foundation::PWSTR, accountname: super::super::Foundation::PWSTR, password: super::super::Foundation::PWSTR, flags: u32) -> super::super::Foundation::NTSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetAlertRaise(alerttype: super::super::Foundation::PWSTR, buffer: *const ::core::ffi::c_void, buffersize: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetAlertRaiseEx(alerttype: super::super::Foundation::PWSTR, variableinfo: *const ::core::ffi::c_void, variableinfosize: u32, servicename: super::super::Foundation::PWSTR) -> u32;
    pub fn NetApiBufferAllocate(bytecount: u32, buffer: *mut *mut ::core::ffi::c_void) -> u32;
    pub fn NetApiBufferFree(buffer: *const ::core::ffi::c_void) -> u32;
    pub fn NetApiBufferReallocate(oldbuffer: *const ::core::ffi::c_void, newbytecount: u32, newbuffer: *mut *mut ::core::ffi::c_void) -> u32;
    pub fn NetApiBufferSize(buffer: *const ::core::ffi::c_void, bytecount: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetAuditClear(server: super::super::Foundation::PWSTR, backupfile: super::super::Foundation::PWSTR, service: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetAuditRead(server: super::super::Foundation::PWSTR, service: super::super::Foundation::PWSTR, auditloghandle: *mut HLOG, offset: u32, reserved1: *mut u32, reserved2: u32, offsetflag: u32, bufptr: *mut *mut u8, prefmaxlen: u32, bytesread: *mut u32, totalavailable: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetAuditWrite(r#type: u32, buf: *mut u8, numbytes: u32, service: super::super::Foundation::PWSTR, reserved: *mut u8) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetConfigGet(server: super::super::Foundation::PWSTR, component: super::super::Foundation::PWSTR, parameter: super::super::Foundation::PWSTR, bufptr: *mut *mut u8) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetConfigGetAll(server: super::super::Foundation::PWSTR, component: super::super::Foundation::PWSTR, bufptr: *mut *mut u8) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetConfigSet(server: super::super::Foundation::PWSTR, reserved1: super::super::Foundation::PWSTR, component: super::super::Foundation::PWSTR, level: u32, reserved2: u32, buf: *mut u8, reserved3: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetCreateProvisioningPackage(pprovisioningparams: *const NETSETUP_PROVISIONING_PARAMS, pppackagebindata: *mut *mut u8, pdwpackagebindatasize: *mut u32, pppackagetextdata: *mut super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetEnumerateComputerNames(server: super::super::Foundation::PWSTR, nametype: NET_COMPUTER_NAME_TYPE, reserved: u32, entrycount: *mut u32, computernames: *mut *mut super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetEnumerateServiceAccounts(servername: super::super::Foundation::PWSTR, flags: u32, accountscount: *mut u32, accounts: *mut *mut *mut u16) -> super::super::Foundation::NTSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetErrorLogClear(uncservername: super::super::Foundation::PWSTR, backupfile: super::super::Foundation::PWSTR, reserved: *const u8) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetErrorLogRead(uncservername: super::super::Foundation::PWSTR, reserved1: super::super::Foundation::PWSTR, errorloghandle: *const HLOG, offset: u32, reserved2: *const u32, reserved3: u32, offsetflag: u32, bufptr: *mut *mut u8, prefmaxsize: u32, bytesread: *mut u32, totalavailable: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetErrorLogWrite(reserved1: *const u8, code: u32, component: super::super::Foundation::PWSTR, buffer: *const u8, numbytes: u32, msgbuf: *const u8, strcount: u32, reserved2: *const u8) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub fn NetFreeAadJoinInformation(pjoininfo: *const DSREG_JOIN_INFO);
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub fn NetGetAadJoinInformation(pcsztenantid: super::super::Foundation::PWSTR, ppjoininfo: *mut *mut DSREG_JOIN_INFO) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetGetAnyDCName(servername: super::super::Foundation::PWSTR, domainname: super::super::Foundation::PWSTR, buffer: *mut *mut u8) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetGetDCName(servername: super::super::Foundation::PWSTR, domainname: super::super::Foundation::PWSTR, buffer: *mut *mut u8) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetGetDisplayInformationIndex(servername: super::super::Foundation::PWSTR, level: u32, prefix: super::super::Foundation::PWSTR, index: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetGetJoinInformation(lpserver: super::super::Foundation::PWSTR, lpnamebuffer: *mut super::super::Foundation::PWSTR, buffertype: *mut NETSETUP_JOIN_STATUS) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetGetJoinableOUs(lpserver: super::super::Foundation::PWSTR, lpdomain: super::super::Foundation::PWSTR, lpaccount: super::super::Foundation::PWSTR, lppassword: super::super::Foundation::PWSTR, oucount: *mut u32, ous: *mut *mut super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetGroupAdd(servername: super::super::Foundation::PWSTR, level: u32, buf: *const u8, parm_err: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetGroupAddUser(servername: super::super::Foundation::PWSTR, groupname: super::super::Foundation::PWSTR, username: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetGroupDel(servername: super::super::Foundation::PWSTR, groupname: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetGroupDelUser(servername: super::super::Foundation::PWSTR, groupname: super::super::Foundation::PWSTR, username: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetGroupEnum(servername: super::super::Foundation::PWSTR, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resume_handle: *mut usize) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetGroupGetInfo(servername: super::super::Foundation::PWSTR, groupname: super::super::Foundation::PWSTR, level: u32, bufptr: *mut *mut u8) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetGroupGetUsers(servername: super::super::Foundation::PWSTR, groupname: super::super::Foundation::PWSTR, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resumehandle: *mut usize) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetGroupSetInfo(servername: super::super::Foundation::PWSTR, groupname: super::super::Foundation::PWSTR, level: u32, buf: *const u8, parm_err: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetGroupSetUsers(servername: super::super::Foundation::PWSTR, groupname: super::super::Foundation::PWSTR, level: u32, buf: *const u8, totalentries: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetIsServiceAccount(servername: super::super::Foundation::PWSTR, accountname: super::super::Foundation::PWSTR, isservice: *mut super::super::Foundation::BOOL) -> super::super::Foundation::NTSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetJoinDomain(lpserver: super::super::Foundation::PWSTR, lpdomain: super::super::Foundation::PWSTR, lpmachineaccountou: super::super::Foundation::PWSTR, lpaccount: super::super::Foundation::PWSTR, lppassword: super::super::Foundation::PWSTR, fjoinoptions: NET_JOIN_DOMAIN_JOIN_OPTIONS) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetLocalGroupAdd(servername: super::super::Foundation::PWSTR, level: u32, buf: *const u8, parm_err: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetLocalGroupAddMember(servername: super::super::Foundation::PWSTR, groupname: super::super::Foundation::PWSTR, membersid: super::super::Foundation::PSID) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetLocalGroupAddMembers(servername: super::super::Foundation::PWSTR, groupname: super::super::Foundation::PWSTR, level: u32, buf: *const u8, totalentries: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetLocalGroupDel(servername: super::super::Foundation::PWSTR, groupname: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetLocalGroupDelMember(servername: super::super::Foundation::PWSTR, groupname: super::super::Foundation::PWSTR, membersid: super::super::Foundation::PSID) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetLocalGroupDelMembers(servername: super::super::Foundation::PWSTR, groupname: super::super::Foundation::PWSTR, level: u32, buf: *const u8, totalentries: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetLocalGroupEnum(servername: super::super::Foundation::PWSTR, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resumehandle: *mut usize) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetLocalGroupGetInfo(servername: super::super::Foundation::PWSTR, groupname: super::super::Foundation::PWSTR, level: u32, bufptr: *mut *mut u8) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetLocalGroupGetMembers(servername: super::super::Foundation::PWSTR, localgroupname: super::super::Foundation::PWSTR, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resumehandle: *mut usize) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetLocalGroupSetInfo(servername: super::super::Foundation::PWSTR, groupname: super::super::Foundation::PWSTR, level: u32, buf: *const u8, parm_err: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetLocalGroupSetMembers(servername: super::super::Foundation::PWSTR, groupname: super::super::Foundation::PWSTR, level: u32, buf: *const u8, totalentries: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetMessageBufferSend(servername: super::super::Foundation::PWSTR, msgname: super::super::Foundation::PWSTR, fromname: super::super::Foundation::PWSTR, buf: *const u8, buflen: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetMessageNameAdd(servername: super::super::Foundation::PWSTR, msgname: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetMessageNameDel(servername: super::super::Foundation::PWSTR, msgname: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetMessageNameEnum(servername: super::super::Foundation::PWSTR, level: u32, bufptr: *const *const u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resume_handle: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetMessageNameGetInfo(servername: super::super::Foundation::PWSTR, msgname: super::super::Foundation::PWSTR, level: u32, bufptr: *const *const u8) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetProvisionComputerAccount(lpdomain: super::super::Foundation::PWSTR, lpmachinename: super::super::Foundation::PWSTR, lpmachineaccountou: super::super::Foundation::PWSTR, lpdcname: super::super::Foundation::PWSTR, dwoptions: NETSETUP_PROVISION, pprovisionbindata: *mut *mut u8, pdwprovisionbindatasize: *mut u32, pprovisiontextdata: *mut super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetQueryDisplayInformation(servername: super::super::Foundation::PWSTR, level: u32, index: u32, entriesrequested: u32, preferredmaximumlength: u32, returnedentrycount: *mut u32, sortedbuffer: *mut *mut ::core::ffi::c_void) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetQueryServiceAccount(servername: super::super::Foundation::PWSTR, accountname: super::super::Foundation::PWSTR, infolevel: u32, buffer: *mut *mut u8) -> super::super::Foundation::NTSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetRemoteComputerSupports(uncservername: super::super::Foundation::PWSTR, optionswanted: NET_REMOTE_COMPUTER_SUPPORTS_OPTIONS, optionssupported: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetRemoteTOD(uncservername: super::super::Foundation::PWSTR, bufferptr: *mut *mut u8) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetRemoveAlternateComputerName(server: super::super::Foundation::PWSTR, alternatename: super::super::Foundation::PWSTR, domainaccount: super::super::Foundation::PWSTR, domainaccountpassword: super::super::Foundation::PWSTR, reserved: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetRemoveServiceAccount(servername: super::super::Foundation::PWSTR, accountname: super::super::Foundation::PWSTR, flags: u32) -> super::super::Foundation::NTSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetRenameMachineInDomain(lpserver: super::super::Foundation::PWSTR, lpnewmachinename: super::super::Foundation::PWSTR, lpaccount: super::super::Foundation::PWSTR, lppassword: super::super::Foundation::PWSTR, frenameoptions: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetReplExportDirAdd(servername: super::super::Foundation::PWSTR, level: u32, buf: *const u8, parm_err: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetReplExportDirDel(servername: super::super::Foundation::PWSTR, dirname: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetReplExportDirEnum(servername: super::super::Foundation::PWSTR, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resumehandle: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetReplExportDirGetInfo(servername: super::super::Foundation::PWSTR, dirname: super::super::Foundation::PWSTR, level: u32, bufptr: *mut *mut u8) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetReplExportDirLock(servername: super::super::Foundation::PWSTR, dirname: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetReplExportDirSetInfo(servername: super::super::Foundation::PWSTR, dirname: super::super::Foundation::PWSTR, level: u32, buf: *const u8, parm_err: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetReplExportDirUnlock(servername: super::super::Foundation::PWSTR, dirname: super::super::Foundation::PWSTR, unlockforce: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetReplGetInfo(servername: super::super::Foundation::PWSTR, level: u32, bufptr: *mut *mut u8) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetReplImportDirAdd(servername: super::super::Foundation::PWSTR, level: u32, buf: *const u8, parm_err: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetReplImportDirDel(servername: super::super::Foundation::PWSTR, dirname: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetReplImportDirEnum(servername: super::super::Foundation::PWSTR, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resumehandle: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetReplImportDirGetInfo(servername: super::super::Foundation::PWSTR, dirname: super::super::Foundation::PWSTR, level: u32, bufptr: *mut *mut u8) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetReplImportDirLock(servername: super::super::Foundation::PWSTR, dirname: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetReplImportDirUnlock(servername: super::super::Foundation::PWSTR, dirname: super::super::Foundation::PWSTR, unlockforce: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetReplSetInfo(servername: super::super::Foundation::PWSTR, level: u32, buf: *const u8, parm_err: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetRequestOfflineDomainJoin(pprovisionbindata: *const u8, cbprovisionbindatasize: u32, dwoptions: NET_REQUEST_PROVISION_OPTIONS, lpwindowspath: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetRequestProvisioningPackageInstall(ppackagebindata: *const u8, dwpackagebindatasize: u32, dwprovisionoptions: NET_REQUEST_PROVISION_OPTIONS, lpwindowspath: super::super::Foundation::PWSTR, pvreserved: *mut ::core::ffi::c_void) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetScheduleJobAdd(servername: super::super::Foundation::PWSTR, buffer: *mut u8, jobid: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetScheduleJobDel(servername: super::super::Foundation::PWSTR, minjobid: u32, maxjobid: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetScheduleJobEnum(servername: super::super::Foundation::PWSTR, pointertobuffer: *mut *mut u8, prefferedmaximumlength: u32, entriesread: *mut u32, totalentries: *mut u32, resumehandle: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetScheduleJobGetInfo(servername: super::super::Foundation::PWSTR, jobid: u32, pointertobuffer: *mut *mut u8) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetServerComputerNameAdd(servername: super::super::Foundation::PWSTR, emulateddomainname: super::super::Foundation::PWSTR, emulatedservername: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetServerComputerNameDel(servername: super::super::Foundation::PWSTR, emulatedservername: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetServerDiskEnum(servername: super::super::Foundation::PWSTR, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resume_handle: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetServerEnum(servername: super::super::Foundation::PWSTR, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, servertype: NET_SERVER_TYPE, domain: super::super::Foundation::PWSTR, resume_handle: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetServerGetInfo(servername: super::super::Foundation::PWSTR, level: u32, bufptr: *mut *mut u8) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetServerSetInfo(servername: super::super::Foundation::PWSTR, level: u32, buf: *const u8, parmerror: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetServerTransportAdd(servername: super::super::Foundation::PWSTR, level: u32, bufptr: *const u8) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetServerTransportAddEx(servername: super::super::Foundation::PWSTR, level: u32, bufptr: *const u8) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetServerTransportDel(servername: super::super::Foundation::PWSTR, level: u32, bufptr: *const u8) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetServerTransportEnum(servername: super::super::Foundation::PWSTR, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resume_handle: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetServiceControl(servername: super::super::Foundation::PWSTR, service: super::super::Foundation::PWSTR, opcode: u32, arg: u32, bufptr: *mut *mut u8) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetServiceEnum(servername: super::super::Foundation::PWSTR, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resume_handle: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetServiceGetInfo(servername: super::super::Foundation::PWSTR, service: super::super::Foundation::PWSTR, level: u32, bufptr: *mut *mut u8) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetServiceInstall(servername: super::super::Foundation::PWSTR, service: super::super::Foundation::PWSTR, argc: u32, argv: *const super::super::Foundation::PWSTR, bufptr: *mut *mut u8) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetSetPrimaryComputerName(server: super::super::Foundation::PWSTR, primaryname: super::super::Foundation::PWSTR, domainaccount: super::super::Foundation::PWSTR, domainaccountpassword: super::super::Foundation::PWSTR, reserved: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetUnjoinDomain(lpserver: super::super::Foundation::PWSTR, lpaccount: super::super::Foundation::PWSTR, lppassword: super::super::Foundation::PWSTR, funjoinoptions: u32) -> u32;
    pub fn NetUseAdd(servername: *const i8, levelflags: u32, buf: *const u8, parm_err: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetUseDel(uncservername: super::super::Foundation::PWSTR, usename: super::super::Foundation::PWSTR, forcelevelflags: FORCE_LEVEL_FLAGS) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetUseEnum(uncservername: super::super::Foundation::PWSTR, levelflags: u32, bufptr: *mut *mut u8, preferedmaximumsize: u32, entriesread: *mut u32, totalentries: *mut u32, resumehandle: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetUseGetInfo(uncservername: super::super::Foundation::PWSTR, usename: super::super::Foundation::PWSTR, levelflags: u32, bufptr: *mut *mut u8) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetUserAdd(servername: super::super::Foundation::PWSTR, level: u32, buf: *const u8, parm_err: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetUserChangePassword(domainname: super::super::Foundation::PWSTR, username: super::super::Foundation::PWSTR, oldpassword: super::super::Foundation::PWSTR, newpassword: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetUserDel(servername: super::super::Foundation::PWSTR, username: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetUserEnum(servername: super::super::Foundation::PWSTR, level: u32, filter: NET_USER_ENUM_FILTER_FLAGS, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resume_handle: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetUserGetGroups(servername: super::super::Foundation::PWSTR, username: super::super::Foundation::PWSTR, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetUserGetInfo(servername: super::super::Foundation::PWSTR, username: super::super::Foundation::PWSTR, level: u32, bufptr: *mut *mut u8) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetUserGetLocalGroups(servername: super::super::Foundation::PWSTR, username: super::super::Foundation::PWSTR, level: u32, flags: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetUserModalsGet(servername: super::super::Foundation::PWSTR, level: u32, bufptr: *mut *mut u8) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetUserModalsSet(servername: super::super::Foundation::PWSTR, level: u32, buf: *const u8, parm_err: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetUserSetGroups(servername: super::super::Foundation::PWSTR, username: super::super::Foundation::PWSTR, level: u32, buf: *const u8, num_entries: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetUserSetInfo(servername: super::super::Foundation::PWSTR, username: super::super::Foundation::PWSTR, level: u32, buf: *const u8, parm_err: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetValidateName(lpserver: super::super::Foundation::PWSTR, lpname: super::super::Foundation::PWSTR, lpaccount: super::super::Foundation::PWSTR, lppassword: super::super::Foundation::PWSTR, nametype: NETSETUP_NAME_TYPE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetValidatePasswordPolicy(servername: super::super::Foundation::PWSTR, qualifier: *mut ::core::ffi::c_void, validationtype: NET_VALIDATE_PASSWORD_TYPE, inputarg: *mut ::core::ffi::c_void, outputarg: *mut *mut ::core::ffi::c_void) -> u32;
    pub fn NetValidatePasswordPolicyFree(outputarg: *mut *mut ::core::ffi::c_void) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetWkstaGetInfo(servername: super::super::Foundation::PWSTR, level: u32, bufptr: *mut *mut u8) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetWkstaSetInfo(servername: super::super::Foundation::PWSTR, level: u32, buffer: *const u8, parm_err: *mut u32) -> u32;
    pub fn NetWkstaTransportAdd(servername: *const i8, level: u32, buf: *const u8, parm_err: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetWkstaTransportDel(servername: super::super::Foundation::PWSTR, transportname: super::super::Foundation::PWSTR, ucond: FORCE_LEVEL_FLAGS) -> u32;
    pub fn NetWkstaTransportEnum(servername: *const i8, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resume_handle: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetWkstaUserEnum(servername: super::super::Foundation::PWSTR, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resumehandle: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetWkstaUserGetInfo(reserved: super::super::Foundation::PWSTR, level: u32, bufptr: *mut *mut u8) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetWkstaUserSetInfo(reserved: super::super::Foundation::PWSTR, level: u32, buf: *const u8, parm_err: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RouterAssert(pszfailedassertion: super::super::Foundation::PSTR, pszfilename: super::super::Foundation::PSTR, dwlinenumber: u32, pszmessage: super::super::Foundation::PSTR);
    #[cfg(feature = "Win32_Foundation")]
    pub fn RouterGetErrorStringA(dwerrorcode: u32, lplpszerrorstring: *mut super::super::Foundation::PSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RouterGetErrorStringW(dwerrorcode: u32, lplpwszerrorstring: *mut super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RouterLogDeregisterA(hloghandle: super::super::Foundation::HANDLE);
    #[cfg(feature = "Win32_Foundation")]
    pub fn RouterLogDeregisterW(hloghandle: super::super::Foundation::HANDLE);
    #[cfg(feature = "Win32_Foundation")]
    pub fn RouterLogEventA(hloghandle: super::super::Foundation::HANDLE, dweventtype: u32, dwmessageid: u32, dwsubstringcount: u32, plpszsubstringarray: *const super::super::Foundation::PSTR, dwerrorcode: u32);
    #[cfg(feature = "Win32_Foundation")]
    pub fn RouterLogEventDataA(hloghandle: super::super::Foundation::HANDLE, dweventtype: u32, dwmessageid: u32, dwsubstringcount: u32, plpszsubstringarray: *const super::super::Foundation::PSTR, dwdatabytes: u32, lpdatabytes: *mut u8);
    #[cfg(feature = "Win32_Foundation")]
    pub fn RouterLogEventDataW(hloghandle: super::super::Foundation::HANDLE, dweventtype: u32, dwmessageid: u32, dwsubstringcount: u32, plpszsubstringarray: *const super::super::Foundation::PWSTR, dwdatabytes: u32, lpdatabytes: *mut u8);
    #[cfg(feature = "Win32_Foundation")]
    pub fn RouterLogEventExA(hloghandle: super::super::Foundation::HANDLE, dweventtype: u32, dwerrorcode: u32, dwmessageid: u32, ptszformat: super::super::Foundation::PSTR);
    #[cfg(feature = "Win32_Foundation")]
    pub fn RouterLogEventExW(hloghandle: super::super::Foundation::HANDLE, dweventtype: u32, dwerrorcode: u32, dwmessageid: u32, ptszformat: super::super::Foundation::PWSTR);
    #[cfg(feature = "Win32_Foundation")]
    pub fn RouterLogEventStringA(hloghandle: super::super::Foundation::HANDLE, dweventtype: u32, dwmessageid: u32, dwsubstringcount: u32, plpszsubstringarray: *const super::super::Foundation::PSTR, dwerrorcode: u32, dwerrorindex: u32);
    #[cfg(feature = "Win32_Foundation")]
    pub fn RouterLogEventStringW(hloghandle: super::super::Foundation::HANDLE, dweventtype: u32, dwmessageid: u32, dwsubstringcount: u32, plpszsubstringarray: *const super::super::Foundation::PWSTR, dwerrorcode: u32, dwerrorindex: u32);
    #[cfg(feature = "Win32_Foundation")]
    pub fn RouterLogEventValistExA(hloghandle: super::super::Foundation::HANDLE, dweventtype: u32, dwerrorcode: u32, dwmessageid: u32, ptszformat: super::super::Foundation::PSTR, arglist: *mut i8);
    #[cfg(feature = "Win32_Foundation")]
    pub fn RouterLogEventValistExW(hloghandle: super::super::Foundation::HANDLE, dweventtype: u32, dwerrorcode: u32, dwmessageid: u32, ptszformat: super::super::Foundation::PWSTR, arglist: *mut i8);
    #[cfg(feature = "Win32_Foundation")]
    pub fn RouterLogEventW(hloghandle: super::super::Foundation::HANDLE, dweventtype: u32, dwmessageid: u32, dwsubstringcount: u32, plpszsubstringarray: *const super::super::Foundation::PWSTR, dwerrorcode: u32);
    #[cfg(feature = "Win32_Foundation")]
    pub fn RouterLogRegisterA(lpszsource: super::super::Foundation::PSTR) -> super::super::Foundation::HANDLE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RouterLogRegisterW(lpszsource: super::super::Foundation::PWSTR) -> super::super::Foundation::HANDLE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetNetScheduleAccountInformation(pwszservername: super::super::Foundation::PWSTR, pwszaccount: super::super::Foundation::PWSTR, pwszpassword: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    pub fn TraceDeregisterA(dwtraceid: u32) -> u32;
    pub fn TraceDeregisterExA(dwtraceid: u32, dwflags: u32) -> u32;
    pub fn TraceDeregisterExW(dwtraceid: u32, dwflags: u32) -> u32;
    pub fn TraceDeregisterW(dwtraceid: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn TraceDumpExA(dwtraceid: u32, dwflags: u32, lpbbytes: *mut u8, dwbytecount: u32, dwgroupsize: u32, baddressprefix: super::super::Foundation::BOOL, lpszprefix: super::super::Foundation::PSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn TraceDumpExW(dwtraceid: u32, dwflags: u32, lpbbytes: *mut u8, dwbytecount: u32, dwgroupsize: u32, baddressprefix: super::super::Foundation::BOOL, lpszprefix: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn TraceGetConsoleA(dwtraceid: u32, lphconsole: *mut super::super::Foundation::HANDLE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn TraceGetConsoleW(dwtraceid: u32, lphconsole: *mut super::super::Foundation::HANDLE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn TracePrintfA(dwtraceid: u32, lpszformat: super::super::Foundation::PSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn TracePrintfExA(dwtraceid: u32, dwflags: u32, lpszformat: super::super::Foundation::PSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn TracePrintfExW(dwtraceid: u32, dwflags: u32, lpszformat: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn TracePrintfW(dwtraceid: u32, lpszformat: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn TracePutsExA(dwtraceid: u32, dwflags: u32, lpszstring: super::super::Foundation::PSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn TracePutsExW(dwtraceid: u32, dwflags: u32, lpszstring: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn TraceRegisterExA(lpszcallername: super::super::Foundation::PSTR, dwflags: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn TraceRegisterExW(lpszcallername: super::super::Foundation::PWSTR, dwflags: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn TraceVprintfExA(dwtraceid: u32, dwflags: u32, lpszformat: super::super::Foundation::PSTR, arglist: *mut i8) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn TraceVprintfExW(dwtraceid: u32, dwflags: u32, lpszformat: super::super::Foundation::PWSTR, arglist: *mut i8) -> u32;
}
pub const AA_AUDIT_ALL: u32 = 1u32;
pub const AA_A_ACL: u32 = 32768u32;
pub const AA_A_CREATE: u32 = 8192u32;
pub const AA_A_DELETE: u32 = 16384u32;
pub const AA_A_OPEN: u32 = 4096u32;
pub const AA_A_OWNER: u32 = 4u32;
pub const AA_A_WRITE: u32 = 8192u32;
pub const AA_CLOSE: u32 = 8u32;
pub const AA_F_ACL: u32 = 2048u32;
pub const AA_F_CREATE: u32 = 512u32;
pub const AA_F_DELETE: u32 = 1024u32;
pub const AA_F_OPEN: u32 = 256u32;
pub const AA_F_WRITE: u32 = 512u32;
pub const AA_S_ACL: u32 = 128u32;
pub const AA_S_CREATE: u32 = 32u32;
pub const AA_S_DELETE: u32 = 64u32;
pub const AA_S_OPEN: u32 = 16u32;
pub const AA_S_WRITE: u32 = 32u32;
pub const ACCESS_ACCESS_LIST_PARMNUM: u32 = 4u32;
pub const ACCESS_ATTR_PARMNUM: u32 = 2u32;
pub const ACCESS_AUDIT: u32 = 1u32;
pub const ACCESS_COUNT_PARMNUM: u32 = 3u32;
pub const ACCESS_FAIL_ACL: u32 = 2048u32;
pub const ACCESS_FAIL_DELETE: u32 = 1024u32;
pub const ACCESS_FAIL_MASK: u32 = 3840u32;
pub const ACCESS_FAIL_OPEN: u32 = 256u32;
pub const ACCESS_FAIL_SHIFT: u32 = 4u32;
pub const ACCESS_FAIL_WRITE: u32 = 512u32;
pub const ACCESS_GROUP: u32 = 32768u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct ACCESS_INFO_0(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct ACCESS_INFO_1(i32);
#[repr(C)]
pub struct ACCESS_INFO_1002(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct ACCESS_LIST(i32);
pub const ACCESS_NONE: u32 = 0u32;
pub const ACCESS_RESOURCE_NAME_PARMNUM: u32 = 1u32;
pub const ACCESS_SUCCESS_ACL: u32 = 128u32;
pub const ACCESS_SUCCESS_DELETE: u32 = 64u32;
pub const ACCESS_SUCCESS_MASK: u32 = 240u32;
pub const ACCESS_SUCCESS_OPEN: u32 = 16u32;
pub const ACCESS_SUCCESS_WRITE: u32 = 32u32;
pub const ACTION_ADMINUNLOCK: u32 = 1u32;
pub const ACTION_LOCKOUT: u32 = 0u32;
#[repr(C)]
pub struct ADMIN_OTHER_INFO(i32);
#[repr(C)]
pub struct AE_ACCLIM(i32);
pub const AE_ACCLIMITEXCD: u32 = 17u32;
pub const AE_ACCRESTRICT: u32 = 4u32;
pub const AE_ACLMOD: u32 = 12u32;
#[repr(C)]
pub struct AE_ACLMOD(i32);
pub const AE_ACLMODFAIL: u32 = 19u32;
pub const AE_ADD: u32 = 2u32;
pub const AE_ADMIN: u32 = 2u32;
pub const AE_ADMINDIS: u32 = 3u32;
pub const AE_ADMINPRIVREQD: u32 = 2u32;
pub const AE_ADMIN_CLOSE: u32 = 2u32;
pub const AE_AUTODIS: u32 = 2u32;
pub const AE_BADPW: u32 = 1u32;
pub const AE_CLOSEFILE: u32 = 9u32;
#[repr(C)]
pub struct AE_CLOSEFILE(i32);
pub const AE_CONNREJ: u32 = 6u32;
#[repr(C)]
pub struct AE_CONNREJ(i32);
pub const AE_CONNSTART: u32 = 4u32;
#[repr(C)]
pub struct AE_CONNSTART(i32);
pub const AE_CONNSTOP: u32 = 5u32;
#[repr(C)]
pub struct AE_CONNSTOP(i32);
pub const AE_DELETE: u32 = 1u32;
pub const AE_ERROR: u32 = 1u32;
pub const AE_GENERAL: u32 = 0u32;
#[repr(C)]
pub struct AE_GENERIC(i32);
pub const AE_GENERIC_TYPE: u32 = 21u32;
pub const AE_GUEST: u32 = 0u32;
pub const AE_LIM_DELETED: u32 = 5u32;
pub const AE_LIM_DISABLED: u32 = 4u32;
pub const AE_LIM_EXPIRED: u32 = 2u32;
pub const AE_LIM_INVAL_WKSTA: u32 = 3u32;
pub const AE_LIM_LOGONHOURS: u32 = 1u32;
pub const AE_LIM_UNKNOWN: u32 = 0u32;
pub const AE_LOCKOUT: u32 = 20u32;
#[repr(C)]
pub struct AE_LOCKOUT(i32);
pub const AE_MOD: u32 = 0u32;
pub const AE_NETLOGDENIED: u32 = 16u32;
pub const AE_NETLOGOFF: u32 = 15u32;
#[repr(C)]
pub struct AE_NETLOGOFF(i32);
pub const AE_NETLOGON: u32 = 14u32;
#[repr(C)]
pub struct AE_NETLOGON(i32);
pub const AE_NOACCESSPERM: u32 = 3u32;
pub const AE_NORMAL: u32 = 0u32;
pub const AE_NORMAL_CLOSE: u32 = 0u32;
pub const AE_RESACCESS: u32 = 7u32;
#[repr(C)]
pub struct AE_RESACCESS(i32);
pub const AE_RESACCESS2: u32 = 18u32;
pub const AE_RESACCESSREJ: u32 = 8u32;
#[repr(C)]
pub struct AE_RESACCESSREJ(i32);
pub const AE_SERVICESTAT: u32 = 11u32;
#[repr(C)]
pub struct AE_SERVICESTAT(i32);
pub const AE_SESSDIS: u32 = 1u32;
pub const AE_SESSLOGOFF: u32 = 2u32;
#[repr(C)]
pub struct AE_SESSLOGOFF(i32);
pub const AE_SESSLOGON: u32 = 1u32;
#[repr(C)]
pub struct AE_SESSLOGON(i32);
pub const AE_SESSPWERR: u32 = 3u32;
#[repr(C)]
pub struct AE_SESSPWERR(i32);
pub const AE_SES_CLOSE: u32 = 1u32;
pub const AE_SRVCONT: u32 = 2u32;
pub const AE_SRVPAUSED: u32 = 1u32;
pub const AE_SRVSTART: u32 = 0u32;
pub const AE_SRVSTATUS: u32 = 0u32;
#[repr(C)]
pub struct AE_SRVSTATUS(i32);
pub const AE_SRVSTOP: u32 = 3u32;
pub const AE_UASMOD: u32 = 13u32;
#[repr(C)]
pub struct AE_UASMOD(i32);
pub const AE_UAS_GROUP: u32 = 1u32;
pub const AE_UAS_MODALS: u32 = 2u32;
pub const AE_UAS_USER: u32 = 0u32;
pub const AE_UNSHARE: u32 = 2u32;
pub const AE_USER: u32 = 1u32;
pub const AE_USERLIMIT: u32 = 0u32;
#[repr(transparent)]
pub struct AF_OP(pub u32);
pub const AF_OP_PRINT: AF_OP = AF_OP(1u32);
pub const AF_OP_COMM: AF_OP = AF_OP(2u32);
pub const AF_OP_SERVER: AF_OP = AF_OP(4u32);
pub const AF_OP_ACCOUNTS: AF_OP = AF_OP(8u32);
pub const ALERTSZ: u32 = 128u32;
pub const ALIGN_SIZE: u32 = 8u32;
pub const ALLOCATE_RESPONSE: u32 = 2u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct AT_ENUM(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct AT_INFO(i32);
#[repr(C)]
pub struct AUDIT_ENTRY(i32);
#[repr(transparent)]
pub struct BIND_FLAGS1(pub i32);
pub const NCN_ADD: BIND_FLAGS1 = BIND_FLAGS1(1i32);
pub const NCN_REMOVE: BIND_FLAGS1 = BIND_FLAGS1(2i32);
pub const NCN_UPDATE: BIND_FLAGS1 = BIND_FLAGS1(4i32);
pub const NCN_ENABLE: BIND_FLAGS1 = BIND_FLAGS1(16i32);
pub const NCN_DISABLE: BIND_FLAGS1 = BIND_FLAGS1(32i32);
pub const NCN_BINDING_PATH: BIND_FLAGS1 = BIND_FLAGS1(256i32);
pub const NCN_PROPERTYCHANGE: BIND_FLAGS1 = BIND_FLAGS1(512i32);
pub const NCN_NET: BIND_FLAGS1 = BIND_FLAGS1(65536i32);
pub const NCN_NETTRANS: BIND_FLAGS1 = BIND_FLAGS1(131072i32);
pub const NCN_NETCLIENT: BIND_FLAGS1 = BIND_FLAGS1(262144i32);
pub const NCN_NETSERVICE: BIND_FLAGS1 = BIND_FLAGS1(524288i32);
pub const CLTYPE_LEN: u32 = 12u32;
pub const CNLEN: u32 = 15u32;
#[repr(transparent)]
pub struct COMPONENT_CHARACTERISTICS(pub i32);
pub const NCF_VIRTUAL: COMPONENT_CHARACTERISTICS = COMPONENT_CHARACTERISTICS(1i32);
pub const NCF_SOFTWARE_ENUMERATED: COMPONENT_CHARACTERISTICS = COMPONENT_CHARACTERISTICS(2i32);
pub const NCF_PHYSICAL: COMPONENT_CHARACTERISTICS = COMPONENT_CHARACTERISTICS(4i32);
pub const NCF_HIDDEN: COMPONENT_CHARACTERISTICS = COMPONENT_CHARACTERISTICS(8i32);
pub const NCF_NO_SERVICE: COMPONENT_CHARACTERISTICS = COMPONENT_CHARACTERISTICS(16i32);
pub const NCF_NOT_USER_REMOVABLE: COMPONENT_CHARACTERISTICS = COMPONENT_CHARACTERISTICS(32i32);
pub const NCF_MULTIPORT_INSTANCED_ADAPTER: COMPONENT_CHARACTERISTICS = COMPONENT_CHARACTERISTICS(64i32);
pub const NCF_HAS_UI: COMPONENT_CHARACTERISTICS = COMPONENT_CHARACTERISTICS(128i32);
pub const NCF_SINGLE_INSTANCE: COMPONENT_CHARACTERISTICS = COMPONENT_CHARACTERISTICS(256i32);
pub const NCF_FILTER: COMPONENT_CHARACTERISTICS = COMPONENT_CHARACTERISTICS(1024i32);
pub const NCF_DONTEXPOSELOWER: COMPONENT_CHARACTERISTICS = COMPONENT_CHARACTERISTICS(4096i32);
pub const NCF_HIDE_BINDING: COMPONENT_CHARACTERISTICS = COMPONENT_CHARACTERISTICS(8192i32);
pub const NCF_NDIS_PROTOCOL: COMPONENT_CHARACTERISTICS = COMPONENT_CHARACTERISTICS(16384i32);
pub const NCF_FIXED_BINDING: COMPONENT_CHARACTERISTICS = COMPONENT_CHARACTERISTICS(131072i32);
pub const NCF_LW_FILTER: COMPONENT_CHARACTERISTICS = COMPONENT_CHARACTERISTICS(262144i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct CONFIG_INFO_0(i32);
pub const COULD_NOT_VERIFY_VOLUMES: i32 = -1073727512i32;
pub const CREATE_BYPASS_CSC: u32 = 2u32;
pub const CREATE_CRED_RESET: u32 = 4u32;
pub const CREATE_GLOBAL_MAPPING: u32 = 256u32;
pub const CREATE_NO_CONNECT: u32 = 1u32;
pub const CREATE_PERSIST_MAPPING: u32 = 32u32;
pub const CREATE_REQUIRE_CONNECTION_INTEGRITY: u32 = 8u32;
pub const CREATE_REQUIRE_CONNECTION_PRIVACY: u32 = 16u32;
pub const CREATE_WRITE_THROUGH_SEMANTICS: u32 = 64u32;
pub const CRYPT_KEY_LEN: u32 = 7u32;
pub const CRYPT_TXT_LEN: u32 = 8u32;
#[repr(transparent)]
pub struct DEFAULT_PAGES(pub i32);
pub const DPP_ADVANCED: DEFAULT_PAGES = DEFAULT_PAGES(1i32);
pub const DEF_MAX_BADPW: u32 = 0u32;
pub const DEF_MAX_PWHIST: u32 = 8u32;
pub const DEF_MIN_PWLEN: u32 = 6u32;
pub const DEF_PWUNIQUENESS: u32 = 5u32;
pub const DEVLEN: u32 = 80u32;
pub const DFS_CONNECTION_FAILURE: i32 = 1073756226i32;
pub const DFS_ERROR_ACTIVEDIRECTORY_OFFLINE: i32 = -1073727301i32;
pub const DFS_ERROR_CLUSTERINFO_FAILED: i32 = -1073727307i32;
pub const DFS_ERROR_COMPUTERINFO_FAILED: i32 = -1073727308i32;
pub const DFS_ERROR_CREATEEVENT_FAILED: i32 = -1073727309i32;
pub const DFS_ERROR_CREATE_REPARSEPOINT_FAILURE: i32 = -1073727321i32;
pub const DFS_ERROR_CREATE_REPARSEPOINT_SUCCESS: i32 = 1073756370i32;
pub const DFS_ERROR_CROSS_FOREST_TRUST_INFO_FAILED: i32 = -1073727274i32;
pub const DFS_ERROR_DCINFO_FAILED: i32 = -1073727306i32;
pub const DFS_ERROR_DSCONNECT_FAILED: i32 = -2147469122i32;
pub const DFS_ERROR_DUPLICATE_LINK: i32 = -1073727277i32;
pub const DFS_ERROR_HANDLENAMESPACE_FAILED: i32 = -1073727304i32;
pub const DFS_ERROR_LINKS_OVERLAP: i32 = -1073727280i32;
pub const DFS_ERROR_LINK_OVERLAP: i32 = -1073727279i32;
pub const DFS_ERROR_MUTLIPLE_ROOTS_NOT_SUPPORTED: i32 = -1073727289i32;
pub const DFS_ERROR_NO_DFS_DATA: i32 = -1073727294i32;
pub const DFS_ERROR_ON_ROOT: i32 = -2147469114i32;
pub const DFS_ERROR_OVERLAPPING_DIRECTORIES: i32 = -1073727319i32;
pub const DFS_ERROR_PREFIXTABLE_FAILED: i32 = -1073727305i32;
pub const DFS_ERROR_REFLECTIONENGINE_FAILED: i32 = -1073727302i32;
pub const DFS_ERROR_REGISTERSTORE_FAILED: i32 = -1073727303i32;
pub const DFS_ERROR_REMOVE_LINK_FAILED: i32 = -1073727284i32;
pub const DFS_ERROR_RESYNCHRONIZE_FAILED: i32 = -1073727285i32;
pub const DFS_ERROR_ROOTSYNCINIT_FAILED: i32 = -1073727310i32;
pub const DFS_ERROR_SECURITYINIT_FAILED: i32 = -1073727313i32;
pub const DFS_ERROR_SITECACHEINIT_FAILED: i32 = -1073727311i32;
pub const DFS_ERROR_SITESUPPOR_FAILED: i32 = -1073727300i32;
pub const DFS_ERROR_TARGET_LIST_INCORRECT: i32 = -1073727281i32;
pub const DFS_ERROR_THREADINIT_FAILED: i32 = -1073727312i32;
pub const DFS_ERROR_TOO_MANY_ERRORS: i32 = -1073727315i32;
pub const DFS_ERROR_TRUSTED_DOMAIN_INFO_FAILED: i32 = -1073727276i32;
pub const DFS_ERROR_UNSUPPORTED_FILESYSTEM: i32 = -1073727320i32;
pub const DFS_ERROR_WINSOCKINIT_FAILED: i32 = -1073727314i32;
pub const DFS_INFO_ACTIVEDIRECTORY_ONLINE: i32 = 1073756332i32;
pub const DFS_INFO_CROSS_FOREST_TRUST_INFO_SUCCESS: i32 = 1073756375i32;
pub const DFS_INFO_DOMAIN_REFERRAL_MIN_OVERFLOW: i32 = 1073756361i32;
pub const DFS_INFO_DS_RECONNECTED: i32 = 1073756353i32;
pub const DFS_INFO_FINISH_BUILDING_NAMESPACE: i32 = 1073756357i32;
pub const DFS_INFO_FINISH_INIT: i32 = 1073756355i32;
pub const DFS_INFO_RECONNECT_DATA: i32 = 1073756356i32;
pub const DFS_INFO_TRUSTED_DOMAIN_INFO_SUCCESS: i32 = 1073756373i32;
pub const DFS_INIT_SUCCESS: i32 = 1073756376i32;
pub const DFS_MAX_DNR_ATTEMPTS: i32 = 1073756229i32;
pub const DFS_OPEN_FAILURE: i32 = 1073756231i32;
pub const DFS_REFERRAL_FAILURE: i32 = 1073756227i32;
pub const DFS_REFERRAL_REQUEST: i32 = 1073756142i32;
pub const DFS_REFERRAL_SUCCESS: i32 = 1073756228i32;
pub const DFS_ROOT_SHARE_ACQUIRE_FAILED: i32 = -2147469095i32;
pub const DFS_ROOT_SHARE_ACQUIRE_SUCCESS: i32 = 1073756378i32;
pub const DFS_SPECIAL_REFERRAL_FAILURE: i32 = 1073756230i32;
pub const DFS_WARN_DOMAIN_REFERRAL_OVERFLOW: i32 = -2147469112i32;
pub const DFS_WARN_INCOMPLETE_MOVE: i32 = -2147469110i32;
pub const DFS_WARN_METADATA_LINK_INFO_INVALID: i32 = -2147469106i32;
pub const DFS_WARN_METADATA_LINK_TYPE_INCORRECT: i32 = -2147469107i32;
pub const DNLEN: u32 = 15u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[repr(C)]
pub struct DSREG_JOIN_INFO(i32);
#[repr(transparent)]
pub struct DSREG_JOIN_TYPE(pub i32);
pub const DSREG_UNKNOWN_JOIN: DSREG_JOIN_TYPE = DSREG_JOIN_TYPE(0i32);
pub const DSREG_DEVICE_JOIN: DSREG_JOIN_TYPE = DSREG_JOIN_TYPE(1i32);
pub const DSREG_WORKPLACE_JOIN: DSREG_JOIN_TYPE = DSREG_JOIN_TYPE(2i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DSREG_USER_INFO(i32);
pub const ENCRYPTED_PWLEN: u32 = 16u32;
#[repr(transparent)]
pub struct ENUM_BINDING_PATHS_FLAGS(pub i32);
pub const EBP_ABOVE: ENUM_BINDING_PATHS_FLAGS = ENUM_BINDING_PATHS_FLAGS(1i32);
pub const EBP_BELOW: ENUM_BINDING_PATHS_FLAGS = ENUM_BINDING_PATHS_FLAGS(2i32);
pub const ERRLOG2_BASE: u32 = 5700u32;
pub const ERRLOG_BASE: u32 = 3100u32;
#[repr(C)]
pub struct ERRLOG_OTHER_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct ERROR_LOG(i32);
pub const EVENT_BAD_ACCOUNT_NAME: i32 = -1073734816i32;
pub const EVENT_BAD_SERVICE_STATE: i32 = -1073734808i32;
pub const EVENT_BOOT_SYSTEM_DRIVERS_FAILED: i32 = -1073734798i32;
pub const EVENT_BOWSER_CANT_READ_REGISTRY: i32 = 1073749853i32;
pub const EVENT_BOWSER_ELECTION_RECEIVED: i32 = 8012i32;
pub const EVENT_BOWSER_ELECTION_SENT_FIND_MASTER_FAILED: i32 = 1073749838i32;
pub const EVENT_BOWSER_ELECTION_SENT_GETBLIST_FAILED: i32 = 1073749837i32;
pub const EVENT_BOWSER_GETBROWSERLIST_THRESHOLD_EXCEEDED: i32 = 1073749855i32;
pub const EVENT_BOWSER_ILLEGAL_DATAGRAM: i32 = -2147475642i32;
pub const EVENT_BOWSER_ILLEGAL_DATAGRAM_THRESHOLD: i32 = -1073733808i32;
pub const EVENT_BOWSER_MAILSLOT_DATAGRAM_THRESHOLD_EXCEEDED: i32 = 1073749854i32;
pub const EVENT_BOWSER_NAME_CONVERSION_FAILED: i32 = -1073733814i32;
pub const EVENT_BOWSER_NON_MASTER_MASTER_ANNOUNCE: i32 = -2147475643i32;
pub const EVENT_BOWSER_NON_PDC_WON_ELECTION: i32 = 1073749852i32;
pub const EVENT_BOWSER_OLD_BACKUP_FOUND: i32 = 1073749848i32;
pub const EVENT_BOWSER_OTHER_MASTER_ON_NET: i32 = -1073733821i32;
pub const EVENT_BOWSER_PDC_LOST_ELECTION: i32 = 1073749851i32;
pub const EVENT_BOWSER_PROMOTED_WHILE_ALREADY_MASTER: i32 = -2147475644i32;
pub const EVENT_BRIDGE_ADAPTER_BIND_FAILED: i32 = -1073727120i32;
pub const EVENT_BRIDGE_ADAPTER_FILTER_FAILED: i32 = -1073727122i32;
pub const EVENT_BRIDGE_ADAPTER_LINK_SPEED_QUERY_FAILED: i32 = -1073727124i32;
pub const EVENT_BRIDGE_ADAPTER_MAC_ADDR_QUERY_FAILED: i32 = -1073727123i32;
pub const EVENT_BRIDGE_ADAPTER_NAME_QUERY_FAILED: i32 = -1073727121i32;
pub const EVENT_BRIDGE_BUFFER_POOL_CREATION_FAILED: i32 = -1073727214i32;
pub const EVENT_BRIDGE_DEVICE_CREATION_FAILED: i32 = -1073727221i32;
pub const EVENT_BRIDGE_ETHERNET_NOT_OFFERED: i32 = -1073727218i32;
pub const EVENT_BRIDGE_INIT_MALLOC_FAILED: i32 = -1073727213i32;
pub const EVENT_BRIDGE_MINIPORT_INIT_FAILED: i32 = -1073727219i32;
pub const EVENT_BRIDGE_MINIPORT_REGISTER_FAILED: i32 = -1073727222i32;
pub const EVENT_BRIDGE_MINIPROT_DEVNAME_MISSING: i32 = -1073727223i32;
pub const EVENT_BRIDGE_NO_BRIDGE_MAC_ADDR: i32 = -1073727220i32;
pub const EVENT_BRIDGE_PACKET_POOL_CREATION_FAILED: i32 = -1073727215i32;
pub const EVENT_BRIDGE_PROTOCOL_REGISTER_FAILED: i32 = -1073727224i32;
pub const EVENT_BRIDGE_THREAD_CREATION_FAILED: i32 = -1073727217i32;
pub const EVENT_BRIDGE_THREAD_REF_FAILED: i32 = -1073727216i32;
pub const EVENT_BROWSER_BACKUP_STOPPED: i32 = -1073733792i32;
pub const EVENT_BROWSER_DEPENDANT_SERVICE_FAILED: i32 = -1073733807i32;
pub const EVENT_BROWSER_DOMAIN_LIST_FAILED: i32 = -2147475626i32;
pub const EVENT_BROWSER_DOMAIN_LIST_RETRIEVED: i32 = 8026i32;
pub const EVENT_BROWSER_ELECTION_SENT_LANMAN_NT_STARTED: i32 = 1073749839i32;
pub const EVENT_BROWSER_ELECTION_SENT_LANMAN_NT_STOPPED: i32 = 1073749857i32;
pub const EVENT_BROWSER_ELECTION_SENT_ROLE_CHANGED: i32 = 1073749859i32;
pub const EVENT_BROWSER_GETBLIST_RECEIVED_NOT_MASTER: i32 = -1073733790i32;
pub const EVENT_BROWSER_ILLEGAL_CONFIG: i32 = -2147475625i32;
pub const EVENT_BROWSER_MASTER_PROMOTION_FAILED: i32 = -1073733815i32;
pub const EVENT_BROWSER_MASTER_PROMOTION_FAILED_NO_MASTER: i32 = -1073733804i32;
pub const EVENT_BROWSER_MASTER_PROMOTION_FAILED_STOPPING: i32 = -1073733805i32;
pub const EVENT_BROWSER_NOT_STARTED_IPX_CONFIG_MISMATCH: i32 = -1073733788i32;
pub const EVENT_BROWSER_OTHERDOMAIN_ADD_FAILED: i32 = -1073733813i32;
pub const EVENT_BROWSER_ROLE_CHANGE_FAILED: i32 = -1073733816i32;
pub const EVENT_BROWSER_SERVER_LIST_FAILED: i32 = -2147475627i32;
pub const EVENT_BROWSER_SERVER_LIST_RETRIEVED: i32 = 8025i32;
pub const EVENT_BROWSER_STATUS_BITS_UPDATE_FAILED: i32 = -1073733817i32;
pub const EVENT_CALL_TO_FUNCTION_FAILED: i32 = -1073734819i32;
pub const EVENT_CALL_TO_FUNCTION_FAILED_II: i32 = -1073734818i32;
pub const EVENT_CIRCULAR_DEPENDENCY_AUTO: i32 = -1073734806i32;
pub const EVENT_CIRCULAR_DEPENDENCY_DEMAND: i32 = -1073734807i32;
pub const EVENT_COMMAND_NOT_INTERACTIVE: i32 = -1073733924i32;
pub const EVENT_COMMAND_START_FAILED: i32 = -1073733923i32;
pub const EVENT_CONNECTION_TIMEOUT: i32 = -1073734815i32;
pub const EVENT_ComputerNameChange: i32 = -2147477637i32;
pub const EVENT_DAV_REDIR_DELAYED_WRITE_FAILED: i32 = -2147468848i32;
pub const EVENT_DCOM_ASSERTION_FAILURE: i32 = -1073731812i32;
pub const EVENT_DCOM_COMPLUS_DISABLED: i32 = -1073731810i32;
pub const EVENT_DCOM_INVALID_ENDPOINT_DATA: i32 = -1073731811i32;
pub const EVENT_DEPEND_ON_LATER_GROUP: i32 = -1073734804i32;
pub const EVENT_DEPEND_ON_LATER_SERVICE: i32 = -1073734805i32;
pub const EVENT_DNSAPI_DEREGISTRATION_FAILED_NOTSUPP: i32 = -2147472466i32;
pub const EVENT_DNSAPI_DEREGISTRATION_FAILED_NOTSUPP_PRIMARY_DN: i32 = -2147472454i32;
pub const EVENT_DNSAPI_DEREGISTRATION_FAILED_OTHER: i32 = -2147472463i32;
pub const EVENT_DNSAPI_DEREGISTRATION_FAILED_OTHER_PRIMARY_DN: i32 = -2147472451i32;
pub const EVENT_DNSAPI_DEREGISTRATION_FAILED_REFUSED: i32 = -2147472465i32;
pub const EVENT_DNSAPI_DEREGISTRATION_FAILED_REFUSED_PRIMARY_DN: i32 = -2147472453i32;
pub const EVENT_DNSAPI_DEREGISTRATION_FAILED_SECURITY: i32 = -2147472464i32;
pub const EVENT_DNSAPI_DEREGISTRATION_FAILED_SECURITY_PRIMARY_DN: i32 = -2147472452i32;
pub const EVENT_DNSAPI_DEREGISTRATION_FAILED_SERVERFAIL: i32 = -2147472467i32;
pub const EVENT_DNSAPI_DEREGISTRATION_FAILED_SERVERFAIL_PRIMARY_DN: i32 = -2147472455i32;
pub const EVENT_DNSAPI_DEREGISTRATION_FAILED_TIMEOUT: i32 = -2147472468i32;
pub const EVENT_DNSAPI_DEREGISTRATION_FAILED_TIMEOUT_PRIMARY_DN: i32 = -2147472456i32;
pub const EVENT_DNSAPI_PTR_DEREGISTRATION_FAILED_NOTSUPP: i32 = -2147472460i32;
pub const EVENT_DNSAPI_PTR_DEREGISTRATION_FAILED_OTHER: i32 = -2147472457i32;
pub const EVENT_DNSAPI_PTR_DEREGISTRATION_FAILED_REFUSED: i32 = -2147472459i32;
pub const EVENT_DNSAPI_PTR_DEREGISTRATION_FAILED_SECURITY: i32 = -2147472458i32;
pub const EVENT_DNSAPI_PTR_DEREGISTRATION_FAILED_SERVERFAIL: i32 = -2147472461i32;
pub const EVENT_DNSAPI_PTR_DEREGISTRATION_FAILED_TIMEOUT: i32 = -2147472462i32;
pub const EVENT_DNSAPI_PTR_REGISTRATION_FAILED_NOTSUPP: i32 = -2147472490i32;
pub const EVENT_DNSAPI_PTR_REGISTRATION_FAILED_OTHER: i32 = -2147472487i32;
pub const EVENT_DNSAPI_PTR_REGISTRATION_FAILED_REFUSED: i32 = -2147472489i32;
pub const EVENT_DNSAPI_PTR_REGISTRATION_FAILED_SECURITY: i32 = -2147472488i32;
pub const EVENT_DNSAPI_PTR_REGISTRATION_FAILED_SERVERFAIL: i32 = -2147472491i32;
pub const EVENT_DNSAPI_PTR_REGISTRATION_FAILED_TIMEOUT: i32 = -2147472492i32;
pub const EVENT_DNSAPI_REGISTERED_ADAPTER: i32 = 1073753024i32;
pub const EVENT_DNSAPI_REGISTERED_ADAPTER_PRIMARY_DN: i32 = 1073753026i32;
pub const EVENT_DNSAPI_REGISTERED_PTR: i32 = 1073753025i32;
pub const EVENT_DNSAPI_REGISTRATION_FAILED_NOTSUPP: i32 = -2147472496i32;
pub const EVENT_DNSAPI_REGISTRATION_FAILED_NOTSUPP_PRIMARY_DN: i32 = -2147472484i32;
pub const EVENT_DNSAPI_REGISTRATION_FAILED_OTHER: i32 = -2147472493i32;
pub const EVENT_DNSAPI_REGISTRATION_FAILED_OTHER_PRIMARY_DN: i32 = -2147472481i32;
pub const EVENT_DNSAPI_REGISTRATION_FAILED_REFUSED: i32 = -2147472495i32;
pub const EVENT_DNSAPI_REGISTRATION_FAILED_REFUSED_PRIMARY_DN: i32 = -2147472483i32;
pub const EVENT_DNSAPI_REGISTRATION_FAILED_SECURITY: i32 = -2147472494i32;
pub const EVENT_DNSAPI_REGISTRATION_FAILED_SECURITY_PRIMARY_DN: i32 = -2147472482i32;
pub const EVENT_DNSAPI_REGISTRATION_FAILED_SERVERFAIL: i32 = -2147472497i32;
pub const EVENT_DNSAPI_REGISTRATION_FAILED_SERVERFAIL_PRIMARY_DN: i32 = -2147472485i32;
pub const EVENT_DNSAPI_REGISTRATION_FAILED_TIMEOUT: i32 = -2147472498i32;
pub const EVENT_DNSAPI_REGISTRATION_FAILED_TIMEOUT_PRIMARY_DN: i32 = -2147472486i32;
pub const EVENT_DNSDomainNameChange: i32 = -2147477636i32;
pub const EVENT_DNS_CACHE_NETWORK_PERF_WARNING: i32 = -2147472598i32;
pub const EVENT_DNS_CACHE_START_FAILURE_LOW_MEMORY: i32 = -1073730817i32;
pub const EVENT_DNS_CACHE_START_FAILURE_NO_CONTROL: i32 = -1073730822i32;
pub const EVENT_DNS_CACHE_START_FAILURE_NO_DLL: i32 = -1073730824i32;
pub const EVENT_DNS_CACHE_START_FAILURE_NO_DONE_EVENT: i32 = -1073730821i32;
pub const EVENT_DNS_CACHE_START_FAILURE_NO_ENTRY: i32 = -1073730823i32;
pub const EVENT_DNS_CACHE_START_FAILURE_NO_RPC: i32 = -1073730820i32;
pub const EVENT_DNS_CACHE_START_FAILURE_NO_SHUTDOWN_NOTIFY: i32 = -1073730819i32;
pub const EVENT_DNS_CACHE_START_FAILURE_NO_UPDATE: i32 = -1073730818i32;
pub const EVENT_DNS_CACHE_UNABLE_TO_REACH_SERVER_WARNING: i32 = -2147472597i32;
pub const EVENT_EQOS_ERROR_MACHINE_POLICY_KEYNAME_SIZE_ZERO: i32 = -1073725118i32;
pub const EVENT_EQOS_ERROR_MACHINE_POLICY_KEYNAME_TOO_LONG: i32 = -1073725120i32;
pub const EVENT_EQOS_ERROR_MACHINE_POLICY_REFERESH: i32 = -1073725124i32;
pub const EVENT_EQOS_ERROR_OPENING_MACHINE_POLICY_ROOT_KEY: i32 = -1073725122i32;
pub const EVENT_EQOS_ERROR_OPENING_MACHINE_POLICY_SUBKEY: i32 = -1073725116i32;
pub const EVENT_EQOS_ERROR_OPENING_USER_POLICY_ROOT_KEY: i32 = -1073725121i32;
pub const EVENT_EQOS_ERROR_OPENING_USER_POLICY_SUBKEY: i32 = -1073725115i32;
pub const EVENT_EQOS_ERROR_PROCESSING_MACHINE_POLICY_FIELD: i32 = -1073725114i32;
pub const EVENT_EQOS_ERROR_PROCESSING_USER_POLICY_FIELD: i32 = -1073725113i32;
pub const EVENT_EQOS_ERROR_SETTING_APP_MARKING: i32 = -1073725111i32;
pub const EVENT_EQOS_ERROR_SETTING_TCP_AUTOTUNING: i32 = -1073725112i32;
pub const EVENT_EQOS_ERROR_USER_POLICY_KEYNAME_SIZE_ZERO: i32 = -1073725117i32;
pub const EVENT_EQOS_ERROR_USER_POLICY_KEYNAME_TOO_LONG: i32 = -1073725119i32;
pub const EVENT_EQOS_ERROR_USER_POLICY_REFERESH: i32 = -1073725123i32;
pub const EVENT_EQOS_INFO_APP_MARKING_ALLOWED: i32 = 1073758335i32;
pub const EVENT_EQOS_INFO_APP_MARKING_IGNORED: i32 = 1073758334i32;
pub const EVENT_EQOS_INFO_APP_MARKING_NOT_CONFIGURED: i32 = 1073758333i32;
pub const EVENT_EQOS_INFO_LOCAL_SETTING_DONT_USE_NLA: i32 = 1073758336i32;
pub const EVENT_EQOS_INFO_MACHINE_POLICY_REFRESH_NO_CHANGE: i32 = 1073758324i32;
pub const EVENT_EQOS_INFO_MACHINE_POLICY_REFRESH_WITH_CHANGE: i32 = 1073758325i32;
pub const EVENT_EQOS_INFO_TCP_AUTOTUNING_HIGHLY_RESTRICTED: i32 = 1073758330i32;
pub const EVENT_EQOS_INFO_TCP_AUTOTUNING_NORMAL: i32 = 1073758332i32;
pub const EVENT_EQOS_INFO_TCP_AUTOTUNING_NOT_CONFIGURED: i32 = 1073758328i32;
pub const EVENT_EQOS_INFO_TCP_AUTOTUNING_OFF: i32 = 1073758329i32;
pub const EVENT_EQOS_INFO_TCP_AUTOTUNING_RESTRICTED: i32 = 1073758331i32;
pub const EVENT_EQOS_INFO_USER_POLICY_REFRESH_NO_CHANGE: i32 = 1073758326i32;
pub const EVENT_EQOS_INFO_USER_POLICY_REFRESH_WITH_CHANGE: i32 = 1073758327i32;
pub const EVENT_EQOS_URL_QOS_APPLICATION_CONFLICT: i32 = 1073758337i32;
pub const EVENT_EQOS_WARNING_MACHINE_POLICY_CONFLICT: i32 = -2147467040i32;
pub const EVENT_EQOS_WARNING_MACHINE_POLICY_NO_FULLPATH_APPNAME: i32 = -2147467038i32;
pub const EVENT_EQOS_WARNING_MACHINE_POLICY_PROFILE_NOT_SPECIFIED: i32 = -2147467044i32;
pub const EVENT_EQOS_WARNING_MACHINE_POLICY_QUOTA_EXCEEDED: i32 = -2147467042i32;
pub const EVENT_EQOS_WARNING_MACHINE_POLICY_VERSION: i32 = -2147467046i32;
pub const EVENT_EQOS_WARNING_TEST_1: i32 = -2147467048i32;
pub const EVENT_EQOS_WARNING_TEST_2: i32 = -2147467047i32;
pub const EVENT_EQOS_WARNING_USER_POLICY_CONFLICT: i32 = -2147467039i32;
pub const EVENT_EQOS_WARNING_USER_POLICY_NO_FULLPATH_APPNAME: i32 = -2147467037i32;
pub const EVENT_EQOS_WARNING_USER_POLICY_PROFILE_NOT_SPECIFIED: i32 = -2147467043i32;
pub const EVENT_EQOS_WARNING_USER_POLICY_QUOTA_EXCEEDED: i32 = -2147467041i32;
pub const EVENT_EQOS_WARNING_USER_POLICY_VERSION: i32 = -2147467045i32;
pub const EVENT_EventLogProductInfo: i32 = -2147477639i32;
pub const EVENT_EventlogAbnormalShutdown: i32 = -2147477640i32;
pub const EVENT_EventlogStarted: i32 = -2147477643i32;
pub const EVENT_EventlogStopped: i32 = -2147477642i32;
pub const EVENT_EventlogUptime: i32 = -2147477635i32;
pub const EVENT_FIRST_LOGON_FAILED: i32 = -1073734811i32;
pub const EVENT_FIRST_LOGON_FAILED_II: i32 = -1073734786i32;
pub const EVENT_FRS_ACCESS_CHECKS_DISABLED: i32 = -2147470131i32;
pub const EVENT_FRS_ACCESS_CHECKS_FAILED_UNKNOWN: i32 = -1073728305i32;
pub const EVENT_FRS_ACCESS_CHECKS_FAILED_USER: i32 = -2147470130i32;
pub const EVENT_FRS_ASSERT: i32 = -1073728318i32;
pub const EVENT_FRS_BAD_REG_DATA: i32 = -2147470101i32;
pub const EVENT_FRS_CANNOT_COMMUNICATE: i32 = -1073728314i32;
pub const EVENT_FRS_CANNOT_CREATE_UUID: i32 = -1073728300i32;
pub const EVENT_FRS_CANNOT_START_BACKUP_RESTORE_IN_PROGRESS: i32 = -1073728303i32;
pub const EVENT_FRS_CANT_OPEN_PREINSTALL: i32 = -1073728273i32;
pub const EVENT_FRS_CANT_OPEN_STAGE: i32 = -1073728274i32;
pub const EVENT_FRS_DATABASE_SPACE: i32 = -1073728313i32;
pub const EVENT_FRS_DISK_WRITE_CACHE_ENABLED: i32 = -2147470136i32;
pub const EVENT_FRS_DS_POLL_ERROR_SUMMARY: i32 = -2147470086i32;
pub const EVENT_FRS_DUPLICATE_IN_CXTION: i32 = -1073728266i32;
pub const EVENT_FRS_DUPLICATE_IN_CXTION_SYSVOL: i32 = -1073728267i32;
pub const EVENT_FRS_ERROR: i32 = -1073728324i32;
pub const EVENT_FRS_ERROR_REPLICA_SET_DELETED: i32 = -2147470088i32;
pub const EVENT_FRS_HUGE_FILE: i32 = -2147470125i32;
pub const EVENT_FRS_IN_ERROR_STATE: i32 = -1073728269i32;
pub const EVENT_FRS_JET_1414: i32 = -1073728311i32;
pub const EVENT_FRS_JOIN_FAIL_TIME_SKEW: i32 = -1073728276i32;
pub const EVENT_FRS_LONG_JOIN: i32 = -2147470140i32;
pub const EVENT_FRS_LONG_JOIN_DONE: i32 = -2147470139i32;
pub const EVENT_FRS_MOVED_PREEXISTING: i32 = -2147470128i32;
pub const EVENT_FRS_NO_DNS_ATTRIBUTE: i32 = -2147470123i32;
pub const EVENT_FRS_NO_SID: i32 = -1073728298i32;
pub const EVENT_FRS_OVERLAPS_LOGGING: i32 = -1073728283i32;
pub const EVENT_FRS_OVERLAPS_OTHER_STAGE: i32 = -1073728279i32;
pub const EVENT_FRS_OVERLAPS_ROOT: i32 = -1073728280i32;
pub const EVENT_FRS_OVERLAPS_STAGE: i32 = -1073728281i32;
pub const EVENT_FRS_OVERLAPS_WORKING: i32 = -1073728282i32;
pub const EVENT_FRS_PREPARE_ROOT_FAILED: i32 = -1073728278i32;
pub const EVENT_FRS_REPLICA_IN_JRNL_WRAP_ERROR: i32 = -1073728263i32;
pub const EVENT_FRS_REPLICA_NO_ROOT_CHANGE: i32 = -1073728268i32;
pub const EVENT_FRS_REPLICA_SET_CREATE_FAIL: i32 = -1073728272i32;
pub const EVENT_FRS_REPLICA_SET_CREATE_OK: i32 = 1073755377i32;
pub const EVENT_FRS_REPLICA_SET_CXTIONS: i32 = 1073755378i32;
pub const EVENT_FRS_RMTCO_TIME_SKEW: i32 = -1073728275i32;
pub const EVENT_FRS_ROOT_HAS_MOVED: i32 = -1073728265i32;
pub const EVENT_FRS_ROOT_NOT_VALID: i32 = -1073728285i32;
pub const EVENT_FRS_STAGE_NOT_VALID: i32 = -1073728284i32;
pub const EVENT_FRS_STAGING_AREA_FULL: i32 = -2147470126i32;
pub const EVENT_FRS_STARTING: i32 = 1073755325i32;
pub const EVENT_FRS_STOPPED: i32 = 1073755327i32;
pub const EVENT_FRS_STOPPED_ASSERT: i32 = -1073728319i32;
pub const EVENT_FRS_STOPPED_FORCE: i32 = -1073728320i32;
pub const EVENT_FRS_STOPPING: i32 = 1073755326i32;
pub const EVENT_FRS_SYSVOL_NOT_READY: i32 = -2147470134i32;
pub const EVENT_FRS_SYSVOL_NOT_READY_PRIMARY: i32 = -2147470133i32;
pub const EVENT_FRS_SYSVOL_READY: i32 = 1073755340i32;
pub const EVENT_FRS_VOLUME_NOT_SUPPORTED: i32 = -1073728317i32;
pub const EVENT_INVALID_DRIVER_DEPENDENCY: i32 = -1073734809i32;
pub const EVENT_IPX_CREATE_DEVICE: i32 = -1073732318i32;
pub const EVENT_IPX_ILLEGAL_CONFIG: i32 = -2147474145i32;
pub const EVENT_IPX_INTERNAL_NET_INVALID: i32 = -1073732320i32;
pub const EVENT_IPX_NEW_DEFAULT_TYPE: i32 = 1073751325i32;
pub const EVENT_IPX_NO_ADAPTERS: i32 = -1073732317i32;
pub const EVENT_IPX_NO_FRAME_TYPES: i32 = -1073732319i32;
pub const EVENT_IPX_SAP_ANNOUNCE: i32 = -2147474146i32;
pub const EVENT_NBT_BAD_BACKUP_WINS_ADDR: i32 = -2147479344i32;
pub const EVENT_NBT_BAD_PRIMARY_WINS_ADDR: i32 = -2147479343i32;
pub const EVENT_NBT_CREATE_ADDRESS: i32 = -1073737517i32;
pub const EVENT_NBT_CREATE_CONNECTION: i32 = -1073737516i32;
pub const EVENT_NBT_CREATE_DEVICE: i32 = -1073737513i32;
pub const EVENT_NBT_CREATE_DRIVER: i32 = -1073737524i32;
pub const EVENT_NBT_DUPLICATE_NAME: i32 = -1073737505i32;
pub const EVENT_NBT_DUPLICATE_NAME_ERROR: i32 = -1073737503i32;
pub const EVENT_NBT_NAME_RELEASE: i32 = -1073737504i32;
pub const EVENT_NBT_NAME_SERVER_ADDRS: i32 = -1073737518i32;
pub const EVENT_NBT_NON_OS_INIT: i32 = -1073737515i32;
pub const EVENT_NBT_NO_BACKUP_WINS: i32 = -2147479346i32;
pub const EVENT_NBT_NO_DEVICES: i32 = -2147479336i32;
pub const EVENT_NBT_NO_RESOURCES: i32 = -1073737502i32;
pub const EVENT_NBT_NO_WINS: i32 = -2147479345i32;
pub const EVENT_NBT_OPEN_REG_LINKAGE: i32 = -1073737511i32;
pub const EVENT_NBT_OPEN_REG_NAMESERVER: i32 = -2147479332i32;
pub const EVENT_NBT_OPEN_REG_PARAMS: i32 = -1073737523i32;
pub const EVENT_NBT_READ_BIND: i32 = -1073737510i32;
pub const EVENT_NBT_READ_EXPORT: i32 = -1073737509i32;
pub const EVENT_NBT_TIMERS: i32 = -1073737514i32;
pub const EVENT_NDIS_ADAPTER_CHECK_ERROR: i32 = -1073736793i32;
pub const EVENT_NDIS_ADAPTER_DISABLED: i32 = -2147478634i32;
pub const EVENT_NDIS_ADAPTER_NOT_FOUND: i32 = -1073736821i32;
pub const EVENT_NDIS_BAD_IO_BASE_ADDRESS: i32 = -1073736812i32;
pub const EVENT_NDIS_BAD_VERSION: i32 = -1073736818i32;
pub const EVENT_NDIS_CABLE_DISCONNECTED_ERROR: i32 = -2147478615i32;
pub const EVENT_NDIS_DMA_CONFLICT: i32 = -2147478629i32;
pub const EVENT_NDIS_DRIVER_FAILURE: i32 = -1073736819i32;
pub const EVENT_NDIS_HARDWARE_FAILURE: i32 = -1073736822i32;
pub const EVENT_NDIS_INTERRUPT_CONFLICT: i32 = -2147478630i32;
pub const EVENT_NDIS_INTERRUPT_CONNECT: i32 = -1073736820i32;
pub const EVENT_NDIS_INVALID_DOWNLOAD_FILE_ERROR: i32 = -1073736804i32;
pub const EVENT_NDIS_INVALID_VALUE_FROM_ADAPTER: i32 = -1073736814i32;
pub const EVENT_NDIS_IO_PORT_CONFLICT: i32 = -2147478633i32;
pub const EVENT_NDIS_LOBE_FAILUE_ERROR: i32 = -2147478621i32;
pub const EVENT_NDIS_MAXFRAMESIZE_ERROR: i32 = -2147478625i32;
pub const EVENT_NDIS_MAXINTERNALBUFS_ERROR: i32 = -2147478624i32;
pub const EVENT_NDIS_MAXMULTICAST_ERROR: i32 = -2147478623i32;
pub const EVENT_NDIS_MAXRECEIVES_ERROR: i32 = -2147478627i32;
pub const EVENT_NDIS_MAXTRANSMITS_ERROR: i32 = -2147478626i32;
pub const EVENT_NDIS_MEMORY_CONFLICT: i32 = -2147478631i32;
pub const EVENT_NDIS_MISSING_CONFIGURATION_PARAMETER: i32 = -1073736813i32;
pub const EVENT_NDIS_NETWORK_ADDRESS: i32 = -1073736816i32;
pub const EVENT_NDIS_OUT_OF_RESOURCE: i32 = -1073736823i32;
pub const EVENT_NDIS_PORT_OR_DMA_CONFLICT: i32 = -2147478632i32;
pub const EVENT_NDIS_PRODUCTID_ERROR: i32 = -2147478622i32;
pub const EVENT_NDIS_RECEIVE_SPACE_SMALL: i32 = 1073746837i32;
pub const EVENT_NDIS_REMOVE_RECEIVED_ERROR: i32 = -2147478619i32;
pub const EVENT_NDIS_RESET_FAILURE_CORRECTION: i32 = -2147478614i32;
pub const EVENT_NDIS_RESET_FAILURE_ERROR: i32 = -2147478616i32;
pub const EVENT_NDIS_RESOURCE_CONFLICT: i32 = -1073736824i32;
pub const EVENT_NDIS_SIGNAL_LOSS_ERROR: i32 = -2147478620i32;
pub const EVENT_NDIS_TIMEOUT: i32 = -2147478641i32;
pub const EVENT_NDIS_TOKEN_RING_CORRECTION: i32 = 1073746854i32;
pub const EVENT_NDIS_UNSUPPORTED_CONFIGURATION: i32 = -1073736815i32;
pub const EVENT_PS_ADMISSIONCONTROL_OVERFLOW: i32 = -2147469537i32;
pub const EVENT_PS_BAD_BESTEFFORT_LIMIT: i32 = -2147469548i32;
pub const EVENT_PS_BINDING_FAILED: i32 = -1073727720i32;
pub const EVENT_PS_GPC_REGISTER_FAILED: i32 = -1073727824i32;
pub const EVENT_PS_INIT_DEVICE_FAILED: i32 = -1073727717i32;
pub const EVENT_PS_MISSING_ADAPTER_REGISTRY_DATA: i32 = -1073727719i32;
pub const EVENT_PS_NETWORK_ADDRESS_FAIL: i32 = -1073727712i32;
pub const EVENT_PS_NO_RESOURCES_FOR_INIT: i32 = -1073727823i32;
pub const EVENT_PS_QUERY_OID_GEN_LINK_SPEED: i32 = -1073727721i32;
pub const EVENT_PS_QUERY_OID_GEN_MAXIMUM_FRAME_SIZE: i32 = -1073727723i32;
pub const EVENT_PS_QUERY_OID_GEN_MAXIMUM_TOTAL_SIZE: i32 = -1073727722i32;
pub const EVENT_PS_REGISTER_ADDRESS_FAMILY_FAILED: i32 = -1073727718i32;
pub const EVENT_PS_REGISTER_MINIPORT_FAILED: i32 = -1073727821i32;
pub const EVENT_PS_REGISTER_PROTOCOL_FAILED: i32 = -1073727822i32;
pub const EVENT_PS_RESOURCE_POOL: i32 = -1073727714i32;
pub const EVENT_PS_WAN_LIMITED_BESTEFFORT: i32 = -2147469539i32;
pub const EVENT_PS_WMI_INSTANCE_NAME_FAILED: i32 = -1073727716i32;
pub const EVENT_RDR_AT_THREAD_MAX: i32 = -2147480622i32;
pub const EVENT_RDR_CANT_BIND_TRANSPORT: i32 = -2147480616i32;
pub const EVENT_RDR_CANT_BUILD_SMB_HEADER: i32 = -2147480613i32;
pub const EVENT_RDR_CANT_CREATE_DEVICE: i32 = -2147480646i32;
pub const EVENT_RDR_CANT_CREATE_THREAD: i32 = -2147480645i32;
pub const EVENT_RDR_CANT_GET_SECURITY_CONTEXT: i32 = -2147480614i32;
pub const EVENT_RDR_CANT_READ_REGISTRY: i32 = -2147480621i32;
pub const EVENT_RDR_CANT_REGISTER_ADDRESS: i32 = -2147480615i32;
pub const EVENT_RDR_CANT_SET_THREAD: i32 = -2147480644i32;
pub const EVENT_RDR_CLOSE_BEHIND: i32 = -2147480637i32;
pub const EVENT_RDR_CONNECTION: i32 = -2147480629i32;
pub const EVENT_RDR_CONNECTION_REFERENCE: i32 = -2147480633i32;
pub const EVENT_RDR_CONTEXTS: i32 = -2147480624i32;
pub const EVENT_RDR_DELAYED_SET_ATTRIBUTES_FAILED: i32 = -2147480618i32;
pub const EVENT_RDR_DELETEONCLOSE_FAILED: i32 = -2147480617i32;
pub const EVENT_RDR_DISPOSITION: i32 = -2147480625i32;
pub const EVENT_RDR_ENCRYPT: i32 = -2147480630i32;
pub const EVENT_RDR_FAILED_UNLOCK: i32 = -2147480639i32;
pub const EVENT_RDR_INVALID_LOCK_REPLY: i32 = -2147480641i32;
pub const EVENT_RDR_INVALID_OPLOCK: i32 = -2147480634i32;
pub const EVENT_RDR_INVALID_REPLY: i32 = -2147480643i32;
pub const EVENT_RDR_INVALID_SMB: i32 = -2147480642i32;
pub const EVENT_RDR_MAXCMDS: i32 = -2147480627i32;
pub const EVENT_RDR_OPLOCK_SMB: i32 = -2147480626i32;
pub const EVENT_RDR_PRIMARY_TRANSPORT_CONNECT_FAILED: i32 = -2147480619i32;
pub const EVENT_RDR_RESOURCE_SHORTAGE: i32 = -2147480647i32;
pub const EVENT_RDR_SECURITY_SIGNATURE_MISMATCH: i32 = -2147480612i32;
pub const EVENT_RDR_SERVER_REFERENCE: i32 = -2147480632i32;
pub const EVENT_RDR_SMB_REFERENCE: i32 = -2147480631i32;
pub const EVENT_RDR_TIMEOUT: i32 = -2147480635i32;
pub const EVENT_RDR_TIMEZONE_BIAS_TOO_LARGE: i32 = -2147480620i32;
pub const EVENT_RDR_UNEXPECTED_ERROR: i32 = -2147480636i32;
pub const EVENT_RDR_WRITE_BEHIND_FLUSH_FAILED: i32 = -2147480623i32;
pub const EVENT_READFILE_TIMEOUT: i32 = -1073734814i32;
pub const EVENT_REVERTED_TO_LASTKNOWNGOOD: i32 = -1073734817i32;
pub const EVENT_RPCSS_ACTIVATION_ERROR: i32 = -1073731817i32;
pub const EVENT_RPCSS_CREATEDEBUGGERPROCESS_FAILURE: i32 = -1073731794i32;
pub const EVENT_RPCSS_CREATEPROCESS_FAILURE: i32 = -1073731824i32;
pub const EVENT_RPCSS_DEFAULT_LAUNCH_ACCESS_DENIED: i32 = -1073731821i32;
pub const EVENT_RPCSS_LAUNCH_ACCESS_DENIED: i32 = -1073731822i32;
pub const EVENT_RPCSS_REMOTE_SIDE_ERROR: i32 = -1073731818i32;
pub const EVENT_RPCSS_REMOTE_SIDE_ERROR_WITH_FILE: i32 = -1073731816i32;
pub const EVENT_RPCSS_REMOTE_SIDE_UNAVAILABLE: i32 = -1073731815i32;
pub const EVENT_RPCSS_RUNAS_CANT_LOGIN: i32 = -1073731820i32;
pub const EVENT_RPCSS_RUNAS_CREATEPROCESS_FAILURE: i32 = -1073731823i32;
pub const EVENT_RPCSS_SERVER_NOT_RESPONDING: i32 = -1073731813i32;
pub const EVENT_RPCSS_SERVER_START_TIMEOUT: i32 = -1073731814i32;
pub const EVENT_RPCSS_START_SERVICE_FAILURE: i32 = -1073731819i32;
pub const EVENT_RPCSS_STOP_SERVICE_FAILURE: i32 = -1073731795i32;
pub const EVENT_RUNNING_LASTKNOWNGOOD: i32 = -1073734797i32;
pub const EVENT_SCOPE_LABEL_TOO_LONG: i32 = -2147479331i32;
pub const EVENT_SCOPE_TOO_LONG: i32 = -2147479330i32;
pub const EVENT_SECOND_LOGON_FAILED: i32 = -1073734810i32;
pub const EVENT_SERVICE_CONFIG_BACKOUT_FAILED: i32 = -1073734787i32;
pub const EVENT_SERVICE_CONTROL_SUCCESS: i32 = 1073748859i32;
pub const EVENT_SERVICE_CRASH: i32 = -1073734793i32;
pub const EVENT_SERVICE_CRASH_NO_ACTION: i32 = -1073734790i32;
pub const EVENT_SERVICE_DIFFERENT_PID_CONNECTED: i32 = -2147476609i32;
pub const EVENT_SERVICE_EXIT_FAILED: i32 = -1073734801i32;
pub const EVENT_SERVICE_EXIT_FAILED_SPECIFIC: i32 = -1073734800i32;
pub const EVENT_SERVICE_LOGON_TYPE_NOT_GRANTED: i32 = -1073734783i32;
pub const EVENT_SERVICE_NOT_INTERACTIVE: i32 = -1073734794i32;
pub const EVENT_SERVICE_RECOVERY_FAILED: i32 = -1073734792i32;
pub const EVENT_SERVICE_SCESRV_FAILED: i32 = -1073734791i32;
pub const EVENT_SERVICE_SHUTDOWN_FAILED: i32 = -1073734781i32;
pub const EVENT_SERVICE_START_AT_BOOT_FAILED: i32 = -1073734799i32;
pub const EVENT_SERVICE_START_FAILED: i32 = -1073734824i32;
pub const EVENT_SERVICE_START_FAILED_GROUP: i32 = -1073734822i32;
pub const EVENT_SERVICE_START_FAILED_II: i32 = -1073734823i32;
pub const EVENT_SERVICE_START_FAILED_NONE: i32 = -1073734821i32;
pub const EVENT_SERVICE_START_HUNG: i32 = -1073734802i32;
pub const EVENT_SERVICE_START_TYPE_CHANGED: i32 = 1073748864i32;
pub const EVENT_SERVICE_STATUS_SUCCESS: i32 = 1073748860i32;
pub const EVENT_SERVICE_STOP_SUCCESS_WITH_REASON: i32 = 1073748866i32;
pub const EVENT_SEVERE_SERVICE_FAILED: i32 = -1073734803i32;
pub const EVENT_SRV_CANT_BIND_DUP_NAME: i32 = -1073739319i32;
pub const EVENT_SRV_CANT_BIND_TO_TRANSPORT: i32 = -2147481144i32;
pub const EVENT_SRV_CANT_CHANGE_DOMAIN_NAME: i32 = -2147481136i32;
pub const EVENT_SRV_CANT_CREATE_DEVICE: i32 = -1073739822i32;
pub const EVENT_SRV_CANT_CREATE_PROCESS: i32 = -1073739821i32;
pub const EVENT_SRV_CANT_CREATE_THREAD: i32 = -1073739820i32;
pub const EVENT_SRV_CANT_GROW_TABLE: i32 = -2147481639i32;
pub const EVENT_SRV_CANT_LOAD_DRIVER: i32 = -2147481140i32;
pub const EVENT_SRV_CANT_MAP_ERROR: i32 = -2147481138i32;
pub const EVENT_SRV_CANT_OPEN_NPFS: i32 = -1073739817i32;
pub const EVENT_SRV_CANT_RECREATE_SHARE: i32 = -2147481137i32;
pub const EVENT_SRV_CANT_START_SCAVENGER: i32 = -1073739814i32;
pub const EVENT_SRV_CANT_UNLOAD_DRIVER: i32 = -2147481139i32;
pub const EVENT_SRV_DISK_FULL: i32 = -2147481635i32;
pub const EVENT_SRV_DOS_ATTACK_DETECTED: i32 = -2147481623i32;
pub const EVENT_SRV_INVALID_REGISTRY_VALUE: i32 = -2147481142i32;
pub const EVENT_SRV_INVALID_REQUEST: i32 = -1073739818i32;
pub const EVENT_SRV_INVALID_SD: i32 = -2147481141i32;
pub const EVENT_SRV_IRP_STACK_SIZE: i32 = -1073739813i32;
pub const EVENT_SRV_KEY_NOT_CREATED: i32 = -1073739322i32;
pub const EVENT_SRV_KEY_NOT_FOUND: i32 = -1073739323i32;
pub const EVENT_SRV_NETWORK_ERROR: i32 = -2147481636i32;
pub const EVENT_SRV_NONPAGED_POOL_LIMIT: i32 = -1073739807i32;
pub const EVENT_SRV_NO_BLOCKING_IO: i32 = -2147481624i32;
pub const EVENT_SRV_NO_FREE_CONNECTIONS: i32 = -2147481626i32;
pub const EVENT_SRV_NO_FREE_RAW_WORK_ITEM: i32 = -2147481625i32;
pub const EVENT_SRV_NO_NONPAGED_POOL: i32 = -1073739805i32;
pub const EVENT_SRV_NO_PAGED_POOL: i32 = -1073739804i32;
pub const EVENT_SRV_NO_TRANSPORTS_BOUND: i32 = -1073739321i32;
pub const EVENT_SRV_NO_VIRTUAL_MEMORY: i32 = -1073739808i32;
pub const EVENT_SRV_NO_WORK_ITEM: i32 = -2147481627i32;
pub const EVENT_SRV_OUT_OF_WORK_ITEM_DOS: i32 = -2147481621i32;
pub const EVENT_SRV_PAGED_POOL_LIMIT: i32 = -1073739806i32;
pub const EVENT_SRV_RESOURCE_SHORTAGE: i32 = -1073739823i32;
pub const EVENT_SRV_SERVICE_FAILED: i32 = -1073739824i32;
pub const EVENT_SRV_TOO_MANY_DOS: i32 = -2147481622i32;
pub const EVENT_SRV_TXF_INIT_FAILED: i32 = -2147481135i32;
pub const EVENT_SRV_UNEXPECTED_DISC: i32 = -1073739819i32;
pub const EVENT_STREAMS_ALLOCB_FAILURE: i32 = -2147479647i32;
pub const EVENT_STREAMS_ALLOCB_FAILURE_CNT: i32 = -2147479646i32;
pub const EVENT_STREAMS_ESBALLOC_FAILURE: i32 = -2147479645i32;
pub const EVENT_STREAMS_ESBALLOC_FAILURE_CNT: i32 = -2147479644i32;
pub const EVENT_STREAMS_STRLOG: i32 = -1073737824i32;
pub const EVENT_TAKE_OWNERSHIP: i32 = -1073734796i32;
pub const EVENT_TCPIP6_STARTED: i32 = 1073744924i32;
pub const EVENT_TCPIP_ADAPTER_REG_FAILURE: i32 = -1073737633i32;
pub const EVENT_TCPIP_ADDRESS_CONFLICT1: i32 = -1073737626i32;
pub const EVENT_TCPIP_ADDRESS_CONFLICT2: i32 = -1073737625i32;
pub const EVENT_TCPIP_AUTOCONFIGURED_ADDRESS_LIMIT_REACHED: i32 = -2147479444i32;
pub const EVENT_TCPIP_AUTOCONFIGURED_ROUTE_LIMIT_REACHED: i32 = -2147479443i32;
pub const EVENT_TCPIP_CREATE_DEVICE_FAILED: i32 = -1073737724i32;
pub const EVENT_TCPIP_DHCP_INIT_FAILED: i32 = -2147479458i32;
pub const EVENT_TCPIP_INTERFACE_BIND_FAILURE: i32 = -1073737617i32;
pub const EVENT_TCPIP_INVALID_ADDRESS: i32 = -1073737637i32;
pub const EVENT_TCPIP_INVALID_DEFAULT_GATEWAY: i32 = -2147479456i32;
pub const EVENT_TCPIP_INVALID_MASK: i32 = -1073737636i32;
pub const EVENT_TCPIP_IPV4_UNINSTALLED: i32 = 1073746027i32;
pub const EVENT_TCPIP_IP_INIT_FAILED: i32 = -1073737628i32;
pub const EVENT_TCPIP_MEDIA_CONNECT: i32 = 1073746025i32;
pub const EVENT_TCPIP_MEDIA_DISCONNECT: i32 = 1073746026i32;
pub const EVENT_TCPIP_NO_ADAPTER_RESOURCES: i32 = -1073737635i32;
pub const EVENT_TCPIP_NO_ADDRESS_LIST: i32 = -1073737631i32;
pub const EVENT_TCPIP_NO_BINDINGS: i32 = -1073737629i32;
pub const EVENT_TCPIP_NO_MASK: i32 = -1073737638i32;
pub const EVENT_TCPIP_NO_MASK_LIST: i32 = -1073737630i32;
pub const EVENT_TCPIP_NO_RESOURCES_FOR_INIT: i32 = -1073737723i32;
pub const EVENT_TCPIP_NTE_CONTEXT_LIST_FAILURE: i32 = -1073737624i32;
pub const EVENT_TCPIP_OUT_OF_ORDER_FRAGMENTS_EXCEEDED: i32 = -2147479442i32;
pub const EVENT_TCPIP_PCF_CLEAR_FILTER_FAILURE: i32 = -1073737530i32;
pub const EVENT_TCPIP_PCF_MISSING_CAPABILITY: i32 = -2147479357i32;
pub const EVENT_TCPIP_PCF_MULTICAST_OID_ISSUE: i32 = -2147479358i32;
pub const EVENT_TCPIP_PCF_NO_ARP_FILTER: i32 = -2147479355i32;
pub const EVENT_TCPIP_PCF_SET_FILTER_FAILURE: i32 = -2147479356i32;
pub const EVENT_TCPIP_TCP_CONNECTIONS_PERF_IMPACTED: i32 = -2147479418i32;
pub const EVENT_TCPIP_TCP_CONNECT_LIMIT_REACHED: i32 = -2147479422i32;
pub const EVENT_TCPIP_TCP_GLOBAL_EPHEMERAL_PORT_SPACE_EXHAUSTED: i32 = -2147479417i32;
pub const EVENT_TCPIP_TCP_INIT_FAILED: i32 = -1073737599i32;
pub const EVENT_TCPIP_TCP_MPP_ATTACKS_DETECTED: i32 = -2147479419i32;
pub const EVENT_TCPIP_TCP_TIME_WAIT_COLLISION: i32 = -2147479421i32;
pub const EVENT_TCPIP_TCP_WSD_WS_RESTRICTED: i32 = -2147479420i32;
pub const EVENT_TCPIP_TOO_MANY_GATEWAYS: i32 = -2147479451i32;
pub const EVENT_TCPIP_TOO_MANY_NETS: i32 = -1073737639i32;
pub const EVENT_TCPIP_UDP_GLOBAL_EPHEMERAL_PORT_SPACE_EXHAUSTED: i32 = -2147479382i32;
pub const EVENT_TCPIP_UDP_LIMIT_REACHED: i32 = -2147479383i32;
pub const EVENT_TRANSACT_INVALID: i32 = -1073734812i32;
pub const EVENT_TRANSACT_TIMEOUT: i32 = -1073734813i32;
pub const EVENT_TRANSPORT_ADAPTER_NOT_FOUND: i32 = -1073732818i32;
pub const EVENT_TRANSPORT_BAD_PROTOCOL: i32 = 1073750835i32;
pub const EVENT_TRANSPORT_BINDING_FAILED: i32 = -1073732819i32;
pub const EVENT_TRANSPORT_QUERY_OID_FAILED: i32 = -1073732816i32;
pub const EVENT_TRANSPORT_REGISTER_FAILED: i32 = -1073732820i32;
pub const EVENT_TRANSPORT_RESOURCE_LIMIT: i32 = -2147474646i32;
pub const EVENT_TRANSPORT_RESOURCE_POOL: i32 = -2147474647i32;
pub const EVENT_TRANSPORT_RESOURCE_SPECIFIC: i32 = -2147474645i32;
pub const EVENT_TRANSPORT_SET_OID_FAILED: i32 = -1073732817i32;
pub const EVENT_TRANSPORT_TOO_MANY_LINKS: i32 = 1073750834i32;
pub const EVENT_TRANSPORT_TRANSFER_DATA: i32 = 1073750833i32;
pub const EVENT_TRK_INTERNAL_ERROR: i32 = -1073729324i32;
pub const EVENT_TRK_SERVICE_CORRUPT_LOG: i32 = -1073729321i32;
pub const EVENT_TRK_SERVICE_DUPLICATE_VOLIDS: i32 = 1073754331i32;
pub const EVENT_TRK_SERVICE_MOVE_QUOTA_EXCEEDED: i32 = -2147471140i32;
pub const EVENT_TRK_SERVICE_START_FAILURE: i32 = -1073729322i32;
pub const EVENT_TRK_SERVICE_START_SUCCESS: i32 = 1073754325i32;
pub const EVENT_TRK_SERVICE_VOLUME_CLAIM: i32 = 1073754330i32;
pub const EVENT_TRK_SERVICE_VOLUME_CREATE: i32 = 1073754329i32;
pub const EVENT_TRK_SERVICE_VOL_QUOTA_EXCEEDED: i32 = -2147471144i32;
pub const EVENT_UP_DRIVER_ON_MP: i32 = -1073735724i32;
pub const EVENT_WEBCLIENT_CLOSE_DELETE_FAILED: i32 = -2147468746i32;
pub const EVENT_WEBCLIENT_CLOSE_PROPPATCH_FAILED: i32 = -2147468745i32;
pub const EVENT_WEBCLIENT_CLOSE_PUT_FAILED: i32 = -2147468747i32;
pub const EVENT_WEBCLIENT_SETINFO_PROPPATCH_FAILED: i32 = -2147468744i32;
pub const EVENT_WINNAT_SESSION_LIMIT_REACHED: i32 = -2147466648i32;
pub const EVENT_WINSOCK_CLOSESOCKET_STUCK: i32 = -2147467646i32;
pub const EVENT_WINSOCK_TDI_FILTER_DETECTED: i32 = -2147467647i32;
pub const EVENT_WSK_OWNINGTHREAD_PARAMETER_IGNORED: i32 = -1073725824i32;
pub const EVLEN: u32 = 16u32;
pub const EXTRA_EXIT_POINT: i32 = -1073727524i32;
pub const EXTRA_EXIT_POINT_DELETED: i32 = -1073727520i32;
pub const EXTRA_EXIT_POINT_NOT_DELETED: i32 = -1073727519i32;
pub const EXTRA_VOLUME: i32 = -1073727521i32;
pub const EXTRA_VOLUME_DELETED: i32 = -1073727514i32;
pub const EXTRA_VOLUME_NOT_DELETED: i32 = -1073727513i32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct FLAT_STRING(i32);
#[repr(transparent)]
pub struct FORCE_LEVEL_FLAGS(pub u32);
pub const USE_NOFORCE: FORCE_LEVEL_FLAGS = FORCE_LEVEL_FLAGS(0u32);
pub const USE_FORCE: FORCE_LEVEL_FLAGS = FORCE_LEVEL_FLAGS(1u32);
pub const USE_LOTS_OF_FORCE: FORCE_LEVEL_FLAGS = FORCE_LEVEL_FLAGS(2u32);
pub const GNLEN: u32 = 256u32;
pub const GROUPIDMASK: u32 = 32768u32;
pub const GROUP_ALL_PARMNUM: u32 = 0u32;
pub const GROUP_ATTRIBUTES_PARMNUM: u32 = 3u32;
pub const GROUP_COMMENT_PARMNUM: u32 = 2u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct GROUP_INFO_0(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct GROUP_INFO_1(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct GROUP_INFO_1002(i32);
#[repr(C)]
pub struct GROUP_INFO_1005(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct GROUP_INFO_2(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct GROUP_INFO_3(i32);
pub const GROUP_NAME_PARMNUM: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct GROUP_USERS_INFO_0(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct GROUP_USERS_INFO_1(i32);
#[repr(C)]
pub struct HARDWARE_ADDRESS(i32);
pub const HARDWARE_ADDRESS_LENGTH: u32 = 6u32;
#[repr(C)]
pub struct HLOG(i32);
#[repr(transparent)]
pub struct IEnumNetCfgBindingInterface(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumNetCfgBindingPath(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumNetCfgComponent(pub *mut ::core::ffi::c_void);
pub const INTERFACE_INFO_REVISION_1: u32 = 1u32;
pub const INVALID_TRACEID: u32 = 4294967295u32;
#[repr(transparent)]
pub struct INetCfg(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INetCfgBindingInterface(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INetCfgBindingPath(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INetCfgClass(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INetCfgClassSetup(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INetCfgClassSetup2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INetCfgComponent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INetCfgComponentBindings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INetCfgComponentControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INetCfgComponentNotifyBinding(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INetCfgComponentNotifyGlobal(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INetCfgComponentPropertyUi(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INetCfgComponentSetup(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INetCfgComponentSysPrep(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INetCfgComponentUpperEdge(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INetCfgLock(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INetCfgPnpReconfigCallback(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INetCfgSysPrep(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INetLanConnectionUiInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INetRasConnectionIpUiInfo(pub *mut ::core::ffi::c_void);
pub const IPX_PROTOCOL_BASE: u32 = 131071u32;
pub const IPX_PROTOCOL_RIP: u32 = 131072u32;
#[repr(transparent)]
pub struct IProvisioningDomain(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProvisioningProfileWireless(pub *mut ::core::ffi::c_void);
pub const IR_PROMISCUOUS: u32 = 0u32;
pub const IR_PROMISCUOUS_MULTICAST: u32 = 1u32;
pub const JOB_ADD_CURRENT_DATE: u32 = 8u32;
pub const JOB_EXEC_ERROR: u32 = 2u32;
pub const JOB_NONINTERACTIVE: u32 = 16u32;
pub const JOB_RUNS_TODAY: u32 = 4u32;
pub const JOB_RUN_PERIODICALLY: u32 = 1u32;
pub const KNOWLEDGE_INCONSISTENCY_DETECTED: i32 = -1073727511i32;
pub const LG_INCLUDE_INDIRECT: u32 = 1u32;
pub const LM20_CNLEN: u32 = 15u32;
pub const LM20_DEVLEN: u32 = 8u32;
pub const LM20_DNLEN: u32 = 15u32;
pub const LM20_GNLEN: u32 = 20u32;
pub const LM20_MAXCOMMENTSZ: u32 = 48u32;
pub const LM20_NNLEN: u32 = 12u32;
pub const LM20_PATHLEN: u32 = 256u32;
pub const LM20_PWLEN: u32 = 14u32;
pub const LM20_QNLEN: u32 = 12u32;
pub const LM20_SERVICE_ACTIVE: u32 = 0u32;
pub const LM20_SERVICE_CONTINUE_PENDING: u32 = 4u32;
pub const LM20_SERVICE_PAUSED: u32 = 12u32;
pub const LM20_SERVICE_PAUSE_PENDING: u32 = 8u32;
pub const LM20_SNLEN: u32 = 15u32;
pub const LM20_STXTLEN: u32 = 63u32;
pub const LM20_UNCLEN: u32 = 17u32;
pub const LM20_UNLEN: u32 = 20u32;
pub const LM_REDIR_FAILURE: i32 = 1073756225i32;
pub const LOCALGROUP_COMMENT_PARMNUM: u32 = 2u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct LOCALGROUP_INFO_0(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct LOCALGROUP_INFO_1(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct LOCALGROUP_INFO_1002(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct LOCALGROUP_MEMBERS_INFO_0(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[repr(C)]
pub struct LOCALGROUP_MEMBERS_INFO_1(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[repr(C)]
pub struct LOCALGROUP_MEMBERS_INFO_2(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct LOCALGROUP_MEMBERS_INFO_3(i32);
pub const LOCALGROUP_NAME_PARMNUM: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct LOCALGROUP_USERS_INFO_0(i32);
pub const LOGFLAGS_BACKWARD: u32 = 1u32;
pub const LOGFLAGS_FORWARD: u32 = 0u32;
pub const LOGFLAGS_SEEK: u32 = 2u32;
pub const LOWER_GET_HINT_MASK: u32 = 65280u32;
pub const LOWER_HINT_MASK: u32 = 255u32;
pub const MACHINE_UNJOINED: i32 = -1073727507i32;
pub const MAJOR_VERSION_MASK: u32 = 15u32;
pub const MAXCOMMENTSZ: u32 = 256u32;
pub const MAXPERMENTRIES: u32 = 64u32;
pub const MAX_LANMAN_MESSAGE_ID: u32 = 5899u32;
pub const MAX_NERR: u32 = 2999u32;
pub const MAX_PASSWD_LEN: u32 = 256u32;
pub const MAX_PREFERRED_LENGTH: u32 = 4294967295u32;
pub const MAX_PROTOCOL_DLL_LEN: u32 = 48u32;
pub const MAX_PROTOCOL_NAME_LEN: u32 = 40u32;
pub const MFE_BOUNDARY_REACHED: u32 = 6u32;
pub const MFE_IIF: u32 = 8u32;
pub const MFE_NOT_FORWARDING: u32 = 2u32;
pub const MFE_NOT_LAST_HOP: u32 = 10u32;
pub const MFE_NO_ERROR: u32 = 0u32;
pub const MFE_NO_MULTICAST: u32 = 7u32;
pub const MFE_NO_ROUTE: u32 = 9u32;
pub const MFE_NO_SPACE: u32 = 13u32;
pub const MFE_OIF_PRUNED: u32 = 5u32;
pub const MFE_OLD_ROUTER: u32 = 11u32;
pub const MFE_PROHIBITED: u32 = 12u32;
pub const MFE_PRUNED_UPSTREAM: u32 = 4u32;
pub const MFE_REACHED_CORE: u32 = 1u32;
pub const MFE_WRONG_IF: u32 = 3u32;
pub const MIN_LANMAN_MESSAGE_ID: u32 = 2100u32;
pub const MISSING_EXIT_POINT: i32 = -1073727523i32;
pub const MISSING_EXIT_POINT_CREATED: i32 = -1073727518i32;
pub const MISSING_EXIT_POINT_NOT_CREATED: i32 = -1073727517i32;
pub const MISSING_VOLUME: i32 = -1073727522i32;
pub const MISSING_VOLUME_CREATED: i32 = -1073727516i32;
pub const MISSING_VOLUME_NOT_CREATED: i32 = -1073727515i32;
pub const MODALS_DOMAIN_ID_PARMNUM: u32 = 9u32;
pub const MODALS_DOMAIN_NAME_PARMNUM: u32 = 8u32;
pub const MODALS_FORCE_LOGOFF_PARMNUM: u32 = 4u32;
pub const MODALS_LOCKOUT_DURATION_PARMNUM: u32 = 10u32;
pub const MODALS_LOCKOUT_OBSERVATION_WINDOW_PARMNUM: u32 = 11u32;
pub const MODALS_LOCKOUT_THRESHOLD_PARMNUM: u32 = 12u32;
pub const MODALS_MAX_PASSWD_AGE_PARMNUM: u32 = 2u32;
pub const MODALS_MIN_PASSWD_AGE_PARMNUM: u32 = 3u32;
pub const MODALS_MIN_PASSWD_LEN_PARMNUM: u32 = 1u32;
pub const MODALS_PASSWD_HIST_LEN_PARMNUM: u32 = 5u32;
pub const MODALS_PRIMARY_PARMNUM: u32 = 7u32;
pub const MODALS_ROLE_PARMNUM: u32 = 6u32;
#[repr(C)]
pub struct MPR_PROTOCOL_0(i32);
pub const MRINFO_DISABLED_FLAG: u32 = 32u32;
pub const MRINFO_DOWN_FLAG: u32 = 16u32;
pub const MRINFO_LEAF_FLAG: u32 = 128u32;
pub const MRINFO_PIM_FLAG: u32 = 4u32;
pub const MRINFO_QUERIER_FLAG: u32 = 64u32;
pub const MRINFO_TUNNEL_FLAG: u32 = 1u32;
#[repr(C)]
pub struct MSA_INFO_0(i32);
#[repr(transparent)]
pub struct MSA_INFO_LEVEL(pub i32);
pub const MsaInfoLevel0: MSA_INFO_LEVEL = MSA_INFO_LEVEL(0i32);
pub const MsaInfoLevelMax: MSA_INFO_LEVEL = MSA_INFO_LEVEL(1i32);
#[repr(transparent)]
pub struct MSA_INFO_STATE(pub i32);
pub const MsaInfoNotExist: MSA_INFO_STATE = MSA_INFO_STATE(1i32);
pub const MsaInfoNotService: MSA_INFO_STATE = MSA_INFO_STATE(2i32);
pub const MsaInfoCannotInstall: MSA_INFO_STATE = MSA_INFO_STATE(3i32);
pub const MsaInfoCanInstall: MSA_INFO_STATE = MSA_INFO_STATE(4i32);
pub const MsaInfoInstalled: MSA_INFO_STATE = MSA_INFO_STATE(5i32);
pub const MSGNAME_FORWARDED_FROM: u32 = 16u32;
pub const MSGNAME_FORWARDED_TO: u32 = 4u32;
pub const MSGNAME_NOT_FORWARDED: u32 = 0u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct MSG_INFO_0(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct MSG_INFO_1(i32);
pub const MS_ROUTER_VERSION: u32 = 1536u32;
#[repr(transparent)]
pub struct NCPNP_RECONFIG_LAYER(pub i32);
pub const NCRL_NDIS: NCPNP_RECONFIG_LAYER = NCPNP_RECONFIG_LAYER(1i32);
pub const NCRL_TDI: NCPNP_RECONFIG_LAYER = NCPNP_RECONFIG_LAYER(2i32);
#[repr(transparent)]
pub struct NCRP_FLAGS(pub i32);
pub const NCRP_QUERY_PROPERTY_UI: NCRP_FLAGS = NCRP_FLAGS(1i32);
pub const NCRP_SHOW_PROPERTY_UI: NCRP_FLAGS = NCRP_FLAGS(2i32);
pub const NELOG_AT_Exec_Err: u32 = 3178u32;
pub const NELOG_AT_cannot_read: u32 = 3174u32;
pub const NELOG_AT_cannot_write: u32 = 3129u32;
pub const NELOG_AT_sched_err: u32 = 3175u32;
pub const NELOG_AT_schedule_file_created: u32 = 3176u32;
pub const NELOG_Access_File_Bad: u32 = 3122u32;
pub const NELOG_Build_Name: u32 = 3170u32;
pub const NELOG_Cant_Make_Msg_File: u32 = 3130u32;
pub const NELOG_DiskFT: u32 = 3221u32;
pub const NELOG_DriverNotLoaded: u32 = 5727u32;
pub const NELOG_Entries_Lost: u32 = 3114u32;
pub const NELOG_Error_in_DLL: u32 = 3256u32;
pub const NELOG_Exec_Netservr_NoMem: u32 = 3131u32;
pub const NELOG_FT_ErrLog_Too_Large: u32 = 3258u32;
pub const NELOG_FT_Update_In_Progress: u32 = 3259u32;
pub const NELOG_FailedToGetComputerName: u32 = 5726u32;
pub const NELOG_FailedToRegisterSC: u32 = 5724u32;
pub const NELOG_FailedToSetServiceStatus: u32 = 5725u32;
pub const NELOG_File_Changed: u32 = 3253u32;
pub const NELOG_Files_Dont_Fit: u32 = 3254u32;
pub const NELOG_HardErr_From_Server: u32 = 3182u32;
pub const NELOG_HotFix: u32 = 3181u32;
pub const NELOG_Init_Chardev_Err: u32 = 3124u32;
pub const NELOG_Init_Exec_Fail: u32 = 3105u32;
pub const NELOG_Init_OpenCreate_Err: u32 = 3110u32;
pub const NELOG_Init_Seg_Overflow: u32 = 3120u32;
pub const NELOG_Internal_Error: u32 = 3100u32;
pub const NELOG_Invalid_Config_File: u32 = 3252u32;
pub const NELOG_Invalid_Config_Line: u32 = 3251u32;
pub const NELOG_Ioctl_Error: u32 = 3108u32;
pub const NELOG_Joined_Domain: u32 = 3260u32;
pub const NELOG_Joined_Workgroup: u32 = 3261u32;
pub const NELOG_Lazy_Write_Err: u32 = 3180u32;
pub const NELOG_LocalSecFail1: u32 = 3183u32;
pub const NELOG_LocalSecFail2: u32 = 3184u32;
pub const NELOG_LocalSecFail3: u32 = 3185u32;
pub const NELOG_LocalSecGeneralFail: u32 = 3186u32;
pub const NELOG_Mail_Slt_Err: u32 = 3173u32;
pub const NELOG_Mailslot_err: u32 = 3127u32;
pub const NELOG_Message_Send: u32 = 3172u32;
pub const NELOG_Missing_Parameter: u32 = 3250u32;
pub const NELOG_Msg_Log_Err: u32 = 3150u32;
pub const NELOG_Msg_Sem_Shutdown: u32 = 3141u32;
pub const NELOG_Msg_Shutdown: u32 = 3140u32;
pub const NELOG_Msg_Unexpected_SMB_Type: u32 = 3152u32;
pub const NELOG_Name_Expansion: u32 = 3171u32;
pub const NELOG_Ncb_Error: u32 = 3106u32;
pub const NELOG_Ncb_TooManyErr: u32 = 3126u32;
pub const NELOG_NetBios: u32 = 3111u32;
pub const NELOG_NetLogonFailedToInitializeAuthzRm: u32 = 5821u32;
pub const NELOG_NetLogonFailedToInitializeRPCSD: u32 = 5822u32;
pub const NELOG_NetWkSta_Internal_Error: u32 = 3190u32;
pub const NELOG_NetWkSta_NCB_Err: u32 = 3195u32;
pub const NELOG_NetWkSta_No_Resource: u32 = 3191u32;
pub const NELOG_NetWkSta_Reset_Err: u32 = 3197u32;
pub const NELOG_NetWkSta_SMB_Err: u32 = 3192u32;
pub const NELOG_NetWkSta_Stuck_VC_Err: u32 = 3194u32;
pub const NELOG_NetWkSta_Too_Many: u32 = 3198u32;
pub const NELOG_NetWkSta_VC_Err: u32 = 3193u32;
pub const NELOG_NetWkSta_Write_Behind_Err: u32 = 3196u32;
pub const NELOG_Net_Not_Started: u32 = 3107u32;
pub const NELOG_NetlogonAddNameFailure: u32 = 5741u32;
pub const NELOG_NetlogonAuthDCFail: u32 = 3210u32;
pub const NELOG_NetlogonAuthDomainDowngraded: u32 = 5791u32;
pub const NELOG_NetlogonAuthNoDomainController: u32 = 5719u32;
pub const NELOG_NetlogonAuthNoTrustLsaSecret: u32 = 5720u32;
pub const NELOG_NetlogonAuthNoTrustSamAccount: u32 = 5721u32;
pub const NELOG_NetlogonAuthNoUplevelDomainController: u32 = 5790u32;
pub const NELOG_NetlogonBadSiteName: u32 = 5779u32;
pub const NELOG_NetlogonBadSubnetName: u32 = 5780u32;
pub const NELOG_NetlogonBrowserDriver: u32 = 5740u32;
pub const NELOG_NetlogonChangeLogCorrupt: u32 = 5705u32;
pub const NELOG_NetlogonDcOldSiteCovered: u32 = 5794u32;
pub const NELOG_NetlogonDcSiteCovered: u32 = 5784u32;
pub const NELOG_NetlogonDcSiteNotCovered: u32 = 5785u32;
pub const NELOG_NetlogonDcSiteNotCoveredAuto: u32 = 5795u32;
pub const NELOG_NetlogonDnsDeregAborted: u32 = 5808u32;
pub const NELOG_NetlogonDnsHostNameLowerCasingFailed: u32 = 5825u32;
pub const NELOG_NetlogonDownLevelLogoffFailed: u32 = 5708u32;
pub const NELOG_NetlogonDownLevelLogonFailed: u32 = 5707u32;
pub const NELOG_NetlogonDuplicateMachineAccounts: u32 = 5738u32;
pub const NELOG_NetlogonDynamicDnsDeregisterFailure: u32 = 5775u32;
pub const NELOG_NetlogonDynamicDnsFailure: u32 = 5782u32;
pub const NELOG_NetlogonDynamicDnsRegisterFailure: u32 = 5774u32;
pub const NELOG_NetlogonDynamicDnsServerFailure: u32 = 5781u32;
pub const NELOG_NetlogonFailedAccountDelta: u32 = 5735u32;
pub const NELOG_NetlogonFailedDnsHostNameUpdate: u32 = 5789u32;
pub const NELOG_NetlogonFailedDomainDelta: u32 = 5729u32;
pub const NELOG_NetlogonFailedFileCreate: u32 = 5776u32;
pub const NELOG_NetlogonFailedGlobalGroupDelta: u32 = 5730u32;
pub const NELOG_NetlogonFailedLocalGroupDelta: u32 = 5731u32;
pub const NELOG_NetlogonFailedPolicyDelta: u32 = 5733u32;
pub const NELOG_NetlogonFailedPrimary: u32 = 3223u32;
pub const NELOG_NetlogonFailedSecretDelta: u32 = 5736u32;
pub const NELOG_NetlogonFailedSpnUpdate: u32 = 5788u32;
pub const NELOG_NetlogonFailedToAddAuthzRpcInterface: u32 = 5820u32;
pub const NELOG_NetlogonFailedToAddRpcInterface: u32 = 5702u32;
pub const NELOG_NetlogonFailedToCreateShare: u32 = 5706u32;
pub const NELOG_NetlogonFailedToReadMailslot: u32 = 5703u32;
pub const NELOG_NetlogonFailedToRegisterSC: u32 = 5704u32;
pub const NELOG_NetlogonFailedToUpdateTrustList: u32 = 5701u32;
pub const NELOG_NetlogonFailedTrustedDomainDelta: u32 = 5734u32;
pub const NELOG_NetlogonFailedUserDelta: u32 = 5732u32;
pub const NELOG_NetlogonFullSyncCallFailed: u32 = 5714u32;
pub const NELOG_NetlogonFullSyncCallSuccess: u32 = 5713u32;
pub const NELOG_NetlogonFullSyncFailed: u32 = 5718u32;
pub const NELOG_NetlogonFullSyncSuccess: u32 = 5717u32;
pub const NELOG_NetlogonGcOldSiteCovered: u32 = 5796u32;
pub const NELOG_NetlogonGcSiteCovered: u32 = 5786u32;
pub const NELOG_NetlogonGcSiteNotCovered: u32 = 5787u32;
pub const NELOG_NetlogonGcSiteNotCoveredAuto: u32 = 5797u32;
pub const NELOG_NetlogonGetSubnetToSite: u32 = 5777u32;
pub const NELOG_NetlogonInvalidDwordParameterValue: u32 = 5804u32;
pub const NELOG_NetlogonInvalidGenericParameterValue: u32 = 5803u32;
pub const NELOG_NetlogonLanmanBdcsNotAllowed: u32 = 5772u32;
pub const NELOG_NetlogonMachinePasswdSetSucceeded: u32 = 5823u32;
pub const NELOG_NetlogonMsaPasswdSetSucceeded: u32 = 5824u32;
pub const NELOG_NetlogonNTLogoffFailed: u32 = 5710u32;
pub const NELOG_NetlogonNTLogonFailed: u32 = 5709u32;
pub const NELOG_NetlogonNdncOldSiteCovered: u32 = 5798u32;
pub const NELOG_NetlogonNdncSiteCovered: u32 = 5792u32;
pub const NELOG_NetlogonNdncSiteNotCovered: u32 = 5793u32;
pub const NELOG_NetlogonNdncSiteNotCoveredAuto: u32 = 5799u32;
pub const NELOG_NetlogonNoAddressToSiteMapping: u32 = 5802u32;
pub const NELOG_NetlogonNoDynamicDns: u32 = 5773u32;
pub const NELOG_NetlogonNoDynamicDnsManual: u32 = 5806u32;
pub const NELOG_NetlogonNoSiteForClient: u32 = 5778u32;
pub const NELOG_NetlogonNoSiteForClients: u32 = 5807u32;
pub const NELOG_NetlogonPartialSiteMappingForClients: u32 = 5810u32;
pub const NELOG_NetlogonPartialSyncCallFailed: u32 = 5712u32;
pub const NELOG_NetlogonPartialSyncCallSuccess: u32 = 5711u32;
pub const NELOG_NetlogonPartialSyncFailed: u32 = 5716u32;
pub const NELOG_NetlogonPartialSyncSuccess: u32 = 5715u32;
pub const NELOG_NetlogonPasswdSetFailed: u32 = 3224u32;
pub const NELOG_NetlogonRejectedRemoteDynamicDnsDeregister: u32 = 5814u32;
pub const NELOG_NetlogonRejectedRemoteDynamicDnsRegister: u32 = 5813u32;
pub const NELOG_NetlogonRemoteDynamicDnsDeregisterFailure: u32 = 5812u32;
pub const NELOG_NetlogonRemoteDynamicDnsRegisterFailure: u32 = 5811u32;
pub const NELOG_NetlogonRemoteDynamicDnsUpdateRequestFailure: u32 = 5815u32;
pub const NELOG_NetlogonRequireSignOrSealError: u32 = 3227u32;
pub const NELOG_NetlogonRpcCallCancelled: u32 = 5783u32;
pub const NELOG_NetlogonRpcPortRequestFailure: u32 = 5809u32;
pub const NELOG_NetlogonSSIInitError: u32 = 5700u32;
pub const NELOG_NetlogonServerAuthFailed: u32 = 5722u32;
pub const NELOG_NetlogonServerAuthFailedNoAccount: u32 = 5805u32;
pub const NELOG_NetlogonServerAuthNoTrustSamAccount: u32 = 5723u32;
pub const NELOG_NetlogonSessionTypeWrong: u32 = 5770u32;
pub const NELOG_NetlogonSpnCrackNamesFailure: u32 = 5801u32;
pub const NELOG_NetlogonSpnMultipleSamAccountNames: u32 = 5800u32;
pub const NELOG_NetlogonSyncError: u32 = 3226u32;
pub const NELOG_NetlogonSystemError: u32 = 5737u32;
pub const NELOG_NetlogonTooManyGlobalGroups: u32 = 5739u32;
pub const NELOG_NetlogonTrackingError: u32 = 3225u32;
pub const NELOG_NetlogonUserValidationReqInitialTimeOut: u32 = 5816u32;
pub const NELOG_NetlogonUserValidationReqRecurringTimeOut: u32 = 5817u32;
pub const NELOG_NetlogonUserValidationReqWaitInitialWarning: u32 = 5818u32;
pub const NELOG_NetlogonUserValidationReqWaitRecurringWarning: u32 = 5819u32;
pub const NELOG_NoTranportLoaded: u32 = 5728u32;
pub const NELOG_OEM_Code: u32 = 3299u32;
pub const NELOG_ReleaseMem_Alert: u32 = 3128u32;
pub const NELOG_Remote_API: u32 = 3125u32;
pub const NELOG_ReplAccessDenied: u32 = 3222u32;
pub const NELOG_ReplBadExport: u32 = 3219u32;
pub const NELOG_ReplBadImport: u32 = 3218u32;
pub const NELOG_ReplBadMsg: u32 = 3215u32;
pub const NELOG_ReplCannotMasterDir: u32 = 3207u32;
pub const NELOG_ReplLogonFailed: u32 = 3211u32;
pub const NELOG_ReplLostMaster: u32 = 3209u32;
pub const NELOG_ReplMaxFiles: u32 = 3213u32;
pub const NELOG_ReplMaxTreeDepth: u32 = 3214u32;
pub const NELOG_ReplNetErr: u32 = 3212u32;
pub const NELOG_ReplSignalFileErr: u32 = 3220u32;
pub const NELOG_ReplSysErr: u32 = 3216u32;
pub const NELOG_ReplUpdateError: u32 = 3208u32;
pub const NELOG_ReplUserCurDir: u32 = 3206u32;
pub const NELOG_ReplUserLoged: u32 = 3217u32;
pub const NELOG_Resource_Shortage: u32 = 3101u32;
pub const NELOG_RplAdapterResource: u32 = 5756u32;
pub const NELOG_RplBackupDatabase: u32 = 5765u32;
pub const NELOG_RplCheckConfigs: u32 = 5760u32;
pub const NELOG_RplCheckSecurity: u32 = 5764u32;
pub const NELOG_RplCreateProfiles: u32 = 5761u32;
pub const NELOG_RplFileCopy: u32 = 5757u32;
pub const NELOG_RplFileDelete: u32 = 5758u32;
pub const NELOG_RplFilePerms: u32 = 5759u32;
pub const NELOG_RplInitDatabase: u32 = 5766u32;
pub const NELOG_RplInitRestoredDatabase: u32 = 5769u32;
pub const NELOG_RplMessages: u32 = 5742u32;
pub const NELOG_RplRegistry: u32 = 5762u32;
pub const NELOG_RplReplaceRPLDISK: u32 = 5763u32;
pub const NELOG_RplRestoreDatabaseFailure: u32 = 5767u32;
pub const NELOG_RplRestoreDatabaseSuccess: u32 = 5768u32;
pub const NELOG_RplSystem: u32 = 5744u32;
pub const NELOG_RplUpgradeDBTo40: u32 = 5771u32;
pub const NELOG_RplWkstaBbcFile: u32 = 5751u32;
pub const NELOG_RplWkstaFileChecksum: u32 = 5749u32;
pub const NELOG_RplWkstaFileLineCount: u32 = 5750u32;
pub const NELOG_RplWkstaFileOpen: u32 = 5746u32;
pub const NELOG_RplWkstaFileRead: u32 = 5747u32;
pub const NELOG_RplWkstaFileSize: u32 = 5752u32;
pub const NELOG_RplWkstaInternal: u32 = 5753u32;
pub const NELOG_RplWkstaMemory: u32 = 5748u32;
pub const NELOG_RplWkstaNetwork: u32 = 5755u32;
pub const NELOG_RplWkstaTimeout: u32 = 5745u32;
pub const NELOG_RplWkstaWrongVersion: u32 = 5754u32;
pub const NELOG_RplXnsBoot: u32 = 5743u32;
pub const NELOG_SMB_Illegal: u32 = 3112u32;
pub const NELOG_Server_Lock_Failure: u32 = 3132u32;
pub const NELOG_Service_Fail: u32 = 3113u32;
pub const NELOG_Srv_Close_Failure: u32 = 3205u32;
pub const NELOG_Srv_No_Mem_Grow: u32 = 3121u32;
pub const NELOG_Srv_Thread_Failure: u32 = 3204u32;
pub const NELOG_Srvnet_NB_Open: u32 = 3177u32;
pub const NELOG_Srvnet_Not_Started: u32 = 3123u32;
pub const NELOG_System_Error: u32 = 3257u32;
pub const NELOG_System_Semaphore: u32 = 3109u32;
pub const NELOG_UPS_CannotOpenDriver: u32 = 3233u32;
pub const NELOG_UPS_CmdFileConfig: u32 = 3235u32;
pub const NELOG_UPS_CmdFileError: u32 = 3232u32;
pub const NELOG_UPS_CmdFileExec: u32 = 3236u32;
pub const NELOG_UPS_PowerBack: u32 = 3234u32;
pub const NELOG_UPS_PowerOut: u32 = 3230u32;
pub const NELOG_UPS_Shutdown: u32 = 3231u32;
pub const NELOG_Unable_To_Lock_Segment: u32 = 3102u32;
pub const NELOG_Unable_To_Unlock_Segment: u32 = 3103u32;
pub const NELOG_Uninstall_Service: u32 = 3104u32;
pub const NELOG_VIO_POPUP_ERR: u32 = 3151u32;
pub const NELOG_Wksta_Bad_Mailslot_SMB: u32 = 3165u32;
pub const NELOG_Wksta_BiosThreadFailure: u32 = 3162u32;
pub const NELOG_Wksta_Compname: u32 = 3161u32;
pub const NELOG_Wksta_HostTab_Full: u32 = 3164u32;
pub const NELOG_Wksta_Infoseg: u32 = 3160u32;
pub const NELOG_Wksta_IniSeg: u32 = 3163u32;
pub const NELOG_Wksta_SSIRelogon: u32 = 3167u32;
pub const NELOG_Wksta_UASInit: u32 = 3166u32;
pub const NELOG_Wrong_DLL_Version: u32 = 3255u32;
pub const NERR_ACFFileIOFail: u32 = 2229u32;
pub const NERR_ACFNoParent: u32 = 2232u32;
pub const NERR_ACFNoRoom: u32 = 2228u32;
pub const NERR_ACFNotFound: u32 = 2219u32;
pub const NERR_ACFNotLoaded: u32 = 2227u32;
pub const NERR_ACFTooManyLists: u32 = 2230u32;
pub const NERR_AccountExpired: u32 = 2239u32;
pub const NERR_AccountLockedOut: u32 = 2702u32;
pub const NERR_AccountUndefined: u32 = 2238u32;
pub const NERR_AcctLimitExceeded: u32 = 2434u32;
pub const NERR_ActiveConns: u32 = 2402u32;
pub const NERR_AddForwarded: u32 = 2275u32;
pub const NERR_AlertExists: u32 = 2430u32;
pub const NERR_AlreadyCloudDomainJoined: u32 = 2700u32;
pub const NERR_AlreadyExists: u32 = 2276u32;
pub const NERR_AlreadyForwarded: u32 = 2274u32;
pub const NERR_AlreadyLoggedOn: u32 = 2200u32;
pub const NERR_BASE: u32 = 2100u32;
pub const NERR_BadAsgType: u32 = 2251u32;
pub const NERR_BadComponent: u32 = 2356u32;
pub const NERR_BadControlRecv: u32 = 2193u32;
pub const NERR_BadDest: u32 = 2382u32;
pub const NERR_BadDev: u32 = 2341u32;
pub const NERR_BadDevString: u32 = 2340u32;
pub const NERR_BadDomainJoinInfo: u32 = 2712u32;
pub const NERR_BadDosFunction: u32 = 2502u32;
pub const NERR_BadDosRetCode: u32 = 2500u32;
pub const NERR_BadEventName: u32 = 2143u32;
pub const NERR_BadFileCheckSum: u32 = 2504u32;
pub const NERR_BadOfflineJoinInfo: u32 = 2710u32;
pub const NERR_BadPassword: u32 = 2203u32;
pub const NERR_BadPasswordCore: u32 = 2403u32;
pub const NERR_BadQueueDevString: u32 = 2334u32;
pub const NERR_BadQueuePriority: u32 = 2335u32;
pub const NERR_BadReceive: u32 = 2282u32;
pub const NERR_BadRecipient: u32 = 2433u32;
pub const NERR_BadServiceName: u32 = 2185u32;
pub const NERR_BadServiceProgName: u32 = 2188u32;
pub const NERR_BadSource: u32 = 2381u32;
pub const NERR_BadTransactConfig: u32 = 2141u32;
pub const NERR_BadUasConfig: u32 = 2450u32;
pub const NERR_BadUsername: u32 = 2202u32;
pub const NERR_BrowserConfiguredToNotRun: u32 = 2550u32;
pub const NERR_BrowserNotStarted: u32 = 2139u32;
pub const NERR_BrowserTableIncomplete: u32 = 2319u32;
pub const NERR_BufTooSmall: u32 = 2123u32;
pub const NERR_CallingRplSrvr: u32 = 2515u32;
pub const NERR_CanNotGrowSegment: u32 = 2233u32;
pub const NERR_CanNotGrowUASFile: u32 = 2456u32;
pub const NERR_CannotUnjoinAadDomain: u32 = 2727u32;
pub const NERR_CantConnectRplSrvr: u32 = 2513u32;
pub const NERR_CantCreateJoinInfo: u32 = 2711u32;
pub const NERR_CantLoadOfflineHive: u32 = 2717u32;
pub const NERR_CantOpenImageFile: u32 = 2514u32;
pub const NERR_CantType: u32 = 2357u32;
pub const NERR_CantVerifyHostname: u32 = 2716u32;
pub const NERR_CfgCompNotFound: u32 = 2146u32;
pub const NERR_CfgParamNotFound: u32 = 2147u32;
pub const NERR_ClientNameNotFound: u32 = 2312u32;
pub const NERR_CommDevInUse: u32 = 2343u32;
pub const NERR_ComputerAccountNotFound: u32 = 2697u32;
pub const NERR_ConnectionInsecure: u32 = 2718u32;
pub const NERR_DCNotFound: u32 = 2453u32;
pub const NERR_DS8DCNotFound: u32 = 2722u32;
pub const NERR_DS8DCRequired: u32 = 2720u32;
pub const NERR_DS9DCNotFound: u32 = 2725u32;
pub const NERR_DataTypeInvalid: u32 = 2167u32;
pub const NERR_DatabaseUpToDate: u32 = 2248u32;
pub const NERR_DefaultJoinRequired: u32 = 2694u32;
pub const NERR_DelComputerName: u32 = 2278u32;
pub const NERR_DeleteLater: u32 = 2298u32;
pub const NERR_DestExists: u32 = 2153u32;
pub const NERR_DestIdle: u32 = 2158u32;
pub const NERR_DestInvalidOp: u32 = 2159u32;
pub const NERR_DestInvalidState: u32 = 2162u32;
pub const NERR_DestNoRoom: u32 = 2157u32;
pub const NERR_DestNotFound: u32 = 2152u32;
pub const NERR_DevInUse: u32 = 2404u32;
pub const NERR_DevInvalidOpCode: u32 = 2331u32;
pub const NERR_DevNotFound: u32 = 2332u32;
pub const NERR_DevNotOpen: u32 = 2333u32;
pub const NERR_DevNotRedirected: u32 = 2107u32;
pub const NERR_DeviceIsShared: u32 = 2252u32;
pub const NERR_DeviceNotShared: u32 = 2311u32;
pub const NERR_DeviceShareConflict: u32 = 2318u32;
pub const NERR_DfsAlreadyShared: u32 = 2664u32;
pub const NERR_DfsBadRenamePath: u32 = 2671u32;
pub const NERR_DfsCantCreateJunctionPoint: u32 = 2669u32;
pub const NERR_DfsCantRemoveDfsRoot: u32 = 2682u32;
pub const NERR_DfsCantRemoveLastServerShare: u32 = 2677u32;
pub const NERR_DfsChildOrParentInDfs: u32 = 2683u32;
pub const NERR_DfsCyclicalName: u32 = 2674u32;
pub const NERR_DfsDataIsIdentical: u32 = 2681u32;
pub const NERR_DfsDuplicateService: u32 = 2676u32;
pub const NERR_DfsInconsistent: u32 = 2679u32;
pub const NERR_DfsInternalCorruption: u32 = 2660u32;
pub const NERR_DfsInternalError: u32 = 2690u32;
pub const NERR_DfsLeafVolume: u32 = 2667u32;
pub const NERR_DfsNoSuchServer: u32 = 2673u32;
pub const NERR_DfsNoSuchShare: u32 = 2665u32;
pub const NERR_DfsNoSuchVolume: u32 = 2662u32;
pub const NERR_DfsNotALeafVolume: u32 = 2666u32;
pub const NERR_DfsNotSupportedInServerDfs: u32 = 2675u32;
pub const NERR_DfsServerNotDfsAware: u32 = 2670u32;
pub const NERR_DfsServerUpgraded: u32 = 2680u32;
pub const NERR_DfsVolumeAlreadyExists: u32 = 2663u32;
pub const NERR_DfsVolumeDataCorrupt: u32 = 2661u32;
pub const NERR_DfsVolumeHasMultipleServers: u32 = 2668u32;
pub const NERR_DfsVolumeIsInterDfs: u32 = 2678u32;
pub const NERR_DfsVolumeIsOffline: u32 = 2672u32;
pub const NERR_DifferentServers: u32 = 2383u32;
pub const NERR_DriverNotFound: u32 = 2166u32;
pub const NERR_DupNameReboot: u32 = 2144u32;
pub const NERR_DuplicateName: u32 = 2297u32;
pub const NERR_DuplicateShare: u32 = 2118u32;
pub const NERR_ErrCommRunSrv: u32 = 2389u32;
pub const NERR_ErrorExecingGhost: u32 = 2391u32;
pub const NERR_ExecFailure: u32 = 2315u32;
pub const NERR_FileIdNotFound: u32 = 2314u32;
pub const NERR_GroupExists: u32 = 2223u32;
pub const NERR_GroupNotFound: u32 = 2220u32;
pub const NERR_GrpMsgProcessor: u32 = 2280u32;
pub const NERR_ImageParamErr: u32 = 2508u32;
pub const NERR_InUseBySpooler: u32 = 2342u32;
pub const NERR_IncompleteDel: u32 = 2299u32;
pub const NERR_InternalError: u32 = 2140u32;
pub const NERR_InvalidAPI: u32 = 2142u32;
pub const NERR_InvalidComputer: u32 = 2351u32;
pub const NERR_InvalidDatabase: u32 = 2247u32;
pub const NERR_InvalidDevice: u32 = 2294u32;
pub const NERR_InvalidLana: u32 = 2400u32;
pub const NERR_InvalidLogSeek: u32 = 2440u32;
pub const NERR_InvalidLogonHours: u32 = 2241u32;
pub const NERR_InvalidMachineNameForJoin: u32 = 2724u32;
pub const NERR_InvalidMaxUsers: u32 = 2122u32;
pub const NERR_InvalidUASOp: u32 = 2451u32;
pub const NERR_InvalidWorkgroupName: u32 = 2695u32;
pub const NERR_InvalidWorkstation: u32 = 2240u32;
pub const NERR_IsDfsShare: u32 = 2321u32;
pub const NERR_ItemNotFound: u32 = 2115u32;
pub const NERR_JobInvalidState: u32 = 2164u32;
pub const NERR_JobNoRoom: u32 = 2156u32;
pub const NERR_JobNotFound: u32 = 2151u32;
pub const NERR_JoinPerformedMustRestart: u32 = 2713u32;
pub const NERR_LDAPCapableDCRequired: u32 = 2721u32;
pub const NERR_LanmanIniError: u32 = 2131u32;
pub const NERR_LastAdmin: u32 = 2452u32;
pub const NERR_LineTooLong: u32 = 2149u32;
pub const NERR_LocalDrive: u32 = 2405u32;
pub const NERR_LocalForward: u32 = 2279u32;
pub const NERR_LogFileChanged: u32 = 2378u32;
pub const NERR_LogFileCorrupt: u32 = 2379u32;
pub const NERR_LogOverflow: u32 = 2377u32;
pub const NERR_LogonDomainExists: u32 = 2216u32;
pub const NERR_LogonNoUserPath: u32 = 2211u32;
pub const NERR_LogonScriptError: u32 = 2212u32;
pub const NERR_LogonServerConflict: u32 = 2210u32;
pub const NERR_LogonServerNotFound: u32 = 2215u32;
pub const NERR_LogonTrackingError: u32 = 2454u32;
pub const NERR_LogonsPaused: u32 = 2209u32;
pub const NERR_MaxLenExceeded: u32 = 2354u32;
pub const NERR_MsgAlreadyStarted: u32 = 2271u32;
pub const NERR_MsgInitFailed: u32 = 2272u32;
pub const NERR_MsgNotStarted: u32 = 2284u32;
pub const NERR_MultipleNets: u32 = 2300u32;
pub const NERR_NameInUse: u32 = 2283u32;
pub const NERR_NameNotForwarded: u32 = 2288u32;
pub const NERR_NameNotFound: u32 = 2273u32;
pub const NERR_NameUsesIncompatibleCodePage: u32 = 2696u32;
pub const NERR_NetNameNotFound: u32 = 2310u32;
pub const NERR_NetNotStarted: u32 = 2102u32;
pub const NERR_NetlogonNotStarted: u32 = 2455u32;
pub const NERR_NetworkError: u32 = 2136u32;
pub const NERR_NoAlternateServers: u32 = 2467u32;
pub const NERR_NoCommDevs: u32 = 2337u32;
pub const NERR_NoComputerName: u32 = 2270u32;
pub const NERR_NoForwardName: u32 = 2286u32;
pub const NERR_NoJoinPending: u32 = 2714u32;
pub const NERR_NoNetworkResource: u32 = 2105u32;
pub const NERR_NoOfflineJoinInfo: u32 = 2709u32;
pub const NERR_NoRoom: u32 = 2119u32;
pub const NERR_NoRplBootSystem: u32 = 2505u32;
pub const NERR_NoSuchAlert: u32 = 2432u32;
pub const NERR_NoSuchConnection: u32 = 2462u32;
pub const NERR_NoSuchServer: u32 = 2460u32;
pub const NERR_NoSuchSession: u32 = 2461u32;
pub const NERR_NonDosFloppyUsed: u32 = 2510u32;
pub const NERR_NonValidatedLogon: u32 = 2217u32;
pub const NERR_NotInCache: u32 = 2235u32;
pub const NERR_NotInDispatchTbl: u32 = 2192u32;
pub const NERR_NotLocalDomain: u32 = 2320u32;
pub const NERR_NotLocalName: u32 = 2285u32;
pub const NERR_NotLoggedOn: u32 = 2201u32;
pub const NERR_NotPrimary: u32 = 2226u32;
pub const NERR_OpenFiles: u32 = 2401u32;
pub const NERR_PasswordCantChange: u32 = 2243u32;
pub const NERR_PasswordExpired: u32 = 2242u32;
pub const NERR_PasswordFilterError: u32 = 2705u32;
pub const NERR_PasswordHistConflict: u32 = 2244u32;
pub const NERR_PasswordMismatch: u32 = 2458u32;
pub const NERR_PasswordMustChange: u32 = 2701u32;
pub const NERR_PasswordNotComplexEnough: u32 = 2704u32;
pub const NERR_PasswordTooLong: u32 = 2703u32;
pub const NERR_PasswordTooRecent: u32 = 2246u32;
pub const NERR_PasswordTooShort: u32 = 2245u32;
pub const NERR_PausedRemote: u32 = 2281u32;
pub const NERR_PersonalSku: u32 = 2698u32;
pub const NERR_PlainTextSecretsRequired: u32 = 2726u32;
pub const NERR_ProcNoRespond: u32 = 2160u32;
pub const NERR_ProcNotFound: u32 = 2168u32;
pub const NERR_ProfileCleanup: u32 = 2372u32;
pub const NERR_ProfileFileTooBig: u32 = 2370u32;
pub const NERR_ProfileLoadErr: u32 = 2374u32;
pub const NERR_ProfileOffset: u32 = 2371u32;
pub const NERR_ProfileSaveErr: u32 = 2375u32;
pub const NERR_ProfileUnknownCmd: u32 = 2373u32;
pub const NERR_ProgNeedsExtraMem: u32 = 2501u32;
pub const NERR_ProvisioningBlobUnsupported: u32 = 2719u32;
pub const NERR_QExists: u32 = 2154u32;
pub const NERR_QInvalidState: u32 = 2163u32;
pub const NERR_QNoRoom: u32 = 2155u32;
pub const NERR_QNotFound: u32 = 2150u32;
pub const NERR_QueueNotFound: u32 = 2338u32;
pub const NERR_RPL_CONNECTED: u32 = 2519u32;
pub const NERR_RedirectedPath: u32 = 2117u32;
pub const NERR_RemoteBootFailed: u32 = 2503u32;
pub const NERR_RemoteErr: u32 = 2127u32;
pub const NERR_RemoteFull: u32 = 2287u32;
pub const NERR_RemoteOnly: u32 = 2106u32;
pub const NERR_ResourceExists: u32 = 2225u32;
pub const NERR_ResourceNotFound: u32 = 2222u32;
pub const NERR_RplAdapterInfoCorrupted: u32 = 2625u32;
pub const NERR_RplAdapterNameUnavailable: u32 = 2633u32;
pub const NERR_RplAdapterNotFound: u32 = 2637u32;
pub const NERR_RplBackupDatabase: u32 = 2636u32;
pub const NERR_RplBadDatabase: u32 = 2612u32;
pub const NERR_RplBadRegistry: u32 = 2611u32;
pub const NERR_RplBootInUse: u32 = 2635u32;
pub const NERR_RplBootInfoCorrupted: u32 = 2628u32;
pub const NERR_RplBootNameUnavailable: u32 = 2640u32;
pub const NERR_RplBootNotFound: u32 = 2631u32;
pub const NERR_RplBootRestart: u32 = 2511u32;
pub const NERR_RplBootServiceTerm: u32 = 2517u32;
pub const NERR_RplBootStartFailed: u32 = 2518u32;
pub const NERR_RplCannotEnum: u32 = 2615u32;
pub const NERR_RplConfigInfoCorrupted: u32 = 2623u32;
pub const NERR_RplConfigNameUnavailable: u32 = 2641u32;
pub const NERR_RplConfigNotEmpty: u32 = 2634u32;
pub const NERR_RplConfigNotFound: u32 = 2624u32;
pub const NERR_RplIncompatibleProfile: u32 = 2632u32;
pub const NERR_RplInternal: u32 = 2626u32;
pub const NERR_RplLoadrDiskErr: u32 = 2507u32;
pub const NERR_RplLoadrNetBiosErr: u32 = 2506u32;
pub const NERR_RplNeedsRPLUSERAcct: u32 = 2630u32;
pub const NERR_RplNoAdaptersStarted: u32 = 2610u32;
pub const NERR_RplNotRplServer: u32 = 2614u32;
pub const NERR_RplProfileInfoCorrupted: u32 = 2619u32;
pub const NERR_RplProfileNameUnavailable: u32 = 2621u32;
pub const NERR_RplProfileNotEmpty: u32 = 2622u32;
pub const NERR_RplProfileNotFound: u32 = 2620u32;
pub const NERR_RplRplfilesShare: u32 = 2613u32;
pub const NERR_RplSrvrCallFailed: u32 = 2512u32;
pub const NERR_RplVendorInfoCorrupted: u32 = 2627u32;
pub const NERR_RplVendorNameUnavailable: u32 = 2639u32;
pub const NERR_RplVendorNotFound: u32 = 2638u32;
pub const NERR_RplWkstaInfoCorrupted: u32 = 2616u32;
pub const NERR_RplWkstaNameUnavailable: u32 = 2618u32;
pub const NERR_RplWkstaNeedsUserAcct: u32 = 2629u32;
pub const NERR_RplWkstaNotFound: u32 = 2617u32;
pub const NERR_RunSrvPaused: u32 = 2385u32;
pub const NERR_SameAsComputerName: u32 = 2253u32;
pub const NERR_ServerNotStarted: u32 = 2114u32;
pub const NERR_ServiceCtlBusy: u32 = 2187u32;
pub const NERR_ServiceCtlNotValid: u32 = 2191u32;
pub const NERR_ServiceCtlTimeout: u32 = 2186u32;
pub const NERR_ServiceEntryLocked: u32 = 2183u32;
pub const NERR_ServiceInstalled: u32 = 2182u32;
pub const NERR_ServiceKillProc: u32 = 2190u32;
pub const NERR_ServiceNotCtrl: u32 = 2189u32;
pub const NERR_ServiceNotInstalled: u32 = 2184u32;
pub const NERR_ServiceNotStarting: u32 = 2194u32;
pub const NERR_ServiceTableFull: u32 = 2181u32;
pub const NERR_ServiceTableLocked: u32 = 2180u32;
pub const NERR_SetupAlreadyJoined: u32 = 2691u32;
pub const NERR_SetupCheckDNSConfig: u32 = 2699u32;
pub const NERR_SetupDomainController: u32 = 2693u32;
pub const NERR_SetupNotJoined: u32 = 2692u32;
pub const NERR_ShareMem: u32 = 2104u32;
pub const NERR_ShareNotFound: u32 = 2392u32;
pub const NERR_SourceIsDir: u32 = 2380u32;
pub const NERR_SpeGroupOp: u32 = 2234u32;
pub const NERR_SpoolNoMemory: u32 = 2165u32;
pub const NERR_SpoolerNotLoaded: u32 = 2161u32;
pub const NERR_StandaloneLogon: u32 = 2214u32;
pub const NERR_StartingRplBoot: u32 = 2516u32;
pub const NERR_Success: u32 = 0u32;
pub const NERR_SyncRequired: u32 = 2249u32;
pub const NERR_TargetVersionUnsupported: u32 = 2723u32;
pub const NERR_TimeDiffAtDC: u32 = 2457u32;
pub const NERR_TmpFile: u32 = 2316u32;
pub const NERR_TooManyAlerts: u32 = 2431u32;
pub const NERR_TooManyConnections: u32 = 2465u32;
pub const NERR_TooManyEntries: u32 = 2362u32;
pub const NERR_TooManyFiles: u32 = 2466u32;
pub const NERR_TooManyImageParams: u32 = 2509u32;
pub const NERR_TooManyItems: u32 = 2121u32;
pub const NERR_TooManyNames: u32 = 2277u32;
pub const NERR_TooManyServers: u32 = 2463u32;
pub const NERR_TooManySessions: u32 = 2464u32;
pub const NERR_TooMuchData: u32 = 2317u32;
pub const NERR_TruncatedBroadcast: u32 = 2289u32;
pub const NERR_TryDownLevel: u32 = 2470u32;
pub const NERR_UPSDriverNotStarted: u32 = 2480u32;
pub const NERR_UPSInvalidCommPort: u32 = 2482u32;
pub const NERR_UPSInvalidConfig: u32 = 2481u32;
pub const NERR_UPSShutdownFailed: u32 = 2484u32;
pub const NERR_UPSSignalAsserted: u32 = 2483u32;
pub const NERR_UnableToAddName_F: u32 = 2205u32;
pub const NERR_UnableToAddName_W: u32 = 2204u32;
pub const NERR_UnableToDelName_F: u32 = 2207u32;
pub const NERR_UnableToDelName_W: u32 = 2206u32;
pub const NERR_UnknownDevDir: u32 = 2116u32;
pub const NERR_UnknownServer: u32 = 2103u32;
pub const NERR_UseNotFound: u32 = 2250u32;
pub const NERR_UserExists: u32 = 2224u32;
pub const NERR_UserInGroup: u32 = 2236u32;
pub const NERR_UserLogon: u32 = 2231u32;
pub const NERR_UserNotFound: u32 = 2221u32;
pub const NERR_UserNotInGroup: u32 = 2237u32;
pub const NERR_ValuesNotSet: u32 = 2715u32;
pub const NERR_WkstaInconsistentState: u32 = 2137u32;
pub const NERR_WkstaNotStarted: u32 = 2138u32;
pub const NERR_WriteFault: u32 = 2295u32;
pub const NETBIOS_NAME_LEN: u32 = 16u32;
pub const NETCFG_E_ACTIVE_RAS_CONNECTIONS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147180506i32 as _);
pub const NETCFG_E_ADAPTER_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147180505i32 as _);
pub const NETCFG_E_ALREADY_INITIALIZED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147180512i32 as _);
pub const NETCFG_E_COMPONENT_REMOVED_PENDING_REBOOT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147180504i32 as _);
pub const NETCFG_E_DUPLICATE_INSTANCEID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147180501i32 as _);
pub const NETCFG_E_IN_USE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147180510i32 as _);
pub const NETCFG_E_MAX_FILTER_LIMIT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147180503i32 as _);
pub const NETCFG_E_NEED_REBOOT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147180507i32 as _);
pub const NETCFG_E_NOT_INITIALIZED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147180511i32 as _);
pub const NETCFG_E_NO_WRITE_LOCK: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147180508i32 as _);
pub const NETCFG_E_VMSWITCH_ACTIVE_OVER_ADAPTER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147180502i32 as _);
pub const NETCFG_S_CAUSED_SETUP_CHANGE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(303140i32 as _);
pub const NETCFG_S_COMMIT_NOW: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(303141i32 as _);
pub const NETCFG_S_DISABLE_QUERY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(303138i32 as _);
pub const NETCFG_S_REBOOT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(303136i32 as _);
pub const NETCFG_S_STILL_REFERENCED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(303139i32 as _);
pub const NETLOGON_CONTROL_BACKUP_CHANGE_LOG: u32 = 65532u32;
pub const NETLOGON_CONTROL_BREAKPOINT: u32 = 65535u32;
pub const NETLOGON_CONTROL_CHANGE_PASSWORD: u32 = 9u32;
pub const NETLOGON_CONTROL_FIND_USER: u32 = 8u32;
pub const NETLOGON_CONTROL_FORCE_DNS_REG: u32 = 11u32;
pub const NETLOGON_CONTROL_PDC_REPLICATE: u32 = 4u32;
pub const NETLOGON_CONTROL_QUERY: u32 = 1u32;
pub const NETLOGON_CONTROL_QUERY_DNS_REG: u32 = 12u32;
pub const NETLOGON_CONTROL_QUERY_ENC_TYPES: u32 = 13u32;
pub const NETLOGON_CONTROL_REDISCOVER: u32 = 5u32;
pub const NETLOGON_CONTROL_REPLICATE: u32 = 2u32;
pub const NETLOGON_CONTROL_SET_DBFLAG: u32 = 65534u32;
pub const NETLOGON_CONTROL_SYNCHRONIZE: u32 = 3u32;
pub const NETLOGON_CONTROL_TC_QUERY: u32 = 6u32;
pub const NETLOGON_CONTROL_TC_VERIFY: u32 = 10u32;
pub const NETLOGON_CONTROL_TRANSPORT_NOTIFY: u32 = 7u32;
pub const NETLOGON_CONTROL_TRUNCATE_LOG: u32 = 65533u32;
pub const NETLOGON_CONTROL_UNLOAD_NETLOGON_DLL: u32 = 65531u32;
pub const NETLOGON_DNS_UPDATE_FAILURE: u32 = 64u32;
pub const NETLOGON_FULL_SYNC_REPLICATION: u32 = 4u32;
pub const NETLOGON_HAS_IP: u32 = 16u32;
pub const NETLOGON_HAS_TIMESERV: u32 = 32u32;
#[repr(C)]
pub struct NETLOGON_INFO_1(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NETLOGON_INFO_2(i32);
#[repr(C)]
pub struct NETLOGON_INFO_3(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NETLOGON_INFO_4(i32);
pub const NETLOGON_REDO_NEEDED: u32 = 8u32;
pub const NETLOGON_REPLICATION_IN_PROGRESS: u32 = 2u32;
pub const NETLOGON_REPLICATION_NEEDED: u32 = 1u32;
pub const NETLOGON_VERIFY_STATUS_RETURNED: u32 = 128u32;
pub const NETLOG_NetlogonNonWindowsSupportsSecureRpc: u32 = 5826u32;
pub const NETLOG_NetlogonUnsecureRpcClient: u32 = 5827u32;
pub const NETLOG_NetlogonUnsecureRpcMachineAllowedBySsdl: u32 = 5830u32;
pub const NETLOG_NetlogonUnsecureRpcTrust: u32 = 5828u32;
pub const NETLOG_NetlogonUnsecureRpcTrustAllowedBySsdl: u32 = 5831u32;
pub const NETLOG_NetlogonUnsecuredRpcMachineTemporarilyAllowed: u32 = 5829u32;
pub const NETMAN_VARTYPE_HARDWARE_ADDRESS: u32 = 1u32;
pub const NETMAN_VARTYPE_STRING: u32 = 2u32;
pub const NETMAN_VARTYPE_ULONG: u32 = 0u32;
pub const NETSETUP_ACCT_DELETE: u32 = 4u32;
pub const NETSETUP_ALT_SAMACCOUNTNAME: u32 = 131072u32;
pub const NETSETUP_DNS_NAME_CHANGES_ONLY: u32 = 4096u32;
pub const NETSETUP_INSTALL_INVOCATION: u32 = 262144u32;
#[repr(transparent)]
pub struct NETSETUP_JOIN_STATUS(pub i32);
pub const NetSetupUnknownStatus: NETSETUP_JOIN_STATUS = NETSETUP_JOIN_STATUS(0i32);
pub const NetSetupUnjoined: NETSETUP_JOIN_STATUS = NETSETUP_JOIN_STATUS(1i32);
pub const NetSetupWorkgroupName: NETSETUP_JOIN_STATUS = NETSETUP_JOIN_STATUS(2i32);
pub const NetSetupDomainName: NETSETUP_JOIN_STATUS = NETSETUP_JOIN_STATUS(3i32);
#[repr(transparent)]
pub struct NETSETUP_NAME_TYPE(pub i32);
pub const NetSetupUnknown: NETSETUP_NAME_TYPE = NETSETUP_NAME_TYPE(0i32);
pub const NetSetupMachine: NETSETUP_NAME_TYPE = NETSETUP_NAME_TYPE(1i32);
pub const NetSetupWorkgroup: NETSETUP_NAME_TYPE = NETSETUP_NAME_TYPE(2i32);
pub const NetSetupDomain: NETSETUP_NAME_TYPE = NETSETUP_NAME_TYPE(3i32);
pub const NetSetupNonExistentDomain: NETSETUP_NAME_TYPE = NETSETUP_NAME_TYPE(4i32);
pub const NetSetupDnsMachine: NETSETUP_NAME_TYPE = NETSETUP_NAME_TYPE(5i32);
#[repr(transparent)]
pub struct NETSETUP_PROVISION(pub u32);
pub const NETSETUP_PROVISION_DOWNLEVEL_PRIV_SUPPORT: NETSETUP_PROVISION = NETSETUP_PROVISION(1u32);
pub const NETSETUP_PROVISION_REUSE_ACCOUNT: NETSETUP_PROVISION = NETSETUP_PROVISION(2u32);
pub const NETSETUP_PROVISION_USE_DEFAULT_PASSWORD: NETSETUP_PROVISION = NETSETUP_PROVISION(4u32);
pub const NETSETUP_PROVISION_SKIP_ACCOUNT_SEARCH: NETSETUP_PROVISION = NETSETUP_PROVISION(8u32);
pub const NETSETUP_PROVISION_ROOT_CA_CERTS: NETSETUP_PROVISION = NETSETUP_PROVISION(16u32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NETSETUP_PROVISIONING_PARAMS(i32);
pub const NETSETUP_PROVISIONING_PARAMS_CURRENT_VERSION: u32 = 2u32;
pub const NETSETUP_PROVISIONING_PARAMS_WIN8_VERSION: u32 = 1u32;
pub const NETSETUP_PROVISION_CHECK_PWD_ONLY: u32 = 2147483648u32;
pub const NETSETUP_PROVISION_PERSISTENTSITE: u32 = 32u32;
#[repr(transparent)]
pub struct NETWORK_INSTALL_TIME(pub i32);
pub const NSF_PRIMARYINSTALL: NETWORK_INSTALL_TIME = NETWORK_INSTALL_TIME(1i32);
pub const NSF_POSTSYSINSTALL: NETWORK_INSTALL_TIME = NETWORK_INSTALL_TIME(2i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NETWORK_NAME(i32);
#[repr(transparent)]
pub struct NETWORK_UPGRADE_TYPE(pub i32);
pub const NSF_WIN16_UPGRADE: NETWORK_UPGRADE_TYPE = NETWORK_UPGRADE_TYPE(16i32);
pub const NSF_WIN95_UPGRADE: NETWORK_UPGRADE_TYPE = NETWORK_UPGRADE_TYPE(32i32);
pub const NSF_WINNT_WKS_UPGRADE: NETWORK_UPGRADE_TYPE = NETWORK_UPGRADE_TYPE(64i32);
pub const NSF_WINNT_SVR_UPGRADE: NETWORK_UPGRADE_TYPE = NETWORK_UPGRADE_TYPE(128i32);
pub const NSF_WINNT_SBS_UPGRADE: NETWORK_UPGRADE_TYPE = NETWORK_UPGRADE_TYPE(256i32);
pub const NSF_COMPONENT_UPDATE: NETWORK_UPGRADE_TYPE = NETWORK_UPGRADE_TYPE(512i32);
#[repr(transparent)]
pub struct NET_COMPUTER_NAME_TYPE(pub i32);
pub const NetPrimaryComputerName: NET_COMPUTER_NAME_TYPE = NET_COMPUTER_NAME_TYPE(0i32);
pub const NetAlternateComputerNames: NET_COMPUTER_NAME_TYPE = NET_COMPUTER_NAME_TYPE(1i32);
pub const NetAllComputerNames: NET_COMPUTER_NAME_TYPE = NET_COMPUTER_NAME_TYPE(2i32);
pub const NetComputerNameTypeMax: NET_COMPUTER_NAME_TYPE = NET_COMPUTER_NAME_TYPE(3i32);
pub const NET_DFS_ENUM: i32 = 1073756324i32;
pub const NET_DFS_ENUMEX: i32 = 1073756325i32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NET_DISPLAY_GROUP(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NET_DISPLAY_MACHINE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NET_DISPLAY_USER(i32);
pub const NET_IGNORE_UNSUPPORTED_FLAGS: u32 = 1u32;
#[repr(transparent)]
pub struct NET_JOIN_DOMAIN_JOIN_OPTIONS(pub u32);
pub const NETSETUP_JOIN_DOMAIN: NET_JOIN_DOMAIN_JOIN_OPTIONS = NET_JOIN_DOMAIN_JOIN_OPTIONS(1u32);
pub const NETSETUP_ACCT_CREATE: NET_JOIN_DOMAIN_JOIN_OPTIONS = NET_JOIN_DOMAIN_JOIN_OPTIONS(2u32);
pub const NETSETUP_WIN9X_UPGRADE: NET_JOIN_DOMAIN_JOIN_OPTIONS = NET_JOIN_DOMAIN_JOIN_OPTIONS(16u32);
pub const NETSETUP_DOMAIN_JOIN_IF_JOINED: NET_JOIN_DOMAIN_JOIN_OPTIONS = NET_JOIN_DOMAIN_JOIN_OPTIONS(32u32);
pub const NETSETUP_JOIN_UNSECURE: NET_JOIN_DOMAIN_JOIN_OPTIONS = NET_JOIN_DOMAIN_JOIN_OPTIONS(64u32);
pub const NETSETUP_MACHINE_PWD_PASSED: NET_JOIN_DOMAIN_JOIN_OPTIONS = NET_JOIN_DOMAIN_JOIN_OPTIONS(128u32);
pub const NETSETUP_DEFER_SPN_SET: NET_JOIN_DOMAIN_JOIN_OPTIONS = NET_JOIN_DOMAIN_JOIN_OPTIONS(256u32);
pub const NETSETUP_JOIN_DC_ACCOUNT: NET_JOIN_DOMAIN_JOIN_OPTIONS = NET_JOIN_DOMAIN_JOIN_OPTIONS(512u32);
pub const NETSETUP_JOIN_WITH_NEW_NAME: NET_JOIN_DOMAIN_JOIN_OPTIONS = NET_JOIN_DOMAIN_JOIN_OPTIONS(1024u32);
pub const NETSETUP_JOIN_READONLY: NET_JOIN_DOMAIN_JOIN_OPTIONS = NET_JOIN_DOMAIN_JOIN_OPTIONS(2048u32);
pub const NETSETUP_AMBIGUOUS_DC: NET_JOIN_DOMAIN_JOIN_OPTIONS = NET_JOIN_DOMAIN_JOIN_OPTIONS(4096u32);
pub const NETSETUP_NO_NETLOGON_CACHE: NET_JOIN_DOMAIN_JOIN_OPTIONS = NET_JOIN_DOMAIN_JOIN_OPTIONS(8192u32);
pub const NETSETUP_DONT_CONTROL_SERVICES: NET_JOIN_DOMAIN_JOIN_OPTIONS = NET_JOIN_DOMAIN_JOIN_OPTIONS(16384u32);
pub const NETSETUP_SET_MACHINE_NAME: NET_JOIN_DOMAIN_JOIN_OPTIONS = NET_JOIN_DOMAIN_JOIN_OPTIONS(32768u32);
pub const NETSETUP_FORCE_SPN_SET: NET_JOIN_DOMAIN_JOIN_OPTIONS = NET_JOIN_DOMAIN_JOIN_OPTIONS(65536u32);
pub const NETSETUP_NO_ACCT_REUSE: NET_JOIN_DOMAIN_JOIN_OPTIONS = NET_JOIN_DOMAIN_JOIN_OPTIONS(131072u32);
pub const NETSETUP_IGNORE_UNSUPPORTED_FLAGS: NET_JOIN_DOMAIN_JOIN_OPTIONS = NET_JOIN_DOMAIN_JOIN_OPTIONS(268435456u32);
#[repr(transparent)]
pub struct NET_REMOTE_COMPUTER_SUPPORTS_OPTIONS(pub i32);
pub const SUPPORTS_REMOTE_ADMIN_PROTOCOL: NET_REMOTE_COMPUTER_SUPPORTS_OPTIONS = NET_REMOTE_COMPUTER_SUPPORTS_OPTIONS(2i32);
pub const SUPPORTS_RPC: NET_REMOTE_COMPUTER_SUPPORTS_OPTIONS = NET_REMOTE_COMPUTER_SUPPORTS_OPTIONS(4i32);
pub const SUPPORTS_SAM_PROTOCOL: NET_REMOTE_COMPUTER_SUPPORTS_OPTIONS = NET_REMOTE_COMPUTER_SUPPORTS_OPTIONS(8i32);
pub const SUPPORTS_UNICODE: NET_REMOTE_COMPUTER_SUPPORTS_OPTIONS = NET_REMOTE_COMPUTER_SUPPORTS_OPTIONS(16i32);
pub const SUPPORTS_LOCAL: NET_REMOTE_COMPUTER_SUPPORTS_OPTIONS = NET_REMOTE_COMPUTER_SUPPORTS_OPTIONS(32i32);
#[repr(transparent)]
pub struct NET_REQUEST_PROVISION_OPTIONS(pub u32);
pub const NETSETUP_PROVISION_ONLINE_CALLER: NET_REQUEST_PROVISION_OPTIONS = NET_REQUEST_PROVISION_OPTIONS(1073741824u32);
#[repr(transparent)]
pub struct NET_SERVER_TYPE(pub u32);
pub const SV_TYPE_WORKSTATION: NET_SERVER_TYPE = NET_SERVER_TYPE(1u32);
pub const SV_TYPE_SERVER: NET_SERVER_TYPE = NET_SERVER_TYPE(2u32);
pub const SV_TYPE_SQLSERVER: NET_SERVER_TYPE = NET_SERVER_TYPE(4u32);
pub const SV_TYPE_DOMAIN_CTRL: NET_SERVER_TYPE = NET_SERVER_TYPE(8u32);
pub const SV_TYPE_DOMAIN_BAKCTRL: NET_SERVER_TYPE = NET_SERVER_TYPE(16u32);
pub const SV_TYPE_TIME_SOURCE: NET_SERVER_TYPE = NET_SERVER_TYPE(32u32);
pub const SV_TYPE_AFP: NET_SERVER_TYPE = NET_SERVER_TYPE(64u32);
pub const SV_TYPE_NOVELL: NET_SERVER_TYPE = NET_SERVER_TYPE(128u32);
pub const SV_TYPE_DOMAIN_MEMBER: NET_SERVER_TYPE = NET_SERVER_TYPE(256u32);
pub const SV_TYPE_PRINTQ_SERVER: NET_SERVER_TYPE = NET_SERVER_TYPE(512u32);
pub const SV_TYPE_DIALIN_SERVER: NET_SERVER_TYPE = NET_SERVER_TYPE(1024u32);
pub const SV_TYPE_XENIX_SERVER: NET_SERVER_TYPE = NET_SERVER_TYPE(2048u32);
pub const SV_TYPE_SERVER_UNIX: NET_SERVER_TYPE = NET_SERVER_TYPE(2048u32);
pub const SV_TYPE_NT: NET_SERVER_TYPE = NET_SERVER_TYPE(4096u32);
pub const SV_TYPE_WFW: NET_SERVER_TYPE = NET_SERVER_TYPE(8192u32);
pub const SV_TYPE_SERVER_MFPN: NET_SERVER_TYPE = NET_SERVER_TYPE(16384u32);
pub const SV_TYPE_SERVER_NT: NET_SERVER_TYPE = NET_SERVER_TYPE(32768u32);
pub const SV_TYPE_POTENTIAL_BROWSER: NET_SERVER_TYPE = NET_SERVER_TYPE(65536u32);
pub const SV_TYPE_BACKUP_BROWSER: NET_SERVER_TYPE = NET_SERVER_TYPE(131072u32);
pub const SV_TYPE_MASTER_BROWSER: NET_SERVER_TYPE = NET_SERVER_TYPE(262144u32);
pub const SV_TYPE_DOMAIN_MASTER: NET_SERVER_TYPE = NET_SERVER_TYPE(524288u32);
pub const SV_TYPE_SERVER_OSF: NET_SERVER_TYPE = NET_SERVER_TYPE(1048576u32);
pub const SV_TYPE_SERVER_VMS: NET_SERVER_TYPE = NET_SERVER_TYPE(2097152u32);
pub const SV_TYPE_WINDOWS: NET_SERVER_TYPE = NET_SERVER_TYPE(4194304u32);
pub const SV_TYPE_DFS: NET_SERVER_TYPE = NET_SERVER_TYPE(8388608u32);
pub const SV_TYPE_CLUSTER_NT: NET_SERVER_TYPE = NET_SERVER_TYPE(16777216u32);
pub const SV_TYPE_TERMINALSERVER: NET_SERVER_TYPE = NET_SERVER_TYPE(33554432u32);
pub const SV_TYPE_CLUSTER_VS_NT: NET_SERVER_TYPE = NET_SERVER_TYPE(67108864u32);
pub const SV_TYPE_DCE: NET_SERVER_TYPE = NET_SERVER_TYPE(268435456u32);
pub const SV_TYPE_ALTERNATE_XPORT: NET_SERVER_TYPE = NET_SERVER_TYPE(536870912u32);
pub const SV_TYPE_LOCAL_LIST_ONLY: NET_SERVER_TYPE = NET_SERVER_TYPE(1073741824u32);
pub const SV_TYPE_DOMAIN_ENUM: NET_SERVER_TYPE = NET_SERVER_TYPE(2147483648u32);
pub const SV_TYPE_ALL: NET_SERVER_TYPE = NET_SERVER_TYPE(4294967295u32);
#[repr(transparent)]
pub struct NET_USER_ENUM_FILTER_FLAGS(pub u32);
pub const FILTER_TEMP_DUPLICATE_ACCOUNT: NET_USER_ENUM_FILTER_FLAGS = NET_USER_ENUM_FILTER_FLAGS(1u32);
pub const FILTER_NORMAL_ACCOUNT: NET_USER_ENUM_FILTER_FLAGS = NET_USER_ENUM_FILTER_FLAGS(2u32);
pub const FILTER_INTERDOMAIN_TRUST_ACCOUNT: NET_USER_ENUM_FILTER_FLAGS = NET_USER_ENUM_FILTER_FLAGS(8u32);
pub const FILTER_WORKSTATION_TRUST_ACCOUNT: NET_USER_ENUM_FILTER_FLAGS = NET_USER_ENUM_FILTER_FLAGS(16u32);
pub const FILTER_SERVER_TRUST_ACCOUNT: NET_USER_ENUM_FILTER_FLAGS = NET_USER_ENUM_FILTER_FLAGS(32u32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NET_VALIDATE_AUTHENTICATION_INPUT_ARG(i32);
pub const NET_VALIDATE_BAD_PASSWORD_COUNT: u32 = 8u32;
pub const NET_VALIDATE_BAD_PASSWORD_TIME: u32 = 2u32;
pub const NET_VALIDATE_LOCKOUT_TIME: u32 = 4u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NET_VALIDATE_OUTPUT_ARG(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NET_VALIDATE_PASSWORD_CHANGE_INPUT_ARG(i32);
#[repr(C)]
pub struct NET_VALIDATE_PASSWORD_HASH(i32);
pub const NET_VALIDATE_PASSWORD_HISTORY: u32 = 32u32;
pub const NET_VALIDATE_PASSWORD_HISTORY_LENGTH: u32 = 16u32;
pub const NET_VALIDATE_PASSWORD_LAST_SET: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NET_VALIDATE_PASSWORD_RESET_INPUT_ARG(i32);
#[repr(transparent)]
pub struct NET_VALIDATE_PASSWORD_TYPE(pub i32);
pub const NetValidateAuthentication: NET_VALIDATE_PASSWORD_TYPE = NET_VALIDATE_PASSWORD_TYPE(1i32);
pub const NetValidatePasswordChange: NET_VALIDATE_PASSWORD_TYPE = NET_VALIDATE_PASSWORD_TYPE(2i32);
pub const NetValidatePasswordReset: NET_VALIDATE_PASSWORD_TYPE = NET_VALIDATE_PASSWORD_TYPE(3i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NET_VALIDATE_PERSISTED_FIELDS(i32);
pub const NON_VALIDATED_LOGON: u32 = 3u32;
pub const NOT_A_DFS_PATH: i32 = 1073756224i32;
pub const NO_PERMISSION_REQUIRED: u32 = 1u32;
pub const NTFRSPRF_COLLECT_RPC_BINDING_ERROR_CONN: i32 = -1073728292i32;
pub const NTFRSPRF_COLLECT_RPC_BINDING_ERROR_SET: i32 = -1073728293i32;
pub const NTFRSPRF_COLLECT_RPC_CALL_ERROR_CONN: i32 = -1073728290i32;
pub const NTFRSPRF_COLLECT_RPC_CALL_ERROR_SET: i32 = -1073728291i32;
pub const NTFRSPRF_OPEN_RPC_BINDING_ERROR_CONN: i32 = -1073728296i32;
pub const NTFRSPRF_OPEN_RPC_BINDING_ERROR_SET: i32 = -1073728297i32;
pub const NTFRSPRF_OPEN_RPC_CALL_ERROR_CONN: i32 = -1073728294i32;
pub const NTFRSPRF_OPEN_RPC_CALL_ERROR_SET: i32 = -1073728295i32;
pub const NTFRSPRF_REGISTRY_ERROR_CONN: i32 = -1073728286i32;
pub const NTFRSPRF_REGISTRY_ERROR_SET: i32 = -1073728287i32;
pub const NTFRSPRF_VIRTUALALLOC_ERROR_CONN: i32 = -1073728288i32;
pub const NTFRSPRF_VIRTUALALLOC_ERROR_SET: i32 = -1073728289i32;
pub const NWSAP_EVENT_BADWANFILTER_VALUE: i32 = -1073733302i32;
pub const NWSAP_EVENT_BIND_FAILED: i32 = -1073733320i32;
pub const NWSAP_EVENT_CARDLISTEVENT_FAIL: i32 = -1073733301i32;
pub const NWSAP_EVENT_CARDMALLOC_FAILED: i32 = -1073733316i32;
pub const NWSAP_EVENT_CREATELPCEVENT_ERROR: i32 = -1073733305i32;
pub const NWSAP_EVENT_CREATELPCPORT_ERROR: i32 = -1073733306i32;
pub const NWSAP_EVENT_GETSOCKNAME_FAILED: i32 = -1073733319i32;
pub const NWSAP_EVENT_HASHTABLE_MALLOC_FAILED: i32 = -1073733308i32;
pub const NWSAP_EVENT_INVALID_FILTERNAME: i32 = -2147475123i32;
pub const NWSAP_EVENT_KEY_NOT_FOUND: i32 = -1073733324i32;
pub const NWSAP_EVENT_LPCHANDLEMEMORY_ERROR: i32 = -1073733303i32;
pub const NWSAP_EVENT_LPCLISTENMEMORY_ERROR: i32 = -1073733304i32;
pub const NWSAP_EVENT_NOCARDS: i32 = -1073733315i32;
pub const NWSAP_EVENT_OPTBCASTINADDR_FAILED: i32 = -1073733317i32;
pub const NWSAP_EVENT_OPTEXTENDEDADDR_FAILED: i32 = -1073733318i32;
pub const NWSAP_EVENT_OPTMAXADAPTERNUM_ERROR: i32 = -1073733293i32;
pub const NWSAP_EVENT_RECVSEM_FAIL: i32 = -1073733313i32;
pub const NWSAP_EVENT_SDMDEVENT_FAIL: i32 = -1073733300i32;
pub const NWSAP_EVENT_SENDEVENT_FAIL: i32 = -1073733312i32;
pub const NWSAP_EVENT_SETOPTBCAST_FAILED: i32 = -1073733321i32;
pub const NWSAP_EVENT_SOCKET_FAILED: i32 = -1073733322i32;
pub const NWSAP_EVENT_STARTLPCWORKER_ERROR: i32 = -1073733307i32;
pub const NWSAP_EVENT_STARTRECEIVE_ERROR: i32 = -1073733311i32;
pub const NWSAP_EVENT_STARTWANCHECK_ERROR: i32 = -1073733294i32;
pub const NWSAP_EVENT_STARTWANWORKER_ERROR: i32 = -1073733295i32;
pub const NWSAP_EVENT_STARTWORKER_ERROR: i32 = -1073733310i32;
pub const NWSAP_EVENT_TABLE_MALLOC_FAILED: i32 = -1073733309i32;
pub const NWSAP_EVENT_THREADEVENT_FAIL: i32 = -1073733314i32;
pub const NWSAP_EVENT_WANBIND_FAILED: i32 = -1073733296i32;
pub const NWSAP_EVENT_WANEVENT_ERROR: i32 = -1073733291i32;
pub const NWSAP_EVENT_WANHANDLEMEMORY_ERROR: i32 = -1073733292i32;
pub const NWSAP_EVENT_WANSEM_FAIL: i32 = -1073733298i32;
pub const NWSAP_EVENT_WANSOCKET_FAILED: i32 = -1073733297i32;
pub const NWSAP_EVENT_WSASTARTUP_FAILED: i32 = -1073733323i32;
#[repr(C)]
pub struct NetProvisioning(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct OBO_TOKEN(i32);
#[repr(transparent)]
pub struct OBO_TOKEN_TYPE(pub i32);
pub const OBO_USER: OBO_TOKEN_TYPE = OBO_TOKEN_TYPE(1i32);
pub const OBO_COMPONENT: OBO_TOKEN_TYPE = OBO_TOKEN_TYPE(2i32);
pub const OBO_SOFTWARE: OBO_TOKEN_TYPE = OBO_TOKEN_TYPE(3i32);
pub const PARMNUM_ALL: u32 = 0u32;
pub const PARMNUM_BASE_INFOLEVEL: u32 = 1000u32;
pub const PARM_ERROR_NONE: u32 = 0u32;
pub const PARM_ERROR_UNKNOWN: u32 = 4294967295u32;
pub const PASSWORD_EXPIRED: u32 = 2u32;
pub const PATHLEN: u32 = 256u32;
pub const PLATFORM_ID_DOS: u32 = 300u32;
pub const PLATFORM_ID_NT: u32 = 500u32;
pub const PLATFORM_ID_OS2: u32 = 400u32;
pub const PLATFORM_ID_OSF: u32 = 600u32;
pub const PLATFORM_ID_VMS: u32 = 700u32;
pub const PREFIX_MISMATCH: i32 = -1073727510i32;
pub const PREFIX_MISMATCH_FIXED: i32 = -1073727509i32;
pub const PREFIX_MISMATCH_NOT_FIXED: i32 = -1073727508i32;
#[repr(C)]
pub struct PRINT_OTHER_INFO(i32);
pub const PRJOB_COMPLETE: u32 = 4u32;
pub const PRJOB_DELETED: u32 = 32768u32;
pub const PRJOB_DESTNOPAPER: u32 = 256u32;
pub const PRJOB_DESTOFFLINE: u32 = 32u32;
pub const PRJOB_DESTPAUSED: u32 = 64u32;
pub const PRJOB_DEVSTATUS: u32 = 508u32;
pub const PRJOB_ERROR: u32 = 16u32;
pub const PRJOB_INTERV: u32 = 8u32;
pub const PRJOB_NOTIFY: u32 = 128u32;
pub const PRJOB_QSTATUS: u32 = 3u32;
pub const PRJOB_QS_PAUSED: u32 = 1u32;
pub const PRJOB_QS_PRINTING: u32 = 3u32;
pub const PRJOB_QS_QUEUED: u32 = 0u32;
pub const PRJOB_QS_SPOOLING: u32 = 2u32;
pub const PROTO_IPV6_DHCP: u32 = 999u32;
pub const PROTO_IP_ALG: u32 = 10010u32;
pub const PROTO_IP_BGMP: u32 = 11u32;
pub const PROTO_IP_BOOTP: u32 = 9999u32;
pub const PROTO_IP_DHCP_ALLOCATOR: u32 = 10004u32;
pub const PROTO_IP_DIFFSERV: u32 = 10008u32;
pub const PROTO_IP_DNS_PROXY: u32 = 10003u32;
pub const PROTO_IP_DTP: u32 = 10013u32;
pub const PROTO_IP_FTP: u32 = 10012u32;
pub const PROTO_IP_H323: u32 = 10011u32;
pub const PROTO_IP_IGMP: u32 = 10u32;
pub const PROTO_IP_MGM: u32 = 10009u32;
pub const PROTO_IP_MSDP: u32 = 9u32;
pub const PROTO_IP_NAT: u32 = 10005u32;
pub const PROTO_IP_VRRP: u32 = 112u32;
pub const PROTO_TYPE_MCAST: u32 = 1u32;
pub const PROTO_TYPE_MS0: u32 = 2u32;
pub const PROTO_TYPE_MS1: u32 = 3u32;
pub const PROTO_TYPE_UCAST: u32 = 0u32;
pub const PROTO_VENDOR_MS0: u32 = 0u32;
pub const PROTO_VENDOR_MS1: u32 = 311u32;
pub const PROTO_VENDOR_MS2: u32 = 16383u32;
pub const PWLEN: u32 = 256u32;
pub const QNLEN: u32 = 80u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct RASCON_IPUI(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct REPL_EDIR_INFO_0(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct REPL_EDIR_INFO_1(i32);
#[repr(C)]
pub struct REPL_EDIR_INFO_1000(i32);
#[repr(C)]
pub struct REPL_EDIR_INFO_1001(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct REPL_EDIR_INFO_2(i32);
pub const REPL_EXPORT_EXTENT_INFOLEVEL: u32 = 1001u32;
pub const REPL_EXPORT_INTEGRITY_INFOLEVEL: u32 = 1000u32;
pub const REPL_EXTENT_FILE: u32 = 1u32;
pub const REPL_EXTENT_TREE: u32 = 2u32;
pub const REPL_GUARDTIME_INFOLEVEL: u32 = 1002u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct REPL_IDIR_INFO_0(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct REPL_IDIR_INFO_1(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct REPL_INFO_0(i32);
#[repr(C)]
pub struct REPL_INFO_1000(i32);
#[repr(C)]
pub struct REPL_INFO_1001(i32);
#[repr(C)]
pub struct REPL_INFO_1002(i32);
#[repr(C)]
pub struct REPL_INFO_1003(i32);
pub const REPL_INTEGRITY_FILE: u32 = 1u32;
pub const REPL_INTEGRITY_TREE: u32 = 2u32;
pub const REPL_INTERVAL_INFOLEVEL: u32 = 1000u32;
pub const REPL_PULSE_INFOLEVEL: u32 = 1001u32;
pub const REPL_RANDOM_INFOLEVEL: u32 = 1003u32;
pub const REPL_ROLE_BOTH: u32 = 3u32;
pub const REPL_ROLE_EXPORT: u32 = 1u32;
pub const REPL_ROLE_IMPORT: u32 = 2u32;
pub const REPL_STATE_NEVER_REPLICATED: u32 = 3u32;
pub const REPL_STATE_NO_MASTER: u32 = 1u32;
pub const REPL_STATE_NO_SYNC: u32 = 2u32;
pub const REPL_STATE_OK: u32 = 0u32;
pub const REPL_UNLOCK_FORCE: u32 = 1u32;
pub const REPL_UNLOCK_NOFORCE: u32 = 0u32;
pub const RF_ADD_ALL_INTERFACES: u32 = 16u32;
pub const RF_DEMAND_UPDATE_ROUTES: u32 = 4u32;
pub const RF_MULTICAST: u32 = 32u32;
pub const RF_POWER: u32 = 64u32;
pub const RF_ROUTING: u32 = 1u32;
pub const RF_ROUTINGV6: u32 = 2u32;
pub const RIS_INTERFACE_ADDRESS_CHANGE: u32 = 0u32;
pub const RIS_INTERFACE_DISABLED: u32 = 2u32;
pub const RIS_INTERFACE_ENABLED: u32 = 1u32;
pub const RIS_INTERFACE_MEDIA_ABSENT: u32 = 4u32;
pub const RIS_INTERFACE_MEDIA_PRESENT: u32 = 3u32;
pub const ROUTING_DOMAIN_INFO_REVISION_1: u32 = 1u32;
#[repr(C)]
pub struct RTR_INFO_BLOCK_HEADER(i32);
pub const RTR_INFO_BLOCK_VERSION: u32 = 1u32;
#[repr(C)]
pub struct RTR_TOC_ENTRY(i32);
pub const RTUTILS_MAX_PROTOCOL_DLL_LEN: u32 = 48u32;
pub const RTUTILS_MAX_PROTOCOL_NAME_LEN: u32 = 40u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SERVER_INFO_100(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SERVER_INFO_1005(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SERVER_INFO_101(i32);
#[repr(C)]
pub struct SERVER_INFO_1010(i32);
#[repr(C)]
pub struct SERVER_INFO_1016(i32);
#[repr(C)]
pub struct SERVER_INFO_1017(i32);
#[repr(C)]
pub struct SERVER_INFO_1018(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SERVER_INFO_102(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SERVER_INFO_103(i32);
#[repr(C)]
pub struct SERVER_INFO_1107(i32);
#[repr(C)]
pub struct SERVER_INFO_1501(i32);
#[repr(C)]
pub struct SERVER_INFO_1502(i32);
#[repr(C)]
pub struct SERVER_INFO_1503(i32);
#[repr(C)]
pub struct SERVER_INFO_1506(i32);
#[repr(C)]
pub struct SERVER_INFO_1509(i32);
#[repr(C)]
pub struct SERVER_INFO_1510(i32);
#[repr(C)]
pub struct SERVER_INFO_1511(i32);
#[repr(C)]
pub struct SERVER_INFO_1512(i32);
#[repr(C)]
pub struct SERVER_INFO_1513(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SERVER_INFO_1514(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SERVER_INFO_1515(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SERVER_INFO_1516(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SERVER_INFO_1518(i32);
#[repr(C)]
pub struct SERVER_INFO_1520(i32);
#[repr(C)]
pub struct SERVER_INFO_1521(i32);
#[repr(C)]
pub struct SERVER_INFO_1522(i32);
#[repr(C)]
pub struct SERVER_INFO_1523(i32);
#[repr(C)]
pub struct SERVER_INFO_1524(i32);
#[repr(C)]
pub struct SERVER_INFO_1525(i32);
#[repr(C)]
pub struct SERVER_INFO_1528(i32);
#[repr(C)]
pub struct SERVER_INFO_1529(i32);
#[repr(C)]
pub struct SERVER_INFO_1530(i32);
#[repr(C)]
pub struct SERVER_INFO_1533(i32);
#[repr(C)]
pub struct SERVER_INFO_1534(i32);
#[repr(C)]
pub struct SERVER_INFO_1535(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SERVER_INFO_1536(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SERVER_INFO_1537(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SERVER_INFO_1538(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SERVER_INFO_1539(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SERVER_INFO_1540(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SERVER_INFO_1541(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SERVER_INFO_1542(i32);
#[repr(C)]
pub struct SERVER_INFO_1543(i32);
#[repr(C)]
pub struct SERVER_INFO_1544(i32);
#[repr(C)]
pub struct SERVER_INFO_1545(i32);
#[repr(C)]
pub struct SERVER_INFO_1546(i32);
#[repr(C)]
pub struct SERVER_INFO_1547(i32);
#[repr(C)]
pub struct SERVER_INFO_1548(i32);
#[repr(C)]
pub struct SERVER_INFO_1549(i32);
#[repr(C)]
pub struct SERVER_INFO_1550(i32);
#[repr(C)]
pub struct SERVER_INFO_1552(i32);
#[repr(C)]
pub struct SERVER_INFO_1553(i32);
#[repr(C)]
pub struct SERVER_INFO_1554(i32);
#[repr(C)]
pub struct SERVER_INFO_1555(i32);
#[repr(C)]
pub struct SERVER_INFO_1556(i32);
#[repr(C)]
pub struct SERVER_INFO_1557(i32);
#[repr(C)]
pub struct SERVER_INFO_1560(i32);
#[repr(C)]
pub struct SERVER_INFO_1561(i32);
#[repr(C)]
pub struct SERVER_INFO_1562(i32);
#[repr(C)]
pub struct SERVER_INFO_1563(i32);
#[repr(C)]
pub struct SERVER_INFO_1564(i32);
#[repr(C)]
pub struct SERVER_INFO_1565(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SERVER_INFO_1566(i32);
#[repr(C)]
pub struct SERVER_INFO_1567(i32);
#[repr(C)]
pub struct SERVER_INFO_1568(i32);
#[repr(C)]
pub struct SERVER_INFO_1569(i32);
#[repr(C)]
pub struct SERVER_INFO_1570(i32);
#[repr(C)]
pub struct SERVER_INFO_1571(i32);
#[repr(C)]
pub struct SERVER_INFO_1572(i32);
#[repr(C)]
pub struct SERVER_INFO_1573(i32);
#[repr(C)]
pub struct SERVER_INFO_1574(i32);
#[repr(C)]
pub struct SERVER_INFO_1575(i32);
#[repr(C)]
pub struct SERVER_INFO_1576(i32);
#[repr(C)]
pub struct SERVER_INFO_1577(i32);
#[repr(C)]
pub struct SERVER_INFO_1578(i32);
#[repr(C)]
pub struct SERVER_INFO_1579(i32);
#[repr(C)]
pub struct SERVER_INFO_1580(i32);
#[repr(C)]
pub struct SERVER_INFO_1581(i32);
#[repr(C)]
pub struct SERVER_INFO_1582(i32);
#[repr(C)]
pub struct SERVER_INFO_1583(i32);
#[repr(C)]
pub struct SERVER_INFO_1584(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SERVER_INFO_1585(i32);
#[repr(C)]
pub struct SERVER_INFO_1586(i32);
#[repr(C)]
pub struct SERVER_INFO_1587(i32);
#[repr(C)]
pub struct SERVER_INFO_1588(i32);
#[repr(C)]
pub struct SERVER_INFO_1590(i32);
#[repr(C)]
pub struct SERVER_INFO_1591(i32);
#[repr(C)]
pub struct SERVER_INFO_1592(i32);
#[repr(C)]
pub struct SERVER_INFO_1593(i32);
#[repr(C)]
pub struct SERVER_INFO_1594(i32);
#[repr(C)]
pub struct SERVER_INFO_1595(i32);
#[repr(C)]
pub struct SERVER_INFO_1596(i32);
#[repr(C)]
pub struct SERVER_INFO_1597(i32);
#[repr(C)]
pub struct SERVER_INFO_1598(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SERVER_INFO_1599(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SERVER_INFO_1600(i32);
#[repr(C)]
pub struct SERVER_INFO_1601(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SERVER_INFO_1602(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SERVER_INFO_402(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SERVER_INFO_403(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SERVER_INFO_502(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SERVER_INFO_503(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SERVER_INFO_598(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SERVER_INFO_599(i32);
#[repr(transparent)]
pub struct SERVER_INFO_HIDDEN(pub u32);
pub const SV_VISIBLE: SERVER_INFO_HIDDEN = SERVER_INFO_HIDDEN(0u32);
pub const SV_HIDDEN: SERVER_INFO_HIDDEN = SERVER_INFO_HIDDEN(1u32);
#[repr(transparent)]
pub struct SERVER_INFO_SECURITY(pub u32);
pub const SV_SHARESECURITY: SERVER_INFO_SECURITY = SERVER_INFO_SECURITY(0u32);
pub const SV_USERSECURITY: SERVER_INFO_SECURITY = SERVER_INFO_SECURITY(1u32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SERVER_TRANSPORT_INFO_0(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SERVER_TRANSPORT_INFO_1(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SERVER_TRANSPORT_INFO_2(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SERVER_TRANSPORT_INFO_3(i32);
pub const SERVICE2_BASE: u32 = 5600u32;
pub const SERVICE_ACCOUNT_FLAG_ADD_AGAINST_RODC: i32 = 2i32;
pub const SERVICE_ACCOUNT_FLAG_LINK_TO_HOST_ONLY: i32 = 1i32;
pub const SERVICE_ACCOUNT_FLAG_REMOVE_OFFLINE: i32 = 2i32;
pub const SERVICE_ACCOUNT_FLAG_UNLINK_FROM_HOST_ONLY: i32 = 1i32;
pub const SERVICE_BASE: u32 = 3050u32;
pub const SERVICE_CCP_CHKPT_NUM: u32 = 255u32;
pub const SERVICE_CCP_NO_HINT: u32 = 0u32;
pub const SERVICE_CCP_QUERY_HINT: u32 = 65536u32;
pub const SERVICE_CCP_WAIT_TIME: u32 = 65280u32;
pub const SERVICE_CTRL_CONTINUE: u32 = 2u32;
pub const SERVICE_CTRL_INTERROGATE: u32 = 0u32;
pub const SERVICE_CTRL_PAUSE: u32 = 1u32;
pub const SERVICE_CTRL_REDIR_COMM: u32 = 4u32;
pub const SERVICE_CTRL_REDIR_DISK: u32 = 1u32;
pub const SERVICE_CTRL_REDIR_PRINT: u32 = 2u32;
pub const SERVICE_CTRL_UNINSTALL: u32 = 3u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SERVICE_INFO_0(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SERVICE_INFO_1(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SERVICE_INFO_2(i32);
pub const SERVICE_INSTALLED: u32 = 3u32;
pub const SERVICE_INSTALL_PENDING: u32 = 1u32;
pub const SERVICE_INSTALL_STATE: u32 = 3u32;
pub const SERVICE_IP_CHKPT_NUM: u32 = 255u32;
pub const SERVICE_IP_NO_HINT: u32 = 0u32;
pub const SERVICE_IP_QUERY_HINT: u32 = 65536u32;
pub const SERVICE_IP_WAITTIME_SHIFT: u32 = 8u32;
pub const SERVICE_IP_WAIT_TIME: u32 = 65280u32;
pub const SERVICE_MAXTIME: u32 = 255u32;
pub const SERVICE_NOT_PAUSABLE: u32 = 0u32;
pub const SERVICE_NOT_UNINSTALLABLE: u32 = 0u32;
pub const SERVICE_NTIP_WAITTIME_SHIFT: u32 = 12u32;
pub const SERVICE_NT_MAXTIME: u32 = 65535u32;
pub const SERVICE_PAUSABLE: u32 = 32u32;
pub const SERVICE_PAUSE_STATE: u32 = 12u32;
pub const SERVICE_REDIR_COMM_PAUSED: u32 = 1024u32;
pub const SERVICE_REDIR_DISK_PAUSED: u32 = 256u32;
pub const SERVICE_REDIR_PAUSED: u32 = 1792u32;
pub const SERVICE_REDIR_PRINT_PAUSED: u32 = 512u32;
pub const SERVICE_RESRV_MASK: u32 = 131071u32;
pub const SERVICE_UIC_AMBIGPARM: u32 = 3058u32;
pub const SERVICE_UIC_BADPARMVAL: u32 = 3051u32;
pub const SERVICE_UIC_CONFIG: u32 = 3055u32;
pub const SERVICE_UIC_CONFLPARM: u32 = 3063u32;
pub const SERVICE_UIC_DUPPARM: u32 = 3059u32;
pub const SERVICE_UIC_EXEC: u32 = 3061u32;
pub const SERVICE_UIC_FILE: u32 = 3064u32;
pub const SERVICE_UIC_INTERNAL: u32 = 3057u32;
pub const SERVICE_UIC_KILL: u32 = 3060u32;
pub const SERVICE_UIC_MISSPARM: u32 = 3052u32;
pub const SERVICE_UIC_M_ADDPAK: u32 = 3090u32;
pub const SERVICE_UIC_M_ANNOUNCE: u32 = 3083u32;
pub const SERVICE_UIC_M_DATABASE_ERROR: u32 = 5602u32;
pub const SERVICE_UIC_M_DISK: u32 = 3071u32;
pub const SERVICE_UIC_M_ERRLOG: u32 = 3088u32;
pub const SERVICE_UIC_M_FILES: u32 = 3079u32;
pub const SERVICE_UIC_M_FILE_UW: u32 = 3089u32;
pub const SERVICE_UIC_M_LANGROUP: u32 = 3081u32;
pub const SERVICE_UIC_M_LANROOT: u32 = 3075u32;
pub const SERVICE_UIC_M_LAZY: u32 = 3091u32;
pub const SERVICE_UIC_M_LOGS: u32 = 3080u32;
pub const SERVICE_UIC_M_LSA_MACHINE_ACCT: u32 = 5601u32;
pub const SERVICE_UIC_M_MEMORY: u32 = 3070u32;
pub const SERVICE_UIC_M_MSGNAME: u32 = 3082u32;
pub const SERVICE_UIC_M_NETLOGON_AUTH: u32 = 3098u32;
pub const SERVICE_UIC_M_NETLOGON_DC_CFLCT: u32 = 3097u32;
pub const SERVICE_UIC_M_NETLOGON_MPATH: u32 = 5600u32;
pub const SERVICE_UIC_M_NETLOGON_NO_DC: u32 = 3096u32;
pub const SERVICE_UIC_M_NULL: u32 = 0u32;
pub const SERVICE_UIC_M_PROCESSES: u32 = 3073u32;
pub const SERVICE_UIC_M_REDIR: u32 = 3076u32;
pub const SERVICE_UIC_M_SECURITY: u32 = 3074u32;
pub const SERVICE_UIC_M_SEC_FILE_ERR: u32 = 3078u32;
pub const SERVICE_UIC_M_SERVER: u32 = 3077u32;
pub const SERVICE_UIC_M_SERVER_SEC_ERR: u32 = 3085u32;
pub const SERVICE_UIC_M_THREADS: u32 = 3072u32;
pub const SERVICE_UIC_M_UAS: u32 = 3084u32;
pub const SERVICE_UIC_M_UAS_INVALID_ROLE: u32 = 3095u32;
pub const SERVICE_UIC_M_UAS_MACHINE_ACCT: u32 = 3092u32;
pub const SERVICE_UIC_M_UAS_PROLOG: u32 = 3099u32;
pub const SERVICE_UIC_M_UAS_SERVERS_NMEMB: u32 = 3093u32;
pub const SERVICE_UIC_M_UAS_SERVERS_NOGRP: u32 = 3094u32;
pub const SERVICE_UIC_M_WKSTA: u32 = 3087u32;
pub const SERVICE_UIC_NORMAL: u32 = 0u32;
pub const SERVICE_UIC_RESOURCE: u32 = 3054u32;
pub const SERVICE_UIC_SUBSERV: u32 = 3062u32;
pub const SERVICE_UIC_SYSTEM: u32 = 3056u32;
pub const SERVICE_UIC_UNKPARM: u32 = 3053u32;
pub const SERVICE_UNINSTALLABLE: u32 = 16u32;
pub const SERVICE_UNINSTALLED: u32 = 0u32;
pub const SERVICE_UNINSTALL_PENDING: u32 = 2u32;
pub const SESSION_CRYPT_KLEN: u32 = 21u32;
pub const SESSION_PWLEN: u32 = 24u32;
pub const SHPWLEN: u32 = 8u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SMB_COMPRESSION_INFO(i32);
#[repr(C)]
pub struct SMB_TREE_CONNECT_PARAMETERS(i32);
#[repr(C)]
pub struct SMB_USE_OPTION_COMPRESSION_PARAMETERS(i32);
pub const SNLEN: u32 = 80u32;
pub const SRV_HASH_GENERATION_ACTIVE: u32 = 2u32;
pub const SRV_SUPPORT_HASH_GENERATION: u32 = 1u32;
#[repr(C)]
pub struct STD_ALERT(i32);
pub const STXTLEN: u32 = 256u32;
pub const SUPPORTS_ANY: i32 = -1i32;
#[repr(transparent)]
pub struct SUPPORTS_BINDING_INTERFACE_FLAGS(pub i32);
pub const NCF_LOWER: SUPPORTS_BINDING_INTERFACE_FLAGS = SUPPORTS_BINDING_INTERFACE_FLAGS(1i32);
pub const NCF_UPPER: SUPPORTS_BINDING_INTERFACE_FLAGS = SUPPORTS_BINDING_INTERFACE_FLAGS(2i32);
pub const SVAUD_BADNETLOGON: u32 = 384u32;
pub const SVAUD_BADSESSLOGON: u32 = 24u32;
pub const SVAUD_BADUSE: u32 = 6144u32;
pub const SVAUD_GOODNETLOGON: u32 = 96u32;
pub const SVAUD_GOODSESSLOGON: u32 = 6u32;
pub const SVAUD_GOODUSE: u32 = 1536u32;
pub const SVAUD_LOGONLIM: u32 = 65536u32;
pub const SVAUD_PERMISSIONS: u32 = 16384u32;
pub const SVAUD_RESOURCE: u32 = 32768u32;
pub const SVAUD_SERVICE: u32 = 1u32;
pub const SVAUD_USERLIST: u32 = 8192u32;
pub const SVI1_NUM_ELEMENTS: u32 = 5u32;
pub const SVI2_NUM_ELEMENTS: u32 = 40u32;
pub const SVI3_NUM_ELEMENTS: u32 = 44u32;
pub const SVTI2_CLUSTER_DNN_NAME: u32 = 16u32;
pub const SVTI2_CLUSTER_NAME: u32 = 8u32;
pub const SVTI2_REMAP_PIPE_NAMES: u32 = 2u32;
pub const SVTI2_RESERVED1: u32 = 4096u32;
pub const SVTI2_RESERVED2: u32 = 8192u32;
pub const SVTI2_RESERVED3: u32 = 16384u32;
pub const SVTI2_SCOPED_NAME: u32 = 4u32;
pub const SVTI2_UNICODE_TRANSPORT_ADDRESS: u32 = 32u32;
pub const SV_ACCEPTDOWNLEVELAPIS_PARMNUM: u32 = 517u32;
pub const SV_ACCESSALERT_PARMNUM: u32 = 40u32;
pub const SV_ACTIVELOCKS_PARMNUM: u32 = 419u32;
pub const SV_ALERTSCHEDULE_PARMNUM: u32 = 547u32;
pub const SV_ALERTSCHED_PARMNUM: u32 = 37u32;
pub const SV_ALERTS_PARMNUM: u32 = 11u32;
pub const SV_ALIST_MTIME_PARMNUM: u32 = 403u32;
pub const SV_ANNDELTA_PARMNUM: u32 = 18u32;
pub const SV_ANNOUNCE_PARMNUM: u32 = 17u32;
pub const SV_AUTOSHARESERVER_PARMNUM: u32 = 592u32;
pub const SV_AUTOSHAREWKS_PARMNUM: u32 = 591u32;
pub const SV_BALANCECOUNT_PARMNUM: u32 = 577u32;
pub const SV_CACHEDDIRECTORYLIMIT_PARMNUM: u32 = 587u32;
pub const SV_CACHEDOPENLIMIT_PARMNUM: u32 = 571u32;
pub const SV_CHDEVJOBS_PARMNUM: u32 = 411u32;
pub const SV_CHDEVQ_PARMNUM: u32 = 410u32;
pub const SV_COMMENT_PARMNUM: u32 = 5u32;
pub const SV_CONNECTIONLESSAUTODISC_PARMNUM: u32 = 562u32;
pub const SV_CONNECTIONNOSESSIONSTIMEOUT_PARMNUM: u32 = 596u32;
pub const SV_CONNECTIONS_PARMNUM: u32 = 412u32;
pub const SV_CRITICALTHREADS_PARMNUM: u32 = 572u32;
pub const SV_DISABLEDOS_PARMNUM: u32 = 600u32;
pub const SV_DISABLESTRICTNAMECHECKING_PARMNUM: u32 = 602u32;
pub const SV_DISC_PARMNUM: u32 = 10u32;
pub const SV_DISKALERT_PARMNUM: u32 = 41u32;
pub const SV_DISKSPACETHRESHOLD_PARMNUM: u32 = 550u32;
pub const SV_DOMAIN_PARMNUM: u32 = 519u32;
pub const SV_ENABLEAUTHENTICATEUSERSHARING_PARMNUM: u32 = 603u32;
pub const SV_ENABLECOMPRESSION_PARMNUM: u32 = 590u32;
pub const SV_ENABLEFCBOPENS_PARMNUM: u32 = 538u32;
pub const SV_ENABLEFORCEDLOGOFF_PARMNUM: u32 = 515u32;
pub const SV_ENABLEOPLOCKFORCECLOSE_PARMNUM: u32 = 537u32;
pub const SV_ENABLEOPLOCKS_PARMNUM: u32 = 536u32;
pub const SV_ENABLERAW_PARMNUM: u32 = 539u32;
pub const SV_ENABLESECURITYSIGNATURE_PARMNUM: u32 = 593u32;
pub const SV_ENABLESHAREDNETDRIVES_PARMNUM: u32 = 540u32;
pub const SV_ENABLESOFTCOMPAT_PARMNUM: u32 = 514u32;
pub const SV_ENABLEW9XSECURITYSIGNATURE_PARMNUM: u32 = 598u32;
pub const SV_ENABLEWFW311DIRECTIPX_PARMNUM: u32 = 574u32;
pub const SV_ENFORCEKERBEROSREAUTHENTICATION_PARMNUM: u32 = 599u32;
pub const SV_ERRORALERT_PARMNUM: u32 = 38u32;
pub const SV_ERRORTHRESHOLD_PARMNUM: u32 = 548u32;
pub const SV_GLIST_MTIME_PARMNUM: u32 = 402u32;
pub const SV_GUESTACC_PARMNUM: u32 = 408u32;
pub const SV_HIDDEN_PARMNUM: u32 = 16u32;
pub const SV_IDLETHREADTIMEOUT_PARMNUM: u32 = 597u32;
pub const SV_INITCONNTABLE_PARMNUM: u32 = 544u32;
pub const SV_INITFILETABLE_PARMNUM: u32 = 545u32;
pub const SV_INITSEARCHTABLE_PARMNUM: u32 = 546u32;
pub const SV_INITSESSTABLE_PARMNUM: u32 = 543u32;
pub const SV_INITWORKITEMS_PARMNUM: u32 = 505u32;
pub const SV_IRPSTACKSIZE_PARMNUM: u32 = 508u32;
pub const SV_LANMASK_PARMNUM: u32 = 407u32;
pub const SV_LINKINFOVALIDTIME_PARMNUM: u32 = 554u32;
pub const SV_LMANNOUNCE_PARMNUM: u32 = 518u32;
pub const SV_LOCKVIOLATIONDELAY_PARMNUM: u32 = 569u32;
pub const SV_LOCKVIOLATIONOFFSET_PARMNUM: u32 = 568u32;
pub const SV_LOCKVIOLATIONRETRIES_PARMNUM: u32 = 567u32;
pub const SV_LOGONALERT_PARMNUM: u32 = 39u32;
pub const SV_LOWDISKSPACEMINIMUM_PARMNUM: u32 = 601u32;
pub const SV_MAXAUDITSZ_PARMNUM: u32 = 43u32;
pub const SV_MAXCOPYLENGTH_PARMNUM: u32 = 588u32;
pub const SV_MAXCOPYREADLEN_PARMNUM: u32 = 520u32;
pub const SV_MAXCOPYWRITELEN_PARMNUM: u32 = 521u32;
pub const SV_MAXFREECONNECTIONS_PARMNUM: u32 = 542u32;
pub const SV_MAXFREELFCBS_PARMNUM: u32 = 581u32;
pub const SV_MAXFREEMFCBS_PARMNUM: u32 = 580u32;
pub const SV_MAXFREEPAGEDPOOLCHUNKS_PARMNUM: u32 = 582u32;
pub const SV_MAXFREERFCBS_PARMNUM: u32 = 579u32;
pub const SV_MAXGLOBALOPENSEARCH_PARMNUM: u32 = 565u32;
pub const SV_MAXKEEPCOMPLSEARCH_PARMNUM: u32 = 525u32;
pub const SV_MAXKEEPSEARCH_PARMNUM: u32 = 523u32;
pub const SV_MAXLINKDELAY_PARMNUM: u32 = 552u32;
pub const SV_MAXMPXCT_PARMNUM: u32 = 533u32;
pub const SV_MAXNONPAGEDMEMORYUSAGE_PARMNUM: u32 = 512u32;
pub const SV_MAXPAGEDMEMORYUSAGE_PARMNUM: u32 = 513u32;
pub const SV_MAXPAGEDPOOLCHUNKSIZE_PARMNUM: u32 = 584u32;
pub const SV_MAXRAWBUFLEN_PARMNUM: u32 = 509u32;
pub const SV_MAXRAWWORKITEMS_PARMNUM: u32 = 557u32;
pub const SV_MAXTHREADSPERQUEUE_PARMNUM: u32 = 586u32;
pub const SV_MAXWORKITEMIDLETIME_PARMNUM: u32 = 556u32;
pub const SV_MAXWORKITEMS_PARMNUM: u32 = 506u32;
pub const SV_MAX_CMD_LEN: u32 = 256u32;
pub const SV_MAX_SRV_HEUR_LEN: u32 = 32u32;
pub const SV_MDLREADSWITCHOVER_PARMNUM: u32 = 570u32;
pub const SV_MINCLIENTBUFFERSIZE_PARMNUM: u32 = 595u32;
pub const SV_MINFREECONNECTIONS_PARMNUM: u32 = 541u32;
pub const SV_MINFREEWORKITEMS_PARMNUM: u32 = 530u32;
pub const SV_MINKEEPCOMPLSEARCH_PARMNUM: u32 = 524u32;
pub const SV_MINKEEPSEARCH_PARMNUM: u32 = 522u32;
pub const SV_MINLINKTHROUGHPUT_PARMNUM: u32 = 553u32;
pub const SV_MINPAGEDPOOLCHUNKSIZE_PARMNUM: u32 = 583u32;
pub const SV_MINRCVQUEUE_PARMNUM: u32 = 529u32;
pub const SV_NAME_PARMNUM: u32 = 102u32;
pub const SV_NETIOALERT_PARMNUM: u32 = 42u32;
pub const SV_NETWORKERRORTHRESHOLD_PARMNUM: u32 = 549u32;
pub const SV_NODISC: i32 = -1i32;
pub const SV_NUMADMIN_PARMNUM: u32 = 406u32;
pub const SV_NUMBIGBUF_PARMNUM: u32 = 422u32;
pub const SV_NUMBLOCKTHREADS_PARMNUM: u32 = 527u32;
pub const SV_NUMFILETASKS_PARMNUM: u32 = 423u32;
pub const SV_NUMREQBUF_PARMNUM: u32 = 420u32;
pub const SV_OPENFILES_PARMNUM: u32 = 414u32;
pub const SV_OPENSEARCH_PARMNUM: u32 = 503u32;
pub const SV_OPLOCKBREAKRESPONSEWAIT_PARMNUM: u32 = 535u32;
pub const SV_OPLOCKBREAKWAIT_PARMNUM: u32 = 534u32;
pub const SV_OTHERQUEUEAFFINITY_PARMNUM: u32 = 575u32;
pub const SV_PLATFORM_ID_NT: u32 = 500u32;
pub const SV_PLATFORM_ID_OS2: u32 = 400u32;
pub const SV_PLATFORM_ID_PARMNUM: u32 = 101u32;
pub const SV_PREFERREDAFFINITY_PARMNUM: u32 = 578u32;
pub const SV_PRODUCTTYPE_PARMNUM: u32 = 560u32;
pub const SV_QUEUESAMPLESECS_PARMNUM: u32 = 576u32;
pub const SV_RAWWORKITEMS_PARMNUM: u32 = 507u32;
pub const SV_REMOVEDUPLICATESEARCHES_PARMNUM: u32 = 566u32;
pub const SV_REQUIRESECURITYSIGNATURE_PARMNUM: u32 = 594u32;
pub const SV_RESTRICTNULLSESSACCESS_PARMNUM: u32 = 573u32;
pub const SV_SCAVQOSINFOUPDATETIME_PARMNUM: u32 = 555u32;
pub const SV_SCAVTIMEOUT_PARMNUM: u32 = 528u32;
pub const SV_SECURITY_PARMNUM: u32 = 405u32;
pub const SV_SENDSFROMPREFERREDPROCESSOR_PARMNUM: u32 = 585u32;
pub const SV_SERVERSIZE_PARMNUM: u32 = 561u32;
pub const SV_SESSCONNS_PARMNUM: u32 = 511u32;
pub const SV_SESSOPENS_PARMNUM: u32 = 501u32;
pub const SV_SESSREQS_PARMNUM: u32 = 417u32;
pub const SV_SESSUSERS_PARMNUM: u32 = 510u32;
pub const SV_SESSVCS_PARMNUM: u32 = 502u32;
pub const SV_SHARES_PARMNUM: u32 = 413u32;
pub const SV_SHARINGVIOLATIONDELAY_PARMNUM: u32 = 564u32;
pub const SV_SHARINGVIOLATIONRETRIES_PARMNUM: u32 = 563u32;
pub const SV_SIZREQBUF_PARMNUM: u32 = 504u32;
pub const SV_SRVHEURISTICS_PARMNUM: u32 = 431u32;
pub const SV_THREADCOUNTADD_PARMNUM: u32 = 526u32;
pub const SV_THREADPRIORITY_PARMNUM: u32 = 532u32;
pub const SV_TIMESOURCE_PARMNUM: u32 = 516u32;
pub const SV_TYPE_PARMNUM: u32 = 105u32;
pub const SV_ULIST_MTIME_PARMNUM: u32 = 401u32;
pub const SV_USERPATH_PARMNUM: u32 = 112u32;
pub const SV_USERS_PARMNUM: u32 = 107u32;
pub const SV_USERS_PER_LICENSE: u32 = 5u32;
pub const SV_VERSION_MAJOR_PARMNUM: u32 = 103u32;
pub const SV_VERSION_MINOR_PARMNUM: u32 = 104u32;
pub const SV_XACTMEMSIZE_PARMNUM: u32 = 531u32;
pub const SW_AUTOPROF_LOAD_MASK: u32 = 1u32;
pub const SW_AUTOPROF_SAVE_MASK: u32 = 2u32;
pub const ServiceAccountPasswordGUID: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 640588233, data2: 24928, data3: 18545, data4: [172, 236, 78, 97, 115, 107, 111, 33] };
#[repr(C)]
pub struct TIME_OF_DAY_INFO(i32);
pub const TITLE_SC_MESSAGE_BOX: i32 = -1073734795i32;
pub const TRACE_NO_STDINFO: u32 = 1u32;
pub const TRACE_NO_SYNCH: u32 = 4u32;
pub const TRACE_USE_CONSOLE: u32 = 2u32;
pub const TRACE_USE_DATE: u32 = 8u32;
pub const TRACE_USE_FILE: u32 = 1u32;
pub const TRACE_USE_MASK: u32 = 2u32;
pub const TRACE_USE_MSEC: u32 = 4u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct TRANSPORT_INFO(i32);
pub const TRANSPORT_NAME_PARMNUM: u32 = 202u32;
pub const TRANSPORT_QUALITYOFSERVICE_PARMNUM: u32 = 201u32;
#[repr(transparent)]
pub struct TRANSPORT_TYPE(pub i32);
pub const UseTransportType_None: TRANSPORT_TYPE = TRANSPORT_TYPE(0i32);
pub const UseTransportType_Wsk: TRANSPORT_TYPE = TRANSPORT_TYPE(1i32);
pub const UseTransportType_Quic: TRANSPORT_TYPE = TRANSPORT_TYPE(2i32);
pub const UF_INTERDOMAIN_TRUST_ACCOUNT: u32 = 2048u32;
pub const UF_MNS_LOGON_ACCOUNT: u32 = 131072u32;
pub const UF_NORMAL_ACCOUNT: u32 = 512u32;
pub const UF_NO_AUTH_DATA_REQUIRED: u32 = 33554432u32;
pub const UF_PARTIAL_SECRETS_ACCOUNT: u32 = 67108864u32;
pub const UF_SERVER_TRUST_ACCOUNT: u32 = 8192u32;
pub const UF_TEMP_DUPLICATE_ACCOUNT: u32 = 256u32;
pub const UF_USE_AES_KEYS: u32 = 134217728u32;
pub const UF_WORKSTATION_TRUST_ACCOUNT: u32 = 4096u32;
pub const UNCLEN: u32 = 17u32;
pub const UNITS_PER_DAY: u32 = 24u32;
pub const UNLEN: u32 = 256u32;
pub const UPPER_GET_HINT_MASK: u32 = 267386880u32;
pub const UPPER_HINT_MASK: u32 = 65280u32;
#[repr(transparent)]
pub struct USER_ACCOUNT_FLAGS(pub u32);
pub const UF_SCRIPT: USER_ACCOUNT_FLAGS = USER_ACCOUNT_FLAGS(1u32);
pub const UF_ACCOUNTDISABLE: USER_ACCOUNT_FLAGS = USER_ACCOUNT_FLAGS(2u32);
pub const UF_HOMEDIR_REQUIRED: USER_ACCOUNT_FLAGS = USER_ACCOUNT_FLAGS(8u32);
pub const UF_PASSWD_NOTREQD: USER_ACCOUNT_FLAGS = USER_ACCOUNT_FLAGS(32u32);
pub const UF_PASSWD_CANT_CHANGE: USER_ACCOUNT_FLAGS = USER_ACCOUNT_FLAGS(64u32);
pub const UF_LOCKOUT: USER_ACCOUNT_FLAGS = USER_ACCOUNT_FLAGS(16u32);
pub const UF_DONT_EXPIRE_PASSWD: USER_ACCOUNT_FLAGS = USER_ACCOUNT_FLAGS(65536u32);
pub const UF_ENCRYPTED_TEXT_PASSWORD_ALLOWED: USER_ACCOUNT_FLAGS = USER_ACCOUNT_FLAGS(128u32);
pub const UF_NOT_DELEGATED: USER_ACCOUNT_FLAGS = USER_ACCOUNT_FLAGS(1048576u32);
pub const UF_SMARTCARD_REQUIRED: USER_ACCOUNT_FLAGS = USER_ACCOUNT_FLAGS(262144u32);
pub const UF_USE_DES_KEY_ONLY: USER_ACCOUNT_FLAGS = USER_ACCOUNT_FLAGS(2097152u32);
pub const UF_DONT_REQUIRE_PREAUTH: USER_ACCOUNT_FLAGS = USER_ACCOUNT_FLAGS(4194304u32);
pub const UF_TRUSTED_FOR_DELEGATION: USER_ACCOUNT_FLAGS = USER_ACCOUNT_FLAGS(524288u32);
pub const UF_PASSWORD_EXPIRED: USER_ACCOUNT_FLAGS = USER_ACCOUNT_FLAGS(8388608u32);
pub const UF_TRUSTED_TO_AUTHENTICATE_FOR_DELEGATION: USER_ACCOUNT_FLAGS = USER_ACCOUNT_FLAGS(16777216u32);
pub const USER_ACCT_EXPIRES_PARMNUM: u32 = 17u32;
pub const USER_AUTH_FLAGS_PARMNUM: u32 = 10u32;
pub const USER_CODE_PAGE_PARMNUM: u32 = 25u32;
pub const USER_COMMENT_PARMNUM: u32 = 7u32;
pub const USER_COUNTRY_CODE_PARMNUM: u32 = 24u32;
pub const USER_FLAGS_PARMNUM: u32 = 8u32;
pub const USER_FULL_NAME_PARMNUM: u32 = 11u32;
pub const USER_HOME_DIR_DRIVE_PARMNUM: u32 = 53u32;
pub const USER_HOME_DIR_PARMNUM: u32 = 6u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct USER_INFO_0(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct USER_INFO_1(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct USER_INFO_10(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct USER_INFO_1003(i32);
#[repr(C)]
pub struct USER_INFO_1005(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct USER_INFO_1006(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct USER_INFO_1007(i32);
#[repr(C)]
pub struct USER_INFO_1008(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct USER_INFO_1009(i32);
#[repr(C)]
pub struct USER_INFO_1010(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct USER_INFO_1011(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct USER_INFO_1012(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct USER_INFO_1013(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct USER_INFO_1014(i32);
#[repr(C)]
pub struct USER_INFO_1017(i32);
#[repr(C)]
pub struct USER_INFO_1018(i32);
#[repr(C)]
pub struct USER_INFO_1020(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct USER_INFO_1023(i32);
#[repr(C)]
pub struct USER_INFO_1024(i32);
#[repr(C)]
pub struct USER_INFO_1025(i32);
#[repr(C)]
pub struct USER_INFO_1051(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct USER_INFO_1052(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct USER_INFO_1053(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct USER_INFO_11(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct USER_INFO_2(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct USER_INFO_20(i32);
#[repr(C)]
pub struct USER_INFO_21(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct USER_INFO_22(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct USER_INFO_23(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct USER_INFO_24(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct USER_INFO_3(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct USER_INFO_4(i32);
pub const USER_LAST_LOGOFF_PARMNUM: u32 = 16u32;
pub const USER_LAST_LOGON_PARMNUM: u32 = 15u32;
pub const USER_LOGON_HOURS_PARMNUM: u32 = 20u32;
pub const USER_LOGON_SERVER_PARMNUM: u32 = 23u32;
pub const USER_MAX_STORAGE_PARMNUM: u32 = 18u32;
#[repr(C)]
pub struct USER_MODALS_INFO_0(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct USER_MODALS_INFO_1(i32);
#[repr(C)]
pub struct USER_MODALS_INFO_1001(i32);
#[repr(C)]
pub struct USER_MODALS_INFO_1002(i32);
#[repr(C)]
pub struct USER_MODALS_INFO_1003(i32);
#[repr(C)]
pub struct USER_MODALS_INFO_1004(i32);
#[repr(C)]
pub struct USER_MODALS_INFO_1005(i32);
#[repr(C)]
pub struct USER_MODALS_INFO_1006(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct USER_MODALS_INFO_1007(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct USER_MODALS_INFO_2(i32);
#[repr(C)]
pub struct USER_MODALS_INFO_3(i32);
#[repr(transparent)]
pub struct USER_MODALS_ROLES(pub u32);
pub const UAS_ROLE_STANDALONE: USER_MODALS_ROLES = USER_MODALS_ROLES(0u32);
pub const UAS_ROLE_MEMBER: USER_MODALS_ROLES = USER_MODALS_ROLES(1u32);
pub const UAS_ROLE_BACKUP: USER_MODALS_ROLES = USER_MODALS_ROLES(2u32);
pub const UAS_ROLE_PRIMARY: USER_MODALS_ROLES = USER_MODALS_ROLES(3u32);
pub const USER_NAME_PARMNUM: u32 = 1u32;
pub const USER_NUM_LOGONS_PARMNUM: u32 = 22u32;
#[repr(C)]
pub struct USER_OTHER_INFO(i32);
pub const USER_PAD_PW_COUNT_PARMNUM: u32 = 21u32;
pub const USER_PARMS_PARMNUM: u32 = 13u32;
pub const USER_PASSWORD_AGE_PARMNUM: u32 = 4u32;
pub const USER_PASSWORD_PARMNUM: u32 = 3u32;
pub const USER_PRIMARY_GROUP_PARMNUM: u32 = 51u32;
#[repr(transparent)]
pub struct USER_PRIV(pub u32);
pub const USER_PRIV_GUEST: USER_PRIV = USER_PRIV(0u32);
pub const USER_PRIV_USER: USER_PRIV = USER_PRIV(1u32);
pub const USER_PRIV_ADMIN: USER_PRIV = USER_PRIV(2u32);
pub const USER_PRIV_MASK: u32 = 3u32;
pub const USER_PRIV_PARMNUM: u32 = 5u32;
pub const USER_PROFILE: u32 = 52u32;
pub const USER_PROFILE_PARMNUM: u32 = 52u32;
pub const USER_SCRIPT_PATH_PARMNUM: u32 = 9u32;
pub const USER_UNITS_PER_WEEK_PARMNUM: u32 = 19u32;
pub const USER_USR_COMMENT_PARMNUM: u32 = 12u32;
pub const USER_WORKSTATIONS_PARMNUM: u32 = 14u32;
pub const USE_ASGTYPE_PARMNUM: u32 = 4u32;
pub const USE_AUTHIDENTITY_PARMNUM: u32 = 8u32;
pub const USE_CHARDEV: u32 = 2u32;
pub const USE_CONN: u32 = 4u32;
pub const USE_DEFAULT_CREDENTIALS: u32 = 4u32;
pub const USE_DISCONN: u32 = 2u32;
pub const USE_DOMAINNAME_PARMNUM: u32 = 6u32;
pub const USE_FLAGS_PARMNUM: u32 = 7u32;
pub const USE_FLAG_GLOBAL_MAPPING: u32 = 65536u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct USE_INFO_0(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct USE_INFO_1(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct USE_INFO_2(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct USE_INFO_3(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct USE_INFO_4(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct USE_INFO_5(i32);
#[repr(transparent)]
pub struct USE_INFO_ASG_TYPE(pub u32);
pub const USE_WILDCARD: USE_INFO_ASG_TYPE = USE_INFO_ASG_TYPE(4294967295u32);
pub const USE_DISKDEV: USE_INFO_ASG_TYPE = USE_INFO_ASG_TYPE(0u32);
pub const USE_SPOOLDEV: USE_INFO_ASG_TYPE = USE_INFO_ASG_TYPE(1u32);
pub const USE_IPC: USE_INFO_ASG_TYPE = USE_INFO_ASG_TYPE(3u32);
pub const USE_LOCAL_PARMNUM: u32 = 1u32;
pub const USE_NETERR: u32 = 3u32;
pub const USE_OK: u32 = 0u32;
pub const USE_OPTIONS_PARMNUM: u32 = 10u32;
#[repr(C)]
pub struct USE_OPTION_DEFERRED_CONNECTION_PARAMETERS(i32);
#[repr(C)]
pub struct USE_OPTION_GENERIC(i32);
#[repr(C)]
pub struct USE_OPTION_PROPERTIES(i32);
#[repr(C)]
pub struct USE_OPTION_TRANSPORT_PARAMETERS(i32);
pub const USE_PASSWORD_PARMNUM: u32 = 3u32;
pub const USE_PAUSED: u32 = 1u32;
pub const USE_RECONN: u32 = 5u32;
pub const USE_REMOTE_PARMNUM: u32 = 2u32;
pub const USE_SD_PARMNUM: u32 = 9u32;
pub const USE_SESSLOST: u32 = 2u32;
pub const USE_SPECIFIC_TRANSPORT: u32 = 2147483648u32;
pub const USE_USERNAME_PARMNUM: u32 = 5u32;
pub const VALIDATED_LOGON: u32 = 0u32;
pub const VALID_LOGOFF: u32 = 1u32;
pub const WKSTA_BUFFERNAMEDPIPES_PARMNUM: u32 = 51u32;
pub const WKSTA_BUFFERREADONLYFILES_PARMNUM: u32 = 59u32;
pub const WKSTA_BUFFILESWITHDENYWRITE_PARMNUM: u32 = 58u32;
pub const WKSTA_CACHEFILETIMEOUT_PARMNUM: u32 = 47u32;
pub const WKSTA_CHARCOUNT_PARMNUM: u32 = 12u32;
pub const WKSTA_CHARTIME_PARMNUM: u32 = 11u32;
pub const WKSTA_CHARWAIT_PARMNUM: u32 = 10u32;
pub const WKSTA_COMPUTERNAME_PARMNUM: u32 = 1u32;
pub const WKSTA_DORMANTFILELIMIT_PARMNUM: u32 = 46u32;
pub const WKSTA_ERRLOGSZ_PARMNUM: u32 = 27u32;
pub const WKSTA_FORCECORECREATEMODE_PARMNUM: u32 = 60u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WKSTA_INFO_100(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WKSTA_INFO_101(i32);
#[repr(C)]
pub struct WKSTA_INFO_1010(i32);
#[repr(C)]
pub struct WKSTA_INFO_1011(i32);
#[repr(C)]
pub struct WKSTA_INFO_1012(i32);
#[repr(C)]
pub struct WKSTA_INFO_1013(i32);
#[repr(C)]
pub struct WKSTA_INFO_1018(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WKSTA_INFO_102(i32);
#[repr(C)]
pub struct WKSTA_INFO_1023(i32);
#[repr(C)]
pub struct WKSTA_INFO_1027(i32);
#[repr(C)]
pub struct WKSTA_INFO_1028(i32);
#[repr(C)]
pub struct WKSTA_INFO_1032(i32);
#[repr(C)]
pub struct WKSTA_INFO_1033(i32);
#[repr(C)]
pub struct WKSTA_INFO_1041(i32);
#[repr(C)]
pub struct WKSTA_INFO_1042(i32);
#[repr(C)]
pub struct WKSTA_INFO_1043(i32);
#[repr(C)]
pub struct WKSTA_INFO_1044(i32);
#[repr(C)]
pub struct WKSTA_INFO_1045(i32);
#[repr(C)]
pub struct WKSTA_INFO_1046(i32);
#[repr(C)]
pub struct WKSTA_INFO_1047(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WKSTA_INFO_1048(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WKSTA_INFO_1049(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WKSTA_INFO_1050(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WKSTA_INFO_1051(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WKSTA_INFO_1052(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WKSTA_INFO_1053(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WKSTA_INFO_1054(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WKSTA_INFO_1055(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WKSTA_INFO_1056(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WKSTA_INFO_1057(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WKSTA_INFO_1058(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WKSTA_INFO_1059(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WKSTA_INFO_1060(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WKSTA_INFO_1061(i32);
#[repr(C)]
pub struct WKSTA_INFO_1062(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WKSTA_INFO_302(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WKSTA_INFO_402(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WKSTA_INFO_502(i32);
pub const WKSTA_KEEPCONN_PARMNUM: u32 = 13u32;
pub const WKSTA_KEEPSEARCH_PARMNUM: u32 = 14u32;
pub const WKSTA_LANGROUP_PARMNUM: u32 = 2u32;
pub const WKSTA_LANROOT_PARMNUM: u32 = 7u32;
pub const WKSTA_LOCKINCREMENT_PARMNUM: u32 = 42u32;
pub const WKSTA_LOCKMAXIMUM_PARMNUM: u32 = 43u32;
pub const WKSTA_LOCKQUOTA_PARMNUM: u32 = 41u32;
pub const WKSTA_LOGGED_ON_USERS_PARMNUM: u32 = 6u32;
pub const WKSTA_LOGON_DOMAIN_PARMNUM: u32 = 8u32;
pub const WKSTA_LOGON_SERVER_PARMNUM: u32 = 9u32;
pub const WKSTA_MAILSLOTS_PARMNUM: u32 = 30u32;
pub const WKSTA_MAXCMDS_PARMNUM: u32 = 15u32;
pub const WKSTA_MAXTHREADS_PARMNUM: u32 = 33u32;
pub const WKSTA_MAXWRKCACHE_PARMNUM: u32 = 17u32;
pub const WKSTA_NUMALERTS_PARMNUM: u32 = 20u32;
pub const WKSTA_NUMCHARBUF_PARMNUM: u32 = 22u32;
pub const WKSTA_NUMDGRAMBUF_PARMNUM: u32 = 31u32;
pub const WKSTA_NUMSERVICES_PARMNUM: u32 = 21u32;
pub const WKSTA_NUMWORKBUF_PARMNUM: u32 = 16u32;
pub const WKSTA_OTH_DOMAINS_PARMNUM: u32 = 101u32;
pub const WKSTA_PIPEINCREMENT_PARMNUM: u32 = 44u32;
pub const WKSTA_PIPEMAXIMUM_PARMNUM: u32 = 45u32;
pub const WKSTA_PLATFORM_ID_PARMNUM: u32 = 100u32;
pub const WKSTA_PRINTBUFTIME_PARMNUM: u32 = 28u32;
pub const WKSTA_READAHEADTHRUPUT_PARMNUM: u32 = 62u32;
pub const WKSTA_SESSTIMEOUT_PARMNUM: u32 = 18u32;
pub const WKSTA_SIZCHARBUF_PARMNUM: u32 = 23u32;
pub const WKSTA_SIZERROR_PARMNUM: u32 = 19u32;
pub const WKSTA_SIZWORKBUF_PARMNUM: u32 = 29u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WKSTA_TRANSPORT_INFO_0(i32);
pub const WKSTA_USE512BYTESMAXTRANSFER_PARMNUM: u32 = 61u32;
pub const WKSTA_USECLOSEBEHIND_PARMNUM: u32 = 50u32;
pub const WKSTA_USEENCRYPTION_PARMNUM: u32 = 57u32;
pub const WKSTA_USELOCKANDREADANDUNLOCK_PARMNUM: u32 = 52u32;
pub const WKSTA_USEOPPORTUNISTICLOCKING_PARMNUM: u32 = 48u32;
pub const WKSTA_USERAWREAD_PARMNUM: u32 = 54u32;
pub const WKSTA_USERAWWRITE_PARMNUM: u32 = 55u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WKSTA_USER_INFO_0(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WKSTA_USER_INFO_1(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WKSTA_USER_INFO_1101(i32);
pub const WKSTA_USEUNLOCKBEHIND_PARMNUM: u32 = 49u32;
pub const WKSTA_USEWRITERAWWITHDATA_PARMNUM: u32 = 56u32;
pub const WKSTA_UTILIZENTCACHING_PARMNUM: u32 = 53u32;
pub const WKSTA_VER_MAJOR_PARMNUM: u32 = 4u32;
pub const WKSTA_VER_MINOR_PARMNUM: u32 = 5u32;
pub const WKSTA_WRKHEURISTICS_PARMNUM: u32 = 32u32;
pub type WORKERFUNCTION = unsafe extern "system" fn(param0: *mut ::core::ffi::c_void);
pub const WZC_PROFILE_API_ERROR_FAILED_TO_LOAD_SCHEMA: u32 = 34u32;
pub const WZC_PROFILE_API_ERROR_FAILED_TO_LOAD_XML: u32 = 33u32;
pub const WZC_PROFILE_API_ERROR_INTERNAL: u32 = 36u32;
pub const WZC_PROFILE_API_ERROR_NOT_SUPPORTED: u32 = 32u32;
pub const WZC_PROFILE_API_ERROR_XML_VALIDATION_FAILED: u32 = 35u32;
pub const WZC_PROFILE_CONFIG_ERROR_1X_NOT_ALLOWED: u32 = 20u32;
pub const WZC_PROFILE_CONFIG_ERROR_1X_NOT_ALLOWED_KEY_REQUIRED: u32 = 21u32;
pub const WZC_PROFILE_CONFIG_ERROR_1X_NOT_ENABLED_KEY_PROVIDED: u32 = 22u32;
pub const WZC_PROFILE_CONFIG_ERROR_EAP_METHOD_NOT_APPLICABLE: u32 = 24u32;
pub const WZC_PROFILE_CONFIG_ERROR_EAP_METHOD_REQUIRED: u32 = 23u32;
pub const WZC_PROFILE_CONFIG_ERROR_INVALID_AUTH_FOR_CONNECTION_TYPE: u32 = 15u32;
pub const WZC_PROFILE_CONFIG_ERROR_INVALID_ENCRYPTION_FOR_AUTHMODE: u32 = 16u32;
pub const WZC_PROFILE_CONFIG_ERROR_KEY_INDEX_NOT_APPLICABLE: u32 = 19u32;
pub const WZC_PROFILE_CONFIG_ERROR_KEY_INDEX_REQUIRED: u32 = 18u32;
pub const WZC_PROFILE_CONFIG_ERROR_KEY_REQUIRED: u32 = 17u32;
pub const WZC_PROFILE_CONFIG_ERROR_WPA_ENCRYPTION_NOT_SUPPORTED: u32 = 26u32;
pub const WZC_PROFILE_CONFIG_ERROR_WPA_NOT_SUPPORTED: u32 = 25u32;
pub const WZC_PROFILE_SET_ERROR_DUPLICATE_NETWORK: u32 = 27u32;
pub const WZC_PROFILE_SET_ERROR_MEMORY_ALLOCATION: u32 = 28u32;
pub const WZC_PROFILE_SET_ERROR_READING_1X_CONFIG: u32 = 29u32;
pub const WZC_PROFILE_SET_ERROR_WRITING_1X_CONFIG: u32 = 30u32;
pub const WZC_PROFILE_SET_ERROR_WRITING_WZC_CFG: u32 = 31u32;
pub const WZC_PROFILE_SUCCESS: u32 = 0u32;
pub const WZC_PROFILE_XML_ERROR_1X_ENABLED: u32 = 10u32;
pub const WZC_PROFILE_XML_ERROR_AUTHENTICATION: u32 = 7u32;
pub const WZC_PROFILE_XML_ERROR_BAD_KEY_INDEX: u32 = 12u32;
pub const WZC_PROFILE_XML_ERROR_BAD_NETWORK_KEY: u32 = 14u32;
pub const WZC_PROFILE_XML_ERROR_BAD_SSID: u32 = 5u32;
pub const WZC_PROFILE_XML_ERROR_BAD_VERSION: u32 = 2u32;
pub const WZC_PROFILE_XML_ERROR_CONNECTION_TYPE: u32 = 6u32;
pub const WZC_PROFILE_XML_ERROR_EAP_METHOD: u32 = 11u32;
pub const WZC_PROFILE_XML_ERROR_ENCRYPTION: u32 = 8u32;
pub const WZC_PROFILE_XML_ERROR_KEY_INDEX_RANGE: u32 = 13u32;
pub const WZC_PROFILE_XML_ERROR_KEY_PROVIDED_AUTOMATICALLY: u32 = 9u32;
pub const WZC_PROFILE_XML_ERROR_NO_VERSION: u32 = 1u32;
pub const WZC_PROFILE_XML_ERROR_SSID_NOT_FOUND: u32 = 4u32;
pub const WZC_PROFILE_XML_ERROR_UNSUPPORTED_VERSION: u32 = 3u32;
#[repr(transparent)]
pub struct tagRASCON_IPUI_FLAGS(pub i32);
pub const RCUIF_VPN: tagRASCON_IPUI_FLAGS = tagRASCON_IPUI_FLAGS(1i32);
pub const RCUIF_DEMAND_DIAL: tagRASCON_IPUI_FLAGS = tagRASCON_IPUI_FLAGS(2i32);
pub const RCUIF_NOT_ADMIN: tagRASCON_IPUI_FLAGS = tagRASCON_IPUI_FLAGS(4i32);
pub const RCUIF_USE_IPv4_STATICADDRESS: tagRASCON_IPUI_FLAGS = tagRASCON_IPUI_FLAGS(8i32);
pub const RCUIF_USE_IPv4_NAME_SERVERS: tagRASCON_IPUI_FLAGS = tagRASCON_IPUI_FLAGS(16i32);
pub const RCUIF_USE_IPv4_REMOTE_GATEWAY: tagRASCON_IPUI_FLAGS = tagRASCON_IPUI_FLAGS(32i32);
pub const RCUIF_USE_IPv4_EXPLICIT_METRIC: tagRASCON_IPUI_FLAGS = tagRASCON_IPUI_FLAGS(64i32);
pub const RCUIF_USE_HEADER_COMPRESSION: tagRASCON_IPUI_FLAGS = tagRASCON_IPUI_FLAGS(128i32);
pub const RCUIF_USE_DISABLE_REGISTER_DNS: tagRASCON_IPUI_FLAGS = tagRASCON_IPUI_FLAGS(256i32);
pub const RCUIF_USE_PRIVATE_DNS_SUFFIX: tagRASCON_IPUI_FLAGS = tagRASCON_IPUI_FLAGS(512i32);
pub const RCUIF_ENABLE_NBT: tagRASCON_IPUI_FLAGS = tagRASCON_IPUI_FLAGS(1024i32);
pub const RCUIF_USE_IPv6_STATICADDRESS: tagRASCON_IPUI_FLAGS = tagRASCON_IPUI_FLAGS(2048i32);
pub const RCUIF_USE_IPv6_NAME_SERVERS: tagRASCON_IPUI_FLAGS = tagRASCON_IPUI_FLAGS(4096i32);
pub const RCUIF_USE_IPv6_REMOTE_GATEWAY: tagRASCON_IPUI_FLAGS = tagRASCON_IPUI_FLAGS(8192i32);
pub const RCUIF_USE_IPv6_EXPLICIT_METRIC: tagRASCON_IPUI_FLAGS = tagRASCON_IPUI_FLAGS(16384i32);
pub const RCUIF_DISABLE_CLASS_BASED_ROUTE: tagRASCON_IPUI_FLAGS = tagRASCON_IPUI_FLAGS(32768i32);
