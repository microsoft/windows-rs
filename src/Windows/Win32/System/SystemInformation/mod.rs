#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub struct CACHE_DESCRIPTOR {
    pub Level: u8,
    pub Associativity: u8,
    pub LineSize: u16,
    pub Size: u32,
    pub Type: PROCESSOR_CACHE_TYPE,
}
impl CACHE_DESCRIPTOR {}
impl ::std::default::Default for CACHE_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CACHE_DESCRIPTOR {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CACHE_DESCRIPTOR").field("Level", &self.Level).field("Associativity", &self.Associativity).field("LineSize", &self.LineSize).field("Size", &self.Size).field("Type", &self.Type).finish()
    }
}
impl ::std::cmp::PartialEq for CACHE_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.Level == other.Level && self.Associativity == other.Associativity && self.LineSize == other.LineSize && self.Size == other.Size && self.Type == other.Type
    }
}
impl ::std::cmp::Eq for CACHE_DESCRIPTOR {}
unsafe impl ::windows::runtime::Abi for CACHE_DESCRIPTOR {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub struct CACHE_RELATIONSHIP {
    pub Level: u8,
    pub Associativity: u8,
    pub LineSize: u16,
    pub CacheSize: u32,
    pub Type: PROCESSOR_CACHE_TYPE,
    pub Reserved: [u8; 18],
    pub GroupCount: u16,
    pub Anonymous: CACHE_RELATIONSHIP_0,
}
impl CACHE_RELATIONSHIP {}
impl ::std::default::Default for CACHE_RELATIONSHIP {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for CACHE_RELATIONSHIP {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for CACHE_RELATIONSHIP {}
unsafe impl ::windows::runtime::Abi for CACHE_RELATIONSHIP {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub union CACHE_RELATIONSHIP_0 {
    pub GroupMask: GROUP_AFFINITY,
    pub GroupMasks: [GROUP_AFFINITY; 1],
}
impl CACHE_RELATIONSHIP_0 {}
impl ::std::default::Default for CACHE_RELATIONSHIP_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for CACHE_RELATIONSHIP_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for CACHE_RELATIONSHIP_0 {}
unsafe impl ::windows::runtime::Abi for CACHE_RELATIONSHIP_0 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct COMPUTER_NAME_FORMAT(pub i32);
pub const ComputerNameNetBIOS: COMPUTER_NAME_FORMAT = COMPUTER_NAME_FORMAT(0i32);
pub const ComputerNameDnsHostname: COMPUTER_NAME_FORMAT = COMPUTER_NAME_FORMAT(1i32);
pub const ComputerNameDnsDomain: COMPUTER_NAME_FORMAT = COMPUTER_NAME_FORMAT(2i32);
pub const ComputerNameDnsFullyQualified: COMPUTER_NAME_FORMAT = COMPUTER_NAME_FORMAT(3i32);
pub const ComputerNamePhysicalNetBIOS: COMPUTER_NAME_FORMAT = COMPUTER_NAME_FORMAT(4i32);
pub const ComputerNamePhysicalDnsHostname: COMPUTER_NAME_FORMAT = COMPUTER_NAME_FORMAT(5i32);
pub const ComputerNamePhysicalDnsDomain: COMPUTER_NAME_FORMAT = COMPUTER_NAME_FORMAT(6i32);
pub const ComputerNamePhysicalDnsFullyQualified: COMPUTER_NAME_FORMAT = COMPUTER_NAME_FORMAT(7i32);
pub const ComputerNameMax: COMPUTER_NAME_FORMAT = COMPUTER_NAME_FORMAT(8i32);
impl ::std::convert::From<i32> for COMPUTER_NAME_FORMAT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for COMPUTER_NAME_FORMAT {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CPU_SET_INFORMATION_TYPE(pub i32);
pub const CpuSetInformation: CPU_SET_INFORMATION_TYPE = CPU_SET_INFORMATION_TYPE(0i32);
impl ::std::convert::From<i32> for CPU_SET_INFORMATION_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CPU_SET_INFORMATION_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DEP_SYSTEM_POLICY_TYPE(pub i32);
pub const DEPPolicyAlwaysOff: DEP_SYSTEM_POLICY_TYPE = DEP_SYSTEM_POLICY_TYPE(0i32);
pub const DEPPolicyAlwaysOn: DEP_SYSTEM_POLICY_TYPE = DEP_SYSTEM_POLICY_TYPE(1i32);
pub const DEPPolicyOptIn: DEP_SYSTEM_POLICY_TYPE = DEP_SYSTEM_POLICY_TYPE(2i32);
pub const DEPPolicyOptOut: DEP_SYSTEM_POLICY_TYPE = DEP_SYSTEM_POLICY_TYPE(3i32);
pub const DEPTotalPolicyCount: DEP_SYSTEM_POLICY_TYPE = DEP_SYSTEM_POLICY_TYPE(4i32);
impl ::std::convert::From<i32> for DEP_SYSTEM_POLICY_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DEP_SYSTEM_POLICY_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DnsHostnameToComputerNameExW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(hostname: Param0, computername: super::super::Foundation::PWSTR, nsize: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DnsHostnameToComputerNameExW(hostname: super::super::Foundation::PWSTR, computername: super::super::Foundation::PWSTR, nsize: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(DnsHostnameToComputerNameExW(hostname.into_param().abi(), ::std::mem::transmute(computername), ::std::mem::transmute(nsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
#[inline]
pub unsafe fn EnumSystemFirmwareTables(firmwaretableprovidersignature: FIRMWARE_TABLE_PROVIDER, pfirmwaretableenumbuffer: *mut FIRMWARE_TABLE_ID, buffersize: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnumSystemFirmwareTables(firmwaretableprovidersignature: FIRMWARE_TABLE_PROVIDER, pfirmwaretableenumbuffer: *mut FIRMWARE_TABLE_ID, buffersize: u32) -> u32;
        }
        ::std::mem::transmute(EnumSystemFirmwareTables(::std::mem::transmute(firmwaretableprovidersignature), ::std::mem::transmute(pfirmwaretableenumbuffer), ::std::mem::transmute(buffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq)]
#[repr(transparent)]
pub struct FIRMWARE_TABLE_ID(pub u32);
impl ::std::default::Default for FIRMWARE_TABLE_ID {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for FIRMWARE_TABLE_ID {}
unsafe impl ::windows::runtime::Abi for FIRMWARE_TABLE_ID {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FIRMWARE_TABLE_PROVIDER(pub u32);
pub const ACPI: FIRMWARE_TABLE_PROVIDER = FIRMWARE_TABLE_PROVIDER(1094930505u32);
pub const FIRM: FIRMWARE_TABLE_PROVIDER = FIRMWARE_TABLE_PROVIDER(1179210317u32);
pub const RSMB: FIRMWARE_TABLE_PROVIDER = FIRMWARE_TABLE_PROVIDER(1381190978u32);
impl ::std::convert::From<u32> for FIRMWARE_TABLE_PROVIDER {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FIRMWARE_TABLE_PROVIDER {
    type Abi = Self;
}
impl ::std::ops::BitOr for FIRMWARE_TABLE_PROVIDER {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for FIRMWARE_TABLE_PROVIDER {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for FIRMWARE_TABLE_PROVIDER {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for FIRMWARE_TABLE_PROVIDER {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for FIRMWARE_TABLE_PROVIDER {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FIRMWARE_TYPE(pub i32);
pub const FirmwareTypeUnknown: FIRMWARE_TYPE = FIRMWARE_TYPE(0i32);
pub const FirmwareTypeBios: FIRMWARE_TYPE = FIRMWARE_TYPE(1i32);
pub const FirmwareTypeUefi: FIRMWARE_TYPE = FIRMWARE_TYPE(2i32);
pub const FirmwareTypeMax: FIRMWARE_TYPE = FIRMWARE_TYPE(3i32);
impl ::std::convert::From<i32> for FIRMWARE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FIRMWARE_TYPE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub struct GROUP_AFFINITY {
    pub Mask: usize,
    pub Group: u16,
    pub Reserved: [u16; 3],
}
impl GROUP_AFFINITY {}
impl ::std::default::Default for GROUP_AFFINITY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for GROUP_AFFINITY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("GROUP_AFFINITY").field("Mask", &self.Mask).field("Group", &self.Group).field("Reserved", &self.Reserved).finish()
    }
}
impl ::std::cmp::PartialEq for GROUP_AFFINITY {
    fn eq(&self, other: &Self) -> bool {
        self.Mask == other.Mask && self.Group == other.Group && self.Reserved == other.Reserved
    }
}
impl ::std::cmp::Eq for GROUP_AFFINITY {}
unsafe impl ::windows::runtime::Abi for GROUP_AFFINITY {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub struct GROUP_RELATIONSHIP {
    pub MaximumGroupCount: u16,
    pub ActiveGroupCount: u16,
    pub Reserved: [u8; 20],
    pub GroupInfo: [PROCESSOR_GROUP_INFO; 1],
}
impl GROUP_RELATIONSHIP {}
impl ::std::default::Default for GROUP_RELATIONSHIP {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for GROUP_RELATIONSHIP {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("GROUP_RELATIONSHIP").field("MaximumGroupCount", &self.MaximumGroupCount).field("ActiveGroupCount", &self.ActiveGroupCount).field("Reserved", &self.Reserved).field("GroupInfo", &self.GroupInfo).finish()
    }
}
impl ::std::cmp::PartialEq for GROUP_RELATIONSHIP {
    fn eq(&self, other: &Self) -> bool {
        self.MaximumGroupCount == other.MaximumGroupCount && self.ActiveGroupCount == other.ActiveGroupCount && self.Reserved == other.Reserved && self.GroupInfo == other.GroupInfo
    }
}
impl ::std::cmp::Eq for GROUP_RELATIONSHIP {}
unsafe impl ::windows::runtime::Abi for GROUP_RELATIONSHIP {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetComputerNameExA(nametype: COMPUTER_NAME_FORMAT, lpbuffer: super::super::Foundation::PSTR, nsize: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetComputerNameExA(nametype: COMPUTER_NAME_FORMAT, lpbuffer: super::super::Foundation::PSTR, nsize: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetComputerNameExA(::std::mem::transmute(nametype), ::std::mem::transmute(lpbuffer), ::std::mem::transmute(nsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetComputerNameExW(nametype: COMPUTER_NAME_FORMAT, lpbuffer: super::super::Foundation::PWSTR, nsize: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetComputerNameExW(nametype: COMPUTER_NAME_FORMAT, lpbuffer: super::super::Foundation::PWSTR, nsize: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetComputerNameExW(::std::mem::transmute(nametype), ::std::mem::transmute(lpbuffer), ::std::mem::transmute(nsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetFirmwareType(firmwaretype: *mut FIRMWARE_TYPE) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetFirmwareType(firmwaretype: *mut FIRMWARE_TYPE) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetFirmwareType(::std::mem::transmute(firmwaretype)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
#[inline]
pub unsafe fn GetIntegratedDisplaySize() -> ::windows::runtime::Result<f64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetIntegratedDisplaySize(sizeininches: *mut f64) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        GetIntegratedDisplaySize(&mut result__).from_abi::<f64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetLocalTime(lpsystemtime: *mut super::super::Foundation::SYSTEMTIME) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetLocalTime(lpsystemtime: *mut super::super::Foundation::SYSTEMTIME);
        }
        ::std::mem::transmute(GetLocalTime(::std::mem::transmute(lpsystemtime)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetLogicalProcessorInformation(buffer: *mut SYSTEM_LOGICAL_PROCESSOR_INFORMATION, returnedlength: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetLogicalProcessorInformation(buffer: *mut SYSTEM_LOGICAL_PROCESSOR_INFORMATION, returnedlength: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetLogicalProcessorInformation(::std::mem::transmute(buffer), ::std::mem::transmute(returnedlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetLogicalProcessorInformationEx(relationshiptype: LOGICAL_PROCESSOR_RELATIONSHIP, buffer: *mut SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX, returnedlength: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetLogicalProcessorInformationEx(relationshiptype: LOGICAL_PROCESSOR_RELATIONSHIP, buffer: *mut SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX, returnedlength: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetLogicalProcessorInformationEx(::std::mem::transmute(relationshiptype), ::std::mem::transmute(buffer), ::std::mem::transmute(returnedlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_System_Diagnostics_Debug`*"]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
#[inline]
pub unsafe fn GetNativeSystemInfo(lpsysteminfo: *mut SYSTEM_INFO) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetNativeSystemInfo(lpsysteminfo: *mut SYSTEM_INFO);
        }
        ::std::mem::transmute(GetNativeSystemInfo(::std::mem::transmute(lpsysteminfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetOsManufacturingMode(pbenabled: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetOsManufacturingMode(pbenabled: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetOsManufacturingMode(::std::mem::transmute(pbenabled)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetOsSafeBootMode(flags: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetOsSafeBootMode(flags: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetOsSafeBootMode(::std::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetPhysicallyInstalledSystemMemory(totalmemoryinkilobytes: *mut u64) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPhysicallyInstalledSystemMemory(totalmemoryinkilobytes: *mut u64) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetPhysicallyInstalledSystemMemory(::std::mem::transmute(totalmemoryinkilobytes)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetProcessorSystemCycleTime(group: u16, buffer: *mut SYSTEM_PROCESSOR_CYCLE_TIME_INFORMATION, returnedlength: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetProcessorSystemCycleTime(group: u16, buffer: *mut SYSTEM_PROCESSOR_CYCLE_TIME_INFORMATION, returnedlength: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetProcessorSystemCycleTime(::std::mem::transmute(group), ::std::mem::transmute(buffer), ::std::mem::transmute(returnedlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetProductInfo(dwosmajorversion: u32, dwosminorversion: u32, dwspmajorversion: u32, dwspminorversion: u32, pdwreturnedproducttype: *mut OS_PRODUCT_TYPE) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetProductInfo(dwosmajorversion: u32, dwosminorversion: u32, dwspmajorversion: u32, dwspminorversion: u32, pdwreturnedproducttype: *mut OS_PRODUCT_TYPE) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetProductInfo(::std::mem::transmute(dwosmajorversion), ::std::mem::transmute(dwosminorversion), ::std::mem::transmute(dwspmajorversion), ::std::mem::transmute(dwspminorversion), ::std::mem::transmute(pdwreturnedproducttype)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
#[inline]
pub unsafe fn GetSystemDEPPolicy() -> DEP_SYSTEM_POLICY_TYPE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetSystemDEPPolicy() -> DEP_SYSTEM_POLICY_TYPE;
        }
        ::std::mem::transmute(GetSystemDEPPolicy())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetSystemDirectoryA(lpbuffer: super::super::Foundation::PSTR, usize: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetSystemDirectoryA(lpbuffer: super::super::Foundation::PSTR, usize: u32) -> u32;
        }
        ::std::mem::transmute(GetSystemDirectoryA(::std::mem::transmute(lpbuffer), ::std::mem::transmute(usize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetSystemDirectoryW(lpbuffer: super::super::Foundation::PWSTR, usize: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetSystemDirectoryW(lpbuffer: super::super::Foundation::PWSTR, usize: u32) -> u32;
        }
        ::std::mem::transmute(GetSystemDirectoryW(::std::mem::transmute(lpbuffer), ::std::mem::transmute(usize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
#[inline]
pub unsafe fn GetSystemFirmwareTable<'a, Param1: ::windows::runtime::IntoParam<'a, FIRMWARE_TABLE_ID>>(firmwaretableprovidersignature: FIRMWARE_TABLE_PROVIDER, firmwaretableid: Param1, pfirmwaretablebuffer: *mut ::std::ffi::c_void, buffersize: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetSystemFirmwareTable(firmwaretableprovidersignature: FIRMWARE_TABLE_PROVIDER, firmwaretableid: FIRMWARE_TABLE_ID, pfirmwaretablebuffer: *mut ::std::ffi::c_void, buffersize: u32) -> u32;
        }
        ::std::mem::transmute(GetSystemFirmwareTable(::std::mem::transmute(firmwaretableprovidersignature), firmwaretableid.into_param().abi(), ::std::mem::transmute(pfirmwaretablebuffer), ::std::mem::transmute(buffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_System_Diagnostics_Debug`*"]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
#[inline]
pub unsafe fn GetSystemInfo(lpsysteminfo: *mut SYSTEM_INFO) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetSystemInfo(lpsysteminfo: *mut SYSTEM_INFO);
        }
        ::std::mem::transmute(GetSystemInfo(::std::mem::transmute(lpsysteminfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetSystemLeapSecondInformation(enabled: *mut super::super::Foundation::BOOL, flags: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetSystemLeapSecondInformation(enabled: *mut super::super::Foundation::BOOL, flags: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetSystemLeapSecondInformation(::std::mem::transmute(enabled), ::std::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetSystemTime(lpsystemtime: *mut super::super::Foundation::SYSTEMTIME) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetSystemTime(lpsystemtime: *mut super::super::Foundation::SYSTEMTIME);
        }
        ::std::mem::transmute(GetSystemTime(::std::mem::transmute(lpsystemtime)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetSystemTimeAdjustment(lptimeadjustment: *mut u32, lptimeincrement: *mut u32, lptimeadjustmentdisabled: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetSystemTimeAdjustment(lptimeadjustment: *mut u32, lptimeincrement: *mut u32, lptimeadjustmentdisabled: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetSystemTimeAdjustment(::std::mem::transmute(lptimeadjustment), ::std::mem::transmute(lptimeincrement), ::std::mem::transmute(lptimeadjustmentdisabled)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetSystemTimeAdjustmentPrecise(lptimeadjustment: *mut u64, lptimeincrement: *mut u64, lptimeadjustmentdisabled: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetSystemTimeAdjustmentPrecise(lptimeadjustment: *mut u64, lptimeincrement: *mut u64, lptimeadjustmentdisabled: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetSystemTimeAdjustmentPrecise(::std::mem::transmute(lptimeadjustment), ::std::mem::transmute(lptimeincrement), ::std::mem::transmute(lptimeadjustmentdisabled)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetSystemTimeAsFileTime(lpsystemtimeasfiletime: *mut super::super::Foundation::FILETIME) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetSystemTimeAsFileTime(lpsystemtimeasfiletime: *mut super::super::Foundation::FILETIME);
        }
        ::std::mem::transmute(GetSystemTimeAsFileTime(::std::mem::transmute(lpsystemtimeasfiletime)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetSystemTimePreciseAsFileTime(lpsystemtimeasfiletime: *mut super::super::Foundation::FILETIME) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetSystemTimePreciseAsFileTime(lpsystemtimeasfiletime: *mut super::super::Foundation::FILETIME);
        }
        ::std::mem::transmute(GetSystemTimePreciseAsFileTime(::std::mem::transmute(lpsystemtimeasfiletime)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetSystemWindowsDirectoryA(lpbuffer: super::super::Foundation::PSTR, usize: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetSystemWindowsDirectoryA(lpbuffer: super::super::Foundation::PSTR, usize: u32) -> u32;
        }
        ::std::mem::transmute(GetSystemWindowsDirectoryA(::std::mem::transmute(lpbuffer), ::std::mem::transmute(usize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetSystemWindowsDirectoryW(lpbuffer: super::super::Foundation::PWSTR, usize: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetSystemWindowsDirectoryW(lpbuffer: super::super::Foundation::PWSTR, usize: u32) -> u32;
        }
        ::std::mem::transmute(GetSystemWindowsDirectoryW(::std::mem::transmute(lpbuffer), ::std::mem::transmute(usize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetSystemWow64Directory2A(lpbuffer: super::super::Foundation::PSTR, usize: u32, imagefilemachinetype: u16) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetSystemWow64Directory2A(lpbuffer: super::super::Foundation::PSTR, usize: u32, imagefilemachinetype: u16) -> u32;
        }
        ::std::mem::transmute(GetSystemWow64Directory2A(::std::mem::transmute(lpbuffer), ::std::mem::transmute(usize), ::std::mem::transmute(imagefilemachinetype)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetSystemWow64Directory2W(lpbuffer: super::super::Foundation::PWSTR, usize: u32, imagefilemachinetype: u16) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetSystemWow64Directory2W(lpbuffer: super::super::Foundation::PWSTR, usize: u32, imagefilemachinetype: u16) -> u32;
        }
        ::std::mem::transmute(GetSystemWow64Directory2W(::std::mem::transmute(lpbuffer), ::std::mem::transmute(usize), ::std::mem::transmute(imagefilemachinetype)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetSystemWow64DirectoryA(lpbuffer: super::super::Foundation::PSTR, usize: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetSystemWow64DirectoryA(lpbuffer: super::super::Foundation::PSTR, usize: u32) -> u32;
        }
        ::std::mem::transmute(GetSystemWow64DirectoryA(::std::mem::transmute(lpbuffer), ::std::mem::transmute(usize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetSystemWow64DirectoryW(lpbuffer: super::super::Foundation::PWSTR, usize: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetSystemWow64DirectoryW(lpbuffer: super::super::Foundation::PWSTR, usize: u32) -> u32;
        }
        ::std::mem::transmute(GetSystemWow64DirectoryW(::std::mem::transmute(lpbuffer), ::std::mem::transmute(usize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
#[inline]
pub unsafe fn GetTickCount() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetTickCount() -> u32;
        }
        ::std::mem::transmute(GetTickCount())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
#[inline]
pub unsafe fn GetTickCount64() -> u64 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetTickCount64() -> u64;
        }
        ::std::mem::transmute(GetTickCount64())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
#[inline]
pub unsafe fn GetVersion() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetVersion() -> u32;
        }
        ::std::mem::transmute(GetVersion())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetVersionExA(lpversioninformation: *mut OSVERSIONINFOA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetVersionExA(lpversioninformation: *mut OSVERSIONINFOA) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetVersionExA(::std::mem::transmute(lpversioninformation)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetVersionExW(lpversioninformation: *mut OSVERSIONINFOW) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetVersionExW(lpversioninformation: *mut OSVERSIONINFOW) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetVersionExW(::std::mem::transmute(lpversioninformation)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetWindowsDirectoryA(lpbuffer: super::super::Foundation::PSTR, usize: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetWindowsDirectoryA(lpbuffer: super::super::Foundation::PSTR, usize: u32) -> u32;
        }
        ::std::mem::transmute(GetWindowsDirectoryA(::std::mem::transmute(lpbuffer), ::std::mem::transmute(usize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetWindowsDirectoryW(lpbuffer: super::super::Foundation::PWSTR, usize: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetWindowsDirectoryW(lpbuffer: super::super::Foundation::PWSTR, usize: u32) -> u32;
        }
        ::std::mem::transmute(GetWindowsDirectoryW(::std::mem::transmute(lpbuffer), ::std::mem::transmute(usize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
#[inline]
pub unsafe fn GlobalMemoryStatus(lpbuffer: *mut MEMORYSTATUS) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GlobalMemoryStatus(lpbuffer: *mut MEMORYSTATUS);
        }
        ::std::mem::transmute(GlobalMemoryStatus(::std::mem::transmute(lpbuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GlobalMemoryStatusEx(lpbuffer: *mut MEMORYSTATUSEX) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GlobalMemoryStatusEx(lpbuffer: *mut MEMORYSTATUSEX) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GlobalMemoryStatusEx(::std::mem::transmute(lpbuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsUserCetAvailableInEnvironment(usercetenvironment: USER_CET_ENVIRONMENT) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsUserCetAvailableInEnvironment(usercetenvironment: USER_CET_ENVIRONMENT) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(IsUserCetAvailableInEnvironment(::std::mem::transmute(usercetenvironment)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsWow64GuestMachineSupported(wowguestmachine: u16) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsWow64GuestMachineSupported(wowguestmachine: u16, machineissupported: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        IsWow64GuestMachineSupported(::std::mem::transmute(wowguestmachine), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct LOGICAL_PROCESSOR_RELATIONSHIP(pub i32);
pub const RelationProcessorCore: LOGICAL_PROCESSOR_RELATIONSHIP = LOGICAL_PROCESSOR_RELATIONSHIP(0i32);
pub const RelationNumaNode: LOGICAL_PROCESSOR_RELATIONSHIP = LOGICAL_PROCESSOR_RELATIONSHIP(1i32);
pub const RelationCache: LOGICAL_PROCESSOR_RELATIONSHIP = LOGICAL_PROCESSOR_RELATIONSHIP(2i32);
pub const RelationProcessorPackage: LOGICAL_PROCESSOR_RELATIONSHIP = LOGICAL_PROCESSOR_RELATIONSHIP(3i32);
pub const RelationGroup: LOGICAL_PROCESSOR_RELATIONSHIP = LOGICAL_PROCESSOR_RELATIONSHIP(4i32);
pub const RelationProcessorDie: LOGICAL_PROCESSOR_RELATIONSHIP = LOGICAL_PROCESSOR_RELATIONSHIP(5i32);
pub const RelationNumaNodeEx: LOGICAL_PROCESSOR_RELATIONSHIP = LOGICAL_PROCESSOR_RELATIONSHIP(6i32);
pub const RelationProcessorModule: LOGICAL_PROCESSOR_RELATIONSHIP = LOGICAL_PROCESSOR_RELATIONSHIP(7i32);
pub const RelationAll: LOGICAL_PROCESSOR_RELATIONSHIP = LOGICAL_PROCESSOR_RELATIONSHIP(65535i32);
impl ::std::convert::From<i32> for LOGICAL_PROCESSOR_RELATIONSHIP {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for LOGICAL_PROCESSOR_RELATIONSHIP {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub struct MEMORYSTATUS {
    pub dwLength: u32,
    pub dwMemoryLoad: u32,
    pub dwTotalPhys: usize,
    pub dwAvailPhys: usize,
    pub dwTotalPageFile: usize,
    pub dwAvailPageFile: usize,
    pub dwTotalVirtual: usize,
    pub dwAvailVirtual: usize,
}
impl MEMORYSTATUS {}
impl ::std::default::Default for MEMORYSTATUS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for MEMORYSTATUS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MEMORYSTATUS")
            .field("dwLength", &self.dwLength)
            .field("dwMemoryLoad", &self.dwMemoryLoad)
            .field("dwTotalPhys", &self.dwTotalPhys)
            .field("dwAvailPhys", &self.dwAvailPhys)
            .field("dwTotalPageFile", &self.dwTotalPageFile)
            .field("dwAvailPageFile", &self.dwAvailPageFile)
            .field("dwTotalVirtual", &self.dwTotalVirtual)
            .field("dwAvailVirtual", &self.dwAvailVirtual)
            .finish()
    }
}
impl ::std::cmp::PartialEq for MEMORYSTATUS {
    fn eq(&self, other: &Self) -> bool {
        self.dwLength == other.dwLength && self.dwMemoryLoad == other.dwMemoryLoad && self.dwTotalPhys == other.dwTotalPhys && self.dwAvailPhys == other.dwAvailPhys && self.dwTotalPageFile == other.dwTotalPageFile && self.dwAvailPageFile == other.dwAvailPageFile && self.dwTotalVirtual == other.dwTotalVirtual && self.dwAvailVirtual == other.dwAvailVirtual
    }
}
impl ::std::cmp::Eq for MEMORYSTATUS {}
unsafe impl ::windows::runtime::Abi for MEMORYSTATUS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub struct MEMORYSTATUSEX {
    pub dwLength: u32,
    pub dwMemoryLoad: u32,
    pub ullTotalPhys: u64,
    pub ullAvailPhys: u64,
    pub ullTotalPageFile: u64,
    pub ullAvailPageFile: u64,
    pub ullTotalVirtual: u64,
    pub ullAvailVirtual: u64,
    pub ullAvailExtendedVirtual: u64,
}
impl MEMORYSTATUSEX {}
impl ::std::default::Default for MEMORYSTATUSEX {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for MEMORYSTATUSEX {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MEMORYSTATUSEX")
            .field("dwLength", &self.dwLength)
            .field("dwMemoryLoad", &self.dwMemoryLoad)
            .field("ullTotalPhys", &self.ullTotalPhys)
            .field("ullAvailPhys", &self.ullAvailPhys)
            .field("ullTotalPageFile", &self.ullTotalPageFile)
            .field("ullAvailPageFile", &self.ullAvailPageFile)
            .field("ullTotalVirtual", &self.ullTotalVirtual)
            .field("ullAvailVirtual", &self.ullAvailVirtual)
            .field("ullAvailExtendedVirtual", &self.ullAvailExtendedVirtual)
            .finish()
    }
}
impl ::std::cmp::PartialEq for MEMORYSTATUSEX {
    fn eq(&self, other: &Self) -> bool {
        self.dwLength == other.dwLength && self.dwMemoryLoad == other.dwMemoryLoad && self.ullTotalPhys == other.ullTotalPhys && self.ullAvailPhys == other.ullAvailPhys && self.ullTotalPageFile == other.ullTotalPageFile && self.ullAvailPageFile == other.ullAvailPageFile && self.ullTotalVirtual == other.ullTotalVirtual && self.ullAvailVirtual == other.ullAvailVirtual && self.ullAvailExtendedVirtual == other.ullAvailExtendedVirtual
    }
}
impl ::std::cmp::Eq for MEMORYSTATUSEX {}
unsafe impl ::windows::runtime::Abi for MEMORYSTATUSEX {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const NTDDI_LONGHORN: u32 = 100663296u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const NTDDI_VERSION: u32 = 167772171u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const NTDDI_VISTA: u32 = 100663296u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const NTDDI_VISTASP1: u32 = 100663552u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const NTDDI_VISTASP2: u32 = 100663808u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const NTDDI_VISTASP3: u32 = 100664064u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const NTDDI_VISTASP4: u32 = 100664320u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const NTDDI_WIN10: u32 = 167772160u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const NTDDI_WIN10_19H1: u32 = 167772167u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const NTDDI_WIN10_CO: u32 = 167772171u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const NTDDI_WIN10_FE: u32 = 167772170u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const NTDDI_WIN10_MN: u32 = 167772169u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const NTDDI_WIN10_RS1: u32 = 167772162u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const NTDDI_WIN10_RS2: u32 = 167772163u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const NTDDI_WIN10_RS3: u32 = 167772164u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const NTDDI_WIN10_RS4: u32 = 167772165u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const NTDDI_WIN10_RS5: u32 = 167772166u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const NTDDI_WIN10_TH2: u32 = 167772161u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const NTDDI_WIN10_VB: u32 = 167772168u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const NTDDI_WIN2K: u32 = 83886080u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const NTDDI_WIN2KSP1: u32 = 83886336u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const NTDDI_WIN2KSP2: u32 = 83886592u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const NTDDI_WIN2KSP3: u32 = 83886848u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const NTDDI_WIN2KSP4: u32 = 83887104u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const NTDDI_WIN4: u32 = 67108864u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const NTDDI_WIN6: u32 = 100663296u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const NTDDI_WIN6SP1: u32 = 100663552u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const NTDDI_WIN6SP2: u32 = 100663808u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const NTDDI_WIN6SP3: u32 = 100664064u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const NTDDI_WIN6SP4: u32 = 100664320u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const NTDDI_WIN7: u32 = 100728832u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const NTDDI_WIN8: u32 = 100794368u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const NTDDI_WINBLUE: u32 = 100859904u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const NTDDI_WINTHRESHOLD: u32 = 167772160u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const NTDDI_WINXP: u32 = 83951616u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const NTDDI_WINXPSP1: u32 = 83951872u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const NTDDI_WINXPSP2: u32 = 83952128u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const NTDDI_WINXPSP3: u32 = 83952384u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const NTDDI_WINXPSP4: u32 = 83952640u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const NTDDI_WS03: u32 = 84017152u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const NTDDI_WS03SP1: u32 = 84017408u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const NTDDI_WS03SP2: u32 = 84017664u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const NTDDI_WS03SP3: u32 = 84017920u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const NTDDI_WS03SP4: u32 = 84018176u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const NTDDI_WS08: u32 = 100663552u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const NTDDI_WS08SP2: u32 = 100663808u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const NTDDI_WS08SP3: u32 = 100664064u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const NTDDI_WS08SP4: u32 = 100664320u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub struct NUMA_NODE_RELATIONSHIP {
    pub NodeNumber: u32,
    pub Reserved: [u8; 18],
    pub GroupCount: u16,
    pub Anonymous: NUMA_NODE_RELATIONSHIP_0,
}
impl NUMA_NODE_RELATIONSHIP {}
impl ::std::default::Default for NUMA_NODE_RELATIONSHIP {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for NUMA_NODE_RELATIONSHIP {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for NUMA_NODE_RELATIONSHIP {}
unsafe impl ::windows::runtime::Abi for NUMA_NODE_RELATIONSHIP {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub union NUMA_NODE_RELATIONSHIP_0 {
    pub GroupMask: GROUP_AFFINITY,
    pub GroupMasks: [GROUP_AFFINITY; 1],
}
impl NUMA_NODE_RELATIONSHIP_0 {}
impl ::std::default::Default for NUMA_NODE_RELATIONSHIP_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for NUMA_NODE_RELATIONSHIP_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for NUMA_NODE_RELATIONSHIP_0 {}
unsafe impl ::windows::runtime::Abi for NUMA_NODE_RELATIONSHIP_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
pub struct OSVERSIONINFOA {
    pub dwOSVersionInfoSize: u32,
    pub dwMajorVersion: u32,
    pub dwMinorVersion: u32,
    pub dwBuildNumber: u32,
    pub dwPlatformId: u32,
    pub szCSDVersion: [super::super::Foundation::CHAR; 128],
}
#[cfg(feature = "Win32_Foundation")]
impl OSVERSIONINFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for OSVERSIONINFOA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for OSVERSIONINFOA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("OSVERSIONINFOA")
            .field("dwOSVersionInfoSize", &self.dwOSVersionInfoSize)
            .field("dwMajorVersion", &self.dwMajorVersion)
            .field("dwMinorVersion", &self.dwMinorVersion)
            .field("dwBuildNumber", &self.dwBuildNumber)
            .field("dwPlatformId", &self.dwPlatformId)
            .field("szCSDVersion", &self.szCSDVersion)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for OSVERSIONINFOA {
    fn eq(&self, other: &Self) -> bool {
        self.dwOSVersionInfoSize == other.dwOSVersionInfoSize && self.dwMajorVersion == other.dwMajorVersion && self.dwMinorVersion == other.dwMinorVersion && self.dwBuildNumber == other.dwBuildNumber && self.dwPlatformId == other.dwPlatformId && self.szCSDVersion == other.szCSDVersion
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for OSVERSIONINFOA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for OSVERSIONINFOA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
pub struct OSVERSIONINFOEXA {
    pub dwOSVersionInfoSize: u32,
    pub dwMajorVersion: u32,
    pub dwMinorVersion: u32,
    pub dwBuildNumber: u32,
    pub dwPlatformId: u32,
    pub szCSDVersion: [super::super::Foundation::CHAR; 128],
    pub wServicePackMajor: u16,
    pub wServicePackMinor: u16,
    pub wSuiteMask: u16,
    pub wProductType: u8,
    pub wReserved: u8,
}
#[cfg(feature = "Win32_Foundation")]
impl OSVERSIONINFOEXA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for OSVERSIONINFOEXA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for OSVERSIONINFOEXA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("OSVERSIONINFOEXA")
            .field("dwOSVersionInfoSize", &self.dwOSVersionInfoSize)
            .field("dwMajorVersion", &self.dwMajorVersion)
            .field("dwMinorVersion", &self.dwMinorVersion)
            .field("dwBuildNumber", &self.dwBuildNumber)
            .field("dwPlatformId", &self.dwPlatformId)
            .field("szCSDVersion", &self.szCSDVersion)
            .field("wServicePackMajor", &self.wServicePackMajor)
            .field("wServicePackMinor", &self.wServicePackMinor)
            .field("wSuiteMask", &self.wSuiteMask)
            .field("wProductType", &self.wProductType)
            .field("wReserved", &self.wReserved)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for OSVERSIONINFOEXA {
    fn eq(&self, other: &Self) -> bool {
        self.dwOSVersionInfoSize == other.dwOSVersionInfoSize
            && self.dwMajorVersion == other.dwMajorVersion
            && self.dwMinorVersion == other.dwMinorVersion
            && self.dwBuildNumber == other.dwBuildNumber
            && self.dwPlatformId == other.dwPlatformId
            && self.szCSDVersion == other.szCSDVersion
            && self.wServicePackMajor == other.wServicePackMajor
            && self.wServicePackMinor == other.wServicePackMinor
            && self.wSuiteMask == other.wSuiteMask
            && self.wProductType == other.wProductType
            && self.wReserved == other.wReserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for OSVERSIONINFOEXA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for OSVERSIONINFOEXA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub struct OSVERSIONINFOEXW {
    pub dwOSVersionInfoSize: u32,
    pub dwMajorVersion: u32,
    pub dwMinorVersion: u32,
    pub dwBuildNumber: u32,
    pub dwPlatformId: u32,
    pub szCSDVersion: [u16; 128],
    pub wServicePackMajor: u16,
    pub wServicePackMinor: u16,
    pub wSuiteMask: u16,
    pub wProductType: u8,
    pub wReserved: u8,
}
impl OSVERSIONINFOEXW {}
impl ::std::default::Default for OSVERSIONINFOEXW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for OSVERSIONINFOEXW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("OSVERSIONINFOEXW")
            .field("dwOSVersionInfoSize", &self.dwOSVersionInfoSize)
            .field("dwMajorVersion", &self.dwMajorVersion)
            .field("dwMinorVersion", &self.dwMinorVersion)
            .field("dwBuildNumber", &self.dwBuildNumber)
            .field("dwPlatformId", &self.dwPlatformId)
            .field("szCSDVersion", &self.szCSDVersion)
            .field("wServicePackMajor", &self.wServicePackMajor)
            .field("wServicePackMinor", &self.wServicePackMinor)
            .field("wSuiteMask", &self.wSuiteMask)
            .field("wProductType", &self.wProductType)
            .field("wReserved", &self.wReserved)
            .finish()
    }
}
impl ::std::cmp::PartialEq for OSVERSIONINFOEXW {
    fn eq(&self, other: &Self) -> bool {
        self.dwOSVersionInfoSize == other.dwOSVersionInfoSize
            && self.dwMajorVersion == other.dwMajorVersion
            && self.dwMinorVersion == other.dwMinorVersion
            && self.dwBuildNumber == other.dwBuildNumber
            && self.dwPlatformId == other.dwPlatformId
            && self.szCSDVersion == other.szCSDVersion
            && self.wServicePackMajor == other.wServicePackMajor
            && self.wServicePackMinor == other.wServicePackMinor
            && self.wSuiteMask == other.wSuiteMask
            && self.wProductType == other.wProductType
            && self.wReserved == other.wReserved
    }
}
impl ::std::cmp::Eq for OSVERSIONINFOEXW {}
unsafe impl ::windows::runtime::Abi for OSVERSIONINFOEXW {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub struct OSVERSIONINFOW {
    pub dwOSVersionInfoSize: u32,
    pub dwMajorVersion: u32,
    pub dwMinorVersion: u32,
    pub dwBuildNumber: u32,
    pub dwPlatformId: u32,
    pub szCSDVersion: [u16; 128],
}
impl OSVERSIONINFOW {}
impl ::std::default::Default for OSVERSIONINFOW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for OSVERSIONINFOW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("OSVERSIONINFOW")
            .field("dwOSVersionInfoSize", &self.dwOSVersionInfoSize)
            .field("dwMajorVersion", &self.dwMajorVersion)
            .field("dwMinorVersion", &self.dwMinorVersion)
            .field("dwBuildNumber", &self.dwBuildNumber)
            .field("dwPlatformId", &self.dwPlatformId)
            .field("szCSDVersion", &self.szCSDVersion)
            .finish()
    }
}
impl ::std::cmp::PartialEq for OSVERSIONINFOW {
    fn eq(&self, other: &Self) -> bool {
        self.dwOSVersionInfoSize == other.dwOSVersionInfoSize && self.dwMajorVersion == other.dwMajorVersion && self.dwMinorVersion == other.dwMinorVersion && self.dwBuildNumber == other.dwBuildNumber && self.dwPlatformId == other.dwPlatformId && self.szCSDVersion == other.szCSDVersion
    }
}
impl ::std::cmp::Eq for OSVERSIONINFOW {}
unsafe impl ::windows::runtime::Abi for OSVERSIONINFOW {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const OSVERSION_MASK: u32 = 4294901760u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct OS_PRODUCT_TYPE(pub u32);
pub const PRODUCT_BUSINESS: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(6u32);
pub const PRODUCT_BUSINESS_N: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(16u32);
pub const PRODUCT_CLUSTER_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(18u32);
pub const PRODUCT_CLUSTER_SERVER_V: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(64u32);
pub const PRODUCT_CORE: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(101u32);
pub const PRODUCT_CORE_COUNTRYSPECIFIC: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(99u32);
pub const PRODUCT_CORE_N: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(98u32);
pub const PRODUCT_CORE_SINGLELANGUAGE: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(100u32);
pub const PRODUCT_DATACENTER_EVALUATION_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(80u32);
pub const PRODUCT_DATACENTER_A_SERVER_CORE: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(145u32);
pub const PRODUCT_STANDARD_A_SERVER_CORE: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(146u32);
pub const PRODUCT_DATACENTER_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(8u32);
pub const PRODUCT_DATACENTER_SERVER_CORE: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(12u32);
pub const PRODUCT_DATACENTER_SERVER_CORE_V: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(39u32);
pub const PRODUCT_DATACENTER_SERVER_V: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(37u32);
pub const PRODUCT_EDUCATION: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(121u32);
pub const PRODUCT_EDUCATION_N: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(122u32);
pub const PRODUCT_ENTERPRISE: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(4u32);
pub const PRODUCT_ENTERPRISE_E: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(70u32);
pub const PRODUCT_ENTERPRISE_EVALUATION: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(72u32);
pub const PRODUCT_ENTERPRISE_N: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(27u32);
pub const PRODUCT_ENTERPRISE_N_EVALUATION: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(84u32);
pub const PRODUCT_ENTERPRISE_S: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(125u32);
pub const PRODUCT_ENTERPRISE_S_EVALUATION: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(129u32);
pub const PRODUCT_ENTERPRISE_S_N: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(126u32);
pub const PRODUCT_ENTERPRISE_S_N_EVALUATION: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(130u32);
pub const PRODUCT_ENTERPRISE_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(10u32);
pub const PRODUCT_ENTERPRISE_SERVER_CORE: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(14u32);
pub const PRODUCT_ENTERPRISE_SERVER_CORE_V: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(41u32);
pub const PRODUCT_ENTERPRISE_SERVER_IA64: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(15u32);
pub const PRODUCT_ENTERPRISE_SERVER_V: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(38u32);
pub const PRODUCT_ESSENTIALBUSINESS_SERVER_ADDL: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(60u32);
pub const PRODUCT_ESSENTIALBUSINESS_SERVER_ADDLSVC: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(62u32);
pub const PRODUCT_ESSENTIALBUSINESS_SERVER_MGMT: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(59u32);
pub const PRODUCT_ESSENTIALBUSINESS_SERVER_MGMTSVC: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(61u32);
pub const PRODUCT_HOME_BASIC: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(2u32);
pub const PRODUCT_HOME_BASIC_E: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(67u32);
pub const PRODUCT_HOME_BASIC_N: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(5u32);
pub const PRODUCT_HOME_PREMIUM: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(3u32);
pub const PRODUCT_HOME_PREMIUM_E: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(68u32);
pub const PRODUCT_HOME_PREMIUM_N: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(26u32);
pub const PRODUCT_HOME_PREMIUM_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(34u32);
pub const PRODUCT_HOME_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(19u32);
pub const PRODUCT_HYPERV: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(42u32);
pub const PRODUCT_IOTUAP: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(123u32);
pub const PRODUCT_IOTUAPCOMMERCIAL: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(131u32);
pub const PRODUCT_MEDIUMBUSINESS_SERVER_MANAGEMENT: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(30u32);
pub const PRODUCT_MEDIUMBUSINESS_SERVER_MESSAGING: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(32u32);
pub const PRODUCT_MEDIUMBUSINESS_SERVER_SECURITY: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(31u32);
pub const PRODUCT_MOBILE_CORE: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(104u32);
pub const PRODUCT_MOBILE_ENTERPRISE: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(133u32);
pub const PRODUCT_MULTIPOINT_PREMIUM_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(77u32);
pub const PRODUCT_MULTIPOINT_STANDARD_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(76u32);
pub const PRODUCT_PRO_WORKSTATION: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(161u32);
pub const PRODUCT_PRO_WORKSTATION_N: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(162u32);
pub const PRODUCT_PROFESSIONAL: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(48u32);
pub const PRODUCT_PROFESSIONAL_E: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(69u32);
pub const PRODUCT_PROFESSIONAL_N: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(49u32);
pub const PRODUCT_PROFESSIONAL_WMC: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(103u32);
pub const PRODUCT_SB_SOLUTION_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(50u32);
pub const PRODUCT_SB_SOLUTION_SERVER_EM: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(54u32);
pub const PRODUCT_SERVER_FOR_SB_SOLUTIONS: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(51u32);
pub const PRODUCT_SERVER_FOR_SB_SOLUTIONS_EM: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(55u32);
pub const PRODUCT_SERVER_FOR_SMALLBUSINESS: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(24u32);
pub const PRODUCT_SERVER_FOR_SMALLBUSINESS_V: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(35u32);
pub const PRODUCT_SERVER_FOUNDATION: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(33u32);
pub const PRODUCT_SMALLBUSINESS_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(9u32);
pub const PRODUCT_SMALLBUSINESS_SERVER_PREMIUM: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(25u32);
pub const PRODUCT_SMALLBUSINESS_SERVER_PREMIUM_CORE: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(63u32);
pub const PRODUCT_SOLUTION_EMBEDDEDSERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(56u32);
pub const PRODUCT_STANDARD_EVALUATION_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(79u32);
pub const PRODUCT_STANDARD_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(7u32);
pub const PRODUCT_STANDARD_SERVER_CORE_: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(13u32);
pub const PRODUCT_STANDARD_SERVER_CORE_V: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(40u32);
pub const PRODUCT_STANDARD_SERVER_V: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(36u32);
pub const PRODUCT_STANDARD_SERVER_SOLUTIONS: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(52u32);
pub const PRODUCT_STANDARD_SERVER_SOLUTIONS_CORE: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(53u32);
pub const PRODUCT_STARTER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(11u32);
pub const PRODUCT_STARTER_E: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(66u32);
pub const PRODUCT_STARTER_N: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(47u32);
pub const PRODUCT_STORAGE_ENTERPRISE_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(23u32);
pub const PRODUCT_STORAGE_ENTERPRISE_SERVER_CORE: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(46u32);
pub const PRODUCT_STORAGE_EXPRESS_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(20u32);
pub const PRODUCT_STORAGE_EXPRESS_SERVER_CORE: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(43u32);
pub const PRODUCT_STORAGE_STANDARD_EVALUATION_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(96u32);
pub const PRODUCT_STORAGE_STANDARD_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(21u32);
pub const PRODUCT_STORAGE_STANDARD_SERVER_CORE: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(44u32);
pub const PRODUCT_STORAGE_WORKGROUP_EVALUATION_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(95u32);
pub const PRODUCT_STORAGE_WORKGROUP_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(22u32);
pub const PRODUCT_STORAGE_WORKGROUP_SERVER_CORE: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(45u32);
pub const PRODUCT_ULTIMATE: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(1u32);
pub const PRODUCT_ULTIMATE_E: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(71u32);
pub const PRODUCT_ULTIMATE_N: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(28u32);
pub const PRODUCT_UNDEFINED: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(0u32);
pub const PRODUCT_WEB_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(17u32);
pub const PRODUCT_WEB_SERVER_CORE: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(29u32);
impl ::std::convert::From<u32> for OS_PRODUCT_TYPE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for OS_PRODUCT_TYPE {
    type Abi = Self;
}
impl ::std::ops::BitOr for OS_PRODUCT_TYPE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for OS_PRODUCT_TYPE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for OS_PRODUCT_TYPE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for OS_PRODUCT_TYPE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for OS_PRODUCT_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Foundation")]
pub type PGET_SYSTEM_WOW64_DIRECTORY_A = unsafe extern "system" fn(lpbuffer: super::super::Foundation::PSTR, usize: u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PGET_SYSTEM_WOW64_DIRECTORY_W = unsafe extern "system" fn(lpbuffer: super::super::Foundation::PWSTR, usize: u32) -> u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PROCESSOR_CACHE_TYPE(pub i32);
pub const CacheUnified: PROCESSOR_CACHE_TYPE = PROCESSOR_CACHE_TYPE(0i32);
pub const CacheInstruction: PROCESSOR_CACHE_TYPE = PROCESSOR_CACHE_TYPE(1i32);
pub const CacheData: PROCESSOR_CACHE_TYPE = PROCESSOR_CACHE_TYPE(2i32);
pub const CacheTrace: PROCESSOR_CACHE_TYPE = PROCESSOR_CACHE_TYPE(3i32);
impl ::std::convert::From<i32> for PROCESSOR_CACHE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PROCESSOR_CACHE_TYPE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub struct PROCESSOR_GROUP_INFO {
    pub MaximumProcessorCount: u8,
    pub ActiveProcessorCount: u8,
    pub Reserved: [u8; 38],
    pub ActiveProcessorMask: usize,
}
impl PROCESSOR_GROUP_INFO {}
impl ::std::default::Default for PROCESSOR_GROUP_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PROCESSOR_GROUP_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PROCESSOR_GROUP_INFO").field("MaximumProcessorCount", &self.MaximumProcessorCount).field("ActiveProcessorCount", &self.ActiveProcessorCount).field("Reserved", &self.Reserved).field("ActiveProcessorMask", &self.ActiveProcessorMask).finish()
    }
}
impl ::std::cmp::PartialEq for PROCESSOR_GROUP_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.MaximumProcessorCount == other.MaximumProcessorCount && self.ActiveProcessorCount == other.ActiveProcessorCount && self.Reserved == other.Reserved && self.ActiveProcessorMask == other.ActiveProcessorMask
    }
}
impl ::std::cmp::Eq for PROCESSOR_GROUP_INFO {}
unsafe impl ::windows::runtime::Abi for PROCESSOR_GROUP_INFO {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub struct PROCESSOR_RELATIONSHIP {
    pub Flags: u8,
    pub EfficiencyClass: u8,
    pub Reserved: [u8; 20],
    pub GroupCount: u16,
    pub GroupMask: [GROUP_AFFINITY; 1],
}
impl PROCESSOR_RELATIONSHIP {}
impl ::std::default::Default for PROCESSOR_RELATIONSHIP {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PROCESSOR_RELATIONSHIP {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PROCESSOR_RELATIONSHIP").field("Flags", &self.Flags).field("EfficiencyClass", &self.EfficiencyClass).field("Reserved", &self.Reserved).field("GroupCount", &self.GroupCount).field("GroupMask", &self.GroupMask).finish()
    }
}
impl ::std::cmp::PartialEq for PROCESSOR_RELATIONSHIP {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.EfficiencyClass == other.EfficiencyClass && self.Reserved == other.Reserved && self.GroupCount == other.GroupCount && self.GroupMask == other.GroupMask
    }
}
impl ::std::cmp::Eq for PROCESSOR_RELATIONSHIP {}
unsafe impl ::windows::runtime::Abi for PROCESSOR_RELATIONSHIP {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const SCEX2_ALT_NETBIOS_NAME: u32 = 1u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const SPVERSION_MASK: u32 = 65280u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const SUBVERSION_MASK: u32 = 255u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub struct SYSTEM_CPU_SET_INFORMATION {
    pub Size: u32,
    pub Type: CPU_SET_INFORMATION_TYPE,
    pub Anonymous: SYSTEM_CPU_SET_INFORMATION_0,
}
impl SYSTEM_CPU_SET_INFORMATION {}
impl ::std::default::Default for SYSTEM_CPU_SET_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for SYSTEM_CPU_SET_INFORMATION {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for SYSTEM_CPU_SET_INFORMATION {}
unsafe impl ::windows::runtime::Abi for SYSTEM_CPU_SET_INFORMATION {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub union SYSTEM_CPU_SET_INFORMATION_0 {
    pub CpuSet: SYSTEM_CPU_SET_INFORMATION_0_0,
}
impl SYSTEM_CPU_SET_INFORMATION_0 {}
impl ::std::default::Default for SYSTEM_CPU_SET_INFORMATION_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for SYSTEM_CPU_SET_INFORMATION_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for SYSTEM_CPU_SET_INFORMATION_0 {}
unsafe impl ::windows::runtime::Abi for SYSTEM_CPU_SET_INFORMATION_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub struct SYSTEM_CPU_SET_INFORMATION_0_0 {
    pub Id: u32,
    pub Group: u16,
    pub LogicalProcessorIndex: u8,
    pub CoreIndex: u8,
    pub LastLevelCacheIndex: u8,
    pub NumaNodeIndex: u8,
    pub EfficiencyClass: u8,
    pub Anonymous1: SYSTEM_CPU_SET_INFORMATION_0_0_0,
    pub Anonymous2: SYSTEM_CPU_SET_INFORMATION_0_0_1,
    pub AllocationTag: u64,
}
impl SYSTEM_CPU_SET_INFORMATION_0_0 {}
impl ::std::default::Default for SYSTEM_CPU_SET_INFORMATION_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for SYSTEM_CPU_SET_INFORMATION_0_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for SYSTEM_CPU_SET_INFORMATION_0_0 {}
unsafe impl ::windows::runtime::Abi for SYSTEM_CPU_SET_INFORMATION_0_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub union SYSTEM_CPU_SET_INFORMATION_0_0_0 {
    pub AllFlags: u8,
    pub Anonymous: SYSTEM_CPU_SET_INFORMATION_0_0_0_0,
}
impl SYSTEM_CPU_SET_INFORMATION_0_0_0 {}
impl ::std::default::Default for SYSTEM_CPU_SET_INFORMATION_0_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for SYSTEM_CPU_SET_INFORMATION_0_0_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for SYSTEM_CPU_SET_INFORMATION_0_0_0 {}
unsafe impl ::windows::runtime::Abi for SYSTEM_CPU_SET_INFORMATION_0_0_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub struct SYSTEM_CPU_SET_INFORMATION_0_0_0_0 {
    pub _bitfield: u8,
}
impl SYSTEM_CPU_SET_INFORMATION_0_0_0_0 {}
impl ::std::default::Default for SYSTEM_CPU_SET_INFORMATION_0_0_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SYSTEM_CPU_SET_INFORMATION_0_0_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::std::cmp::PartialEq for SYSTEM_CPU_SET_INFORMATION_0_0_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for SYSTEM_CPU_SET_INFORMATION_0_0_0_0 {}
unsafe impl ::windows::runtime::Abi for SYSTEM_CPU_SET_INFORMATION_0_0_0_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub union SYSTEM_CPU_SET_INFORMATION_0_0_1 {
    pub Reserved: u32,
    pub SchedulingClass: u8,
}
impl SYSTEM_CPU_SET_INFORMATION_0_0_1 {}
impl ::std::default::Default for SYSTEM_CPU_SET_INFORMATION_0_0_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for SYSTEM_CPU_SET_INFORMATION_0_0_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for SYSTEM_CPU_SET_INFORMATION_0_0_1 {}
unsafe impl ::windows::runtime::Abi for SYSTEM_CPU_SET_INFORMATION_0_0_1 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
#[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_System_Diagnostics_Debug`*"]
pub struct SYSTEM_INFO {
    pub Anonymous: SYSTEM_INFO_0,
    pub dwPageSize: u32,
    pub lpMinimumApplicationAddress: *mut ::std::ffi::c_void,
    pub lpMaximumApplicationAddress: *mut ::std::ffi::c_void,
    pub dwActiveProcessorMask: usize,
    pub dwNumberOfProcessors: u32,
    pub dwProcessorType: u32,
    pub dwAllocationGranularity: u32,
    pub wProcessorLevel: u16,
    pub wProcessorRevision: u16,
}
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl SYSTEM_INFO {}
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl ::std::default::Default for SYSTEM_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl ::std::cmp::PartialEq for SYSTEM_INFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl ::std::cmp::Eq for SYSTEM_INFO {}
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
unsafe impl ::windows::runtime::Abi for SYSTEM_INFO {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
#[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_System_Diagnostics_Debug`*"]
pub union SYSTEM_INFO_0 {
    pub dwOemId: u32,
    pub Anonymous: SYSTEM_INFO_0_0,
}
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl SYSTEM_INFO_0 {}
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl ::std::default::Default for SYSTEM_INFO_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl ::std::cmp::PartialEq for SYSTEM_INFO_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl ::std::cmp::Eq for SYSTEM_INFO_0 {}
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
unsafe impl ::windows::runtime::Abi for SYSTEM_INFO_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
#[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_System_Diagnostics_Debug`*"]
pub struct SYSTEM_INFO_0_0 {
    pub wProcessorArchitecture: super::Diagnostics::Debug::PROCESSOR_ARCHITECTURE,
    pub wReserved: u16,
}
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl SYSTEM_INFO_0_0 {}
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl ::std::default::Default for SYSTEM_INFO_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl ::std::fmt::Debug for SYSTEM_INFO_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("wProcessorArchitecture", &self.wProcessorArchitecture).field("wReserved", &self.wReserved).finish()
    }
}
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl ::std::cmp::PartialEq for SYSTEM_INFO_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.wProcessorArchitecture == other.wProcessorArchitecture && self.wReserved == other.wReserved
    }
}
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl ::std::cmp::Eq for SYSTEM_INFO_0_0 {}
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
unsafe impl ::windows::runtime::Abi for SYSTEM_INFO_0_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub struct SYSTEM_LOGICAL_PROCESSOR_INFORMATION {
    pub ProcessorMask: usize,
    pub Relationship: LOGICAL_PROCESSOR_RELATIONSHIP,
    pub Anonymous: SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0,
}
impl SYSTEM_LOGICAL_PROCESSOR_INFORMATION {}
impl ::std::default::Default for SYSTEM_LOGICAL_PROCESSOR_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for SYSTEM_LOGICAL_PROCESSOR_INFORMATION {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for SYSTEM_LOGICAL_PROCESSOR_INFORMATION {}
unsafe impl ::windows::runtime::Abi for SYSTEM_LOGICAL_PROCESSOR_INFORMATION {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub union SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0 {
    pub ProcessorCore: SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0_1,
    pub NumaNode: SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0_0,
    pub Cache: CACHE_DESCRIPTOR,
    pub Reserved: [u64; 2],
}
impl SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0 {}
impl ::std::default::Default for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0 {}
unsafe impl ::windows::runtime::Abi for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub struct SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0_0 {
    pub NodeNumber: u32,
}
impl SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0_0 {}
impl ::std::default::Default for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_NumaNode_e__Struct").field("NodeNumber", &self.NodeNumber).finish()
    }
}
impl ::std::cmp::PartialEq for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.NodeNumber == other.NodeNumber
    }
}
impl ::std::cmp::Eq for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0_0 {}
unsafe impl ::windows::runtime::Abi for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub struct SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0_1 {
    pub Flags: u8,
}
impl SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0_1 {}
impl ::std::default::Default for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0_1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_ProcessorCore_e__Struct").field("Flags", &self.Flags).finish()
    }
}
impl ::std::cmp::PartialEq for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags
    }
}
impl ::std::cmp::Eq for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0_1 {}
unsafe impl ::windows::runtime::Abi for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0_1 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub struct SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX {
    pub Relationship: LOGICAL_PROCESSOR_RELATIONSHIP,
    pub Size: u32,
    pub Anonymous: SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX_0,
}
impl SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX {}
impl ::std::default::Default for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX {}
unsafe impl ::windows::runtime::Abi for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub union SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX_0 {
    pub Processor: PROCESSOR_RELATIONSHIP,
    pub NumaNode: NUMA_NODE_RELATIONSHIP,
    pub Cache: CACHE_RELATIONSHIP,
    pub Group: GROUP_RELATIONSHIP,
}
impl SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX_0 {}
impl ::std::default::Default for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX_0 {}
unsafe impl ::windows::runtime::Abi for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
pub struct SYSTEM_POOL_ZEROING_INFORMATION {
    pub PoolZeroingSupportPresent: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl SYSTEM_POOL_ZEROING_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SYSTEM_POOL_ZEROING_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for SYSTEM_POOL_ZEROING_INFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SYSTEM_POOL_ZEROING_INFORMATION").field("PoolZeroingSupportPresent", &self.PoolZeroingSupportPresent).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SYSTEM_POOL_ZEROING_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.PoolZeroingSupportPresent == other.PoolZeroingSupportPresent
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SYSTEM_POOL_ZEROING_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SYSTEM_POOL_ZEROING_INFORMATION {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub struct SYSTEM_PROCESSOR_CYCLE_TIME_INFORMATION {
    pub CycleTime: u64,
}
impl SYSTEM_PROCESSOR_CYCLE_TIME_INFORMATION {}
impl ::std::default::Default for SYSTEM_PROCESSOR_CYCLE_TIME_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SYSTEM_PROCESSOR_CYCLE_TIME_INFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SYSTEM_PROCESSOR_CYCLE_TIME_INFORMATION").field("CycleTime", &self.CycleTime).finish()
    }
}
impl ::std::cmp::PartialEq for SYSTEM_PROCESSOR_CYCLE_TIME_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.CycleTime == other.CycleTime
    }
}
impl ::std::cmp::Eq for SYSTEM_PROCESSOR_CYCLE_TIME_INFORMATION {}
unsafe impl ::windows::runtime::Abi for SYSTEM_PROCESSOR_CYCLE_TIME_INFORMATION {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub struct SYSTEM_SUPPORTED_PROCESSOR_ARCHITECTURES_INFORMATION {
    pub _bitfield: u32,
}
impl SYSTEM_SUPPORTED_PROCESSOR_ARCHITECTURES_INFORMATION {}
impl ::std::default::Default for SYSTEM_SUPPORTED_PROCESSOR_ARCHITECTURES_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SYSTEM_SUPPORTED_PROCESSOR_ARCHITECTURES_INFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SYSTEM_SUPPORTED_PROCESSOR_ARCHITECTURES_INFORMATION").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::std::cmp::PartialEq for SYSTEM_SUPPORTED_PROCESSOR_ARCHITECTURES_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for SYSTEM_SUPPORTED_PROCESSOR_ARCHITECTURES_INFORMATION {}
unsafe impl ::windows::runtime::Abi for SYSTEM_SUPPORTED_PROCESSOR_ARCHITECTURES_INFORMATION {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetComputerNameA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(lpcomputername: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetComputerNameA(lpcomputername: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetComputerNameA(lpcomputername.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetComputerNameEx2W<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(nametype: COMPUTER_NAME_FORMAT, flags: u32, lpbuffer: Param2) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetComputerNameEx2W(nametype: COMPUTER_NAME_FORMAT, flags: u32, lpbuffer: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetComputerNameEx2W(::std::mem::transmute(nametype), ::std::mem::transmute(flags), lpbuffer.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetComputerNameExA<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(nametype: COMPUTER_NAME_FORMAT, lpbuffer: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetComputerNameExA(nametype: COMPUTER_NAME_FORMAT, lpbuffer: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetComputerNameExA(::std::mem::transmute(nametype), lpbuffer.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetComputerNameExW<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(nametype: COMPUTER_NAME_FORMAT, lpbuffer: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetComputerNameExW(nametype: COMPUTER_NAME_FORMAT, lpbuffer: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetComputerNameExW(::std::mem::transmute(nametype), lpbuffer.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetComputerNameW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(lpcomputername: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetComputerNameW(lpcomputername: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetComputerNameW(lpcomputername.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetLocalTime(lpsystemtime: *const super::super::Foundation::SYSTEMTIME) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetLocalTime(lpsystemtime: *const super::super::Foundation::SYSTEMTIME) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetLocalTime(::std::mem::transmute(lpsystemtime)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetSystemTime(lpsystemtime: *const super::super::Foundation::SYSTEMTIME) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetSystemTime(lpsystemtime: *const super::super::Foundation::SYSTEMTIME) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetSystemTime(::std::mem::transmute(lpsystemtime)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetSystemTimeAdjustment<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(dwtimeadjustment: u32, btimeadjustmentdisabled: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetSystemTimeAdjustment(dwtimeadjustment: u32, btimeadjustmentdisabled: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetSystemTimeAdjustment(::std::mem::transmute(dwtimeadjustment), btimeadjustmentdisabled.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetSystemTimeAdjustmentPrecise<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(dwtimeadjustment: u64, btimeadjustmentdisabled: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetSystemTimeAdjustmentPrecise(dwtimeadjustment: u64, btimeadjustmentdisabled: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetSystemTimeAdjustmentPrecise(::std::mem::transmute(dwtimeadjustment), btimeadjustmentdisabled.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct USER_CET_ENVIRONMENT(pub u32);
pub const USER_CET_ENVIRONMENT_WIN32_PROCESS: USER_CET_ENVIRONMENT = USER_CET_ENVIRONMENT(0u32);
pub const USER_CET_ENVIRONMENT_SGX2_ENCLAVE: USER_CET_ENVIRONMENT = USER_CET_ENVIRONMENT(2u32);
pub const USER_CET_ENVIRONMENT_VBS_ENCLAVE: USER_CET_ENVIRONMENT = USER_CET_ENVIRONMENT(16u32);
pub const USER_CET_ENVIRONMENT_VBS_BASIC_ENCLAVE: USER_CET_ENVIRONMENT = USER_CET_ENVIRONMENT(17u32);
impl ::std::convert::From<u32> for USER_CET_ENVIRONMENT {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for USER_CET_ENVIRONMENT {
    type Abi = Self;
}
impl ::std::ops::BitOr for USER_CET_ENVIRONMENT {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for USER_CET_ENVIRONMENT {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for USER_CET_ENVIRONMENT {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for USER_CET_ENVIRONMENT {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for USER_CET_ENVIRONMENT {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct VER_FLAGS(pub u32);
pub const VER_MINORVERSION: VER_FLAGS = VER_FLAGS(1u32);
pub const VER_MAJORVERSION: VER_FLAGS = VER_FLAGS(2u32);
pub const VER_BUILDNUMBER: VER_FLAGS = VER_FLAGS(4u32);
pub const VER_PLATFORMID: VER_FLAGS = VER_FLAGS(8u32);
pub const VER_SERVICEPACKMINOR: VER_FLAGS = VER_FLAGS(16u32);
pub const VER_SERVICEPACKMAJOR: VER_FLAGS = VER_FLAGS(32u32);
pub const VER_SUITENAME: VER_FLAGS = VER_FLAGS(64u32);
pub const VER_PRODUCT_TYPE: VER_FLAGS = VER_FLAGS(128u32);
impl ::std::convert::From<u32> for VER_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VER_FLAGS {
    type Abi = Self;
}
impl ::std::ops::BitOr for VER_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for VER_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for VER_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for VER_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for VER_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
#[inline]
pub unsafe fn VerSetConditionMask(conditionmask: u64, typemask: VER_FLAGS, condition: u8) -> u64 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VerSetConditionMask(conditionmask: u64, typemask: VER_FLAGS, condition: u8) -> u64;
        }
        ::std::mem::transmute(VerSetConditionMask(::std::mem::transmute(conditionmask), ::std::mem::transmute(typemask), ::std::mem::transmute(condition)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VerifyVersionInfoA(lpversioninformation: *mut OSVERSIONINFOEXA, dwtypemask: VER_FLAGS, dwlconditionmask: u64) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VerifyVersionInfoA(lpversioninformation: *mut OSVERSIONINFOEXA, dwtypemask: VER_FLAGS, dwlconditionmask: u64) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(VerifyVersionInfoA(::std::mem::transmute(lpversioninformation), ::std::mem::transmute(dwtypemask), ::std::mem::transmute(dwlconditionmask)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VerifyVersionInfoW(lpversioninformation: *mut OSVERSIONINFOEXW, dwtypemask: VER_FLAGS, dwlconditionmask: u64) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VerifyVersionInfoW(lpversioninformation: *mut OSVERSIONINFOEXW, dwtypemask: VER_FLAGS, dwlconditionmask: u64) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(VerifyVersionInfoW(::std::mem::transmute(lpversioninformation), ::std::mem::transmute(dwtypemask), ::std::mem::transmute(dwlconditionmask)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const WDK_NTDDI_VERSION: u32 = 167772171u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const _WIN32_IE_IE100: u32 = 2560u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const _WIN32_IE_IE110: u32 = 2560u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const _WIN32_IE_IE20: u32 = 512u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const _WIN32_IE_IE30: u32 = 768u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const _WIN32_IE_IE302: u32 = 770u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const _WIN32_IE_IE40: u32 = 1024u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const _WIN32_IE_IE401: u32 = 1025u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const _WIN32_IE_IE50: u32 = 1280u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const _WIN32_IE_IE501: u32 = 1281u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const _WIN32_IE_IE55: u32 = 1360u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const _WIN32_IE_IE60: u32 = 1536u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const _WIN32_IE_IE60SP1: u32 = 1537u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const _WIN32_IE_IE60SP2: u32 = 1539u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const _WIN32_IE_IE70: u32 = 1792u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const _WIN32_IE_IE80: u32 = 2048u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const _WIN32_IE_IE90: u32 = 2304u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const _WIN32_IE_LONGHORN: u32 = 1792u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const _WIN32_IE_NT4: u32 = 512u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const _WIN32_IE_NT4SP1: u32 = 512u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const _WIN32_IE_NT4SP2: u32 = 512u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const _WIN32_IE_NT4SP3: u32 = 770u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const _WIN32_IE_NT4SP4: u32 = 1025u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const _WIN32_IE_NT4SP5: u32 = 1025u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const _WIN32_IE_NT4SP6: u32 = 1280u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const _WIN32_IE_WIN10: u32 = 2560u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const _WIN32_IE_WIN2K: u32 = 1281u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const _WIN32_IE_WIN2KSP1: u32 = 1281u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const _WIN32_IE_WIN2KSP2: u32 = 1281u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const _WIN32_IE_WIN2KSP3: u32 = 1281u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const _WIN32_IE_WIN2KSP4: u32 = 1281u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const _WIN32_IE_WIN6: u32 = 1792u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const _WIN32_IE_WIN7: u32 = 2048u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const _WIN32_IE_WIN8: u32 = 2560u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const _WIN32_IE_WIN98: u32 = 1025u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const _WIN32_IE_WIN98SE: u32 = 1280u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const _WIN32_IE_WINBLUE: u32 = 2560u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const _WIN32_IE_WINME: u32 = 1360u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const _WIN32_IE_WINTHRESHOLD: u32 = 2560u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const _WIN32_IE_WS03: u32 = 1538u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const _WIN32_IE_WS03SP1: u32 = 1539u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const _WIN32_IE_XP: u32 = 1536u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const _WIN32_IE_XPSP1: u32 = 1537u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const _WIN32_IE_XPSP2: u32 = 1539u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const _WIN32_WINNT_LONGHORN: u32 = 1536u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const _WIN32_WINNT_NT4: u32 = 1024u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const _WIN32_WINNT_VISTA: u32 = 1536u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const _WIN32_WINNT_WIN10: u32 = 2560u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const _WIN32_WINNT_WIN2K: u32 = 1280u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const _WIN32_WINNT_WIN6: u32 = 1536u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const _WIN32_WINNT_WIN7: u32 = 1537u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const _WIN32_WINNT_WIN8: u32 = 1538u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const _WIN32_WINNT_WINBLUE: u32 = 1539u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const _WIN32_WINNT_WINTHRESHOLD: u32 = 2560u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const _WIN32_WINNT_WINXP: u32 = 1281u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const _WIN32_WINNT_WS03: u32 = 1282u32;
#[doc = "*Required features: `Win32_System_SystemInformation`*"]
pub const _WIN32_WINNT_WS08: u32 = 1536u32;
