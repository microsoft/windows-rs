#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct APPSTATE(pub i32);
pub const ABSENT: APPSTATE = APPSTATE(0i32);
pub const ASSIGNED: APPSTATE = APPSTATE(1i32);
pub const PUBLISHED: APPSTATE = APPSTATE(2i32);
impl ::core::convert::From<i32> for APPSTATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for APPSTATE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BrowseForGPO(lpbrowseinfo: *mut GPOBROWSEINFO) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BrowseForGPO(lpbrowseinfo: *mut GPOBROWSEINFO) -> ::windows::runtime::HRESULT;
        }
        BrowseForGPO(::core::mem::transmute(lpbrowseinfo)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const CLSID_GPESnapIn: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x8fc0b734_a0e1_11d1_a7d3_0000f87571e3);
pub const CLSID_GroupPolicyObject: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xea502722_a23d_11d1_a7d3_0000f87571e3);
pub const CLSID_RSOPSnapIn: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x6dc3804b_7212_458d_adb0_9a07e2ae1fa2);
#[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CommandLineFromMsiDescriptor<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(descriptor: Param0, commandline: super::super::Foundation::PWSTR, commandlinelength: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CommandLineFromMsiDescriptor(descriptor: super::super::Foundation::PWSTR, commandline: super::super::Foundation::PWSTR, commandlinelength: *mut u32) -> u32;
        }
        ::core::mem::transmute(CommandLineFromMsiDescriptor(descriptor.into_param().abi(), ::core::mem::transmute(commandline), ::core::mem::transmute(commandlinelength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateGPOLink<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(lpgpo: Param0, lpcontainer: Param1, fhighpriority: Param2) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateGPOLink(lpgpo: super::super::Foundation::PWSTR, lpcontainer: super::super::Foundation::PWSTR, fhighpriority: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
        }
        CreateGPOLink(lpgpo.into_param().abi(), lpcontainer.into_param().abi(), fhighpriority.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
#[repr(transparent)]
pub struct CriticalPolicySectionHandle(pub isize);
impl ::core::default::Default for CriticalPolicySectionHandle {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for CriticalPolicySectionHandle {}
unsafe impl ::windows::runtime::Abi for CriticalPolicySectionHandle {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeleteAllGPOLinks<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(lpcontainer: Param0) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeleteAllGPOLinks(lpcontainer: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        DeleteAllGPOLinks(lpcontainer.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeleteGPOLink<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(lpgpo: Param0, lpcontainer: Param1) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeleteGPOLink(lpgpo: super::super::Foundation::PWSTR, lpcontainer: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        DeleteGPOLink(lpgpo.into_param().abi(), lpcontainer.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnterCriticalPolicySection<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(bmachine: Param0) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnterCriticalPolicySection(bmachine: super::super::Foundation::BOOL) -> super::super::Foundation::HANDLE;
        }
        ::core::mem::transmute(EnterCriticalPolicySection(bmachine.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ExportRSoPData<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(lpnamespace: Param0, lpfilename: Param1) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ExportRSoPData(lpnamespace: super::super::Foundation::PWSTR, lpfilename: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        ExportRSoPData(lpnamespace.into_param().abi(), lpfilename.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
pub const FLAG_ASSUME_COMP_WQLFILTER_TRUE: u32 = 33554432u32;
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
pub const FLAG_ASSUME_SLOW_LINK: u32 = 536870912u32;
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
pub const FLAG_ASSUME_USER_WQLFILTER_TRUE: u32 = 67108864u32;
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
pub const FLAG_FORCE_CREATENAMESPACE: u32 = 4u32;
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
pub const FLAG_LOOPBACK_MERGE: u32 = 268435456u32;
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
pub const FLAG_LOOPBACK_REPLACE: u32 = 134217728u32;
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
pub const FLAG_NO_COMPUTER: u32 = 2u32;
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
pub const FLAG_NO_CSE_INVOKE: u32 = 1073741824u32;
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
pub const FLAG_NO_GPO_FILTER: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
pub const FLAG_NO_USER: u32 = 1u32;
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
pub const FLAG_PLANNING_MODE: u32 = 16777216u32;
#[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FreeGPOListA(pgpolist: *const GROUP_POLICY_OBJECTA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FreeGPOListA(pgpolist: *const GROUP_POLICY_OBJECTA) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FreeGPOListA(::core::mem::transmute(pgpolist)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FreeGPOListW(pgpolist: *const GROUP_POLICY_OBJECTW) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FreeGPOListW(pgpolist: *const GROUP_POLICY_OBJECTW) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FreeGPOListW(::core::mem::transmute(pgpolist)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
pub const GPC_BLOCK_POLICY: u32 = 1u32;
pub const GPM: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xf5694708_88fe_4b35_babf_e56162d5fbc8);
pub const GPMAsyncCancel: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x372796a9_76ec_479d_ad6c_556318ed5f9d);
pub const GPMBackup: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xed1a54b8_5efa_482a_93c0_8ad86f0d68c3);
pub const GPMBackupCollection: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xeb8f035b_70db_4a9f_9676_37c25994e9dc);
pub const GPMBackupDir: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xfce4a59d_0f21_4afa_b859_e6d0c62cd10c);
pub const GPMBackupDirEx: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xe8c0988a_cf03_4c5b_8be2_2aa9ad32aada);
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct GPMBackupType(pub i32);
pub const typeGPO: GPMBackupType = GPMBackupType(0i32);
pub const typeStarterGPO: GPMBackupType = GPMBackupType(1i32);
impl ::core::convert::From<i32> for GPMBackupType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GPMBackupType {
    type Abi = Self;
}
pub const GPMCSECollection: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xcf92b828_2d44_4b61_b10a_b327afd42da8);
pub const GPMClientSideExtension: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xc1a2e70e_659c_4b1a_940b_f88b0af9c8a4);
pub const GPMConstants: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x3855e880_cd9e_4d0c_9eaf_1579283a1888);
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct GPMDestinationOption(pub i32);
pub const opDestinationSameAsSource: GPMDestinationOption = GPMDestinationOption(0i32);
pub const opDestinationNone: GPMDestinationOption = GPMDestinationOption(1i32);
pub const opDestinationByRelativeName: GPMDestinationOption = GPMDestinationOption(2i32);
pub const opDestinationSet: GPMDestinationOption = GPMDestinationOption(3i32);
impl ::core::convert::From<i32> for GPMDestinationOption {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GPMDestinationOption {
    type Abi = Self;
}
pub const GPMDomain: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x710901be_1050_4cb1_838a_c5cff259e183);
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct GPMEntryType(pub i32);
pub const typeUser: GPMEntryType = GPMEntryType(0i32);
pub const typeComputer: GPMEntryType = GPMEntryType(1i32);
pub const typeLocalGroup: GPMEntryType = GPMEntryType(2i32);
pub const typeGlobalGroup: GPMEntryType = GPMEntryType(3i32);
pub const typeUniversalGroup: GPMEntryType = GPMEntryType(4i32);
pub const typeUNCPath: GPMEntryType = GPMEntryType(5i32);
pub const typeUnknown: GPMEntryType = GPMEntryType(6i32);
impl ::core::convert::From<i32> for GPMEntryType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GPMEntryType {
    type Abi = Self;
}
pub const GPMGPO: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xd2ce2994_59b5_4064_b581_4d68486a16c4);
pub const GPMGPOCollection: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x7a057325_832d_4de3_a41f_c780436a4e09);
pub const GPMGPOLink: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xc1df9880_5303_42c6_8a3c_0488e1bf7364);
pub const GPMGPOLinksCollection: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xf6ed581a_49a5_47e2_b771_fd8dc02b6259);
pub const GPMMapEntry: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x8c975253_5431_4471_b35d_0626c928258a);
pub const GPMMapEntryCollection: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x0cf75d5b_a3a1_4c55_b4fe_9e149c41f66d);
pub const GPMMigrationTable: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x55af4043_2a06_4f72_abef_631b44079c76);
pub const GPMPermission: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x5871a40a_e9c0_46ec_913e_944ef9225a94);
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct GPMPermissionType(pub i32);
pub const permGPOApply: GPMPermissionType = GPMPermissionType(65536i32);
pub const permGPORead: GPMPermissionType = GPMPermissionType(65792i32);
pub const permGPOEdit: GPMPermissionType = GPMPermissionType(65793i32);
pub const permGPOEditSecurityAndDelete: GPMPermissionType = GPMPermissionType(65794i32);
pub const permGPOCustom: GPMPermissionType = GPMPermissionType(65795i32);
pub const permWMIFilterEdit: GPMPermissionType = GPMPermissionType(131072i32);
pub const permWMIFilterFullControl: GPMPermissionType = GPMPermissionType(131073i32);
pub const permWMIFilterCustom: GPMPermissionType = GPMPermissionType(131074i32);
pub const permSOMLink: GPMPermissionType = GPMPermissionType(1835008i32);
pub const permSOMLogging: GPMPermissionType = GPMPermissionType(1573120i32);
pub const permSOMPlanning: GPMPermissionType = GPMPermissionType(1573376i32);
pub const permSOMWMICreate: GPMPermissionType = GPMPermissionType(1049344i32);
pub const permSOMWMIFullControl: GPMPermissionType = GPMPermissionType(1049345i32);
pub const permSOMGPOCreate: GPMPermissionType = GPMPermissionType(1049600i32);
pub const permStarterGPORead: GPMPermissionType = GPMPermissionType(197888i32);
pub const permStarterGPOEdit: GPMPermissionType = GPMPermissionType(197889i32);
pub const permStarterGPOFullControl: GPMPermissionType = GPMPermissionType(197890i32);
pub const permStarterGPOCustom: GPMPermissionType = GPMPermissionType(197891i32);
pub const permSOMStarterGPOCreate: GPMPermissionType = GPMPermissionType(1049856i32);
impl ::core::convert::From<i32> for GPMPermissionType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GPMPermissionType {
    type Abi = Self;
}
pub const GPMRSOP: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x489b0caf_9ec2_4eb7_91f5_b6f71d43da8c);
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct GPMRSOPMode(pub i32);
pub const rsopUnknown: GPMRSOPMode = GPMRSOPMode(0i32);
pub const rsopPlanning: GPMRSOPMode = GPMRSOPMode(1i32);
pub const rsopLogging: GPMRSOPMode = GPMRSOPMode(2i32);
impl ::core::convert::From<i32> for GPMRSOPMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GPMRSOPMode {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct GPMReportType(pub i32);
pub const repXML: GPMReportType = GPMReportType(0i32);
pub const repHTML: GPMReportType = GPMReportType(1i32);
pub const repInfraXML: GPMReportType = GPMReportType(2i32);
pub const repInfraRefreshXML: GPMReportType = GPMReportType(3i32);
pub const repClientHealthXML: GPMReportType = GPMReportType(4i32);
pub const repClientHealthRefreshXML: GPMReportType = GPMReportType(5i32);
impl ::core::convert::From<i32> for GPMReportType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GPMReportType {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct GPMReportingOptions(pub i32);
pub const opReportLegacy: GPMReportingOptions = GPMReportingOptions(0i32);
pub const opReportComments: GPMReportingOptions = GPMReportingOptions(1i32);
impl ::core::convert::From<i32> for GPMReportingOptions {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GPMReportingOptions {
    type Abi = Self;
}
pub const GPMResult: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x92101ac0_9287_4206_a3b2_4bdb73d225f6);
pub const GPMSOM: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x32d93fac_450e_44cf_829c_8b22ff6bdae1);
pub const GPMSOMCollection: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x24c1f147_3720_4f5b_a9c3_06b4e4f931d2);
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct GPMSOMType(pub i32);
pub const somSite: GPMSOMType = GPMSOMType(0i32);
pub const somDomain: GPMSOMType = GPMSOMType(1i32);
pub const somOU: GPMSOMType = GPMSOMType(2i32);
impl ::core::convert::From<i32> for GPMSOMType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GPMSOMType {
    type Abi = Self;
}
pub const GPMSearchCriteria: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x17aaca26_5ce0_44fa_8cc0_5259e6483566);
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct GPMSearchOperation(pub i32);
pub const opEquals: GPMSearchOperation = GPMSearchOperation(0i32);
pub const opContains: GPMSearchOperation = GPMSearchOperation(1i32);
pub const opNotContains: GPMSearchOperation = GPMSearchOperation(2i32);
pub const opNotEquals: GPMSearchOperation = GPMSearchOperation(3i32);
impl ::core::convert::From<i32> for GPMSearchOperation {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GPMSearchOperation {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct GPMSearchProperty(pub i32);
pub const gpoPermissions: GPMSearchProperty = GPMSearchProperty(0i32);
pub const gpoEffectivePermissions: GPMSearchProperty = GPMSearchProperty(1i32);
pub const gpoDisplayName: GPMSearchProperty = GPMSearchProperty(2i32);
pub const gpoWMIFilter: GPMSearchProperty = GPMSearchProperty(3i32);
pub const gpoID: GPMSearchProperty = GPMSearchProperty(4i32);
pub const gpoComputerExtensions: GPMSearchProperty = GPMSearchProperty(5i32);
pub const gpoUserExtensions: GPMSearchProperty = GPMSearchProperty(6i32);
pub const somLinks: GPMSearchProperty = GPMSearchProperty(7i32);
pub const gpoDomain: GPMSearchProperty = GPMSearchProperty(8i32);
pub const backupMostRecent: GPMSearchProperty = GPMSearchProperty(9i32);
pub const starterGPOPermissions: GPMSearchProperty = GPMSearchProperty(10i32);
pub const starterGPOEffectivePermissions: GPMSearchProperty = GPMSearchProperty(11i32);
pub const starterGPODisplayName: GPMSearchProperty = GPMSearchProperty(12i32);
pub const starterGPOID: GPMSearchProperty = GPMSearchProperty(13i32);
pub const starterGPODomain: GPMSearchProperty = GPMSearchProperty(14i32);
impl ::core::convert::From<i32> for GPMSearchProperty {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GPMSearchProperty {
    type Abi = Self;
}
pub const GPMSecurityInfo: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x547a5e8f_9162_4516_a4df_9ddb9686d846);
pub const GPMSitesContainer: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x229f5c42_852c_4b30_945f_c522be9bd386);
pub const GPMStarterGPOBackup: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x389e400a_d8ef_455b_a861_5f9ca34a6a02);
pub const GPMStarterGPOBackupCollection: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xe75ea59d_1aeb_4cb5_a78a_281daa582406);
pub const GPMStarterGPOCollection: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x82f8aa8b_49ba_43b2_956e_3397f9b94c3a);
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct GPMStarterGPOType(pub i32);
pub const typeSystem: GPMStarterGPOType = GPMStarterGPOType(0i32);
pub const typeCustom: GPMStarterGPOType = GPMStarterGPOType(1i32);
impl ::core::convert::From<i32> for GPMStarterGPOType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GPMStarterGPOType {
    type Abi = Self;
}
pub const GPMStatusMessage: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x4b77cc94_d255_409b_bc62_370881715a19);
pub const GPMStatusMsgCollection: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x2824e4be_4bcc_4cac_9e60_0e3ed7f12496);
pub const GPMTemplate: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xecf1d454_71da_4e2f_a8c0_8185465911d9);
pub const GPMTrustee: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xc54a700d_19b6_4211_bcb0_e8e2475e471e);
pub const GPMWMIFilter: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x626745d8_0dea_4062_bf60_cfc5b1ca1286);
pub const GPMWMIFilterCollection: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x74dc6d28_e820_47d6_a0b8_f08d93d7fa33);
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
pub const GPM_DONOTUSE_W2KDC: u32 = 2u32;
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
pub const GPM_DONOT_VALIDATEDC: u32 = 1u32;
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
pub const GPM_MIGRATIONTABLE_ONLY: u32 = 1u32;
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
pub const GPM_PROCESS_SECURITY: u32 = 2u32;
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
pub const GPM_USE_ANYDC: u32 = 1u32;
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
pub const GPM_USE_PDC: u32 = 0u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
pub struct GPOBROWSEINFO {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub lpTitle: super::super::Foundation::PWSTR,
    pub lpInitialOU: super::super::Foundation::PWSTR,
    pub lpDSPath: super::super::Foundation::PWSTR,
    pub dwDSPathSize: u32,
    pub lpName: super::super::Foundation::PWSTR,
    pub dwNameSize: u32,
    pub gpoType: GROUP_POLICY_OBJECT_TYPE,
    pub gpoHint: GROUP_POLICY_HINT_TYPE,
}
#[cfg(feature = "Win32_Foundation")]
impl GPOBROWSEINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GPOBROWSEINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for GPOBROWSEINFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("GPOBROWSEINFO")
            .field("dwSize", &self.dwSize)
            .field("dwFlags", &self.dwFlags)
            .field("hwndOwner", &self.hwndOwner)
            .field("lpTitle", &self.lpTitle)
            .field("lpInitialOU", &self.lpInitialOU)
            .field("lpDSPath", &self.lpDSPath)
            .field("dwDSPathSize", &self.dwDSPathSize)
            .field("lpName", &self.lpName)
            .field("dwNameSize", &self.dwNameSize)
            .field("gpoType", &self.gpoType)
            .field("gpoHint", &self.gpoHint)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GPOBROWSEINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwFlags == other.dwFlags && self.hwndOwner == other.hwndOwner && self.lpTitle == other.lpTitle && self.lpInitialOU == other.lpInitialOU && self.lpDSPath == other.lpDSPath && self.dwDSPathSize == other.dwDSPathSize && self.lpName == other.lpName && self.dwNameSize == other.dwNameSize && self.gpoType == other.gpoType && self.gpoHint == other.gpoHint
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GPOBROWSEINFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for GPOBROWSEINFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
pub const GPO_BROWSE_DISABLENEW: u32 = 1u32;
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
pub const GPO_BROWSE_INITTOALL: u32 = 16u32;
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
pub const GPO_BROWSE_NOCOMPUTERS: u32 = 2u32;
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
pub const GPO_BROWSE_NODSGPOS: u32 = 4u32;
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
pub const GPO_BROWSE_NOUSERGPOS: u32 = 32u32;
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
pub const GPO_BROWSE_OPENBUTTON: u32 = 8u32;
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
pub const GPO_BROWSE_SENDAPPLYONEDIT: u32 = 64u32;
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
pub const GPO_FLAG_DISABLE: u32 = 1u32;
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
pub const GPO_FLAG_FORCE: u32 = 2u32;
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
pub const GPO_INFO_FLAG_ASYNC_FOREGROUND: u32 = 4096u32;
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
pub const GPO_INFO_FLAG_BACKGROUND: u32 = 16u32;
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
pub const GPO_INFO_FLAG_FORCED_REFRESH: u32 = 1024u32;
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
pub const GPO_INFO_FLAG_LINKTRANSITION: u32 = 256u32;
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
pub const GPO_INFO_FLAG_LOGRSOP_TRANSITION: u32 = 512u32;
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
pub const GPO_INFO_FLAG_MACHINE: u32 = 1u32;
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
pub const GPO_INFO_FLAG_NOCHANGES: u32 = 128u32;
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
pub const GPO_INFO_FLAG_SAFEMODE_BOOT: u32 = 2048u32;
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
pub const GPO_INFO_FLAG_SLOWLINK: u32 = 32u32;
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
pub const GPO_INFO_FLAG_VERBOSE: u32 = 64u32;
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct GPO_LINK(pub i32);
pub const GPLinkUnknown: GPO_LINK = GPO_LINK(0i32);
pub const GPLinkMachine: GPO_LINK = GPO_LINK(1i32);
pub const GPLinkSite: GPO_LINK = GPO_LINK(2i32);
pub const GPLinkDomain: GPO_LINK = GPO_LINK(3i32);
pub const GPLinkOrganizationalUnit: GPO_LINK = GPO_LINK(4i32);
impl ::core::convert::From<i32> for GPO_LINK {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GPO_LINK {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
pub const GPO_LIST_FLAG_MACHINE: u32 = 1u32;
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
pub const GPO_LIST_FLAG_NO_SECURITYFILTERS: u32 = 8u32;
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
pub const GPO_LIST_FLAG_NO_WMIFILTERS: u32 = 4u32;
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
pub const GPO_LIST_FLAG_SITEONLY: u32 = 2u32;
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
pub const GPO_OPEN_LOAD_REGISTRY: u32 = 1u32;
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
pub const GPO_OPEN_READ_ONLY: u32 = 2u32;
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
pub const GPO_OPTION_DISABLE_MACHINE: u32 = 2u32;
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
pub const GPO_OPTION_DISABLE_USER: u32 = 1u32;
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
pub const GPO_SECTION_MACHINE: u32 = 2u32;
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
pub const GPO_SECTION_ROOT: u32 = 0u32;
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
pub const GPO_SECTION_USER: u32 = 1u32;
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct GROUP_POLICY_HINT_TYPE(pub i32);
pub const GPHintUnknown: GROUP_POLICY_HINT_TYPE = GROUP_POLICY_HINT_TYPE(0i32);
pub const GPHintMachine: GROUP_POLICY_HINT_TYPE = GROUP_POLICY_HINT_TYPE(1i32);
pub const GPHintSite: GROUP_POLICY_HINT_TYPE = GROUP_POLICY_HINT_TYPE(2i32);
pub const GPHintDomain: GROUP_POLICY_HINT_TYPE = GROUP_POLICY_HINT_TYPE(3i32);
pub const GPHintOrganizationalUnit: GROUP_POLICY_HINT_TYPE = GROUP_POLICY_HINT_TYPE(4i32);
impl ::core::convert::From<i32> for GROUP_POLICY_HINT_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GROUP_POLICY_HINT_TYPE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
pub struct GROUP_POLICY_OBJECTA {
    pub dwOptions: u32,
    pub dwVersion: u32,
    pub lpDSPath: super::super::Foundation::PSTR,
    pub lpFileSysPath: super::super::Foundation::PSTR,
    pub lpDisplayName: super::super::Foundation::PSTR,
    pub szGPOName: [super::super::Foundation::CHAR; 50],
    pub GPOLink: GPO_LINK,
    pub lParam: super::super::Foundation::LPARAM,
    pub pNext: *mut GROUP_POLICY_OBJECTA,
    pub pPrev: *mut GROUP_POLICY_OBJECTA,
    pub lpExtensions: super::super::Foundation::PSTR,
    pub lParam2: super::super::Foundation::LPARAM,
    pub lpLink: super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl GROUP_POLICY_OBJECTA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GROUP_POLICY_OBJECTA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for GROUP_POLICY_OBJECTA {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("GROUP_POLICY_OBJECTA")
            .field("dwOptions", &self.dwOptions)
            .field("dwVersion", &self.dwVersion)
            .field("lpDSPath", &self.lpDSPath)
            .field("lpFileSysPath", &self.lpFileSysPath)
            .field("lpDisplayName", &self.lpDisplayName)
            .field("szGPOName", &self.szGPOName)
            .field("GPOLink", &self.GPOLink)
            .field("lParam", &self.lParam)
            .field("pNext", &self.pNext)
            .field("pPrev", &self.pPrev)
            .field("lpExtensions", &self.lpExtensions)
            .field("lParam2", &self.lParam2)
            .field("lpLink", &self.lpLink)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GROUP_POLICY_OBJECTA {
    fn eq(&self, other: &Self) -> bool {
        self.dwOptions == other.dwOptions && self.dwVersion == other.dwVersion && self.lpDSPath == other.lpDSPath && self.lpFileSysPath == other.lpFileSysPath && self.lpDisplayName == other.lpDisplayName && self.szGPOName == other.szGPOName && self.GPOLink == other.GPOLink && self.lParam == other.lParam && self.pNext == other.pNext && self.pPrev == other.pPrev && self.lpExtensions == other.lpExtensions && self.lParam2 == other.lParam2 && self.lpLink == other.lpLink
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GROUP_POLICY_OBJECTA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for GROUP_POLICY_OBJECTA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
pub struct GROUP_POLICY_OBJECTW {
    pub dwOptions: u32,
    pub dwVersion: u32,
    pub lpDSPath: super::super::Foundation::PWSTR,
    pub lpFileSysPath: super::super::Foundation::PWSTR,
    pub lpDisplayName: super::super::Foundation::PWSTR,
    pub szGPOName: [u16; 50],
    pub GPOLink: GPO_LINK,
    pub lParam: super::super::Foundation::LPARAM,
    pub pNext: *mut GROUP_POLICY_OBJECTW,
    pub pPrev: *mut GROUP_POLICY_OBJECTW,
    pub lpExtensions: super::super::Foundation::PWSTR,
    pub lParam2: super::super::Foundation::LPARAM,
    pub lpLink: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl GROUP_POLICY_OBJECTW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GROUP_POLICY_OBJECTW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for GROUP_POLICY_OBJECTW {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("GROUP_POLICY_OBJECTW")
            .field("dwOptions", &self.dwOptions)
            .field("dwVersion", &self.dwVersion)
            .field("lpDSPath", &self.lpDSPath)
            .field("lpFileSysPath", &self.lpFileSysPath)
            .field("lpDisplayName", &self.lpDisplayName)
            .field("szGPOName", &self.szGPOName)
            .field("GPOLink", &self.GPOLink)
            .field("lParam", &self.lParam)
            .field("pNext", &self.pNext)
            .field("pPrev", &self.pPrev)
            .field("lpExtensions", &self.lpExtensions)
            .field("lParam2", &self.lParam2)
            .field("lpLink", &self.lpLink)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GROUP_POLICY_OBJECTW {
    fn eq(&self, other: &Self) -> bool {
        self.dwOptions == other.dwOptions && self.dwVersion == other.dwVersion && self.lpDSPath == other.lpDSPath && self.lpFileSysPath == other.lpFileSysPath && self.lpDisplayName == other.lpDisplayName && self.szGPOName == other.szGPOName && self.GPOLink == other.GPOLink && self.lParam == other.lParam && self.pNext == other.pNext && self.pPrev == other.pPrev && self.lpExtensions == other.lpExtensions && self.lParam2 == other.lParam2 && self.lpLink == other.lpLink
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GROUP_POLICY_OBJECTW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for GROUP_POLICY_OBJECTW {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct GROUP_POLICY_OBJECT_TYPE(pub i32);
pub const GPOTypeLocal: GROUP_POLICY_OBJECT_TYPE = GROUP_POLICY_OBJECT_TYPE(0i32);
pub const GPOTypeRemote: GROUP_POLICY_OBJECT_TYPE = GROUP_POLICY_OBJECT_TYPE(1i32);
pub const GPOTypeDS: GROUP_POLICY_OBJECT_TYPE = GROUP_POLICY_OBJECT_TYPE(2i32);
pub const GPOTypeLocalUser: GROUP_POLICY_OBJECT_TYPE = GROUP_POLICY_OBJECT_TYPE(3i32);
pub const GPOTypeLocalGroup: GROUP_POLICY_OBJECT_TYPE = GROUP_POLICY_OBJECT_TYPE(4i32);
impl ::core::convert::From<i32> for GROUP_POLICY_OBJECT_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GROUP_POLICY_OBJECT_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GenerateGPNotification<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(bmachine: Param0, lpwszmgmtproduct: Param1, dwmgmtproductoptions: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GenerateGPNotification(bmachine: super::super::Foundation::BOOL, lpwszmgmtproduct: super::super::Foundation::PWSTR, dwmgmtproductoptions: u32) -> u32;
        }
        ::core::mem::transmute(GenerateGPNotification(bmachine.into_param().abi(), lpwszmgmtproduct.into_param().abi(), ::core::mem::transmute(dwmgmtproductoptions)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetAppliedGPOListA<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSID>>(dwflags: u32, pmachinename: Param1, psiduser: Param2, pguidextension: *const ::windows::runtime::GUID, ppgpolist: *mut *mut GROUP_POLICY_OBJECTA) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetAppliedGPOListA(dwflags: u32, pmachinename: super::super::Foundation::PSTR, psiduser: super::super::Foundation::PSID, pguidextension: *const ::windows::runtime::GUID, ppgpolist: *mut *mut GROUP_POLICY_OBJECTA) -> u32;
        }
        ::core::mem::transmute(GetAppliedGPOListA(::core::mem::transmute(dwflags), pmachinename.into_param().abi(), psiduser.into_param().abi(), ::core::mem::transmute(pguidextension), ::core::mem::transmute(ppgpolist)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetAppliedGPOListW<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSID>>(dwflags: u32, pmachinename: Param1, psiduser: Param2, pguidextension: *const ::windows::runtime::GUID, ppgpolist: *mut *mut GROUP_POLICY_OBJECTW) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetAppliedGPOListW(dwflags: u32, pmachinename: super::super::Foundation::PWSTR, psiduser: super::super::Foundation::PSID, pguidextension: *const ::windows::runtime::GUID, ppgpolist: *mut *mut GROUP_POLICY_OBJECTW) -> u32;
        }
        ::core::mem::transmute(GetAppliedGPOListW(::core::mem::transmute(dwflags), pmachinename.into_param().abi(), psiduser.into_param().abi(), ::core::mem::transmute(pguidextension), ::core::mem::transmute(ppgpolist)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetGPOListA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(htoken: Param0, lpname: Param1, lphostname: Param2, lpcomputername: Param3, dwflags: u32, pgpolist: *mut *mut GROUP_POLICY_OBJECTA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetGPOListA(htoken: super::super::Foundation::HANDLE, lpname: super::super::Foundation::PSTR, lphostname: super::super::Foundation::PSTR, lpcomputername: super::super::Foundation::PSTR, dwflags: u32, pgpolist: *mut *mut GROUP_POLICY_OBJECTA) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetGPOListA(htoken.into_param().abi(), lpname.into_param().abi(), lphostname.into_param().abi(), lpcomputername.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(pgpolist)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetGPOListW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(
    htoken: Param0,
    lpname: Param1,
    lphostname: Param2,
    lpcomputername: Param3,
    dwflags: u32,
    pgpolist: *mut *mut GROUP_POLICY_OBJECTW,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetGPOListW(htoken: super::super::Foundation::HANDLE, lpname: super::super::Foundation::PWSTR, lphostname: super::super::Foundation::PWSTR, lpcomputername: super::super::Foundation::PWSTR, dwflags: u32, pgpolist: *mut *mut GROUP_POLICY_OBJECTW) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetGPOListW(htoken.into_param().abi(), lpname.into_param().abi(), lphostname.into_param().abi(), lpcomputername.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(pgpolist)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetLocalManagedApplicationData<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(productcode: Param0, displayname: *mut super::super::Foundation::PWSTR, supporturl: *mut super::super::Foundation::PWSTR) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetLocalManagedApplicationData(productcode: super::super::Foundation::PWSTR, displayname: *mut super::super::Foundation::PWSTR, supporturl: *mut super::super::Foundation::PWSTR);
        }
        ::core::mem::transmute(GetLocalManagedApplicationData(productcode.into_param().abi(), ::core::mem::transmute(displayname), ::core::mem::transmute(supporturl)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetLocalManagedApplications<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(buserapps: Param0, pdwapps: *mut u32, prglocalapps: *mut *mut LOCALMANAGEDAPPLICATION) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetLocalManagedApplications(buserapps: super::super::Foundation::BOOL, pdwapps: *mut u32, prglocalapps: *mut *mut LOCALMANAGEDAPPLICATION) -> u32;
        }
        ::core::mem::transmute(GetLocalManagedApplications(buserapps.into_param().abi(), ::core::mem::transmute(pdwapps), ::core::mem::transmute(prglocalapps)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`, `Win32_UI_Shell`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell"))]
#[inline]
pub unsafe fn GetManagedApplicationCategories(dwreserved: u32, pappcategory: *mut super::super::UI::Shell::APPCATEGORYINFOLIST) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetManagedApplicationCategories(dwreserved: u32, pappcategory: *mut super::super::UI::Shell::APPCATEGORYINFOLIST) -> u32;
        }
        ::core::mem::transmute(GetManagedApplicationCategories(::core::mem::transmute(dwreserved), ::core::mem::transmute(pappcategory)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetManagedApplications(pcategory: *const ::windows::runtime::GUID, dwqueryflags: u32, dwinfolevel: u32, pdwapps: *mut u32, prgmanagedapps: *mut *mut MANAGEDAPPLICATION) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetManagedApplications(pcategory: *const ::windows::runtime::GUID, dwqueryflags: u32, dwinfolevel: u32, pdwapps: *mut u32, prgmanagedapps: *mut *mut MANAGEDAPPLICATION) -> u32;
        }
        ::core::mem::transmute(GetManagedApplications(::core::mem::transmute(pcategory), ::core::mem::transmute(dwqueryflags), ::core::mem::transmute(dwinfolevel), ::core::mem::transmute(pdwapps), ::core::mem::transmute(prgmanagedapps)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IGPEInformation(pub ::windows::runtime::IUnknown);
impl IGPEInformation {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn GetName(&self, pszname: super::super::Foundation::PWSTR, cchmaxlength: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pszname), ::core::mem::transmute(cchmaxlength)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn GetDisplayName(&self, pszname: super::super::Foundation::PWSTR, cchmaxlength: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pszname), ::core::mem::transmute(cchmaxlength)).ok()
    }
    #[cfg(feature = "Win32_System_Registry")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_System_Registry`*"]
    pub unsafe fn GetRegistryKey(&self, dwsection: u32, hkey: *mut super::Registry::HKEY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwsection), ::core::mem::transmute(hkey)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn GetDSPath(&self, dwsection: u32, pszpath: super::super::Foundation::PWSTR, cchmaxpath: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwsection), ::core::mem::transmute(pszpath), ::core::mem::transmute(cchmaxpath)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn GetFileSysPath(&self, dwsection: u32, pszpath: super::super::Foundation::PWSTR, cchmaxpath: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwsection), ::core::mem::transmute(pszpath), ::core::mem::transmute(cchmaxpath)).ok()
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn GetOptions(&self, dwoptions: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoptions)).ok()
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn GetType(&self, gpotype: *mut GROUP_POLICY_OBJECT_TYPE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(gpotype)).ok()
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn GetHint(&self, gphint: *mut GROUP_POLICY_HINT_TYPE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(gphint)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn PolicyChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bmachine: Param0, badd: Param1, pguidextension: *mut ::windows::runtime::GUID, pguidsnapin: *mut ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), bmachine.into_param().abi(), badd.into_param().abi(), ::core::mem::transmute(pguidextension), ::core::mem::transmute(pguidsnapin)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IGPEInformation {
    type Vtable = IGPEInformation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x8fc0b735_a0e1_11d1_a7d3_0000f87571e3);
}
impl ::core::convert::From<IGPEInformation> for ::windows::runtime::IUnknown {
    fn from(value: IGPEInformation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IGPEInformation> for ::windows::runtime::IUnknown {
    fn from(value: &IGPEInformation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IGPEInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IGPEInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGPEInformation_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszname: super::super::Foundation::PWSTR, cchmaxlength: i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszname: super::super::Foundation::PWSTR, cchmaxlength: i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Registry")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwsection: u32, hkey: *mut super::Registry::HKEY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Registry"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwsection: u32, pszpath: super::super::Foundation::PWSTR, cchmaxpath: i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwsection: u32, pszpath: super::super::Foundation::PWSTR, cchmaxpath: i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwoptions: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, gpotype: *mut GROUP_POLICY_OBJECT_TYPE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, gphint: *mut GROUP_POLICY_HINT_TYPE) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bmachine: super::super::Foundation::BOOL, badd: super::super::Foundation::BOOL, pguidextension: *mut ::windows::runtime::GUID, pguidsnapin: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IGPM(pub ::windows::runtime::IUnknown);
impl IGPM {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn GetDomain<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrdomain: Param0, bstrdomaincontroller: Param1, ldcflags: i32) -> ::windows::runtime::Result<IGPMDomain> {
        let mut result__: <IGPMDomain as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), bstrdomain.into_param().abi(), bstrdomaincontroller.into_param().abi(), ::core::mem::transmute(ldcflags), &mut result__).from_abi::<IGPMDomain>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn GetBackupDir<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrbackupdir: Param0) -> ::windows::runtime::Result<IGPMBackupDir> {
        let mut result__: <IGPMBackupDir as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), bstrbackupdir.into_param().abi(), &mut result__).from_abi::<IGPMBackupDir>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn GetSitesContainer<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrforest: Param0, bstrdomain: Param1, bstrdomaincontroller: Param2, ldcflags: i32) -> ::windows::runtime::Result<IGPMSitesContainer> {
        let mut result__: <IGPMSitesContainer as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), bstrforest.into_param().abi(), bstrdomain.into_param().abi(), bstrdomaincontroller.into_param().abi(), ::core::mem::transmute(ldcflags), &mut result__).from_abi::<IGPMSitesContainer>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn GetRSOP<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, gpmrsopmode: GPMRSOPMode, bstrnamespace: Param1, lflags: i32) -> ::windows::runtime::Result<IGPMRSOP> {
        let mut result__: <IGPMRSOP as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(gpmrsopmode), bstrnamespace.into_param().abi(), ::core::mem::transmute(lflags), &mut result__).from_abi::<IGPMRSOP>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn CreatePermission<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrtrustee: Param0, perm: GPMPermissionType, binheritable: i16) -> ::windows::runtime::Result<IGPMPermission> {
        let mut result__: <IGPMPermission as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), bstrtrustee.into_param().abi(), ::core::mem::transmute(perm), ::core::mem::transmute(binheritable), &mut result__).from_abi::<IGPMPermission>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn CreateSearchCriteria(&self) -> ::windows::runtime::Result<IGPMSearchCriteria> {
        let mut result__: <IGPMSearchCriteria as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IGPMSearchCriteria>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn CreateTrustee<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrtrustee: Param0) -> ::windows::runtime::Result<IGPMTrustee> {
        let mut result__: <IGPMTrustee as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), bstrtrustee.into_param().abi(), &mut result__).from_abi::<IGPMTrustee>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn GetClientSideExtensions(&self) -> ::windows::runtime::Result<IGPMCSECollection> {
        let mut result__: <IGPMCSECollection as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IGPMCSECollection>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn GetConstants(&self) -> ::windows::runtime::Result<IGPMConstants> {
        let mut result__: <IGPMConstants as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IGPMConstants>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn GetMigrationTable<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrmigrationtablepath: Param0) -> ::windows::runtime::Result<IGPMMigrationTable> {
        let mut result__: <IGPMMigrationTable as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), bstrmigrationtablepath.into_param().abi(), &mut result__).from_abi::<IGPMMigrationTable>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn CreateMigrationTable(&self) -> ::windows::runtime::Result<IGPMMigrationTable> {
        let mut result__: <IGPMMigrationTable as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IGPMMigrationTable>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn InitializeReporting<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstradmpath: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), bstradmpath.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IGPM {
    type Vtable = IGPM_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xf5fae809_3bd6_4da9_a65e_17665b41d763);
}
impl ::core::convert::From<IGPM> for ::windows::runtime::IUnknown {
    fn from(value: IGPM) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IGPM> for ::windows::runtime::IUnknown {
    fn from(value: &IGPM) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IGPM {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IGPM {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGPM> for super::Com::IDispatch {
    fn from(value: IGPM) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGPM> for super::Com::IDispatch {
    fn from(value: &IGPM) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for IGPM {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for &IGPM {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGPM_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrdomain: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdomaincontroller: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ldcflags: i32, pigpmdomain: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrbackupdir: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pigpmbackupdir: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrforest: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdomain: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdomaincontroller: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ldcflags: i32, ppigpmsitescontainer: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, gpmrsopmode: GPMRSOPMode, bstrnamespace: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, ppigpmrsop: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrtrustee: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, perm: GPMPermissionType, binheritable: i16, ppperm: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppigpmsearchcriteria: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrtrustee: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppigpmtrustee: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppigpmcsecollection: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppigpmconstants: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrmigrationtablepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppmigrationtable: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppmigrationtable: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstradmpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IGPM2(pub ::windows::runtime::IUnknown);
impl IGPM2 {
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_System_Com`*"]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::Com::ITypeInfo> {
        let mut result__: <super::Com::ITypeInfo as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), &mut result__).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(dispidmember),
            ::core::mem::transmute(riid),
            ::core::mem::transmute(lcid),
            ::core::mem::transmute(wflags),
            ::core::mem::transmute(pdispparams),
            ::core::mem::transmute(pvarresult),
            ::core::mem::transmute(pexcepinfo),
            ::core::mem::transmute(puargerr),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn GetDomain<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrdomain: Param0, bstrdomaincontroller: Param1, ldcflags: i32) -> ::windows::runtime::Result<IGPMDomain> {
        let mut result__: <IGPMDomain as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), bstrdomain.into_param().abi(), bstrdomaincontroller.into_param().abi(), ::core::mem::transmute(ldcflags), &mut result__).from_abi::<IGPMDomain>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn GetBackupDir<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrbackupdir: Param0) -> ::windows::runtime::Result<IGPMBackupDir> {
        let mut result__: <IGPMBackupDir as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), bstrbackupdir.into_param().abi(), &mut result__).from_abi::<IGPMBackupDir>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn GetSitesContainer<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrforest: Param0, bstrdomain: Param1, bstrdomaincontroller: Param2, ldcflags: i32) -> ::windows::runtime::Result<IGPMSitesContainer> {
        let mut result__: <IGPMSitesContainer as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), bstrforest.into_param().abi(), bstrdomain.into_param().abi(), bstrdomaincontroller.into_param().abi(), ::core::mem::transmute(ldcflags), &mut result__).from_abi::<IGPMSitesContainer>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn GetRSOP<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, gpmrsopmode: GPMRSOPMode, bstrnamespace: Param1, lflags: i32) -> ::windows::runtime::Result<IGPMRSOP> {
        let mut result__: <IGPMRSOP as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(gpmrsopmode), bstrnamespace.into_param().abi(), ::core::mem::transmute(lflags), &mut result__).from_abi::<IGPMRSOP>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn CreatePermission<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrtrustee: Param0, perm: GPMPermissionType, binheritable: i16) -> ::windows::runtime::Result<IGPMPermission> {
        let mut result__: <IGPMPermission as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), bstrtrustee.into_param().abi(), ::core::mem::transmute(perm), ::core::mem::transmute(binheritable), &mut result__).from_abi::<IGPMPermission>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn CreateSearchCriteria(&self) -> ::windows::runtime::Result<IGPMSearchCriteria> {
        let mut result__: <IGPMSearchCriteria as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IGPMSearchCriteria>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn CreateTrustee<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrtrustee: Param0) -> ::windows::runtime::Result<IGPMTrustee> {
        let mut result__: <IGPMTrustee as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), bstrtrustee.into_param().abi(), &mut result__).from_abi::<IGPMTrustee>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn GetClientSideExtensions(&self) -> ::windows::runtime::Result<IGPMCSECollection> {
        let mut result__: <IGPMCSECollection as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IGPMCSECollection>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn GetConstants(&self) -> ::windows::runtime::Result<IGPMConstants> {
        let mut result__: <IGPMConstants as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IGPMConstants>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn GetMigrationTable<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrmigrationtablepath: Param0) -> ::windows::runtime::Result<IGPMMigrationTable> {
        let mut result__: <IGPMMigrationTable as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), bstrmigrationtablepath.into_param().abi(), &mut result__).from_abi::<IGPMMigrationTable>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn CreateMigrationTable(&self) -> ::windows::runtime::Result<IGPMMigrationTable> {
        let mut result__: <IGPMMigrationTable as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IGPMMigrationTable>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn InitializeReporting<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstradmpath: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), bstradmpath.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn GetBackupDirEx<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrbackupdir: Param0, backupdirtype: GPMBackupType) -> ::windows::runtime::Result<IGPMBackupDirEx> {
        let mut result__: <IGPMBackupDirEx as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), bstrbackupdir.into_param().abi(), ::core::mem::transmute(backupdirtype), &mut result__).from_abi::<IGPMBackupDirEx>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn InitializeReportingEx<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstradmpath: Param0, reportingoptions: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self), bstradmpath.into_param().abi(), ::core::mem::transmute(reportingoptions)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IGPM2 {
    type Vtable = IGPM2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x00238f8a_3d86_41ac_8f5e_06a6638a634a);
}
impl ::core::convert::From<IGPM2> for ::windows::runtime::IUnknown {
    fn from(value: IGPM2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IGPM2> for ::windows::runtime::IUnknown {
    fn from(value: &IGPM2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IGPM2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IGPM2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IGPM2> for IGPM {
    fn from(value: IGPM2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IGPM2> for IGPM {
    fn from(value: &IGPM2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IGPM> for IGPM2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IGPM> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IGPM> for &IGPM2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IGPM> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGPM2> for super::Com::IDispatch {
    fn from(value: IGPM2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGPM2> for super::Com::IDispatch {
    fn from(value: &IGPM2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for IGPM2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for &IGPM2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGPM2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrdomain: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdomaincontroller: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ldcflags: i32, pigpmdomain: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrbackupdir: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pigpmbackupdir: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrforest: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdomain: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdomaincontroller: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ldcflags: i32, ppigpmsitescontainer: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, gpmrsopmode: GPMRSOPMode, bstrnamespace: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, ppigpmrsop: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrtrustee: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, perm: GPMPermissionType, binheritable: i16, ppperm: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppigpmsearchcriteria: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrtrustee: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppigpmtrustee: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppigpmcsecollection: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppigpmconstants: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrmigrationtablepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppmigrationtable: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppmigrationtable: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstradmpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrbackupdir: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, backupdirtype: GPMBackupType, ppigpmbackupdirex: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstradmpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, reportingoptions: i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IGPMAsyncCancel(pub ::windows::runtime::IUnknown);
impl IGPMAsyncCancel {
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn Cancel(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IGPMAsyncCancel {
    type Vtable = IGPMAsyncCancel_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xddc67754_be67_4541_8166_f48166868c9c);
}
impl ::core::convert::From<IGPMAsyncCancel> for ::windows::runtime::IUnknown {
    fn from(value: IGPMAsyncCancel) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IGPMAsyncCancel> for ::windows::runtime::IUnknown {
    fn from(value: &IGPMAsyncCancel) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IGPMAsyncCancel {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IGPMAsyncCancel {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGPMAsyncCancel> for super::Com::IDispatch {
    fn from(value: IGPMAsyncCancel) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGPMAsyncCancel> for super::Com::IDispatch {
    fn from(value: &IGPMAsyncCancel) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for IGPMAsyncCancel {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for &IGPMAsyncCancel {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGPMAsyncCancel_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IGPMAsyncProgress(pub ::windows::runtime::IUnknown);
impl IGPMAsyncProgress {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Status<'a, Param4: ::windows::runtime::IntoParam<'a, IGPMStatusMsgCollection>>(&self, lprogressnumerator: i32, lprogressdenominator: i32, hrstatus: ::windows::runtime::HRESULT, presult: *const super::Com::VARIANT, ppigpmstatusmsgcollection: Param4) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(lprogressnumerator), ::core::mem::transmute(lprogressdenominator), ::core::mem::transmute(hrstatus), ::core::mem::transmute(presult), ppigpmstatusmsgcollection.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IGPMAsyncProgress {
    type Vtable = IGPMAsyncProgress_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x6aac29f8_5948_4324_bf70_423818942dbc);
}
impl ::core::convert::From<IGPMAsyncProgress> for ::windows::runtime::IUnknown {
    fn from(value: IGPMAsyncProgress) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IGPMAsyncProgress> for ::windows::runtime::IUnknown {
    fn from(value: &IGPMAsyncProgress) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IGPMAsyncProgress {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IGPMAsyncProgress {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGPMAsyncProgress> for super::Com::IDispatch {
    fn from(value: IGPMAsyncProgress) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGPMAsyncProgress> for super::Com::IDispatch {
    fn from(value: &IGPMAsyncProgress) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for IGPMAsyncProgress {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for &IGPMAsyncProgress {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGPMAsyncProgress_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lprogressnumerator: i32, lprogressdenominator: i32, hrstatus: ::windows::runtime::HRESULT, presult: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppigpmstatusmsgcollection: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IGPMBackup(pub ::windows::runtime::IUnknown);
impl IGPMBackup {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn ID(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn GPOID(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn GPODomain(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn GPODisplayName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn Timestamp(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn Comment(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn BackupDir(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn Delete(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn GenerateReport(&self, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(gpmreporttype), ::core::mem::transmute(pvargpmprogress), ::core::mem::transmute(pvargpmcancel), ::core::mem::transmute(ppigpmresult)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn GenerateReportToFile<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, gpmreporttype: GPMReportType, bstrtargetfilepath: Param1) -> ::windows::runtime::Result<IGPMResult> {
        let mut result__: <IGPMResult as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(gpmreporttype), bstrtargetfilepath.into_param().abi(), &mut result__).from_abi::<IGPMResult>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IGPMBackup {
    type Vtable = IGPMBackup_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xd8a16a35_3b0d_416b_8d02_4df6f95a7119);
}
impl ::core::convert::From<IGPMBackup> for ::windows::runtime::IUnknown {
    fn from(value: IGPMBackup) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IGPMBackup> for ::windows::runtime::IUnknown {
    fn from(value: &IGPMBackup) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IGPMBackup {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IGPMBackup {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGPMBackup> for super::Com::IDispatch {
    fn from(value: IGPMBackup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGPMBackup> for super::Com::IDispatch {
    fn from(value: &IGPMBackup) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for IGPMBackup {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for &IGPMBackup {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGPMBackup_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut f64) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, gpmreporttype: GPMReportType, pvargpmprogress: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, pvargpmcancel: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppigpmresult: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, gpmreporttype: GPMReportType, bstrtargetfilepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppigpmresult: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IGPMBackupCollection(pub ::windows::runtime::IUnknown);
impl IGPMBackupCollection {
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn Count(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Item(&self, lindex: i32) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(lindex), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(feature = "Win32_System_Ole")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_System_Ole`*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::runtime::Result<super::Ole::IEnumVARIANT> {
        let mut result__: <super::Ole::IEnumVARIANT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Ole::IEnumVARIANT>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IGPMBackupCollection {
    type Vtable = IGPMBackupCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xc786fc0f_26d8_4bab_a745_39ca7e800cac);
}
impl ::core::convert::From<IGPMBackupCollection> for ::windows::runtime::IUnknown {
    fn from(value: IGPMBackupCollection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IGPMBackupCollection> for ::windows::runtime::IUnknown {
    fn from(value: &IGPMBackupCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IGPMBackupCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IGPMBackupCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGPMBackupCollection> for super::Com::IDispatch {
    fn from(value: IGPMBackupCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGPMBackupCollection> for super::Com::IDispatch {
    fn from(value: &IGPMBackupCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for IGPMBackupCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for &IGPMBackupCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGPMBackupCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lindex: i32, pval: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_System_Ole")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppigpmbackup: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))] usize,
);
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IGPMBackupDir(pub ::windows::runtime::IUnknown);
impl IGPMBackupDir {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn BackupDirectory(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn GetBackup<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrid: Param0) -> ::windows::runtime::Result<IGPMBackup> {
        let mut result__: <IGPMBackup as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), bstrid.into_param().abi(), &mut result__).from_abi::<IGPMBackup>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn SearchBackups<'a, Param0: ::windows::runtime::IntoParam<'a, IGPMSearchCriteria>>(&self, pigpmsearchcriteria: Param0) -> ::windows::runtime::Result<IGPMBackupCollection> {
        let mut result__: <IGPMBackupCollection as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), pigpmsearchcriteria.into_param().abi(), &mut result__).from_abi::<IGPMBackupCollection>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IGPMBackupDir {
    type Vtable = IGPMBackupDir_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xb1568bed_0a93_4acc_810f_afe7081019b9);
}
impl ::core::convert::From<IGPMBackupDir> for ::windows::runtime::IUnknown {
    fn from(value: IGPMBackupDir) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IGPMBackupDir> for ::windows::runtime::IUnknown {
    fn from(value: &IGPMBackupDir) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IGPMBackupDir {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IGPMBackupDir {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGPMBackupDir> for super::Com::IDispatch {
    fn from(value: IGPMBackupDir) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGPMBackupDir> for super::Com::IDispatch {
    fn from(value: &IGPMBackupDir) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for IGPMBackupDir {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for &IGPMBackupDir {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGPMBackupDir_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppbackup: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pigpmsearchcriteria: ::windows::runtime::RawPtr, ppigpmbackupcollection: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IGPMBackupDirEx(pub ::windows::runtime::IUnknown);
impl IGPMBackupDirEx {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn BackupDir(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn BackupType(&self) -> ::windows::runtime::Result<GPMBackupType> {
        let mut result__: <GPMBackupType as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMBackupType>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn GetBackup<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrid: Param0) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), bstrid.into_param().abi(), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn SearchBackups<'a, Param0: ::windows::runtime::IntoParam<'a, IGPMSearchCriteria>>(&self, pigpmsearchcriteria: Param0) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), pigpmsearchcriteria.into_param().abi(), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IGPMBackupDirEx {
    type Vtable = IGPMBackupDirEx_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xf8dc55ed_3ba0_4864_aad4_d365189ee1d5);
}
impl ::core::convert::From<IGPMBackupDirEx> for ::windows::runtime::IUnknown {
    fn from(value: IGPMBackupDirEx) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IGPMBackupDirEx> for ::windows::runtime::IUnknown {
    fn from(value: &IGPMBackupDirEx) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IGPMBackupDirEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IGPMBackupDirEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGPMBackupDirEx> for super::Com::IDispatch {
    fn from(value: IGPMBackupDirEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGPMBackupDirEx> for super::Com::IDispatch {
    fn from(value: &IGPMBackupDirEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for IGPMBackupDirEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for &IGPMBackupDirEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGPMBackupDirEx_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrbackupdir: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pgpmbackuptype: *mut GPMBackupType) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarbackup: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pigpmsearchcriteria: ::windows::runtime::RawPtr, pvarbackupcollection: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IGPMCSECollection(pub ::windows::runtime::IUnknown);
impl IGPMCSECollection {
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn Count(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Item(&self, lindex: i32) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(lindex), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(feature = "Win32_System_Ole")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_System_Ole`*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::runtime::Result<super::Ole::IEnumVARIANT> {
        let mut result__: <super::Ole::IEnumVARIANT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Ole::IEnumVARIANT>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IGPMCSECollection {
    type Vtable = IGPMCSECollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x2e52a97d_0a4a_4a6f_85db_201622455da0);
}
impl ::core::convert::From<IGPMCSECollection> for ::windows::runtime::IUnknown {
    fn from(value: IGPMCSECollection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IGPMCSECollection> for ::windows::runtime::IUnknown {
    fn from(value: &IGPMCSECollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IGPMCSECollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IGPMCSECollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGPMCSECollection> for super::Com::IDispatch {
    fn from(value: IGPMCSECollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGPMCSECollection> for super::Com::IDispatch {
    fn from(value: &IGPMCSECollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for IGPMCSECollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for &IGPMCSECollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGPMCSECollection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lindex: i32, pval: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_System_Ole")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppigpmcses: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))] usize,
);
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IGPMClientSideExtension(pub ::windows::runtime::IUnknown);
impl IGPMClientSideExtension {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn ID(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn DisplayName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn IsUserEnabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn IsComputerEnabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IGPMClientSideExtension {
    type Vtable = IGPMClientSideExtension_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x69da7488_b8db_415e_9266_901be4d49928);
}
impl ::core::convert::From<IGPMClientSideExtension> for ::windows::runtime::IUnknown {
    fn from(value: IGPMClientSideExtension) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IGPMClientSideExtension> for ::windows::runtime::IUnknown {
    fn from(value: &IGPMClientSideExtension) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IGPMClientSideExtension {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IGPMClientSideExtension {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGPMClientSideExtension> for super::Com::IDispatch {
    fn from(value: IGPMClientSideExtension) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGPMClientSideExtension> for super::Com::IDispatch {
    fn from(value: &IGPMClientSideExtension) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for IGPMClientSideExtension {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for &IGPMClientSideExtension {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGPMClientSideExtension_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvbenabled: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvbenabled: *mut i16) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IGPMConstants(pub ::windows::runtime::IUnknown);
impl IGPMConstants {
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn PermGPOApply(&self) -> ::windows::runtime::Result<GPMPermissionType> {
        let mut result__: <GPMPermissionType as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMPermissionType>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn PermGPORead(&self) -> ::windows::runtime::Result<GPMPermissionType> {
        let mut result__: <GPMPermissionType as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMPermissionType>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn PermGPOEdit(&self) -> ::windows::runtime::Result<GPMPermissionType> {
        let mut result__: <GPMPermissionType as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMPermissionType>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn PermGPOEditSecurityAndDelete(&self) -> ::windows::runtime::Result<GPMPermissionType> {
        let mut result__: <GPMPermissionType as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMPermissionType>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn PermGPOCustom(&self) -> ::windows::runtime::Result<GPMPermissionType> {
        let mut result__: <GPMPermissionType as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMPermissionType>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn PermWMIFilterEdit(&self) -> ::windows::runtime::Result<GPMPermissionType> {
        let mut result__: <GPMPermissionType as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMPermissionType>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn PermWMIFilterFullControl(&self) -> ::windows::runtime::Result<GPMPermissionType> {
        let mut result__: <GPMPermissionType as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMPermissionType>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn PermWMIFilterCustom(&self) -> ::windows::runtime::Result<GPMPermissionType> {
        let mut result__: <GPMPermissionType as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMPermissionType>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn PermSOMLink(&self) -> ::windows::runtime::Result<GPMPermissionType> {
        let mut result__: <GPMPermissionType as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMPermissionType>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn PermSOMLogging(&self) -> ::windows::runtime::Result<GPMPermissionType> {
        let mut result__: <GPMPermissionType as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMPermissionType>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn PermSOMPlanning(&self) -> ::windows::runtime::Result<GPMPermissionType> {
        let mut result__: <GPMPermissionType as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMPermissionType>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn PermSOMGPOCreate(&self) -> ::windows::runtime::Result<GPMPermissionType> {
        let mut result__: <GPMPermissionType as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMPermissionType>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn PermSOMWMICreate(&self) -> ::windows::runtime::Result<GPMPermissionType> {
        let mut result__: <GPMPermissionType as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMPermissionType>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn PermSOMWMIFullControl(&self) -> ::windows::runtime::Result<GPMPermissionType> {
        let mut result__: <GPMPermissionType as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMPermissionType>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn SearchPropertyGPOPermissions(&self) -> ::windows::runtime::Result<GPMSearchProperty> {
        let mut result__: <GPMSearchProperty as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMSearchProperty>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn SearchPropertyGPOEffectivePermissions(&self) -> ::windows::runtime::Result<GPMSearchProperty> {
        let mut result__: <GPMSearchProperty as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMSearchProperty>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn SearchPropertyGPODisplayName(&self) -> ::windows::runtime::Result<GPMSearchProperty> {
        let mut result__: <GPMSearchProperty as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMSearchProperty>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn SearchPropertyGPOWMIFilter(&self) -> ::windows::runtime::Result<GPMSearchProperty> {
        let mut result__: <GPMSearchProperty as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMSearchProperty>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn SearchPropertyGPOID(&self) -> ::windows::runtime::Result<GPMSearchProperty> {
        let mut result__: <GPMSearchProperty as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMSearchProperty>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn SearchPropertyGPOComputerExtensions(&self) -> ::windows::runtime::Result<GPMSearchProperty> {
        let mut result__: <GPMSearchProperty as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).26)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMSearchProperty>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn SearchPropertyGPOUserExtensions(&self) -> ::windows::runtime::Result<GPMSearchProperty> {
        let mut result__: <GPMSearchProperty as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).27)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMSearchProperty>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn SearchPropertySOMLinks(&self) -> ::windows::runtime::Result<GPMSearchProperty> {
        let mut result__: <GPMSearchProperty as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).28)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMSearchProperty>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn SearchPropertyGPODomain(&self) -> ::windows::runtime::Result<GPMSearchProperty> {
        let mut result__: <GPMSearchProperty as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).29)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMSearchProperty>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn SearchPropertyBackupMostRecent(&self) -> ::windows::runtime::Result<GPMSearchProperty> {
        let mut result__: <GPMSearchProperty as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).30)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMSearchProperty>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn SearchOpEquals(&self) -> ::windows::runtime::Result<GPMSearchOperation> {
        let mut result__: <GPMSearchOperation as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).31)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMSearchOperation>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn SearchOpContains(&self) -> ::windows::runtime::Result<GPMSearchOperation> {
        let mut result__: <GPMSearchOperation as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).32)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMSearchOperation>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn SearchOpNotContains(&self) -> ::windows::runtime::Result<GPMSearchOperation> {
        let mut result__: <GPMSearchOperation as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).33)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMSearchOperation>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn SearchOpNotEquals(&self) -> ::windows::runtime::Result<GPMSearchOperation> {
        let mut result__: <GPMSearchOperation as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).34)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMSearchOperation>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn UsePDC(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).35)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn UseAnyDC(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).36)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn DoNotUseW2KDC(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).37)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn SOMSite(&self) -> ::windows::runtime::Result<GPMSOMType> {
        let mut result__: <GPMSOMType as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).38)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMSOMType>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn SOMDomain(&self) -> ::windows::runtime::Result<GPMSOMType> {
        let mut result__: <GPMSOMType as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).39)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMSOMType>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn SOMOU(&self) -> ::windows::runtime::Result<GPMSOMType> {
        let mut result__: <GPMSOMType as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).40)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMSOMType>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn SecurityFlags(&self, vbowner: i16, vbgroup: i16, vbdacl: i16, vbsacl: i16) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).41)(::core::mem::transmute_copy(self), ::core::mem::transmute(vbowner), ::core::mem::transmute(vbgroup), ::core::mem::transmute(vbdacl), ::core::mem::transmute(vbsacl), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn DoNotValidateDC(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).42)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn ReportHTML(&self) -> ::windows::runtime::Result<GPMReportType> {
        let mut result__: <GPMReportType as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).43)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMReportType>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn ReportXML(&self) -> ::windows::runtime::Result<GPMReportType> {
        let mut result__: <GPMReportType as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).44)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMReportType>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn RSOPModeUnknown(&self) -> ::windows::runtime::Result<GPMRSOPMode> {
        let mut result__: <GPMRSOPMode as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).45)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMRSOPMode>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn RSOPModePlanning(&self) -> ::windows::runtime::Result<GPMRSOPMode> {
        let mut result__: <GPMRSOPMode as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).46)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMRSOPMode>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn RSOPModeLogging(&self) -> ::windows::runtime::Result<GPMRSOPMode> {
        let mut result__: <GPMRSOPMode as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).47)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMRSOPMode>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn EntryTypeUser(&self) -> ::windows::runtime::Result<GPMEntryType> {
        let mut result__: <GPMEntryType as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).48)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMEntryType>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn EntryTypeComputer(&self) -> ::windows::runtime::Result<GPMEntryType> {
        let mut result__: <GPMEntryType as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).49)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMEntryType>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn EntryTypeLocalGroup(&self) -> ::windows::runtime::Result<GPMEntryType> {
        let mut result__: <GPMEntryType as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).50)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMEntryType>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn EntryTypeGlobalGroup(&self) -> ::windows::runtime::Result<GPMEntryType> {
        let mut result__: <GPMEntryType as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).51)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMEntryType>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn EntryTypeUniversalGroup(&self) -> ::windows::runtime::Result<GPMEntryType> {
        let mut result__: <GPMEntryType as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).52)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMEntryType>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn EntryTypeUNCPath(&self) -> ::windows::runtime::Result<GPMEntryType> {
        let mut result__: <GPMEntryType as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).53)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMEntryType>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn EntryTypeUnknown(&self) -> ::windows::runtime::Result<GPMEntryType> {
        let mut result__: <GPMEntryType as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).54)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMEntryType>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn DestinationOptionSameAsSource(&self) -> ::windows::runtime::Result<GPMDestinationOption> {
        let mut result__: <GPMDestinationOption as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).55)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMDestinationOption>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn DestinationOptionNone(&self) -> ::windows::runtime::Result<GPMDestinationOption> {
        let mut result__: <GPMDestinationOption as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).56)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMDestinationOption>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn DestinationOptionByRelativeName(&self) -> ::windows::runtime::Result<GPMDestinationOption> {
        let mut result__: <GPMDestinationOption as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).57)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMDestinationOption>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn DestinationOptionSet(&self) -> ::windows::runtime::Result<GPMDestinationOption> {
        let mut result__: <GPMDestinationOption as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).58)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMDestinationOption>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn MigrationTableOnly(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).59)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn ProcessSecurity(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).60)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn RsopLoggingNoComputer(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).61)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn RsopLoggingNoUser(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).62)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn RsopPlanningAssumeSlowLink(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).63)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn RsopPlanningLoopbackOption(&self, vbmerge: i16) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).64)(::core::mem::transmute_copy(self), ::core::mem::transmute(vbmerge), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn RsopPlanningAssumeUserWQLFilterTrue(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).65)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn RsopPlanningAssumeCompWQLFilterTrue(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).66)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IGPMConstants {
    type Vtable = IGPMConstants_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x50ef73e6_d35c_4c8d_be63_7ea5d2aac5c4);
}
impl ::core::convert::From<IGPMConstants> for ::windows::runtime::IUnknown {
    fn from(value: IGPMConstants) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IGPMConstants> for ::windows::runtime::IUnknown {
    fn from(value: &IGPMConstants) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IGPMConstants {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IGPMConstants {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGPMConstants> for super::Com::IDispatch {
    fn from(value: IGPMConstants) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGPMConstants> for super::Com::IDispatch {
    fn from(value: &IGPMConstants) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for IGPMConstants {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for &IGPMConstants {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGPMConstants_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMPermissionType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMPermissionType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMPermissionType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMPermissionType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMPermissionType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMPermissionType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMPermissionType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMPermissionType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMPermissionType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMPermissionType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMPermissionType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMPermissionType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMPermissionType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMPermissionType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMSearchProperty) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMSearchProperty) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMSearchProperty) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMSearchProperty) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMSearchProperty) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMSearchProperty) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMSearchProperty) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMSearchProperty) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMSearchProperty) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMSearchProperty) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMSearchOperation) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMSearchOperation) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMSearchOperation) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMSearchOperation) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMSOMType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMSOMType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMSOMType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, vbowner: i16, vbgroup: i16, vbdacl: i16, vbsacl: i16, pval: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMReportType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMReportType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMRSOPMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMRSOPMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMRSOPMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMEntryType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMEntryType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMEntryType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMEntryType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMEntryType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMEntryType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMEntryType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMDestinationOption) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMDestinationOption) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMDestinationOption) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMDestinationOption) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, vbmerge: i16, pval: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IGPMConstants2(pub ::windows::runtime::IUnknown);
impl IGPMConstants2 {
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_System_Com`*"]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::Com::ITypeInfo> {
        let mut result__: <super::Com::ITypeInfo as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), &mut result__).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(dispidmember),
            ::core::mem::transmute(riid),
            ::core::mem::transmute(lcid),
            ::core::mem::transmute(wflags),
            ::core::mem::transmute(pdispparams),
            ::core::mem::transmute(pvarresult),
            ::core::mem::transmute(pexcepinfo),
            ::core::mem::transmute(puargerr),
        )
        .ok()
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn PermGPOApply(&self) -> ::windows::runtime::Result<GPMPermissionType> {
        let mut result__: <GPMPermissionType as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMPermissionType>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn PermGPORead(&self) -> ::windows::runtime::Result<GPMPermissionType> {
        let mut result__: <GPMPermissionType as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMPermissionType>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn PermGPOEdit(&self) -> ::windows::runtime::Result<GPMPermissionType> {
        let mut result__: <GPMPermissionType as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMPermissionType>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn PermGPOEditSecurityAndDelete(&self) -> ::windows::runtime::Result<GPMPermissionType> {
        let mut result__: <GPMPermissionType as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMPermissionType>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn PermGPOCustom(&self) -> ::windows::runtime::Result<GPMPermissionType> {
        let mut result__: <GPMPermissionType as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMPermissionType>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn PermWMIFilterEdit(&self) -> ::windows::runtime::Result<GPMPermissionType> {
        let mut result__: <GPMPermissionType as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMPermissionType>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn PermWMIFilterFullControl(&self) -> ::windows::runtime::Result<GPMPermissionType> {
        let mut result__: <GPMPermissionType as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMPermissionType>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn PermWMIFilterCustom(&self) -> ::windows::runtime::Result<GPMPermissionType> {
        let mut result__: <GPMPermissionType as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMPermissionType>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn PermSOMLink(&self) -> ::windows::runtime::Result<GPMPermissionType> {
        let mut result__: <GPMPermissionType as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMPermissionType>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn PermSOMLogging(&self) -> ::windows::runtime::Result<GPMPermissionType> {
        let mut result__: <GPMPermissionType as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMPermissionType>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn PermSOMPlanning(&self) -> ::windows::runtime::Result<GPMPermissionType> {
        let mut result__: <GPMPermissionType as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMPermissionType>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn PermSOMGPOCreate(&self) -> ::windows::runtime::Result<GPMPermissionType> {
        let mut result__: <GPMPermissionType as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMPermissionType>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn PermSOMWMICreate(&self) -> ::windows::runtime::Result<GPMPermissionType> {
        let mut result__: <GPMPermissionType as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMPermissionType>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn PermSOMWMIFullControl(&self) -> ::windows::runtime::Result<GPMPermissionType> {
        let mut result__: <GPMPermissionType as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMPermissionType>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn SearchPropertyGPOPermissions(&self) -> ::windows::runtime::Result<GPMSearchProperty> {
        let mut result__: <GPMSearchProperty as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMSearchProperty>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn SearchPropertyGPOEffectivePermissions(&self) -> ::windows::runtime::Result<GPMSearchProperty> {
        let mut result__: <GPMSearchProperty as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMSearchProperty>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn SearchPropertyGPODisplayName(&self) -> ::windows::runtime::Result<GPMSearchProperty> {
        let mut result__: <GPMSearchProperty as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMSearchProperty>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn SearchPropertyGPOWMIFilter(&self) -> ::windows::runtime::Result<GPMSearchProperty> {
        let mut result__: <GPMSearchProperty as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMSearchProperty>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn SearchPropertyGPOID(&self) -> ::windows::runtime::Result<GPMSearchProperty> {
        let mut result__: <GPMSearchProperty as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMSearchProperty>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn SearchPropertyGPOComputerExtensions(&self) -> ::windows::runtime::Result<GPMSearchProperty> {
        let mut result__: <GPMSearchProperty as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).26)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMSearchProperty>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn SearchPropertyGPOUserExtensions(&self) -> ::windows::runtime::Result<GPMSearchProperty> {
        let mut result__: <GPMSearchProperty as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).27)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMSearchProperty>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn SearchPropertySOMLinks(&self) -> ::windows::runtime::Result<GPMSearchProperty> {
        let mut result__: <GPMSearchProperty as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).28)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMSearchProperty>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn SearchPropertyGPODomain(&self) -> ::windows::runtime::Result<GPMSearchProperty> {
        let mut result__: <GPMSearchProperty as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).29)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMSearchProperty>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn SearchPropertyBackupMostRecent(&self) -> ::windows::runtime::Result<GPMSearchProperty> {
        let mut result__: <GPMSearchProperty as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).30)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMSearchProperty>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn SearchOpEquals(&self) -> ::windows::runtime::Result<GPMSearchOperation> {
        let mut result__: <GPMSearchOperation as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).31)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMSearchOperation>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn SearchOpContains(&self) -> ::windows::runtime::Result<GPMSearchOperation> {
        let mut result__: <GPMSearchOperation as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).32)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMSearchOperation>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn SearchOpNotContains(&self) -> ::windows::runtime::Result<GPMSearchOperation> {
        let mut result__: <GPMSearchOperation as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).33)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMSearchOperation>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn SearchOpNotEquals(&self) -> ::windows::runtime::Result<GPMSearchOperation> {
        let mut result__: <GPMSearchOperation as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).34)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMSearchOperation>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn UsePDC(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).35)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn UseAnyDC(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).36)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn DoNotUseW2KDC(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).37)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn SOMSite(&self) -> ::windows::runtime::Result<GPMSOMType> {
        let mut result__: <GPMSOMType as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).38)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMSOMType>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn SOMDomain(&self) -> ::windows::runtime::Result<GPMSOMType> {
        let mut result__: <GPMSOMType as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).39)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMSOMType>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn SOMOU(&self) -> ::windows::runtime::Result<GPMSOMType> {
        let mut result__: <GPMSOMType as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).40)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMSOMType>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn SecurityFlags(&self, vbowner: i16, vbgroup: i16, vbdacl: i16, vbsacl: i16) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).41)(::core::mem::transmute_copy(self), ::core::mem::transmute(vbowner), ::core::mem::transmute(vbgroup), ::core::mem::transmute(vbdacl), ::core::mem::transmute(vbsacl), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn DoNotValidateDC(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).42)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn ReportHTML(&self) -> ::windows::runtime::Result<GPMReportType> {
        let mut result__: <GPMReportType as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).43)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMReportType>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn ReportXML(&self) -> ::windows::runtime::Result<GPMReportType> {
        let mut result__: <GPMReportType as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).44)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMReportType>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn RSOPModeUnknown(&self) -> ::windows::runtime::Result<GPMRSOPMode> {
        let mut result__: <GPMRSOPMode as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).45)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMRSOPMode>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn RSOPModePlanning(&self) -> ::windows::runtime::Result<GPMRSOPMode> {
        let mut result__: <GPMRSOPMode as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).46)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMRSOPMode>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn RSOPModeLogging(&self) -> ::windows::runtime::Result<GPMRSOPMode> {
        let mut result__: <GPMRSOPMode as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).47)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMRSOPMode>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn EntryTypeUser(&self) -> ::windows::runtime::Result<GPMEntryType> {
        let mut result__: <GPMEntryType as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).48)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMEntryType>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn EntryTypeComputer(&self) -> ::windows::runtime::Result<GPMEntryType> {
        let mut result__: <GPMEntryType as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).49)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMEntryType>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn EntryTypeLocalGroup(&self) -> ::windows::runtime::Result<GPMEntryType> {
        let mut result__: <GPMEntryType as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).50)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMEntryType>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn EntryTypeGlobalGroup(&self) -> ::windows::runtime::Result<GPMEntryType> {
        let mut result__: <GPMEntryType as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).51)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMEntryType>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn EntryTypeUniversalGroup(&self) -> ::windows::runtime::Result<GPMEntryType> {
        let mut result__: <GPMEntryType as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).52)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMEntryType>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn EntryTypeUNCPath(&self) -> ::windows::runtime::Result<GPMEntryType> {
        let mut result__: <GPMEntryType as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).53)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMEntryType>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn EntryTypeUnknown(&self) -> ::windows::runtime::Result<GPMEntryType> {
        let mut result__: <GPMEntryType as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).54)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMEntryType>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn DestinationOptionSameAsSource(&self) -> ::windows::runtime::Result<GPMDestinationOption> {
        let mut result__: <GPMDestinationOption as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).55)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMDestinationOption>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn DestinationOptionNone(&self) -> ::windows::runtime::Result<GPMDestinationOption> {
        let mut result__: <GPMDestinationOption as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).56)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMDestinationOption>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn DestinationOptionByRelativeName(&self) -> ::windows::runtime::Result<GPMDestinationOption> {
        let mut result__: <GPMDestinationOption as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).57)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMDestinationOption>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn DestinationOptionSet(&self) -> ::windows::runtime::Result<GPMDestinationOption> {
        let mut result__: <GPMDestinationOption as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).58)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMDestinationOption>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn MigrationTableOnly(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).59)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn ProcessSecurity(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).60)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn RsopLoggingNoComputer(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).61)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn RsopLoggingNoUser(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).62)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn RsopPlanningAssumeSlowLink(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).63)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn RsopPlanningLoopbackOption(&self, vbmerge: i16) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).64)(::core::mem::transmute_copy(self), ::core::mem::transmute(vbmerge), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn RsopPlanningAssumeUserWQLFilterTrue(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).65)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn RsopPlanningAssumeCompWQLFilterTrue(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).66)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn BackupTypeGPO(&self) -> ::windows::runtime::Result<GPMBackupType> {
        let mut result__: <GPMBackupType as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).67)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMBackupType>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn BackupTypeStarterGPO(&self) -> ::windows::runtime::Result<GPMBackupType> {
        let mut result__: <GPMBackupType as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).68)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMBackupType>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn StarterGPOTypeSystem(&self) -> ::windows::runtime::Result<GPMStarterGPOType> {
        let mut result__: <GPMStarterGPOType as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).69)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMStarterGPOType>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn StarterGPOTypeCustom(&self) -> ::windows::runtime::Result<GPMStarterGPOType> {
        let mut result__: <GPMStarterGPOType as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).70)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMStarterGPOType>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn SearchPropertyStarterGPOPermissions(&self) -> ::windows::runtime::Result<GPMSearchProperty> {
        let mut result__: <GPMSearchProperty as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).71)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMSearchProperty>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn SearchPropertyStarterGPOEffectivePermissions(&self) -> ::windows::runtime::Result<GPMSearchProperty> {
        let mut result__: <GPMSearchProperty as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).72)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMSearchProperty>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn SearchPropertyStarterGPODisplayName(&self) -> ::windows::runtime::Result<GPMSearchProperty> {
        let mut result__: <GPMSearchProperty as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).73)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMSearchProperty>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn SearchPropertyStarterGPOID(&self) -> ::windows::runtime::Result<GPMSearchProperty> {
        let mut result__: <GPMSearchProperty as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).74)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMSearchProperty>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn SearchPropertyStarterGPODomain(&self) -> ::windows::runtime::Result<GPMSearchProperty> {
        let mut result__: <GPMSearchProperty as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).75)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMSearchProperty>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn PermStarterGPORead(&self) -> ::windows::runtime::Result<GPMPermissionType> {
        let mut result__: <GPMPermissionType as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).76)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMPermissionType>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn PermStarterGPOEdit(&self) -> ::windows::runtime::Result<GPMPermissionType> {
        let mut result__: <GPMPermissionType as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).77)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMPermissionType>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn PermStarterGPOFullControl(&self) -> ::windows::runtime::Result<GPMPermissionType> {
        let mut result__: <GPMPermissionType as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).78)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMPermissionType>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn PermStarterGPOCustom(&self) -> ::windows::runtime::Result<GPMPermissionType> {
        let mut result__: <GPMPermissionType as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).79)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMPermissionType>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn ReportLegacy(&self) -> ::windows::runtime::Result<GPMReportingOptions> {
        let mut result__: <GPMReportingOptions as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).80)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMReportingOptions>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn ReportComments(&self) -> ::windows::runtime::Result<GPMReportingOptions> {
        let mut result__: <GPMReportingOptions as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).81)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMReportingOptions>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IGPMConstants2 {
    type Vtable = IGPMConstants2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x05ae21b0_ac09_4032_a26f_9e7da786dc19);
}
impl ::core::convert::From<IGPMConstants2> for ::windows::runtime::IUnknown {
    fn from(value: IGPMConstants2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IGPMConstants2> for ::windows::runtime::IUnknown {
    fn from(value: &IGPMConstants2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IGPMConstants2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IGPMConstants2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IGPMConstants2> for IGPMConstants {
    fn from(value: IGPMConstants2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IGPMConstants2> for IGPMConstants {
    fn from(value: &IGPMConstants2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IGPMConstants> for IGPMConstants2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IGPMConstants> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IGPMConstants> for &IGPMConstants2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IGPMConstants> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGPMConstants2> for super::Com::IDispatch {
    fn from(value: IGPMConstants2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGPMConstants2> for super::Com::IDispatch {
    fn from(value: &IGPMConstants2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for IGPMConstants2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for &IGPMConstants2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGPMConstants2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMPermissionType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMPermissionType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMPermissionType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMPermissionType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMPermissionType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMPermissionType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMPermissionType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMPermissionType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMPermissionType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMPermissionType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMPermissionType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMPermissionType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMPermissionType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMPermissionType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMSearchProperty) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMSearchProperty) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMSearchProperty) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMSearchProperty) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMSearchProperty) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMSearchProperty) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMSearchProperty) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMSearchProperty) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMSearchProperty) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMSearchProperty) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMSearchOperation) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMSearchOperation) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMSearchOperation) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMSearchOperation) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMSOMType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMSOMType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMSOMType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, vbowner: i16, vbgroup: i16, vbdacl: i16, vbsacl: i16, pval: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMReportType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMReportType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMRSOPMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMRSOPMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMRSOPMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMEntryType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMEntryType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMEntryType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMEntryType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMEntryType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMEntryType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMEntryType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMDestinationOption) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMDestinationOption) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMDestinationOption) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMDestinationOption) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, vbmerge: i16, pval: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMBackupType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMBackupType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMStarterGPOType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMStarterGPOType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMSearchProperty) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMSearchProperty) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMSearchProperty) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMSearchProperty) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMSearchProperty) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMPermissionType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMPermissionType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMPermissionType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMPermissionType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMReportingOptions) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMReportingOptions) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IGPMDomain(pub ::windows::runtime::IUnknown);
impl IGPMDomain {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn DomainController(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn Domain(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn CreateGPO(&self) -> ::windows::runtime::Result<IGPMGPO> {
        let mut result__: <IGPMGPO as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IGPMGPO>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn GetGPO<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrguid: Param0) -> ::windows::runtime::Result<IGPMGPO> {
        let mut result__: <IGPMGPO as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), bstrguid.into_param().abi(), &mut result__).from_abi::<IGPMGPO>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn SearchGPOs<'a, Param0: ::windows::runtime::IntoParam<'a, IGPMSearchCriteria>>(&self, pigpmsearchcriteria: Param0) -> ::windows::runtime::Result<IGPMGPOCollection> {
        let mut result__: <IGPMGPOCollection as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), pigpmsearchcriteria.into_param().abi(), &mut result__).from_abi::<IGPMGPOCollection>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn RestoreGPO<'a, Param0: ::windows::runtime::IntoParam<'a, IGPMBackup>>(&self, pigpmbackup: Param0, ldcflags: i32, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), pigpmbackup.into_param().abi(), ::core::mem::transmute(ldcflags), ::core::mem::transmute(pvargpmprogress), ::core::mem::transmute(pvargpmcancel), ::core::mem::transmute(ppigpmresult)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn GetSOM<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrpath: Param0) -> ::windows::runtime::Result<IGPMSOM> {
        let mut result__: <IGPMSOM as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), bstrpath.into_param().abi(), &mut result__).from_abi::<IGPMSOM>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn SearchSOMs<'a, Param0: ::windows::runtime::IntoParam<'a, IGPMSearchCriteria>>(&self, pigpmsearchcriteria: Param0) -> ::windows::runtime::Result<IGPMSOMCollection> {
        let mut result__: <IGPMSOMCollection as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), pigpmsearchcriteria.into_param().abi(), &mut result__).from_abi::<IGPMSOMCollection>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn GetWMIFilter<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrpath: Param0) -> ::windows::runtime::Result<IGPMWMIFilter> {
        let mut result__: <IGPMWMIFilter as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), bstrpath.into_param().abi(), &mut result__).from_abi::<IGPMWMIFilter>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn SearchWMIFilters<'a, Param0: ::windows::runtime::IntoParam<'a, IGPMSearchCriteria>>(&self, pigpmsearchcriteria: Param0) -> ::windows::runtime::Result<IGPMWMIFilterCollection> {
        let mut result__: <IGPMWMIFilterCollection as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), pigpmsearchcriteria.into_param().abi(), &mut result__).from_abi::<IGPMWMIFilterCollection>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IGPMDomain {
    type Vtable = IGPMDomain_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x6b21cc14_5a00_4f44_a738_feec8a94c7e3);
}
impl ::core::convert::From<IGPMDomain> for ::windows::runtime::IUnknown {
    fn from(value: IGPMDomain) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IGPMDomain> for ::windows::runtime::IUnknown {
    fn from(value: &IGPMDomain) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IGPMDomain {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IGPMDomain {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGPMDomain> for super::Com::IDispatch {
    fn from(value: IGPMDomain) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGPMDomain> for super::Com::IDispatch {
    fn from(value: &IGPMDomain) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for IGPMDomain {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for &IGPMDomain {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGPMDomain_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppnewgpo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrguid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppgpo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pigpmsearchcriteria: ::windows::runtime::RawPtr, ppigpmgpocollection: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pigpmbackup: ::windows::runtime::RawPtr, ldcflags: i32, pvargpmprogress: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, pvargpmcancel: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppigpmresult: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppsom: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pigpmsearchcriteria: ::windows::runtime::RawPtr, ppigpmsomcollection: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppwmifilter: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pigpmsearchcriteria: ::windows::runtime::RawPtr, ppigpmwmifiltercollection: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IGPMDomain2(pub ::windows::runtime::IUnknown);
impl IGPMDomain2 {
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_System_Com`*"]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::Com::ITypeInfo> {
        let mut result__: <super::Com::ITypeInfo as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), &mut result__).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(dispidmember),
            ::core::mem::transmute(riid),
            ::core::mem::transmute(lcid),
            ::core::mem::transmute(wflags),
            ::core::mem::transmute(pdispparams),
            ::core::mem::transmute(pvarresult),
            ::core::mem::transmute(pexcepinfo),
            ::core::mem::transmute(puargerr),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn DomainController(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn Domain(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn CreateGPO(&self) -> ::windows::runtime::Result<IGPMGPO> {
        let mut result__: <IGPMGPO as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IGPMGPO>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn GetGPO<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrguid: Param0) -> ::windows::runtime::Result<IGPMGPO> {
        let mut result__: <IGPMGPO as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), bstrguid.into_param().abi(), &mut result__).from_abi::<IGPMGPO>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn SearchGPOs<'a, Param0: ::windows::runtime::IntoParam<'a, IGPMSearchCriteria>>(&self, pigpmsearchcriteria: Param0) -> ::windows::runtime::Result<IGPMGPOCollection> {
        let mut result__: <IGPMGPOCollection as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), pigpmsearchcriteria.into_param().abi(), &mut result__).from_abi::<IGPMGPOCollection>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn RestoreGPO<'a, Param0: ::windows::runtime::IntoParam<'a, IGPMBackup>>(&self, pigpmbackup: Param0, ldcflags: i32, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), pigpmbackup.into_param().abi(), ::core::mem::transmute(ldcflags), ::core::mem::transmute(pvargpmprogress), ::core::mem::transmute(pvargpmcancel), ::core::mem::transmute(ppigpmresult)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn GetSOM<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrpath: Param0) -> ::windows::runtime::Result<IGPMSOM> {
        let mut result__: <IGPMSOM as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), bstrpath.into_param().abi(), &mut result__).from_abi::<IGPMSOM>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn SearchSOMs<'a, Param0: ::windows::runtime::IntoParam<'a, IGPMSearchCriteria>>(&self, pigpmsearchcriteria: Param0) -> ::windows::runtime::Result<IGPMSOMCollection> {
        let mut result__: <IGPMSOMCollection as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), pigpmsearchcriteria.into_param().abi(), &mut result__).from_abi::<IGPMSOMCollection>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn GetWMIFilter<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrpath: Param0) -> ::windows::runtime::Result<IGPMWMIFilter> {
        let mut result__: <IGPMWMIFilter as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), bstrpath.into_param().abi(), &mut result__).from_abi::<IGPMWMIFilter>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn SearchWMIFilters<'a, Param0: ::windows::runtime::IntoParam<'a, IGPMSearchCriteria>>(&self, pigpmsearchcriteria: Param0) -> ::windows::runtime::Result<IGPMWMIFilterCollection> {
        let mut result__: <IGPMWMIFilterCollection as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), pigpmsearchcriteria.into_param().abi(), &mut result__).from_abi::<IGPMWMIFilterCollection>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn CreateStarterGPO(&self) -> ::windows::runtime::Result<IGPMStarterGPO> {
        let mut result__: <IGPMStarterGPO as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IGPMStarterGPO>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn CreateGPOFromStarterGPO<'a, Param0: ::windows::runtime::IntoParam<'a, IGPMStarterGPO>>(&self, pgpotemplate: Param0) -> ::windows::runtime::Result<IGPMGPO> {
        let mut result__: <IGPMGPO as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), pgpotemplate.into_param().abi(), &mut result__).from_abi::<IGPMGPO>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn GetStarterGPO<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrguid: Param0) -> ::windows::runtime::Result<IGPMStarterGPO> {
        let mut result__: <IGPMStarterGPO as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), bstrguid.into_param().abi(), &mut result__).from_abi::<IGPMStarterGPO>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn SearchStarterGPOs<'a, Param0: ::windows::runtime::IntoParam<'a, IGPMSearchCriteria>>(&self, pigpmsearchcriteria: Param0) -> ::windows::runtime::Result<IGPMStarterGPOCollection> {
        let mut result__: <IGPMStarterGPOCollection as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self), pigpmsearchcriteria.into_param().abi(), &mut result__).from_abi::<IGPMStarterGPOCollection>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn LoadStarterGPO<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrloadfile: Param0, boverwrite: i16, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::core::mem::transmute_copy(self), bstrloadfile.into_param().abi(), ::core::mem::transmute(boverwrite), ::core::mem::transmute(pvargpmprogress), ::core::mem::transmute(pvargpmcancel), ::core::mem::transmute(ppigpmresult)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn RestoreStarterGPO<'a, Param0: ::windows::runtime::IntoParam<'a, IGPMStarterGPOBackup>>(&self, pigpmtmplbackup: Param0, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::core::mem::transmute_copy(self), pigpmtmplbackup.into_param().abi(), ::core::mem::transmute(pvargpmprogress), ::core::mem::transmute(pvargpmcancel), ::core::mem::transmute(ppigpmresult)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IGPMDomain2 {
    type Vtable = IGPMDomain2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x7ca6bb8b_f1eb_490a_938d_3c4e51c768e6);
}
impl ::core::convert::From<IGPMDomain2> for ::windows::runtime::IUnknown {
    fn from(value: IGPMDomain2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IGPMDomain2> for ::windows::runtime::IUnknown {
    fn from(value: &IGPMDomain2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IGPMDomain2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IGPMDomain2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IGPMDomain2> for IGPMDomain {
    fn from(value: IGPMDomain2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IGPMDomain2> for IGPMDomain {
    fn from(value: &IGPMDomain2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IGPMDomain> for IGPMDomain2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IGPMDomain> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IGPMDomain> for &IGPMDomain2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IGPMDomain> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGPMDomain2> for super::Com::IDispatch {
    fn from(value: IGPMDomain2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGPMDomain2> for super::Com::IDispatch {
    fn from(value: &IGPMDomain2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for IGPMDomain2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for &IGPMDomain2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGPMDomain2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppnewgpo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrguid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppgpo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pigpmsearchcriteria: ::windows::runtime::RawPtr, ppigpmgpocollection: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pigpmbackup: ::windows::runtime::RawPtr, ldcflags: i32, pvargpmprogress: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, pvargpmcancel: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppigpmresult: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppsom: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pigpmsearchcriteria: ::windows::runtime::RawPtr, ppigpmsomcollection: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppwmifilter: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pigpmsearchcriteria: ::windows::runtime::RawPtr, ppigpmwmifiltercollection: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppnewtemplate: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pgpotemplate: ::windows::runtime::RawPtr, ppnewgpo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrguid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pptemplate: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pigpmsearchcriteria: ::windows::runtime::RawPtr, ppigpmtemplatecollection: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrloadfile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, boverwrite: i16, pvargpmprogress: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, pvargpmcancel: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppigpmresult: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pigpmtmplbackup: ::windows::runtime::RawPtr, pvargpmprogress: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, pvargpmcancel: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppigpmresult: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IGPMDomain3(pub ::windows::runtime::IUnknown);
impl IGPMDomain3 {
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_System_Com`*"]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::Com::ITypeInfo> {
        let mut result__: <super::Com::ITypeInfo as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), &mut result__).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(dispidmember),
            ::core::mem::transmute(riid),
            ::core::mem::transmute(lcid),
            ::core::mem::transmute(wflags),
            ::core::mem::transmute(pdispparams),
            ::core::mem::transmute(pvarresult),
            ::core::mem::transmute(pexcepinfo),
            ::core::mem::transmute(puargerr),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn DomainController(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn Domain(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn CreateGPO(&self) -> ::windows::runtime::Result<IGPMGPO> {
        let mut result__: <IGPMGPO as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IGPMGPO>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn GetGPO<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrguid: Param0) -> ::windows::runtime::Result<IGPMGPO> {
        let mut result__: <IGPMGPO as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), bstrguid.into_param().abi(), &mut result__).from_abi::<IGPMGPO>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn SearchGPOs<'a, Param0: ::windows::runtime::IntoParam<'a, IGPMSearchCriteria>>(&self, pigpmsearchcriteria: Param0) -> ::windows::runtime::Result<IGPMGPOCollection> {
        let mut result__: <IGPMGPOCollection as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), pigpmsearchcriteria.into_param().abi(), &mut result__).from_abi::<IGPMGPOCollection>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn RestoreGPO<'a, Param0: ::windows::runtime::IntoParam<'a, IGPMBackup>>(&self, pigpmbackup: Param0, ldcflags: i32, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), pigpmbackup.into_param().abi(), ::core::mem::transmute(ldcflags), ::core::mem::transmute(pvargpmprogress), ::core::mem::transmute(pvargpmcancel), ::core::mem::transmute(ppigpmresult)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn GetSOM<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrpath: Param0) -> ::windows::runtime::Result<IGPMSOM> {
        let mut result__: <IGPMSOM as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), bstrpath.into_param().abi(), &mut result__).from_abi::<IGPMSOM>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn SearchSOMs<'a, Param0: ::windows::runtime::IntoParam<'a, IGPMSearchCriteria>>(&self, pigpmsearchcriteria: Param0) -> ::windows::runtime::Result<IGPMSOMCollection> {
        let mut result__: <IGPMSOMCollection as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), pigpmsearchcriteria.into_param().abi(), &mut result__).from_abi::<IGPMSOMCollection>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn GetWMIFilter<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrpath: Param0) -> ::windows::runtime::Result<IGPMWMIFilter> {
        let mut result__: <IGPMWMIFilter as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), bstrpath.into_param().abi(), &mut result__).from_abi::<IGPMWMIFilter>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn SearchWMIFilters<'a, Param0: ::windows::runtime::IntoParam<'a, IGPMSearchCriteria>>(&self, pigpmsearchcriteria: Param0) -> ::windows::runtime::Result<IGPMWMIFilterCollection> {
        let mut result__: <IGPMWMIFilterCollection as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), pigpmsearchcriteria.into_param().abi(), &mut result__).from_abi::<IGPMWMIFilterCollection>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn CreateStarterGPO(&self) -> ::windows::runtime::Result<IGPMStarterGPO> {
        let mut result__: <IGPMStarterGPO as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IGPMStarterGPO>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn CreateGPOFromStarterGPO<'a, Param0: ::windows::runtime::IntoParam<'a, IGPMStarterGPO>>(&self, pgpotemplate: Param0) -> ::windows::runtime::Result<IGPMGPO> {
        let mut result__: <IGPMGPO as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), pgpotemplate.into_param().abi(), &mut result__).from_abi::<IGPMGPO>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn GetStarterGPO<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrguid: Param0) -> ::windows::runtime::Result<IGPMStarterGPO> {
        let mut result__: <IGPMStarterGPO as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), bstrguid.into_param().abi(), &mut result__).from_abi::<IGPMStarterGPO>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn SearchStarterGPOs<'a, Param0: ::windows::runtime::IntoParam<'a, IGPMSearchCriteria>>(&self, pigpmsearchcriteria: Param0) -> ::windows::runtime::Result<IGPMStarterGPOCollection> {
        let mut result__: <IGPMStarterGPOCollection as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self), pigpmsearchcriteria.into_param().abi(), &mut result__).from_abi::<IGPMStarterGPOCollection>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn LoadStarterGPO<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrloadfile: Param0, boverwrite: i16, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::core::mem::transmute_copy(self), bstrloadfile.into_param().abi(), ::core::mem::transmute(boverwrite), ::core::mem::transmute(pvargpmprogress), ::core::mem::transmute(pvargpmcancel), ::core::mem::transmute(ppigpmresult)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn RestoreStarterGPO<'a, Param0: ::windows::runtime::IntoParam<'a, IGPMStarterGPOBackup>>(&self, pigpmtmplbackup: Param0, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::core::mem::transmute_copy(self), pigpmtmplbackup.into_param().abi(), ::core::mem::transmute(pvargpmprogress), ::core::mem::transmute(pvargpmcancel), ::core::mem::transmute(ppigpmresult)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn GenerateReport(&self, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(gpmreporttype), ::core::mem::transmute(pvargpmprogress), ::core::mem::transmute(pvargpmcancel), ::core::mem::transmute(ppigpmresult)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn InfrastructureDC(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn SetInfrastructureDC<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, newval: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(::core::mem::transmute_copy(self), newval.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn SetInfrastructureFlags(&self, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IGPMDomain3 {
    type Vtable = IGPMDomain3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x0077fdfe_88c7_4acf_a11d_d10a7c310a03);
}
impl ::core::convert::From<IGPMDomain3> for ::windows::runtime::IUnknown {
    fn from(value: IGPMDomain3) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IGPMDomain3> for ::windows::runtime::IUnknown {
    fn from(value: &IGPMDomain3) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IGPMDomain3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IGPMDomain3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IGPMDomain3> for IGPMDomain2 {
    fn from(value: IGPMDomain3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IGPMDomain3> for IGPMDomain2 {
    fn from(value: &IGPMDomain3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IGPMDomain2> for IGPMDomain3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IGPMDomain2> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IGPMDomain2> for &IGPMDomain3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IGPMDomain2> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IGPMDomain3> for IGPMDomain {
    fn from(value: IGPMDomain3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IGPMDomain3> for IGPMDomain {
    fn from(value: &IGPMDomain3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IGPMDomain> for IGPMDomain3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IGPMDomain> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IGPMDomain> for &IGPMDomain3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IGPMDomain> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGPMDomain3> for super::Com::IDispatch {
    fn from(value: IGPMDomain3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGPMDomain3> for super::Com::IDispatch {
    fn from(value: &IGPMDomain3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for IGPMDomain3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for &IGPMDomain3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGPMDomain3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppnewgpo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrguid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppgpo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pigpmsearchcriteria: ::windows::runtime::RawPtr, ppigpmgpocollection: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pigpmbackup: ::windows::runtime::RawPtr, ldcflags: i32, pvargpmprogress: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, pvargpmcancel: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppigpmresult: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppsom: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pigpmsearchcriteria: ::windows::runtime::RawPtr, ppigpmsomcollection: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppwmifilter: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pigpmsearchcriteria: ::windows::runtime::RawPtr, ppigpmwmifiltercollection: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppnewtemplate: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pgpotemplate: ::windows::runtime::RawPtr, ppnewgpo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrguid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pptemplate: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pigpmsearchcriteria: ::windows::runtime::RawPtr, ppigpmtemplatecollection: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrloadfile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, boverwrite: i16, pvargpmprogress: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, pvargpmcancel: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppigpmresult: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pigpmtmplbackup: ::windows::runtime::RawPtr, pvargpmprogress: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, pvargpmcancel: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppigpmresult: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, gpmreporttype: GPMReportType, pvargpmprogress: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, pvargpmcancel: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppigpmresult: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwflags: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IGPMGPO(pub ::windows::runtime::IUnknown);
impl IGPMGPO {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn DisplayName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn SetDisplayName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, newval: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), newval.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn Path(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn ID(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn DomainName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn CreationTime(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn ModificationTime(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn UserDSVersionNumber(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn ComputerDSVersionNumber(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn UserSysvolVersionNumber(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn ComputerSysvolVersionNumber(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn GetWMIFilter(&self) -> ::windows::runtime::Result<IGPMWMIFilter> {
        let mut result__: <IGPMWMIFilter as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IGPMWMIFilter>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn SetWMIFilter<'a, Param0: ::windows::runtime::IntoParam<'a, IGPMWMIFilter>>(&self, pigpmwmifilter: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), pigpmwmifilter.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn SetUserEnabled(&self, vbenabled: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(vbenabled)).ok()
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn SetComputerEnabled(&self, vbenabled: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(vbenabled)).ok()
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn IsUserEnabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn IsComputerEnabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn GetSecurityInfo(&self) -> ::windows::runtime::Result<IGPMSecurityInfo> {
        let mut result__: <IGPMSecurityInfo as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IGPMSecurityInfo>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn SetSecurityInfo<'a, Param0: ::windows::runtime::IntoParam<'a, IGPMSecurityInfo>>(&self, psecurityinfo: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(::core::mem::transmute_copy(self), psecurityinfo.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn Delete(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Backup<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrbackupdir: Param0, bstrcomment: Param1, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::core::mem::transmute_copy(self), bstrbackupdir.into_param().abi(), bstrcomment.into_param().abi(), ::core::mem::transmute(pvargpmprogress), ::core::mem::transmute(pvargpmcancel), ::core::mem::transmute(ppigpmresult)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Import<'a, Param1: ::windows::runtime::IntoParam<'a, IGPMBackup>>(&self, lflags: i32, pigpmbackup: Param1, pvarmigrationtable: *const super::Com::VARIANT, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), pigpmbackup.into_param().abi(), ::core::mem::transmute(pvarmigrationtable), ::core::mem::transmute(pvargpmprogress), ::core::mem::transmute(pvargpmcancel), ::core::mem::transmute(ppigpmresult)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn GenerateReport(&self, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).29)(::core::mem::transmute_copy(self), ::core::mem::transmute(gpmreporttype), ::core::mem::transmute(pvargpmprogress), ::core::mem::transmute(pvargpmcancel), ::core::mem::transmute(ppigpmresult)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn GenerateReportToFile<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, gpmreporttype: GPMReportType, bstrtargetfilepath: Param1) -> ::windows::runtime::Result<IGPMResult> {
        let mut result__: <IGPMResult as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).30)(::core::mem::transmute_copy(self), ::core::mem::transmute(gpmreporttype), bstrtargetfilepath.into_param().abi(), &mut result__).from_abi::<IGPMResult>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn CopyTo<'a, Param1: ::windows::runtime::IntoParam<'a, IGPMDomain>>(&self, lflags: i32, pigpmdomain: Param1, pvarnewdisplayname: *const super::Com::VARIANT, pvarmigrationtable: *const super::Com::VARIANT, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).31)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(lflags),
            pigpmdomain.into_param().abi(),
            ::core::mem::transmute(pvarnewdisplayname),
            ::core::mem::transmute(pvarmigrationtable),
            ::core::mem::transmute(pvargpmprogress),
            ::core::mem::transmute(pvargpmcancel),
            ::core::mem::transmute(ppigpmresult),
        )
        .ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_System_Com`*"]
    pub unsafe fn SetSecurityDescriptor<'a, Param1: ::windows::runtime::IntoParam<'a, super::Com::IDispatch>>(&self, lflags: i32, psd: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).32)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), psd.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_System_Com`*"]
    pub unsafe fn GetSecurityDescriptor(&self, lflags: i32) -> ::windows::runtime::Result<super::Com::IDispatch> {
        let mut result__: <super::Com::IDispatch as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).33)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), &mut result__).from_abi::<super::Com::IDispatch>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn IsACLConsistent(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).34)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn MakeACLConsistent(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).35)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IGPMGPO {
    type Vtable = IGPMGPO_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x58cc4352_1ca3_48e5_9864_1da4d6e0d60f);
}
impl ::core::convert::From<IGPMGPO> for ::windows::runtime::IUnknown {
    fn from(value: IGPMGPO) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IGPMGPO> for ::windows::runtime::IUnknown {
    fn from(value: &IGPMGPO) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IGPMGPO {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IGPMGPO {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGPMGPO> for super::Com::IDispatch {
    fn from(value: IGPMGPO) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGPMGPO> for super::Com::IDispatch {
    fn from(value: &IGPMGPO) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for IGPMGPO {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for &IGPMGPO {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGPMGPO_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdate: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdate: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppigpmwmifilter: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pigpmwmifilter: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, vbenabled: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, vbenabled: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvbenabled: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvbenabled: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppsecurityinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psecurityinfo: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrbackupdir: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrcomment: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvargpmprogress: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, pvargpmcancel: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppigpmresult: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lflags: i32, pigpmbackup: ::windows::runtime::RawPtr, pvarmigrationtable: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, pvargpmprogress: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, pvargpmcancel: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppigpmresult: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, gpmreporttype: GPMReportType, pvargpmprogress: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, pvargpmcancel: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppigpmresult: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, gpmreporttype: GPMReportType, bstrtargetfilepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppigpmresult: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lflags: i32, pigpmdomain: ::windows::runtime::RawPtr, pvarnewdisplayname: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, pvarmigrationtable: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, pvargpmprogress: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, pvargpmcancel: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppigpmresult: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lflags: i32, psd: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lflags: i32, ppsd: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvbconsistent: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IGPMGPO2(pub ::windows::runtime::IUnknown);
impl IGPMGPO2 {
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_System_Com`*"]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::Com::ITypeInfo> {
        let mut result__: <super::Com::ITypeInfo as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), &mut result__).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(dispidmember),
            ::core::mem::transmute(riid),
            ::core::mem::transmute(lcid),
            ::core::mem::transmute(wflags),
            ::core::mem::transmute(pdispparams),
            ::core::mem::transmute(pvarresult),
            ::core::mem::transmute(pexcepinfo),
            ::core::mem::transmute(puargerr),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn DisplayName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn SetDisplayName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, newval: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), newval.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn Path(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn ID(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn DomainName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn CreationTime(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn ModificationTime(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn UserDSVersionNumber(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn ComputerDSVersionNumber(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn UserSysvolVersionNumber(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn ComputerSysvolVersionNumber(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn GetWMIFilter(&self) -> ::windows::runtime::Result<IGPMWMIFilter> {
        let mut result__: <IGPMWMIFilter as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IGPMWMIFilter>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn SetWMIFilter<'a, Param0: ::windows::runtime::IntoParam<'a, IGPMWMIFilter>>(&self, pigpmwmifilter: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), pigpmwmifilter.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn SetUserEnabled(&self, vbenabled: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(vbenabled)).ok()
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn SetComputerEnabled(&self, vbenabled: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(vbenabled)).ok()
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn IsUserEnabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn IsComputerEnabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn GetSecurityInfo(&self) -> ::windows::runtime::Result<IGPMSecurityInfo> {
        let mut result__: <IGPMSecurityInfo as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IGPMSecurityInfo>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn SetSecurityInfo<'a, Param0: ::windows::runtime::IntoParam<'a, IGPMSecurityInfo>>(&self, psecurityinfo: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(::core::mem::transmute_copy(self), psecurityinfo.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn Delete(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Backup<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrbackupdir: Param0, bstrcomment: Param1, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::core::mem::transmute_copy(self), bstrbackupdir.into_param().abi(), bstrcomment.into_param().abi(), ::core::mem::transmute(pvargpmprogress), ::core::mem::transmute(pvargpmcancel), ::core::mem::transmute(ppigpmresult)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Import<'a, Param1: ::windows::runtime::IntoParam<'a, IGPMBackup>>(&self, lflags: i32, pigpmbackup: Param1, pvarmigrationtable: *const super::Com::VARIANT, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), pigpmbackup.into_param().abi(), ::core::mem::transmute(pvarmigrationtable), ::core::mem::transmute(pvargpmprogress), ::core::mem::transmute(pvargpmcancel), ::core::mem::transmute(ppigpmresult)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn GenerateReport(&self, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).29)(::core::mem::transmute_copy(self), ::core::mem::transmute(gpmreporttype), ::core::mem::transmute(pvargpmprogress), ::core::mem::transmute(pvargpmcancel), ::core::mem::transmute(ppigpmresult)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn GenerateReportToFile<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, gpmreporttype: GPMReportType, bstrtargetfilepath: Param1) -> ::windows::runtime::Result<IGPMResult> {
        let mut result__: <IGPMResult as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).30)(::core::mem::transmute_copy(self), ::core::mem::transmute(gpmreporttype), bstrtargetfilepath.into_param().abi(), &mut result__).from_abi::<IGPMResult>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn CopyTo<'a, Param1: ::windows::runtime::IntoParam<'a, IGPMDomain>>(&self, lflags: i32, pigpmdomain: Param1, pvarnewdisplayname: *const super::Com::VARIANT, pvarmigrationtable: *const super::Com::VARIANT, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).31)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(lflags),
            pigpmdomain.into_param().abi(),
            ::core::mem::transmute(pvarnewdisplayname),
            ::core::mem::transmute(pvarmigrationtable),
            ::core::mem::transmute(pvargpmprogress),
            ::core::mem::transmute(pvargpmcancel),
            ::core::mem::transmute(ppigpmresult),
        )
        .ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_System_Com`*"]
    pub unsafe fn SetSecurityDescriptor<'a, Param1: ::windows::runtime::IntoParam<'a, super::Com::IDispatch>>(&self, lflags: i32, psd: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).32)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), psd.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_System_Com`*"]
    pub unsafe fn GetSecurityDescriptor(&self, lflags: i32) -> ::windows::runtime::Result<super::Com::IDispatch> {
        let mut result__: <super::Com::IDispatch as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).33)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), &mut result__).from_abi::<super::Com::IDispatch>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn IsACLConsistent(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).34)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn MakeACLConsistent(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).35)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn Description(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).36)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn SetDescription<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, newval: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).37)(::core::mem::transmute_copy(self), newval.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IGPMGPO2 {
    type Vtable = IGPMGPO2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x8a66a210_b78b_4d99_88e2_c306a817c925);
}
impl ::core::convert::From<IGPMGPO2> for ::windows::runtime::IUnknown {
    fn from(value: IGPMGPO2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IGPMGPO2> for ::windows::runtime::IUnknown {
    fn from(value: &IGPMGPO2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IGPMGPO2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IGPMGPO2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IGPMGPO2> for IGPMGPO {
    fn from(value: IGPMGPO2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IGPMGPO2> for IGPMGPO {
    fn from(value: &IGPMGPO2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IGPMGPO> for IGPMGPO2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IGPMGPO> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IGPMGPO> for &IGPMGPO2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IGPMGPO> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGPMGPO2> for super::Com::IDispatch {
    fn from(value: IGPMGPO2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGPMGPO2> for super::Com::IDispatch {
    fn from(value: &IGPMGPO2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for IGPMGPO2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for &IGPMGPO2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGPMGPO2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdate: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdate: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppigpmwmifilter: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pigpmwmifilter: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, vbenabled: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, vbenabled: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvbenabled: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvbenabled: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppsecurityinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psecurityinfo: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrbackupdir: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrcomment: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvargpmprogress: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, pvargpmcancel: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppigpmresult: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lflags: i32, pigpmbackup: ::windows::runtime::RawPtr, pvarmigrationtable: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, pvargpmprogress: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, pvargpmcancel: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppigpmresult: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, gpmreporttype: GPMReportType, pvargpmprogress: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, pvargpmcancel: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppigpmresult: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, gpmreporttype: GPMReportType, bstrtargetfilepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppigpmresult: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lflags: i32, pigpmdomain: ::windows::runtime::RawPtr, pvarnewdisplayname: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, pvarmigrationtable: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, pvargpmprogress: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, pvargpmcancel: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppigpmresult: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lflags: i32, psd: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lflags: i32, ppsd: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvbconsistent: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IGPMGPO3(pub ::windows::runtime::IUnknown);
impl IGPMGPO3 {
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_System_Com`*"]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::Com::ITypeInfo> {
        let mut result__: <super::Com::ITypeInfo as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), &mut result__).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(dispidmember),
            ::core::mem::transmute(riid),
            ::core::mem::transmute(lcid),
            ::core::mem::transmute(wflags),
            ::core::mem::transmute(pdispparams),
            ::core::mem::transmute(pvarresult),
            ::core::mem::transmute(pexcepinfo),
            ::core::mem::transmute(puargerr),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn DisplayName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn SetDisplayName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, newval: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), newval.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn Path(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn ID(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn DomainName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn CreationTime(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn ModificationTime(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn UserDSVersionNumber(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn ComputerDSVersionNumber(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn UserSysvolVersionNumber(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn ComputerSysvolVersionNumber(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn GetWMIFilter(&self) -> ::windows::runtime::Result<IGPMWMIFilter> {
        let mut result__: <IGPMWMIFilter as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IGPMWMIFilter>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn SetWMIFilter<'a, Param0: ::windows::runtime::IntoParam<'a, IGPMWMIFilter>>(&self, pigpmwmifilter: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), pigpmwmifilter.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn SetUserEnabled(&self, vbenabled: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(vbenabled)).ok()
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn SetComputerEnabled(&self, vbenabled: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(vbenabled)).ok()
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn IsUserEnabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn IsComputerEnabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn GetSecurityInfo(&self) -> ::windows::runtime::Result<IGPMSecurityInfo> {
        let mut result__: <IGPMSecurityInfo as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IGPMSecurityInfo>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn SetSecurityInfo<'a, Param0: ::windows::runtime::IntoParam<'a, IGPMSecurityInfo>>(&self, psecurityinfo: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(::core::mem::transmute_copy(self), psecurityinfo.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn Delete(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Backup<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrbackupdir: Param0, bstrcomment: Param1, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::core::mem::transmute_copy(self), bstrbackupdir.into_param().abi(), bstrcomment.into_param().abi(), ::core::mem::transmute(pvargpmprogress), ::core::mem::transmute(pvargpmcancel), ::core::mem::transmute(ppigpmresult)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Import<'a, Param1: ::windows::runtime::IntoParam<'a, IGPMBackup>>(&self, lflags: i32, pigpmbackup: Param1, pvarmigrationtable: *const super::Com::VARIANT, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), pigpmbackup.into_param().abi(), ::core::mem::transmute(pvarmigrationtable), ::core::mem::transmute(pvargpmprogress), ::core::mem::transmute(pvargpmcancel), ::core::mem::transmute(ppigpmresult)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn GenerateReport(&self, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).29)(::core::mem::transmute_copy(self), ::core::mem::transmute(gpmreporttype), ::core::mem::transmute(pvargpmprogress), ::core::mem::transmute(pvargpmcancel), ::core::mem::transmute(ppigpmresult)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn GenerateReportToFile<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, gpmreporttype: GPMReportType, bstrtargetfilepath: Param1) -> ::windows::runtime::Result<IGPMResult> {
        let mut result__: <IGPMResult as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).30)(::core::mem::transmute_copy(self), ::core::mem::transmute(gpmreporttype), bstrtargetfilepath.into_param().abi(), &mut result__).from_abi::<IGPMResult>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn CopyTo<'a, Param1: ::windows::runtime::IntoParam<'a, IGPMDomain>>(&self, lflags: i32, pigpmdomain: Param1, pvarnewdisplayname: *const super::Com::VARIANT, pvarmigrationtable: *const super::Com::VARIANT, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).31)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(lflags),
            pigpmdomain.into_param().abi(),
            ::core::mem::transmute(pvarnewdisplayname),
            ::core::mem::transmute(pvarmigrationtable),
            ::core::mem::transmute(pvargpmprogress),
            ::core::mem::transmute(pvargpmcancel),
            ::core::mem::transmute(ppigpmresult),
        )
        .ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_System_Com`*"]
    pub unsafe fn SetSecurityDescriptor<'a, Param1: ::windows::runtime::IntoParam<'a, super::Com::IDispatch>>(&self, lflags: i32, psd: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).32)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), psd.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_System_Com`*"]
    pub unsafe fn GetSecurityDescriptor(&self, lflags: i32) -> ::windows::runtime::Result<super::Com::IDispatch> {
        let mut result__: <super::Com::IDispatch as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).33)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), &mut result__).from_abi::<super::Com::IDispatch>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn IsACLConsistent(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).34)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn MakeACLConsistent(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).35)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn Description(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).36)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn SetDescription<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, newval: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).37)(::core::mem::transmute_copy(self), newval.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn InfrastructureDC(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).38)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn SetInfrastructureDC<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, newval: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).39)(::core::mem::transmute_copy(self), newval.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn SetInfrastructureFlags(&self, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).40)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IGPMGPO3 {
    type Vtable = IGPMGPO3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x7cf123a1_f94a_4112_bfae_6aa1db9cb248);
}
impl ::core::convert::From<IGPMGPO3> for ::windows::runtime::IUnknown {
    fn from(value: IGPMGPO3) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IGPMGPO3> for ::windows::runtime::IUnknown {
    fn from(value: &IGPMGPO3) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IGPMGPO3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IGPMGPO3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IGPMGPO3> for IGPMGPO2 {
    fn from(value: IGPMGPO3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IGPMGPO3> for IGPMGPO2 {
    fn from(value: &IGPMGPO3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IGPMGPO2> for IGPMGPO3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IGPMGPO2> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IGPMGPO2> for &IGPMGPO3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IGPMGPO2> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IGPMGPO3> for IGPMGPO {
    fn from(value: IGPMGPO3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IGPMGPO3> for IGPMGPO {
    fn from(value: &IGPMGPO3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IGPMGPO> for IGPMGPO3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IGPMGPO> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IGPMGPO> for &IGPMGPO3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IGPMGPO> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGPMGPO3> for super::Com::IDispatch {
    fn from(value: IGPMGPO3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGPMGPO3> for super::Com::IDispatch {
    fn from(value: &IGPMGPO3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for IGPMGPO3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for &IGPMGPO3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGPMGPO3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdate: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdate: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppigpmwmifilter: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pigpmwmifilter: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, vbenabled: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, vbenabled: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvbenabled: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvbenabled: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppsecurityinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psecurityinfo: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrbackupdir: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrcomment: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvargpmprogress: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, pvargpmcancel: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppigpmresult: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lflags: i32, pigpmbackup: ::windows::runtime::RawPtr, pvarmigrationtable: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, pvargpmprogress: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, pvargpmcancel: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppigpmresult: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, gpmreporttype: GPMReportType, pvargpmprogress: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, pvargpmcancel: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppigpmresult: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, gpmreporttype: GPMReportType, bstrtargetfilepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppigpmresult: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lflags: i32, pigpmdomain: ::windows::runtime::RawPtr, pvarnewdisplayname: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, pvarmigrationtable: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, pvargpmprogress: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, pvargpmcancel: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppigpmresult: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lflags: i32, psd: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lflags: i32, ppsd: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvbconsistent: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwflags: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IGPMGPOCollection(pub ::windows::runtime::IUnknown);
impl IGPMGPOCollection {
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn Count(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Item(&self, lindex: i32) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(lindex), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(feature = "Win32_System_Ole")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_System_Ole`*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::runtime::Result<super::Ole::IEnumVARIANT> {
        let mut result__: <super::Ole::IEnumVARIANT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Ole::IEnumVARIANT>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IGPMGPOCollection {
    type Vtable = IGPMGPOCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xf0f0d5cf_70ca_4c39_9e29_b642f8726c01);
}
impl ::core::convert::From<IGPMGPOCollection> for ::windows::runtime::IUnknown {
    fn from(value: IGPMGPOCollection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IGPMGPOCollection> for ::windows::runtime::IUnknown {
    fn from(value: &IGPMGPOCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IGPMGPOCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IGPMGPOCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGPMGPOCollection> for super::Com::IDispatch {
    fn from(value: IGPMGPOCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGPMGPOCollection> for super::Com::IDispatch {
    fn from(value: &IGPMGPOCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for IGPMGPOCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for &IGPMGPOCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGPMGPOCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lindex: i32, pval: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_System_Ole")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppigpmgpos: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))] usize,
);
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IGPMGPOLink(pub ::windows::runtime::IUnknown);
impl IGPMGPOLink {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn GPOID(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn GPODomain(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn Enabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn SetEnabled(&self, newval: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn Enforced(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn SetEnforced(&self, newval: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn SOMLinkOrder(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn SOM(&self) -> ::windows::runtime::Result<IGPMSOM> {
        let mut result__: <IGPMSOM as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IGPMSOM>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn Delete(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IGPMGPOLink {
    type Vtable = IGPMGPOLink_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x434b99bd_5de7_478a_809c_c251721df70c);
}
impl ::core::convert::From<IGPMGPOLink> for ::windows::runtime::IUnknown {
    fn from(value: IGPMGPOLink) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IGPMGPOLink> for ::windows::runtime::IUnknown {
    fn from(value: &IGPMGPOLink) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IGPMGPOLink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IGPMGPOLink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGPMGPOLink> for super::Com::IDispatch {
    fn from(value: IGPMGPOLink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGPMGPOLink> for super::Com::IDispatch {
    fn from(value: &IGPMGPOLink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for IGPMGPOLink {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for &IGPMGPOLink {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGPMGPOLink_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lval: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppigpmsom: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IGPMGPOLinksCollection(pub ::windows::runtime::IUnknown);
impl IGPMGPOLinksCollection {
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn Count(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Item(&self, lindex: i32) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(lindex), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(feature = "Win32_System_Ole")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_System_Ole`*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::runtime::Result<super::Ole::IEnumVARIANT> {
        let mut result__: <super::Ole::IEnumVARIANT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Ole::IEnumVARIANT>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IGPMGPOLinksCollection {
    type Vtable = IGPMGPOLinksCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x189d7b68_16bd_4d0d_a2ec_2e6aa2288c7f);
}
impl ::core::convert::From<IGPMGPOLinksCollection> for ::windows::runtime::IUnknown {
    fn from(value: IGPMGPOLinksCollection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IGPMGPOLinksCollection> for ::windows::runtime::IUnknown {
    fn from(value: &IGPMGPOLinksCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IGPMGPOLinksCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IGPMGPOLinksCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGPMGPOLinksCollection> for super::Com::IDispatch {
    fn from(value: IGPMGPOLinksCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGPMGPOLinksCollection> for super::Com::IDispatch {
    fn from(value: &IGPMGPOLinksCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for IGPMGPOLinksCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for &IGPMGPOLinksCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGPMGPOLinksCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lindex: i32, pval: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_System_Ole")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppigpmlinks: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))] usize,
);
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IGPMMapEntry(pub ::windows::runtime::IUnknown);
impl IGPMMapEntry {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn Source(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn Destination(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn DestinationOption(&self) -> ::windows::runtime::Result<GPMDestinationOption> {
        let mut result__: <GPMDestinationOption as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMDestinationOption>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn EntryType(&self) -> ::windows::runtime::Result<GPMEntryType> {
        let mut result__: <GPMEntryType as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMEntryType>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IGPMMapEntry {
    type Vtable = IGPMMapEntry_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x8e79ad06_2381_4444_be4c_ff693e6e6f2b);
}
impl ::core::convert::From<IGPMMapEntry> for ::windows::runtime::IUnknown {
    fn from(value: IGPMMapEntry) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IGPMMapEntry> for ::windows::runtime::IUnknown {
    fn from(value: &IGPMMapEntry) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IGPMMapEntry {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IGPMMapEntry {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGPMMapEntry> for super::Com::IDispatch {
    fn from(value: IGPMMapEntry) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGPMMapEntry> for super::Com::IDispatch {
    fn from(value: &IGPMMapEntry) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for IGPMMapEntry {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for &IGPMMapEntry {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGPMMapEntry_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrsource: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrdestination: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pgpmdestoption: *mut GPMDestinationOption) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pgpmentrytype: *mut GPMEntryType) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IGPMMapEntryCollection(pub ::windows::runtime::IUnknown);
impl IGPMMapEntryCollection {
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn Count(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Item(&self, lindex: i32) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(lindex), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(feature = "Win32_System_Ole")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_System_Ole`*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::runtime::Result<super::Ole::IEnumVARIANT> {
        let mut result__: <super::Ole::IEnumVARIANT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Ole::IEnumVARIANT>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IGPMMapEntryCollection {
    type Vtable = IGPMMapEntryCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xbb0bf49b_e53f_443f_b807_8be22bfb6d42);
}
impl ::core::convert::From<IGPMMapEntryCollection> for ::windows::runtime::IUnknown {
    fn from(value: IGPMMapEntryCollection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IGPMMapEntryCollection> for ::windows::runtime::IUnknown {
    fn from(value: &IGPMMapEntryCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IGPMMapEntryCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IGPMMapEntryCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGPMMapEntryCollection> for super::Com::IDispatch {
    fn from(value: IGPMMapEntryCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGPMMapEntryCollection> for super::Com::IDispatch {
    fn from(value: &IGPMMapEntryCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for IGPMMapEntryCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for &IGPMMapEntryCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGPMMapEntryCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lindex: i32, pval: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_System_Ole")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))] usize,
);
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IGPMMigrationTable(pub ::windows::runtime::IUnknown);
impl IGPMMigrationTable {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn Save<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrmigrationtablepath: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), bstrmigrationtablepath.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Add<'a, Param1: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>>(&self, lflags: i32, var: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), var.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn AddEntry<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrsource: Param0, gpmentrytype: GPMEntryType, pvardestination: *const super::Com::VARIANT) -> ::windows::runtime::Result<IGPMMapEntry> {
        let mut result__: <IGPMMapEntry as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), bstrsource.into_param().abi(), ::core::mem::transmute(gpmentrytype), ::core::mem::transmute(pvardestination), &mut result__).from_abi::<IGPMMapEntry>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn GetEntry<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrsource: Param0) -> ::windows::runtime::Result<IGPMMapEntry> {
        let mut result__: <IGPMMapEntry as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), bstrsource.into_param().abi(), &mut result__).from_abi::<IGPMMapEntry>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn DeleteEntry<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrsource: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), bstrsource.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn UpdateDestination<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrsource: Param0, pvardestination: *const super::Com::VARIANT) -> ::windows::runtime::Result<IGPMMapEntry> {
        let mut result__: <IGPMMapEntry as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), bstrsource.into_param().abi(), ::core::mem::transmute(pvardestination), &mut result__).from_abi::<IGPMMapEntry>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn Validate(&self) -> ::windows::runtime::Result<IGPMResult> {
        let mut result__: <IGPMResult as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IGPMResult>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn GetEntries(&self) -> ::windows::runtime::Result<IGPMMapEntryCollection> {
        let mut result__: <IGPMMapEntryCollection as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IGPMMapEntryCollection>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IGPMMigrationTable {
    type Vtable = IGPMMigrationTable_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x48f823b1_efaf_470b_b6ed_40d14ee1a4ec);
}
impl ::core::convert::From<IGPMMigrationTable> for ::windows::runtime::IUnknown {
    fn from(value: IGPMMigrationTable) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IGPMMigrationTable> for ::windows::runtime::IUnknown {
    fn from(value: &IGPMMigrationTable) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IGPMMigrationTable {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IGPMMigrationTable {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGPMMigrationTable> for super::Com::IDispatch {
    fn from(value: IGPMMigrationTable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGPMMigrationTable> for super::Com::IDispatch {
    fn from(value: &IGPMMigrationTable) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for IGPMMigrationTable {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for &IGPMMigrationTable {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGPMMigrationTable_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrmigrationtablepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lflags: i32, var: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrsource: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, gpmentrytype: GPMEntryType, pvardestination: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppentry: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrsource: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppentry: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrsource: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrsource: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvardestination: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppentry: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppresult: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppentries: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IGPMPermission(pub ::windows::runtime::IUnknown);
impl IGPMPermission {
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn Inherited(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn Inheritable(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn Denied(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn Permission(&self) -> ::windows::runtime::Result<GPMPermissionType> {
        let mut result__: <GPMPermissionType as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMPermissionType>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn Trustee(&self) -> ::windows::runtime::Result<IGPMTrustee> {
        let mut result__: <IGPMTrustee as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IGPMTrustee>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IGPMPermission {
    type Vtable = IGPMPermission_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x35ebca40_e1a1_4a02_8905_d79416fb464a);
}
impl ::core::convert::From<IGPMPermission> for ::windows::runtime::IUnknown {
    fn from(value: IGPMPermission) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IGPMPermission> for ::windows::runtime::IUnknown {
    fn from(value: &IGPMPermission) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IGPMPermission {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IGPMPermission {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGPMPermission> for super::Com::IDispatch {
    fn from(value: IGPMPermission) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGPMPermission> for super::Com::IDispatch {
    fn from(value: &IGPMPermission) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for IGPMPermission {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for &IGPMPermission {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGPMPermission_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMPermissionType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppigpmtrustee: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IGPMRSOP(pub ::windows::runtime::IUnknown);
impl IGPMRSOP {
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn Mode(&self) -> ::windows::runtime::Result<GPMRSOPMode> {
        let mut result__: <GPMRSOPMode as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMRSOPMode>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn Namespace(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn SetLoggingComputer<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrval: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), bstrval.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn LoggingComputer(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn SetLoggingUser<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrval: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), bstrval.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn LoggingUser(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn SetLoggingFlags(&self, lval: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(lval)).ok()
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn LoggingFlags(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn SetPlanningFlags(&self, lval: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(lval)).ok()
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn PlanningFlags(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn SetPlanningDomainController<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrval: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), bstrval.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn PlanningDomainController(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn SetPlanningSiteName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrval: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), bstrval.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn PlanningSiteName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn SetPlanningUser<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrval: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::core::mem::transmute_copy(self), bstrval.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn PlanningUser(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn SetPlanningUserSOM<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrval: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::core::mem::transmute_copy(self), bstrval.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn PlanningUserSOM(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn SetPlanningUserWMIFilters<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>>(&self, varval: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(::core::mem::transmute_copy(self), varval.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn PlanningUserWMIFilters(&self) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).26)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn SetPlanningUserSecurityGroups<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>>(&self, varval: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::core::mem::transmute_copy(self), varval.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn PlanningUserSecurityGroups(&self) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).28)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn SetPlanningComputer<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrval: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).29)(::core::mem::transmute_copy(self), bstrval.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn PlanningComputer(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).30)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn SetPlanningComputerSOM<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrval: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).31)(::core::mem::transmute_copy(self), bstrval.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn PlanningComputerSOM(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).32)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn SetPlanningComputerWMIFilters<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>>(&self, varval: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).33)(::core::mem::transmute_copy(self), varval.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn PlanningComputerWMIFilters(&self) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).34)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn SetPlanningComputerSecurityGroups<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>>(&self, varval: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).35)(::core::mem::transmute_copy(self), varval.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn PlanningComputerSecurityGroups(&self) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).36)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn LoggingEnumerateUsers(&self) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).37)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn CreateQueryResults(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).38)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn ReleaseQueryResults(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).39)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn GenerateReport(&self, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).40)(::core::mem::transmute_copy(self), ::core::mem::transmute(gpmreporttype), ::core::mem::transmute(pvargpmprogress), ::core::mem::transmute(pvargpmcancel), ::core::mem::transmute(ppigpmresult)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn GenerateReportToFile<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, gpmreporttype: GPMReportType, bstrtargetfilepath: Param1) -> ::windows::runtime::Result<IGPMResult> {
        let mut result__: <IGPMResult as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).41)(::core::mem::transmute_copy(self), ::core::mem::transmute(gpmreporttype), bstrtargetfilepath.into_param().abi(), &mut result__).from_abi::<IGPMResult>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IGPMRSOP {
    type Vtable = IGPMRSOP_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x49ed785a_3237_4ff2_b1f0_fdf5a8d5a1ee);
}
impl ::core::convert::From<IGPMRSOP> for ::windows::runtime::IUnknown {
    fn from(value: IGPMRSOP) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IGPMRSOP> for ::windows::runtime::IUnknown {
    fn from(value: &IGPMRSOP) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IGPMRSOP {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IGPMRSOP {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGPMRSOP> for super::Com::IDispatch {
    fn from(value: IGPMRSOP) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGPMRSOP> for super::Com::IDispatch {
    fn from(value: &IGPMRSOP) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for IGPMRSOP {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for &IGPMRSOP {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGPMRSOP_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMRSOPMode) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lval: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lval: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lval: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lval: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, varval: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, varval: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, varval: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, varval: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, varval: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, varval: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, varval: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, varval: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, varval: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, gpmreporttype: GPMReportType, pvargpmprogress: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, pvargpmcancel: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppigpmresult: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, gpmreporttype: GPMReportType, bstrtargetfilepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppigpmresult: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IGPMResult(pub ::windows::runtime::IUnknown);
impl IGPMResult {
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn Status(&self) -> ::windows::runtime::Result<IGPMStatusMsgCollection> {
        let mut result__: <IGPMStatusMsgCollection as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IGPMStatusMsgCollection>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Result(&self) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn OverallStatus(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IGPMResult {
    type Vtable = IGPMResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x86dff7e9_f76f_42ab_9570_cebc6be8a52d);
}
impl ::core::convert::From<IGPMResult> for ::windows::runtime::IUnknown {
    fn from(value: IGPMResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IGPMResult> for ::windows::runtime::IUnknown {
    fn from(value: &IGPMResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IGPMResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IGPMResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGPMResult> for super::Com::IDispatch {
    fn from(value: IGPMResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGPMResult> for super::Com::IDispatch {
    fn from(value: &IGPMResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for IGPMResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for &IGPMResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGPMResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppigpmstatusmsgcollection: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IGPMSOM(pub ::windows::runtime::IUnknown);
impl IGPMSOM {
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn GPOInheritanceBlocked(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn SetGPOInheritanceBlocked(&self, newval: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn Name(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn Path(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn CreateGPOLink<'a, Param1: ::windows::runtime::IntoParam<'a, IGPMGPO>>(&self, llinkpos: i32, pgpo: Param1) -> ::windows::runtime::Result<IGPMGPOLink> {
        let mut result__: <IGPMGPOLink as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(llinkpos), pgpo.into_param().abi(), &mut result__).from_abi::<IGPMGPOLink>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn Type(&self) -> ::windows::runtime::Result<GPMSOMType> {
        let mut result__: <GPMSOMType as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMSOMType>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn GetGPOLinks(&self) -> ::windows::runtime::Result<IGPMGPOLinksCollection> {
        let mut result__: <IGPMGPOLinksCollection as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IGPMGPOLinksCollection>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn GetInheritedGPOLinks(&self) -> ::windows::runtime::Result<IGPMGPOLinksCollection> {
        let mut result__: <IGPMGPOLinksCollection as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IGPMGPOLinksCollection>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn GetSecurityInfo(&self) -> ::windows::runtime::Result<IGPMSecurityInfo> {
        let mut result__: <IGPMSecurityInfo as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IGPMSecurityInfo>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn SetSecurityInfo<'a, Param0: ::windows::runtime::IntoParam<'a, IGPMSecurityInfo>>(&self, psecurityinfo: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), psecurityinfo.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IGPMSOM {
    type Vtable = IGPMSOM_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xc0a7f09e_05a1_4f0c_8158_9e5c33684f6b);
}
impl ::core::convert::From<IGPMSOM> for ::windows::runtime::IUnknown {
    fn from(value: IGPMSOM) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IGPMSOM> for ::windows::runtime::IUnknown {
    fn from(value: &IGPMSOM) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IGPMSOM {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IGPMSOM {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGPMSOM> for super::Com::IDispatch {
    fn from(value: IGPMSOM) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGPMSOM> for super::Com::IDispatch {
    fn from(value: &IGPMSOM) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for IGPMSOM {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for &IGPMSOM {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGPMSOM_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, llinkpos: i32, pgpo: ::windows::runtime::RawPtr, ppnewgpolink: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMSOMType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppgpolinks: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppgpolinks: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppsecurityinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psecurityinfo: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IGPMSOMCollection(pub ::windows::runtime::IUnknown);
impl IGPMSOMCollection {
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn Count(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Item(&self, lindex: i32) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(lindex), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(feature = "Win32_System_Ole")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_System_Ole`*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::runtime::Result<super::Ole::IEnumVARIANT> {
        let mut result__: <super::Ole::IEnumVARIANT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Ole::IEnumVARIANT>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IGPMSOMCollection {
    type Vtable = IGPMSOMCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xadc1688e_00e4_4495_abba_bed200df0cab);
}
impl ::core::convert::From<IGPMSOMCollection> for ::windows::runtime::IUnknown {
    fn from(value: IGPMSOMCollection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IGPMSOMCollection> for ::windows::runtime::IUnknown {
    fn from(value: &IGPMSOMCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IGPMSOMCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IGPMSOMCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGPMSOMCollection> for super::Com::IDispatch {
    fn from(value: IGPMSOMCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGPMSOMCollection> for super::Com::IDispatch {
    fn from(value: &IGPMSOMCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for IGPMSOMCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for &IGPMSOMCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGPMSOMCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lindex: i32, pval: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_System_Ole")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppigpmsom: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))] usize,
);
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IGPMSearchCriteria(pub ::windows::runtime::IUnknown);
impl IGPMSearchCriteria {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Add<'a, Param2: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>>(&self, searchproperty: GPMSearchProperty, searchoperation: GPMSearchOperation, varvalue: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(searchproperty), ::core::mem::transmute(searchoperation), varvalue.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IGPMSearchCriteria {
    type Vtable = IGPMSearchCriteria_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xd6f11c42_829b_48d4_83f5_3615b67dfc22);
}
impl ::core::convert::From<IGPMSearchCriteria> for ::windows::runtime::IUnknown {
    fn from(value: IGPMSearchCriteria) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IGPMSearchCriteria> for ::windows::runtime::IUnknown {
    fn from(value: &IGPMSearchCriteria) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IGPMSearchCriteria {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IGPMSearchCriteria {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGPMSearchCriteria> for super::Com::IDispatch {
    fn from(value: IGPMSearchCriteria) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGPMSearchCriteria> for super::Com::IDispatch {
    fn from(value: &IGPMSearchCriteria) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for IGPMSearchCriteria {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for &IGPMSearchCriteria {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGPMSearchCriteria_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, searchproperty: GPMSearchProperty, searchoperation: GPMSearchOperation, varvalue: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IGPMSecurityInfo(pub ::windows::runtime::IUnknown);
impl IGPMSecurityInfo {
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn Count(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Item(&self, lindex: i32) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(lindex), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(feature = "Win32_System_Ole")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_System_Ole`*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::runtime::Result<super::Ole::IEnumVARIANT> {
        let mut result__: <super::Ole::IEnumVARIANT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Ole::IEnumVARIANT>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn Add<'a, Param0: ::windows::runtime::IntoParam<'a, IGPMPermission>>(&self, pperm: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), pperm.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn Remove<'a, Param0: ::windows::runtime::IntoParam<'a, IGPMPermission>>(&self, pperm: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), pperm.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn RemoveTrustee<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrtrustee: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), bstrtrustee.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IGPMSecurityInfo {
    type Vtable = IGPMSecurityInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xb6c31ed4_1c93_4d3e_ae84_eb6d61161b60);
}
impl ::core::convert::From<IGPMSecurityInfo> for ::windows::runtime::IUnknown {
    fn from(value: IGPMSecurityInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IGPMSecurityInfo> for ::windows::runtime::IUnknown {
    fn from(value: &IGPMSecurityInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IGPMSecurityInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IGPMSecurityInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGPMSecurityInfo> for super::Com::IDispatch {
    fn from(value: IGPMSecurityInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGPMSecurityInfo> for super::Com::IDispatch {
    fn from(value: &IGPMSecurityInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for IGPMSecurityInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for &IGPMSecurityInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGPMSecurityInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lindex: i32, pval: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_System_Ole")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pperm: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pperm: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrtrustee: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IGPMSitesContainer(pub ::windows::runtime::IUnknown);
impl IGPMSitesContainer {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn DomainController(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn Domain(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn Forest(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn GetSite<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrsitename: Param0) -> ::windows::runtime::Result<IGPMSOM> {
        let mut result__: <IGPMSOM as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), bstrsitename.into_param().abi(), &mut result__).from_abi::<IGPMSOM>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn SearchSites<'a, Param0: ::windows::runtime::IntoParam<'a, IGPMSearchCriteria>>(&self, pigpmsearchcriteria: Param0) -> ::windows::runtime::Result<IGPMSOMCollection> {
        let mut result__: <IGPMSOMCollection as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), pigpmsearchcriteria.into_param().abi(), &mut result__).from_abi::<IGPMSOMCollection>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IGPMSitesContainer {
    type Vtable = IGPMSitesContainer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x4725a899_2782_4d27_a6bb_d499246ffd72);
}
impl ::core::convert::From<IGPMSitesContainer> for ::windows::runtime::IUnknown {
    fn from(value: IGPMSitesContainer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IGPMSitesContainer> for ::windows::runtime::IUnknown {
    fn from(value: &IGPMSitesContainer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IGPMSitesContainer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IGPMSitesContainer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGPMSitesContainer> for super::Com::IDispatch {
    fn from(value: IGPMSitesContainer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGPMSitesContainer> for super::Com::IDispatch {
    fn from(value: &IGPMSitesContainer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for IGPMSitesContainer {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for &IGPMSitesContainer {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGPMSitesContainer_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrsitename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppsom: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pigpmsearchcriteria: ::windows::runtime::RawPtr, ppigpmsomcollection: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IGPMStarterGPO(pub ::windows::runtime::IUnknown);
impl IGPMStarterGPO {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn DisplayName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn SetDisplayName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, newval: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), newval.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn Description(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn SetDescription<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, newval: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), newval.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn Author(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn Product(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn CreationTime(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn ID(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn ModifiedTime(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn Type(&self) -> ::windows::runtime::Result<GPMStarterGPOType> {
        let mut result__: <GPMStarterGPOType as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMStarterGPOType>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn ComputerVersion(&self) -> ::windows::runtime::Result<u16> {
        let mut result__: <u16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u16>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn UserVersion(&self) -> ::windows::runtime::Result<u16> {
        let mut result__: <u16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn StarterGPOVersion(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn Delete(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Save<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(
        &self,
        bstrsavefile: Param0,
        boverwrite: i16,
        bsaveassystem: i16,
        bstrlanguage: *const super::Com::VARIANT,
        bstrauthor: *const super::Com::VARIANT,
        bstrproduct: *const super::Com::VARIANT,
        bstruniqueid: *const super::Com::VARIANT,
        bstrversion: *const super::Com::VARIANT,
        pvargpmprogress: *const super::Com::VARIANT,
        pvargpmcancel: *mut super::Com::VARIANT,
        ppigpmresult: *mut ::core::option::Option<IGPMResult>,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(
            ::core::mem::transmute_copy(self),
            bstrsavefile.into_param().abi(),
            ::core::mem::transmute(boverwrite),
            ::core::mem::transmute(bsaveassystem),
            ::core::mem::transmute(bstrlanguage),
            ::core::mem::transmute(bstrauthor),
            ::core::mem::transmute(bstrproduct),
            ::core::mem::transmute(bstruniqueid),
            ::core::mem::transmute(bstrversion),
            ::core::mem::transmute(pvargpmprogress),
            ::core::mem::transmute(pvargpmcancel),
            ::core::mem::transmute(ppigpmresult),
        )
        .ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Backup<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrbackupdir: Param0, bstrcomment: Param1, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::core::mem::transmute_copy(self), bstrbackupdir.into_param().abi(), bstrcomment.into_param().abi(), ::core::mem::transmute(pvargpmprogress), ::core::mem::transmute(pvargpmcancel), ::core::mem::transmute(ppigpmresult)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn CopyTo(&self, pvarnewdisplayname: *const super::Com::VARIANT, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *const super::Com::VARIANT) -> ::windows::runtime::Result<IGPMResult> {
        let mut result__: <IGPMResult as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvarnewdisplayname), ::core::mem::transmute(pvargpmprogress), ::core::mem::transmute(pvargpmcancel), &mut result__).from_abi::<IGPMResult>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn GenerateReport(&self, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *const super::Com::VARIANT) -> ::windows::runtime::Result<IGPMResult> {
        let mut result__: <IGPMResult as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(gpmreporttype), ::core::mem::transmute(pvargpmprogress), ::core::mem::transmute(pvargpmcancel), &mut result__).from_abi::<IGPMResult>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn GenerateReportToFile<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, gpmreporttype: GPMReportType, bstrtargetfilepath: Param1) -> ::windows::runtime::Result<IGPMResult> {
        let mut result__: <IGPMResult as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(gpmreporttype), bstrtargetfilepath.into_param().abi(), &mut result__).from_abi::<IGPMResult>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn GetSecurityInfo(&self) -> ::windows::runtime::Result<IGPMSecurityInfo> {
        let mut result__: <IGPMSecurityInfo as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).26)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IGPMSecurityInfo>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn SetSecurityInfo<'a, Param0: ::windows::runtime::IntoParam<'a, IGPMSecurityInfo>>(&self, psecurityinfo: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::core::mem::transmute_copy(self), psecurityinfo.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IGPMStarterGPO {
    type Vtable = IGPMStarterGPO_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xdfc3f61b_8880_4490_9337_d29c7ba8c2f0);
}
impl ::core::convert::From<IGPMStarterGPO> for ::windows::runtime::IUnknown {
    fn from(value: IGPMStarterGPO) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IGPMStarterGPO> for ::windows::runtime::IUnknown {
    fn from(value: &IGPMStarterGPO) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IGPMStarterGPO {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IGPMStarterGPO {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGPMStarterGPO> for super::Com::IDispatch {
    fn from(value: IGPMStarterGPO) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGPMStarterGPO> for super::Com::IDispatch {
    fn from(value: &IGPMStarterGPO) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for IGPMStarterGPO {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for &IGPMStarterGPO {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGPMStarterGPO_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut f64) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut GPMStarterGPOType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut u16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bstrsavefile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        boverwrite: i16,
        bsaveassystem: i16,
        bstrlanguage: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>,
        bstrauthor: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>,
        bstrproduct: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>,
        bstruniqueid: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>,
        bstrversion: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>,
        pvargpmprogress: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>,
        pvargpmcancel: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>,
        ppigpmresult: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrbackupdir: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrcomment: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvargpmprogress: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, pvargpmcancel: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppigpmresult: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvarnewdisplayname: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, pvargpmprogress: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, pvargpmcancel: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppigpmresult: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, gpmreporttype: GPMReportType, pvargpmprogress: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, pvargpmcancel: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppigpmresult: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, gpmreporttype: GPMReportType, bstrtargetfilepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppigpmresult: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppsecurityinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psecurityinfo: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IGPMStarterGPOBackup(pub ::windows::runtime::IUnknown);
impl IGPMStarterGPOBackup {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn BackupDir(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn Comment(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn DisplayName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn Domain(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn StarterGPOID(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn ID(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn Timestamp(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn Type(&self) -> ::windows::runtime::Result<GPMStarterGPOType> {
        let mut result__: <GPMStarterGPOType as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GPMStarterGPOType>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn Delete(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn GenerateReport(&self, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(gpmreporttype), ::core::mem::transmute(pvargpmprogress), ::core::mem::transmute(pvargpmcancel), ::core::mem::transmute(ppigpmresult)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn GenerateReportToFile<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, gpmreporttype: GPMReportType, bstrtargetfilepath: Param1) -> ::windows::runtime::Result<IGPMResult> {
        let mut result__: <IGPMResult as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(gpmreporttype), bstrtargetfilepath.into_param().abi(), &mut result__).from_abi::<IGPMResult>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IGPMStarterGPOBackup {
    type Vtable = IGPMStarterGPOBackup_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x51d98eda_a87e_43dd_b80a_0b66ef1938d6);
}
impl ::core::convert::From<IGPMStarterGPOBackup> for ::windows::runtime::IUnknown {
    fn from(value: IGPMStarterGPOBackup) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IGPMStarterGPOBackup> for ::windows::runtime::IUnknown {
    fn from(value: &IGPMStarterGPOBackup) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IGPMStarterGPOBackup {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IGPMStarterGPOBackup {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGPMStarterGPOBackup> for super::Com::IDispatch {
    fn from(value: IGPMStarterGPOBackup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGPMStarterGPOBackup> for super::Com::IDispatch {
    fn from(value: &IGPMStarterGPOBackup) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for IGPMStarterGPOBackup {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for &IGPMStarterGPOBackup {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGPMStarterGPOBackup_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrbackupdir: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrcomment: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrdisplayname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrtemplatedomain: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrtemplateid: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrid: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptimestamp: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptype: *mut GPMStarterGPOType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, gpmreporttype: GPMReportType, pvargpmprogress: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, pvargpmcancel: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppigpmresult: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, gpmreporttype: GPMReportType, bstrtargetfilepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppigpmresult: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IGPMStarterGPOBackupCollection(pub ::windows::runtime::IUnknown);
impl IGPMStarterGPOBackupCollection {
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn Count(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Item(&self, lindex: i32) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(lindex), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(feature = "Win32_System_Ole")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_System_Ole`*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::runtime::Result<super::Ole::IEnumVARIANT> {
        let mut result__: <super::Ole::IEnumVARIANT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Ole::IEnumVARIANT>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IGPMStarterGPOBackupCollection {
    type Vtable = IGPMStarterGPOBackupCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xc998031d_add0_4bb5_8dea_298505d8423b);
}
impl ::core::convert::From<IGPMStarterGPOBackupCollection> for ::windows::runtime::IUnknown {
    fn from(value: IGPMStarterGPOBackupCollection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IGPMStarterGPOBackupCollection> for ::windows::runtime::IUnknown {
    fn from(value: &IGPMStarterGPOBackupCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IGPMStarterGPOBackupCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IGPMStarterGPOBackupCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGPMStarterGPOBackupCollection> for super::Com::IDispatch {
    fn from(value: IGPMStarterGPOBackupCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGPMStarterGPOBackupCollection> for super::Com::IDispatch {
    fn from(value: &IGPMStarterGPOBackupCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for IGPMStarterGPOBackupCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for &IGPMStarterGPOBackupCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGPMStarterGPOBackupCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lindex: i32, pval: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_System_Ole")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppigpmtmplbackup: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))] usize,
);
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IGPMStarterGPOCollection(pub ::windows::runtime::IUnknown);
impl IGPMStarterGPOCollection {
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn Count(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Item(&self, lindex: i32) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(lindex), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(feature = "Win32_System_Ole")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_System_Ole`*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::runtime::Result<super::Ole::IEnumVARIANT> {
        let mut result__: <super::Ole::IEnumVARIANT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Ole::IEnumVARIANT>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IGPMStarterGPOCollection {
    type Vtable = IGPMStarterGPOCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x2e522729_2219_44ad_933a_64dfd650c423);
}
impl ::core::convert::From<IGPMStarterGPOCollection> for ::windows::runtime::IUnknown {
    fn from(value: IGPMStarterGPOCollection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IGPMStarterGPOCollection> for ::windows::runtime::IUnknown {
    fn from(value: &IGPMStarterGPOCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IGPMStarterGPOCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IGPMStarterGPOCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGPMStarterGPOCollection> for super::Com::IDispatch {
    fn from(value: IGPMStarterGPOCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGPMStarterGPOCollection> for super::Com::IDispatch {
    fn from(value: &IGPMStarterGPOCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for IGPMStarterGPOCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for &IGPMStarterGPOCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGPMStarterGPOCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lindex: i32, pval: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_System_Ole")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppigpmtemplates: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))] usize,
);
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IGPMStatusMessage(pub ::windows::runtime::IUnknown);
impl IGPMStatusMessage {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn ObjectPath(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn ErrorCode(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn ExtensionName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn SettingsName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn OperationCode(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn Message(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IGPMStatusMessage {
    type Vtable = IGPMStatusMessage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x8496c22f_f3de_4a1f_8f58_603caaa93d7b);
}
impl ::core::convert::From<IGPMStatusMessage> for ::windows::runtime::IUnknown {
    fn from(value: IGPMStatusMessage) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IGPMStatusMessage> for ::windows::runtime::IUnknown {
    fn from(value: &IGPMStatusMessage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IGPMStatusMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IGPMStatusMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGPMStatusMessage> for super::Com::IDispatch {
    fn from(value: IGPMStatusMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGPMStatusMessage> for super::Com::IDispatch {
    fn from(value: &IGPMStatusMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for IGPMStatusMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for &IGPMStatusMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGPMStatusMessage_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IGPMStatusMsgCollection(pub ::windows::runtime::IUnknown);
impl IGPMStatusMsgCollection {
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn Count(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Item(&self, lindex: i32) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(lindex), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(feature = "Win32_System_Ole")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_System_Ole`*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::runtime::Result<super::Ole::IEnumVARIANT> {
        let mut result__: <super::Ole::IEnumVARIANT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Ole::IEnumVARIANT>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IGPMStatusMsgCollection {
    type Vtable = IGPMStatusMsgCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x9b6e1af0_1a92_40f3_a59d_f36ac1f728b7);
}
impl ::core::convert::From<IGPMStatusMsgCollection> for ::windows::runtime::IUnknown {
    fn from(value: IGPMStatusMsgCollection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IGPMStatusMsgCollection> for ::windows::runtime::IUnknown {
    fn from(value: &IGPMStatusMsgCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IGPMStatusMsgCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IGPMStatusMsgCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGPMStatusMsgCollection> for super::Com::IDispatch {
    fn from(value: IGPMStatusMsgCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGPMStatusMsgCollection> for super::Com::IDispatch {
    fn from(value: &IGPMStatusMsgCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for IGPMStatusMsgCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for &IGPMStatusMsgCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGPMStatusMsgCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lindex: i32, pval: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_System_Ole")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))] usize,
);
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IGPMTrustee(pub ::windows::runtime::IUnknown);
impl IGPMTrustee {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn TrusteeSid(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn TrusteeName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn TrusteeDomain(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn TrusteeDSPath(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn TrusteeType(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IGPMTrustee {
    type Vtable = IGPMTrustee_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x3b466da8_c1a4_4b2a_999a_befcdd56cefb);
}
impl ::core::convert::From<IGPMTrustee> for ::windows::runtime::IUnknown {
    fn from(value: IGPMTrustee) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IGPMTrustee> for ::windows::runtime::IUnknown {
    fn from(value: &IGPMTrustee) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IGPMTrustee {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IGPMTrustee {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGPMTrustee> for super::Com::IDispatch {
    fn from(value: IGPMTrustee) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGPMTrustee> for super::Com::IDispatch {
    fn from(value: &IGPMTrustee) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for IGPMTrustee {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for &IGPMTrustee {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGPMTrustee_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lval: *mut i32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IGPMWMIFilter(pub ::windows::runtime::IUnknown);
impl IGPMWMIFilter {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn Path(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn SetName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, newval: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), newval.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn Name(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn SetDescription<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, newval: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), newval.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn Description(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn GetQueryList(&self) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn GetSecurityInfo(&self) -> ::windows::runtime::Result<IGPMSecurityInfo> {
        let mut result__: <IGPMSecurityInfo as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IGPMSecurityInfo>(result__)
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn SetSecurityInfo<'a, Param0: ::windows::runtime::IntoParam<'a, IGPMSecurityInfo>>(&self, psecurityinfo: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), psecurityinfo.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IGPMWMIFilter {
    type Vtable = IGPMWMIFilter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xef2ff9b4_3c27_459a_b979_038305cec75d);
}
impl ::core::convert::From<IGPMWMIFilter> for ::windows::runtime::IUnknown {
    fn from(value: IGPMWMIFilter) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IGPMWMIFilter> for ::windows::runtime::IUnknown {
    fn from(value: &IGPMWMIFilter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IGPMWMIFilter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IGPMWMIFilter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGPMWMIFilter> for super::Com::IDispatch {
    fn from(value: IGPMWMIFilter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGPMWMIFilter> for super::Com::IDispatch {
    fn from(value: &IGPMWMIFilter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for IGPMWMIFilter {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for &IGPMWMIFilter {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGPMWMIFilter_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pqrylist: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppsecurityinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psecurityinfo: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IGPMWMIFilterCollection(pub ::windows::runtime::IUnknown);
impl IGPMWMIFilterCollection {
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn Count(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Item(&self, lindex: i32) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(lindex), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(feature = "Win32_System_Ole")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_System_Ole`*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::runtime::Result<super::Ole::IEnumVARIANT> {
        let mut result__: <super::Ole::IEnumVARIANT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Ole::IEnumVARIANT>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IGPMWMIFilterCollection {
    type Vtable = IGPMWMIFilterCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x5782d582_1a36_4661_8a94_c3c32551945b);
}
impl ::core::convert::From<IGPMWMIFilterCollection> for ::windows::runtime::IUnknown {
    fn from(value: IGPMWMIFilterCollection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IGPMWMIFilterCollection> for ::windows::runtime::IUnknown {
    fn from(value: &IGPMWMIFilterCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IGPMWMIFilterCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IGPMWMIFilterCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGPMWMIFilterCollection> for super::Com::IDispatch {
    fn from(value: IGPMWMIFilterCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGPMWMIFilterCollection> for super::Com::IDispatch {
    fn from(value: &IGPMWMIFilterCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for IGPMWMIFilterCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for &IGPMWMIFilterCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGPMWMIFilterCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lindex: i32, pval: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_System_Ole")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))] usize,
);
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IGroupPolicyObject(pub ::windows::runtime::IUnknown);
impl IGroupPolicyObject {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn New<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszdomainname: Param0, pszdisplayname: Param1, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pszdomainname.into_param().abi(), pszdisplayname.into_param().abi(), ::core::mem::transmute(dwflags)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn OpenDSGPO<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszpath: Param0, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pszpath.into_param().abi(), ::core::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn OpenLocalMachineGPO(&self, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn OpenRemoteMachineGPO<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszcomputername: Param0, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pszcomputername.into_param().abi(), ::core::mem::transmute(dwflags)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn Save<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bmachine: Param0, badd: Param1, pguidextension: *mut ::windows::runtime::GUID, pguid: *mut ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), bmachine.into_param().abi(), badd.into_param().abi(), ::core::mem::transmute(pguidextension), ::core::mem::transmute(pguid)).ok()
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn Delete(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn GetName(&self, pszname: super::super::Foundation::PWSTR, cchmaxlength: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(pszname), ::core::mem::transmute(cchmaxlength)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn GetDisplayName(&self, pszname: super::super::Foundation::PWSTR, cchmaxlength: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(pszname), ::core::mem::transmute(cchmaxlength)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn SetDisplayName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszname: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), pszname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn GetPath(&self, pszpath: super::super::Foundation::PWSTR, cchmaxlength: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(pszpath), ::core::mem::transmute(cchmaxlength)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn GetDSPath(&self, dwsection: u32, pszpath: super::super::Foundation::PWSTR, cchmaxpath: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwsection), ::core::mem::transmute(pszpath), ::core::mem::transmute(cchmaxpath)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn GetFileSysPath(&self, dwsection: u32, pszpath: super::super::Foundation::PWSTR, cchmaxpath: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwsection), ::core::mem::transmute(pszpath), ::core::mem::transmute(cchmaxpath)).ok()
    }
    #[cfg(feature = "Win32_System_Registry")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_System_Registry`*"]
    pub unsafe fn GetRegistryKey(&self, dwsection: u32, hkey: *mut super::Registry::HKEY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwsection), ::core::mem::transmute(hkey)).ok()
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn GetOptions(&self, dwoptions: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoptions)).ok()
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn SetOptions(&self, dwoptions: u32, dwmask: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoptions), ::core::mem::transmute(dwmask)).ok()
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn GetType(&self, gpotype: *mut GROUP_POLICY_OBJECT_TYPE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(gpotype)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn GetMachineName(&self, pszname: super::super::Foundation::PWSTR, cchmaxlength: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(pszname), ::core::mem::transmute(cchmaxlength)).ok()
    }
    #[cfg(feature = "Win32_UI_Controls")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_UI_Controls`*"]
    pub unsafe fn GetPropertySheetPages(&self, hpages: *mut *mut super::super::UI::Controls::HPROPSHEETPAGE, upagecount: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(hpages), ::core::mem::transmute(upagecount)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IGroupPolicyObject {
    type Vtable = IGroupPolicyObject_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xea502723_a23d_11d1_a7d3_0000f87571e3);
}
impl ::core::convert::From<IGroupPolicyObject> for ::windows::runtime::IUnknown {
    fn from(value: IGroupPolicyObject) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IGroupPolicyObject> for ::windows::runtime::IUnknown {
    fn from(value: &IGroupPolicyObject) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IGroupPolicyObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IGroupPolicyObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGroupPolicyObject_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszdomainname: super::super::Foundation::PWSTR, pszdisplayname: super::super::Foundation::PWSTR, dwflags: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszpath: super::super::Foundation::PWSTR, dwflags: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwflags: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszcomputername: super::super::Foundation::PWSTR, dwflags: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bmachine: super::super::Foundation::BOOL, badd: super::super::Foundation::BOOL, pguidextension: *mut ::windows::runtime::GUID, pguid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszname: super::super::Foundation::PWSTR, cchmaxlength: i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszname: super::super::Foundation::PWSTR, cchmaxlength: i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszname: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszpath: super::super::Foundation::PWSTR, cchmaxlength: i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwsection: u32, pszpath: super::super::Foundation::PWSTR, cchmaxpath: i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwsection: u32, pszpath: super::super::Foundation::PWSTR, cchmaxpath: i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Registry")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwsection: u32, hkey: *mut super::Registry::HKEY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Registry"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwoptions: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwoptions: u32, dwmask: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, gpotype: *mut GROUP_POLICY_OBJECT_TYPE) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszname: super::super::Foundation::PWSTR, cchmaxlength: i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_UI_Controls")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hpages: *mut *mut super::super::UI::Controls::HPROPSHEETPAGE, upagecount: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_UI_Controls"))] usize,
);
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
pub struct INSTALLDATA {
    pub Type: INSTALLSPECTYPE,
    pub Spec: INSTALLSPEC,
}
#[cfg(feature = "Win32_Foundation")]
impl INSTALLDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INSTALLDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for INSTALLDATA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for INSTALLDATA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for INSTALLDATA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
pub union INSTALLSPEC {
    pub AppName: INSTALLSPEC_0,
    pub FileExt: super::super::Foundation::PWSTR,
    pub ProgId: super::super::Foundation::PWSTR,
    pub COMClass: INSTALLSPEC_1,
}
#[cfg(feature = "Win32_Foundation")]
impl INSTALLSPEC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INSTALLSPEC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for INSTALLSPEC {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for INSTALLSPEC {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for INSTALLSPEC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct INSTALLSPEC_0 {
    pub Name: super::super::Foundation::PWSTR,
    pub GPOId: ::windows::runtime::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl INSTALLSPEC_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INSTALLSPEC_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for INSTALLSPEC_0 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_AppName_e__Struct").field("Name", &self.Name).field("GPOId", &self.GPOId).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for INSTALLSPEC_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Name == other.Name && self.GPOId == other.GPOId
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for INSTALLSPEC_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for INSTALLSPEC_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct INSTALLSPEC_1 {
    pub Clsid: ::windows::runtime::GUID,
    pub ClsCtx: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl INSTALLSPEC_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INSTALLSPEC_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for INSTALLSPEC_1 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_COMClass_e__Struct").field("Clsid", &self.Clsid).field("ClsCtx", &self.ClsCtx).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for INSTALLSPEC_1 {
    fn eq(&self, other: &Self) -> bool {
        self.Clsid == other.Clsid && self.ClsCtx == other.ClsCtx
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for INSTALLSPEC_1 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for INSTALLSPEC_1 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct INSTALLSPECTYPE(pub i32);
pub const APPNAME: INSTALLSPECTYPE = INSTALLSPECTYPE(1i32);
pub const FILEEXT: INSTALLSPECTYPE = INSTALLSPECTYPE(2i32);
pub const PROGID: INSTALLSPECTYPE = INSTALLSPECTYPE(3i32);
pub const COMCLASS: INSTALLSPECTYPE = INSTALLSPECTYPE(4i32);
impl ::core::convert::From<i32> for INSTALLSPECTYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for INSTALLSPECTYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRSOPInformation(pub ::windows::runtime::IUnknown);
impl IRSOPInformation {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn GetNamespace(&self, dwsection: u32, pszname: super::super::Foundation::PWSTR, cchmaxlength: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwsection), ::core::mem::transmute(pszname), ::core::mem::transmute(cchmaxlength)).ok()
    }
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub unsafe fn GetFlags(&self, pdwflags: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwflags)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    pub unsafe fn GetEventLogEntryText<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszeventsource: Param0, pszeventlogname: Param1, pszeventtime: Param2, dweventid: u32) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pszeventsource.into_param().abi(), pszeventlogname.into_param().abi(), pszeventtime.into_param().abi(), ::core::mem::transmute(dweventid), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IRSOPInformation {
    type Vtable = IRSOPInformation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x9a5a81b5_d9c7_49ef_9d11_ddf50968c48d);
}
impl ::core::convert::From<IRSOPInformation> for ::windows::runtime::IUnknown {
    fn from(value: IRSOPInformation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRSOPInformation> for ::windows::runtime::IUnknown {
    fn from(value: &IRSOPInformation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IRSOPInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IRSOPInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRSOPInformation_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwsection: u32, pszname: super::super::Foundation::PWSTR, cchmaxlength: i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwflags: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszeventsource: super::super::Foundation::PWSTR, pszeventlogname: super::super::Foundation::PWSTR, pszeventtime: super::super::Foundation::PWSTR, dweventid: u32, ppsztext: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ImportRSoPData<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(lpnamespace: Param0, lpfilename: Param1) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImportRSoPData(lpnamespace: super::super::Foundation::PWSTR, lpfilename: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        ImportRSoPData(lpnamespace.into_param().abi(), lpfilename.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InstallApplication(pinstallinfo: *const INSTALLDATA) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InstallApplication(pinstallinfo: *const INSTALLDATA) -> u32;
        }
        ::core::mem::transmute(InstallApplication(::core::mem::transmute(pinstallinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
pub struct LOCALMANAGEDAPPLICATION {
    pub pszDeploymentName: super::super::Foundation::PWSTR,
    pub pszPolicyName: super::super::Foundation::PWSTR,
    pub pszProductId: super::super::Foundation::PWSTR,
    pub dwState: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl LOCALMANAGEDAPPLICATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for LOCALMANAGEDAPPLICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for LOCALMANAGEDAPPLICATION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("LOCALMANAGEDAPPLICATION").field("pszDeploymentName", &self.pszDeploymentName).field("pszPolicyName", &self.pszPolicyName).field("pszProductId", &self.pszProductId).field("dwState", &self.dwState).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for LOCALMANAGEDAPPLICATION {
    fn eq(&self, other: &Self) -> bool {
        self.pszDeploymentName == other.pszDeploymentName && self.pszPolicyName == other.pszPolicyName && self.pszProductId == other.pszProductId && self.dwState == other.dwState
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for LOCALMANAGEDAPPLICATION {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for LOCALMANAGEDAPPLICATION {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
pub const LOCALSTATE_ASSIGNED: u32 = 1u32;
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
pub const LOCALSTATE_ORPHANED: u32 = 32u32;
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
pub const LOCALSTATE_POLICYREMOVE_ORPHAN: u32 = 8u32;
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
pub const LOCALSTATE_POLICYREMOVE_UNINSTALL: u32 = 16u32;
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
pub const LOCALSTATE_PUBLISHED: u32 = 2u32;
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
pub const LOCALSTATE_UNINSTALLED: u32 = 64u32;
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
pub const LOCALSTATE_UNINSTALL_UNMANAGED: u32 = 4u32;
#[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LeaveCriticalPolicySection<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hsection: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LeaveCriticalPolicySection(hsection: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(LeaveCriticalPolicySection(hsection.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
pub struct MANAGEDAPPLICATION {
    pub pszPackageName: super::super::Foundation::PWSTR,
    pub pszPublisher: super::super::Foundation::PWSTR,
    pub dwVersionHi: u32,
    pub dwVersionLo: u32,
    pub dwRevision: u32,
    pub GpoId: ::windows::runtime::GUID,
    pub pszPolicyName: super::super::Foundation::PWSTR,
    pub ProductId: ::windows::runtime::GUID,
    pub Language: u16,
    pub pszOwner: super::super::Foundation::PWSTR,
    pub pszCompany: super::super::Foundation::PWSTR,
    pub pszComments: super::super::Foundation::PWSTR,
    pub pszContact: super::super::Foundation::PWSTR,
    pub pszSupportUrl: super::super::Foundation::PWSTR,
    pub dwPathType: u32,
    pub bInstalled: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl MANAGEDAPPLICATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MANAGEDAPPLICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MANAGEDAPPLICATION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("MANAGEDAPPLICATION")
            .field("pszPackageName", &self.pszPackageName)
            .field("pszPublisher", &self.pszPublisher)
            .field("dwVersionHi", &self.dwVersionHi)
            .field("dwVersionLo", &self.dwVersionLo)
            .field("dwRevision", &self.dwRevision)
            .field("GpoId", &self.GpoId)
            .field("pszPolicyName", &self.pszPolicyName)
            .field("ProductId", &self.ProductId)
            .field("Language", &self.Language)
            .field("pszOwner", &self.pszOwner)
            .field("pszCompany", &self.pszCompany)
            .field("pszComments", &self.pszComments)
            .field("pszContact", &self.pszContact)
            .field("pszSupportUrl", &self.pszSupportUrl)
            .field("dwPathType", &self.dwPathType)
            .field("bInstalled", &self.bInstalled)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MANAGEDAPPLICATION {
    fn eq(&self, other: &Self) -> bool {
        self.pszPackageName == other.pszPackageName
            && self.pszPublisher == other.pszPublisher
            && self.dwVersionHi == other.dwVersionHi
            && self.dwVersionLo == other.dwVersionLo
            && self.dwRevision == other.dwRevision
            && self.GpoId == other.GpoId
            && self.pszPolicyName == other.pszPolicyName
            && self.ProductId == other.ProductId
            && self.Language == other.Language
            && self.pszOwner == other.pszOwner
            && self.pszCompany == other.pszCompany
            && self.pszComments == other.pszComments
            && self.pszContact == other.pszContact
            && self.pszSupportUrl == other.pszSupportUrl
            && self.dwPathType == other.dwPathType
            && self.bInstalled == other.bInstalled
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MANAGEDAPPLICATION {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MANAGEDAPPLICATION {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
pub const MANAGED_APPS_FROMCATEGORY: u32 = 2u32;
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
pub const MANAGED_APPS_INFOLEVEL_DEFAULT: u32 = 65536u32;
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
pub const MANAGED_APPS_USERAPPLICATIONS: u32 = 1u32;
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
pub const MANAGED_APPTYPE_SETUPEXE: u32 = 2u32;
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
pub const MANAGED_APPTYPE_UNSUPPORTED: u32 = 3u32;
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
pub const MANAGED_APPTYPE_WINDOWSINSTALLER: u32 = 1u32;
pub const NODEID_Machine: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x8fc0b737_a0e1_11d1_a7d3_0000f87571e3);
pub const NODEID_MachineSWSettings: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x8fc0b73a_a0e1_11d1_a7d3_0000f87571e3);
pub const NODEID_RSOPMachine: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xbd4c1a2e_0b7a_4a62_a6b0_c0577539c97e);
pub const NODEID_RSOPMachineSWSettings: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x6a76273e_eb8e_45db_94c5_25663a5f2c1a);
pub const NODEID_RSOPUser: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xab87364f_0cec_4cd8_9bf8_898f34628fb8);
pub const NODEID_RSOPUserSWSettings: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xe52c5ce3_fd27_4402_84de_d9a5f2858910);
pub const NODEID_User: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x8fc0b738_a0e1_11d1_a7d3_0000f87571e3);
pub const NODEID_UserSWSettings: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x8fc0b73c_a0e1_11d1_a7d3_0000f87571e3);
#[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Wmi`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Wmi"))]
pub type PFNGENERATEGROUPPOLICY = unsafe extern "system" fn(dwflags: u32, pbabort: *mut super::super::Foundation::BOOL, pwszsite: super::super::Foundation::PWSTR, pcomputertarget: *const ::core::mem::ManuallyDrop<RSOP_TARGET>, pusertarget: *const ::core::mem::ManuallyDrop<RSOP_TARGET>) -> u32;
#[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`, `Win32_System_Registry`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub type PFNPROCESSGROUPPOLICY = unsafe extern "system" fn(dwflags: u32, htoken: super::super::Foundation::HANDLE, hkeyroot: super::Registry::HKEY, pdeletedgpolist: *const GROUP_POLICY_OBJECTA, pchangedgpolist: *const GROUP_POLICY_OBJECTA, phandle: usize, pbabort: *mut super::super::Foundation::BOOL, pstatuscallback: ::windows::runtime::RawPtr) -> u32;
#[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`, `Win32_System_Registry`, `Win32_System_Wmi`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry", feature = "Win32_System_Wmi"))]
pub type PFNPROCESSGROUPPOLICYEX = unsafe extern "system" fn(dwflags: u32, htoken: super::super::Foundation::HANDLE, hkeyroot: super::Registry::HKEY, pdeletedgpolist: *const GROUP_POLICY_OBJECTA, pchangedgpolist: *const GROUP_POLICY_OBJECTA, phandle: usize, pbabort: *mut super::super::Foundation::BOOL, pstatuscallback: ::windows::runtime::RawPtr, pwbemservices: ::windows::runtime::RawPtr, prsopstatus: *mut ::windows::runtime::HRESULT) -> u32;
#[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFNSTATUSMESSAGECALLBACK = unsafe extern "system" fn(bverbose: super::super::Foundation::BOOL, lpmessage: super::super::Foundation::PWSTR) -> u32;
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
pub const PI_APPLYPOLICY: u32 = 2u32;
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
pub const PI_NOUI: u32 = 1u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
pub struct POLICYSETTINGSTATUSINFO {
    pub szKey: super::super::Foundation::PWSTR,
    pub szEventSource: super::super::Foundation::PWSTR,
    pub szEventLogName: super::super::Foundation::PWSTR,
    pub dwEventID: u32,
    pub dwErrorCode: u32,
    pub status: SETTINGSTATUS,
    pub timeLogged: super::super::Foundation::SYSTEMTIME,
}
#[cfg(feature = "Win32_Foundation")]
impl POLICYSETTINGSTATUSINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for POLICYSETTINGSTATUSINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for POLICYSETTINGSTATUSINFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("POLICYSETTINGSTATUSINFO")
            .field("szKey", &self.szKey)
            .field("szEventSource", &self.szEventSource)
            .field("szEventLogName", &self.szEventLogName)
            .field("dwEventID", &self.dwEventID)
            .field("dwErrorCode", &self.dwErrorCode)
            .field("status", &self.status)
            .field("timeLogged", &self.timeLogged)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for POLICYSETTINGSTATUSINFO {
    fn eq(&self, other: &Self) -> bool {
        self.szKey == other.szKey && self.szEventSource == other.szEventSource && self.szEventLogName == other.szEventLogName && self.dwEventID == other.dwEventID && self.dwErrorCode == other.dwErrorCode && self.status == other.status && self.timeLogged == other.timeLogged
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for POLICYSETTINGSTATUSINFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for POLICYSETTINGSTATUSINFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
pub const PT_MANDATORY: u32 = 4u32;
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
pub const PT_ROAMING: u32 = 2u32;
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
pub const PT_ROAMING_PREEXISTING: u32 = 8u32;
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
pub const PT_TEMPORARY: u32 = 1u32;
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
#[inline]
pub unsafe fn ProcessGroupPolicyCompleted(extensionid: *const ::windows::runtime::GUID, pasynchandle: usize, dwstatus: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ProcessGroupPolicyCompleted(extensionid: *const ::windows::runtime::GUID, pasynchandle: usize, dwstatus: u32) -> u32;
        }
        ::core::mem::transmute(ProcessGroupPolicyCompleted(::core::mem::transmute(extensionid), ::core::mem::transmute(pasynchandle), ::core::mem::transmute(dwstatus)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
#[inline]
pub unsafe fn ProcessGroupPolicyCompletedEx(extensionid: *const ::windows::runtime::GUID, pasynchandle: usize, dwstatus: u32, rsopstatus: ::windows::runtime::HRESULT) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ProcessGroupPolicyCompletedEx(extensionid: *const ::windows::runtime::GUID, pasynchandle: usize, dwstatus: u32, rsopstatus: ::windows::runtime::HRESULT) -> u32;
        }
        ::core::mem::transmute(ProcessGroupPolicyCompletedEx(::core::mem::transmute(extensionid), ::core::mem::transmute(pasynchandle), ::core::mem::transmute(dwstatus), ::core::mem::transmute(rsopstatus)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
pub const RP_FORCE: u32 = 1u32;
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
pub const RP_SYNC: u32 = 2u32;
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
pub const RSOP_COMPUTER_ACCESS_DENIED: u32 = 2u32;
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
pub const RSOP_INFO_FLAG_DIAGNOSTIC_MODE: u32 = 1u32;
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
pub const RSOP_NO_COMPUTER: u32 = 65536u32;
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
pub const RSOP_NO_USER: u32 = 131072u32;
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
pub const RSOP_PLANNING_ASSUME_COMP_WQLFILTER_TRUE: u32 = 16u32;
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
pub const RSOP_PLANNING_ASSUME_LOOPBACK_MERGE: u32 = 2u32;
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
pub const RSOP_PLANNING_ASSUME_LOOPBACK_REPLACE: u32 = 4u32;
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
pub const RSOP_PLANNING_ASSUME_SLOW_LINK: u32 = 1u32;
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
pub const RSOP_PLANNING_ASSUME_USER_WQLFILTER_TRUE: u32 = 8u32;
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Wmi"))]
#[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Wmi`*"]
pub struct RSOP_TARGET {
    pub pwszAccountName: super::super::Foundation::PWSTR,
    pub pwszNewSOM: super::super::Foundation::PWSTR,
    pub psaSecurityGroups: *mut super::Com::SAFEARRAY,
    pub pRsopToken: *mut ::core::ffi::c_void,
    pub pGPOList: *mut GROUP_POLICY_OBJECTA,
    pub pWbemServices: ::core::option::Option<super::Wmi::IWbemServices>,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Wmi"))]
impl RSOP_TARGET {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Wmi"))]
impl ::core::default::Default for RSOP_TARGET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Wmi"))]
impl ::core::fmt::Debug for RSOP_TARGET {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("RSOP_TARGET").field("pwszAccountName", &self.pwszAccountName).field("pwszNewSOM", &self.pwszNewSOM).field("psaSecurityGroups", &self.psaSecurityGroups).field("pRsopToken", &self.pRsopToken).field("pGPOList", &self.pGPOList).field("pWbemServices", &self.pWbemServices).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Wmi"))]
impl ::core::cmp::PartialEq for RSOP_TARGET {
    fn eq(&self, other: &Self) -> bool {
        self.pwszAccountName == other.pwszAccountName && self.pwszNewSOM == other.pwszNewSOM && self.psaSecurityGroups == other.psaSecurityGroups && self.pRsopToken == other.pRsopToken && self.pGPOList == other.pGPOList && self.pWbemServices == other.pWbemServices
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Wmi"))]
impl ::core::cmp::Eq for RSOP_TARGET {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Wmi"))]
unsafe impl ::windows::runtime::Abi for RSOP_TARGET {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
pub const RSOP_TEMPNAMESPACE_EXISTS: u32 = 4u32;
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
pub const RSOP_USER_ACCESS_DENIED: u32 = 1u32;
#[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RefreshPolicy<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(bmachine: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RefreshPolicy(bmachine: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(RefreshPolicy(bmachine.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RefreshPolicyEx<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(bmachine: Param0, dwoptions: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RefreshPolicyEx(bmachine: super::super::Foundation::BOOL, dwoptions: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(RefreshPolicyEx(bmachine.into_param().abi(), ::core::mem::transmute(dwoptions)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegisterGPNotification<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(hevent: Param0, bmachine: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegisterGPNotification(hevent: super::super::Foundation::HANDLE, bmachine: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(RegisterGPNotification(hevent.into_param().abi(), bmachine.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`, `Win32_Security`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn RsopAccessCheckByType<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSID>>(
    psecuritydescriptor: *const super::super::Security::SECURITY_DESCRIPTOR,
    pprincipalselfsid: Param1,
    prsoptoken: *const ::core::ffi::c_void,
    dwdesiredaccessmask: u32,
    pobjecttypelist: *const super::super::Security::OBJECT_TYPE_LIST,
    objecttypelistlength: u32,
    pgenericmapping: *const super::super::Security::GENERIC_MAPPING,
    pprivilegeset: *const super::super::Security::PRIVILEGE_SET,
    pdwprivilegesetlength: *const u32,
    pdwgrantedaccessmask: *mut u32,
    pbaccessstatus: *mut i32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RsopAccessCheckByType(
                psecuritydescriptor: *const super::super::Security::SECURITY_DESCRIPTOR,
                pprincipalselfsid: super::super::Foundation::PSID,
                prsoptoken: *const ::core::ffi::c_void,
                dwdesiredaccessmask: u32,
                pobjecttypelist: *const super::super::Security::OBJECT_TYPE_LIST,
                objecttypelistlength: u32,
                pgenericmapping: *const super::super::Security::GENERIC_MAPPING,
                pprivilegeset: *const super::super::Security::PRIVILEGE_SET,
                pdwprivilegesetlength: *const u32,
                pdwgrantedaccessmask: *mut u32,
                pbaccessstatus: *mut i32,
            ) -> ::windows::runtime::HRESULT;
        }
        RsopAccessCheckByType(
            ::core::mem::transmute(psecuritydescriptor),
            pprincipalselfsid.into_param().abi(),
            ::core::mem::transmute(prsoptoken),
            ::core::mem::transmute(dwdesiredaccessmask),
            ::core::mem::transmute(pobjecttypelist),
            ::core::mem::transmute(objecttypelistlength),
            ::core::mem::transmute(pgenericmapping),
            ::core::mem::transmute(pprivilegeset),
            ::core::mem::transmute(pdwprivilegesetlength),
            ::core::mem::transmute(pdwgrantedaccessmask),
            ::core::mem::transmute(pbaccessstatus),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RsopFileAccessCheck<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(pszfilename: Param0, prsoptoken: *const ::core::ffi::c_void, dwdesiredaccessmask: u32, pdwgrantedaccessmask: *mut u32, pbaccessstatus: *mut i32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RsopFileAccessCheck(pszfilename: super::super::Foundation::PWSTR, prsoptoken: *const ::core::ffi::c_void, dwdesiredaccessmask: u32, pdwgrantedaccessmask: *mut u32, pbaccessstatus: *mut i32) -> ::windows::runtime::HRESULT;
        }
        RsopFileAccessCheck(pszfilename.into_param().abi(), ::core::mem::transmute(prsoptoken), ::core::mem::transmute(dwdesiredaccessmask), ::core::mem::transmute(pdwgrantedaccessmask), ::core::mem::transmute(pbaccessstatus)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_System_Wmi`*"]
#[cfg(feature = "Win32_System_Wmi")]
#[inline]
pub unsafe fn RsopResetPolicySettingStatus<'a, Param1: ::windows::runtime::IntoParam<'a, super::Wmi::IWbemServices>, Param2: ::windows::runtime::IntoParam<'a, super::Wmi::IWbemClassObject>>(dwflags: u32, pservices: Param1, psettinginstance: Param2) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RsopResetPolicySettingStatus(dwflags: u32, pservices: ::windows::runtime::RawPtr, psettinginstance: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        RsopResetPolicySettingStatus(::core::mem::transmute(dwflags), pservices.into_param().abi(), psettinginstance.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`, `Win32_System_Wmi`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Wmi"))]
#[inline]
pub unsafe fn RsopSetPolicySettingStatus<'a, Param1: ::windows::runtime::IntoParam<'a, super::Wmi::IWbemServices>, Param2: ::windows::runtime::IntoParam<'a, super::Wmi::IWbemClassObject>>(dwflags: u32, pservices: Param1, psettinginstance: Param2, ninfo: u32, pstatus: *const POLICYSETTINGSTATUSINFO) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RsopSetPolicySettingStatus(dwflags: u32, pservices: ::windows::runtime::RawPtr, psettinginstance: ::windows::runtime::RawPtr, ninfo: u32, pstatus: *const POLICYSETTINGSTATUSINFO) -> ::windows::runtime::HRESULT;
        }
        RsopSetPolicySettingStatus(::core::mem::transmute(dwflags), pservices.into_param().abi(), psettinginstance.into_param().abi(), ::core::mem::transmute(ninfo), ::core::mem::transmute(pstatus)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_GroupPolicy`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SETTINGSTATUS(pub i32);
pub const RSOPUnspecified: SETTINGSTATUS = SETTINGSTATUS(0i32);
pub const RSOPApplied: SETTINGSTATUS = SETTINGSTATUS(1i32);
pub const RSOPIgnored: SETTINGSTATUS = SETTINGSTATUS(2i32);
pub const RSOPFailed: SETTINGSTATUS = SETTINGSTATUS(3i32);
pub const RSOPSubsettingFailed: SETTINGSTATUS = SETTINGSTATUS(4i32);
impl ::core::convert::From<i32> for SETTINGSTATUS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SETTINGSTATUS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UninstallApplication<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(productcode: Param0, dwstatus: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UninstallApplication(productcode: super::super::Foundation::PWSTR, dwstatus: u32) -> u32;
        }
        ::core::mem::transmute(UninstallApplication(productcode.into_param().abi(), ::core::mem::transmute(dwstatus)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UnregisterGPNotification<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hevent: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UnregisterGPNotification(hevent: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(UnregisterGPNotification(hevent.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
