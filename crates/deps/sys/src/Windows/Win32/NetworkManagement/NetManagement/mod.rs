#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNetScheduleAccountInformation();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn I_NetLogonControl2();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LogErrorA();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LogErrorW();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LogEventA();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LogEventW();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`*"]
    pub fn MprSetupProtocolEnum();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`*"]
    pub fn MprSetupProtocolFree();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetAccessAdd();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetAccessDel();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetAccessEnum();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetAccessGetInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetAccessGetUserPerms();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetAccessSetInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetAddAlternateComputerName();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetAddServiceAccount();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetAlertRaise();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetAlertRaiseEx();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`*"]
    pub fn NetApiBufferAllocate();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`*"]
    pub fn NetApiBufferFree();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`*"]
    pub fn NetApiBufferReallocate();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`*"]
    pub fn NetApiBufferSize();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetAuditClear();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetAuditRead();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetAuditWrite();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetConfigGet();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetConfigGetAll();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetConfigSet();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetCreateProvisioningPackage();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetEnumerateComputerNames();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetEnumerateServiceAccounts();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetErrorLogClear();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetErrorLogRead();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetErrorLogWrite();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`, `Win32_Security_Cryptography`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub fn NetFreeAadJoinInformation();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`, `Win32_Security_Cryptography`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub fn NetGetAadJoinInformation();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetGetAnyDCName();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetGetDCName();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetGetDisplayInformationIndex();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetGetJoinInformation();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetGetJoinableOUs();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetGroupAdd();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetGroupAddUser();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetGroupDel();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetGroupDelUser();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetGroupEnum();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetGroupGetInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetGroupGetUsers();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetGroupSetInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetGroupSetUsers();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetIsServiceAccount();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetJoinDomain();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetLocalGroupAdd();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetLocalGroupAddMember();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetLocalGroupAddMembers();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetLocalGroupDel();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetLocalGroupDelMember();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetLocalGroupDelMembers();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetLocalGroupEnum();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetLocalGroupGetInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetLocalGroupGetMembers();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetLocalGroupSetInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetLocalGroupSetMembers();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetMessageBufferSend();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetMessageNameAdd();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetMessageNameDel();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetMessageNameEnum();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetMessageNameGetInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetProvisionComputerAccount();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetQueryDisplayInformation();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetQueryServiceAccount();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetRemoteComputerSupports();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetRemoteTOD();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetRemoveAlternateComputerName();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetRemoveServiceAccount();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetRenameMachineInDomain();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetReplExportDirAdd();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetReplExportDirDel();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetReplExportDirEnum();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetReplExportDirGetInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetReplExportDirLock();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetReplExportDirSetInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetReplExportDirUnlock();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetReplGetInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetReplImportDirAdd();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetReplImportDirDel();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetReplImportDirEnum();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetReplImportDirGetInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetReplImportDirLock();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetReplImportDirUnlock();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetReplSetInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetRequestOfflineDomainJoin();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetRequestProvisioningPackageInstall();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetScheduleJobAdd();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetScheduleJobDel();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetScheduleJobEnum();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetScheduleJobGetInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetServerComputerNameAdd();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetServerComputerNameDel();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetServerDiskEnum();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetServerEnum();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetServerGetInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetServerSetInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetServerTransportAdd();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetServerTransportAddEx();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetServerTransportDel();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetServerTransportEnum();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetServiceControl();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetServiceEnum();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetServiceGetInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetServiceInstall();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetSetPrimaryComputerName();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetUnjoinDomain();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`*"]
    pub fn NetUseAdd();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetUseDel();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetUseEnum();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetUseGetInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetUserAdd();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetUserChangePassword();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetUserDel();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetUserEnum();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetUserGetGroups();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetUserGetInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetUserGetLocalGroups();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetUserModalsGet();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetUserModalsSet();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetUserSetGroups();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetUserSetInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetValidateName();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetValidatePasswordPolicy();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`*"]
    pub fn NetValidatePasswordPolicyFree();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetWkstaGetInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetWkstaSetInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`*"]
    pub fn NetWkstaTransportAdd();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetWkstaTransportDel();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`*"]
    pub fn NetWkstaTransportEnum();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetWkstaUserEnum();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetWkstaUserGetInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetWkstaUserSetInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RouterAssert();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RouterGetErrorStringA();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RouterGetErrorStringW();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RouterLogDeregisterA();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RouterLogDeregisterW();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RouterLogEventA();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RouterLogEventDataA();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RouterLogEventDataW();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RouterLogEventExA();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RouterLogEventExW();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RouterLogEventStringA();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RouterLogEventStringW();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RouterLogEventValistExA();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RouterLogEventValistExW();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RouterLogEventW();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RouterLogRegisterA();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RouterLogRegisterW();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetNetScheduleAccountInformation();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`*"]
    pub fn TraceDeregisterA();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`*"]
    pub fn TraceDeregisterExA();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`*"]
    pub fn TraceDeregisterExW();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`*"]
    pub fn TraceDeregisterW();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TraceDumpExA();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TraceDumpExW();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TraceGetConsoleA();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TraceGetConsoleW();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TracePrintfA();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TracePrintfExA();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TracePrintfExW();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TracePrintfW();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TracePutsExA();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TracePutsExW();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TraceRegisterExA();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TraceRegisterExW();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TraceVprintfExA();
    #[doc = "*Required features: `Win32_NetworkManagement_NetManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TraceVprintfExW();
}
