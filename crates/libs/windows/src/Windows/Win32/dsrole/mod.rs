#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn DsRoleFreeMemory(buffer: super::PVOID) {
    windows_core::link!("netapi32.dll" "system" fn DsRoleFreeMemory(buffer : super::PVOID));
    unsafe { DsRoleFreeMemory(buffer as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn DsRoleGetPrimaryDomainInformation(lpserver: &windows_core::PCWSTR, infolevel: DSROLE_PRIMARY_DOMAIN_INFO_LEVEL, buffer: *mut super::PBYTE) -> super::DWORD {
    windows_core::link!("netapi32.dll" "system" fn DsRoleGetPrimaryDomainInformation(lpserver : windows_core::PCWSTR, infolevel : DSROLE_PRIMARY_DOMAIN_INFO_LEVEL, buffer : *mut super::PBYTE) -> super::DWORD);
    unsafe { DsRoleGetPrimaryDomainInformation(core::mem::transmute_copy(lpserver), infolevel, buffer as _) }
}
pub type DSROLE_MACHINE_ROLE = i32;
pub type DSROLE_OPERATION_STATE = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DSROLE_OPERATION_STATE_INFO {
    pub OperationState: DSROLE_OPERATION_STATE,
}
pub const DSROLE_PRIMARY_DOMAIN_GUID_PRESENT: i32 = 16777216;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DSROLE_PRIMARY_DOMAIN_INFO_BASIC {
    pub MachineRole: DSROLE_MACHINE_ROLE,
    pub Flags: super::ULONG,
    pub DomainNameFlat: windows_core::PWSTR,
    pub DomainNameDns: windows_core::PWSTR,
    pub DomainForestName: windows_core::PWSTR,
    pub DomainGuid: windows_core::GUID,
}
pub type DSROLE_PRIMARY_DOMAIN_INFO_LEVEL = i32;
pub const DSROLE_PRIMARY_DS_MIXED_MODE: i32 = 2;
pub const DSROLE_PRIMARY_DS_READONLY: i32 = 8;
pub const DSROLE_PRIMARY_DS_RUNNING: i32 = 1;
pub type DSROLE_SERVER_STATE = i32;
pub const DSROLE_UPGRADE_IN_PROGRESS: i32 = 4;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DSROLE_UPGRADE_STATUS_INFO {
    pub OperationState: super::ULONG,
    pub PreviousServerState: DSROLE_SERVER_STATE,
}
pub const DsRoleOperationActive: DSROLE_OPERATION_STATE = 1;
pub const DsRoleOperationIdle: DSROLE_OPERATION_STATE = 0;
pub const DsRoleOperationNeedReboot: DSROLE_OPERATION_STATE = 2;
pub const DsRoleOperationState: DSROLE_PRIMARY_DOMAIN_INFO_LEVEL = 3;
pub const DsRolePrimaryDomainInfoBasic: DSROLE_PRIMARY_DOMAIN_INFO_LEVEL = 1;
pub const DsRoleServerBackup: DSROLE_SERVER_STATE = 2;
pub const DsRoleServerPrimary: DSROLE_SERVER_STATE = 1;
pub const DsRoleServerUnknown: DSROLE_SERVER_STATE = 0;
pub const DsRoleUpgradeStatus: DSROLE_PRIMARY_DOMAIN_INFO_LEVEL = 2;
pub const DsRole_RoleBackupDomainController: DSROLE_MACHINE_ROLE = 4;
pub const DsRole_RoleMemberServer: DSROLE_MACHINE_ROLE = 3;
pub const DsRole_RoleMemberWorkstation: DSROLE_MACHINE_ROLE = 1;
pub const DsRole_RolePrimaryDomainController: DSROLE_MACHINE_ROLE = 5;
pub const DsRole_RoleStandaloneServer: DSROLE_MACHINE_ROLE = 2;
pub const DsRole_RoleStandaloneWorkstation: DSROLE_MACHINE_ROLE = 0;
pub type PDSROLE_OPERATION_STATE_INFO = *mut DSROLE_OPERATION_STATE_INFO;
#[cfg(feature = "shared")]
pub type PDSROLE_PRIMARY_DOMAIN_INFO_BASIC = *mut DSROLE_PRIMARY_DOMAIN_INFO_BASIC;
pub type PDSROLE_SERVER_STATE = *mut DSROLE_SERVER_STATE;
#[cfg(feature = "shared")]
pub type PDSROLE_UPGRADE_STATUS_INFO = *mut DSROLE_UPGRADE_STATUS_INFO;
