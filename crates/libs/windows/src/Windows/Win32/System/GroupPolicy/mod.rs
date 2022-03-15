#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct APPSTATE(pub i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const ABSENT: APPSTATE = APPSTATE(0i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const ASSIGNED: APPSTATE = APPSTATE(1i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const PUBLISHED: APPSTATE = APPSTATE(2i32);
impl ::core::marker::Copy for APPSTATE {}
impl ::core::clone::Clone for APPSTATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for APPSTATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for APPSTATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for APPSTATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("APPSTATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BrowseForGPO(lpbrowseinfo: *mut GPOBROWSEINFO) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BrowseForGPO(lpbrowseinfo: *mut GPOBROWSEINFO) -> ::windows::core::HRESULT;
        }
        BrowseForGPO(::core::mem::transmute(lpbrowseinfo)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const CLSID_GPESnapIn: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8fc0b734_a0e1_11d1_a7d3_0000f87571e3);
pub const CLSID_GroupPolicyObject: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xea502722_a23d_11d1_a7d3_0000f87571e3);
pub const CLSID_RSOPSnapIn: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6dc3804b_7212_458d_adb0_9a07e2ae1fa2);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
#[inline]
pub unsafe fn CommandLineFromMsiDescriptor<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(descriptor: Param0, commandline: ::windows::core::PWSTR, commandlinelength: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CommandLineFromMsiDescriptor(descriptor: ::windows::core::PCWSTR, commandline: ::windows::core::PWSTR, commandlinelength: *mut u32) -> u32;
        }
        ::core::mem::transmute(CommandLineFromMsiDescriptor(descriptor.into_param().abi(), ::core::mem::transmute(commandline), ::core::mem::transmute(commandlinelength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateGPOLink<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(lpgpo: Param0, lpcontainer: Param1, fhighpriority: Param2) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateGPOLink(lpgpo: ::windows::core::PCWSTR, lpcontainer: ::windows::core::PCWSTR, fhighpriority: super::super::Foundation::BOOL) -> ::windows::core::HRESULT;
        }
        CreateGPOLink(lpgpo.into_param().abi(), lpcontainer.into_param().abi(), fhighpriority.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CriticalPolicySectionHandle(pub isize);
impl CriticalPolicySectionHandle {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
    pub fn ok(self) -> ::windows::core::Result<Self> {
        if !self.is_invalid() {
            Ok(self)
        } else {
            Err(::windows::core::Error::from_win32())
        }
    }
}
impl ::core::default::Default for CriticalPolicySectionHandle {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for CriticalPolicySectionHandle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for CriticalPolicySectionHandle {}
impl ::core::fmt::Debug for CriticalPolicySectionHandle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CriticalPolicySectionHandle").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Abi for CriticalPolicySectionHandle {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
#[inline]
pub unsafe fn DeleteAllGPOLinks<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(lpcontainer: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeleteAllGPOLinks(lpcontainer: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
        }
        DeleteAllGPOLinks(lpcontainer.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
#[inline]
pub unsafe fn DeleteGPOLink<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(lpgpo: Param0, lpcontainer: Param1) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeleteGPOLink(lpgpo: ::windows::core::PCWSTR, lpcontainer: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
        }
        DeleteGPOLink(lpgpo.into_param().abi(), lpcontainer.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnterCriticalPolicySection<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(bmachine: Param0) -> super::super::Foundation::HANDLE {
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
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
#[inline]
pub unsafe fn ExportRSoPData<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(lpnamespace: Param0, lpfilename: Param1) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ExportRSoPData(lpnamespace: ::windows::core::PCWSTR, lpfilename: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
        }
        ExportRSoPData(lpnamespace.into_param().abi(), lpfilename.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const FLAG_ASSUME_COMP_WQLFILTER_TRUE: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const FLAG_ASSUME_SLOW_LINK: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const FLAG_ASSUME_USER_WQLFILTER_TRUE: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const FLAG_FORCE_CREATENAMESPACE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const FLAG_LOOPBACK_MERGE: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const FLAG_LOOPBACK_REPLACE: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const FLAG_NO_COMPUTER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const FLAG_NO_CSE_INVOKE: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const FLAG_NO_GPO_FILTER: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const FLAG_NO_USER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const FLAG_PLANNING_MODE: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
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
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
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
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPC_BLOCK_POLICY: u32 = 1u32;
pub const GPM: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf5694708_88fe_4b35_babf_e56162d5fbc8);
pub const GPMAsyncCancel: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x372796a9_76ec_479d_ad6c_556318ed5f9d);
pub const GPMBackup: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xed1a54b8_5efa_482a_93c0_8ad86f0d68c3);
pub const GPMBackupCollection: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeb8f035b_70db_4a9f_9676_37c25994e9dc);
pub const GPMBackupDir: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfce4a59d_0f21_4afa_b859_e6d0c62cd10c);
pub const GPMBackupDirEx: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe8c0988a_cf03_4c5b_8be2_2aa9ad32aada);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GPMBackupType(pub i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const typeGPO: GPMBackupType = GPMBackupType(0i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const typeStarterGPO: GPMBackupType = GPMBackupType(1i32);
impl ::core::marker::Copy for GPMBackupType {}
impl ::core::clone::Clone for GPMBackupType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GPMBackupType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for GPMBackupType {
    type Abi = Self;
}
impl ::core::fmt::Debug for GPMBackupType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GPMBackupType").field(&self.0).finish()
    }
}
pub const GPMCSECollection: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcf92b828_2d44_4b61_b10a_b327afd42da8);
pub const GPMClientSideExtension: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc1a2e70e_659c_4b1a_940b_f88b0af9c8a4);
pub const GPMConstants: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3855e880_cd9e_4d0c_9eaf_1579283a1888);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GPMDestinationOption(pub i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const opDestinationSameAsSource: GPMDestinationOption = GPMDestinationOption(0i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const opDestinationNone: GPMDestinationOption = GPMDestinationOption(1i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const opDestinationByRelativeName: GPMDestinationOption = GPMDestinationOption(2i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const opDestinationSet: GPMDestinationOption = GPMDestinationOption(3i32);
impl ::core::marker::Copy for GPMDestinationOption {}
impl ::core::clone::Clone for GPMDestinationOption {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GPMDestinationOption {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for GPMDestinationOption {
    type Abi = Self;
}
impl ::core::fmt::Debug for GPMDestinationOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GPMDestinationOption").field(&self.0).finish()
    }
}
pub const GPMDomain: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x710901be_1050_4cb1_838a_c5cff259e183);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GPMEntryType(pub i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const typeUser: GPMEntryType = GPMEntryType(0i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const typeComputer: GPMEntryType = GPMEntryType(1i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const typeLocalGroup: GPMEntryType = GPMEntryType(2i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const typeGlobalGroup: GPMEntryType = GPMEntryType(3i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const typeUniversalGroup: GPMEntryType = GPMEntryType(4i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const typeUNCPath: GPMEntryType = GPMEntryType(5i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const typeUnknown: GPMEntryType = GPMEntryType(6i32);
impl ::core::marker::Copy for GPMEntryType {}
impl ::core::clone::Clone for GPMEntryType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GPMEntryType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for GPMEntryType {
    type Abi = Self;
}
impl ::core::fmt::Debug for GPMEntryType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GPMEntryType").field(&self.0).finish()
    }
}
pub const GPMGPO: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd2ce2994_59b5_4064_b581_4d68486a16c4);
pub const GPMGPOCollection: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7a057325_832d_4de3_a41f_c780436a4e09);
pub const GPMGPOLink: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc1df9880_5303_42c6_8a3c_0488e1bf7364);
pub const GPMGPOLinksCollection: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf6ed581a_49a5_47e2_b771_fd8dc02b6259);
pub const GPMMapEntry: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8c975253_5431_4471_b35d_0626c928258a);
pub const GPMMapEntryCollection: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0cf75d5b_a3a1_4c55_b4fe_9e149c41f66d);
pub const GPMMigrationTable: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x55af4043_2a06_4f72_abef_631b44079c76);
pub const GPMPermission: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5871a40a_e9c0_46ec_913e_944ef9225a94);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GPMPermissionType(pub i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const permGPOApply: GPMPermissionType = GPMPermissionType(65536i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const permGPORead: GPMPermissionType = GPMPermissionType(65792i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const permGPOEdit: GPMPermissionType = GPMPermissionType(65793i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const permGPOEditSecurityAndDelete: GPMPermissionType = GPMPermissionType(65794i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const permGPOCustom: GPMPermissionType = GPMPermissionType(65795i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const permWMIFilterEdit: GPMPermissionType = GPMPermissionType(131072i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const permWMIFilterFullControl: GPMPermissionType = GPMPermissionType(131073i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const permWMIFilterCustom: GPMPermissionType = GPMPermissionType(131074i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const permSOMLink: GPMPermissionType = GPMPermissionType(1835008i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const permSOMLogging: GPMPermissionType = GPMPermissionType(1573120i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const permSOMPlanning: GPMPermissionType = GPMPermissionType(1573376i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const permSOMWMICreate: GPMPermissionType = GPMPermissionType(1049344i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const permSOMWMIFullControl: GPMPermissionType = GPMPermissionType(1049345i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const permSOMGPOCreate: GPMPermissionType = GPMPermissionType(1049600i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const permStarterGPORead: GPMPermissionType = GPMPermissionType(197888i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const permStarterGPOEdit: GPMPermissionType = GPMPermissionType(197889i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const permStarterGPOFullControl: GPMPermissionType = GPMPermissionType(197890i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const permStarterGPOCustom: GPMPermissionType = GPMPermissionType(197891i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const permSOMStarterGPOCreate: GPMPermissionType = GPMPermissionType(1049856i32);
impl ::core::marker::Copy for GPMPermissionType {}
impl ::core::clone::Clone for GPMPermissionType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GPMPermissionType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for GPMPermissionType {
    type Abi = Self;
}
impl ::core::fmt::Debug for GPMPermissionType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GPMPermissionType").field(&self.0).finish()
    }
}
pub const GPMRSOP: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x489b0caf_9ec2_4eb7_91f5_b6f71d43da8c);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GPMRSOPMode(pub i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const rsopUnknown: GPMRSOPMode = GPMRSOPMode(0i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const rsopPlanning: GPMRSOPMode = GPMRSOPMode(1i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const rsopLogging: GPMRSOPMode = GPMRSOPMode(2i32);
impl ::core::marker::Copy for GPMRSOPMode {}
impl ::core::clone::Clone for GPMRSOPMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GPMRSOPMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for GPMRSOPMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for GPMRSOPMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GPMRSOPMode").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GPMReportType(pub i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const repXML: GPMReportType = GPMReportType(0i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const repHTML: GPMReportType = GPMReportType(1i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const repInfraXML: GPMReportType = GPMReportType(2i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const repInfraRefreshXML: GPMReportType = GPMReportType(3i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const repClientHealthXML: GPMReportType = GPMReportType(4i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const repClientHealthRefreshXML: GPMReportType = GPMReportType(5i32);
impl ::core::marker::Copy for GPMReportType {}
impl ::core::clone::Clone for GPMReportType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GPMReportType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for GPMReportType {
    type Abi = Self;
}
impl ::core::fmt::Debug for GPMReportType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GPMReportType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GPMReportingOptions(pub i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const opReportLegacy: GPMReportingOptions = GPMReportingOptions(0i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const opReportComments: GPMReportingOptions = GPMReportingOptions(1i32);
impl ::core::marker::Copy for GPMReportingOptions {}
impl ::core::clone::Clone for GPMReportingOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GPMReportingOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for GPMReportingOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for GPMReportingOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GPMReportingOptions").field(&self.0).finish()
    }
}
pub const GPMResult: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x92101ac0_9287_4206_a3b2_4bdb73d225f6);
pub const GPMSOM: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x32d93fac_450e_44cf_829c_8b22ff6bdae1);
pub const GPMSOMCollection: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x24c1f147_3720_4f5b_a9c3_06b4e4f931d2);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GPMSOMType(pub i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const somSite: GPMSOMType = GPMSOMType(0i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const somDomain: GPMSOMType = GPMSOMType(1i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const somOU: GPMSOMType = GPMSOMType(2i32);
impl ::core::marker::Copy for GPMSOMType {}
impl ::core::clone::Clone for GPMSOMType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GPMSOMType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for GPMSOMType {
    type Abi = Self;
}
impl ::core::fmt::Debug for GPMSOMType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GPMSOMType").field(&self.0).finish()
    }
}
pub const GPMSearchCriteria: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x17aaca26_5ce0_44fa_8cc0_5259e6483566);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GPMSearchOperation(pub i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const opEquals: GPMSearchOperation = GPMSearchOperation(0i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const opContains: GPMSearchOperation = GPMSearchOperation(1i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const opNotContains: GPMSearchOperation = GPMSearchOperation(2i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const opNotEquals: GPMSearchOperation = GPMSearchOperation(3i32);
impl ::core::marker::Copy for GPMSearchOperation {}
impl ::core::clone::Clone for GPMSearchOperation {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GPMSearchOperation {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for GPMSearchOperation {
    type Abi = Self;
}
impl ::core::fmt::Debug for GPMSearchOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GPMSearchOperation").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GPMSearchProperty(pub i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const gpoPermissions: GPMSearchProperty = GPMSearchProperty(0i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const gpoEffectivePermissions: GPMSearchProperty = GPMSearchProperty(1i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const gpoDisplayName: GPMSearchProperty = GPMSearchProperty(2i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const gpoWMIFilter: GPMSearchProperty = GPMSearchProperty(3i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const gpoID: GPMSearchProperty = GPMSearchProperty(4i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const gpoComputerExtensions: GPMSearchProperty = GPMSearchProperty(5i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const gpoUserExtensions: GPMSearchProperty = GPMSearchProperty(6i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const somLinks: GPMSearchProperty = GPMSearchProperty(7i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const gpoDomain: GPMSearchProperty = GPMSearchProperty(8i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const backupMostRecent: GPMSearchProperty = GPMSearchProperty(9i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const starterGPOPermissions: GPMSearchProperty = GPMSearchProperty(10i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const starterGPOEffectivePermissions: GPMSearchProperty = GPMSearchProperty(11i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const starterGPODisplayName: GPMSearchProperty = GPMSearchProperty(12i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const starterGPOID: GPMSearchProperty = GPMSearchProperty(13i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const starterGPODomain: GPMSearchProperty = GPMSearchProperty(14i32);
impl ::core::marker::Copy for GPMSearchProperty {}
impl ::core::clone::Clone for GPMSearchProperty {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GPMSearchProperty {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for GPMSearchProperty {
    type Abi = Self;
}
impl ::core::fmt::Debug for GPMSearchProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GPMSearchProperty").field(&self.0).finish()
    }
}
pub const GPMSecurityInfo: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x547a5e8f_9162_4516_a4df_9ddb9686d846);
pub const GPMSitesContainer: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x229f5c42_852c_4b30_945f_c522be9bd386);
pub const GPMStarterGPOBackup: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x389e400a_d8ef_455b_a861_5f9ca34a6a02);
pub const GPMStarterGPOBackupCollection: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe75ea59d_1aeb_4cb5_a78a_281daa582406);
pub const GPMStarterGPOCollection: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x82f8aa8b_49ba_43b2_956e_3397f9b94c3a);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GPMStarterGPOType(pub i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const typeSystem: GPMStarterGPOType = GPMStarterGPOType(0i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const typeCustom: GPMStarterGPOType = GPMStarterGPOType(1i32);
impl ::core::marker::Copy for GPMStarterGPOType {}
impl ::core::clone::Clone for GPMStarterGPOType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GPMStarterGPOType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for GPMStarterGPOType {
    type Abi = Self;
}
impl ::core::fmt::Debug for GPMStarterGPOType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GPMStarterGPOType").field(&self.0).finish()
    }
}
pub const GPMStatusMessage: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4b77cc94_d255_409b_bc62_370881715a19);
pub const GPMStatusMsgCollection: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2824e4be_4bcc_4cac_9e60_0e3ed7f12496);
pub const GPMTemplate: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xecf1d454_71da_4e2f_a8c0_8185465911d9);
pub const GPMTrustee: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc54a700d_19b6_4211_bcb0_e8e2475e471e);
pub const GPMWMIFilter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x626745d8_0dea_4062_bf60_cfc5b1ca1286);
pub const GPMWMIFilterCollection: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x74dc6d28_e820_47d6_a0b8_f08d93d7fa33);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPM_DONOTUSE_W2KDC: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPM_DONOT_VALIDATEDC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPM_MIGRATIONTABLE_ONLY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPM_PROCESS_SECURITY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPM_USE_ANYDC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPM_USE_PDC: u32 = 0u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct GPOBROWSEINFO {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub lpTitle: ::windows::core::PWSTR,
    pub lpInitialOU: ::windows::core::PWSTR,
    pub lpDSPath: ::windows::core::PWSTR,
    pub dwDSPathSize: u32,
    pub lpName: ::windows::core::PWSTR,
    pub dwNameSize: u32,
    pub gpoType: GROUP_POLICY_OBJECT_TYPE,
    pub gpoHint: GROUP_POLICY_HINT_TYPE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GPOBROWSEINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GPOBROWSEINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for GPOBROWSEINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GPOBROWSEINFO").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).field("hwndOwner", &self.hwndOwner).field("lpTitle", &self.lpTitle).field("lpInitialOU", &self.lpInitialOU).field("lpDSPath", &self.lpDSPath).field("dwDSPathSize", &self.dwDSPathSize).field("lpName", &self.lpName).field("dwNameSize", &self.dwNameSize).field("gpoType", &self.gpoType).field("gpoHint", &self.gpoHint).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for GPOBROWSEINFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GPOBROWSEINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GPOBROWSEINFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GPOBROWSEINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GPOBROWSEINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPO_BROWSE_DISABLENEW: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPO_BROWSE_INITTOALL: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPO_BROWSE_NOCOMPUTERS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPO_BROWSE_NODSGPOS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPO_BROWSE_NOUSERGPOS: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPO_BROWSE_OPENBUTTON: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPO_BROWSE_SENDAPPLYONEDIT: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPO_FLAG_DISABLE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPO_FLAG_FORCE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPO_INFO_FLAG_ASYNC_FOREGROUND: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPO_INFO_FLAG_BACKGROUND: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPO_INFO_FLAG_FORCED_REFRESH: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPO_INFO_FLAG_LINKTRANSITION: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPO_INFO_FLAG_LOGRSOP_TRANSITION: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPO_INFO_FLAG_MACHINE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPO_INFO_FLAG_NOCHANGES: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPO_INFO_FLAG_SAFEMODE_BOOT: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPO_INFO_FLAG_SLOWLINK: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPO_INFO_FLAG_VERBOSE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GPO_LINK(pub i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPLinkUnknown: GPO_LINK = GPO_LINK(0i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPLinkMachine: GPO_LINK = GPO_LINK(1i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPLinkSite: GPO_LINK = GPO_LINK(2i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPLinkDomain: GPO_LINK = GPO_LINK(3i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPLinkOrganizationalUnit: GPO_LINK = GPO_LINK(4i32);
impl ::core::marker::Copy for GPO_LINK {}
impl ::core::clone::Clone for GPO_LINK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GPO_LINK {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for GPO_LINK {
    type Abi = Self;
}
impl ::core::fmt::Debug for GPO_LINK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GPO_LINK").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPO_LIST_FLAG_MACHINE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPO_LIST_FLAG_NO_SECURITYFILTERS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPO_LIST_FLAG_NO_WMIFILTERS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPO_LIST_FLAG_SITEONLY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPO_OPEN_LOAD_REGISTRY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPO_OPEN_READ_ONLY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPO_OPTION_DISABLE_MACHINE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPO_OPTION_DISABLE_USER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPO_SECTION_MACHINE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPO_SECTION_ROOT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPO_SECTION_USER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GP_DLLNAME: &'static str = "DllName";
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GP_ENABLEASYNCHRONOUSPROCESSING: &'static str = "EnableAsynchronousProcessing";
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GP_MAXNOGPOLISTCHANGESINTERVAL: &'static str = "MaxNoGPOListChangesInterval";
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GP_NOBACKGROUNDPOLICY: &'static str = "NoBackgroundPolicy";
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GP_NOGPOLISTCHANGES: &'static str = "NoGPOListChanges";
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GP_NOMACHINEPOLICY: &'static str = "NoMachinePolicy";
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GP_NOSLOWLINK: &'static str = "NoSlowLink";
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GP_NOTIFYLINKTRANSITION: &'static str = "NotifyLinkTransition";
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GP_NOUSERPOLICY: &'static str = "NoUserPolicy";
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GP_PERUSERLOCALSETTINGS: &'static str = "PerUserLocalSettings";
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GP_PROCESSGROUPPOLICY: &'static str = "ProcessGroupPolicy";
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GP_REQUIRESSUCCESSFULREGISTRY: &'static str = "RequiresSuccessfulRegistry";
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GROUP_POLICY_HINT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPHintUnknown: GROUP_POLICY_HINT_TYPE = GROUP_POLICY_HINT_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPHintMachine: GROUP_POLICY_HINT_TYPE = GROUP_POLICY_HINT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPHintSite: GROUP_POLICY_HINT_TYPE = GROUP_POLICY_HINT_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPHintDomain: GROUP_POLICY_HINT_TYPE = GROUP_POLICY_HINT_TYPE(3i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPHintOrganizationalUnit: GROUP_POLICY_HINT_TYPE = GROUP_POLICY_HINT_TYPE(4i32);
impl ::core::marker::Copy for GROUP_POLICY_HINT_TYPE {}
impl ::core::clone::Clone for GROUP_POLICY_HINT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GROUP_POLICY_HINT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for GROUP_POLICY_HINT_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for GROUP_POLICY_HINT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GROUP_POLICY_HINT_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct GROUP_POLICY_OBJECTA {
    pub dwOptions: u32,
    pub dwVersion: u32,
    pub lpDSPath: ::windows::core::PSTR,
    pub lpFileSysPath: ::windows::core::PSTR,
    pub lpDisplayName: ::windows::core::PSTR,
    pub szGPOName: [super::super::Foundation::CHAR; 50],
    pub GPOLink: GPO_LINK,
    pub lParam: super::super::Foundation::LPARAM,
    pub pNext: *mut GROUP_POLICY_OBJECTA,
    pub pPrev: *mut GROUP_POLICY_OBJECTA,
    pub lpExtensions: ::windows::core::PSTR,
    pub lParam2: super::super::Foundation::LPARAM,
    pub lpLink: ::windows::core::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GROUP_POLICY_OBJECTA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GROUP_POLICY_OBJECTA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for GROUP_POLICY_OBJECTA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GROUP_POLICY_OBJECTA")
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
unsafe impl ::windows::core::Abi for GROUP_POLICY_OBJECTA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GROUP_POLICY_OBJECTA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GROUP_POLICY_OBJECTA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GROUP_POLICY_OBJECTA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GROUP_POLICY_OBJECTA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct GROUP_POLICY_OBJECTW {
    pub dwOptions: u32,
    pub dwVersion: u32,
    pub lpDSPath: ::windows::core::PWSTR,
    pub lpFileSysPath: ::windows::core::PWSTR,
    pub lpDisplayName: ::windows::core::PWSTR,
    pub szGPOName: [u16; 50],
    pub GPOLink: GPO_LINK,
    pub lParam: super::super::Foundation::LPARAM,
    pub pNext: *mut GROUP_POLICY_OBJECTW,
    pub pPrev: *mut GROUP_POLICY_OBJECTW,
    pub lpExtensions: ::windows::core::PWSTR,
    pub lParam2: super::super::Foundation::LPARAM,
    pub lpLink: ::windows::core::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GROUP_POLICY_OBJECTW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GROUP_POLICY_OBJECTW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for GROUP_POLICY_OBJECTW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GROUP_POLICY_OBJECTW")
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
unsafe impl ::windows::core::Abi for GROUP_POLICY_OBJECTW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GROUP_POLICY_OBJECTW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GROUP_POLICY_OBJECTW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GROUP_POLICY_OBJECTW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GROUP_POLICY_OBJECTW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GROUP_POLICY_OBJECT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPOTypeLocal: GROUP_POLICY_OBJECT_TYPE = GROUP_POLICY_OBJECT_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPOTypeRemote: GROUP_POLICY_OBJECT_TYPE = GROUP_POLICY_OBJECT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPOTypeDS: GROUP_POLICY_OBJECT_TYPE = GROUP_POLICY_OBJECT_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPOTypeLocalUser: GROUP_POLICY_OBJECT_TYPE = GROUP_POLICY_OBJECT_TYPE(3i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPOTypeLocalGroup: GROUP_POLICY_OBJECT_TYPE = GROUP_POLICY_OBJECT_TYPE(4i32);
impl ::core::marker::Copy for GROUP_POLICY_OBJECT_TYPE {}
impl ::core::clone::Clone for GROUP_POLICY_OBJECT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GROUP_POLICY_OBJECT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for GROUP_POLICY_OBJECT_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for GROUP_POLICY_OBJECT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GROUP_POLICY_OBJECT_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GenerateGPNotification<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(bmachine: Param0, lpwszmgmtproduct: Param1, dwmgmtproductoptions: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GenerateGPNotification(bmachine: super::super::Foundation::BOOL, lpwszmgmtproduct: ::windows::core::PCWSTR, dwmgmtproductoptions: u32) -> u32;
        }
        ::core::mem::transmute(GenerateGPNotification(bmachine.into_param().abi(), lpwszmgmtproduct.into_param().abi(), ::core::mem::transmute(dwmgmtproductoptions)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetAppliedGPOListA<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PSID>>(dwflags: u32, pmachinename: Param1, psiduser: Param2, pguidextension: *const ::windows::core::GUID, ppgpolist: *mut *mut GROUP_POLICY_OBJECTA) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetAppliedGPOListA(dwflags: u32, pmachinename: ::windows::core::PCSTR, psiduser: super::super::Foundation::PSID, pguidextension: *const ::windows::core::GUID, ppgpolist: *mut *mut GROUP_POLICY_OBJECTA) -> u32;
        }
        ::core::mem::transmute(GetAppliedGPOListA(::core::mem::transmute(dwflags), pmachinename.into_param().abi(), psiduser.into_param().abi(), ::core::mem::transmute(pguidextension), ::core::mem::transmute(ppgpolist)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetAppliedGPOListW<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PSID>>(dwflags: u32, pmachinename: Param1, psiduser: Param2, pguidextension: *const ::windows::core::GUID, ppgpolist: *mut *mut GROUP_POLICY_OBJECTW) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetAppliedGPOListW(dwflags: u32, pmachinename: ::windows::core::PCWSTR, psiduser: super::super::Foundation::PSID, pguidextension: *const ::windows::core::GUID, ppgpolist: *mut *mut GROUP_POLICY_OBJECTW) -> u32;
        }
        ::core::mem::transmute(GetAppliedGPOListW(::core::mem::transmute(dwflags), pmachinename.into_param().abi(), psiduser.into_param().abi(), ::core::mem::transmute(pguidextension), ::core::mem::transmute(ppgpolist)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetGPOListA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(htoken: Param0, lpname: Param1, lphostname: Param2, lpcomputername: Param3, dwflags: u32, pgpolist: *mut *mut GROUP_POLICY_OBJECTA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetGPOListA(htoken: super::super::Foundation::HANDLE, lpname: ::windows::core::PCSTR, lphostname: ::windows::core::PCSTR, lpcomputername: ::windows::core::PCSTR, dwflags: u32, pgpolist: *mut *mut GROUP_POLICY_OBJECTA) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetGPOListA(htoken.into_param().abi(), lpname.into_param().abi(), lphostname.into_param().abi(), lpcomputername.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(pgpolist)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetGPOListW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(htoken: Param0, lpname: Param1, lphostname: Param2, lpcomputername: Param3, dwflags: u32, pgpolist: *mut *mut GROUP_POLICY_OBJECTW) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetGPOListW(htoken: super::super::Foundation::HANDLE, lpname: ::windows::core::PCWSTR, lphostname: ::windows::core::PCWSTR, lpcomputername: ::windows::core::PCWSTR, dwflags: u32, pgpolist: *mut *mut GROUP_POLICY_OBJECTW) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetGPOListW(htoken.into_param().abi(), lpname.into_param().abi(), lphostname.into_param().abi(), lpcomputername.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(pgpolist)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
#[inline]
pub unsafe fn GetLocalManagedApplicationData<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(productcode: Param0, displayname: *mut ::windows::core::PWSTR, supporturl: *mut ::windows::core::PWSTR) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetLocalManagedApplicationData(productcode: ::windows::core::PCWSTR, displayname: *mut ::windows::core::PWSTR, supporturl: *mut ::windows::core::PWSTR);
        }
        GetLocalManagedApplicationData(productcode.into_param().abi(), ::core::mem::transmute(displayname), ::core::mem::transmute(supporturl))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetLocalManagedApplications<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(buserapps: Param0, pdwapps: *mut u32, prglocalapps: *mut *mut LOCALMANAGEDAPPLICATION) -> u32 {
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
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_UI_Shell\"`*"]
#[cfg(feature = "Win32_UI_Shell")]
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
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetManagedApplications(pcategory: *const ::windows::core::GUID, dwqueryflags: u32, dwinfolevel: u32, pdwapps: *mut u32, prgmanagedapps: *mut *mut MANAGEDAPPLICATION) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetManagedApplications(pcategory: *const ::windows::core::GUID, dwqueryflags: u32, dwinfolevel: u32, pdwapps: *mut u32, prgmanagedapps: *mut *mut MANAGEDAPPLICATION) -> u32;
        }
        ::core::mem::transmute(GetManagedApplications(::core::mem::transmute(pcategory), ::core::mem::transmute(dwqueryflags), ::core::mem::transmute(dwinfolevel), ::core::mem::transmute(pdwapps), ::core::mem::transmute(prgmanagedapps)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
#[repr(transparent)]
pub struct IGPEInformation(::windows::core::IUnknown);
impl IGPEInformation {
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn GetName(&self, pszname: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetName)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pszname)), pszname.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn GetDisplayName(&self, pszname: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDisplayName)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pszname)), pszname.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Registry\"`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub unsafe fn GetRegistryKey(&self, dwsection: u32, hkey: *mut super::Registry::HKEY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetRegistryKey)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwsection), ::core::mem::transmute(hkey)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn GetDSPath(&self, dwsection: u32, pszpath: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDSPath)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwsection), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pszpath)), pszpath.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn GetFileSysPath(&self, dwsection: u32, pszpath: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetFileSysPath)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwsection), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pszpath)), pszpath.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn GetOptions(&self, dwoptions: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetOptions)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoptions)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn GetType(&self, gpotype: *mut GROUP_POLICY_OBJECT_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetType)(::core::mem::transmute_copy(self), ::core::mem::transmute(gpotype)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn GetHint(&self, gphint: *mut GROUP_POLICY_HINT_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetHint)(::core::mem::transmute_copy(self), ::core::mem::transmute(gphint)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PolicyChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bmachine: Param0, badd: Param1, pguidextension: *mut ::windows::core::GUID, pguidsnapin: *mut ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).PolicyChanged)(::core::mem::transmute_copy(self), bmachine.into_param().abi(), badd.into_param().abi(), ::core::mem::transmute(pguidextension), ::core::mem::transmute(pguidsnapin)).ok()
    }
}
impl ::core::convert::From<IGPEInformation> for ::windows::core::IUnknown {
    fn from(value: IGPEInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IGPEInformation> for ::windows::core::IUnknown {
    fn from(value: &IGPEInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IGPEInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IGPEInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IGPEInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IGPEInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGPEInformation {}
impl ::core::fmt::Debug for IGPEInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGPEInformation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IGPEInformation {
    type Vtable = IGPEInformation_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8fc0b735_a0e1_11d1_a7d3_0000f87571e3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGPEInformation_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszname: ::windows::core::PWSTR, cchmaxlength: i32) -> ::windows::core::HRESULT,
    pub GetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszname: ::windows::core::PWSTR, cchmaxlength: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Registry")]
    pub GetRegistryKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwsection: u32, hkey: *mut super::Registry::HKEY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Registry"))]
    GetRegistryKey: usize,
    pub GetDSPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwsection: u32, pszpath: ::windows::core::PWSTR, cchmaxpath: i32) -> ::windows::core::HRESULT,
    pub GetFileSysPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwsection: u32, pszpath: ::windows::core::PWSTR, cchmaxpath: i32) -> ::windows::core::HRESULT,
    pub GetOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoptions: *mut u32) -> ::windows::core::HRESULT,
    pub GetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gpotype: *mut GROUP_POLICY_OBJECT_TYPE) -> ::windows::core::HRESULT,
    pub GetHint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gphint: *mut GROUP_POLICY_HINT_TYPE) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub PolicyChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bmachine: super::super::Foundation::BOOL, badd: super::super::Foundation::BOOL, pguidextension: *mut ::windows::core::GUID, pguidsnapin: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PolicyChanged: usize,
}
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IGPM(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IGPM {
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetDomain<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrdomain: Param0, bstrdomaincontroller: Param1, ldcflags: i32) -> ::windows::core::Result<IGPMDomain> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDomain)(::core::mem::transmute_copy(self), bstrdomain.into_param().abi(), bstrdomaincontroller.into_param().abi(), ::core::mem::transmute(ldcflags), ::core::mem::transmute(&mut result__)).from_abi::<IGPMDomain>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetBackupDir<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrbackupdir: Param0) -> ::windows::core::Result<IGPMBackupDir> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetBackupDir)(::core::mem::transmute_copy(self), bstrbackupdir.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IGPMBackupDir>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetSitesContainer<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrforest: Param0, bstrdomain: Param1, bstrdomaincontroller: Param2, ldcflags: i32) -> ::windows::core::Result<IGPMSitesContainer> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSitesContainer)(::core::mem::transmute_copy(self), bstrforest.into_param().abi(), bstrdomain.into_param().abi(), bstrdomaincontroller.into_param().abi(), ::core::mem::transmute(ldcflags), ::core::mem::transmute(&mut result__)).from_abi::<IGPMSitesContainer>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetRSOP<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, gpmrsopmode: GPMRSOPMode, bstrnamespace: Param1, lflags: i32) -> ::windows::core::Result<IGPMRSOP> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetRSOP)(::core::mem::transmute_copy(self), ::core::mem::transmute(gpmrsopmode), bstrnamespace.into_param().abi(), ::core::mem::transmute(lflags), ::core::mem::transmute(&mut result__)).from_abi::<IGPMRSOP>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePermission<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrtrustee: Param0, perm: GPMPermissionType, binheritable: i16) -> ::windows::core::Result<IGPMPermission> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreatePermission)(::core::mem::transmute_copy(self), bstrtrustee.into_param().abi(), ::core::mem::transmute(perm), ::core::mem::transmute(binheritable), ::core::mem::transmute(&mut result__)).from_abi::<IGPMPermission>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateSearchCriteria(&self) -> ::windows::core::Result<IGPMSearchCriteria> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateSearchCriteria)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IGPMSearchCriteria>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn CreateTrustee<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrtrustee: Param0) -> ::windows::core::Result<IGPMTrustee> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateTrustee)(::core::mem::transmute_copy(self), bstrtrustee.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IGPMTrustee>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetClientSideExtensions(&self) -> ::windows::core::Result<IGPMCSECollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetClientSideExtensions)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IGPMCSECollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetConstants(&self) -> ::windows::core::Result<IGPMConstants> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetConstants)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IGPMConstants>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetMigrationTable<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrmigrationtablepath: Param0) -> ::windows::core::Result<IGPMMigrationTable> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetMigrationTable)(::core::mem::transmute_copy(self), bstrmigrationtablepath.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IGPMMigrationTable>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateMigrationTable(&self) -> ::windows::core::Result<IGPMMigrationTable> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateMigrationTable)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IGPMMigrationTable>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InitializeReporting<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstradmpath: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).InitializeReporting)(::core::mem::transmute_copy(self), bstradmpath.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGPM> for ::windows::core::IUnknown {
    fn from(value: IGPM) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGPM> for ::windows::core::IUnknown {
    fn from(value: &IGPM) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IGPM {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IGPM {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IGPM {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a IGPM {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IGPM {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IGPM {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IGPM {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IGPM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGPM").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IGPM {
    type Vtable = IGPM_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf5fae809_3bd6_4da9_a65e_17665b41d763);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPM_Vtbl {
    pub base: super::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetDomain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdomain: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdomaincontroller: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ldcflags: i32, pigpmdomain: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetDomain: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetBackupDir: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrbackupdir: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pigpmbackupdir: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetBackupDir: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetSitesContainer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrforest: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdomain: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdomaincontroller: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ldcflags: i32, ppigpmsitescontainer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetSitesContainer: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetRSOP: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gpmrsopmode: GPMRSOPMode, bstrnamespace: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, ppigpmrsop: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetRSOP: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub CreatePermission: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrtrustee: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, perm: GPMPermissionType, binheritable: i16, ppperm: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    CreatePermission: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateSearchCriteria: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppigpmsearchcriteria: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateSearchCriteria: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub CreateTrustee: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrtrustee: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppigpmtrustee: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    CreateTrustee: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetClientSideExtensions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppigpmcsecollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetClientSideExtensions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetConstants: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppigpmconstants: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetConstants: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetMigrationTable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrmigrationtablepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppmigrationtable: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetMigrationTable: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateMigrationTable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppmigrationtable: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateMigrationTable: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub InitializeReporting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstradmpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InitializeReporting: usize,
}
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IGPM2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IGPM2 {
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetDomain<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrdomain: Param0, bstrdomaincontroller: Param1, ldcflags: i32) -> ::windows::core::Result<IGPMDomain> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetDomain)(::core::mem::transmute_copy(self), bstrdomain.into_param().abi(), bstrdomaincontroller.into_param().abi(), ::core::mem::transmute(ldcflags), ::core::mem::transmute(&mut result__)).from_abi::<IGPMDomain>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetBackupDir<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrbackupdir: Param0) -> ::windows::core::Result<IGPMBackupDir> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetBackupDir)(::core::mem::transmute_copy(self), bstrbackupdir.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IGPMBackupDir>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetSitesContainer<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrforest: Param0, bstrdomain: Param1, bstrdomaincontroller: Param2, ldcflags: i32) -> ::windows::core::Result<IGPMSitesContainer> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetSitesContainer)(::core::mem::transmute_copy(self), bstrforest.into_param().abi(), bstrdomain.into_param().abi(), bstrdomaincontroller.into_param().abi(), ::core::mem::transmute(ldcflags), ::core::mem::transmute(&mut result__)).from_abi::<IGPMSitesContainer>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetRSOP<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, gpmrsopmode: GPMRSOPMode, bstrnamespace: Param1, lflags: i32) -> ::windows::core::Result<IGPMRSOP> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetRSOP)(::core::mem::transmute_copy(self), ::core::mem::transmute(gpmrsopmode), bstrnamespace.into_param().abi(), ::core::mem::transmute(lflags), ::core::mem::transmute(&mut result__)).from_abi::<IGPMRSOP>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePermission<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrtrustee: Param0, perm: GPMPermissionType, binheritable: i16) -> ::windows::core::Result<IGPMPermission> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreatePermission)(::core::mem::transmute_copy(self), bstrtrustee.into_param().abi(), ::core::mem::transmute(perm), ::core::mem::transmute(binheritable), ::core::mem::transmute(&mut result__)).from_abi::<IGPMPermission>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateSearchCriteria(&self) -> ::windows::core::Result<IGPMSearchCriteria> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateSearchCriteria)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IGPMSearchCriteria>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn CreateTrustee<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrtrustee: Param0) -> ::windows::core::Result<IGPMTrustee> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateTrustee)(::core::mem::transmute_copy(self), bstrtrustee.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IGPMTrustee>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetClientSideExtensions(&self) -> ::windows::core::Result<IGPMCSECollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetClientSideExtensions)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IGPMCSECollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetConstants(&self) -> ::windows::core::Result<IGPMConstants> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetConstants)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IGPMConstants>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetMigrationTable<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrmigrationtablepath: Param0) -> ::windows::core::Result<IGPMMigrationTable> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetMigrationTable)(::core::mem::transmute_copy(self), bstrmigrationtablepath.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IGPMMigrationTable>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateMigrationTable(&self) -> ::windows::core::Result<IGPMMigrationTable> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateMigrationTable)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IGPMMigrationTable>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InitializeReporting<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstradmpath: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.InitializeReporting)(::core::mem::transmute_copy(self), bstradmpath.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetBackupDirEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrbackupdir: Param0, backupdirtype: GPMBackupType) -> ::windows::core::Result<IGPMBackupDirEx> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetBackupDirEx)(::core::mem::transmute_copy(self), bstrbackupdir.into_param().abi(), ::core::mem::transmute(backupdirtype), ::core::mem::transmute(&mut result__)).from_abi::<IGPMBackupDirEx>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InitializeReportingEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstradmpath: Param0, reportingoptions: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).InitializeReportingEx)(::core::mem::transmute_copy(self), bstradmpath.into_param().abi(), ::core::mem::transmute(reportingoptions)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGPM2> for ::windows::core::IUnknown {
    fn from(value: IGPM2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGPM2> for ::windows::core::IUnknown {
    fn from(value: &IGPM2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IGPM2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IGPM2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IGPM2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a IGPM2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGPM2> for IGPM {
    fn from(value: IGPM2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGPM2> for IGPM {
    fn from(value: &IGPM2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IGPM> for IGPM2 {
    fn into_param(self) -> ::windows::core::Param<'a, IGPM> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IGPM> for &'a IGPM2 {
    fn into_param(self) -> ::windows::core::Param<'a, IGPM> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IGPM2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IGPM2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IGPM2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IGPM2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGPM2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IGPM2 {
    type Vtable = IGPM2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00238f8a_3d86_41ac_8f5e_06a6638a634a);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPM2_Vtbl {
    pub base: IGPM_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetBackupDirEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrbackupdir: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, backupdirtype: GPMBackupType, ppigpmbackupdirex: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetBackupDirEx: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub InitializeReportingEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstradmpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, reportingoptions: i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InitializeReportingEx: usize,
}
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IGPMAsyncCancel(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IGPMAsyncCancel {
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn Cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Cancel)(::core::mem::transmute_copy(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGPMAsyncCancel> for ::windows::core::IUnknown {
    fn from(value: IGPMAsyncCancel) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGPMAsyncCancel> for ::windows::core::IUnknown {
    fn from(value: &IGPMAsyncCancel) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IGPMAsyncCancel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IGPMAsyncCancel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IGPMAsyncCancel {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a IGPMAsyncCancel {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IGPMAsyncCancel {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IGPMAsyncCancel {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IGPMAsyncCancel {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IGPMAsyncCancel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGPMAsyncCancel").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IGPMAsyncCancel {
    type Vtable = IGPMAsyncCancel_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xddc67754_be67_4541_8166_f48166868c9c);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMAsyncCancel_Vtbl {
    pub base: super::Com::IDispatch_Vtbl,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IGPMAsyncProgress(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IGPMAsyncProgress {
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Status<'a, Param4: ::windows::core::IntoParam<'a, IGPMStatusMsgCollection>>(&self, lprogressnumerator: i32, lprogressdenominator: i32, hrstatus: ::windows::core::HRESULT, presult: *const super::Com::VARIANT, ppigpmstatusmsgcollection: Param4) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Status)(::core::mem::transmute_copy(self), ::core::mem::transmute(lprogressnumerator), ::core::mem::transmute(lprogressdenominator), ::core::mem::transmute(hrstatus), ::core::mem::transmute(presult), ppigpmstatusmsgcollection.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGPMAsyncProgress> for ::windows::core::IUnknown {
    fn from(value: IGPMAsyncProgress) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGPMAsyncProgress> for ::windows::core::IUnknown {
    fn from(value: &IGPMAsyncProgress) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IGPMAsyncProgress {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IGPMAsyncProgress {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IGPMAsyncProgress {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a IGPMAsyncProgress {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IGPMAsyncProgress {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IGPMAsyncProgress {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IGPMAsyncProgress {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IGPMAsyncProgress {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGPMAsyncProgress").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IGPMAsyncProgress {
    type Vtable = IGPMAsyncProgress_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6aac29f8_5948_4324_bf70_423818942dbc);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMAsyncProgress_Vtbl {
    pub base: super::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lprogressnumerator: i32, lprogressdenominator: i32, hrstatus: ::windows::core::HRESULT, presult: *const super::Com::VARIANT, ppigpmstatusmsgcollection: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Status: usize,
}
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IGPMBackup(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IGPMBackup {
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ID)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GPOID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GPOID)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GPODomain(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GPODomain)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GPODisplayName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GPODisplayName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn Timestamp(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Timestamp)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Comment(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Comment)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BackupDir(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).BackupDir)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Delete)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GenerateReport(&self, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GenerateReport)(::core::mem::transmute_copy(self), ::core::mem::transmute(gpmreporttype), ::core::mem::transmute(pvargpmprogress), ::core::mem::transmute(pvargpmcancel), ::core::mem::transmute(ppigpmresult)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GenerateReportToFile<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, gpmreporttype: GPMReportType, bstrtargetfilepath: Param1) -> ::windows::core::Result<IGPMResult> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GenerateReportToFile)(::core::mem::transmute_copy(self), ::core::mem::transmute(gpmreporttype), bstrtargetfilepath.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IGPMResult>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGPMBackup> for ::windows::core::IUnknown {
    fn from(value: IGPMBackup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGPMBackup> for ::windows::core::IUnknown {
    fn from(value: &IGPMBackup) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IGPMBackup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IGPMBackup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IGPMBackup {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a IGPMBackup {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IGPMBackup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IGPMBackup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IGPMBackup {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IGPMBackup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGPMBackup").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IGPMBackup {
    type Vtable = IGPMBackup_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd8a16a35_3b0d_416b_8d02_4df6f95a7119);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMBackup_Vtbl {
    pub base: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub ID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GPOID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GPOID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GPODomain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GPODomain: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GPODisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GPODisplayName: usize,
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Comment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Comment: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub BackupDir: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    BackupDir: usize,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GenerateReport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GenerateReport: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GenerateReportToFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gpmreporttype: GPMReportType, bstrtargetfilepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppigpmresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GenerateReportToFile: usize,
}
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IGPMBackupCollection(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IGPMBackupCollection {
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Count)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Item(&self, lindex: i32) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Item)(::core::mem::transmute_copy(self), ::core::mem::transmute(lindex), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<super::Ole::IEnumVARIANT> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self)._NewEnum)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Ole::IEnumVARIANT>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGPMBackupCollection> for ::windows::core::IUnknown {
    fn from(value: IGPMBackupCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGPMBackupCollection> for ::windows::core::IUnknown {
    fn from(value: &IGPMBackupCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IGPMBackupCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IGPMBackupCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IGPMBackupCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a IGPMBackupCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IGPMBackupCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IGPMBackupCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IGPMBackupCollection {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IGPMBackupCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGPMBackupCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IGPMBackupCollection {
    type Vtable = IGPMBackupCollection_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc786fc0f_26d8_4bab_a745_39ca7e800cac);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMBackupCollection_Vtbl {
    pub base: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lindex: i32, pval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Item: usize,
    #[cfg(feature = "Win32_System_Ole")]
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppigpmbackup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    _NewEnum: usize,
}
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IGPMBackupDir(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IGPMBackupDir {
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BackupDirectory(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).BackupDirectory)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetBackup<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrid: Param0) -> ::windows::core::Result<IGPMBackup> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetBackup)(::core::mem::transmute_copy(self), bstrid.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IGPMBackup>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SearchBackups<'a, Param0: ::windows::core::IntoParam<'a, IGPMSearchCriteria>>(&self, pigpmsearchcriteria: Param0) -> ::windows::core::Result<IGPMBackupCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SearchBackups)(::core::mem::transmute_copy(self), pigpmsearchcriteria.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IGPMBackupCollection>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGPMBackupDir> for ::windows::core::IUnknown {
    fn from(value: IGPMBackupDir) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGPMBackupDir> for ::windows::core::IUnknown {
    fn from(value: &IGPMBackupDir) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IGPMBackupDir {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IGPMBackupDir {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IGPMBackupDir {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a IGPMBackupDir {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IGPMBackupDir {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IGPMBackupDir {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IGPMBackupDir {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IGPMBackupDir {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGPMBackupDir").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IGPMBackupDir {
    type Vtable = IGPMBackupDir_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb1568bed_0a93_4acc_810f_afe7081019b9);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMBackupDir_Vtbl {
    pub base: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub BackupDirectory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    BackupDirectory: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetBackup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppbackup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetBackup: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SearchBackups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pigpmsearchcriteria: ::windows::core::RawPtr, ppigpmbackupcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SearchBackups: usize,
}
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IGPMBackupDirEx(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IGPMBackupDirEx {
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BackupDir(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).BackupDir)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn BackupType(&self) -> ::windows::core::Result<GPMBackupType> {
        let mut result__: GPMBackupType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).BackupType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMBackupType>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetBackup<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrid: Param0) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetBackup)(::core::mem::transmute_copy(self), bstrid.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SearchBackups<'a, Param0: ::windows::core::IntoParam<'a, IGPMSearchCriteria>>(&self, pigpmsearchcriteria: Param0) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SearchBackups)(::core::mem::transmute_copy(self), pigpmsearchcriteria.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::VARIANT>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGPMBackupDirEx> for ::windows::core::IUnknown {
    fn from(value: IGPMBackupDirEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGPMBackupDirEx> for ::windows::core::IUnknown {
    fn from(value: &IGPMBackupDirEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IGPMBackupDirEx {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IGPMBackupDirEx {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IGPMBackupDirEx {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a IGPMBackupDirEx {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IGPMBackupDirEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IGPMBackupDirEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IGPMBackupDirEx {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IGPMBackupDirEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGPMBackupDirEx").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IGPMBackupDirEx {
    type Vtable = IGPMBackupDirEx_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf8dc55ed_3ba0_4864_aad4_d365189ee1d5);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMBackupDirEx_Vtbl {
    pub base: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub BackupDir: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrbackupdir: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    BackupDir: usize,
    pub BackupType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pgpmbackuptype: *mut GPMBackupType) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetBackup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarbackup: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetBackup: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SearchBackups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pigpmsearchcriteria: ::windows::core::RawPtr, pvarbackupcollection: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SearchBackups: usize,
}
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IGPMCSECollection(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IGPMCSECollection {
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Count)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Item(&self, lindex: i32) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Item)(::core::mem::transmute_copy(self), ::core::mem::transmute(lindex), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<super::Ole::IEnumVARIANT> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self)._NewEnum)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Ole::IEnumVARIANT>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGPMCSECollection> for ::windows::core::IUnknown {
    fn from(value: IGPMCSECollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGPMCSECollection> for ::windows::core::IUnknown {
    fn from(value: &IGPMCSECollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IGPMCSECollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IGPMCSECollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IGPMCSECollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a IGPMCSECollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IGPMCSECollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IGPMCSECollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IGPMCSECollection {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IGPMCSECollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGPMCSECollection").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IGPMCSECollection {
    type Vtable = IGPMCSECollection_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2e52a97d_0a4a_4a6f_85db_201622455da0);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMCSECollection_Vtbl {
    pub base: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lindex: i32, pval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Item: usize,
    #[cfg(feature = "Win32_System_Ole")]
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppigpmcses: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    _NewEnum: usize,
}
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IGPMClientSideExtension(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IGPMClientSideExtension {
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ID)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DisplayName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).DisplayName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn IsUserEnabled(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).IsUserEnabled)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn IsComputerEnabled(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).IsComputerEnabled)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGPMClientSideExtension> for ::windows::core::IUnknown {
    fn from(value: IGPMClientSideExtension) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGPMClientSideExtension> for ::windows::core::IUnknown {
    fn from(value: &IGPMClientSideExtension) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IGPMClientSideExtension {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IGPMClientSideExtension {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IGPMClientSideExtension {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a IGPMClientSideExtension {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IGPMClientSideExtension {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IGPMClientSideExtension {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IGPMClientSideExtension {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IGPMClientSideExtension {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGPMClientSideExtension").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IGPMClientSideExtension {
    type Vtable = IGPMClientSideExtension_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x69da7488_b8db_415e_9266_901be4d49928);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMClientSideExtension_Vtbl {
    pub base: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub ID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DisplayName: usize,
    pub IsUserEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvbenabled: *mut i16) -> ::windows::core::HRESULT,
    pub IsComputerEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvbenabled: *mut i16) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IGPMConstants(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IGPMConstants {
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn PermGPOApply(&self) -> ::windows::core::Result<GPMPermissionType> {
        let mut result__: GPMPermissionType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).PermGPOApply)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMPermissionType>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn PermGPORead(&self) -> ::windows::core::Result<GPMPermissionType> {
        let mut result__: GPMPermissionType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).PermGPORead)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMPermissionType>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn PermGPOEdit(&self) -> ::windows::core::Result<GPMPermissionType> {
        let mut result__: GPMPermissionType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).PermGPOEdit)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMPermissionType>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn PermGPOEditSecurityAndDelete(&self) -> ::windows::core::Result<GPMPermissionType> {
        let mut result__: GPMPermissionType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).PermGPOEditSecurityAndDelete)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMPermissionType>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn PermGPOCustom(&self) -> ::windows::core::Result<GPMPermissionType> {
        let mut result__: GPMPermissionType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).PermGPOCustom)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMPermissionType>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn PermWMIFilterEdit(&self) -> ::windows::core::Result<GPMPermissionType> {
        let mut result__: GPMPermissionType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).PermWMIFilterEdit)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMPermissionType>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn PermWMIFilterFullControl(&self) -> ::windows::core::Result<GPMPermissionType> {
        let mut result__: GPMPermissionType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).PermWMIFilterFullControl)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMPermissionType>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn PermWMIFilterCustom(&self) -> ::windows::core::Result<GPMPermissionType> {
        let mut result__: GPMPermissionType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).PermWMIFilterCustom)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMPermissionType>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn PermSOMLink(&self) -> ::windows::core::Result<GPMPermissionType> {
        let mut result__: GPMPermissionType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).PermSOMLink)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMPermissionType>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn PermSOMLogging(&self) -> ::windows::core::Result<GPMPermissionType> {
        let mut result__: GPMPermissionType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).PermSOMLogging)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMPermissionType>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn PermSOMPlanning(&self) -> ::windows::core::Result<GPMPermissionType> {
        let mut result__: GPMPermissionType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).PermSOMPlanning)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMPermissionType>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn PermSOMGPOCreate(&self) -> ::windows::core::Result<GPMPermissionType> {
        let mut result__: GPMPermissionType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).PermSOMGPOCreate)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMPermissionType>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn PermSOMWMICreate(&self) -> ::windows::core::Result<GPMPermissionType> {
        let mut result__: GPMPermissionType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).PermSOMWMICreate)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMPermissionType>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn PermSOMWMIFullControl(&self) -> ::windows::core::Result<GPMPermissionType> {
        let mut result__: GPMPermissionType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).PermSOMWMIFullControl)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMPermissionType>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn SearchPropertyGPOPermissions(&self) -> ::windows::core::Result<GPMSearchProperty> {
        let mut result__: GPMSearchProperty = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SearchPropertyGPOPermissions)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMSearchProperty>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn SearchPropertyGPOEffectivePermissions(&self) -> ::windows::core::Result<GPMSearchProperty> {
        let mut result__: GPMSearchProperty = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SearchPropertyGPOEffectivePermissions)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMSearchProperty>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn SearchPropertyGPODisplayName(&self) -> ::windows::core::Result<GPMSearchProperty> {
        let mut result__: GPMSearchProperty = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SearchPropertyGPODisplayName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMSearchProperty>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn SearchPropertyGPOWMIFilter(&self) -> ::windows::core::Result<GPMSearchProperty> {
        let mut result__: GPMSearchProperty = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SearchPropertyGPOWMIFilter)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMSearchProperty>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn SearchPropertyGPOID(&self) -> ::windows::core::Result<GPMSearchProperty> {
        let mut result__: GPMSearchProperty = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SearchPropertyGPOID)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMSearchProperty>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn SearchPropertyGPOComputerExtensions(&self) -> ::windows::core::Result<GPMSearchProperty> {
        let mut result__: GPMSearchProperty = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SearchPropertyGPOComputerExtensions)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMSearchProperty>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn SearchPropertyGPOUserExtensions(&self) -> ::windows::core::Result<GPMSearchProperty> {
        let mut result__: GPMSearchProperty = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SearchPropertyGPOUserExtensions)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMSearchProperty>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn SearchPropertySOMLinks(&self) -> ::windows::core::Result<GPMSearchProperty> {
        let mut result__: GPMSearchProperty = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SearchPropertySOMLinks)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMSearchProperty>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn SearchPropertyGPODomain(&self) -> ::windows::core::Result<GPMSearchProperty> {
        let mut result__: GPMSearchProperty = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SearchPropertyGPODomain)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMSearchProperty>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn SearchPropertyBackupMostRecent(&self) -> ::windows::core::Result<GPMSearchProperty> {
        let mut result__: GPMSearchProperty = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SearchPropertyBackupMostRecent)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMSearchProperty>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn SearchOpEquals(&self) -> ::windows::core::Result<GPMSearchOperation> {
        let mut result__: GPMSearchOperation = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SearchOpEquals)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMSearchOperation>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn SearchOpContains(&self) -> ::windows::core::Result<GPMSearchOperation> {
        let mut result__: GPMSearchOperation = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SearchOpContains)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMSearchOperation>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn SearchOpNotContains(&self) -> ::windows::core::Result<GPMSearchOperation> {
        let mut result__: GPMSearchOperation = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SearchOpNotContains)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMSearchOperation>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn SearchOpNotEquals(&self) -> ::windows::core::Result<GPMSearchOperation> {
        let mut result__: GPMSearchOperation = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SearchOpNotEquals)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMSearchOperation>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn UsePDC(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).UsePDC)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn UseAnyDC(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).UseAnyDC)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn DoNotUseW2KDC(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).DoNotUseW2KDC)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn SOMSite(&self) -> ::windows::core::Result<GPMSOMType> {
        let mut result__: GPMSOMType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SOMSite)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMSOMType>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn SOMDomain(&self) -> ::windows::core::Result<GPMSOMType> {
        let mut result__: GPMSOMType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SOMDomain)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMSOMType>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn SOMOU(&self) -> ::windows::core::Result<GPMSOMType> {
        let mut result__: GPMSOMType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SOMOU)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMSOMType>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn SecurityFlags(&self, vbowner: i16, vbgroup: i16, vbdacl: i16, vbsacl: i16) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SecurityFlags)(::core::mem::transmute_copy(self), ::core::mem::transmute(vbowner), ::core::mem::transmute(vbgroup), ::core::mem::transmute(vbdacl), ::core::mem::transmute(vbsacl), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn DoNotValidateDC(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).DoNotValidateDC)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn ReportHTML(&self) -> ::windows::core::Result<GPMReportType> {
        let mut result__: GPMReportType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ReportHTML)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMReportType>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn ReportXML(&self) -> ::windows::core::Result<GPMReportType> {
        let mut result__: GPMReportType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ReportXML)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMReportType>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn RSOPModeUnknown(&self) -> ::windows::core::Result<GPMRSOPMode> {
        let mut result__: GPMRSOPMode = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).RSOPModeUnknown)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMRSOPMode>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn RSOPModePlanning(&self) -> ::windows::core::Result<GPMRSOPMode> {
        let mut result__: GPMRSOPMode = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).RSOPModePlanning)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMRSOPMode>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn RSOPModeLogging(&self) -> ::windows::core::Result<GPMRSOPMode> {
        let mut result__: GPMRSOPMode = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).RSOPModeLogging)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMRSOPMode>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn EntryTypeUser(&self) -> ::windows::core::Result<GPMEntryType> {
        let mut result__: GPMEntryType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).EntryTypeUser)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMEntryType>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn EntryTypeComputer(&self) -> ::windows::core::Result<GPMEntryType> {
        let mut result__: GPMEntryType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).EntryTypeComputer)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMEntryType>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn EntryTypeLocalGroup(&self) -> ::windows::core::Result<GPMEntryType> {
        let mut result__: GPMEntryType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).EntryTypeLocalGroup)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMEntryType>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn EntryTypeGlobalGroup(&self) -> ::windows::core::Result<GPMEntryType> {
        let mut result__: GPMEntryType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).EntryTypeGlobalGroup)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMEntryType>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn EntryTypeUniversalGroup(&self) -> ::windows::core::Result<GPMEntryType> {
        let mut result__: GPMEntryType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).EntryTypeUniversalGroup)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMEntryType>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn EntryTypeUNCPath(&self) -> ::windows::core::Result<GPMEntryType> {
        let mut result__: GPMEntryType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).EntryTypeUNCPath)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMEntryType>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn EntryTypeUnknown(&self) -> ::windows::core::Result<GPMEntryType> {
        let mut result__: GPMEntryType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).EntryTypeUnknown)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMEntryType>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn DestinationOptionSameAsSource(&self) -> ::windows::core::Result<GPMDestinationOption> {
        let mut result__: GPMDestinationOption = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).DestinationOptionSameAsSource)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMDestinationOption>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn DestinationOptionNone(&self) -> ::windows::core::Result<GPMDestinationOption> {
        let mut result__: GPMDestinationOption = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).DestinationOptionNone)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMDestinationOption>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn DestinationOptionByRelativeName(&self) -> ::windows::core::Result<GPMDestinationOption> {
        let mut result__: GPMDestinationOption = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).DestinationOptionByRelativeName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMDestinationOption>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn DestinationOptionSet(&self) -> ::windows::core::Result<GPMDestinationOption> {
        let mut result__: GPMDestinationOption = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).DestinationOptionSet)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMDestinationOption>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn MigrationTableOnly(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).MigrationTableOnly)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn ProcessSecurity(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ProcessSecurity)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn RsopLoggingNoComputer(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).RsopLoggingNoComputer)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn RsopLoggingNoUser(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).RsopLoggingNoUser)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn RsopPlanningAssumeSlowLink(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).RsopPlanningAssumeSlowLink)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn RsopPlanningLoopbackOption(&self, vbmerge: i16) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).RsopPlanningLoopbackOption)(::core::mem::transmute_copy(self), ::core::mem::transmute(vbmerge), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn RsopPlanningAssumeUserWQLFilterTrue(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).RsopPlanningAssumeUserWQLFilterTrue)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn RsopPlanningAssumeCompWQLFilterTrue(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).RsopPlanningAssumeCompWQLFilterTrue)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGPMConstants> for ::windows::core::IUnknown {
    fn from(value: IGPMConstants) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGPMConstants> for ::windows::core::IUnknown {
    fn from(value: &IGPMConstants) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IGPMConstants {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IGPMConstants {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IGPMConstants {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a IGPMConstants {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IGPMConstants {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IGPMConstants {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IGPMConstants {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IGPMConstants {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGPMConstants").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IGPMConstants {
    type Vtable = IGPMConstants_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x50ef73e6_d35c_4c8d_be63_7ea5d2aac5c4);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMConstants_Vtbl {
    pub base: super::Com::IDispatch_Vtbl,
    pub PermGPOApply: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT,
    pub PermGPORead: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT,
    pub PermGPOEdit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT,
    pub PermGPOEditSecurityAndDelete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT,
    pub PermGPOCustom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT,
    pub PermWMIFilterEdit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT,
    pub PermWMIFilterFullControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT,
    pub PermWMIFilterCustom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT,
    pub PermSOMLink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT,
    pub PermSOMLogging: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT,
    pub PermSOMPlanning: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT,
    pub PermSOMGPOCreate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT,
    pub PermSOMWMICreate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT,
    pub PermSOMWMIFullControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT,
    pub SearchPropertyGPOPermissions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows::core::HRESULT,
    pub SearchPropertyGPOEffectivePermissions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows::core::HRESULT,
    pub SearchPropertyGPODisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows::core::HRESULT,
    pub SearchPropertyGPOWMIFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows::core::HRESULT,
    pub SearchPropertyGPOID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows::core::HRESULT,
    pub SearchPropertyGPOComputerExtensions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows::core::HRESULT,
    pub SearchPropertyGPOUserExtensions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows::core::HRESULT,
    pub SearchPropertySOMLinks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows::core::HRESULT,
    pub SearchPropertyGPODomain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows::core::HRESULT,
    pub SearchPropertyBackupMostRecent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows::core::HRESULT,
    pub SearchOpEquals: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchOperation) -> ::windows::core::HRESULT,
    pub SearchOpContains: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchOperation) -> ::windows::core::HRESULT,
    pub SearchOpNotContains: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchOperation) -> ::windows::core::HRESULT,
    pub SearchOpNotEquals: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchOperation) -> ::windows::core::HRESULT,
    pub UsePDC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT,
    pub UseAnyDC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT,
    pub DoNotUseW2KDC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT,
    pub SOMSite: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMSOMType) -> ::windows::core::HRESULT,
    pub SOMDomain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMSOMType) -> ::windows::core::HRESULT,
    pub SOMOU: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMSOMType) -> ::windows::core::HRESULT,
    pub SecurityFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vbowner: i16, vbgroup: i16, vbdacl: i16, vbsacl: i16, pval: *mut i32) -> ::windows::core::HRESULT,
    pub DoNotValidateDC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT,
    pub ReportHTML: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMReportType) -> ::windows::core::HRESULT,
    pub ReportXML: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMReportType) -> ::windows::core::HRESULT,
    pub RSOPModeUnknown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMRSOPMode) -> ::windows::core::HRESULT,
    pub RSOPModePlanning: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMRSOPMode) -> ::windows::core::HRESULT,
    pub RSOPModeLogging: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMRSOPMode) -> ::windows::core::HRESULT,
    pub EntryTypeUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMEntryType) -> ::windows::core::HRESULT,
    pub EntryTypeComputer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMEntryType) -> ::windows::core::HRESULT,
    pub EntryTypeLocalGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMEntryType) -> ::windows::core::HRESULT,
    pub EntryTypeGlobalGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMEntryType) -> ::windows::core::HRESULT,
    pub EntryTypeUniversalGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMEntryType) -> ::windows::core::HRESULT,
    pub EntryTypeUNCPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMEntryType) -> ::windows::core::HRESULT,
    pub EntryTypeUnknown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMEntryType) -> ::windows::core::HRESULT,
    pub DestinationOptionSameAsSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMDestinationOption) -> ::windows::core::HRESULT,
    pub DestinationOptionNone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMDestinationOption) -> ::windows::core::HRESULT,
    pub DestinationOptionByRelativeName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMDestinationOption) -> ::windows::core::HRESULT,
    pub DestinationOptionSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMDestinationOption) -> ::windows::core::HRESULT,
    pub MigrationTableOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT,
    pub ProcessSecurity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT,
    pub RsopLoggingNoComputer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT,
    pub RsopLoggingNoUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT,
    pub RsopPlanningAssumeSlowLink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT,
    pub RsopPlanningLoopbackOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vbmerge: i16, pval: *mut i32) -> ::windows::core::HRESULT,
    pub RsopPlanningAssumeUserWQLFilterTrue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT,
    pub RsopPlanningAssumeCompWQLFilterTrue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IGPMConstants2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IGPMConstants2 {
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn PermGPOApply(&self) -> ::windows::core::Result<GPMPermissionType> {
        let mut result__: GPMPermissionType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.PermGPOApply)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMPermissionType>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn PermGPORead(&self) -> ::windows::core::Result<GPMPermissionType> {
        let mut result__: GPMPermissionType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.PermGPORead)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMPermissionType>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn PermGPOEdit(&self) -> ::windows::core::Result<GPMPermissionType> {
        let mut result__: GPMPermissionType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.PermGPOEdit)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMPermissionType>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn PermGPOEditSecurityAndDelete(&self) -> ::windows::core::Result<GPMPermissionType> {
        let mut result__: GPMPermissionType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.PermGPOEditSecurityAndDelete)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMPermissionType>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn PermGPOCustom(&self) -> ::windows::core::Result<GPMPermissionType> {
        let mut result__: GPMPermissionType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.PermGPOCustom)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMPermissionType>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn PermWMIFilterEdit(&self) -> ::windows::core::Result<GPMPermissionType> {
        let mut result__: GPMPermissionType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.PermWMIFilterEdit)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMPermissionType>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn PermWMIFilterFullControl(&self) -> ::windows::core::Result<GPMPermissionType> {
        let mut result__: GPMPermissionType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.PermWMIFilterFullControl)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMPermissionType>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn PermWMIFilterCustom(&self) -> ::windows::core::Result<GPMPermissionType> {
        let mut result__: GPMPermissionType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.PermWMIFilterCustom)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMPermissionType>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn PermSOMLink(&self) -> ::windows::core::Result<GPMPermissionType> {
        let mut result__: GPMPermissionType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.PermSOMLink)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMPermissionType>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn PermSOMLogging(&self) -> ::windows::core::Result<GPMPermissionType> {
        let mut result__: GPMPermissionType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.PermSOMLogging)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMPermissionType>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn PermSOMPlanning(&self) -> ::windows::core::Result<GPMPermissionType> {
        let mut result__: GPMPermissionType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.PermSOMPlanning)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMPermissionType>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn PermSOMGPOCreate(&self) -> ::windows::core::Result<GPMPermissionType> {
        let mut result__: GPMPermissionType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.PermSOMGPOCreate)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMPermissionType>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn PermSOMWMICreate(&self) -> ::windows::core::Result<GPMPermissionType> {
        let mut result__: GPMPermissionType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.PermSOMWMICreate)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMPermissionType>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn PermSOMWMIFullControl(&self) -> ::windows::core::Result<GPMPermissionType> {
        let mut result__: GPMPermissionType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.PermSOMWMIFullControl)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMPermissionType>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn SearchPropertyGPOPermissions(&self) -> ::windows::core::Result<GPMSearchProperty> {
        let mut result__: GPMSearchProperty = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.SearchPropertyGPOPermissions)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMSearchProperty>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn SearchPropertyGPOEffectivePermissions(&self) -> ::windows::core::Result<GPMSearchProperty> {
        let mut result__: GPMSearchProperty = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.SearchPropertyGPOEffectivePermissions)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMSearchProperty>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn SearchPropertyGPODisplayName(&self) -> ::windows::core::Result<GPMSearchProperty> {
        let mut result__: GPMSearchProperty = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.SearchPropertyGPODisplayName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMSearchProperty>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn SearchPropertyGPOWMIFilter(&self) -> ::windows::core::Result<GPMSearchProperty> {
        let mut result__: GPMSearchProperty = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.SearchPropertyGPOWMIFilter)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMSearchProperty>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn SearchPropertyGPOID(&self) -> ::windows::core::Result<GPMSearchProperty> {
        let mut result__: GPMSearchProperty = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.SearchPropertyGPOID)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMSearchProperty>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn SearchPropertyGPOComputerExtensions(&self) -> ::windows::core::Result<GPMSearchProperty> {
        let mut result__: GPMSearchProperty = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.SearchPropertyGPOComputerExtensions)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMSearchProperty>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn SearchPropertyGPOUserExtensions(&self) -> ::windows::core::Result<GPMSearchProperty> {
        let mut result__: GPMSearchProperty = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.SearchPropertyGPOUserExtensions)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMSearchProperty>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn SearchPropertySOMLinks(&self) -> ::windows::core::Result<GPMSearchProperty> {
        let mut result__: GPMSearchProperty = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.SearchPropertySOMLinks)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMSearchProperty>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn SearchPropertyGPODomain(&self) -> ::windows::core::Result<GPMSearchProperty> {
        let mut result__: GPMSearchProperty = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.SearchPropertyGPODomain)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMSearchProperty>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn SearchPropertyBackupMostRecent(&self) -> ::windows::core::Result<GPMSearchProperty> {
        let mut result__: GPMSearchProperty = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.SearchPropertyBackupMostRecent)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMSearchProperty>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn SearchOpEquals(&self) -> ::windows::core::Result<GPMSearchOperation> {
        let mut result__: GPMSearchOperation = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.SearchOpEquals)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMSearchOperation>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn SearchOpContains(&self) -> ::windows::core::Result<GPMSearchOperation> {
        let mut result__: GPMSearchOperation = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.SearchOpContains)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMSearchOperation>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn SearchOpNotContains(&self) -> ::windows::core::Result<GPMSearchOperation> {
        let mut result__: GPMSearchOperation = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.SearchOpNotContains)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMSearchOperation>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn SearchOpNotEquals(&self) -> ::windows::core::Result<GPMSearchOperation> {
        let mut result__: GPMSearchOperation = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.SearchOpNotEquals)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMSearchOperation>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn UsePDC(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.UsePDC)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn UseAnyDC(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.UseAnyDC)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn DoNotUseW2KDC(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.DoNotUseW2KDC)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn SOMSite(&self) -> ::windows::core::Result<GPMSOMType> {
        let mut result__: GPMSOMType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.SOMSite)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMSOMType>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn SOMDomain(&self) -> ::windows::core::Result<GPMSOMType> {
        let mut result__: GPMSOMType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.SOMDomain)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMSOMType>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn SOMOU(&self) -> ::windows::core::Result<GPMSOMType> {
        let mut result__: GPMSOMType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.SOMOU)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMSOMType>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn SecurityFlags(&self, vbowner: i16, vbgroup: i16, vbdacl: i16, vbsacl: i16) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.SecurityFlags)(::core::mem::transmute_copy(self), ::core::mem::transmute(vbowner), ::core::mem::transmute(vbgroup), ::core::mem::transmute(vbdacl), ::core::mem::transmute(vbsacl), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn DoNotValidateDC(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.DoNotValidateDC)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn ReportHTML(&self) -> ::windows::core::Result<GPMReportType> {
        let mut result__: GPMReportType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.ReportHTML)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMReportType>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn ReportXML(&self) -> ::windows::core::Result<GPMReportType> {
        let mut result__: GPMReportType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.ReportXML)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMReportType>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn RSOPModeUnknown(&self) -> ::windows::core::Result<GPMRSOPMode> {
        let mut result__: GPMRSOPMode = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.RSOPModeUnknown)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMRSOPMode>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn RSOPModePlanning(&self) -> ::windows::core::Result<GPMRSOPMode> {
        let mut result__: GPMRSOPMode = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.RSOPModePlanning)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMRSOPMode>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn RSOPModeLogging(&self) -> ::windows::core::Result<GPMRSOPMode> {
        let mut result__: GPMRSOPMode = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.RSOPModeLogging)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMRSOPMode>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn EntryTypeUser(&self) -> ::windows::core::Result<GPMEntryType> {
        let mut result__: GPMEntryType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.EntryTypeUser)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMEntryType>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn EntryTypeComputer(&self) -> ::windows::core::Result<GPMEntryType> {
        let mut result__: GPMEntryType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.EntryTypeComputer)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMEntryType>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn EntryTypeLocalGroup(&self) -> ::windows::core::Result<GPMEntryType> {
        let mut result__: GPMEntryType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.EntryTypeLocalGroup)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMEntryType>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn EntryTypeGlobalGroup(&self) -> ::windows::core::Result<GPMEntryType> {
        let mut result__: GPMEntryType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.EntryTypeGlobalGroup)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMEntryType>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn EntryTypeUniversalGroup(&self) -> ::windows::core::Result<GPMEntryType> {
        let mut result__: GPMEntryType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.EntryTypeUniversalGroup)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMEntryType>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn EntryTypeUNCPath(&self) -> ::windows::core::Result<GPMEntryType> {
        let mut result__: GPMEntryType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.EntryTypeUNCPath)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMEntryType>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn EntryTypeUnknown(&self) -> ::windows::core::Result<GPMEntryType> {
        let mut result__: GPMEntryType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.EntryTypeUnknown)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMEntryType>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn DestinationOptionSameAsSource(&self) -> ::windows::core::Result<GPMDestinationOption> {
        let mut result__: GPMDestinationOption = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.DestinationOptionSameAsSource)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMDestinationOption>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn DestinationOptionNone(&self) -> ::windows::core::Result<GPMDestinationOption> {
        let mut result__: GPMDestinationOption = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.DestinationOptionNone)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMDestinationOption>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn DestinationOptionByRelativeName(&self) -> ::windows::core::Result<GPMDestinationOption> {
        let mut result__: GPMDestinationOption = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.DestinationOptionByRelativeName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMDestinationOption>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn DestinationOptionSet(&self) -> ::windows::core::Result<GPMDestinationOption> {
        let mut result__: GPMDestinationOption = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.DestinationOptionSet)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMDestinationOption>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn MigrationTableOnly(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.MigrationTableOnly)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn ProcessSecurity(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.ProcessSecurity)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn RsopLoggingNoComputer(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.RsopLoggingNoComputer)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn RsopLoggingNoUser(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.RsopLoggingNoUser)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn RsopPlanningAssumeSlowLink(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.RsopPlanningAssumeSlowLink)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn RsopPlanningLoopbackOption(&self, vbmerge: i16) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.RsopPlanningLoopbackOption)(::core::mem::transmute_copy(self), ::core::mem::transmute(vbmerge), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn RsopPlanningAssumeUserWQLFilterTrue(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.RsopPlanningAssumeUserWQLFilterTrue)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn RsopPlanningAssumeCompWQLFilterTrue(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.RsopPlanningAssumeCompWQLFilterTrue)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn BackupTypeGPO(&self) -> ::windows::core::Result<GPMBackupType> {
        let mut result__: GPMBackupType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).BackupTypeGPO)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMBackupType>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn BackupTypeStarterGPO(&self) -> ::windows::core::Result<GPMBackupType> {
        let mut result__: GPMBackupType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).BackupTypeStarterGPO)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMBackupType>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn StarterGPOTypeSystem(&self) -> ::windows::core::Result<GPMStarterGPOType> {
        let mut result__: GPMStarterGPOType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).StarterGPOTypeSystem)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMStarterGPOType>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn StarterGPOTypeCustom(&self) -> ::windows::core::Result<GPMStarterGPOType> {
        let mut result__: GPMStarterGPOType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).StarterGPOTypeCustom)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMStarterGPOType>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn SearchPropertyStarterGPOPermissions(&self) -> ::windows::core::Result<GPMSearchProperty> {
        let mut result__: GPMSearchProperty = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SearchPropertyStarterGPOPermissions)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMSearchProperty>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn SearchPropertyStarterGPOEffectivePermissions(&self) -> ::windows::core::Result<GPMSearchProperty> {
        let mut result__: GPMSearchProperty = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SearchPropertyStarterGPOEffectivePermissions)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMSearchProperty>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn SearchPropertyStarterGPODisplayName(&self) -> ::windows::core::Result<GPMSearchProperty> {
        let mut result__: GPMSearchProperty = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SearchPropertyStarterGPODisplayName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMSearchProperty>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn SearchPropertyStarterGPOID(&self) -> ::windows::core::Result<GPMSearchProperty> {
        let mut result__: GPMSearchProperty = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SearchPropertyStarterGPOID)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMSearchProperty>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn SearchPropertyStarterGPODomain(&self) -> ::windows::core::Result<GPMSearchProperty> {
        let mut result__: GPMSearchProperty = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SearchPropertyStarterGPODomain)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMSearchProperty>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn PermStarterGPORead(&self) -> ::windows::core::Result<GPMPermissionType> {
        let mut result__: GPMPermissionType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).PermStarterGPORead)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMPermissionType>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn PermStarterGPOEdit(&self) -> ::windows::core::Result<GPMPermissionType> {
        let mut result__: GPMPermissionType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).PermStarterGPOEdit)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMPermissionType>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn PermStarterGPOFullControl(&self) -> ::windows::core::Result<GPMPermissionType> {
        let mut result__: GPMPermissionType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).PermStarterGPOFullControl)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMPermissionType>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn PermStarterGPOCustom(&self) -> ::windows::core::Result<GPMPermissionType> {
        let mut result__: GPMPermissionType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).PermStarterGPOCustom)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMPermissionType>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn ReportLegacy(&self) -> ::windows::core::Result<GPMReportingOptions> {
        let mut result__: GPMReportingOptions = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ReportLegacy)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMReportingOptions>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn ReportComments(&self) -> ::windows::core::Result<GPMReportingOptions> {
        let mut result__: GPMReportingOptions = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ReportComments)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMReportingOptions>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGPMConstants2> for ::windows::core::IUnknown {
    fn from(value: IGPMConstants2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGPMConstants2> for ::windows::core::IUnknown {
    fn from(value: &IGPMConstants2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IGPMConstants2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IGPMConstants2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IGPMConstants2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a IGPMConstants2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGPMConstants2> for IGPMConstants {
    fn from(value: IGPMConstants2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGPMConstants2> for IGPMConstants {
    fn from(value: &IGPMConstants2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IGPMConstants> for IGPMConstants2 {
    fn into_param(self) -> ::windows::core::Param<'a, IGPMConstants> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IGPMConstants> for &'a IGPMConstants2 {
    fn into_param(self) -> ::windows::core::Param<'a, IGPMConstants> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IGPMConstants2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IGPMConstants2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IGPMConstants2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IGPMConstants2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGPMConstants2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IGPMConstants2 {
    type Vtable = IGPMConstants2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x05ae21b0_ac09_4032_a26f_9e7da786dc19);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMConstants2_Vtbl {
    pub base: IGPMConstants_Vtbl,
    pub BackupTypeGPO: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMBackupType) -> ::windows::core::HRESULT,
    pub BackupTypeStarterGPO: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMBackupType) -> ::windows::core::HRESULT,
    pub StarterGPOTypeSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMStarterGPOType) -> ::windows::core::HRESULT,
    pub StarterGPOTypeCustom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMStarterGPOType) -> ::windows::core::HRESULT,
    pub SearchPropertyStarterGPOPermissions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows::core::HRESULT,
    pub SearchPropertyStarterGPOEffectivePermissions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows::core::HRESULT,
    pub SearchPropertyStarterGPODisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows::core::HRESULT,
    pub SearchPropertyStarterGPOID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows::core::HRESULT,
    pub SearchPropertyStarterGPODomain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows::core::HRESULT,
    pub PermStarterGPORead: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT,
    pub PermStarterGPOEdit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT,
    pub PermStarterGPOFullControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT,
    pub PermStarterGPOCustom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT,
    pub ReportLegacy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMReportingOptions) -> ::windows::core::HRESULT,
    pub ReportComments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMReportingOptions) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IGPMDomain(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IGPMDomain {
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DomainController(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).DomainController)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Domain(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Domain)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateGPO(&self) -> ::windows::core::Result<IGPMGPO> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateGPO)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IGPMGPO>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetGPO<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrguid: Param0) -> ::windows::core::Result<IGPMGPO> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetGPO)(::core::mem::transmute_copy(self), bstrguid.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IGPMGPO>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SearchGPOs<'a, Param0: ::windows::core::IntoParam<'a, IGPMSearchCriteria>>(&self, pigpmsearchcriteria: Param0) -> ::windows::core::Result<IGPMGPOCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SearchGPOs)(::core::mem::transmute_copy(self), pigpmsearchcriteria.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IGPMGPOCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn RestoreGPO<'a, Param0: ::windows::core::IntoParam<'a, IGPMBackup>>(&self, pigpmbackup: Param0, ldcflags: i32, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RestoreGPO)(::core::mem::transmute_copy(self), pigpmbackup.into_param().abi(), ::core::mem::transmute(ldcflags), ::core::mem::transmute(pvargpmprogress), ::core::mem::transmute(pvargpmcancel), ::core::mem::transmute(ppigpmresult)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetSOM<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrpath: Param0) -> ::windows::core::Result<IGPMSOM> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSOM)(::core::mem::transmute_copy(self), bstrpath.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IGPMSOM>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SearchSOMs<'a, Param0: ::windows::core::IntoParam<'a, IGPMSearchCriteria>>(&self, pigpmsearchcriteria: Param0) -> ::windows::core::Result<IGPMSOMCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SearchSOMs)(::core::mem::transmute_copy(self), pigpmsearchcriteria.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IGPMSOMCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetWMIFilter<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrpath: Param0) -> ::windows::core::Result<IGPMWMIFilter> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetWMIFilter)(::core::mem::transmute_copy(self), bstrpath.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IGPMWMIFilter>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SearchWMIFilters<'a, Param0: ::windows::core::IntoParam<'a, IGPMSearchCriteria>>(&self, pigpmsearchcriteria: Param0) -> ::windows::core::Result<IGPMWMIFilterCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SearchWMIFilters)(::core::mem::transmute_copy(self), pigpmsearchcriteria.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IGPMWMIFilterCollection>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGPMDomain> for ::windows::core::IUnknown {
    fn from(value: IGPMDomain) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGPMDomain> for ::windows::core::IUnknown {
    fn from(value: &IGPMDomain) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IGPMDomain {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IGPMDomain {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IGPMDomain {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a IGPMDomain {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IGPMDomain {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IGPMDomain {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IGPMDomain {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IGPMDomain {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGPMDomain").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IGPMDomain {
    type Vtable = IGPMDomain_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6b21cc14_5a00_4f44_a738_feec8a94c7e3);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMDomain_Vtbl {
    pub base: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub DomainController: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DomainController: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Domain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Domain: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateGPO: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppnewgpo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateGPO: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetGPO: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrguid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppgpo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetGPO: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SearchGPOs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pigpmsearchcriteria: ::windows::core::RawPtr, ppigpmgpocollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SearchGPOs: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub RestoreGPO: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pigpmbackup: ::windows::core::RawPtr, ldcflags: i32, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    RestoreGPO: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetSOM: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppsom: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetSOM: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SearchSOMs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pigpmsearchcriteria: ::windows::core::RawPtr, ppigpmsomcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SearchSOMs: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetWMIFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppwmifilter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetWMIFilter: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SearchWMIFilters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pigpmsearchcriteria: ::windows::core::RawPtr, ppigpmwmifiltercollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SearchWMIFilters: usize,
}
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IGPMDomain2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IGPMDomain2 {
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DomainController(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.DomainController)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Domain(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Domain)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateGPO(&self) -> ::windows::core::Result<IGPMGPO> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateGPO)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IGPMGPO>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetGPO<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrguid: Param0) -> ::windows::core::Result<IGPMGPO> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetGPO)(::core::mem::transmute_copy(self), bstrguid.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IGPMGPO>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SearchGPOs<'a, Param0: ::windows::core::IntoParam<'a, IGPMSearchCriteria>>(&self, pigpmsearchcriteria: Param0) -> ::windows::core::Result<IGPMGPOCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.SearchGPOs)(::core::mem::transmute_copy(self), pigpmsearchcriteria.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IGPMGPOCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn RestoreGPO<'a, Param0: ::windows::core::IntoParam<'a, IGPMBackup>>(&self, pigpmbackup: Param0, ldcflags: i32, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.RestoreGPO)(::core::mem::transmute_copy(self), pigpmbackup.into_param().abi(), ::core::mem::transmute(ldcflags), ::core::mem::transmute(pvargpmprogress), ::core::mem::transmute(pvargpmcancel), ::core::mem::transmute(ppigpmresult)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetSOM<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrpath: Param0) -> ::windows::core::Result<IGPMSOM> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetSOM)(::core::mem::transmute_copy(self), bstrpath.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IGPMSOM>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SearchSOMs<'a, Param0: ::windows::core::IntoParam<'a, IGPMSearchCriteria>>(&self, pigpmsearchcriteria: Param0) -> ::windows::core::Result<IGPMSOMCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.SearchSOMs)(::core::mem::transmute_copy(self), pigpmsearchcriteria.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IGPMSOMCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetWMIFilter<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrpath: Param0) -> ::windows::core::Result<IGPMWMIFilter> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetWMIFilter)(::core::mem::transmute_copy(self), bstrpath.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IGPMWMIFilter>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SearchWMIFilters<'a, Param0: ::windows::core::IntoParam<'a, IGPMSearchCriteria>>(&self, pigpmsearchcriteria: Param0) -> ::windows::core::Result<IGPMWMIFilterCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.SearchWMIFilters)(::core::mem::transmute_copy(self), pigpmsearchcriteria.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IGPMWMIFilterCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateStarterGPO(&self) -> ::windows::core::Result<IGPMStarterGPO> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateStarterGPO)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IGPMStarterGPO>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateGPOFromStarterGPO<'a, Param0: ::windows::core::IntoParam<'a, IGPMStarterGPO>>(&self, pgpotemplate: Param0) -> ::windows::core::Result<IGPMGPO> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateGPOFromStarterGPO)(::core::mem::transmute_copy(self), pgpotemplate.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IGPMGPO>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetStarterGPO<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrguid: Param0) -> ::windows::core::Result<IGPMStarterGPO> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetStarterGPO)(::core::mem::transmute_copy(self), bstrguid.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IGPMStarterGPO>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SearchStarterGPOs<'a, Param0: ::windows::core::IntoParam<'a, IGPMSearchCriteria>>(&self, pigpmsearchcriteria: Param0) -> ::windows::core::Result<IGPMStarterGPOCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SearchStarterGPOs)(::core::mem::transmute_copy(self), pigpmsearchcriteria.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IGPMStarterGPOCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn LoadStarterGPO<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrloadfile: Param0, boverwrite: i16, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).LoadStarterGPO)(::core::mem::transmute_copy(self), bstrloadfile.into_param().abi(), ::core::mem::transmute(boverwrite), ::core::mem::transmute(pvargpmprogress), ::core::mem::transmute(pvargpmcancel), ::core::mem::transmute(ppigpmresult)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn RestoreStarterGPO<'a, Param0: ::windows::core::IntoParam<'a, IGPMStarterGPOBackup>>(&self, pigpmtmplbackup: Param0, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RestoreStarterGPO)(::core::mem::transmute_copy(self), pigpmtmplbackup.into_param().abi(), ::core::mem::transmute(pvargpmprogress), ::core::mem::transmute(pvargpmcancel), ::core::mem::transmute(ppigpmresult)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGPMDomain2> for ::windows::core::IUnknown {
    fn from(value: IGPMDomain2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGPMDomain2> for ::windows::core::IUnknown {
    fn from(value: &IGPMDomain2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IGPMDomain2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IGPMDomain2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IGPMDomain2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a IGPMDomain2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGPMDomain2> for IGPMDomain {
    fn from(value: IGPMDomain2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGPMDomain2> for IGPMDomain {
    fn from(value: &IGPMDomain2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IGPMDomain> for IGPMDomain2 {
    fn into_param(self) -> ::windows::core::Param<'a, IGPMDomain> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IGPMDomain> for &'a IGPMDomain2 {
    fn into_param(self) -> ::windows::core::Param<'a, IGPMDomain> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IGPMDomain2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IGPMDomain2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IGPMDomain2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IGPMDomain2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGPMDomain2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IGPMDomain2 {
    type Vtable = IGPMDomain2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7ca6bb8b_f1eb_490a_938d_3c4e51c768e6);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMDomain2_Vtbl {
    pub base: IGPMDomain_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateStarterGPO: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppnewtemplate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateStarterGPO: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateGPOFromStarterGPO: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pgpotemplate: ::windows::core::RawPtr, ppnewgpo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateGPOFromStarterGPO: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetStarterGPO: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrguid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pptemplate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetStarterGPO: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SearchStarterGPOs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pigpmsearchcriteria: ::windows::core::RawPtr, ppigpmtemplatecollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SearchStarterGPOs: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub LoadStarterGPO: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrloadfile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, boverwrite: i16, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    LoadStarterGPO: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub RestoreStarterGPO: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pigpmtmplbackup: ::windows::core::RawPtr, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    RestoreStarterGPO: usize,
}
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IGPMDomain3(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IGPMDomain3 {
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DomainController(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.DomainController)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Domain(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.Domain)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateGPO(&self) -> ::windows::core::Result<IGPMGPO> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.CreateGPO)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IGPMGPO>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetGPO<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrguid: Param0) -> ::windows::core::Result<IGPMGPO> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetGPO)(::core::mem::transmute_copy(self), bstrguid.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IGPMGPO>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SearchGPOs<'a, Param0: ::windows::core::IntoParam<'a, IGPMSearchCriteria>>(&self, pigpmsearchcriteria: Param0) -> ::windows::core::Result<IGPMGPOCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.SearchGPOs)(::core::mem::transmute_copy(self), pigpmsearchcriteria.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IGPMGPOCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn RestoreGPO<'a, Param0: ::windows::core::IntoParam<'a, IGPMBackup>>(&self, pigpmbackup: Param0, ldcflags: i32, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.RestoreGPO)(::core::mem::transmute_copy(self), pigpmbackup.into_param().abi(), ::core::mem::transmute(ldcflags), ::core::mem::transmute(pvargpmprogress), ::core::mem::transmute(pvargpmcancel), ::core::mem::transmute(ppigpmresult)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetSOM<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrpath: Param0) -> ::windows::core::Result<IGPMSOM> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetSOM)(::core::mem::transmute_copy(self), bstrpath.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IGPMSOM>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SearchSOMs<'a, Param0: ::windows::core::IntoParam<'a, IGPMSearchCriteria>>(&self, pigpmsearchcriteria: Param0) -> ::windows::core::Result<IGPMSOMCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.SearchSOMs)(::core::mem::transmute_copy(self), pigpmsearchcriteria.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IGPMSOMCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetWMIFilter<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrpath: Param0) -> ::windows::core::Result<IGPMWMIFilter> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetWMIFilter)(::core::mem::transmute_copy(self), bstrpath.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IGPMWMIFilter>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SearchWMIFilters<'a, Param0: ::windows::core::IntoParam<'a, IGPMSearchCriteria>>(&self, pigpmsearchcriteria: Param0) -> ::windows::core::Result<IGPMWMIFilterCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.SearchWMIFilters)(::core::mem::transmute_copy(self), pigpmsearchcriteria.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IGPMWMIFilterCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateStarterGPO(&self) -> ::windows::core::Result<IGPMStarterGPO> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateStarterGPO)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IGPMStarterGPO>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateGPOFromStarterGPO<'a, Param0: ::windows::core::IntoParam<'a, IGPMStarterGPO>>(&self, pgpotemplate: Param0) -> ::windows::core::Result<IGPMGPO> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateGPOFromStarterGPO)(::core::mem::transmute_copy(self), pgpotemplate.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IGPMGPO>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetStarterGPO<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrguid: Param0) -> ::windows::core::Result<IGPMStarterGPO> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetStarterGPO)(::core::mem::transmute_copy(self), bstrguid.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IGPMStarterGPO>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SearchStarterGPOs<'a, Param0: ::windows::core::IntoParam<'a, IGPMSearchCriteria>>(&self, pigpmsearchcriteria: Param0) -> ::windows::core::Result<IGPMStarterGPOCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.SearchStarterGPOs)(::core::mem::transmute_copy(self), pigpmsearchcriteria.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IGPMStarterGPOCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn LoadStarterGPO<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrloadfile: Param0, boverwrite: i16, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.LoadStarterGPO)(::core::mem::transmute_copy(self), bstrloadfile.into_param().abi(), ::core::mem::transmute(boverwrite), ::core::mem::transmute(pvargpmprogress), ::core::mem::transmute(pvargpmcancel), ::core::mem::transmute(ppigpmresult)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn RestoreStarterGPO<'a, Param0: ::windows::core::IntoParam<'a, IGPMStarterGPOBackup>>(&self, pigpmtmplbackup: Param0, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.RestoreStarterGPO)(::core::mem::transmute_copy(self), pigpmtmplbackup.into_param().abi(), ::core::mem::transmute(pvargpmprogress), ::core::mem::transmute(pvargpmcancel), ::core::mem::transmute(ppigpmresult)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GenerateReport(&self, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GenerateReport)(::core::mem::transmute_copy(self), ::core::mem::transmute(gpmreporttype), ::core::mem::transmute(pvargpmprogress), ::core::mem::transmute(pvargpmcancel), ::core::mem::transmute(ppigpmresult)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InfrastructureDC(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).InfrastructureDC)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetInfrastructureDC<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, newval: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetInfrastructureDC)(::core::mem::transmute_copy(self), newval.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn SetInfrastructureFlags(&self, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetInfrastructureFlags)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGPMDomain3> for ::windows::core::IUnknown {
    fn from(value: IGPMDomain3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGPMDomain3> for ::windows::core::IUnknown {
    fn from(value: &IGPMDomain3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IGPMDomain3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IGPMDomain3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IGPMDomain3 {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a IGPMDomain3 {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGPMDomain3> for IGPMDomain {
    fn from(value: IGPMDomain3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGPMDomain3> for IGPMDomain {
    fn from(value: &IGPMDomain3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IGPMDomain> for IGPMDomain3 {
    fn into_param(self) -> ::windows::core::Param<'a, IGPMDomain> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IGPMDomain> for &'a IGPMDomain3 {
    fn into_param(self) -> ::windows::core::Param<'a, IGPMDomain> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGPMDomain3> for IGPMDomain2 {
    fn from(value: IGPMDomain3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGPMDomain3> for IGPMDomain2 {
    fn from(value: &IGPMDomain3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IGPMDomain2> for IGPMDomain3 {
    fn into_param(self) -> ::windows::core::Param<'a, IGPMDomain2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IGPMDomain2> for &'a IGPMDomain3 {
    fn into_param(self) -> ::windows::core::Param<'a, IGPMDomain2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IGPMDomain3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IGPMDomain3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IGPMDomain3 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IGPMDomain3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGPMDomain3").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IGPMDomain3 {
    type Vtable = IGPMDomain3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0077fdfe_88c7_4acf_a11d_d10a7c310a03);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMDomain3_Vtbl {
    pub base: IGPMDomain2_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GenerateReport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GenerateReport: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub InfrastructureDC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InfrastructureDC: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetInfrastructureDC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetInfrastructureDC: usize,
    pub SetInfrastructureFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IGPMGPO(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IGPMGPO {
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DisplayName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).DisplayName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDisplayName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, newval: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDisplayName)(::core::mem::transmute_copy(self), newval.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Path(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Path)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ID)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DomainName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).DomainName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn CreationTime(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreationTime)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn ModificationTime(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ModificationTime)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn UserDSVersionNumber(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).UserDSVersionNumber)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn ComputerDSVersionNumber(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ComputerDSVersionNumber)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn UserSysvolVersionNumber(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).UserSysvolVersionNumber)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn ComputerSysvolVersionNumber(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ComputerSysvolVersionNumber)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetWMIFilter(&self) -> ::windows::core::Result<IGPMWMIFilter> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetWMIFilter)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IGPMWMIFilter>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetWMIFilter<'a, Param0: ::windows::core::IntoParam<'a, IGPMWMIFilter>>(&self, pigpmwmifilter: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetWMIFilter)(::core::mem::transmute_copy(self), pigpmwmifilter.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn SetUserEnabled(&self, vbenabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetUserEnabled)(::core::mem::transmute_copy(self), ::core::mem::transmute(vbenabled)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn SetComputerEnabled(&self, vbenabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetComputerEnabled)(::core::mem::transmute_copy(self), ::core::mem::transmute(vbenabled)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn IsUserEnabled(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).IsUserEnabled)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn IsComputerEnabled(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).IsComputerEnabled)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSecurityInfo(&self) -> ::windows::core::Result<IGPMSecurityInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSecurityInfo)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IGPMSecurityInfo>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSecurityInfo<'a, Param0: ::windows::core::IntoParam<'a, IGPMSecurityInfo>>(&self, psecurityinfo: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSecurityInfo)(::core::mem::transmute_copy(self), psecurityinfo.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Delete)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Backup<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrbackupdir: Param0, bstrcomment: Param1, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Backup)(::core::mem::transmute_copy(self), bstrbackupdir.into_param().abi(), bstrcomment.into_param().abi(), ::core::mem::transmute(pvargpmprogress), ::core::mem::transmute(pvargpmcancel), ::core::mem::transmute(ppigpmresult)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Import<'a, Param1: ::windows::core::IntoParam<'a, IGPMBackup>>(&self, lflags: i32, pigpmbackup: Param1, pvarmigrationtable: *const super::Com::VARIANT, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Import)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), pigpmbackup.into_param().abi(), ::core::mem::transmute(pvarmigrationtable), ::core::mem::transmute(pvargpmprogress), ::core::mem::transmute(pvargpmcancel), ::core::mem::transmute(ppigpmresult)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GenerateReport(&self, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GenerateReport)(::core::mem::transmute_copy(self), ::core::mem::transmute(gpmreporttype), ::core::mem::transmute(pvargpmprogress), ::core::mem::transmute(pvargpmcancel), ::core::mem::transmute(ppigpmresult)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GenerateReportToFile<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, gpmreporttype: GPMReportType, bstrtargetfilepath: Param1) -> ::windows::core::Result<IGPMResult> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GenerateReportToFile)(::core::mem::transmute_copy(self), ::core::mem::transmute(gpmreporttype), bstrtargetfilepath.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IGPMResult>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CopyTo<'a, Param1: ::windows::core::IntoParam<'a, IGPMDomain>>(&self, lflags: i32, pigpmdomain: Param1, pvarnewdisplayname: *const super::Com::VARIANT, pvarmigrationtable: *const super::Com::VARIANT, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CopyTo)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), pigpmdomain.into_param().abi(), ::core::mem::transmute(pvarnewdisplayname), ::core::mem::transmute(pvarmigrationtable), ::core::mem::transmute(pvargpmprogress), ::core::mem::transmute(pvargpmcancel), ::core::mem::transmute(ppigpmresult)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSecurityDescriptor<'a, Param1: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, lflags: i32, psd: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSecurityDescriptor)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), psd.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSecurityDescriptor(&self, lflags: i32) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSecurityDescriptor)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::IDispatch>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn IsACLConsistent(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).IsACLConsistent)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn MakeACLConsistent(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).MakeACLConsistent)(::core::mem::transmute_copy(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGPMGPO> for ::windows::core::IUnknown {
    fn from(value: IGPMGPO) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGPMGPO> for ::windows::core::IUnknown {
    fn from(value: &IGPMGPO) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IGPMGPO {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IGPMGPO {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IGPMGPO {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a IGPMGPO {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IGPMGPO {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IGPMGPO {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IGPMGPO {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IGPMGPO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGPMGPO").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IGPMGPO {
    type Vtable = IGPMGPO_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x58cc4352_1ca3_48e5_9864_1da4d6e0d60f);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMGPO_Vtbl {
    pub base: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DisplayName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDisplayName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Path: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Path: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DomainName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DomainName: usize,
    pub CreationTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdate: *mut f64) -> ::windows::core::HRESULT,
    pub ModificationTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdate: *mut f64) -> ::windows::core::HRESULT,
    pub UserDSVersionNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT,
    pub ComputerDSVersionNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT,
    pub UserSysvolVersionNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT,
    pub ComputerSysvolVersionNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetWMIFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppigpmwmifilter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetWMIFilter: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetWMIFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pigpmwmifilter: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetWMIFilter: usize,
    pub SetUserEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vbenabled: i16) -> ::windows::core::HRESULT,
    pub SetComputerEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vbenabled: i16) -> ::windows::core::HRESULT,
    pub IsUserEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvbenabled: *mut i16) -> ::windows::core::HRESULT,
    pub IsComputerEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvbenabled: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetSecurityInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsecurityinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetSecurityInfo: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetSecurityInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psecurityinfo: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetSecurityInfo: usize,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Backup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrbackupdir: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrcomment: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Backup: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Import: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, pigpmbackup: ::windows::core::RawPtr, pvarmigrationtable: *const super::Com::VARIANT, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Import: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GenerateReport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GenerateReport: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GenerateReportToFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gpmreporttype: GPMReportType, bstrtargetfilepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppigpmresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GenerateReportToFile: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CopyTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, pigpmdomain: ::windows::core::RawPtr, pvarnewdisplayname: *const super::Com::VARIANT, pvarmigrationtable: *const super::Com::VARIANT, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CopyTo: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetSecurityDescriptor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, psd: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetSecurityDescriptor: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetSecurityDescriptor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, ppsd: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetSecurityDescriptor: usize,
    pub IsACLConsistent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvbconsistent: *mut i16) -> ::windows::core::HRESULT,
    pub MakeACLConsistent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IGPMGPO2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IGPMGPO2 {
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DisplayName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.DisplayName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDisplayName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, newval: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetDisplayName)(::core::mem::transmute_copy(self), newval.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Path(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Path)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.ID)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DomainName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.DomainName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn CreationTime(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreationTime)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn ModificationTime(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.ModificationTime)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn UserDSVersionNumber(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.UserDSVersionNumber)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn ComputerDSVersionNumber(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.ComputerDSVersionNumber)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn UserSysvolVersionNumber(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.UserSysvolVersionNumber)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn ComputerSysvolVersionNumber(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.ComputerSysvolVersionNumber)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetWMIFilter(&self) -> ::windows::core::Result<IGPMWMIFilter> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetWMIFilter)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IGPMWMIFilter>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetWMIFilter<'a, Param0: ::windows::core::IntoParam<'a, IGPMWMIFilter>>(&self, pigpmwmifilter: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetWMIFilter)(::core::mem::transmute_copy(self), pigpmwmifilter.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn SetUserEnabled(&self, vbenabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetUserEnabled)(::core::mem::transmute_copy(self), ::core::mem::transmute(vbenabled)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn SetComputerEnabled(&self, vbenabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetComputerEnabled)(::core::mem::transmute_copy(self), ::core::mem::transmute(vbenabled)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn IsUserEnabled(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.IsUserEnabled)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn IsComputerEnabled(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.IsComputerEnabled)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSecurityInfo(&self) -> ::windows::core::Result<IGPMSecurityInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetSecurityInfo)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IGPMSecurityInfo>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSecurityInfo<'a, Param0: ::windows::core::IntoParam<'a, IGPMSecurityInfo>>(&self, psecurityinfo: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetSecurityInfo)(::core::mem::transmute_copy(self), psecurityinfo.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Delete)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Backup<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrbackupdir: Param0, bstrcomment: Param1, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Backup)(::core::mem::transmute_copy(self), bstrbackupdir.into_param().abi(), bstrcomment.into_param().abi(), ::core::mem::transmute(pvargpmprogress), ::core::mem::transmute(pvargpmcancel), ::core::mem::transmute(ppigpmresult)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Import<'a, Param1: ::windows::core::IntoParam<'a, IGPMBackup>>(&self, lflags: i32, pigpmbackup: Param1, pvarmigrationtable: *const super::Com::VARIANT, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Import)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), pigpmbackup.into_param().abi(), ::core::mem::transmute(pvarmigrationtable), ::core::mem::transmute(pvargpmprogress), ::core::mem::transmute(pvargpmcancel), ::core::mem::transmute(ppigpmresult)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GenerateReport(&self, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GenerateReport)(::core::mem::transmute_copy(self), ::core::mem::transmute(gpmreporttype), ::core::mem::transmute(pvargpmprogress), ::core::mem::transmute(pvargpmcancel), ::core::mem::transmute(ppigpmresult)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GenerateReportToFile<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, gpmreporttype: GPMReportType, bstrtargetfilepath: Param1) -> ::windows::core::Result<IGPMResult> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GenerateReportToFile)(::core::mem::transmute_copy(self), ::core::mem::transmute(gpmreporttype), bstrtargetfilepath.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IGPMResult>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CopyTo<'a, Param1: ::windows::core::IntoParam<'a, IGPMDomain>>(&self, lflags: i32, pigpmdomain: Param1, pvarnewdisplayname: *const super::Com::VARIANT, pvarmigrationtable: *const super::Com::VARIANT, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.CopyTo)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), pigpmdomain.into_param().abi(), ::core::mem::transmute(pvarnewdisplayname), ::core::mem::transmute(pvarmigrationtable), ::core::mem::transmute(pvargpmprogress), ::core::mem::transmute(pvargpmcancel), ::core::mem::transmute(ppigpmresult)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSecurityDescriptor<'a, Param1: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, lflags: i32, psd: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetSecurityDescriptor)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), psd.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSecurityDescriptor(&self, lflags: i32) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetSecurityDescriptor)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::IDispatch>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn IsACLConsistent(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.IsACLConsistent)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn MakeACLConsistent(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.MakeACLConsistent)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Description)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, newval: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDescription)(::core::mem::transmute_copy(self), newval.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGPMGPO2> for ::windows::core::IUnknown {
    fn from(value: IGPMGPO2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGPMGPO2> for ::windows::core::IUnknown {
    fn from(value: &IGPMGPO2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IGPMGPO2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IGPMGPO2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IGPMGPO2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a IGPMGPO2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGPMGPO2> for IGPMGPO {
    fn from(value: IGPMGPO2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGPMGPO2> for IGPMGPO {
    fn from(value: &IGPMGPO2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IGPMGPO> for IGPMGPO2 {
    fn into_param(self) -> ::windows::core::Param<'a, IGPMGPO> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IGPMGPO> for &'a IGPMGPO2 {
    fn into_param(self) -> ::windows::core::Param<'a, IGPMGPO> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IGPMGPO2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IGPMGPO2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IGPMGPO2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IGPMGPO2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGPMGPO2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IGPMGPO2 {
    type Vtable = IGPMGPO2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8a66a210_b78b_4d99_88e2_c306a817c925);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMGPO2_Vtbl {
    pub base: IGPMGPO_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Description: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDescription: usize,
}
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IGPMGPO3(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IGPMGPO3 {
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DisplayName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.DisplayName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDisplayName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, newval: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetDisplayName)(::core::mem::transmute_copy(self), newval.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Path(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.Path)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.ID)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DomainName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.DomainName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn CreationTime(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.CreationTime)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn ModificationTime(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.ModificationTime)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn UserDSVersionNumber(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.UserDSVersionNumber)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn ComputerDSVersionNumber(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.ComputerDSVersionNumber)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn UserSysvolVersionNumber(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.UserSysvolVersionNumber)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn ComputerSysvolVersionNumber(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.ComputerSysvolVersionNumber)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetWMIFilter(&self) -> ::windows::core::Result<IGPMWMIFilter> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetWMIFilter)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IGPMWMIFilter>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetWMIFilter<'a, Param0: ::windows::core::IntoParam<'a, IGPMWMIFilter>>(&self, pigpmwmifilter: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetWMIFilter)(::core::mem::transmute_copy(self), pigpmwmifilter.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn SetUserEnabled(&self, vbenabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetUserEnabled)(::core::mem::transmute_copy(self), ::core::mem::transmute(vbenabled)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn SetComputerEnabled(&self, vbenabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetComputerEnabled)(::core::mem::transmute_copy(self), ::core::mem::transmute(vbenabled)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn IsUserEnabled(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.IsUserEnabled)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn IsComputerEnabled(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.IsComputerEnabled)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSecurityInfo(&self) -> ::windows::core::Result<IGPMSecurityInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetSecurityInfo)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IGPMSecurityInfo>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSecurityInfo<'a, Param0: ::windows::core::IntoParam<'a, IGPMSecurityInfo>>(&self, psecurityinfo: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetSecurityInfo)(::core::mem::transmute_copy(self), psecurityinfo.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.Delete)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Backup<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrbackupdir: Param0, bstrcomment: Param1, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.Backup)(::core::mem::transmute_copy(self), bstrbackupdir.into_param().abi(), bstrcomment.into_param().abi(), ::core::mem::transmute(pvargpmprogress), ::core::mem::transmute(pvargpmcancel), ::core::mem::transmute(ppigpmresult)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Import<'a, Param1: ::windows::core::IntoParam<'a, IGPMBackup>>(&self, lflags: i32, pigpmbackup: Param1, pvarmigrationtable: *const super::Com::VARIANT, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.Import)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), pigpmbackup.into_param().abi(), ::core::mem::transmute(pvarmigrationtable), ::core::mem::transmute(pvargpmprogress), ::core::mem::transmute(pvargpmcancel), ::core::mem::transmute(ppigpmresult)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GenerateReport(&self, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.GenerateReport)(::core::mem::transmute_copy(self), ::core::mem::transmute(gpmreporttype), ::core::mem::transmute(pvargpmprogress), ::core::mem::transmute(pvargpmcancel), ::core::mem::transmute(ppigpmresult)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GenerateReportToFile<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, gpmreporttype: GPMReportType, bstrtargetfilepath: Param1) -> ::windows::core::Result<IGPMResult> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GenerateReportToFile)(::core::mem::transmute_copy(self), ::core::mem::transmute(gpmreporttype), bstrtargetfilepath.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IGPMResult>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CopyTo<'a, Param1: ::windows::core::IntoParam<'a, IGPMDomain>>(&self, lflags: i32, pigpmdomain: Param1, pvarnewdisplayname: *const super::Com::VARIANT, pvarmigrationtable: *const super::Com::VARIANT, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.CopyTo)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), pigpmdomain.into_param().abi(), ::core::mem::transmute(pvarnewdisplayname), ::core::mem::transmute(pvarmigrationtable), ::core::mem::transmute(pvargpmprogress), ::core::mem::transmute(pvargpmcancel), ::core::mem::transmute(ppigpmresult)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSecurityDescriptor<'a, Param1: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, lflags: i32, psd: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetSecurityDescriptor)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), psd.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSecurityDescriptor(&self, lflags: i32) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetSecurityDescriptor)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::IDispatch>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn IsACLConsistent(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.IsACLConsistent)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn MakeACLConsistent(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.MakeACLConsistent)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Description)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, newval: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetDescription)(::core::mem::transmute_copy(self), newval.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InfrastructureDC(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).InfrastructureDC)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetInfrastructureDC<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, newval: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetInfrastructureDC)(::core::mem::transmute_copy(self), newval.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn SetInfrastructureFlags(&self, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetInfrastructureFlags)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGPMGPO3> for ::windows::core::IUnknown {
    fn from(value: IGPMGPO3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGPMGPO3> for ::windows::core::IUnknown {
    fn from(value: &IGPMGPO3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IGPMGPO3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IGPMGPO3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IGPMGPO3 {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a IGPMGPO3 {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGPMGPO3> for IGPMGPO {
    fn from(value: IGPMGPO3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGPMGPO3> for IGPMGPO {
    fn from(value: &IGPMGPO3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IGPMGPO> for IGPMGPO3 {
    fn into_param(self) -> ::windows::core::Param<'a, IGPMGPO> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IGPMGPO> for &'a IGPMGPO3 {
    fn into_param(self) -> ::windows::core::Param<'a, IGPMGPO> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGPMGPO3> for IGPMGPO2 {
    fn from(value: IGPMGPO3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGPMGPO3> for IGPMGPO2 {
    fn from(value: &IGPMGPO3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IGPMGPO2> for IGPMGPO3 {
    fn into_param(self) -> ::windows::core::Param<'a, IGPMGPO2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IGPMGPO2> for &'a IGPMGPO3 {
    fn into_param(self) -> ::windows::core::Param<'a, IGPMGPO2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IGPMGPO3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IGPMGPO3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IGPMGPO3 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IGPMGPO3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGPMGPO3").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IGPMGPO3 {
    type Vtable = IGPMGPO3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7cf123a1_f94a_4112_bfae_6aa1db9cb248);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMGPO3_Vtbl {
    pub base: IGPMGPO2_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub InfrastructureDC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InfrastructureDC: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetInfrastructureDC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetInfrastructureDC: usize,
    pub SetInfrastructureFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IGPMGPOCollection(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IGPMGPOCollection {
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Count)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Item(&self, lindex: i32) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Item)(::core::mem::transmute_copy(self), ::core::mem::transmute(lindex), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<super::Ole::IEnumVARIANT> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self)._NewEnum)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Ole::IEnumVARIANT>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGPMGPOCollection> for ::windows::core::IUnknown {
    fn from(value: IGPMGPOCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGPMGPOCollection> for ::windows::core::IUnknown {
    fn from(value: &IGPMGPOCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IGPMGPOCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IGPMGPOCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IGPMGPOCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a IGPMGPOCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IGPMGPOCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IGPMGPOCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IGPMGPOCollection {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IGPMGPOCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGPMGPOCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IGPMGPOCollection {
    type Vtable = IGPMGPOCollection_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf0f0d5cf_70ca_4c39_9e29_b642f8726c01);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMGPOCollection_Vtbl {
    pub base: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lindex: i32, pval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Item: usize,
    #[cfg(feature = "Win32_System_Ole")]
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppigpmgpos: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    _NewEnum: usize,
}
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IGPMGPOLink(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IGPMGPOLink {
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GPOID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GPOID)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GPODomain(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GPODomain)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn Enabled(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Enabled)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn SetEnabled(&self, newval: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetEnabled)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn Enforced(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Enforced)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn SetEnforced(&self, newval: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetEnforced)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn SOMLinkOrder(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SOMLinkOrder)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SOM(&self) -> ::windows::core::Result<IGPMSOM> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SOM)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IGPMSOM>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Delete)(::core::mem::transmute_copy(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGPMGPOLink> for ::windows::core::IUnknown {
    fn from(value: IGPMGPOLink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGPMGPOLink> for ::windows::core::IUnknown {
    fn from(value: &IGPMGPOLink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IGPMGPOLink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IGPMGPOLink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IGPMGPOLink {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a IGPMGPOLink {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IGPMGPOLink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IGPMGPOLink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IGPMGPOLink {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IGPMGPOLink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGPMGPOLink").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IGPMGPOLink {
    type Vtable = IGPMGPOLink_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x434b99bd_5de7_478a_809c_c251721df70c);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMGPOLink_Vtbl {
    pub base: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GPOID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GPOID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GPODomain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GPODomain: usize,
    pub Enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows::core::HRESULT,
    pub Enforced: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT,
    pub SetEnforced: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows::core::HRESULT,
    pub SOMLinkOrder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lval: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SOM: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppigpmsom: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SOM: usize,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IGPMGPOLinksCollection(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IGPMGPOLinksCollection {
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Count)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Item(&self, lindex: i32) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Item)(::core::mem::transmute_copy(self), ::core::mem::transmute(lindex), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<super::Ole::IEnumVARIANT> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self)._NewEnum)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Ole::IEnumVARIANT>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGPMGPOLinksCollection> for ::windows::core::IUnknown {
    fn from(value: IGPMGPOLinksCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGPMGPOLinksCollection> for ::windows::core::IUnknown {
    fn from(value: &IGPMGPOLinksCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IGPMGPOLinksCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IGPMGPOLinksCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IGPMGPOLinksCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a IGPMGPOLinksCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IGPMGPOLinksCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IGPMGPOLinksCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IGPMGPOLinksCollection {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IGPMGPOLinksCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGPMGPOLinksCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IGPMGPOLinksCollection {
    type Vtable = IGPMGPOLinksCollection_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x189d7b68_16bd_4d0d_a2ec_2e6aa2288c7f);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMGPOLinksCollection_Vtbl {
    pub base: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lindex: i32, pval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Item: usize,
    #[cfg(feature = "Win32_System_Ole")]
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppigpmlinks: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    _NewEnum: usize,
}
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IGPMMapEntry(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IGPMMapEntry {
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Source(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Source)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Destination(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Destination)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn DestinationOption(&self) -> ::windows::core::Result<GPMDestinationOption> {
        let mut result__: GPMDestinationOption = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).DestinationOption)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMDestinationOption>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn EntryType(&self) -> ::windows::core::Result<GPMEntryType> {
        let mut result__: GPMEntryType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).EntryType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMEntryType>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGPMMapEntry> for ::windows::core::IUnknown {
    fn from(value: IGPMMapEntry) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGPMMapEntry> for ::windows::core::IUnknown {
    fn from(value: &IGPMMapEntry) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IGPMMapEntry {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IGPMMapEntry {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IGPMMapEntry {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a IGPMMapEntry {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IGPMMapEntry {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IGPMMapEntry {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IGPMMapEntry {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IGPMMapEntry {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGPMMapEntry").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IGPMMapEntry {
    type Vtable = IGPMMapEntry_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8e79ad06_2381_4444_be4c_ff693e6e6f2b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMMapEntry_Vtbl {
    pub base: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Source: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrsource: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Source: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Destination: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdestination: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Destination: usize,
    pub DestinationOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pgpmdestoption: *mut GPMDestinationOption) -> ::windows::core::HRESULT,
    pub EntryType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pgpmentrytype: *mut GPMEntryType) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IGPMMapEntryCollection(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IGPMMapEntryCollection {
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Count)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Item(&self, lindex: i32) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Item)(::core::mem::transmute_copy(self), ::core::mem::transmute(lindex), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<super::Ole::IEnumVARIANT> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self)._NewEnum)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Ole::IEnumVARIANT>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGPMMapEntryCollection> for ::windows::core::IUnknown {
    fn from(value: IGPMMapEntryCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGPMMapEntryCollection> for ::windows::core::IUnknown {
    fn from(value: &IGPMMapEntryCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IGPMMapEntryCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IGPMMapEntryCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IGPMMapEntryCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a IGPMMapEntryCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IGPMMapEntryCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IGPMMapEntryCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IGPMMapEntryCollection {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IGPMMapEntryCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGPMMapEntryCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IGPMMapEntryCollection {
    type Vtable = IGPMMapEntryCollection_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbb0bf49b_e53f_443f_b807_8be22bfb6d42);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMMapEntryCollection_Vtbl {
    pub base: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lindex: i32, pval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Item: usize,
    #[cfg(feature = "Win32_System_Ole")]
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    _NewEnum: usize,
}
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IGPMMigrationTable(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IGPMMigrationTable {
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Save<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrmigrationtablepath: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Save)(::core::mem::transmute_copy(self), bstrmigrationtablepath.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Add<'a, Param1: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, lflags: i32, var: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Add)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), var.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddEntry<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrsource: Param0, gpmentrytype: GPMEntryType, pvardestination: *const super::Com::VARIANT) -> ::windows::core::Result<IGPMMapEntry> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).AddEntry)(::core::mem::transmute_copy(self), bstrsource.into_param().abi(), ::core::mem::transmute(gpmentrytype), ::core::mem::transmute(pvardestination), ::core::mem::transmute(&mut result__)).from_abi::<IGPMMapEntry>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetEntry<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrsource: Param0) -> ::windows::core::Result<IGPMMapEntry> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetEntry)(::core::mem::transmute_copy(self), bstrsource.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IGPMMapEntry>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeleteEntry<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrsource: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DeleteEntry)(::core::mem::transmute_copy(self), bstrsource.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn UpdateDestination<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrsource: Param0, pvardestination: *const super::Com::VARIANT) -> ::windows::core::Result<IGPMMapEntry> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).UpdateDestination)(::core::mem::transmute_copy(self), bstrsource.into_param().abi(), ::core::mem::transmute(pvardestination), ::core::mem::transmute(&mut result__)).from_abi::<IGPMMapEntry>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Validate(&self) -> ::windows::core::Result<IGPMResult> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Validate)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IGPMResult>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetEntries(&self) -> ::windows::core::Result<IGPMMapEntryCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetEntries)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IGPMMapEntryCollection>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGPMMigrationTable> for ::windows::core::IUnknown {
    fn from(value: IGPMMigrationTable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGPMMigrationTable> for ::windows::core::IUnknown {
    fn from(value: &IGPMMigrationTable) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IGPMMigrationTable {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IGPMMigrationTable {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IGPMMigrationTable {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a IGPMMigrationTable {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IGPMMigrationTable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IGPMMigrationTable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IGPMMigrationTable {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IGPMMigrationTable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGPMMigrationTable").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IGPMMigrationTable {
    type Vtable = IGPMMigrationTable_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x48f823b1_efaf_470b_b6ed_40d14ee1a4ec);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMMigrationTable_Vtbl {
    pub base: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Save: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrmigrationtablepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Save: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, var: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Add: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddEntry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrsource: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, gpmentrytype: GPMEntryType, pvardestination: *const super::Com::VARIANT, ppentry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddEntry: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetEntry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrsource: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppentry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetEntry: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DeleteEntry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrsource: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DeleteEntry: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub UpdateDestination: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrsource: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvardestination: *const super::Com::VARIANT, ppentry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    UpdateDestination: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Validate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Validate: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetEntries: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppentries: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetEntries: usize,
}
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IGPMPermission(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IGPMPermission {
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn Inherited(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Inherited)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn Inheritable(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Inheritable)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn Denied(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Denied)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn Permission(&self) -> ::windows::core::Result<GPMPermissionType> {
        let mut result__: GPMPermissionType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Permission)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMPermissionType>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Trustee(&self) -> ::windows::core::Result<IGPMTrustee> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Trustee)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IGPMTrustee>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGPMPermission> for ::windows::core::IUnknown {
    fn from(value: IGPMPermission) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGPMPermission> for ::windows::core::IUnknown {
    fn from(value: &IGPMPermission) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IGPMPermission {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IGPMPermission {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IGPMPermission {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a IGPMPermission {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IGPMPermission {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IGPMPermission {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IGPMPermission {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IGPMPermission {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGPMPermission").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IGPMPermission {
    type Vtable = IGPMPermission_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x35ebca40_e1a1_4a02_8905_d79416fb464a);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMPermission_Vtbl {
    pub base: super::Com::IDispatch_Vtbl,
    pub Inherited: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT,
    pub Inheritable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT,
    pub Denied: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT,
    pub Permission: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Trustee: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppigpmtrustee: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Trustee: usize,
}
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IGPMRSOP(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IGPMRSOP {
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn Mode(&self) -> ::windows::core::Result<GPMRSOPMode> {
        let mut result__: GPMRSOPMode = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Mode)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMRSOPMode>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Namespace(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Namespace)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLoggingComputer<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrval: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetLoggingComputer)(::core::mem::transmute_copy(self), bstrval.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LoggingComputer(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).LoggingComputer)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLoggingUser<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrval: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetLoggingUser)(::core::mem::transmute_copy(self), bstrval.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LoggingUser(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).LoggingUser)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn SetLoggingFlags(&self, lval: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetLoggingFlags)(::core::mem::transmute_copy(self), ::core::mem::transmute(lval)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn LoggingFlags(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).LoggingFlags)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn SetPlanningFlags(&self, lval: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPlanningFlags)(::core::mem::transmute_copy(self), ::core::mem::transmute(lval)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn PlanningFlags(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).PlanningFlags)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPlanningDomainController<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrval: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPlanningDomainController)(::core::mem::transmute_copy(self), bstrval.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PlanningDomainController(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).PlanningDomainController)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPlanningSiteName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrval: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPlanningSiteName)(::core::mem::transmute_copy(self), bstrval.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PlanningSiteName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).PlanningSiteName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPlanningUser<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrval: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPlanningUser)(::core::mem::transmute_copy(self), bstrval.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PlanningUser(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).PlanningUser)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPlanningUserSOM<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrval: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPlanningUserSOM)(::core::mem::transmute_copy(self), bstrval.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PlanningUserSOM(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).PlanningUserSOM)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetPlanningUserWMIFilters<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, varval: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPlanningUserWMIFilters)(::core::mem::transmute_copy(self), varval.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PlanningUserWMIFilters(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).PlanningUserWMIFilters)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetPlanningUserSecurityGroups<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, varval: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPlanningUserSecurityGroups)(::core::mem::transmute_copy(self), varval.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PlanningUserSecurityGroups(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).PlanningUserSecurityGroups)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPlanningComputer<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrval: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPlanningComputer)(::core::mem::transmute_copy(self), bstrval.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PlanningComputer(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).PlanningComputer)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPlanningComputerSOM<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrval: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPlanningComputerSOM)(::core::mem::transmute_copy(self), bstrval.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PlanningComputerSOM(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).PlanningComputerSOM)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetPlanningComputerWMIFilters<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, varval: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPlanningComputerWMIFilters)(::core::mem::transmute_copy(self), varval.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PlanningComputerWMIFilters(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).PlanningComputerWMIFilters)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetPlanningComputerSecurityGroups<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, varval: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPlanningComputerSecurityGroups)(::core::mem::transmute_copy(self), varval.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PlanningComputerSecurityGroups(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).PlanningComputerSecurityGroups)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn LoggingEnumerateUsers(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).LoggingEnumerateUsers)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn CreateQueryResults(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CreateQueryResults)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn ReleaseQueryResults(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReleaseQueryResults)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GenerateReport(&self, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GenerateReport)(::core::mem::transmute_copy(self), ::core::mem::transmute(gpmreporttype), ::core::mem::transmute(pvargpmprogress), ::core::mem::transmute(pvargpmcancel), ::core::mem::transmute(ppigpmresult)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GenerateReportToFile<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, gpmreporttype: GPMReportType, bstrtargetfilepath: Param1) -> ::windows::core::Result<IGPMResult> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GenerateReportToFile)(::core::mem::transmute_copy(self), ::core::mem::transmute(gpmreporttype), bstrtargetfilepath.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IGPMResult>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGPMRSOP> for ::windows::core::IUnknown {
    fn from(value: IGPMRSOP) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGPMRSOP> for ::windows::core::IUnknown {
    fn from(value: &IGPMRSOP) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IGPMRSOP {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IGPMRSOP {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IGPMRSOP {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a IGPMRSOP {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IGPMRSOP {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IGPMRSOP {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IGPMRSOP {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IGPMRSOP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGPMRSOP").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IGPMRSOP {
    type Vtable = IGPMRSOP_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x49ed785a_3237_4ff2_b1f0_fdf5a8d5a1ee);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMRSOP_Vtbl {
    pub base: super::Com::IDispatch_Vtbl,
    pub Mode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMRSOPMode) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Namespace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Namespace: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetLoggingComputer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetLoggingComputer: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub LoggingComputer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LoggingComputer: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetLoggingUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetLoggingUser: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub LoggingUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LoggingUser: usize,
    pub SetLoggingFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lval: i32) -> ::windows::core::HRESULT,
    pub LoggingFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lval: *mut i32) -> ::windows::core::HRESULT,
    pub SetPlanningFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lval: i32) -> ::windows::core::HRESULT,
    pub PlanningFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lval: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetPlanningDomainController: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetPlanningDomainController: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub PlanningDomainController: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PlanningDomainController: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetPlanningSiteName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetPlanningSiteName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub PlanningSiteName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PlanningSiteName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetPlanningUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetPlanningUser: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub PlanningUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PlanningUser: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetPlanningUserSOM: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetPlanningUserSOM: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub PlanningUserSOM: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PlanningUserSOM: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetPlanningUserWMIFilters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varval: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetPlanningUserWMIFilters: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PlanningUserWMIFilters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PlanningUserWMIFilters: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetPlanningUserSecurityGroups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varval: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetPlanningUserSecurityGroups: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PlanningUserSecurityGroups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PlanningUserSecurityGroups: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetPlanningComputer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetPlanningComputer: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub PlanningComputer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PlanningComputer: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetPlanningComputerSOM: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetPlanningComputerSOM: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub PlanningComputerSOM: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PlanningComputerSOM: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetPlanningComputerWMIFilters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varval: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetPlanningComputerWMIFilters: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PlanningComputerWMIFilters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PlanningComputerWMIFilters: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetPlanningComputerSecurityGroups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varval: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetPlanningComputerSecurityGroups: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PlanningComputerSecurityGroups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PlanningComputerSecurityGroups: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub LoggingEnumerateUsers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    LoggingEnumerateUsers: usize,
    pub CreateQueryResults: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ReleaseQueryResults: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GenerateReport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GenerateReport: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GenerateReportToFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gpmreporttype: GPMReportType, bstrtargetfilepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppigpmresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GenerateReportToFile: usize,
}
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IGPMResult(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IGPMResult {
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Status(&self) -> ::windows::core::Result<IGPMStatusMsgCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Status)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IGPMStatusMsgCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Result(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Result)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn OverallStatus(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OverallStatus)(::core::mem::transmute_copy(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGPMResult> for ::windows::core::IUnknown {
    fn from(value: IGPMResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGPMResult> for ::windows::core::IUnknown {
    fn from(value: &IGPMResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IGPMResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IGPMResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IGPMResult {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a IGPMResult {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IGPMResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IGPMResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IGPMResult {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IGPMResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGPMResult").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IGPMResult {
    type Vtable = IGPMResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x86dff7e9_f76f_42ab_9570_cebc6be8a52d);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMResult_Vtbl {
    pub base: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppigpmstatusmsgcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Status: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Result: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarresult: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Result: usize,
    pub OverallStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IGPMSOM(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IGPMSOM {
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn GPOInheritanceBlocked(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GPOInheritanceBlocked)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn SetGPOInheritanceBlocked(&self, newval: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetGPOInheritanceBlocked)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Name)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Path(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Path)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateGPOLink<'a, Param1: ::windows::core::IntoParam<'a, IGPMGPO>>(&self, llinkpos: i32, pgpo: Param1) -> ::windows::core::Result<IGPMGPOLink> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateGPOLink)(::core::mem::transmute_copy(self), ::core::mem::transmute(llinkpos), pgpo.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IGPMGPOLink>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn Type(&self) -> ::windows::core::Result<GPMSOMType> {
        let mut result__: GPMSOMType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Type)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMSOMType>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetGPOLinks(&self) -> ::windows::core::Result<IGPMGPOLinksCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetGPOLinks)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IGPMGPOLinksCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetInheritedGPOLinks(&self) -> ::windows::core::Result<IGPMGPOLinksCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetInheritedGPOLinks)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IGPMGPOLinksCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSecurityInfo(&self) -> ::windows::core::Result<IGPMSecurityInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSecurityInfo)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IGPMSecurityInfo>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSecurityInfo<'a, Param0: ::windows::core::IntoParam<'a, IGPMSecurityInfo>>(&self, psecurityinfo: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSecurityInfo)(::core::mem::transmute_copy(self), psecurityinfo.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGPMSOM> for ::windows::core::IUnknown {
    fn from(value: IGPMSOM) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGPMSOM> for ::windows::core::IUnknown {
    fn from(value: &IGPMSOM) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IGPMSOM {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IGPMSOM {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IGPMSOM {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a IGPMSOM {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IGPMSOM {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IGPMSOM {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IGPMSOM {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IGPMSOM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGPMSOM").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IGPMSOM {
    type Vtable = IGPMSOM_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc0a7f09e_05a1_4f0c_8158_9e5c33684f6b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMSOM_Vtbl {
    pub base: super::Com::IDispatch_Vtbl,
    pub GPOInheritanceBlocked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT,
    pub SetGPOInheritanceBlocked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Path: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Path: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateGPOLink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, llinkpos: i32, pgpo: ::windows::core::RawPtr, ppnewgpolink: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateGPOLink: usize,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMSOMType) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetGPOLinks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppgpolinks: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetGPOLinks: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetInheritedGPOLinks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppgpolinks: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetInheritedGPOLinks: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetSecurityInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsecurityinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetSecurityInfo: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetSecurityInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psecurityinfo: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetSecurityInfo: usize,
}
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IGPMSOMCollection(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IGPMSOMCollection {
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Count)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Item(&self, lindex: i32) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Item)(::core::mem::transmute_copy(self), ::core::mem::transmute(lindex), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<super::Ole::IEnumVARIANT> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self)._NewEnum)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Ole::IEnumVARIANT>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGPMSOMCollection> for ::windows::core::IUnknown {
    fn from(value: IGPMSOMCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGPMSOMCollection> for ::windows::core::IUnknown {
    fn from(value: &IGPMSOMCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IGPMSOMCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IGPMSOMCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IGPMSOMCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a IGPMSOMCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IGPMSOMCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IGPMSOMCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IGPMSOMCollection {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IGPMSOMCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGPMSOMCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IGPMSOMCollection {
    type Vtable = IGPMSOMCollection_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xadc1688e_00e4_4495_abba_bed200df0cab);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMSOMCollection_Vtbl {
    pub base: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lindex: i32, pval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Item: usize,
    #[cfg(feature = "Win32_System_Ole")]
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppigpmsom: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    _NewEnum: usize,
}
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IGPMSearchCriteria(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IGPMSearchCriteria {
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Add<'a, Param2: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, searchproperty: GPMSearchProperty, searchoperation: GPMSearchOperation, varvalue: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Add)(::core::mem::transmute_copy(self), ::core::mem::transmute(searchproperty), ::core::mem::transmute(searchoperation), varvalue.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGPMSearchCriteria> for ::windows::core::IUnknown {
    fn from(value: IGPMSearchCriteria) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGPMSearchCriteria> for ::windows::core::IUnknown {
    fn from(value: &IGPMSearchCriteria) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IGPMSearchCriteria {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IGPMSearchCriteria {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IGPMSearchCriteria {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a IGPMSearchCriteria {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IGPMSearchCriteria {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IGPMSearchCriteria {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IGPMSearchCriteria {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IGPMSearchCriteria {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGPMSearchCriteria").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IGPMSearchCriteria {
    type Vtable = IGPMSearchCriteria_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd6f11c42_829b_48d4_83f5_3615b67dfc22);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMSearchCriteria_Vtbl {
    pub base: super::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, searchproperty: GPMSearchProperty, searchoperation: GPMSearchOperation, varvalue: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Add: usize,
}
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IGPMSecurityInfo(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IGPMSecurityInfo {
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Count)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Item(&self, lindex: i32) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Item)(::core::mem::transmute_copy(self), ::core::mem::transmute(lindex), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<super::Ole::IEnumVARIANT> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self)._NewEnum)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Ole::IEnumVARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Add<'a, Param0: ::windows::core::IntoParam<'a, IGPMPermission>>(&self, pperm: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Add)(::core::mem::transmute_copy(self), pperm.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Remove<'a, Param0: ::windows::core::IntoParam<'a, IGPMPermission>>(&self, pperm: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Remove)(::core::mem::transmute_copy(self), pperm.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemoveTrustee<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrtrustee: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveTrustee)(::core::mem::transmute_copy(self), bstrtrustee.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGPMSecurityInfo> for ::windows::core::IUnknown {
    fn from(value: IGPMSecurityInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGPMSecurityInfo> for ::windows::core::IUnknown {
    fn from(value: &IGPMSecurityInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IGPMSecurityInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IGPMSecurityInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IGPMSecurityInfo {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a IGPMSecurityInfo {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IGPMSecurityInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IGPMSecurityInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IGPMSecurityInfo {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IGPMSecurityInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGPMSecurityInfo").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IGPMSecurityInfo {
    type Vtable = IGPMSecurityInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb6c31ed4_1c93_4d3e_ae84_eb6d61161b60);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMSecurityInfo_Vtbl {
    pub base: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lindex: i32, pval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Item: usize,
    #[cfg(feature = "Win32_System_Ole")]
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    _NewEnum: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pperm: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pperm: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Remove: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RemoveTrustee: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrtrustee: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RemoveTrustee: usize,
}
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IGPMSitesContainer(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IGPMSitesContainer {
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DomainController(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).DomainController)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Domain(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Domain)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Forest(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Forest)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetSite<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrsitename: Param0) -> ::windows::core::Result<IGPMSOM> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSite)(::core::mem::transmute_copy(self), bstrsitename.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IGPMSOM>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SearchSites<'a, Param0: ::windows::core::IntoParam<'a, IGPMSearchCriteria>>(&self, pigpmsearchcriteria: Param0) -> ::windows::core::Result<IGPMSOMCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SearchSites)(::core::mem::transmute_copy(self), pigpmsearchcriteria.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IGPMSOMCollection>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGPMSitesContainer> for ::windows::core::IUnknown {
    fn from(value: IGPMSitesContainer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGPMSitesContainer> for ::windows::core::IUnknown {
    fn from(value: &IGPMSitesContainer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IGPMSitesContainer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IGPMSitesContainer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IGPMSitesContainer {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a IGPMSitesContainer {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IGPMSitesContainer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IGPMSitesContainer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IGPMSitesContainer {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IGPMSitesContainer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGPMSitesContainer").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IGPMSitesContainer {
    type Vtable = IGPMSitesContainer_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4725a899_2782_4d27_a6bb_d499246ffd72);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMSitesContainer_Vtbl {
    pub base: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub DomainController: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DomainController: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Domain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Domain: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Forest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Forest: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetSite: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrsitename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppsom: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetSite: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SearchSites: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pigpmsearchcriteria: ::windows::core::RawPtr, ppigpmsomcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SearchSites: usize,
}
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IGPMStarterGPO(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IGPMStarterGPO {
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DisplayName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).DisplayName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDisplayName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, newval: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDisplayName)(::core::mem::transmute_copy(self), newval.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Description)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, newval: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDescription)(::core::mem::transmute_copy(self), newval.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Author(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Author)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Product(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Product)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn CreationTime(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreationTime)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ID)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn ModifiedTime(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ModifiedTime)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn Type(&self) -> ::windows::core::Result<GPMStarterGPOType> {
        let mut result__: GPMStarterGPOType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Type)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMStarterGPOType>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn ComputerVersion(&self) -> ::windows::core::Result<u16> {
        let mut result__: u16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ComputerVersion)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn UserVersion(&self) -> ::windows::core::Result<u16> {
        let mut result__: u16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).UserVersion)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn StarterGPOVersion(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).StarterGPOVersion)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Delete)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Save<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrsavefile: Param0, boverwrite: i16, bsaveassystem: i16, bstrlanguage: *const super::Com::VARIANT, bstrauthor: *const super::Com::VARIANT, bstrproduct: *const super::Com::VARIANT, bstruniqueid: *const super::Com::VARIANT, bstrversion: *const super::Com::VARIANT, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Save)(::core::mem::transmute_copy(self), bstrsavefile.into_param().abi(), ::core::mem::transmute(boverwrite), ::core::mem::transmute(bsaveassystem), ::core::mem::transmute(bstrlanguage), ::core::mem::transmute(bstrauthor), ::core::mem::transmute(bstrproduct), ::core::mem::transmute(bstruniqueid), ::core::mem::transmute(bstrversion), ::core::mem::transmute(pvargpmprogress), ::core::mem::transmute(pvargpmcancel), ::core::mem::transmute(ppigpmresult)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Backup<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrbackupdir: Param0, bstrcomment: Param1, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Backup)(::core::mem::transmute_copy(self), bstrbackupdir.into_param().abi(), bstrcomment.into_param().abi(), ::core::mem::transmute(pvargpmprogress), ::core::mem::transmute(pvargpmcancel), ::core::mem::transmute(ppigpmresult)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CopyTo(&self, pvarnewdisplayname: *const super::Com::VARIANT, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *const super::Com::VARIANT) -> ::windows::core::Result<IGPMResult> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CopyTo)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvarnewdisplayname), ::core::mem::transmute(pvargpmprogress), ::core::mem::transmute(pvargpmcancel), ::core::mem::transmute(&mut result__)).from_abi::<IGPMResult>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GenerateReport(&self, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *const super::Com::VARIANT) -> ::windows::core::Result<IGPMResult> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GenerateReport)(::core::mem::transmute_copy(self), ::core::mem::transmute(gpmreporttype), ::core::mem::transmute(pvargpmprogress), ::core::mem::transmute(pvargpmcancel), ::core::mem::transmute(&mut result__)).from_abi::<IGPMResult>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GenerateReportToFile<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, gpmreporttype: GPMReportType, bstrtargetfilepath: Param1) -> ::windows::core::Result<IGPMResult> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GenerateReportToFile)(::core::mem::transmute_copy(self), ::core::mem::transmute(gpmreporttype), bstrtargetfilepath.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IGPMResult>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSecurityInfo(&self) -> ::windows::core::Result<IGPMSecurityInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSecurityInfo)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IGPMSecurityInfo>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSecurityInfo<'a, Param0: ::windows::core::IntoParam<'a, IGPMSecurityInfo>>(&self, psecurityinfo: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSecurityInfo)(::core::mem::transmute_copy(self), psecurityinfo.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGPMStarterGPO> for ::windows::core::IUnknown {
    fn from(value: IGPMStarterGPO) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGPMStarterGPO> for ::windows::core::IUnknown {
    fn from(value: &IGPMStarterGPO) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IGPMStarterGPO {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IGPMStarterGPO {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IGPMStarterGPO {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a IGPMStarterGPO {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IGPMStarterGPO {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IGPMStarterGPO {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IGPMStarterGPO {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IGPMStarterGPO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGPMStarterGPO").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IGPMStarterGPO {
    type Vtable = IGPMStarterGPO_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdfc3f61b_8880_4490_9337_d29c7ba8c2f0);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMStarterGPO_Vtbl {
    pub base: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DisplayName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDisplayName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Description: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDescription: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Author: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Author: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Product: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Product: usize,
    pub CreationTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ID: usize,
    pub ModifiedTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut f64) -> ::windows::core::HRESULT,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMStarterGPOType) -> ::windows::core::HRESULT,
    pub ComputerVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut u16) -> ::windows::core::HRESULT,
    pub UserVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub StarterGPOVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    StarterGPOVersion: usize,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Save: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrsavefile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, boverwrite: i16, bsaveassystem: i16, bstrlanguage: *const super::Com::VARIANT, bstrauthor: *const super::Com::VARIANT, bstrproduct: *const super::Com::VARIANT, bstruniqueid: *const super::Com::VARIANT, bstrversion: *const super::Com::VARIANT, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Save: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Backup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrbackupdir: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrcomment: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Backup: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CopyTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarnewdisplayname: *const super::Com::VARIANT, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *const super::Com::VARIANT, ppigpmresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CopyTo: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GenerateReport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *const super::Com::VARIANT, ppigpmresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GenerateReport: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GenerateReportToFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gpmreporttype: GPMReportType, bstrtargetfilepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppigpmresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GenerateReportToFile: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetSecurityInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsecurityinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetSecurityInfo: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetSecurityInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psecurityinfo: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetSecurityInfo: usize,
}
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IGPMStarterGPOBackup(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IGPMStarterGPOBackup {
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BackupDir(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).BackupDir)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Comment(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Comment)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DisplayName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).DisplayName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Domain(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Domain)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn StarterGPOID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).StarterGPOID)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ID)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn Timestamp(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Timestamp)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn Type(&self) -> ::windows::core::Result<GPMStarterGPOType> {
        let mut result__: GPMStarterGPOType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Type)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GPMStarterGPOType>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Delete)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GenerateReport(&self, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GenerateReport)(::core::mem::transmute_copy(self), ::core::mem::transmute(gpmreporttype), ::core::mem::transmute(pvargpmprogress), ::core::mem::transmute(pvargpmcancel), ::core::mem::transmute(ppigpmresult)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GenerateReportToFile<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, gpmreporttype: GPMReportType, bstrtargetfilepath: Param1) -> ::windows::core::Result<IGPMResult> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GenerateReportToFile)(::core::mem::transmute_copy(self), ::core::mem::transmute(gpmreporttype), bstrtargetfilepath.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IGPMResult>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGPMStarterGPOBackup> for ::windows::core::IUnknown {
    fn from(value: IGPMStarterGPOBackup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGPMStarterGPOBackup> for ::windows::core::IUnknown {
    fn from(value: &IGPMStarterGPOBackup) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IGPMStarterGPOBackup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IGPMStarterGPOBackup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IGPMStarterGPOBackup {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a IGPMStarterGPOBackup {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IGPMStarterGPOBackup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IGPMStarterGPOBackup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IGPMStarterGPOBackup {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IGPMStarterGPOBackup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGPMStarterGPOBackup").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IGPMStarterGPOBackup {
    type Vtable = IGPMStarterGPOBackup_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51d98eda_a87e_43dd_b80a_0b66ef1938d6);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMStarterGPOBackup_Vtbl {
    pub base: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub BackupDir: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrbackupdir: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    BackupDir: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Comment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrcomment: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Comment: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdisplayname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DisplayName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Domain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrtemplatedomain: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Domain: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub StarterGPOID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrtemplateid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    StarterGPOID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ID: usize,
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptimestamp: *mut f64) -> ::windows::core::HRESULT,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptype: *mut GPMStarterGPOType) -> ::windows::core::HRESULT,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GenerateReport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GenerateReport: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GenerateReportToFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gpmreporttype: GPMReportType, bstrtargetfilepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppigpmresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GenerateReportToFile: usize,
}
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IGPMStarterGPOBackupCollection(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IGPMStarterGPOBackupCollection {
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Count)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Item(&self, lindex: i32) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Item)(::core::mem::transmute_copy(self), ::core::mem::transmute(lindex), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<super::Ole::IEnumVARIANT> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self)._NewEnum)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Ole::IEnumVARIANT>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGPMStarterGPOBackupCollection> for ::windows::core::IUnknown {
    fn from(value: IGPMStarterGPOBackupCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGPMStarterGPOBackupCollection> for ::windows::core::IUnknown {
    fn from(value: &IGPMStarterGPOBackupCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IGPMStarterGPOBackupCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IGPMStarterGPOBackupCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IGPMStarterGPOBackupCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a IGPMStarterGPOBackupCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IGPMStarterGPOBackupCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IGPMStarterGPOBackupCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IGPMStarterGPOBackupCollection {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IGPMStarterGPOBackupCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGPMStarterGPOBackupCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IGPMStarterGPOBackupCollection {
    type Vtable = IGPMStarterGPOBackupCollection_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc998031d_add0_4bb5_8dea_298505d8423b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMStarterGPOBackupCollection_Vtbl {
    pub base: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lindex: i32, pval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Item: usize,
    #[cfg(feature = "Win32_System_Ole")]
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppigpmtmplbackup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    _NewEnum: usize,
}
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IGPMStarterGPOCollection(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IGPMStarterGPOCollection {
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Count)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Item(&self, lindex: i32) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Item)(::core::mem::transmute_copy(self), ::core::mem::transmute(lindex), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<super::Ole::IEnumVARIANT> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self)._NewEnum)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Ole::IEnumVARIANT>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGPMStarterGPOCollection> for ::windows::core::IUnknown {
    fn from(value: IGPMStarterGPOCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGPMStarterGPOCollection> for ::windows::core::IUnknown {
    fn from(value: &IGPMStarterGPOCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IGPMStarterGPOCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IGPMStarterGPOCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IGPMStarterGPOCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a IGPMStarterGPOCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IGPMStarterGPOCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IGPMStarterGPOCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IGPMStarterGPOCollection {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IGPMStarterGPOCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGPMStarterGPOCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IGPMStarterGPOCollection {
    type Vtable = IGPMStarterGPOCollection_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2e522729_2219_44ad_933a_64dfd650c423);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMStarterGPOCollection_Vtbl {
    pub base: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lindex: i32, pval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Item: usize,
    #[cfg(feature = "Win32_System_Ole")]
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppigpmtemplates: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    _NewEnum: usize,
}
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IGPMStatusMessage(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IGPMStatusMessage {
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ObjectPath(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ObjectPath)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn ErrorCode(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ErrorCode)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExtensionName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ExtensionName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SettingsName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SettingsName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn OperationCode(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OperationCode)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Message(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Message)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGPMStatusMessage> for ::windows::core::IUnknown {
    fn from(value: IGPMStatusMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGPMStatusMessage> for ::windows::core::IUnknown {
    fn from(value: &IGPMStatusMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IGPMStatusMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IGPMStatusMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IGPMStatusMessage {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a IGPMStatusMessage {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IGPMStatusMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IGPMStatusMessage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IGPMStatusMessage {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IGPMStatusMessage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGPMStatusMessage").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IGPMStatusMessage {
    type Vtable = IGPMStatusMessage_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8496c22f_f3de_4a1f_8f58_603caaa93d7b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMStatusMessage_Vtbl {
    pub base: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub ObjectPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ObjectPath: usize,
    pub ErrorCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ExtensionName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ExtensionName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SettingsName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SettingsName: usize,
    pub OperationCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Message: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Message: usize,
}
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IGPMStatusMsgCollection(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IGPMStatusMsgCollection {
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Count)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Item(&self, lindex: i32) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Item)(::core::mem::transmute_copy(self), ::core::mem::transmute(lindex), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<super::Ole::IEnumVARIANT> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self)._NewEnum)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Ole::IEnumVARIANT>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGPMStatusMsgCollection> for ::windows::core::IUnknown {
    fn from(value: IGPMStatusMsgCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGPMStatusMsgCollection> for ::windows::core::IUnknown {
    fn from(value: &IGPMStatusMsgCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IGPMStatusMsgCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IGPMStatusMsgCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IGPMStatusMsgCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a IGPMStatusMsgCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IGPMStatusMsgCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IGPMStatusMsgCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IGPMStatusMsgCollection {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IGPMStatusMsgCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGPMStatusMsgCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IGPMStatusMsgCollection {
    type Vtable = IGPMStatusMsgCollection_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b6e1af0_1a92_40f3_a59d_f36ac1f728b7);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMStatusMsgCollection_Vtbl {
    pub base: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lindex: i32, pval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Item: usize,
    #[cfg(feature = "Win32_System_Ole")]
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    _NewEnum: usize,
}
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IGPMTrustee(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IGPMTrustee {
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TrusteeSid(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).TrusteeSid)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TrusteeName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).TrusteeName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TrusteeDomain(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).TrusteeDomain)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TrusteeDSPath(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).TrusteeDSPath)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn TrusteeType(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).TrusteeType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGPMTrustee> for ::windows::core::IUnknown {
    fn from(value: IGPMTrustee) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGPMTrustee> for ::windows::core::IUnknown {
    fn from(value: &IGPMTrustee) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IGPMTrustee {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IGPMTrustee {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IGPMTrustee {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a IGPMTrustee {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IGPMTrustee {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IGPMTrustee {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IGPMTrustee {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IGPMTrustee {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGPMTrustee").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IGPMTrustee {
    type Vtable = IGPMTrustee_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3b466da8_c1a4_4b2a_999a_befcdd56cefb);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMTrustee_Vtbl {
    pub base: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub TrusteeSid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TrusteeSid: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub TrusteeName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TrusteeName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub TrusteeDomain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TrusteeDomain: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub TrusteeDSPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TrusteeDSPath: usize,
    pub TrusteeType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lval: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IGPMWMIFilter(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IGPMWMIFilter {
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Path(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Path)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, newval: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetName)(::core::mem::transmute_copy(self), newval.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Name)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, newval: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDescription)(::core::mem::transmute_copy(self), newval.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Description)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetQueryList(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetQueryList)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSecurityInfo(&self) -> ::windows::core::Result<IGPMSecurityInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSecurityInfo)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IGPMSecurityInfo>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSecurityInfo<'a, Param0: ::windows::core::IntoParam<'a, IGPMSecurityInfo>>(&self, psecurityinfo: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSecurityInfo)(::core::mem::transmute_copy(self), psecurityinfo.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGPMWMIFilter> for ::windows::core::IUnknown {
    fn from(value: IGPMWMIFilter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGPMWMIFilter> for ::windows::core::IUnknown {
    fn from(value: &IGPMWMIFilter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IGPMWMIFilter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IGPMWMIFilter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IGPMWMIFilter {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a IGPMWMIFilter {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IGPMWMIFilter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IGPMWMIFilter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IGPMWMIFilter {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IGPMWMIFilter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGPMWMIFilter").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IGPMWMIFilter {
    type Vtable = IGPMWMIFilter_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xef2ff9b4_3c27_459a_b979_038305cec75d);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMWMIFilter_Vtbl {
    pub base: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Path: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Path: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDescription: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Description: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetQueryList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pqrylist: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetQueryList: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetSecurityInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsecurityinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetSecurityInfo: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetSecurityInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psecurityinfo: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetSecurityInfo: usize,
}
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IGPMWMIFilterCollection(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IGPMWMIFilterCollection {
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Count)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Item(&self, lindex: i32) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Item)(::core::mem::transmute_copy(self), ::core::mem::transmute(lindex), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<super::Ole::IEnumVARIANT> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self)._NewEnum)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Ole::IEnumVARIANT>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGPMWMIFilterCollection> for ::windows::core::IUnknown {
    fn from(value: IGPMWMIFilterCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGPMWMIFilterCollection> for ::windows::core::IUnknown {
    fn from(value: &IGPMWMIFilterCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IGPMWMIFilterCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IGPMWMIFilterCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IGPMWMIFilterCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a IGPMWMIFilterCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IGPMWMIFilterCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IGPMWMIFilterCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IGPMWMIFilterCollection {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IGPMWMIFilterCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGPMWMIFilterCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IGPMWMIFilterCollection {
    type Vtable = IGPMWMIFilterCollection_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5782d582_1a36_4661_8a94_c3c32551945b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMWMIFilterCollection_Vtbl {
    pub base: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lindex: i32, pval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Item: usize,
    #[cfg(feature = "Win32_System_Ole")]
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    _NewEnum: usize,
}
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
#[repr(transparent)]
pub struct IGroupPolicyObject(::windows::core::IUnknown);
impl IGroupPolicyObject {
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn New<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pszdomainname: Param0, pszdisplayname: Param1, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).New)(::core::mem::transmute_copy(self), pszdomainname.into_param().abi(), pszdisplayname.into_param().abi(), ::core::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn OpenDSGPO<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pszpath: Param0, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OpenDSGPO)(::core::mem::transmute_copy(self), pszpath.into_param().abi(), ::core::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn OpenLocalMachineGPO(&self, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OpenLocalMachineGPO)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn OpenRemoteMachineGPO<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pszcomputername: Param0, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OpenRemoteMachineGPO)(::core::mem::transmute_copy(self), pszcomputername.into_param().abi(), ::core::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Save<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bmachine: Param0, badd: Param1, pguidextension: *mut ::windows::core::GUID, pguid: *mut ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Save)(::core::mem::transmute_copy(self), bmachine.into_param().abi(), badd.into_param().abi(), ::core::mem::transmute(pguidextension), ::core::mem::transmute(pguid)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Delete)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn GetName(&self, pszname: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetName)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pszname)), pszname.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn GetDisplayName(&self, pszname: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDisplayName)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pszname)), pszname.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn SetDisplayName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pszname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDisplayName)(::core::mem::transmute_copy(self), pszname.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn GetPath(&self, pszpath: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPath)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pszpath)), pszpath.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn GetDSPath(&self, dwsection: u32, pszpath: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDSPath)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwsection), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pszpath)), pszpath.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn GetFileSysPath(&self, dwsection: u32, pszpath: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetFileSysPath)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwsection), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pszpath)), pszpath.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Registry\"`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub unsafe fn GetRegistryKey(&self, dwsection: u32, hkey: *mut super::Registry::HKEY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetRegistryKey)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwsection), ::core::mem::transmute(hkey)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn GetOptions(&self, dwoptions: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetOptions)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoptions)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn SetOptions(&self, dwoptions: u32, dwmask: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetOptions)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoptions), ::core::mem::transmute(dwmask)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn GetType(&self, gpotype: *mut GROUP_POLICY_OBJECT_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetType)(::core::mem::transmute_copy(self), ::core::mem::transmute(gpotype)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn GetMachineName(&self, pszname: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetMachineName)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pszname)), pszname.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_UI_Controls\"`*"]
    #[cfg(feature = "Win32_UI_Controls")]
    pub unsafe fn GetPropertySheetPages(&self, hpages: *mut *mut super::super::UI::Controls::HPROPSHEETPAGE, upagecount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPropertySheetPages)(::core::mem::transmute_copy(self), ::core::mem::transmute(hpages), ::core::mem::transmute(upagecount)).ok()
    }
}
impl ::core::convert::From<IGroupPolicyObject> for ::windows::core::IUnknown {
    fn from(value: IGroupPolicyObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IGroupPolicyObject> for ::windows::core::IUnknown {
    fn from(value: &IGroupPolicyObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IGroupPolicyObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IGroupPolicyObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IGroupPolicyObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IGroupPolicyObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGroupPolicyObject {}
impl ::core::fmt::Debug for IGroupPolicyObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGroupPolicyObject").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IGroupPolicyObject {
    type Vtable = IGroupPolicyObject_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xea502723_a23d_11d1_a7d3_0000f87571e3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGroupPolicyObject_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub New: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszdomainname: ::windows::core::PCWSTR, pszdisplayname: ::windows::core::PCWSTR, dwflags: u32) -> ::windows::core::HRESULT,
    pub OpenDSGPO: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows::core::PCWSTR, dwflags: u32) -> ::windows::core::HRESULT,
    pub OpenLocalMachineGPO: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT,
    pub OpenRemoteMachineGPO: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszcomputername: ::windows::core::PCWSTR, dwflags: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Save: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bmachine: super::super::Foundation::BOOL, badd: super::super::Foundation::BOOL, pguidextension: *mut ::windows::core::GUID, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Save: usize,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszname: ::windows::core::PWSTR, cchmaxlength: i32) -> ::windows::core::HRESULT,
    pub GetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszname: ::windows::core::PWSTR, cchmaxlength: i32) -> ::windows::core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows::core::PWSTR, cchmaxlength: i32) -> ::windows::core::HRESULT,
    pub GetDSPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwsection: u32, pszpath: ::windows::core::PWSTR, cchmaxpath: i32) -> ::windows::core::HRESULT,
    pub GetFileSysPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwsection: u32, pszpath: ::windows::core::PWSTR, cchmaxpath: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Registry")]
    pub GetRegistryKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwsection: u32, hkey: *mut super::Registry::HKEY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Registry"))]
    GetRegistryKey: usize,
    pub GetOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoptions: *mut u32) -> ::windows::core::HRESULT,
    pub SetOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoptions: u32, dwmask: u32) -> ::windows::core::HRESULT,
    pub GetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gpotype: *mut GROUP_POLICY_OBJECT_TYPE) -> ::windows::core::HRESULT,
    pub GetMachineName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszname: ::windows::core::PWSTR, cchmaxlength: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_UI_Controls")]
    pub GetPropertySheetPages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hpages: *mut *mut super::super::UI::Controls::HPROPSHEETPAGE, upagecount: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Controls"))]
    GetPropertySheetPages: usize,
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub struct INSTALLDATA {
    pub Type: INSTALLSPECTYPE,
    pub Spec: INSTALLSPEC,
}
impl ::core::marker::Copy for INSTALLDATA {}
impl ::core::clone::Clone for INSTALLDATA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for INSTALLDATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for INSTALLDATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<INSTALLDATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for INSTALLDATA {}
impl ::core::default::Default for INSTALLDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub union INSTALLSPEC {
    pub AppName: INSTALLSPEC_0,
    pub FileExt: ::windows::core::PWSTR,
    pub ProgId: ::windows::core::PWSTR,
    pub COMClass: INSTALLSPEC_1,
}
impl ::core::marker::Copy for INSTALLSPEC {}
impl ::core::clone::Clone for INSTALLSPEC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for INSTALLSPEC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for INSTALLSPEC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<INSTALLSPEC>()) == 0 }
    }
}
impl ::core::cmp::Eq for INSTALLSPEC {}
impl ::core::default::Default for INSTALLSPEC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub struct INSTALLSPEC_0 {
    pub Name: ::windows::core::PWSTR,
    pub GPOId: ::windows::core::GUID,
}
impl ::core::marker::Copy for INSTALLSPEC_0 {}
impl ::core::clone::Clone for INSTALLSPEC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for INSTALLSPEC_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INSTALLSPEC_0").field("Name", &self.Name).field("GPOId", &self.GPOId).finish()
    }
}
unsafe impl ::windows::core::Abi for INSTALLSPEC_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for INSTALLSPEC_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<INSTALLSPEC_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for INSTALLSPEC_0 {}
impl ::core::default::Default for INSTALLSPEC_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub struct INSTALLSPEC_1 {
    pub Clsid: ::windows::core::GUID,
    pub ClsCtx: u32,
}
impl ::core::marker::Copy for INSTALLSPEC_1 {}
impl ::core::clone::Clone for INSTALLSPEC_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for INSTALLSPEC_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INSTALLSPEC_1").field("Clsid", &self.Clsid).field("ClsCtx", &self.ClsCtx).finish()
    }
}
unsafe impl ::windows::core::Abi for INSTALLSPEC_1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for INSTALLSPEC_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<INSTALLSPEC_1>()) == 0 }
    }
}
impl ::core::cmp::Eq for INSTALLSPEC_1 {}
impl ::core::default::Default for INSTALLSPEC_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct INSTALLSPECTYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const APPNAME: INSTALLSPECTYPE = INSTALLSPECTYPE(1i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const FILEEXT: INSTALLSPECTYPE = INSTALLSPECTYPE(2i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const PROGID: INSTALLSPECTYPE = INSTALLSPECTYPE(3i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const COMCLASS: INSTALLSPECTYPE = INSTALLSPECTYPE(4i32);
impl ::core::marker::Copy for INSTALLSPECTYPE {}
impl ::core::clone::Clone for INSTALLSPECTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for INSTALLSPECTYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for INSTALLSPECTYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for INSTALLSPECTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INSTALLSPECTYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
#[repr(transparent)]
pub struct IRSOPInformation(::windows::core::IUnknown);
impl IRSOPInformation {
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn GetNamespace(&self, dwsection: u32, pszname: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetNamespace)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwsection), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pszname)), pszname.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn GetFlags(&self, pdwflags: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetFlags)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwflags)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub unsafe fn GetEventLogEntryText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pszeventsource: Param0, pszeventlogname: Param1, pszeventtime: Param2, dweventid: u32) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetEventLogEntryText)(::core::mem::transmute_copy(self), pszeventsource.into_param().abi(), pszeventlogname.into_param().abi(), pszeventtime.into_param().abi(), ::core::mem::transmute(dweventid), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
}
impl ::core::convert::From<IRSOPInformation> for ::windows::core::IUnknown {
    fn from(value: IRSOPInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRSOPInformation> for ::windows::core::IUnknown {
    fn from(value: &IRSOPInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRSOPInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRSOPInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRSOPInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRSOPInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRSOPInformation {}
impl ::core::fmt::Debug for IRSOPInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRSOPInformation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRSOPInformation {
    type Vtable = IRSOPInformation_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9a5a81b5_d9c7_49ef_9d11_ddf50968c48d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRSOPInformation_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetNamespace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwsection: u32, pszname: ::windows::core::PWSTR, cchmaxlength: i32) -> ::windows::core::HRESULT,
    pub GetFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT,
    pub GetEventLogEntryText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszeventsource: ::windows::core::PCWSTR, pszeventlogname: ::windows::core::PCWSTR, pszeventtime: ::windows::core::PCWSTR, dweventid: u32, ppsztext: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
#[inline]
pub unsafe fn ImportRSoPData<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(lpnamespace: Param0, lpfilename: Param1) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImportRSoPData(lpnamespace: ::windows::core::PCWSTR, lpfilename: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
        }
        ImportRSoPData(lpnamespace.into_param().abi(), lpfilename.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
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
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub struct LOCALMANAGEDAPPLICATION {
    pub pszDeploymentName: ::windows::core::PWSTR,
    pub pszPolicyName: ::windows::core::PWSTR,
    pub pszProductId: ::windows::core::PWSTR,
    pub dwState: u32,
}
impl ::core::marker::Copy for LOCALMANAGEDAPPLICATION {}
impl ::core::clone::Clone for LOCALMANAGEDAPPLICATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for LOCALMANAGEDAPPLICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LOCALMANAGEDAPPLICATION").field("pszDeploymentName", &self.pszDeploymentName).field("pszPolicyName", &self.pszPolicyName).field("pszProductId", &self.pszProductId).field("dwState", &self.dwState).finish()
    }
}
unsafe impl ::windows::core::Abi for LOCALMANAGEDAPPLICATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for LOCALMANAGEDAPPLICATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<LOCALMANAGEDAPPLICATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for LOCALMANAGEDAPPLICATION {}
impl ::core::default::Default for LOCALMANAGEDAPPLICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const LOCALSTATE_ASSIGNED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const LOCALSTATE_ORPHANED: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const LOCALSTATE_POLICYREMOVE_ORPHAN: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const LOCALSTATE_POLICYREMOVE_UNINSTALL: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const LOCALSTATE_PUBLISHED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const LOCALSTATE_UNINSTALLED: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const LOCALSTATE_UNINSTALL_UNMANAGED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LeaveCriticalPolicySection<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hsection: Param0) -> super::super::Foundation::BOOL {
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
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MANAGEDAPPLICATION {
    pub pszPackageName: ::windows::core::PWSTR,
    pub pszPublisher: ::windows::core::PWSTR,
    pub dwVersionHi: u32,
    pub dwVersionLo: u32,
    pub dwRevision: u32,
    pub GpoId: ::windows::core::GUID,
    pub pszPolicyName: ::windows::core::PWSTR,
    pub ProductId: ::windows::core::GUID,
    pub Language: u16,
    pub pszOwner: ::windows::core::PWSTR,
    pub pszCompany: ::windows::core::PWSTR,
    pub pszComments: ::windows::core::PWSTR,
    pub pszContact: ::windows::core::PWSTR,
    pub pszSupportUrl: ::windows::core::PWSTR,
    pub dwPathType: u32,
    pub bInstalled: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MANAGEDAPPLICATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MANAGEDAPPLICATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MANAGEDAPPLICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MANAGEDAPPLICATION")
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
unsafe impl ::windows::core::Abi for MANAGEDAPPLICATION {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MANAGEDAPPLICATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MANAGEDAPPLICATION>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MANAGEDAPPLICATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MANAGEDAPPLICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const MANAGED_APPS_FROMCATEGORY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const MANAGED_APPS_INFOLEVEL_DEFAULT: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const MANAGED_APPS_USERAPPLICATIONS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const MANAGED_APPTYPE_SETUPEXE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const MANAGED_APPTYPE_UNSUPPORTED: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const MANAGED_APPTYPE_WINDOWSINSTALLER: u32 = 1u32;
pub const NODEID_Machine: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8fc0b737_a0e1_11d1_a7d3_0000f87571e3);
pub const NODEID_MachineSWSettings: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8fc0b73a_a0e1_11d1_a7d3_0000f87571e3);
pub const NODEID_RSOPMachine: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbd4c1a2e_0b7a_4a62_a6b0_c0577539c97e);
pub const NODEID_RSOPMachineSWSettings: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6a76273e_eb8e_45db_94c5_25663a5f2c1a);
pub const NODEID_RSOPUser: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xab87364f_0cec_4cd8_9bf8_898f34628fb8);
pub const NODEID_RSOPUserSWSettings: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe52c5ce3_fd27_4402_84de_d9a5f2858910);
pub const NODEID_User: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8fc0b738_a0e1_11d1_a7d3_0000f87571e3);
pub const NODEID_UserSWSettings: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8fc0b73c_a0e1_11d1_a7d3_0000f87571e3);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Wmi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Wmi"))]
pub type PFNGENERATEGROUPPOLICY = ::core::option::Option<unsafe extern "system" fn(dwflags: u32, pbabort: *mut super::super::Foundation::BOOL, pwszsite: ::windows::core::PCWSTR, pcomputertarget: *const RSOP_TARGET, pusertarget: *const RSOP_TARGET) -> u32>;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub type PFNPROCESSGROUPPOLICY = ::core::option::Option<unsafe extern "system" fn(dwflags: u32, htoken: super::super::Foundation::HANDLE, hkeyroot: super::Registry::HKEY, pdeletedgpolist: *const GROUP_POLICY_OBJECTA, pchangedgpolist: *const GROUP_POLICY_OBJECTA, phandle: usize, pbabort: *mut super::super::Foundation::BOOL, pstatuscallback: PFNSTATUSMESSAGECALLBACK) -> u32>;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`, `\"Win32_System_Wmi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry", feature = "Win32_System_Wmi"))]
pub type PFNPROCESSGROUPPOLICYEX = ::core::option::Option<unsafe extern "system" fn(dwflags: u32, htoken: super::super::Foundation::HANDLE, hkeyroot: super::Registry::HKEY, pdeletedgpolist: *const GROUP_POLICY_OBJECTA, pchangedgpolist: *const GROUP_POLICY_OBJECTA, phandle: usize, pbabort: *mut super::super::Foundation::BOOL, pstatuscallback: PFNSTATUSMESSAGECALLBACK, pwbemservices: ::core::option::Option<super::Wmi::IWbemServices>, prsopstatus: *mut ::windows::core::HRESULT) -> u32>;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFNSTATUSMESSAGECALLBACK = ::core::option::Option<unsafe extern "system" fn(bverbose: super::super::Foundation::BOOL, lpmessage: ::windows::core::PCWSTR) -> u32>;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const PI_APPLYPOLICY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const PI_NOUI: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct POLICYSETTINGSTATUSINFO {
    pub szKey: ::windows::core::PWSTR,
    pub szEventSource: ::windows::core::PWSTR,
    pub szEventLogName: ::windows::core::PWSTR,
    pub dwEventID: u32,
    pub dwErrorCode: u32,
    pub status: SETTINGSTATUS,
    pub timeLogged: super::super::Foundation::SYSTEMTIME,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for POLICYSETTINGSTATUSINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for POLICYSETTINGSTATUSINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for POLICYSETTINGSTATUSINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("POLICYSETTINGSTATUSINFO").field("szKey", &self.szKey).field("szEventSource", &self.szEventSource).field("szEventLogName", &self.szEventLogName).field("dwEventID", &self.dwEventID).field("dwErrorCode", &self.dwErrorCode).field("status", &self.status).field("timeLogged", &self.timeLogged).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for POLICYSETTINGSTATUSINFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for POLICYSETTINGSTATUSINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<POLICYSETTINGSTATUSINFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for POLICYSETTINGSTATUSINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for POLICYSETTINGSTATUSINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const PT_MANDATORY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const PT_ROAMING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const PT_ROAMING_PREEXISTING: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const PT_TEMPORARY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
#[inline]
pub unsafe fn ProcessGroupPolicyCompleted(extensionid: *const ::windows::core::GUID, pasynchandle: usize, dwstatus: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ProcessGroupPolicyCompleted(extensionid: *const ::windows::core::GUID, pasynchandle: usize, dwstatus: u32) -> u32;
        }
        ::core::mem::transmute(ProcessGroupPolicyCompleted(::core::mem::transmute(extensionid), ::core::mem::transmute(pasynchandle), ::core::mem::transmute(dwstatus)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
#[inline]
pub unsafe fn ProcessGroupPolicyCompletedEx(extensionid: *const ::windows::core::GUID, pasynchandle: usize, dwstatus: u32, rsopstatus: ::windows::core::HRESULT) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ProcessGroupPolicyCompletedEx(extensionid: *const ::windows::core::GUID, pasynchandle: usize, dwstatus: u32, rsopstatus: ::windows::core::HRESULT) -> u32;
        }
        ::core::mem::transmute(ProcessGroupPolicyCompletedEx(::core::mem::transmute(extensionid), ::core::mem::transmute(pasynchandle), ::core::mem::transmute(dwstatus), ::core::mem::transmute(rsopstatus)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const RP_FORCE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const RP_SYNC: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const RSOP_COMPUTER_ACCESS_DENIED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const RSOP_INFO_FLAG_DIAGNOSTIC_MODE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const RSOP_NO_COMPUTER: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const RSOP_NO_USER: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const RSOP_PLANNING_ASSUME_COMP_WQLFILTER_TRUE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const RSOP_PLANNING_ASSUME_LOOPBACK_MERGE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const RSOP_PLANNING_ASSUME_LOOPBACK_REPLACE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const RSOP_PLANNING_ASSUME_SLOW_LINK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const RSOP_PLANNING_ASSUME_USER_WQLFILTER_TRUE: u32 = 8u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Wmi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Wmi"))]
pub struct RSOP_TARGET {
    pub pwszAccountName: ::windows::core::PWSTR,
    pub pwszNewSOM: ::windows::core::PWSTR,
    pub psaSecurityGroups: *mut super::Com::SAFEARRAY,
    pub pRsopToken: *mut ::core::ffi::c_void,
    pub pGPOList: *mut GROUP_POLICY_OBJECTA,
    pub pWbemServices: ::core::option::Option<super::Wmi::IWbemServices>,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Wmi"))]
impl ::core::clone::Clone for RSOP_TARGET {
    fn clone(&self) -> Self {
        Self {
            pwszAccountName: self.pwszAccountName,
            pwszNewSOM: self.pwszNewSOM,
            psaSecurityGroups: self.psaSecurityGroups,
            pRsopToken: self.pRsopToken,
            pGPOList: self.pGPOList,
            pWbemServices: self.pWbemServices.clone(),
        }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Wmi"))]
impl ::core::fmt::Debug for RSOP_TARGET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RSOP_TARGET").field("pwszAccountName", &self.pwszAccountName).field("pwszNewSOM", &self.pwszNewSOM).field("psaSecurityGroups", &self.psaSecurityGroups).field("pRsopToken", &self.pRsopToken).field("pGPOList", &self.pGPOList).field("pWbemServices", &self.pWbemServices).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Wmi"))]
unsafe impl ::windows::core::Abi for RSOP_TARGET {
    type Abi = ::core::mem::ManuallyDrop<Self>;
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
impl ::core::default::Default for RSOP_TARGET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const RSOP_TEMPNAMESPACE_EXISTS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const RSOP_USER_ACCESS_DENIED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RefreshPolicy<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(bmachine: Param0) -> super::super::Foundation::BOOL {
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
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RefreshPolicyEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(bmachine: Param0, dwoptions: u32) -> super::super::Foundation::BOOL {
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
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegisterGPNotification<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(hevent: Param0, bmachine: Param1) -> super::super::Foundation::BOOL {
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
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn RsopAccessCheckByType<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSID>>(psecuritydescriptor: *const super::super::Security::SECURITY_DESCRIPTOR, pprincipalselfsid: Param1, prsoptoken: *const ::core::ffi::c_void, dwdesiredaccessmask: u32, pobjecttypelist: &[super::super::Security::OBJECT_TYPE_LIST], pgenericmapping: *const super::super::Security::GENERIC_MAPPING, pprivilegeset: *const super::super::Security::PRIVILEGE_SET, pdwprivilegesetlength: *const u32, pdwgrantedaccessmask: *mut u32, pbaccessstatus: *mut i32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RsopAccessCheckByType(psecuritydescriptor: *const super::super::Security::SECURITY_DESCRIPTOR, pprincipalselfsid: super::super::Foundation::PSID, prsoptoken: *const ::core::ffi::c_void, dwdesiredaccessmask: u32, pobjecttypelist: *const super::super::Security::OBJECT_TYPE_LIST, objecttypelistlength: u32, pgenericmapping: *const super::super::Security::GENERIC_MAPPING, pprivilegeset: *const super::super::Security::PRIVILEGE_SET, pdwprivilegesetlength: *const u32, pdwgrantedaccessmask: *mut u32, pbaccessstatus: *mut i32) -> ::windows::core::HRESULT;
        }
        RsopAccessCheckByType(::core::mem::transmute(psecuritydescriptor), pprincipalselfsid.into_param().abi(), ::core::mem::transmute(prsoptoken), ::core::mem::transmute(dwdesiredaccessmask), ::core::mem::transmute(::windows::core::as_ptr_or_null(pobjecttypelist)), pobjecttypelist.len() as _, ::core::mem::transmute(pgenericmapping), ::core::mem::transmute(pprivilegeset), ::core::mem::transmute(pdwprivilegesetlength), ::core::mem::transmute(pdwgrantedaccessmask), ::core::mem::transmute(pbaccessstatus)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
#[inline]
pub unsafe fn RsopFileAccessCheck<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(pszfilename: Param0, prsoptoken: *const ::core::ffi::c_void, dwdesiredaccessmask: u32, pdwgrantedaccessmask: *mut u32, pbaccessstatus: *mut i32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RsopFileAccessCheck(pszfilename: ::windows::core::PCWSTR, prsoptoken: *const ::core::ffi::c_void, dwdesiredaccessmask: u32, pdwgrantedaccessmask: *mut u32, pbaccessstatus: *mut i32) -> ::windows::core::HRESULT;
        }
        RsopFileAccessCheck(pszfilename.into_param().abi(), ::core::mem::transmute(prsoptoken), ::core::mem::transmute(dwdesiredaccessmask), ::core::mem::transmute(pdwgrantedaccessmask), ::core::mem::transmute(pbaccessstatus)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Wmi\"`*"]
#[cfg(feature = "Win32_System_Wmi")]
#[inline]
pub unsafe fn RsopResetPolicySettingStatus<'a, Param1: ::windows::core::IntoParam<'a, super::Wmi::IWbemServices>, Param2: ::windows::core::IntoParam<'a, super::Wmi::IWbemClassObject>>(dwflags: u32, pservices: Param1, psettinginstance: Param2) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RsopResetPolicySettingStatus(dwflags: u32, pservices: ::windows::core::RawPtr, psettinginstance: ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        RsopResetPolicySettingStatus(::core::mem::transmute(dwflags), pservices.into_param().abi(), psettinginstance.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Wmi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Wmi"))]
#[inline]
pub unsafe fn RsopSetPolicySettingStatus<'a, Param1: ::windows::core::IntoParam<'a, super::Wmi::IWbemServices>, Param2: ::windows::core::IntoParam<'a, super::Wmi::IWbemClassObject>>(dwflags: u32, pservices: Param1, psettinginstance: Param2, pstatus: &[POLICYSETTINGSTATUSINFO]) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RsopSetPolicySettingStatus(dwflags: u32, pservices: ::windows::core::RawPtr, psettinginstance: ::windows::core::RawPtr, ninfo: u32, pstatus: *const POLICYSETTINGSTATUSINFO) -> ::windows::core::HRESULT;
        }
        RsopSetPolicySettingStatus(::core::mem::transmute(dwflags), pservices.into_param().abi(), psettinginstance.into_param().abi(), pstatus.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(pstatus))).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SETTINGSTATUS(pub i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const RSOPUnspecified: SETTINGSTATUS = SETTINGSTATUS(0i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const RSOPApplied: SETTINGSTATUS = SETTINGSTATUS(1i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const RSOPIgnored: SETTINGSTATUS = SETTINGSTATUS(2i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const RSOPFailed: SETTINGSTATUS = SETTINGSTATUS(3i32);
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const RSOPSubsettingFailed: SETTINGSTATUS = SETTINGSTATUS(4i32);
impl ::core::marker::Copy for SETTINGSTATUS {}
impl ::core::clone::Clone for SETTINGSTATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SETTINGSTATUS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SETTINGSTATUS {
    type Abi = Self;
}
impl ::core::fmt::Debug for SETTINGSTATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SETTINGSTATUS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
#[inline]
pub unsafe fn UninstallApplication<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(productcode: Param0, dwstatus: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UninstallApplication(productcode: ::windows::core::PCWSTR, dwstatus: u32) -> u32;
        }
        ::core::mem::transmute(UninstallApplication(productcode.into_param().abi(), ::core::mem::transmute(dwstatus)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UnregisterGPNotification<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hevent: Param0) -> super::super::Foundation::BOOL {
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
#[cfg(feature = "implement")]
::core::include!("impl.rs");
