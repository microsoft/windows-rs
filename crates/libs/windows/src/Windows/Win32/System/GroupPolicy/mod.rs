pub const ABSENT: APPSTATE = 0i32;
pub const ADMXCOMMENTS_EXTENSION_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x6c5a2a86_9eb3_42b9_aa83_a7371ba011b9);
pub const APPNAME: INSTALLSPECTYPE = 1i32;
pub const ASSIGNED: APPSTATE = 1i32;
pub const CLSID_GPESnapIn: windows_core::GUID = windows_core::GUID::from_u128(0x8fc0b734_a0e1_11d1_a7d3_0000f87571e3);
pub const CLSID_GroupPolicyObject: windows_core::GUID = windows_core::GUID::from_u128(0xea502722_a23d_11d1_a7d3_0000f87571e3);
pub const CLSID_RSOPSnapIn: windows_core::GUID = windows_core::GUID::from_u128(0x6dc3804b_7212_458d_adb0_9a07e2ae1fa2);
pub const COMCLASS: INSTALLSPECTYPE = 4i32;
pub const FILEEXT: INSTALLSPECTYPE = 2i32;
pub const FLAG_ASSUME_COMP_WQLFILTER_TRUE: u32 = 33554432u32;
pub const FLAG_ASSUME_SLOW_LINK: u32 = 536870912u32;
pub const FLAG_ASSUME_USER_WQLFILTER_TRUE: u32 = 67108864u32;
pub const FLAG_FORCE_CREATENAMESPACE: u32 = 4u32;
pub const FLAG_LOOPBACK_MERGE: u32 = 268435456u32;
pub const FLAG_LOOPBACK_REPLACE: u32 = 134217728u32;
pub const FLAG_NO_COMPUTER: u32 = 2u32;
pub const FLAG_NO_CSE_INVOKE: u32 = 1073741824u32;
pub const FLAG_NO_GPO_FILTER: u32 = 2147483648u32;
pub const FLAG_NO_USER: u32 = 1u32;
pub const FLAG_PLANNING_MODE: u32 = 16777216u32;
pub const GPC_BLOCK_POLICY: u32 = 1u32;
pub const GPHintDomain: GROUP_POLICY_HINT_TYPE = 3i32;
pub const GPHintMachine: GROUP_POLICY_HINT_TYPE = 1i32;
pub const GPHintOrganizationalUnit: GROUP_POLICY_HINT_TYPE = 4i32;
pub const GPHintSite: GROUP_POLICY_HINT_TYPE = 2i32;
pub const GPHintUnknown: GROUP_POLICY_HINT_TYPE = 0i32;
pub const GPLinkDomain: GPO_LINK = 3i32;
pub const GPLinkMachine: GPO_LINK = 1i32;
pub const GPLinkOrganizationalUnit: GPO_LINK = 4i32;
pub const GPLinkSite: GPO_LINK = 2i32;
pub const GPLinkUnknown: GPO_LINK = 0i32;
pub const GPM_DONOTUSE_W2KDC: u32 = 2u32;
pub const GPM_DONOT_VALIDATEDC: u32 = 1u32;
pub const GPM_MIGRATIONTABLE_ONLY: u32 = 1u32;
pub const GPM_PROCESS_SECURITY: u32 = 2u32;
pub const GPM_USE_ANYDC: u32 = 1u32;
pub const GPM_USE_PDC: u32 = 0u32;
pub const GPOTypeDS: GROUP_POLICY_OBJECT_TYPE = 2i32;
pub const GPOTypeLocal: GROUP_POLICY_OBJECT_TYPE = 0i32;
pub const GPOTypeLocalGroup: GROUP_POLICY_OBJECT_TYPE = 4i32;
pub const GPOTypeLocalUser: GROUP_POLICY_OBJECT_TYPE = 3i32;
pub const GPOTypeRemote: GROUP_POLICY_OBJECT_TYPE = 1i32;
pub const GPO_BROWSE_DISABLENEW: u32 = 1u32;
pub const GPO_BROWSE_INITTOALL: u32 = 16u32;
pub const GPO_BROWSE_NOCOMPUTERS: u32 = 2u32;
pub const GPO_BROWSE_NODSGPOS: u32 = 4u32;
pub const GPO_BROWSE_NOUSERGPOS: u32 = 32u32;
pub const GPO_BROWSE_OPENBUTTON: u32 = 8u32;
pub const GPO_BROWSE_SENDAPPLYONEDIT: u32 = 64u32;
pub const GPO_FLAG_DISABLE: u32 = 1u32;
pub const GPO_FLAG_FORCE: u32 = 2u32;
pub const GPO_INFO_FLAG_ASYNC_FOREGROUND: u32 = 4096u32;
pub const GPO_INFO_FLAG_BACKGROUND: u32 = 16u32;
pub const GPO_INFO_FLAG_FORCED_REFRESH: u32 = 1024u32;
pub const GPO_INFO_FLAG_LINKTRANSITION: u32 = 256u32;
pub const GPO_INFO_FLAG_LOGRSOP_TRANSITION: u32 = 512u32;
pub const GPO_INFO_FLAG_MACHINE: u32 = 1u32;
pub const GPO_INFO_FLAG_NOCHANGES: u32 = 128u32;
pub const GPO_INFO_FLAG_SAFEMODE_BOOT: u32 = 2048u32;
pub const GPO_INFO_FLAG_SLOWLINK: u32 = 32u32;
pub const GPO_INFO_FLAG_VERBOSE: u32 = 64u32;
pub const GPO_LIST_FLAG_MACHINE: u32 = 1u32;
pub const GPO_LIST_FLAG_NO_SECURITYFILTERS: u32 = 8u32;
pub const GPO_LIST_FLAG_NO_WMIFILTERS: u32 = 4u32;
pub const GPO_LIST_FLAG_SITEONLY: u32 = 2u32;
pub const GPO_OPEN_LOAD_REGISTRY: GPO_OPEN_FLAGS = 1u32;
pub const GPO_OPEN_READ_ONLY: GPO_OPEN_FLAGS = 2u32;
pub const GPO_OPTION_DISABLE_MACHINE: GPO_OPTIONS = 2u32;
pub const GPO_OPTION_DISABLE_USER: GPO_OPTIONS = 1u32;
pub const GPO_SECTION_MACHINE: GPO_SECTION = 2u32;
pub const GPO_SECTION_ROOT: GPO_SECTION = 0u32;
pub const GPO_SECTION_USER: GPO_SECTION = 1u32;
pub const GP_DLLNAME: windows_core::PCWSTR = windows_core::w!("DllName");
pub const GP_ENABLEASYNCHRONOUSPROCESSING: windows_core::PCWSTR = windows_core::w!("EnableAsynchronousProcessing");
pub const GP_MAXNOGPOLISTCHANGESINTERVAL: windows_core::PCWSTR = windows_core::w!("MaxNoGPOListChangesInterval");
pub const GP_NOBACKGROUNDPOLICY: windows_core::PCWSTR = windows_core::w!("NoBackgroundPolicy");
pub const GP_NOGPOLISTCHANGES: windows_core::PCWSTR = windows_core::w!("NoGPOListChanges");
pub const GP_NOMACHINEPOLICY: windows_core::PCWSTR = windows_core::w!("NoMachinePolicy");
pub const GP_NOSLOWLINK: windows_core::PCWSTR = windows_core::w!("NoSlowLink");
pub const GP_NOTIFYLINKTRANSITION: windows_core::PCWSTR = windows_core::w!("NotifyLinkTransition");
pub const GP_NOUSERPOLICY: windows_core::PCWSTR = windows_core::w!("NoUserPolicy");
pub const GP_PERUSERLOCALSETTINGS: windows_core::PCWSTR = windows_core::w!("PerUserLocalSettings");
pub const GP_PROCESSGROUPPOLICY: windows_core::PCWSTR = windows_core::w!("ProcessGroupPolicy");
pub const GP_REQUIRESSUCCESSFULREGISTRY: windows_core::PCWSTR = windows_core::w!("RequiresSuccessfulRegistry");
pub const GROUP_POLICY_TRIGGER_EVENT_PROVIDER_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xbd2f4252_5e1e_49fc_9a30_f3978ad89ee2);
pub const LOCALSTATE_ASSIGNED: u32 = 1u32;
pub const LOCALSTATE_ORPHANED: u32 = 32u32;
pub const LOCALSTATE_POLICYREMOVE_ORPHAN: u32 = 8u32;
pub const LOCALSTATE_POLICYREMOVE_UNINSTALL: u32 = 16u32;
pub const LOCALSTATE_PUBLISHED: u32 = 2u32;
pub const LOCALSTATE_UNINSTALLED: u32 = 64u32;
pub const LOCALSTATE_UNINSTALL_UNMANAGED: u32 = 4u32;
pub const MACHINE_POLICY_PRESENT_TRIGGER_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x659fcae6_5bdb_4da9_b1ff_ca2a178d46e0);
pub const MANAGED_APPS_FROMCATEGORY: u32 = 2u32;
pub const MANAGED_APPS_INFOLEVEL_DEFAULT: u32 = 65536u32;
pub const MANAGED_APPS_USERAPPLICATIONS: u32 = 1u32;
pub const MANAGED_APPTYPE_SETUPEXE: u32 = 2u32;
pub const MANAGED_APPTYPE_UNSUPPORTED: u32 = 3u32;
pub const MANAGED_APPTYPE_WINDOWSINSTALLER: u32 = 1u32;
pub const NODEID_Machine: windows_core::GUID = windows_core::GUID::from_u128(0x8fc0b737_a0e1_11d1_a7d3_0000f87571e3);
pub const NODEID_MachineSWSettings: windows_core::GUID = windows_core::GUID::from_u128(0x8fc0b73a_a0e1_11d1_a7d3_0000f87571e3);
pub const NODEID_RSOPMachine: windows_core::GUID = windows_core::GUID::from_u128(0xbd4c1a2e_0b7a_4a62_a6b0_c0577539c97e);
pub const NODEID_RSOPMachineSWSettings: windows_core::GUID = windows_core::GUID::from_u128(0x6a76273e_eb8e_45db_94c5_25663a5f2c1a);
pub const NODEID_RSOPUser: windows_core::GUID = windows_core::GUID::from_u128(0xab87364f_0cec_4cd8_9bf8_898f34628fb8);
pub const NODEID_RSOPUserSWSettings: windows_core::GUID = windows_core::GUID::from_u128(0xe52c5ce3_fd27_4402_84de_d9a5f2858910);
pub const NODEID_User: windows_core::GUID = windows_core::GUID::from_u128(0x8fc0b738_a0e1_11d1_a7d3_0000f87571e3);
pub const NODEID_UserSWSettings: windows_core::GUID = windows_core::GUID::from_u128(0x8fc0b73c_a0e1_11d1_a7d3_0000f87571e3);
pub const PI_APPLYPOLICY: u32 = 2u32;
pub const PI_NOUI: u32 = 1u32;
pub const PROGID: INSTALLSPECTYPE = 3i32;
pub const PT_MANDATORY: u32 = 4u32;
pub const PT_ROAMING: u32 = 2u32;
pub const PT_ROAMING_PREEXISTING: u32 = 8u32;
pub const PT_TEMPORARY: u32 = 1u32;
pub const PUBLISHED: APPSTATE = 2i32;
pub const REGISTRY_EXTENSION_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x35378eac_683f_11d2_a89a_00c04fbbcfa2);
pub const RP_FORCE: u32 = 1u32;
pub const RP_SYNC: u32 = 2u32;
pub const RSOPApplied: SETTINGSTATUS = 1i32;
pub const RSOPFailed: SETTINGSTATUS = 3i32;
pub const RSOPIgnored: SETTINGSTATUS = 2i32;
pub const RSOPSubsettingFailed: SETTINGSTATUS = 4i32;
pub const RSOPUnspecified: SETTINGSTATUS = 0i32;
pub const RSOP_COMPUTER_ACCESS_DENIED: u32 = 2u32;
pub const RSOP_INFO_FLAG_DIAGNOSTIC_MODE: u32 = 1u32;
pub const RSOP_NO_COMPUTER: u32 = 65536u32;
pub const RSOP_NO_USER: u32 = 131072u32;
pub const RSOP_PLANNING_ASSUME_COMP_WQLFILTER_TRUE: u32 = 16u32;
pub const RSOP_PLANNING_ASSUME_LOOPBACK_MERGE: u32 = 2u32;
pub const RSOP_PLANNING_ASSUME_LOOPBACK_REPLACE: u32 = 4u32;
pub const RSOP_PLANNING_ASSUME_SLOW_LINK: u32 = 1u32;
pub const RSOP_PLANNING_ASSUME_USER_WQLFILTER_TRUE: u32 = 8u32;
pub const RSOP_TEMPNAMESPACE_EXISTS: u32 = 4u32;
pub const RSOP_USER_ACCESS_DENIED: u32 = 1u32;
pub const USER_POLICY_PRESENT_TRIGGER_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x54fb46c8_f089_464c_b1fd_59d1b62c3b50);
pub const backupMostRecent: GPMSearchProperty = 9i32;
pub const gpoComputerExtensions: GPMSearchProperty = 5i32;
pub const gpoDisplayName: GPMSearchProperty = 2i32;
pub const gpoDomain: GPMSearchProperty = 8i32;
pub const gpoEffectivePermissions: GPMSearchProperty = 1i32;
pub const gpoID: GPMSearchProperty = 4i32;
pub const gpoPermissions: GPMSearchProperty = 0i32;
pub const gpoUserExtensions: GPMSearchProperty = 6i32;
pub const gpoWMIFilter: GPMSearchProperty = 3i32;
pub const opContains: GPMSearchOperation = 1i32;
pub const opDestinationByRelativeName: GPMDestinationOption = 2i32;
pub const opDestinationNone: GPMDestinationOption = 1i32;
pub const opDestinationSameAsSource: GPMDestinationOption = 0i32;
pub const opDestinationSet: GPMDestinationOption = 3i32;
pub const opEquals: GPMSearchOperation = 0i32;
pub const opNotContains: GPMSearchOperation = 2i32;
pub const opNotEquals: GPMSearchOperation = 3i32;
pub const opReportComments: GPMReportingOptions = 1i32;
pub const opReportLegacy: GPMReportingOptions = 0i32;
pub const permGPOApply: GPMPermissionType = 65536i32;
pub const permGPOCustom: GPMPermissionType = 65795i32;
pub const permGPOEdit: GPMPermissionType = 65793i32;
pub const permGPOEditSecurityAndDelete: GPMPermissionType = 65794i32;
pub const permGPORead: GPMPermissionType = 65792i32;
pub const permSOMGPOCreate: GPMPermissionType = 1049600i32;
pub const permSOMLink: GPMPermissionType = 1835008i32;
pub const permSOMLogging: GPMPermissionType = 1573120i32;
pub const permSOMPlanning: GPMPermissionType = 1573376i32;
pub const permSOMStarterGPOCreate: GPMPermissionType = 1049856i32;
pub const permSOMWMICreate: GPMPermissionType = 1049344i32;
pub const permSOMWMIFullControl: GPMPermissionType = 1049345i32;
pub const permStarterGPOCustom: GPMPermissionType = 197891i32;
pub const permStarterGPOEdit: GPMPermissionType = 197889i32;
pub const permStarterGPOFullControl: GPMPermissionType = 197890i32;
pub const permStarterGPORead: GPMPermissionType = 197888i32;
pub const permWMIFilterCustom: GPMPermissionType = 131074i32;
pub const permWMIFilterEdit: GPMPermissionType = 131072i32;
pub const permWMIFilterFullControl: GPMPermissionType = 131073i32;
pub const repClientHealthRefreshXML: GPMReportType = 5i32;
pub const repClientHealthXML: GPMReportType = 4i32;
pub const repHTML: GPMReportType = 1i32;
pub const repInfraRefreshXML: GPMReportType = 3i32;
pub const repInfraXML: GPMReportType = 2i32;
pub const repXML: GPMReportType = 0i32;
pub const rsopLogging: GPMRSOPMode = 2i32;
pub const rsopPlanning: GPMRSOPMode = 1i32;
pub const rsopUnknown: GPMRSOPMode = 0i32;
pub const somDomain: GPMSOMType = 1i32;
pub const somLinks: GPMSearchProperty = 7i32;
pub const somOU: GPMSOMType = 2i32;
pub const somSite: GPMSOMType = 0i32;
pub const starterGPODisplayName: GPMSearchProperty = 12i32;
pub const starterGPODomain: GPMSearchProperty = 14i32;
pub const starterGPOEffectivePermissions: GPMSearchProperty = 11i32;
pub const starterGPOID: GPMSearchProperty = 13i32;
pub const starterGPOPermissions: GPMSearchProperty = 10i32;
pub const typeComputer: GPMEntryType = 1i32;
pub const typeCustom: GPMStarterGPOType = 1i32;
pub const typeGPO: GPMBackupType = 0i32;
pub const typeGlobalGroup: GPMEntryType = 3i32;
pub const typeLocalGroup: GPMEntryType = 2i32;
pub const typeStarterGPO: GPMBackupType = 1i32;
pub const typeSystem: GPMStarterGPOType = 0i32;
pub const typeUNCPath: GPMEntryType = 5i32;
pub const typeUniversalGroup: GPMEntryType = 4i32;
pub const typeUnknown: GPMEntryType = 6i32;
pub const typeUser: GPMEntryType = 0i32;
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct APPSTATE(pub i32);
impl windows_core::TypeKind for APPSTATE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GPMBackupType(pub i32);
impl windows_core::TypeKind for GPMBackupType {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GPMDestinationOption(pub i32);
impl windows_core::TypeKind for GPMDestinationOption {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GPMEntryType(pub i32);
impl windows_core::TypeKind for GPMEntryType {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GPMPermissionType(pub i32);
impl windows_core::TypeKind for GPMPermissionType {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GPMRSOPMode(pub i32);
impl windows_core::TypeKind for GPMRSOPMode {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GPMReportType(pub i32);
impl windows_core::TypeKind for GPMReportType {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GPMReportingOptions(pub i32);
impl windows_core::TypeKind for GPMReportingOptions {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GPMSOMType(pub i32);
impl windows_core::TypeKind for GPMSOMType {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GPMSearchOperation(pub i32);
impl windows_core::TypeKind for GPMSearchOperation {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GPMSearchProperty(pub i32);
impl windows_core::TypeKind for GPMSearchProperty {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GPMStarterGPOType(pub i32);
impl windows_core::TypeKind for GPMStarterGPOType {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GPO_LINK(pub i32);
impl windows_core::TypeKind for GPO_LINK {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GPO_OPEN_FLAGS(pub u32);
impl windows_core::TypeKind for GPO_OPEN_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GPO_OPTIONS(pub u32);
impl windows_core::TypeKind for GPO_OPTIONS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GPO_SECTION(pub u32);
impl windows_core::TypeKind for GPO_SECTION {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GROUP_POLICY_HINT_TYPE(pub i32);
impl windows_core::TypeKind for GROUP_POLICY_HINT_TYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GROUP_POLICY_OBJECT_TYPE(pub i32);
impl windows_core::TypeKind for GROUP_POLICY_OBJECT_TYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct INSTALLSPECTYPE(pub i32);
impl windows_core::TypeKind for INSTALLSPECTYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SETTINGSTATUS(pub i32);
impl windows_core::TypeKind for SETTINGSTATUS {
    type TypeKind = windows_core::CopyType;
}
pub const GPM: windows_core::GUID = windows_core::GUID::from_u128(0xf5694708_88fe_4b35_babf_e56162d5fbc8);
pub const GPMAsyncCancel: windows_core::GUID = windows_core::GUID::from_u128(0x372796a9_76ec_479d_ad6c_556318ed5f9d);
pub const GPMBackup: windows_core::GUID = windows_core::GUID::from_u128(0xed1a54b8_5efa_482a_93c0_8ad86f0d68c3);
pub const GPMBackupCollection: windows_core::GUID = windows_core::GUID::from_u128(0xeb8f035b_70db_4a9f_9676_37c25994e9dc);
pub const GPMBackupDir: windows_core::GUID = windows_core::GUID::from_u128(0xfce4a59d_0f21_4afa_b859_e6d0c62cd10c);
pub const GPMBackupDirEx: windows_core::GUID = windows_core::GUID::from_u128(0xe8c0988a_cf03_4c5b_8be2_2aa9ad32aada);
pub const GPMCSECollection: windows_core::GUID = windows_core::GUID::from_u128(0xcf92b828_2d44_4b61_b10a_b327afd42da8);
pub const GPMClientSideExtension: windows_core::GUID = windows_core::GUID::from_u128(0xc1a2e70e_659c_4b1a_940b_f88b0af9c8a4);
pub const GPMConstants: windows_core::GUID = windows_core::GUID::from_u128(0x3855e880_cd9e_4d0c_9eaf_1579283a1888);
pub const GPMDomain: windows_core::GUID = windows_core::GUID::from_u128(0x710901be_1050_4cb1_838a_c5cff259e183);
pub const GPMGPO: windows_core::GUID = windows_core::GUID::from_u128(0xd2ce2994_59b5_4064_b581_4d68486a16c4);
pub const GPMGPOCollection: windows_core::GUID = windows_core::GUID::from_u128(0x7a057325_832d_4de3_a41f_c780436a4e09);
pub const GPMGPOLink: windows_core::GUID = windows_core::GUID::from_u128(0xc1df9880_5303_42c6_8a3c_0488e1bf7364);
pub const GPMGPOLinksCollection: windows_core::GUID = windows_core::GUID::from_u128(0xf6ed581a_49a5_47e2_b771_fd8dc02b6259);
pub const GPMMapEntry: windows_core::GUID = windows_core::GUID::from_u128(0x8c975253_5431_4471_b35d_0626c928258a);
pub const GPMMapEntryCollection: windows_core::GUID = windows_core::GUID::from_u128(0x0cf75d5b_a3a1_4c55_b4fe_9e149c41f66d);
pub const GPMMigrationTable: windows_core::GUID = windows_core::GUID::from_u128(0x55af4043_2a06_4f72_abef_631b44079c76);
pub const GPMPermission: windows_core::GUID = windows_core::GUID::from_u128(0x5871a40a_e9c0_46ec_913e_944ef9225a94);
pub const GPMRSOP: windows_core::GUID = windows_core::GUID::from_u128(0x489b0caf_9ec2_4eb7_91f5_b6f71d43da8c);
pub const GPMResult: windows_core::GUID = windows_core::GUID::from_u128(0x92101ac0_9287_4206_a3b2_4bdb73d225f6);
pub const GPMSOM: windows_core::GUID = windows_core::GUID::from_u128(0x32d93fac_450e_44cf_829c_8b22ff6bdae1);
pub const GPMSOMCollection: windows_core::GUID = windows_core::GUID::from_u128(0x24c1f147_3720_4f5b_a9c3_06b4e4f931d2);
pub const GPMSearchCriteria: windows_core::GUID = windows_core::GUID::from_u128(0x17aaca26_5ce0_44fa_8cc0_5259e6483566);
pub const GPMSecurityInfo: windows_core::GUID = windows_core::GUID::from_u128(0x547a5e8f_9162_4516_a4df_9ddb9686d846);
pub const GPMSitesContainer: windows_core::GUID = windows_core::GUID::from_u128(0x229f5c42_852c_4b30_945f_c522be9bd386);
pub const GPMStarterGPOBackup: windows_core::GUID = windows_core::GUID::from_u128(0x389e400a_d8ef_455b_a861_5f9ca34a6a02);
pub const GPMStarterGPOBackupCollection: windows_core::GUID = windows_core::GUID::from_u128(0xe75ea59d_1aeb_4cb5_a78a_281daa582406);
pub const GPMStarterGPOCollection: windows_core::GUID = windows_core::GUID::from_u128(0x82f8aa8b_49ba_43b2_956e_3397f9b94c3a);
pub const GPMStatusMessage: windows_core::GUID = windows_core::GUID::from_u128(0x4b77cc94_d255_409b_bc62_370881715a19);
pub const GPMStatusMsgCollection: windows_core::GUID = windows_core::GUID::from_u128(0x2824e4be_4bcc_4cac_9e60_0e3ed7f12496);
pub const GPMTemplate: windows_core::GUID = windows_core::GUID::from_u128(0xecf1d454_71da_4e2f_a8c0_8185465911d9);
pub const GPMTrustee: windows_core::GUID = windows_core::GUID::from_u128(0xc54a700d_19b6_4211_bcb0_e8e2475e471e);
pub const GPMWMIFilter: windows_core::GUID = windows_core::GUID::from_u128(0x626745d8_0dea_4062_bf60_cfc5b1ca1286);
pub const GPMWMIFilterCollection: windows_core::GUID = windows_core::GUID::from_u128(0x74dc6d28_e820_47d6_a0b8_f08d93d7fa33);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GPOBROWSEINFO {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub lpTitle: windows_core::PWSTR,
    pub lpInitialOU: windows_core::PWSTR,
    pub lpDSPath: windows_core::PWSTR,
    pub dwDSPathSize: u32,
    pub lpName: windows_core::PWSTR,
    pub dwNameSize: u32,
    pub gpoType: GROUP_POLICY_OBJECT_TYPE,
    pub gpoHint: GROUP_POLICY_HINT_TYPE,
}
impl Default for GPOBROWSEINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for GPOBROWSEINFO {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GROUP_POLICY_OBJECTA {
    pub dwOptions: u32,
    pub dwVersion: u32,
    pub lpDSPath: windows_core::PSTR,
    pub lpFileSysPath: windows_core::PSTR,
    pub lpDisplayName: windows_core::PSTR,
    pub szGPOName: [i8; 50],
    pub GPOLink: GPO_LINK,
    pub lParam: super::super::Foundation::LPARAM,
    pub pNext: *mut GROUP_POLICY_OBJECTA,
    pub pPrev: *mut GROUP_POLICY_OBJECTA,
    pub lpExtensions: windows_core::PSTR,
    pub lParam2: super::super::Foundation::LPARAM,
    pub lpLink: windows_core::PSTR,
}
impl Default for GROUP_POLICY_OBJECTA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for GROUP_POLICY_OBJECTA {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GROUP_POLICY_OBJECTW {
    pub dwOptions: u32,
    pub dwVersion: u32,
    pub lpDSPath: windows_core::PWSTR,
    pub lpFileSysPath: windows_core::PWSTR,
    pub lpDisplayName: windows_core::PWSTR,
    pub szGPOName: [u16; 50],
    pub GPOLink: GPO_LINK,
    pub lParam: super::super::Foundation::LPARAM,
    pub pNext: *mut GROUP_POLICY_OBJECTW,
    pub pPrev: *mut GROUP_POLICY_OBJECTW,
    pub lpExtensions: windows_core::PWSTR,
    pub lParam2: super::super::Foundation::LPARAM,
    pub lpLink: windows_core::PWSTR,
}
impl Default for GROUP_POLICY_OBJECTW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for GROUP_POLICY_OBJECTW {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct INSTALLDATA {
    pub Type: INSTALLSPECTYPE,
    pub Spec: INSTALLSPEC,
}
impl Default for INSTALLDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for INSTALLDATA {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub union INSTALLSPEC {
    pub AppName: INSTALLSPEC_0,
    pub FileExt: windows_core::PWSTR,
    pub ProgId: windows_core::PWSTR,
    pub COMClass: INSTALLSPEC_1,
}
impl Default for INSTALLSPEC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for INSTALLSPEC {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct INSTALLSPEC_0 {
    pub Name: windows_core::PWSTR,
    pub GPOId: windows_core::GUID,
}
impl Default for INSTALLSPEC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for INSTALLSPEC_0 {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct INSTALLSPEC_1 {
    pub Clsid: windows_core::GUID,
    pub ClsCtx: u32,
}
impl Default for INSTALLSPEC_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for INSTALLSPEC_1 {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LOCALMANAGEDAPPLICATION {
    pub pszDeploymentName: windows_core::PWSTR,
    pub pszPolicyName: windows_core::PWSTR,
    pub pszProductId: windows_core::PWSTR,
    pub dwState: u32,
}
impl Default for LOCALMANAGEDAPPLICATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for LOCALMANAGEDAPPLICATION {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MANAGEDAPPLICATION {
    pub pszPackageName: windows_core::PWSTR,
    pub pszPublisher: windows_core::PWSTR,
    pub dwVersionHi: u32,
    pub dwVersionLo: u32,
    pub dwRevision: u32,
    pub GpoId: windows_core::GUID,
    pub pszPolicyName: windows_core::PWSTR,
    pub ProductId: windows_core::GUID,
    pub Language: u16,
    pub pszOwner: windows_core::PWSTR,
    pub pszCompany: windows_core::PWSTR,
    pub pszComments: windows_core::PWSTR,
    pub pszContact: windows_core::PWSTR,
    pub pszSupportUrl: windows_core::PWSTR,
    pub dwPathType: u32,
    pub bInstalled: super::super::Foundation::BOOL,
}
impl Default for MANAGEDAPPLICATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for MANAGEDAPPLICATION {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct POLICYSETTINGSTATUSINFO {
    pub szKey: windows_core::PWSTR,
    pub szEventSource: windows_core::PWSTR,
    pub szEventLogName: windows_core::PWSTR,
    pub dwEventID: u32,
    pub dwErrorCode: u32,
    pub status: SETTINGSTATUS,
    pub timeLogged: super::super::Foundation::SYSTEMTIME,
}
impl Default for POLICYSETTINGSTATUSINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for POLICYSETTINGSTATUSINFO {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Wmi"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RSOP_TARGET {
    pub pwszAccountName: windows_core::PWSTR,
    pub pwszNewSOM: windows_core::PWSTR,
    pub psaSecurityGroups: *mut super::Com::SAFEARRAY,
    pub pRsopToken: *mut core::ffi::c_void,
    pub pGPOList: *mut GROUP_POLICY_OBJECTA,
    pub pWbemServices: Option<super::Wmi::IWbemServices>,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Wmi"))]
impl Default for RSOP_TARGET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Wmi"))]
impl windows_core::TypeKind for RSOP_TARGET {
    type TypeKind = windows_core::CloneType;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Wmi"))]
pub type PFNGENERATEGROUPPOLICY = Option<unsafe extern "system" fn(dwflags: u32, pbabort: *mut super::super::Foundation::BOOL, pwszsite: windows_core::PCWSTR, pcomputertarget: *const RSOP_TARGET, pusertarget: *const RSOP_TARGET) -> u32>;
#[cfg(feature = "Win32_System_Registry")]
pub type PFNPROCESSGROUPPOLICY = Option<unsafe extern "system" fn(dwflags: u32, htoken: super::super::Foundation::HANDLE, hkeyroot: super::Registry::HKEY, pdeletedgpolist: *const GROUP_POLICY_OBJECTA, pchangedgpolist: *const GROUP_POLICY_OBJECTA, phandle: usize, pbabort: *mut super::super::Foundation::BOOL, pstatuscallback: PFNSTATUSMESSAGECALLBACK) -> u32>;
#[cfg(all(feature = "Win32_System_Registry", feature = "Win32_System_Wmi"))]
pub type PFNPROCESSGROUPPOLICYEX = Option<unsafe extern "system" fn(dwflags: u32, htoken: super::super::Foundation::HANDLE, hkeyroot: super::Registry::HKEY, pdeletedgpolist: *const GROUP_POLICY_OBJECTA, pchangedgpolist: *const GROUP_POLICY_OBJECTA, phandle: usize, pbabort: *mut super::super::Foundation::BOOL, pstatuscallback: PFNSTATUSMESSAGECALLBACK, pwbemservices: Option<super::Wmi::IWbemServices>, prsopstatus: *mut windows_core::HRESULT) -> u32>;
pub type PFNSTATUSMESSAGECALLBACK = Option<unsafe extern "system" fn(bverbose: super::super::Foundation::BOOL, lpmessage: windows_core::PCWSTR) -> u32>;
