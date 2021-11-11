#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNetScheduleAccountInformation(pwszservername: super::super::Foundation::PWSTR, ccaccount: u32, wszaccount: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn I_NetLogonControl2(servername: super::super::Foundation::PWSTR, functioncode: u32, querylevel: u32, data: *const u8, buffer: *mut *mut u8) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LogErrorA(dwmessageid: u32, cnumberofsubstrings: u32, plpwssubstrings: *const super::super::Foundation::PSTR, dwerrorcode: u32);
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LogErrorW(dwmessageid: u32, cnumberofsubstrings: u32, plpwssubstrings: *const super::super::Foundation::PWSTR, dwerrorcode: u32);
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LogEventA(weventtype: u32, dwmessageid: u32, cnumberofsubstrings: u32, plpwssubstrings: *const super::super::Foundation::PSTR);
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LogEventW(weventtype: u32, dwmessageid: u32, cnumberofsubstrings: u32, plpwssubstrings: *const super::super::Foundation::PWSTR);
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`*"]
    pub fn MprSetupProtocolEnum(dwtransportid: u32, lplpbuffer: *mut *mut u8, lpdwentriesread: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`*"]
    pub fn MprSetupProtocolFree(lpbuffer: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetAccessAdd(servername: super::super::Foundation::PWSTR, level: u32, buf: *const u8, parm_err: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetAccessDel(servername: super::super::Foundation::PWSTR, resource: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetAccessEnum(servername: super::super::Foundation::PWSTR, basepath: super::super::Foundation::PWSTR, recursive: u32, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resume_handle: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetAccessGetInfo(servername: super::super::Foundation::PWSTR, resource: super::super::Foundation::PWSTR, level: u32, bufptr: *mut *mut u8) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetAccessGetUserPerms(servername: super::super::Foundation::PWSTR, ugname: super::super::Foundation::PWSTR, resource: super::super::Foundation::PWSTR, perms: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetAccessSetInfo(servername: super::super::Foundation::PWSTR, resource: super::super::Foundation::PWSTR, level: u32, buf: *const u8, parm_err: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetAddAlternateComputerName(server: super::super::Foundation::PWSTR, alternatename: super::super::Foundation::PWSTR, domainaccount: super::super::Foundation::PWSTR, domainaccountpassword: super::super::Foundation::PWSTR, reserved: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetAddServiceAccount(servername: super::super::Foundation::PWSTR, accountname: super::super::Foundation::PWSTR, password: super::super::Foundation::PWSTR, flags: u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetAlertRaise(alerttype: super::super::Foundation::PWSTR, buffer: *const ::core::ffi::c_void, buffersize: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetAlertRaiseEx(alerttype: super::super::Foundation::PWSTR, variableinfo: *const ::core::ffi::c_void, variableinfosize: u32, servicename: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`*"]
    pub fn NetApiBufferAllocate(bytecount: u32, buffer: *mut *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`*"]
    pub fn NetApiBufferFree(buffer: *const ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`*"]
    pub fn NetApiBufferReallocate(oldbuffer: *const ::core::ffi::c_void, newbytecount: u32, newbuffer: *mut *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`*"]
    pub fn NetApiBufferSize(buffer: *const ::core::ffi::c_void, bytecount: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetAuditClear(server: super::super::Foundation::PWSTR, backupfile: super::super::Foundation::PWSTR, service: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetAuditRead(server: super::super::Foundation::PWSTR, service: super::super::Foundation::PWSTR, auditloghandle: *mut HLOG, offset: u32, reserved1: *mut u32, reserved2: u32, offsetflag: u32, bufptr: *mut *mut u8, prefmaxlen: u32, bytesread: *mut u32, totalavailable: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetAuditWrite(r#type: u32, buf: *mut u8, numbytes: u32, service: super::super::Foundation::PWSTR, reserved: *mut u8) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetConfigGet(server: super::super::Foundation::PWSTR, component: super::super::Foundation::PWSTR, parameter: super::super::Foundation::PWSTR, bufptr: *mut *mut u8) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetConfigGetAll(server: super::super::Foundation::PWSTR, component: super::super::Foundation::PWSTR, bufptr: *mut *mut u8) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetConfigSet(server: super::super::Foundation::PWSTR, reserved1: super::super::Foundation::PWSTR, component: super::super::Foundation::PWSTR, level: u32, reserved2: u32, buf: *mut u8, reserved3: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetCreateProvisioningPackage(pprovisioningparams: *const NETSETUP_PROVISIONING_PARAMS, pppackagebindata: *mut *mut u8, pdwpackagebindatasize: *mut u32, pppackagetextdata: *mut super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetEnumerateComputerNames(server: super::super::Foundation::PWSTR, nametype: NET_COMPUTER_NAME_TYPE, reserved: u32, entrycount: *mut u32, computernames: *mut *mut super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetEnumerateServiceAccounts(servername: super::super::Foundation::PWSTR, flags: u32, accountscount: *mut u32, accounts: *mut *mut *mut u16) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetErrorLogClear(uncservername: super::super::Foundation::PWSTR, backupfile: super::super::Foundation::PWSTR, reserved: *const u8) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetErrorLogRead(uncservername: super::super::Foundation::PWSTR, reserved1: super::super::Foundation::PWSTR, errorloghandle: *const HLOG, offset: u32, reserved2: *const u32, reserved3: u32, offsetflag: u32, bufptr: *mut *mut u8, prefmaxsize: u32, bytesread: *mut u32, totalavailable: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetErrorLogWrite(reserved1: *const u8, code: u32, component: super::super::Foundation::PWSTR, buffer: *const u8, numbytes: u32, msgbuf: *const u8, strcount: u32, reserved2: *const u8) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`, `Win32_Security_Cryptography`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub fn NetFreeAadJoinInformation(pjoininfo: *const DSREG_JOIN_INFO);
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`, `Win32_Security_Cryptography`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub fn NetGetAadJoinInformation(pcsztenantid: super::super::Foundation::PWSTR, ppjoininfo: *mut *mut DSREG_JOIN_INFO) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetGetAnyDCName(servername: super::super::Foundation::PWSTR, domainname: super::super::Foundation::PWSTR, buffer: *mut *mut u8) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetGetDCName(servername: super::super::Foundation::PWSTR, domainname: super::super::Foundation::PWSTR, buffer: *mut *mut u8) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetGetDisplayInformationIndex(servername: super::super::Foundation::PWSTR, level: u32, prefix: super::super::Foundation::PWSTR, index: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetGetJoinInformation(lpserver: super::super::Foundation::PWSTR, lpnamebuffer: *mut super::super::Foundation::PWSTR, buffertype: *mut NETSETUP_JOIN_STATUS) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetGetJoinableOUs(lpserver: super::super::Foundation::PWSTR, lpdomain: super::super::Foundation::PWSTR, lpaccount: super::super::Foundation::PWSTR, lppassword: super::super::Foundation::PWSTR, oucount: *mut u32, ous: *mut *mut super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetGroupAdd(servername: super::super::Foundation::PWSTR, level: u32, buf: *const u8, parm_err: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetGroupAddUser(servername: super::super::Foundation::PWSTR, groupname: super::super::Foundation::PWSTR, username: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetGroupDel(servername: super::super::Foundation::PWSTR, groupname: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetGroupDelUser(servername: super::super::Foundation::PWSTR, groupname: super::super::Foundation::PWSTR, username: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetGroupEnum(servername: super::super::Foundation::PWSTR, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resume_handle: *mut usize) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetGroupGetInfo(servername: super::super::Foundation::PWSTR, groupname: super::super::Foundation::PWSTR, level: u32, bufptr: *mut *mut u8) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetGroupGetUsers(servername: super::super::Foundation::PWSTR, groupname: super::super::Foundation::PWSTR, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resumehandle: *mut usize) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetGroupSetInfo(servername: super::super::Foundation::PWSTR, groupname: super::super::Foundation::PWSTR, level: u32, buf: *const u8, parm_err: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetGroupSetUsers(servername: super::super::Foundation::PWSTR, groupname: super::super::Foundation::PWSTR, level: u32, buf: *const u8, totalentries: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetIsServiceAccount(servername: super::super::Foundation::PWSTR, accountname: super::super::Foundation::PWSTR, isservice: *mut super::super::Foundation::BOOL) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetJoinDomain(lpserver: super::super::Foundation::PWSTR, lpdomain: super::super::Foundation::PWSTR, lpmachineaccountou: super::super::Foundation::PWSTR, lpaccount: super::super::Foundation::PWSTR, lppassword: super::super::Foundation::PWSTR, fjoinoptions: NET_JOIN_DOMAIN_JOIN_OPTIONS) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetLocalGroupAdd(servername: super::super::Foundation::PWSTR, level: u32, buf: *const u8, parm_err: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetLocalGroupAddMember(servername: super::super::Foundation::PWSTR, groupname: super::super::Foundation::PWSTR, membersid: super::super::Foundation::PSID) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetLocalGroupAddMembers(servername: super::super::Foundation::PWSTR, groupname: super::super::Foundation::PWSTR, level: u32, buf: *const u8, totalentries: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetLocalGroupDel(servername: super::super::Foundation::PWSTR, groupname: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetLocalGroupDelMember(servername: super::super::Foundation::PWSTR, groupname: super::super::Foundation::PWSTR, membersid: super::super::Foundation::PSID) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetLocalGroupDelMembers(servername: super::super::Foundation::PWSTR, groupname: super::super::Foundation::PWSTR, level: u32, buf: *const u8, totalentries: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetLocalGroupEnum(servername: super::super::Foundation::PWSTR, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resumehandle: *mut usize) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetLocalGroupGetInfo(servername: super::super::Foundation::PWSTR, groupname: super::super::Foundation::PWSTR, level: u32, bufptr: *mut *mut u8) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetLocalGroupGetMembers(servername: super::super::Foundation::PWSTR, localgroupname: super::super::Foundation::PWSTR, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resumehandle: *mut usize) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetLocalGroupSetInfo(servername: super::super::Foundation::PWSTR, groupname: super::super::Foundation::PWSTR, level: u32, buf: *const u8, parm_err: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetLocalGroupSetMembers(servername: super::super::Foundation::PWSTR, groupname: super::super::Foundation::PWSTR, level: u32, buf: *const u8, totalentries: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetMessageBufferSend(servername: super::super::Foundation::PWSTR, msgname: super::super::Foundation::PWSTR, fromname: super::super::Foundation::PWSTR, buf: *const u8, buflen: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetMessageNameAdd(servername: super::super::Foundation::PWSTR, msgname: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetMessageNameDel(servername: super::super::Foundation::PWSTR, msgname: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetMessageNameEnum(servername: super::super::Foundation::PWSTR, level: u32, bufptr: *const *const u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resume_handle: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetMessageNameGetInfo(servername: super::super::Foundation::PWSTR, msgname: super::super::Foundation::PWSTR, level: u32, bufptr: *const *const u8) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetProvisionComputerAccount(lpdomain: super::super::Foundation::PWSTR, lpmachinename: super::super::Foundation::PWSTR, lpmachineaccountou: super::super::Foundation::PWSTR, lpdcname: super::super::Foundation::PWSTR, dwoptions: NETSETUP_PROVISION, pprovisionbindata: *mut *mut u8, pdwprovisionbindatasize: *mut u32, pprovisiontextdata: *mut super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetQueryDisplayInformation(servername: super::super::Foundation::PWSTR, level: u32, index: u32, entriesrequested: u32, preferredmaximumlength: u32, returnedentrycount: *mut u32, sortedbuffer: *mut *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetQueryServiceAccount(servername: super::super::Foundation::PWSTR, accountname: super::super::Foundation::PWSTR, infolevel: u32, buffer: *mut *mut u8) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetRemoteComputerSupports(uncservername: super::super::Foundation::PWSTR, optionswanted: NET_REMOTE_COMPUTER_SUPPORTS_OPTIONS, optionssupported: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetRemoteTOD(uncservername: super::super::Foundation::PWSTR, bufferptr: *mut *mut u8) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetRemoveAlternateComputerName(server: super::super::Foundation::PWSTR, alternatename: super::super::Foundation::PWSTR, domainaccount: super::super::Foundation::PWSTR, domainaccountpassword: super::super::Foundation::PWSTR, reserved: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetRemoveServiceAccount(servername: super::super::Foundation::PWSTR, accountname: super::super::Foundation::PWSTR, flags: u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetRenameMachineInDomain(lpserver: super::super::Foundation::PWSTR, lpnewmachinename: super::super::Foundation::PWSTR, lpaccount: super::super::Foundation::PWSTR, lppassword: super::super::Foundation::PWSTR, frenameoptions: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetReplExportDirAdd(servername: super::super::Foundation::PWSTR, level: u32, buf: *const u8, parm_err: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetReplExportDirDel(servername: super::super::Foundation::PWSTR, dirname: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetReplExportDirEnum(servername: super::super::Foundation::PWSTR, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resumehandle: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetReplExportDirGetInfo(servername: super::super::Foundation::PWSTR, dirname: super::super::Foundation::PWSTR, level: u32, bufptr: *mut *mut u8) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetReplExportDirLock(servername: super::super::Foundation::PWSTR, dirname: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetReplExportDirSetInfo(servername: super::super::Foundation::PWSTR, dirname: super::super::Foundation::PWSTR, level: u32, buf: *const u8, parm_err: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetReplExportDirUnlock(servername: super::super::Foundation::PWSTR, dirname: super::super::Foundation::PWSTR, unlockforce: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetReplGetInfo(servername: super::super::Foundation::PWSTR, level: u32, bufptr: *mut *mut u8) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetReplImportDirAdd(servername: super::super::Foundation::PWSTR, level: u32, buf: *const u8, parm_err: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetReplImportDirDel(servername: super::super::Foundation::PWSTR, dirname: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetReplImportDirEnum(servername: super::super::Foundation::PWSTR, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resumehandle: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetReplImportDirGetInfo(servername: super::super::Foundation::PWSTR, dirname: super::super::Foundation::PWSTR, level: u32, bufptr: *mut *mut u8) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetReplImportDirLock(servername: super::super::Foundation::PWSTR, dirname: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetReplImportDirUnlock(servername: super::super::Foundation::PWSTR, dirname: super::super::Foundation::PWSTR, unlockforce: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetReplSetInfo(servername: super::super::Foundation::PWSTR, level: u32, buf: *const u8, parm_err: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetRequestOfflineDomainJoin(pprovisionbindata: *const u8, cbprovisionbindatasize: u32, dwoptions: NET_REQUEST_PROVISION_OPTIONS, lpwindowspath: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetRequestProvisioningPackageInstall(ppackagebindata: *const u8, dwpackagebindatasize: u32, dwprovisionoptions: NET_REQUEST_PROVISION_OPTIONS, lpwindowspath: super::super::Foundation::PWSTR, pvreserved: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetScheduleJobAdd(servername: super::super::Foundation::PWSTR, buffer: *mut u8, jobid: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetScheduleJobDel(servername: super::super::Foundation::PWSTR, minjobid: u32, maxjobid: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetScheduleJobEnum(servername: super::super::Foundation::PWSTR, pointertobuffer: *mut *mut u8, prefferedmaximumlength: u32, entriesread: *mut u32, totalentries: *mut u32, resumehandle: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetScheduleJobGetInfo(servername: super::super::Foundation::PWSTR, jobid: u32, pointertobuffer: *mut *mut u8) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetServerComputerNameAdd(servername: super::super::Foundation::PWSTR, emulateddomainname: super::super::Foundation::PWSTR, emulatedservername: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetServerComputerNameDel(servername: super::super::Foundation::PWSTR, emulatedservername: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetServerDiskEnum(servername: super::super::Foundation::PWSTR, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resume_handle: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetServerEnum(servername: super::super::Foundation::PWSTR, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, servertype: NET_SERVER_TYPE, domain: super::super::Foundation::PWSTR, resume_handle: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetServerGetInfo(servername: super::super::Foundation::PWSTR, level: u32, bufptr: *mut *mut u8) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetServerSetInfo(servername: super::super::Foundation::PWSTR, level: u32, buf: *const u8, parmerror: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetServerTransportAdd(servername: super::super::Foundation::PWSTR, level: u32, bufptr: *const u8) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetServerTransportAddEx(servername: super::super::Foundation::PWSTR, level: u32, bufptr: *const u8) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetServerTransportDel(servername: super::super::Foundation::PWSTR, level: u32, bufptr: *const u8) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetServerTransportEnum(servername: super::super::Foundation::PWSTR, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resume_handle: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetServiceControl(servername: super::super::Foundation::PWSTR, service: super::super::Foundation::PWSTR, opcode: u32, arg: u32, bufptr: *mut *mut u8) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetServiceEnum(servername: super::super::Foundation::PWSTR, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resume_handle: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetServiceGetInfo(servername: super::super::Foundation::PWSTR, service: super::super::Foundation::PWSTR, level: u32, bufptr: *mut *mut u8) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetServiceInstall(servername: super::super::Foundation::PWSTR, service: super::super::Foundation::PWSTR, argc: u32, argv: *const super::super::Foundation::PWSTR, bufptr: *mut *mut u8) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetSetPrimaryComputerName(server: super::super::Foundation::PWSTR, primaryname: super::super::Foundation::PWSTR, domainaccount: super::super::Foundation::PWSTR, domainaccountpassword: super::super::Foundation::PWSTR, reserved: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetUnjoinDomain(lpserver: super::super::Foundation::PWSTR, lpaccount: super::super::Foundation::PWSTR, lppassword: super::super::Foundation::PWSTR, funjoinoptions: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`*"]
    pub fn NetUseAdd(servername: *const i8, levelflags: u32, buf: *const u8, parm_err: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetUseDel(uncservername: super::super::Foundation::PWSTR, usename: super::super::Foundation::PWSTR, forcelevelflags: FORCE_LEVEL_FLAGS) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetUseEnum(uncservername: super::super::Foundation::PWSTR, levelflags: u32, bufptr: *mut *mut u8, preferedmaximumsize: u32, entriesread: *mut u32, totalentries: *mut u32, resumehandle: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetUseGetInfo(uncservername: super::super::Foundation::PWSTR, usename: super::super::Foundation::PWSTR, levelflags: u32, bufptr: *mut *mut u8) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetUserAdd(servername: super::super::Foundation::PWSTR, level: u32, buf: *const u8, parm_err: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetUserChangePassword(domainname: super::super::Foundation::PWSTR, username: super::super::Foundation::PWSTR, oldpassword: super::super::Foundation::PWSTR, newpassword: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetUserDel(servername: super::super::Foundation::PWSTR, username: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetUserEnum(servername: super::super::Foundation::PWSTR, level: u32, filter: NET_USER_ENUM_FILTER_FLAGS, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resume_handle: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetUserGetGroups(servername: super::super::Foundation::PWSTR, username: super::super::Foundation::PWSTR, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetUserGetInfo(servername: super::super::Foundation::PWSTR, username: super::super::Foundation::PWSTR, level: u32, bufptr: *mut *mut u8) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetUserGetLocalGroups(servername: super::super::Foundation::PWSTR, username: super::super::Foundation::PWSTR, level: u32, flags: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetUserModalsGet(servername: super::super::Foundation::PWSTR, level: u32, bufptr: *mut *mut u8) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetUserModalsSet(servername: super::super::Foundation::PWSTR, level: u32, buf: *const u8, parm_err: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetUserSetGroups(servername: super::super::Foundation::PWSTR, username: super::super::Foundation::PWSTR, level: u32, buf: *const u8, num_entries: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetUserSetInfo(servername: super::super::Foundation::PWSTR, username: super::super::Foundation::PWSTR, level: u32, buf: *const u8, parm_err: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetValidateName(lpserver: super::super::Foundation::PWSTR, lpname: super::super::Foundation::PWSTR, lpaccount: super::super::Foundation::PWSTR, lppassword: super::super::Foundation::PWSTR, nametype: NETSETUP_NAME_TYPE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetValidatePasswordPolicy(servername: super::super::Foundation::PWSTR, qualifier: *mut ::core::ffi::c_void, validationtype: NET_VALIDATE_PASSWORD_TYPE, inputarg: *mut ::core::ffi::c_void, outputarg: *mut *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`*"]
    pub fn NetValidatePasswordPolicyFree(outputarg: *mut *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetWkstaGetInfo(servername: super::super::Foundation::PWSTR, level: u32, bufptr: *mut *mut u8) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetWkstaSetInfo(servername: super::super::Foundation::PWSTR, level: u32, buffer: *const u8, parm_err: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`*"]
    pub fn NetWkstaTransportAdd(servername: *const i8, level: u32, buf: *const u8, parm_err: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetWkstaTransportDel(servername: super::super::Foundation::PWSTR, transportname: super::super::Foundation::PWSTR, ucond: FORCE_LEVEL_FLAGS) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`*"]
    pub fn NetWkstaTransportEnum(servername: *const i8, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resume_handle: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetWkstaUserEnum(servername: super::super::Foundation::PWSTR, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resumehandle: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetWkstaUserGetInfo(reserved: super::super::Foundation::PWSTR, level: u32, bufptr: *mut *mut u8) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetWkstaUserSetInfo(reserved: super::super::Foundation::PWSTR, level: u32, buf: *const u8, parm_err: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RouterAssert(pszfailedassertion: super::super::Foundation::PSTR, pszfilename: super::super::Foundation::PSTR, dwlinenumber: u32, pszmessage: super::super::Foundation::PSTR);
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RouterGetErrorStringA(dwerrorcode: u32, lplpszerrorstring: *mut super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RouterGetErrorStringW(dwerrorcode: u32, lplpwszerrorstring: *mut super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RouterLogDeregisterA(hloghandle: super::super::Foundation::HANDLE);
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RouterLogDeregisterW(hloghandle: super::super::Foundation::HANDLE);
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RouterLogEventA(hloghandle: super::super::Foundation::HANDLE, dweventtype: u32, dwmessageid: u32, dwsubstringcount: u32, plpszsubstringarray: *const super::super::Foundation::PSTR, dwerrorcode: u32);
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RouterLogEventDataA(hloghandle: super::super::Foundation::HANDLE, dweventtype: u32, dwmessageid: u32, dwsubstringcount: u32, plpszsubstringarray: *const super::super::Foundation::PSTR, dwdatabytes: u32, lpdatabytes: *mut u8);
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RouterLogEventDataW(hloghandle: super::super::Foundation::HANDLE, dweventtype: u32, dwmessageid: u32, dwsubstringcount: u32, plpszsubstringarray: *const super::super::Foundation::PWSTR, dwdatabytes: u32, lpdatabytes: *mut u8);
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RouterLogEventExA(hloghandle: super::super::Foundation::HANDLE, dweventtype: u32, dwerrorcode: u32, dwmessageid: u32, ptszformat: super::super::Foundation::PSTR);
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RouterLogEventExW(hloghandle: super::super::Foundation::HANDLE, dweventtype: u32, dwerrorcode: u32, dwmessageid: u32, ptszformat: super::super::Foundation::PWSTR);
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RouterLogEventStringA(hloghandle: super::super::Foundation::HANDLE, dweventtype: u32, dwmessageid: u32, dwsubstringcount: u32, plpszsubstringarray: *const super::super::Foundation::PSTR, dwerrorcode: u32, dwerrorindex: u32);
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RouterLogEventStringW(hloghandle: super::super::Foundation::HANDLE, dweventtype: u32, dwmessageid: u32, dwsubstringcount: u32, plpszsubstringarray: *const super::super::Foundation::PWSTR, dwerrorcode: u32, dwerrorindex: u32);
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RouterLogEventValistExA(hloghandle: super::super::Foundation::HANDLE, dweventtype: u32, dwerrorcode: u32, dwmessageid: u32, ptszformat: super::super::Foundation::PSTR, arglist: *mut i8);
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RouterLogEventValistExW(hloghandle: super::super::Foundation::HANDLE, dweventtype: u32, dwerrorcode: u32, dwmessageid: u32, ptszformat: super::super::Foundation::PWSTR, arglist: *mut i8);
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RouterLogEventW(hloghandle: super::super::Foundation::HANDLE, dweventtype: u32, dwmessageid: u32, dwsubstringcount: u32, plpszsubstringarray: *const super::super::Foundation::PWSTR, dwerrorcode: u32);
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RouterLogRegisterA(lpszsource: super::super::Foundation::PSTR) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RouterLogRegisterW(lpszsource: super::super::Foundation::PWSTR) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetNetScheduleAccountInformation(pwszservername: super::super::Foundation::PWSTR, pwszaccount: super::super::Foundation::PWSTR, pwszpassword: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`*"]
    pub fn TraceDeregisterA(dwtraceid: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`*"]
    pub fn TraceDeregisterExA(dwtraceid: u32, dwflags: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`*"]
    pub fn TraceDeregisterExW(dwtraceid: u32, dwflags: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`*"]
    pub fn TraceDeregisterW(dwtraceid: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TraceDumpExA(dwtraceid: u32, dwflags: u32, lpbbytes: *mut u8, dwbytecount: u32, dwgroupsize: u32, baddressprefix: super::super::Foundation::BOOL, lpszprefix: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TraceDumpExW(dwtraceid: u32, dwflags: u32, lpbbytes: *mut u8, dwbytecount: u32, dwgroupsize: u32, baddressprefix: super::super::Foundation::BOOL, lpszprefix: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TraceGetConsoleA(dwtraceid: u32, lphconsole: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TraceGetConsoleW(dwtraceid: u32, lphconsole: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TracePrintfA(dwtraceid: u32, lpszformat: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TracePrintfExA(dwtraceid: u32, dwflags: u32, lpszformat: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TracePrintfExW(dwtraceid: u32, dwflags: u32, lpszformat: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TracePrintfW(dwtraceid: u32, lpszformat: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TracePutsExA(dwtraceid: u32, dwflags: u32, lpszstring: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TracePutsExW(dwtraceid: u32, dwflags: u32, lpszstring: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TraceRegisterExA(lpszcallername: super::super::Foundation::PSTR, dwflags: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TraceRegisterExW(lpszcallername: super::super::Foundation::PWSTR, dwflags: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TraceVprintfExA(dwtraceid: u32, dwflags: u32, lpszformat: super::super::Foundation::PSTR, arglist: *mut i8) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TraceVprintfExW(dwtraceid: u32, dwflags: u32, lpszformat: super::super::Foundation::PWSTR, arglist: *mut i8) -> u32;
}
