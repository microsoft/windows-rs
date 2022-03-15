#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "Win32_System_Performance_HardwareCounterProfiling")]
pub mod HardwareCounterProfiling;
pub const AppearPropPage: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe49741e9_93a8_4ab1_8e96_bf4482282e9c);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AutoPathFormat(pub i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaNone: AutoPathFormat = AutoPathFormat(0i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaPattern: AutoPathFormat = AutoPathFormat(1i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaComputer: AutoPathFormat = AutoPathFormat(2i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaMonthDayHour: AutoPathFormat = AutoPathFormat(256i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaSerialNumber: AutoPathFormat = AutoPathFormat(512i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaYearDayOfYear: AutoPathFormat = AutoPathFormat(1024i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaYearMonth: AutoPathFormat = AutoPathFormat(2048i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaYearMonthDay: AutoPathFormat = AutoPathFormat(4096i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaYearMonthDayHour: AutoPathFormat = AutoPathFormat(8192i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaMonthDayHourMinute: AutoPathFormat = AutoPathFormat(16384i32);
impl ::core::marker::Copy for AutoPathFormat {}
impl ::core::clone::Clone for AutoPathFormat {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AutoPathFormat {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AutoPathFormat {
    type Abi = Self;
}
impl ::core::fmt::Debug for AutoPathFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AutoPathFormat").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn BackupPerfRegistryToFileW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(szfilename: Param0, szcommentstring: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BackupPerfRegistryToFileW(szfilename: ::windows::core::PCWSTR, szcommentstring: ::windows::core::PCWSTR) -> u32;
        }
        ::core::mem::transmute(BackupPerfRegistryToFileW(szfilename.into_param().abi(), szcommentstring.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const BootTraceSession: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03837538_098b_11d8_9414_505054503030);
pub const BootTraceSessionCollection: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03837539_098b_11d8_9414_505054503030);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ClockType(pub i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaTimeStamp: ClockType = ClockType(0i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaPerformance: ClockType = ClockType(1i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaSystem: ClockType = ClockType(2i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaCycle: ClockType = ClockType(3i32);
impl ::core::marker::Copy for ClockType {}
impl ::core::clone::Clone for ClockType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ClockType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ClockType {
    type Abi = Self;
}
impl ::core::fmt::Debug for ClockType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ClockType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CommitMode(pub i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaCreateNew: CommitMode = CommitMode(1i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaModify: CommitMode = CommitMode(2i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaCreateOrModify: CommitMode = CommitMode(3i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaUpdateRunningInstance: CommitMode = CommitMode(16i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaFlushTrace: CommitMode = CommitMode(32i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaValidateOnly: CommitMode = CommitMode(4096i32);
impl ::core::marker::Copy for CommitMode {}
impl ::core::clone::Clone for CommitMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CommitMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CommitMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for CommitMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CommitMode").field(&self.0).finish()
    }
}
pub const CounterItem: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc4d2d8e0_d1dd_11ce_940f_008029004348);
pub const CounterItem2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x43196c62_c31f_4ce3_a02e_79efe0f6a525);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub type CounterPathCallBack = ::core::option::Option<unsafe extern "system" fn(param0: usize) -> i32>;
pub const CounterPropPage: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcf948561_ede8_11ce_941e_008029004347);
pub const Counters: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb2b066d2_2aac_11cf_942f_008029004347);
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct DICounterItem(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl DICounterItem {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<DICounterItem> for ::windows::core::IUnknown {
    fn from(value: DICounterItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&DICounterItem> for ::windows::core::IUnknown {
    fn from(value: &DICounterItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DICounterItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DICounterItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<DICounterItem> for super::Com::IDispatch {
    fn from(value: DICounterItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&DICounterItem> for super::Com::IDispatch {
    fn from(value: &DICounterItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for DICounterItem {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a DICounterItem {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for DICounterItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for DICounterItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for DICounterItem {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for DICounterItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DICounterItem").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for DICounterItem {
    type Vtable = DICounterItem_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc08c4ff2_0e2e_11cf_942c_008029004347);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct DICounterItem_Vtbl {
    pub base: super::Com::IDispatch_Vtbl,
}
pub const DIID_DICounterItem: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc08c4ff2_0e2e_11cf_942c_008029004347);
pub const DIID_DILogFileItem: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8d093ffc_f777_4917_82d1_833fbc54c58f);
pub const DIID_DISystemMonitor: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x13d73d81_c32e_11cf_9398_00aa00a3ddea);
pub const DIID_DISystemMonitorEvents: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x84979930_4ab3_11cf_943a_008029004347);
pub const DIID_DISystemMonitorInternal: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x194eb242_c32c_11cf_9398_00aa00a3ddea);
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct DILogFileItem(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl DILogFileItem {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<DILogFileItem> for ::windows::core::IUnknown {
    fn from(value: DILogFileItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&DILogFileItem> for ::windows::core::IUnknown {
    fn from(value: &DILogFileItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DILogFileItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DILogFileItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<DILogFileItem> for super::Com::IDispatch {
    fn from(value: DILogFileItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&DILogFileItem> for super::Com::IDispatch {
    fn from(value: &DILogFileItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for DILogFileItem {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a DILogFileItem {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for DILogFileItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for DILogFileItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for DILogFileItem {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for DILogFileItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DILogFileItem").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for DILogFileItem {
    type Vtable = DILogFileItem_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8d093ffc_f777_4917_82d1_833fbc54c58f);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct DILogFileItem_Vtbl {
    pub base: super::Com::IDispatch_Vtbl,
}
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct DISystemMonitor(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl DISystemMonitor {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<DISystemMonitor> for ::windows::core::IUnknown {
    fn from(value: DISystemMonitor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&DISystemMonitor> for ::windows::core::IUnknown {
    fn from(value: &DISystemMonitor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DISystemMonitor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DISystemMonitor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<DISystemMonitor> for super::Com::IDispatch {
    fn from(value: DISystemMonitor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&DISystemMonitor> for super::Com::IDispatch {
    fn from(value: &DISystemMonitor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for DISystemMonitor {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a DISystemMonitor {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for DISystemMonitor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for DISystemMonitor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for DISystemMonitor {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for DISystemMonitor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISystemMonitor").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for DISystemMonitor {
    type Vtable = DISystemMonitor_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x13d73d81_c32e_11cf_9398_00aa00a3ddea);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct DISystemMonitor_Vtbl {
    pub base: super::Com::IDispatch_Vtbl,
}
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct DISystemMonitorEvents(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl DISystemMonitorEvents {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<DISystemMonitorEvents> for ::windows::core::IUnknown {
    fn from(value: DISystemMonitorEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&DISystemMonitorEvents> for ::windows::core::IUnknown {
    fn from(value: &DISystemMonitorEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DISystemMonitorEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DISystemMonitorEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<DISystemMonitorEvents> for super::Com::IDispatch {
    fn from(value: DISystemMonitorEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&DISystemMonitorEvents> for super::Com::IDispatch {
    fn from(value: &DISystemMonitorEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for DISystemMonitorEvents {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a DISystemMonitorEvents {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for DISystemMonitorEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for DISystemMonitorEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for DISystemMonitorEvents {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for DISystemMonitorEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISystemMonitorEvents").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for DISystemMonitorEvents {
    type Vtable = DISystemMonitorEvents_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x84979930_4ab3_11cf_943a_008029004347);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct DISystemMonitorEvents_Vtbl {
    pub base: super::Com::IDispatch_Vtbl,
}
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct DISystemMonitorInternal(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl DISystemMonitorInternal {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<DISystemMonitorInternal> for ::windows::core::IUnknown {
    fn from(value: DISystemMonitorInternal) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&DISystemMonitorInternal> for ::windows::core::IUnknown {
    fn from(value: &DISystemMonitorInternal) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DISystemMonitorInternal {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DISystemMonitorInternal {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<DISystemMonitorInternal> for super::Com::IDispatch {
    fn from(value: DISystemMonitorInternal) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&DISystemMonitorInternal> for super::Com::IDispatch {
    fn from(value: &DISystemMonitorInternal) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for DISystemMonitorInternal {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a DISystemMonitorInternal {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for DISystemMonitorInternal {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for DISystemMonitorInternal {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for DISystemMonitorInternal {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for DISystemMonitorInternal {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISystemMonitorInternal").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for DISystemMonitorInternal {
    type Vtable = DISystemMonitorInternal_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x194eb242_c32c_11cf_9398_00aa00a3ddea);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct DISystemMonitorInternal_Vtbl {
    pub base: super::Com::IDispatch_Vtbl,
}
pub const DataCollectorSet: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03837521_098b_11d8_9414_505054503030);
pub const DataCollectorSetCollection: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03837525_098b_11d8_9414_505054503030);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DataCollectorSetStatus(pub i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaStopped: DataCollectorSetStatus = DataCollectorSetStatus(0i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaRunning: DataCollectorSetStatus = DataCollectorSetStatus(1i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaCompiling: DataCollectorSetStatus = DataCollectorSetStatus(2i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaPending: DataCollectorSetStatus = DataCollectorSetStatus(3i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaUndefined: DataCollectorSetStatus = DataCollectorSetStatus(4i32);
impl ::core::marker::Copy for DataCollectorSetStatus {}
impl ::core::clone::Clone for DataCollectorSetStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DataCollectorSetStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DataCollectorSetStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for DataCollectorSetStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DataCollectorSetStatus").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DataCollectorType(pub i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaPerformanceCounter: DataCollectorType = DataCollectorType(0i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaTrace: DataCollectorType = DataCollectorType(1i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaConfiguration: DataCollectorType = DataCollectorType(2i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaAlert: DataCollectorType = DataCollectorType(3i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaApiTrace: DataCollectorType = DataCollectorType(4i32);
impl ::core::marker::Copy for DataCollectorType {}
impl ::core::clone::Clone for DataCollectorType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DataCollectorType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DataCollectorType {
    type Abi = Self;
}
impl ::core::fmt::Debug for DataCollectorType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DataCollectorType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DataManagerSteps(pub i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaCreateReport: DataManagerSteps = DataManagerSteps(1i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaRunRules: DataManagerSteps = DataManagerSteps(2i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaCreateHtml: DataManagerSteps = DataManagerSteps(4i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaFolderActions: DataManagerSteps = DataManagerSteps(8i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaResourceFreeing: DataManagerSteps = DataManagerSteps(16i32);
impl ::core::marker::Copy for DataManagerSteps {}
impl ::core::clone::Clone for DataManagerSteps {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DataManagerSteps {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DataManagerSteps {
    type Abi = Self;
}
impl ::core::fmt::Debug for DataManagerSteps {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DataManagerSteps").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DataSourceTypeConstants(pub i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const sysmonNullDataSource: DataSourceTypeConstants = DataSourceTypeConstants(-1i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const sysmonCurrentActivity: DataSourceTypeConstants = DataSourceTypeConstants(1i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const sysmonLogFiles: DataSourceTypeConstants = DataSourceTypeConstants(2i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const sysmonSqlLog: DataSourceTypeConstants = DataSourceTypeConstants(3i32);
impl ::core::marker::Copy for DataSourceTypeConstants {}
impl ::core::clone::Clone for DataSourceTypeConstants {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DataSourceTypeConstants {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DataSourceTypeConstants {
    type Abi = Self;
}
impl ::core::fmt::Debug for DataSourceTypeConstants {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DataSourceTypeConstants").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DisplayTypeConstants(pub i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const sysmonLineGraph: DisplayTypeConstants = DisplayTypeConstants(1i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const sysmonHistogram: DisplayTypeConstants = DisplayTypeConstants(2i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const sysmonReport: DisplayTypeConstants = DisplayTypeConstants(3i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const sysmonChartArea: DisplayTypeConstants = DisplayTypeConstants(4i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const sysmonChartStackedArea: DisplayTypeConstants = DisplayTypeConstants(5i32);
impl ::core::marker::Copy for DisplayTypeConstants {}
impl ::core::clone::Clone for DisplayTypeConstants {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DisplayTypeConstants {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DisplayTypeConstants {
    type Abi = Self;
}
impl ::core::fmt::Debug for DisplayTypeConstants {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayTypeConstants").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FileFormat(pub i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaCommaSeparated: FileFormat = FileFormat(0i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaTabSeparated: FileFormat = FileFormat(1i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaSql: FileFormat = FileFormat(2i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaBinary: FileFormat = FileFormat(3i32);
impl ::core::marker::Copy for FileFormat {}
impl ::core::clone::Clone for FileFormat {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FileFormat {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FileFormat {
    type Abi = Self;
}
impl ::core::fmt::Debug for FileFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FileFormat").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FolderActionSteps(pub i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaCreateCab: FolderActionSteps = FolderActionSteps(1i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaDeleteData: FolderActionSteps = FolderActionSteps(2i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaSendCab: FolderActionSteps = FolderActionSteps(4i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaDeleteCab: FolderActionSteps = FolderActionSteps(8i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaDeleteReport: FolderActionSteps = FolderActionSteps(16i32);
impl ::core::marker::Copy for FolderActionSteps {}
impl ::core::clone::Clone for FolderActionSteps {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FolderActionSteps {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FolderActionSteps {
    type Abi = Self;
}
impl ::core::fmt::Debug for FolderActionSteps {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FolderActionSteps").field(&self.0).finish()
    }
}
pub const GeneralPropPage: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc3e5d3d2_1a03_11cf_942d_008029004347);
pub const GraphPropPage: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc3e5d3d3_1a03_11cf_942d_008029004347);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const H_WBEM_DATASOURCE: i32 = -1i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IAlertDataCollector(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IAlertDataCollector {
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DataCollectorSet(&self) -> ::windows::core::Result<IDataCollectorSet> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.DataCollectorSet)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDataCollectorSet>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetDataCollectorSet<'a, Param0: ::windows::core::IntoParam<'a, IDataCollectorSet>>(&self, group: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetDataCollectorSet)(::core::mem::transmute_copy(self), group.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn DataCollectorType(&self) -> ::windows::core::Result<DataCollectorType> {
        let mut result__: DataCollectorType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.DataCollectorType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DataCollectorType>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FileName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.FileName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFileName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetFileName)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn FileNameFormat(&self) -> ::windows::core::Result<AutoPathFormat> {
        let mut result__: AutoPathFormat = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.FileNameFormat)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<AutoPathFormat>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetFileNameFormat(&self, format: AutoPathFormat) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetFileNameFormat)(::core::mem::transmute_copy(self), ::core::mem::transmute(format)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FileNameFormatPattern(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.FileNameFormatPattern)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFileNameFormatPattern<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, pattern: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetFileNameFormatPattern)(::core::mem::transmute_copy(self), pattern.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LatestOutputLocation(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.LatestOutputLocation)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLatestOutputLocation<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetLatestOutputLocation)(::core::mem::transmute_copy(self), path.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn LogAppend(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.LogAppend)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetLogAppend(&self, append: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetLogAppend)(::core::mem::transmute_copy(self), ::core::mem::transmute(append)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn LogCircular(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.LogCircular)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetLogCircular(&self, circular: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetLogCircular)(::core::mem::transmute_copy(self), ::core::mem::transmute(circular)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn LogOverwrite(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.LogOverwrite)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetLogOverwrite(&self, overwrite: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetLogOverwrite)(::core::mem::transmute_copy(self), ::core::mem::transmute(overwrite)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Name)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetName)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OutputLocation(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.OutputLocation)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn Index(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Index)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetIndex(&self, index: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetIndex)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Xml(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Xml)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn SetXml<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, xml: Param0) -> ::windows::core::Result<IValueMap> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.SetXml)(::core::mem::transmute_copy(self), xml.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IValueMap>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateOutputLocation(&self, latest: i16) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateOutputLocation)(::core::mem::transmute_copy(self), ::core::mem::transmute(latest), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AlertThresholds(&self) -> ::windows::core::Result<*mut super::Com::SAFEARRAY> {
        let mut result__: *mut super::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).AlertThresholds)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetAlertThresholds(&self, alerts: *const super::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAlertThresholds)(::core::mem::transmute_copy(self), ::core::mem::transmute(alerts)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn EventLog(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).EventLog)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetEventLog(&self, log: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetEventLog)(::core::mem::transmute_copy(self), ::core::mem::transmute(log)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SampleInterval(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SampleInterval)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetSampleInterval(&self, interval: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSampleInterval)(::core::mem::transmute_copy(self), ::core::mem::transmute(interval)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Task(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Task)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetTask<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, task: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTask)(::core::mem::transmute_copy(self), task.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn TaskRunAsSelf(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).TaskRunAsSelf)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetTaskRunAsSelf(&self, runasself: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTaskRunAsSelf)(::core::mem::transmute_copy(self), ::core::mem::transmute(runasself)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TaskArguments(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).TaskArguments)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetTaskArguments<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, task: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTaskArguments)(::core::mem::transmute_copy(self), task.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TaskUserTextArguments(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).TaskUserTextArguments)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetTaskUserTextArguments<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, task: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTaskUserTextArguments)(::core::mem::transmute_copy(self), task.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TriggerDataCollectorSet(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).TriggerDataCollectorSet)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetTriggerDataCollectorSet<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTriggerDataCollectorSet)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAlertDataCollector> for ::windows::core::IUnknown {
    fn from(value: IAlertDataCollector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAlertDataCollector> for ::windows::core::IUnknown {
    fn from(value: &IAlertDataCollector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAlertDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAlertDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAlertDataCollector> for super::Com::IDispatch {
    fn from(value: IAlertDataCollector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAlertDataCollector> for super::Com::IDispatch {
    fn from(value: &IAlertDataCollector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IAlertDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a IAlertDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAlertDataCollector> for IDataCollector {
    fn from(value: IAlertDataCollector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAlertDataCollector> for IDataCollector {
    fn from(value: &IAlertDataCollector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IDataCollector> for IAlertDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, IDataCollector> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IDataCollector> for &'a IAlertDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, IDataCollector> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IAlertDataCollector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IAlertDataCollector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IAlertDataCollector {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IAlertDataCollector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAlertDataCollector").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IAlertDataCollector {
    type Vtable = IAlertDataCollector_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03837516_098b_11d8_9414_505054503030);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAlertDataCollector_Vtbl {
    pub base: IDataCollector_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub AlertThresholds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, alerts: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AlertThresholds: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetAlertThresholds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, alerts: *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetAlertThresholds: usize,
    pub EventLog: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, log: *mut i16) -> ::windows::core::HRESULT,
    pub SetEventLog: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, log: i16) -> ::windows::core::HRESULT,
    pub SampleInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interval: *mut u32) -> ::windows::core::HRESULT,
    pub SetSampleInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interval: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Task: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, task: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Task: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetTask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, task: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetTask: usize,
    pub TaskRunAsSelf: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, runasself: *mut i16) -> ::windows::core::HRESULT,
    pub SetTaskRunAsSelf: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, runasself: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub TaskArguments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, task: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TaskArguments: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetTaskArguments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, task: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetTaskArguments: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub TaskUserTextArguments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, task: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TaskUserTextArguments: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetTaskUserTextArguments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, task: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetTaskUserTextArguments: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub TriggerDataCollectorSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TriggerDataCollectorSet: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetTriggerDataCollectorSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetTriggerDataCollectorSet: usize,
}
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IApiTracingDataCollector(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IApiTracingDataCollector {
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DataCollectorSet(&self) -> ::windows::core::Result<IDataCollectorSet> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.DataCollectorSet)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDataCollectorSet>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetDataCollectorSet<'a, Param0: ::windows::core::IntoParam<'a, IDataCollectorSet>>(&self, group: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetDataCollectorSet)(::core::mem::transmute_copy(self), group.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn DataCollectorType(&self) -> ::windows::core::Result<DataCollectorType> {
        let mut result__: DataCollectorType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.DataCollectorType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DataCollectorType>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FileName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.FileName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFileName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetFileName)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn FileNameFormat(&self) -> ::windows::core::Result<AutoPathFormat> {
        let mut result__: AutoPathFormat = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.FileNameFormat)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<AutoPathFormat>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetFileNameFormat(&self, format: AutoPathFormat) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetFileNameFormat)(::core::mem::transmute_copy(self), ::core::mem::transmute(format)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FileNameFormatPattern(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.FileNameFormatPattern)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFileNameFormatPattern<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, pattern: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetFileNameFormatPattern)(::core::mem::transmute_copy(self), pattern.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LatestOutputLocation(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.LatestOutputLocation)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLatestOutputLocation<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetLatestOutputLocation)(::core::mem::transmute_copy(self), path.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn LogAppend(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.LogAppend)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetLogAppend(&self, append: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetLogAppend)(::core::mem::transmute_copy(self), ::core::mem::transmute(append)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn LogCircular(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.LogCircular)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetLogCircular(&self, circular: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetLogCircular)(::core::mem::transmute_copy(self), ::core::mem::transmute(circular)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn LogOverwrite(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.LogOverwrite)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetLogOverwrite(&self, overwrite: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetLogOverwrite)(::core::mem::transmute_copy(self), ::core::mem::transmute(overwrite)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Name)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetName)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OutputLocation(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.OutputLocation)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn Index(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Index)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetIndex(&self, index: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetIndex)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Xml(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Xml)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn SetXml<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, xml: Param0) -> ::windows::core::Result<IValueMap> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.SetXml)(::core::mem::transmute_copy(self), xml.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IValueMap>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateOutputLocation(&self, latest: i16) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateOutputLocation)(::core::mem::transmute_copy(self), ::core::mem::transmute(latest), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn LogApiNamesOnly(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).LogApiNamesOnly)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetLogApiNamesOnly(&self, logapinames: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetLogApiNamesOnly)(::core::mem::transmute_copy(self), ::core::mem::transmute(logapinames)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn LogApisRecursively(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).LogApisRecursively)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetLogApisRecursively(&self, logrecursively: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetLogApisRecursively)(::core::mem::transmute_copy(self), ::core::mem::transmute(logrecursively)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExePath(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ExePath)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetExePath<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, exepath: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetExePath)(::core::mem::transmute_copy(self), exepath.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LogFilePath(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).LogFilePath)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLogFilePath<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, logfilepath: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetLogFilePath)(::core::mem::transmute_copy(self), logfilepath.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn IncludeModules(&self) -> ::windows::core::Result<*mut super::Com::SAFEARRAY> {
        let mut result__: *mut super::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).IncludeModules)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetIncludeModules(&self, includemodules: *const super::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetIncludeModules)(::core::mem::transmute_copy(self), ::core::mem::transmute(includemodules)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn IncludeApis(&self) -> ::windows::core::Result<*mut super::Com::SAFEARRAY> {
        let mut result__: *mut super::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).IncludeApis)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetIncludeApis(&self, includeapis: *const super::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetIncludeApis)(::core::mem::transmute_copy(self), ::core::mem::transmute(includeapis)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ExcludeApis(&self) -> ::windows::core::Result<*mut super::Com::SAFEARRAY> {
        let mut result__: *mut super::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ExcludeApis)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetExcludeApis(&self, excludeapis: *const super::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetExcludeApis)(::core::mem::transmute_copy(self), ::core::mem::transmute(excludeapis)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IApiTracingDataCollector> for ::windows::core::IUnknown {
    fn from(value: IApiTracingDataCollector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IApiTracingDataCollector> for ::windows::core::IUnknown {
    fn from(value: &IApiTracingDataCollector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IApiTracingDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IApiTracingDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IApiTracingDataCollector> for super::Com::IDispatch {
    fn from(value: IApiTracingDataCollector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IApiTracingDataCollector> for super::Com::IDispatch {
    fn from(value: &IApiTracingDataCollector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IApiTracingDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a IApiTracingDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IApiTracingDataCollector> for IDataCollector {
    fn from(value: IApiTracingDataCollector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IApiTracingDataCollector> for IDataCollector {
    fn from(value: &IApiTracingDataCollector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IDataCollector> for IApiTracingDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, IDataCollector> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IDataCollector> for &'a IApiTracingDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, IDataCollector> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IApiTracingDataCollector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IApiTracingDataCollector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IApiTracingDataCollector {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IApiTracingDataCollector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IApiTracingDataCollector").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IApiTracingDataCollector {
    type Vtable = IApiTracingDataCollector_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0383751a_098b_11d8_9414_505054503030);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IApiTracingDataCollector_Vtbl {
    pub base: IDataCollector_Vtbl,
    pub LogApiNamesOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, logapinames: *mut i16) -> ::windows::core::HRESULT,
    pub SetLogApiNamesOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, logapinames: i16) -> ::windows::core::HRESULT,
    pub LogApisRecursively: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, logrecursively: *mut i16) -> ::windows::core::HRESULT,
    pub SetLogApisRecursively: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, logrecursively: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ExePath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, exepath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ExePath: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetExePath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, exepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetExePath: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub LogFilePath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, logfilepath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LogFilePath: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetLogFilePath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, logfilepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetLogFilePath: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub IncludeModules: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, includemodules: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    IncludeModules: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetIncludeModules: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, includemodules: *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetIncludeModules: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub IncludeApis: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, includeapis: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    IncludeApis: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetIncludeApis: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, includeapis: *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetIncludeApis: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ExcludeApis: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, excludeapis: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ExcludeApis: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetExcludeApis: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, excludeapis: *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetExcludeApis: usize,
}
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IConfigurationDataCollector(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IConfigurationDataCollector {
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DataCollectorSet(&self) -> ::windows::core::Result<IDataCollectorSet> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.DataCollectorSet)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDataCollectorSet>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetDataCollectorSet<'a, Param0: ::windows::core::IntoParam<'a, IDataCollectorSet>>(&self, group: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetDataCollectorSet)(::core::mem::transmute_copy(self), group.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn DataCollectorType(&self) -> ::windows::core::Result<DataCollectorType> {
        let mut result__: DataCollectorType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.DataCollectorType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DataCollectorType>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FileName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.FileName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFileName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetFileName)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn FileNameFormat(&self) -> ::windows::core::Result<AutoPathFormat> {
        let mut result__: AutoPathFormat = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.FileNameFormat)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<AutoPathFormat>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetFileNameFormat(&self, format: AutoPathFormat) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetFileNameFormat)(::core::mem::transmute_copy(self), ::core::mem::transmute(format)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FileNameFormatPattern(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.FileNameFormatPattern)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFileNameFormatPattern<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, pattern: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetFileNameFormatPattern)(::core::mem::transmute_copy(self), pattern.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LatestOutputLocation(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.LatestOutputLocation)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLatestOutputLocation<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetLatestOutputLocation)(::core::mem::transmute_copy(self), path.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn LogAppend(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.LogAppend)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetLogAppend(&self, append: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetLogAppend)(::core::mem::transmute_copy(self), ::core::mem::transmute(append)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn LogCircular(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.LogCircular)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetLogCircular(&self, circular: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetLogCircular)(::core::mem::transmute_copy(self), ::core::mem::transmute(circular)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn LogOverwrite(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.LogOverwrite)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetLogOverwrite(&self, overwrite: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetLogOverwrite)(::core::mem::transmute_copy(self), ::core::mem::transmute(overwrite)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Name)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetName)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OutputLocation(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.OutputLocation)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn Index(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Index)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetIndex(&self, index: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetIndex)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Xml(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Xml)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn SetXml<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, xml: Param0) -> ::windows::core::Result<IValueMap> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.SetXml)(::core::mem::transmute_copy(self), xml.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IValueMap>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateOutputLocation(&self, latest: i16) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateOutputLocation)(::core::mem::transmute_copy(self), ::core::mem::transmute(latest), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn FileMaxCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).FileMaxCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetFileMaxCount(&self, count: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetFileMaxCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(count)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn FileMaxRecursiveDepth(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).FileMaxRecursiveDepth)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetFileMaxRecursiveDepth(&self, depth: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetFileMaxRecursiveDepth)(::core::mem::transmute_copy(self), ::core::mem::transmute(depth)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn FileMaxTotalSize(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).FileMaxTotalSize)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetFileMaxTotalSize(&self, size: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetFileMaxTotalSize)(::core::mem::transmute_copy(self), ::core::mem::transmute(size)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Files(&self) -> ::windows::core::Result<*mut super::Com::SAFEARRAY> {
        let mut result__: *mut super::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Files)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetFiles(&self, files: *const super::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetFiles)(::core::mem::transmute_copy(self), ::core::mem::transmute(files)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ManagementQueries(&self) -> ::windows::core::Result<*mut super::Com::SAFEARRAY> {
        let mut result__: *mut super::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ManagementQueries)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetManagementQueries(&self, queries: *const super::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetManagementQueries)(::core::mem::transmute_copy(self), ::core::mem::transmute(queries)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn QueryNetworkAdapters(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).QueryNetworkAdapters)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetQueryNetworkAdapters(&self, network: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetQueryNetworkAdapters)(::core::mem::transmute_copy(self), ::core::mem::transmute(network)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RegistryKeys(&self) -> ::windows::core::Result<*mut super::Com::SAFEARRAY> {
        let mut result__: *mut super::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).RegistryKeys)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetRegistryKeys(&self, query: *const super::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetRegistryKeys)(::core::mem::transmute_copy(self), ::core::mem::transmute(query)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn RegistryMaxRecursiveDepth(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).RegistryMaxRecursiveDepth)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetRegistryMaxRecursiveDepth(&self, depth: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetRegistryMaxRecursiveDepth)(::core::mem::transmute_copy(self), ::core::mem::transmute(depth)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SystemStateFile(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SystemStateFile)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSystemStateFile<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, filename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSystemStateFile)(::core::mem::transmute_copy(self), filename.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IConfigurationDataCollector> for ::windows::core::IUnknown {
    fn from(value: IConfigurationDataCollector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IConfigurationDataCollector> for ::windows::core::IUnknown {
    fn from(value: &IConfigurationDataCollector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IConfigurationDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IConfigurationDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IConfigurationDataCollector> for super::Com::IDispatch {
    fn from(value: IConfigurationDataCollector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IConfigurationDataCollector> for super::Com::IDispatch {
    fn from(value: &IConfigurationDataCollector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IConfigurationDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a IConfigurationDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IConfigurationDataCollector> for IDataCollector {
    fn from(value: IConfigurationDataCollector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IConfigurationDataCollector> for IDataCollector {
    fn from(value: &IConfigurationDataCollector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IDataCollector> for IConfigurationDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, IDataCollector> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IDataCollector> for &'a IConfigurationDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, IDataCollector> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IConfigurationDataCollector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IConfigurationDataCollector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IConfigurationDataCollector {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IConfigurationDataCollector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IConfigurationDataCollector").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IConfigurationDataCollector {
    type Vtable = IConfigurationDataCollector_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03837514_098b_11d8_9414_505054503030);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IConfigurationDataCollector_Vtbl {
    pub base: IDataCollector_Vtbl,
    pub FileMaxCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT,
    pub SetFileMaxCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: u32) -> ::windows::core::HRESULT,
    pub FileMaxRecursiveDepth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, depth: *mut u32) -> ::windows::core::HRESULT,
    pub SetFileMaxRecursiveDepth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, depth: u32) -> ::windows::core::HRESULT,
    pub FileMaxTotalSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, size: *mut u32) -> ::windows::core::HRESULT,
    pub SetFileMaxTotalSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, size: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Files: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, files: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Files: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetFiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, files: *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetFiles: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ManagementQueries: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, queries: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ManagementQueries: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetManagementQueries: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, queries: *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetManagementQueries: usize,
    pub QueryNetworkAdapters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, network: *mut i16) -> ::windows::core::HRESULT,
    pub SetQueryNetworkAdapters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, network: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub RegistryKeys: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, query: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RegistryKeys: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetRegistryKeys: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, query: *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetRegistryKeys: usize,
    pub RegistryMaxRecursiveDepth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, depth: *mut u32) -> ::windows::core::HRESULT,
    pub SetRegistryMaxRecursiveDepth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, depth: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SystemStateFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SystemStateFile: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSystemStateFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSystemStateFile: usize,
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[repr(transparent)]
pub struct ICounterItem(::windows::core::IUnknown);
impl ICounterItem {
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn Value(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Value)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetColor(&self, color: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetColor)(::core::mem::transmute_copy(self), ::core::mem::transmute(color)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn Color(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Color)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetWidth(&self, iwidth: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetWidth)(::core::mem::transmute_copy(self), ::core::mem::transmute(iwidth)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn Width(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Width)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetLineStyle(&self, ilinestyle: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetLineStyle)(::core::mem::transmute_copy(self), ::core::mem::transmute(ilinestyle)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn LineStyle(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).LineStyle)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetScaleFactor(&self, iscale: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetScaleFactor)(::core::mem::transmute_copy(self), ::core::mem::transmute(iscale)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn ScaleFactor(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ScaleFactor)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Path(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Path)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn GetValue(&self, value: *mut f64, status: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetValue)(::core::mem::transmute_copy(self), ::core::mem::transmute(value), ::core::mem::transmute(status)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn GetStatistics(&self, max: *mut f64, min: *mut f64, avg: *mut f64, status: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetStatistics)(::core::mem::transmute_copy(self), ::core::mem::transmute(max), ::core::mem::transmute(min), ::core::mem::transmute(avg), ::core::mem::transmute(status)).ok()
    }
}
impl ::core::convert::From<ICounterItem> for ::windows::core::IUnknown {
    fn from(value: ICounterItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICounterItem> for ::windows::core::IUnknown {
    fn from(value: &ICounterItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ICounterItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ICounterItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICounterItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICounterItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICounterItem {}
impl ::core::fmt::Debug for ICounterItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICounterItem").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICounterItem {
    type Vtable = ICounterItem_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x771a9520_ee28_11ce_941e_008029004347);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICounterItem_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdblvalue: *mut f64) -> ::windows::core::HRESULT,
    pub SetColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, color: u32) -> ::windows::core::HRESULT,
    pub Color: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcolor: *mut u32) -> ::windows::core::HRESULT,
    pub SetWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iwidth: i32) -> ::windows::core::HRESULT,
    pub Width: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pivalue: *mut i32) -> ::windows::core::HRESULT,
    pub SetLineStyle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ilinestyle: i32) -> ::windows::core::HRESULT,
    pub LineStyle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pivalue: *mut i32) -> ::windows::core::HRESULT,
    pub SetScaleFactor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iscale: i32) -> ::windows::core::HRESULT,
    pub ScaleFactor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pivalue: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Path: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstrvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Path: usize,
    pub GetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut f64, status: *mut i32) -> ::windows::core::HRESULT,
    pub GetStatistics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, max: *mut f64, min: *mut f64, avg: *mut f64, status: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[repr(transparent)]
pub struct ICounterItem2(::windows::core::IUnknown);
impl ICounterItem2 {
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn Value(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Value)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetColor(&self, color: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetColor)(::core::mem::transmute_copy(self), ::core::mem::transmute(color)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn Color(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Color)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetWidth(&self, iwidth: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetWidth)(::core::mem::transmute_copy(self), ::core::mem::transmute(iwidth)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn Width(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Width)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetLineStyle(&self, ilinestyle: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetLineStyle)(::core::mem::transmute_copy(self), ::core::mem::transmute(ilinestyle)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn LineStyle(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.LineStyle)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetScaleFactor(&self, iscale: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetScaleFactor)(::core::mem::transmute_copy(self), ::core::mem::transmute(iscale)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn ScaleFactor(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.ScaleFactor)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Path(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Path)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn GetValue(&self, value: *mut f64, status: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetValue)(::core::mem::transmute_copy(self), ::core::mem::transmute(value), ::core::mem::transmute(status)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn GetStatistics(&self, max: *mut f64, min: *mut f64, avg: *mut f64, status: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetStatistics)(::core::mem::transmute_copy(self), ::core::mem::transmute(max), ::core::mem::transmute(min), ::core::mem::transmute(avg), ::core::mem::transmute(status)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetSelected(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSelected)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn Selected(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Selected)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetVisible(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetVisible)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn Visible(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Visible)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetDataAt(&self, iindex: i32, iwhich: SysmonDataType) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDataAt)(::core::mem::transmute_copy(self), ::core::mem::transmute(iindex), ::core::mem::transmute(iwhich), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::VARIANT>(result__)
    }
}
impl ::core::convert::From<ICounterItem2> for ::windows::core::IUnknown {
    fn from(value: ICounterItem2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICounterItem2> for ::windows::core::IUnknown {
    fn from(value: &ICounterItem2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ICounterItem2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ICounterItem2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ICounterItem2> for ICounterItem {
    fn from(value: ICounterItem2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICounterItem2> for ICounterItem {
    fn from(value: &ICounterItem2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICounterItem> for ICounterItem2 {
    fn into_param(self) -> ::windows::core::Param<'a, ICounterItem> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICounterItem> for &'a ICounterItem2 {
    fn into_param(self) -> ::windows::core::Param<'a, ICounterItem> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICounterItem2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICounterItem2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICounterItem2 {}
impl ::core::fmt::Debug for ICounterItem2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICounterItem2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICounterItem2 {
    type Vtable = ICounterItem2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeefcd4e1_ea1c_4435_b7f4_e341ba03b4f9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICounterItem2_Vtbl {
    pub base: ICounterItem_Vtbl,
    pub SetSelected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT,
    pub Selected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub SetVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT,
    pub Visible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetDataAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iindex: i32, iwhich: SysmonDataType, pvariant: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetDataAt: usize,
}
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ICounters(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ICounters {
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Count)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self)._NewEnum)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Item<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, index: Param0) -> ::windows::core::Result<DICounterItem> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Item)(::core::mem::transmute_copy(self), index.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<DICounterItem>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Add<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, pathname: Param0) -> ::windows::core::Result<DICounterItem> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Add)(::core::mem::transmute_copy(self), pathname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<DICounterItem>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Remove<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, index: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Remove)(::core::mem::transmute_copy(self), index.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ICounters> for ::windows::core::IUnknown {
    fn from(value: ICounters) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ICounters> for ::windows::core::IUnknown {
    fn from(value: &ICounters) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ICounters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ICounters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ICounters> for super::Com::IDispatch {
    fn from(value: ICounters) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ICounters> for super::Com::IDispatch {
    fn from(value: &ICounters) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for ICounters {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a ICounters {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ICounters {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ICounters {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ICounters {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ICounters {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICounters").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ICounters {
    type Vtable = ICounters_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79167962_28fc_11cf_942f_008029004347);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ICounters_Vtbl {
    pub base: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plong: *mut i32) -> ::windows::core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppiunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppi: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Item: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pathname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppi: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    Add: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Remove: usize,
}
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IDataCollector(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IDataCollector {
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DataCollectorSet(&self) -> ::windows::core::Result<IDataCollectorSet> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).DataCollectorSet)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDataCollectorSet>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetDataCollectorSet<'a, Param0: ::windows::core::IntoParam<'a, IDataCollectorSet>>(&self, group: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDataCollectorSet)(::core::mem::transmute_copy(self), group.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn DataCollectorType(&self) -> ::windows::core::Result<DataCollectorType> {
        let mut result__: DataCollectorType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).DataCollectorType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DataCollectorType>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FileName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).FileName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFileName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetFileName)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn FileNameFormat(&self) -> ::windows::core::Result<AutoPathFormat> {
        let mut result__: AutoPathFormat = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).FileNameFormat)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<AutoPathFormat>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetFileNameFormat(&self, format: AutoPathFormat) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetFileNameFormat)(::core::mem::transmute_copy(self), ::core::mem::transmute(format)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FileNameFormatPattern(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).FileNameFormatPattern)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFileNameFormatPattern<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, pattern: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetFileNameFormatPattern)(::core::mem::transmute_copy(self), pattern.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LatestOutputLocation(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).LatestOutputLocation)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLatestOutputLocation<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetLatestOutputLocation)(::core::mem::transmute_copy(self), path.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn LogAppend(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).LogAppend)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetLogAppend(&self, append: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetLogAppend)(::core::mem::transmute_copy(self), ::core::mem::transmute(append)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn LogCircular(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).LogCircular)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetLogCircular(&self, circular: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetLogCircular)(::core::mem::transmute_copy(self), ::core::mem::transmute(circular)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn LogOverwrite(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).LogOverwrite)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetLogOverwrite(&self, overwrite: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetLogOverwrite)(::core::mem::transmute_copy(self), ::core::mem::transmute(overwrite)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Name)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetName)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OutputLocation(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).OutputLocation)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn Index(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Index)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetIndex(&self, index: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetIndex)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Xml(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Xml)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn SetXml<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, xml: Param0) -> ::windows::core::Result<IValueMap> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SetXml)(::core::mem::transmute_copy(self), xml.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IValueMap>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateOutputLocation(&self, latest: i16) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateOutputLocation)(::core::mem::transmute_copy(self), ::core::mem::transmute(latest), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IDataCollector> for ::windows::core::IUnknown {
    fn from(value: IDataCollector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IDataCollector> for ::windows::core::IUnknown {
    fn from(value: &IDataCollector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IDataCollector> for super::Com::IDispatch {
    fn from(value: IDataCollector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IDataCollector> for super::Com::IDispatch {
    fn from(value: &IDataCollector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a IDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IDataCollector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IDataCollector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IDataCollector {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IDataCollector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDataCollector").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IDataCollector {
    type Vtable = IDataCollector_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x038374ff_098b_11d8_9414_505054503030);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IDataCollector_Vtbl {
    pub base: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub DataCollectorSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, group: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DataCollectorSet: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetDataCollectorSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, group: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetDataCollectorSet: usize,
    pub DataCollectorType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: *mut DataCollectorType) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub FileName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FileName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetFileName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetFileName: usize,
    pub FileNameFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: *mut AutoPathFormat) -> ::windows::core::HRESULT,
    pub SetFileNameFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: AutoPathFormat) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub FileNameFormatPattern: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pattern: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FileNameFormatPattern: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetFileNameFormatPattern: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pattern: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetFileNameFormatPattern: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub LatestOutputLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LatestOutputLocation: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetLatestOutputLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetLatestOutputLocation: usize,
    pub LogAppend: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, append: *mut i16) -> ::windows::core::HRESULT,
    pub SetLogAppend: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, append: i16) -> ::windows::core::HRESULT,
    pub LogCircular: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, circular: *mut i16) -> ::windows::core::HRESULT,
    pub SetLogCircular: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, circular: i16) -> ::windows::core::HRESULT,
    pub LogOverwrite: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, overwrite: *mut i16) -> ::windows::core::HRESULT,
    pub SetLogOverwrite: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, overwrite: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OutputLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OutputLocation: usize,
    pub Index: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: *mut i32) -> ::windows::core::HRESULT,
    pub SetIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Xml: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xml: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Xml: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub SetXml: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xml: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, validation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    SetXml: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateOutputLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, latest: i16, location: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateOutputLocation: usize,
}
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IDataCollectorCollection(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IDataCollectorCollection {
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Count)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Item<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, index: Param0) -> ::windows::core::Result<IDataCollector> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Item)(::core::mem::transmute_copy(self), index.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IDataCollector>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self)._NewEnum)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Add<'a, Param0: ::windows::core::IntoParam<'a, IDataCollector>>(&self, collector: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Add)(::core::mem::transmute_copy(self), collector.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Remove<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, collector: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Remove)(::core::mem::transmute_copy(self), collector.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn Clear(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Clear)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddRange<'a, Param0: ::windows::core::IntoParam<'a, IDataCollectorCollection>>(&self, collectors: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddRange)(::core::mem::transmute_copy(self), collectors.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn CreateDataCollectorFromXml<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrxml: Param0, pvalidation: *mut ::core::option::Option<IValueMap>, pcollector: *mut ::core::option::Option<IDataCollector>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CreateDataCollectorFromXml)(::core::mem::transmute_copy(self), bstrxml.into_param().abi(), ::core::mem::transmute(pvalidation), ::core::mem::transmute(pcollector)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateDataCollector(&self, r#type: DataCollectorType) -> ::windows::core::Result<IDataCollector> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateDataCollector)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), ::core::mem::transmute(&mut result__)).from_abi::<IDataCollector>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IDataCollectorCollection> for ::windows::core::IUnknown {
    fn from(value: IDataCollectorCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IDataCollectorCollection> for ::windows::core::IUnknown {
    fn from(value: &IDataCollectorCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDataCollectorCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDataCollectorCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IDataCollectorCollection> for super::Com::IDispatch {
    fn from(value: IDataCollectorCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IDataCollectorCollection> for super::Com::IDispatch {
    fn from(value: &IDataCollectorCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IDataCollectorCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a IDataCollectorCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IDataCollectorCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IDataCollectorCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IDataCollectorCollection {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IDataCollectorCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDataCollectorCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IDataCollectorCollection {
    type Vtable = IDataCollectorCollection_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03837502_098b_11d8_9414_505054503030);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IDataCollectorCollection_Vtbl {
    pub base: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, collector: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, collector: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, collector: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Remove: usize,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub AddRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, collectors: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddRange: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub CreateDataCollectorFromXml: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrxml: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvalidation: *mut ::windows::core::RawPtr, pcollector: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    CreateDataCollectorFromXml: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateDataCollector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: DataCollectorType, collector: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateDataCollector: usize,
}
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IDataCollectorSet(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IDataCollectorSet {
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DataCollectors(&self) -> ::windows::core::Result<IDataCollectorCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).DataCollectors)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDataCollectorCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn Duration(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Duration)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetDuration(&self, seconds: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDuration)(::core::mem::transmute_copy(self), ::core::mem::transmute(seconds)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Description)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, description: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDescription)(::core::mem::transmute_copy(self), description.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DescriptionUnresolved(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).DescriptionUnresolved)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DisplayName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).DisplayName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDisplayName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, displayname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDisplayName)(::core::mem::transmute_copy(self), displayname.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DisplayNameUnresolved(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).DisplayNameUnresolved)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Keywords(&self) -> ::windows::core::Result<*mut super::Com::SAFEARRAY> {
        let mut result__: *mut super::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Keywords)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetKeywords(&self, keywords: *const super::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetKeywords)(::core::mem::transmute_copy(self), ::core::mem::transmute(keywords)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LatestOutputLocation(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).LatestOutputLocation)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLatestOutputLocation<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetLatestOutputLocation)(::core::mem::transmute_copy(self), path.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Name)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OutputLocation(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).OutputLocation)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RootPath(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).RootPath)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRootPath<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, folder: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetRootPath)(::core::mem::transmute_copy(self), folder.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn Segment(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Segment)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetSegment(&self, segment: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSegment)(::core::mem::transmute_copy(self), ::core::mem::transmute(segment)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SegmentMaxDuration(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SegmentMaxDuration)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetSegmentMaxDuration(&self, seconds: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSegmentMaxDuration)(::core::mem::transmute_copy(self), ::core::mem::transmute(seconds)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SegmentMaxSize(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SegmentMaxSize)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetSegmentMaxSize(&self, size: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSegmentMaxSize)(::core::mem::transmute_copy(self), ::core::mem::transmute(size)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SerialNumber(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SerialNumber)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetSerialNumber(&self, index: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSerialNumber)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Server(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Server)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn Status(&self) -> ::windows::core::Result<DataCollectorSetStatus> {
        let mut result__: DataCollectorSetStatus = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Status)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DataCollectorSetStatus>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Subdirectory(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Subdirectory)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSubdirectory<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, folder: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSubdirectory)(::core::mem::transmute_copy(self), folder.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SubdirectoryFormat(&self) -> ::windows::core::Result<AutoPathFormat> {
        let mut result__: AutoPathFormat = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SubdirectoryFormat)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<AutoPathFormat>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetSubdirectoryFormat(&self, format: AutoPathFormat) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSubdirectoryFormat)(::core::mem::transmute_copy(self), ::core::mem::transmute(format)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SubdirectoryFormatPattern(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SubdirectoryFormatPattern)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSubdirectoryFormatPattern<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, pattern: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSubdirectoryFormatPattern)(::core::mem::transmute_copy(self), pattern.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Task(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Task)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetTask<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, task: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTask)(::core::mem::transmute_copy(self), task.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn TaskRunAsSelf(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).TaskRunAsSelf)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetTaskRunAsSelf(&self, runasself: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTaskRunAsSelf)(::core::mem::transmute_copy(self), ::core::mem::transmute(runasself)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TaskArguments(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).TaskArguments)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetTaskArguments<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, task: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTaskArguments)(::core::mem::transmute_copy(self), task.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TaskUserTextArguments(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).TaskUserTextArguments)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetTaskUserTextArguments<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, usertext: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTaskUserTextArguments)(::core::mem::transmute_copy(self), usertext.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Schedules(&self) -> ::windows::core::Result<IScheduleCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Schedules)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IScheduleCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SchedulesEnabled(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SchedulesEnabled)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetSchedulesEnabled(&self, enabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSchedulesEnabled)(::core::mem::transmute_copy(self), ::core::mem::transmute(enabled)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UserAccount(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).UserAccount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Xml(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Xml)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Security(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Security)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSecurity<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrsecurity: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSecurity)(::core::mem::transmute_copy(self), bstrsecurity.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn StopOnCompletion(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).StopOnCompletion)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetStopOnCompletion(&self, stop: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetStopOnCompletion)(::core::mem::transmute_copy(self), ::core::mem::transmute(stop)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DataManager(&self) -> ::windows::core::Result<IDataManager> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).DataManager)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDataManager>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCredentials<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, user: Param0, password: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCredentials)(::core::mem::transmute_copy(self), user.into_param().abi(), password.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Query<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0, server: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Query)(::core::mem::transmute_copy(self), name.into_param().abi(), server.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Commit<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0, server: Param1, mode: CommitMode) -> ::windows::core::Result<IValueMap> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Commit)(::core::mem::transmute_copy(self), name.into_param().abi(), server.into_param().abi(), ::core::mem::transmute(mode), ::core::mem::transmute(&mut result__)).from_abi::<IValueMap>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Delete)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn Start(&self, synchronous: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Start)(::core::mem::transmute_copy(self), ::core::mem::transmute(synchronous)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn Stop(&self, synchronous: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Stop)(::core::mem::transmute_copy(self), ::core::mem::transmute(synchronous)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn SetXml<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, xml: Param0) -> ::windows::core::Result<IValueMap> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SetXml)(::core::mem::transmute_copy(self), xml.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IValueMap>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetValue<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, key: Param0, value: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetValue)(::core::mem::transmute_copy(self), key.into_param().abi(), value.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetValue<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, key: Param0) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetValue)(::core::mem::transmute_copy(self), key.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IDataCollectorSet> for ::windows::core::IUnknown {
    fn from(value: IDataCollectorSet) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IDataCollectorSet> for ::windows::core::IUnknown {
    fn from(value: &IDataCollectorSet) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDataCollectorSet {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDataCollectorSet {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IDataCollectorSet> for super::Com::IDispatch {
    fn from(value: IDataCollectorSet) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IDataCollectorSet> for super::Com::IDispatch {
    fn from(value: &IDataCollectorSet) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IDataCollectorSet {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a IDataCollectorSet {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IDataCollectorSet {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IDataCollectorSet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IDataCollectorSet {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IDataCollectorSet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDataCollectorSet").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IDataCollectorSet {
    type Vtable = IDataCollectorSet_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03837520_098b_11d8_9414_505054503030);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IDataCollectorSet_Vtbl {
    pub base: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub DataCollectors: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, collectors: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DataCollectors: usize,
    pub Duration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, seconds: *mut u32) -> ::windows::core::HRESULT,
    pub SetDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, seconds: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Description: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDescription: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DescriptionUnresolved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, descr: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DescriptionUnresolved: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, displayname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DisplayName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, displayname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDisplayName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DisplayNameUnresolved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DisplayNameUnresolved: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Keywords: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, keywords: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Keywords: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetKeywords: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, keywords: *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetKeywords: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub LatestOutputLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LatestOutputLocation: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetLatestOutputLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetLatestOutputLocation: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OutputLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OutputLocation: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RootPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, folder: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RootPath: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetRootPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, folder: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetRootPath: usize,
    pub Segment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, segment: *mut i16) -> ::windows::core::HRESULT,
    pub SetSegment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, segment: i16) -> ::windows::core::HRESULT,
    pub SegmentMaxDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, seconds: *mut u32) -> ::windows::core::HRESULT,
    pub SetSegmentMaxDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, seconds: u32) -> ::windows::core::HRESULT,
    pub SegmentMaxSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, size: *mut u32) -> ::windows::core::HRESULT,
    pub SetSegmentMaxSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, size: u32) -> ::windows::core::HRESULT,
    pub SerialNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: *mut u32) -> ::windows::core::HRESULT,
    pub SetSerialNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Server: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, server: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Server: usize,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: *mut DataCollectorSetStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Subdirectory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, folder: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Subdirectory: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSubdirectory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, folder: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSubdirectory: usize,
    pub SubdirectoryFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: *mut AutoPathFormat) -> ::windows::core::HRESULT,
    pub SetSubdirectoryFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: AutoPathFormat) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SubdirectoryFormatPattern: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pattern: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SubdirectoryFormatPattern: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSubdirectoryFormatPattern: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pattern: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSubdirectoryFormatPattern: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Task: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, task: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Task: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetTask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, task: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetTask: usize,
    pub TaskRunAsSelf: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, runasself: *mut i16) -> ::windows::core::HRESULT,
    pub SetTaskRunAsSelf: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, runasself: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub TaskArguments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, task: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TaskArguments: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetTaskArguments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, task: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetTaskArguments: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub TaskUserTextArguments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usertext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TaskUserTextArguments: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetTaskUserTextArguments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usertext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetTaskUserTextArguments: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Schedules: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppschedules: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Schedules: usize,
    pub SchedulesEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: *mut i16) -> ::windows::core::HRESULT,
    pub SetSchedulesEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub UserAccount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UserAccount: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Xml: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xml: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Xml: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Security: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrsecurity: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Security: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSecurity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrsecurity: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSecurity: usize,
    pub StopOnCompletion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stop: *mut i16) -> ::windows::core::HRESULT,
    pub SetStopOnCompletion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stop: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub DataManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, datamanager: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DataManager: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetCredentials: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, password: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetCredentials: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Query: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, server: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Query: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub Commit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, server: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, mode: CommitMode, validation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    Commit: usize,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, synchronous: i16) -> ::windows::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, synchronous: i16) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub SetXml: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xml: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, validation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    SetXml: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetValue: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetValue: usize,
}
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IDataCollectorSetCollection(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IDataCollectorSetCollection {
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Count)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Item<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, index: Param0) -> ::windows::core::Result<IDataCollectorSet> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Item)(::core::mem::transmute_copy(self), index.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IDataCollectorSet>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self)._NewEnum)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Add<'a, Param0: ::windows::core::IntoParam<'a, IDataCollectorSet>>(&self, set: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Add)(::core::mem::transmute_copy(self), set.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Remove<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, set: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Remove)(::core::mem::transmute_copy(self), set.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn Clear(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Clear)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddRange<'a, Param0: ::windows::core::IntoParam<'a, IDataCollectorSetCollection>>(&self, sets: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddRange)(::core::mem::transmute_copy(self), sets.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDataCollectorSets<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, server: Param0, filter: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDataCollectorSets)(::core::mem::transmute_copy(self), server.into_param().abi(), filter.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IDataCollectorSetCollection> for ::windows::core::IUnknown {
    fn from(value: IDataCollectorSetCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IDataCollectorSetCollection> for ::windows::core::IUnknown {
    fn from(value: &IDataCollectorSetCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDataCollectorSetCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDataCollectorSetCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IDataCollectorSetCollection> for super::Com::IDispatch {
    fn from(value: IDataCollectorSetCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IDataCollectorSetCollection> for super::Com::IDispatch {
    fn from(value: &IDataCollectorSetCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IDataCollectorSetCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a IDataCollectorSetCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IDataCollectorSetCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IDataCollectorSetCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IDataCollectorSetCollection {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IDataCollectorSetCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDataCollectorSetCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IDataCollectorSetCollection {
    type Vtable = IDataCollectorSetCollection_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03837524_098b_11d8_9414_505054503030);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IDataCollectorSetCollection_Vtbl {
    pub base: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, set: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, set: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, set: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Remove: usize,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub AddRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sets: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddRange: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDataCollectorSets: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, server: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, filter: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDataCollectorSets: usize,
}
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IDataManager(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IDataManager {
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn Enabled(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Enabled)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetEnabled(&self, fenabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetEnabled)(::core::mem::transmute_copy(self), ::core::mem::transmute(fenabled)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn CheckBeforeRunning(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CheckBeforeRunning)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetCheckBeforeRunning(&self, fcheck: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCheckBeforeRunning)(::core::mem::transmute_copy(self), ::core::mem::transmute(fcheck)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn MinFreeDisk(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).MinFreeDisk)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetMinFreeDisk(&self, minfreedisk: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMinFreeDisk)(::core::mem::transmute_copy(self), ::core::mem::transmute(minfreedisk)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn MaxSize(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).MaxSize)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetMaxSize(&self, ulmaxsize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMaxSize)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulmaxsize)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn MaxFolderCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).MaxFolderCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetMaxFolderCount(&self, ulmaxfoldercount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMaxFolderCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulmaxfoldercount)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn ResourcePolicy(&self) -> ::windows::core::Result<ResourcePolicy> {
        let mut result__: ResourcePolicy = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ResourcePolicy)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ResourcePolicy>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetResourcePolicy(&self, policy: ResourcePolicy) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetResourcePolicy)(::core::mem::transmute_copy(self), ::core::mem::transmute(policy)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn FolderActions(&self) -> ::windows::core::Result<IFolderActionCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).FolderActions)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFolderActionCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReportSchema(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ReportSchema)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetReportSchema<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, reportschema: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetReportSchema)(::core::mem::transmute_copy(self), reportschema.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReportFileName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ReportFileName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetReportFileName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, pbstrfilename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetReportFileName)(::core::mem::transmute_copy(self), pbstrfilename.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RuleTargetFileName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).RuleTargetFileName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRuleTargetFileName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, filename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetRuleTargetFileName)(::core::mem::transmute_copy(self), filename.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EventsFileName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).EventsFileName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEventsFileName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, pbstrfilename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetEventsFileName)(::core::mem::transmute_copy(self), pbstrfilename.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Rules(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Rules)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRules<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrxml: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetRules)(::core::mem::transmute_copy(self), bstrxml.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Run<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, steps: DataManagerSteps, bstrfolder: Param1) -> ::windows::core::Result<IValueMap> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Run)(::core::mem::transmute_copy(self), ::core::mem::transmute(steps), bstrfolder.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IValueMap>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Extract<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, cabfilename: Param0, destinationpath: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Extract)(::core::mem::transmute_copy(self), cabfilename.into_param().abi(), destinationpath.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IDataManager> for ::windows::core::IUnknown {
    fn from(value: IDataManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IDataManager> for ::windows::core::IUnknown {
    fn from(value: &IDataManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDataManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDataManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IDataManager> for super::Com::IDispatch {
    fn from(value: IDataManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IDataManager> for super::Com::IDispatch {
    fn from(value: &IDataManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IDataManager {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a IDataManager {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IDataManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IDataManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IDataManager {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IDataManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDataManager").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IDataManager {
    type Vtable = IDataManager_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03837541_098b_11d8_9414_505054503030);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IDataManager_Vtbl {
    pub base: super::Com::IDispatch_Vtbl,
    pub Enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfenabled: *mut i16) -> ::windows::core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fenabled: i16) -> ::windows::core::HRESULT,
    pub CheckBeforeRunning: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfcheck: *mut i16) -> ::windows::core::HRESULT,
    pub SetCheckBeforeRunning: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fcheck: i16) -> ::windows::core::HRESULT,
    pub MinFreeDisk: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, minfreedisk: *mut u32) -> ::windows::core::HRESULT,
    pub SetMinFreeDisk: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, minfreedisk: u32) -> ::windows::core::HRESULT,
    pub MaxSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pulmaxsize: *mut u32) -> ::windows::core::HRESULT,
    pub SetMaxSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulmaxsize: u32) -> ::windows::core::HRESULT,
    pub MaxFolderCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pulmaxfoldercount: *mut u32) -> ::windows::core::HRESULT,
    pub SetMaxFolderCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulmaxfoldercount: u32) -> ::windows::core::HRESULT,
    pub ResourcePolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppolicy: *mut ResourcePolicy) -> ::windows::core::HRESULT,
    pub SetResourcePolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, policy: ResourcePolicy) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub FolderActions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, actions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    FolderActions: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ReportSchema: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reportschema: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ReportSchema: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetReportSchema: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reportschema: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetReportSchema: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ReportFileName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrfilename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ReportFileName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetReportFileName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetReportFileName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RuleTargetFileName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RuleTargetFileName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetRuleTargetFileName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetRuleTargetFileName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub EventsFileName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrfilename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EventsFileName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEventsFileName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEventsFileName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Rules: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrxml: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Rules: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetRules: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrxml: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetRules: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub Run: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, steps: DataManagerSteps, bstrfolder: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, errors: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    Run: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Extract: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cabfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, destinationpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Extract: usize,
}
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFolderAction(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFolderAction {
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn Age(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Age)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetAge(&self, ulage: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAge)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulage)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn Size(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Size)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetSize(&self, ulage: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSize)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulage)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn Actions(&self) -> ::windows::core::Result<FolderActionSteps> {
        let mut result__: FolderActionSteps = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Actions)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FolderActionSteps>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetActions(&self, steps: FolderActionSteps) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetActions)(::core::mem::transmute_copy(self), ::core::mem::transmute(steps)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SendCabTo(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SendCabTo)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSendCabTo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrdestination: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSendCabTo)(::core::mem::transmute_copy(self), bstrdestination.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFolderAction> for ::windows::core::IUnknown {
    fn from(value: IFolderAction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFolderAction> for ::windows::core::IUnknown {
    fn from(value: &IFolderAction) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFolderAction {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFolderAction {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFolderAction> for super::Com::IDispatch {
    fn from(value: IFolderAction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFolderAction> for super::Com::IDispatch {
    fn from(value: &IFolderAction) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IFolderAction {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a IFolderAction {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFolderAction {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFolderAction {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFolderAction {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFolderAction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFolderAction").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFolderAction {
    type Vtable = IFolderAction_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03837543_098b_11d8_9414_505054503030);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFolderAction_Vtbl {
    pub base: super::Com::IDispatch_Vtbl,
    pub Age: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pulage: *mut u32) -> ::windows::core::HRESULT,
    pub SetAge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulage: u32) -> ::windows::core::HRESULT,
    pub Size: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pulage: *mut u32) -> ::windows::core::HRESULT,
    pub SetSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulage: u32) -> ::windows::core::HRESULT,
    pub Actions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, steps: *mut FolderActionSteps) -> ::windows::core::HRESULT,
    pub SetActions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, steps: FolderActionSteps) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SendCabTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdestination: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SendCabTo: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSendCabTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdestination: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSendCabTo: usize,
}
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFolderActionCollection(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFolderActionCollection {
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn Count(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Count)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Item<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, index: Param0) -> ::windows::core::Result<IFolderAction> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Item)(::core::mem::transmute_copy(self), index.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IFolderAction>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self)._NewEnum)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Add<'a, Param0: ::windows::core::IntoParam<'a, IFolderAction>>(&self, action: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Add)(::core::mem::transmute_copy(self), action.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Remove<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, index: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Remove)(::core::mem::transmute_copy(self), index.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn Clear(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Clear)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddRange<'a, Param0: ::windows::core::IntoParam<'a, IFolderActionCollection>>(&self, actions: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddRange)(::core::mem::transmute_copy(self), actions.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateFolderAction(&self) -> ::windows::core::Result<IFolderAction> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateFolderAction)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFolderAction>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFolderActionCollection> for ::windows::core::IUnknown {
    fn from(value: IFolderActionCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFolderActionCollection> for ::windows::core::IUnknown {
    fn from(value: &IFolderActionCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFolderActionCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFolderActionCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFolderActionCollection> for super::Com::IDispatch {
    fn from(value: IFolderActionCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFolderActionCollection> for super::Com::IDispatch {
    fn from(value: &IFolderActionCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IFolderActionCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a IFolderActionCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFolderActionCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFolderActionCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFolderActionCollection {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFolderActionCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFolderActionCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFolderActionCollection {
    type Vtable = IFolderActionCollection_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03837544_098b_11d8_9414_505054503030);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFolderActionCollection_Vtbl {
    pub base: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, action: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#enum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, action: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Remove: usize,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub AddRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, actions: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddRange: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateFolderAction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, folderaction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateFolderAction: usize,
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[repr(transparent)]
pub struct ILogFileItem(::windows::core::IUnknown);
impl ILogFileItem {
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Path(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Path)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
impl ::core::convert::From<ILogFileItem> for ::windows::core::IUnknown {
    fn from(value: ILogFileItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILogFileItem> for ::windows::core::IUnknown {
    fn from(value: &ILogFileItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ILogFileItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ILogFileItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ILogFileItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ILogFileItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ILogFileItem {}
impl ::core::fmt::Debug for ILogFileItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ILogFileItem").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ILogFileItem {
    type Vtable = ILogFileItem_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd6b518dd_05c7_418a_89e6_4f9ce8c6841e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILogFileItem_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Path: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstrvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Path: usize,
}
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ILogFiles(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ILogFiles {
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Count)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self)._NewEnum)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Item<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, index: Param0) -> ::windows::core::Result<DILogFileItem> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Item)(::core::mem::transmute_copy(self), index.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<DILogFileItem>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Add<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, pathname: Param0) -> ::windows::core::Result<DILogFileItem> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Add)(::core::mem::transmute_copy(self), pathname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<DILogFileItem>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Remove<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, index: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Remove)(::core::mem::transmute_copy(self), index.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ILogFiles> for ::windows::core::IUnknown {
    fn from(value: ILogFiles) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ILogFiles> for ::windows::core::IUnknown {
    fn from(value: &ILogFiles) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ILogFiles {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ILogFiles {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ILogFiles> for super::Com::IDispatch {
    fn from(value: ILogFiles) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ILogFiles> for super::Com::IDispatch {
    fn from(value: &ILogFiles) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for ILogFiles {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a ILogFiles {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ILogFiles {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ILogFiles {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ILogFiles {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ILogFiles {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ILogFiles").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ILogFiles {
    type Vtable = ILogFiles_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6a2a97e6_6851_41ea_87ad_2a8225335865);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ILogFiles_Vtbl {
    pub base: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plong: *mut i32) -> ::windows::core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppiunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppi: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Item: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pathname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppi: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    Add: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Remove: usize,
}
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IPerformanceCounterDataCollector(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IPerformanceCounterDataCollector {
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DataCollectorSet(&self) -> ::windows::core::Result<IDataCollectorSet> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.DataCollectorSet)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDataCollectorSet>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetDataCollectorSet<'a, Param0: ::windows::core::IntoParam<'a, IDataCollectorSet>>(&self, group: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetDataCollectorSet)(::core::mem::transmute_copy(self), group.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn DataCollectorType(&self) -> ::windows::core::Result<DataCollectorType> {
        let mut result__: DataCollectorType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.DataCollectorType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DataCollectorType>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FileName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.FileName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFileName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetFileName)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn FileNameFormat(&self) -> ::windows::core::Result<AutoPathFormat> {
        let mut result__: AutoPathFormat = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.FileNameFormat)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<AutoPathFormat>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetFileNameFormat(&self, format: AutoPathFormat) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetFileNameFormat)(::core::mem::transmute_copy(self), ::core::mem::transmute(format)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FileNameFormatPattern(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.FileNameFormatPattern)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFileNameFormatPattern<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, pattern: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetFileNameFormatPattern)(::core::mem::transmute_copy(self), pattern.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LatestOutputLocation(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.LatestOutputLocation)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLatestOutputLocation<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetLatestOutputLocation)(::core::mem::transmute_copy(self), path.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn LogAppend(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.LogAppend)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetLogAppend(&self, append: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetLogAppend)(::core::mem::transmute_copy(self), ::core::mem::transmute(append)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn LogCircular(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.LogCircular)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetLogCircular(&self, circular: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetLogCircular)(::core::mem::transmute_copy(self), ::core::mem::transmute(circular)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn LogOverwrite(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.LogOverwrite)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetLogOverwrite(&self, overwrite: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetLogOverwrite)(::core::mem::transmute_copy(self), ::core::mem::transmute(overwrite)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Name)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetName)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OutputLocation(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.OutputLocation)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn Index(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Index)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetIndex(&self, index: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetIndex)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Xml(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Xml)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn SetXml<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, xml: Param0) -> ::windows::core::Result<IValueMap> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.SetXml)(::core::mem::transmute_copy(self), xml.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IValueMap>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateOutputLocation(&self, latest: i16) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateOutputLocation)(::core::mem::transmute_copy(self), ::core::mem::transmute(latest), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DataSourceName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).DataSourceName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDataSourceName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, dsn: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDataSourceName)(::core::mem::transmute_copy(self), dsn.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PerformanceCounters(&self) -> ::windows::core::Result<*mut super::Com::SAFEARRAY> {
        let mut result__: *mut super::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).PerformanceCounters)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetPerformanceCounters(&self, counters: *const super::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPerformanceCounters)(::core::mem::transmute_copy(self), ::core::mem::transmute(counters)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn LogFileFormat(&self) -> ::windows::core::Result<FileFormat> {
        let mut result__: FileFormat = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).LogFileFormat)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FileFormat>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetLogFileFormat(&self, format: FileFormat) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetLogFileFormat)(::core::mem::transmute_copy(self), ::core::mem::transmute(format)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SampleInterval(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SampleInterval)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetSampleInterval(&self, interval: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSampleInterval)(::core::mem::transmute_copy(self), ::core::mem::transmute(interval)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SegmentMaxRecords(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SegmentMaxRecords)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetSegmentMaxRecords(&self, records: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSegmentMaxRecords)(::core::mem::transmute_copy(self), ::core::mem::transmute(records)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IPerformanceCounterDataCollector> for ::windows::core::IUnknown {
    fn from(value: IPerformanceCounterDataCollector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IPerformanceCounterDataCollector> for ::windows::core::IUnknown {
    fn from(value: &IPerformanceCounterDataCollector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPerformanceCounterDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPerformanceCounterDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IPerformanceCounterDataCollector> for super::Com::IDispatch {
    fn from(value: IPerformanceCounterDataCollector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IPerformanceCounterDataCollector> for super::Com::IDispatch {
    fn from(value: &IPerformanceCounterDataCollector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IPerformanceCounterDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a IPerformanceCounterDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IPerformanceCounterDataCollector> for IDataCollector {
    fn from(value: IPerformanceCounterDataCollector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IPerformanceCounterDataCollector> for IDataCollector {
    fn from(value: &IPerformanceCounterDataCollector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IDataCollector> for IPerformanceCounterDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, IDataCollector> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IDataCollector> for &'a IPerformanceCounterDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, IDataCollector> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IPerformanceCounterDataCollector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IPerformanceCounterDataCollector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IPerformanceCounterDataCollector {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IPerformanceCounterDataCollector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPerformanceCounterDataCollector").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IPerformanceCounterDataCollector {
    type Vtable = IPerformanceCounterDataCollector_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03837506_098b_11d8_9414_505054503030);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPerformanceCounterDataCollector_Vtbl {
    pub base: IDataCollector_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub DataSourceName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dsn: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DataSourceName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDataSourceName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dsn: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDataSourceName: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PerformanceCounters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, counters: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PerformanceCounters: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetPerformanceCounters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, counters: *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetPerformanceCounters: usize,
    pub LogFileFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: *mut FileFormat) -> ::windows::core::HRESULT,
    pub SetLogFileFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: FileFormat) -> ::windows::core::HRESULT,
    pub SampleInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interval: *mut u32) -> ::windows::core::HRESULT,
    pub SetSampleInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interval: u32) -> ::windows::core::HRESULT,
    pub SegmentMaxRecords: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, records: *mut u32) -> ::windows::core::HRESULT,
    pub SetSegmentMaxRecords: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, records: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISchedule(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISchedule {
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn StartDate(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).StartDate)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetStartDate<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, start: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetStartDate)(::core::mem::transmute_copy(self), start.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn EndDate(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).EndDate)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetEndDate<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, end: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetEndDate)(::core::mem::transmute_copy(self), end.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn StartTime(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).StartTime)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetStartTime<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, start: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetStartTime)(::core::mem::transmute_copy(self), start.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn Days(&self) -> ::windows::core::Result<WeekDays> {
        let mut result__: WeekDays = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Days)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<WeekDays>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetDays(&self, days: WeekDays) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDays)(::core::mem::transmute_copy(self), ::core::mem::transmute(days)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISchedule> for ::windows::core::IUnknown {
    fn from(value: ISchedule) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISchedule> for ::windows::core::IUnknown {
    fn from(value: &ISchedule) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISchedule {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISchedule {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISchedule> for super::Com::IDispatch {
    fn from(value: ISchedule) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISchedule> for super::Com::IDispatch {
    fn from(value: &ISchedule) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for ISchedule {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a ISchedule {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISchedule {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISchedule {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISchedule {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISchedule {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISchedule").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISchedule {
    type Vtable = ISchedule_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0383753a_098b_11d8_9414_505054503030);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISchedule_Vtbl {
    pub base: super::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub StartDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, start: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    StartDate: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetStartDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, start: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetStartDate: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub EndDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, end: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    EndDate: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetEndDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, end: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetEndDate: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub StartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, start: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    StartTime: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetStartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, start: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetStartTime: usize,
    pub Days: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, days: *mut WeekDays) -> ::windows::core::HRESULT,
    pub SetDays: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, days: WeekDays) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IScheduleCollection(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IScheduleCollection {
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Count)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Item<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, index: Param0) -> ::windows::core::Result<ISchedule> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Item)(::core::mem::transmute_copy(self), index.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<ISchedule>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self)._NewEnum)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Add<'a, Param0: ::windows::core::IntoParam<'a, ISchedule>>(&self, pschedule: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Add)(::core::mem::transmute_copy(self), pschedule.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Remove<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, vschedule: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Remove)(::core::mem::transmute_copy(self), vschedule.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn Clear(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Clear)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddRange<'a, Param0: ::windows::core::IntoParam<'a, IScheduleCollection>>(&self, pschedules: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddRange)(::core::mem::transmute_copy(self), pschedules.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateSchedule(&self) -> ::windows::core::Result<ISchedule> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateSchedule)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ISchedule>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IScheduleCollection> for ::windows::core::IUnknown {
    fn from(value: IScheduleCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IScheduleCollection> for ::windows::core::IUnknown {
    fn from(value: &IScheduleCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IScheduleCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IScheduleCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IScheduleCollection> for super::Com::IDispatch {
    fn from(value: IScheduleCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IScheduleCollection> for super::Com::IDispatch {
    fn from(value: &IScheduleCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IScheduleCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a IScheduleCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IScheduleCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IScheduleCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IScheduleCollection {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IScheduleCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IScheduleCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IScheduleCollection {
    type Vtable = IScheduleCollection_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0383753d_098b_11d8_9414_505054503030);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IScheduleCollection_Vtbl {
    pub base: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppschedule: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ienum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pschedule: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vschedule: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Remove: usize,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub AddRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pschedules: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddRange: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateSchedule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, schedule: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateSchedule: usize,
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[repr(transparent)]
pub struct ISystemMonitor(::windows::core::IUnknown);
impl ISystemMonitor {
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn Appearance(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Appearance)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetAppearance(&self, iappearance: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAppearance)(::core::mem::transmute_copy(self), ::core::mem::transmute(iappearance)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn BackColor(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).BackColor)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetBackColor(&self, color: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBackColor)(::core::mem::transmute_copy(self), ::core::mem::transmute(color)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn BorderStyle(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).BorderStyle)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetBorderStyle(&self, iborderstyle: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBorderStyle)(::core::mem::transmute_copy(self), ::core::mem::transmute(iborderstyle)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn ForeColor(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ForeColor)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetForeColor(&self, color: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetForeColor)(::core::mem::transmute_copy(self), ::core::mem::transmute(color)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Font(&self) -> ::windows::core::Result<super::Ole::IFontDisp> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Font)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Ole::IFontDisp>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn putref_Font<'a, Param0: ::windows::core::IntoParam<'a, super::Ole::IFontDisp>>(&self, pfont: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).putref_Font)(::core::mem::transmute_copy(self), pfont.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Counters(&self) -> ::windows::core::Result<ICounters> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Counters)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ICounters>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetShowVerticalGrid(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetShowVerticalGrid)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn ShowVerticalGrid(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ShowVerticalGrid)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetShowHorizontalGrid(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetShowHorizontalGrid)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn ShowHorizontalGrid(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ShowHorizontalGrid)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetShowLegend(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetShowLegend)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn ShowLegend(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ShowLegend)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetShowScaleLabels(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetShowScaleLabels)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn ShowScaleLabels(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ShowScaleLabels)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetShowValueBar(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetShowValueBar)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn ShowValueBar(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ShowValueBar)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetMaximumScale(&self, ivalue: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMaximumScale)(::core::mem::transmute_copy(self), ::core::mem::transmute(ivalue)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn MaximumScale(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).MaximumScale)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetMinimumScale(&self, ivalue: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMinimumScale)(::core::mem::transmute_copy(self), ::core::mem::transmute(ivalue)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn MinimumScale(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).MinimumScale)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetUpdateInterval(&self, fvalue: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetUpdateInterval)(::core::mem::transmute_copy(self), ::core::mem::transmute(fvalue)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn UpdateInterval(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).UpdateInterval)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetDisplayType(&self, edisplaytype: DisplayTypeConstants) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDisplayType)(::core::mem::transmute_copy(self), ::core::mem::transmute(edisplaytype)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn DisplayType(&self) -> ::windows::core::Result<DisplayTypeConstants> {
        let mut result__: DisplayTypeConstants = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).DisplayType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DisplayTypeConstants>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetManualUpdate(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetManualUpdate)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn ManualUpdate(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ManualUpdate)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetGraphTitle<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstitle: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetGraphTitle)(::core::mem::transmute_copy(self), bstitle.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GraphTitle(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GraphTitle)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetYAxisLabel<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstitle: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetYAxisLabel)(::core::mem::transmute_copy(self), bstitle.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn YAxisLabel(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).YAxisLabel)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn CollectSample(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CollectSample)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn UpdateGraph(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UpdateGraph)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn BrowseCounters(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BrowseCounters)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn DisplayProperties(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DisplayProperties)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn Counter(&self, iindex: i32) -> ::windows::core::Result<ICounterItem> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Counter)(::core::mem::transmute_copy(self), ::core::mem::transmute(iindex), ::core::mem::transmute(&mut result__)).from_abi::<ICounterItem>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddCounter<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bspath: Param0) -> ::windows::core::Result<ICounterItem> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).AddCounter)(::core::mem::transmute_copy(self), bspath.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<ICounterItem>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn DeleteCounter<'a, Param0: ::windows::core::IntoParam<'a, ICounterItem>>(&self, pctr: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DeleteCounter)(::core::mem::transmute_copy(self), pctr.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn BackColorCtl(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).BackColorCtl)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetBackColorCtl(&self, color: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBackColorCtl)(::core::mem::transmute_copy(self), ::core::mem::transmute(color)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLogFileName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bsfilename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetLogFileName)(::core::mem::transmute_copy(self), bsfilename.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LogFileName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).LogFileName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetLogViewStart(&self, starttime: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetLogViewStart)(::core::mem::transmute_copy(self), ::core::mem::transmute(starttime)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn LogViewStart(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).LogViewStart)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetLogViewStop(&self, stoptime: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetLogViewStop)(::core::mem::transmute_copy(self), ::core::mem::transmute(stoptime)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn LogViewStop(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).LogViewStop)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn GridColor(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GridColor)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetGridColor(&self, color: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetGridColor)(::core::mem::transmute_copy(self), ::core::mem::transmute(color)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn TimeBarColor(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).TimeBarColor)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetTimeBarColor(&self, color: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTimeBarColor)(::core::mem::transmute_copy(self), ::core::mem::transmute(color)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn Highlight(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Highlight)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetHighlight(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetHighlight)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn ShowToolbar(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ShowToolbar)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetShowToolbar(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetShowToolbar)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn Paste(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Paste)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn Copy(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Copy)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetReadOnly(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetReadOnly)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn ReadOnly(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ReadOnly)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetReportValueType(&self, ereportvaluetype: ReportValueTypeConstants) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetReportValueType)(::core::mem::transmute_copy(self), ::core::mem::transmute(ereportvaluetype)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn ReportValueType(&self) -> ::windows::core::Result<ReportValueTypeConstants> {
        let mut result__: ReportValueTypeConstants = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ReportValueType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ReportValueTypeConstants>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetMonitorDuplicateInstances(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMonitorDuplicateInstances)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn MonitorDuplicateInstances(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).MonitorDuplicateInstances)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetDisplayFilter(&self, ivalue: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDisplayFilter)(::core::mem::transmute_copy(self), ::core::mem::transmute(ivalue)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn DisplayFilter(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).DisplayFilter)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn LogFiles(&self) -> ::windows::core::Result<ILogFiles> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).LogFiles)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ILogFiles>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetDataSourceType(&self, edatasourcetype: DataSourceTypeConstants) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDataSourceType)(::core::mem::transmute_copy(self), ::core::mem::transmute(edatasourcetype)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn DataSourceType(&self) -> ::windows::core::Result<DataSourceTypeConstants> {
        let mut result__: DataSourceTypeConstants = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).DataSourceType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DataSourceTypeConstants>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSqlDsnName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bssqldsnname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSqlDsnName)(::core::mem::transmute_copy(self), bssqldsnname.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SqlDsnName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SqlDsnName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSqlLogSetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bssqllogsetname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSqlLogSetName)(::core::mem::transmute_copy(self), bssqllogsetname.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SqlLogSetName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SqlLogSetName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
impl ::core::convert::From<ISystemMonitor> for ::windows::core::IUnknown {
    fn from(value: ISystemMonitor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISystemMonitor> for ::windows::core::IUnknown {
    fn from(value: &ISystemMonitor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISystemMonitor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISystemMonitor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISystemMonitor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISystemMonitor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISystemMonitor {}
impl ::core::fmt::Debug for ISystemMonitor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISystemMonitor").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISystemMonitor {
    type Vtable = ISystemMonitor_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x194eb241_c32c_11cf_9398_00aa00a3ddea);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemMonitor_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Appearance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iappearance: *mut i32) -> ::windows::core::HRESULT,
    pub SetAppearance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iappearance: i32) -> ::windows::core::HRESULT,
    pub BackColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcolor: *mut u32) -> ::windows::core::HRESULT,
    pub SetBackColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, color: u32) -> ::windows::core::HRESULT,
    pub BorderStyle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iborderstyle: *mut i32) -> ::windows::core::HRESULT,
    pub SetBorderStyle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iborderstyle: i32) -> ::windows::core::HRESULT,
    pub ForeColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcolor: *mut u32) -> ::windows::core::HRESULT,
    pub SetForeColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, color: u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Font: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppfont: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Font: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub putref_Font: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfont: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    putref_Font: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Counters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppicounters: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Counters: usize,
    pub SetShowVerticalGrid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT,
    pub ShowVerticalGrid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub SetShowHorizontalGrid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT,
    pub ShowHorizontalGrid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub SetShowLegend: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT,
    pub ShowLegend: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub SetShowScaleLabels: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT,
    pub ShowScaleLabels: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub SetShowValueBar: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT,
    pub ShowValueBar: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub SetMaximumScale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ivalue: i32) -> ::windows::core::HRESULT,
    pub MaximumScale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pivalue: *mut i32) -> ::windows::core::HRESULT,
    pub SetMinimumScale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ivalue: i32) -> ::windows::core::HRESULT,
    pub MinimumScale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pivalue: *mut i32) -> ::windows::core::HRESULT,
    pub SetUpdateInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fvalue: f32) -> ::windows::core::HRESULT,
    pub UpdateInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfvalue: *mut f32) -> ::windows::core::HRESULT,
    pub SetDisplayType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, edisplaytype: DisplayTypeConstants) -> ::windows::core::HRESULT,
    pub DisplayType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pedisplaytype: *mut DisplayTypeConstants) -> ::windows::core::HRESULT,
    pub SetManualUpdate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT,
    pub ManualUpdate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetGraphTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstitle: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetGraphTitle: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GraphTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstitle: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GraphTitle: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetYAxisLabel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstitle: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetYAxisLabel: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub YAxisLabel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstitle: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    YAxisLabel: usize,
    pub CollectSample: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub UpdateGraph: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub BrowseCounters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DisplayProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Counter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iindex: i32, ppicounter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub AddCounter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bspath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppicounter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddCounter: usize,
    pub DeleteCounter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctr: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub BackColorCtl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcolor: *mut u32) -> ::windows::core::HRESULT,
    pub SetBackColorCtl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, color: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetLogFileName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bsfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetLogFileName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub LogFileName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bsfilename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LogFileName: usize,
    pub SetLogViewStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, starttime: f64) -> ::windows::core::HRESULT,
    pub LogViewStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, starttime: *mut f64) -> ::windows::core::HRESULT,
    pub SetLogViewStop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stoptime: f64) -> ::windows::core::HRESULT,
    pub LogViewStop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stoptime: *mut f64) -> ::windows::core::HRESULT,
    pub GridColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcolor: *mut u32) -> ::windows::core::HRESULT,
    pub SetGridColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, color: u32) -> ::windows::core::HRESULT,
    pub TimeBarColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcolor: *mut u32) -> ::windows::core::HRESULT,
    pub SetTimeBarColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, color: u32) -> ::windows::core::HRESULT,
    pub Highlight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub SetHighlight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT,
    pub ShowToolbar: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub SetShowToolbar: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT,
    pub Paste: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Copy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetReadOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT,
    pub ReadOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub SetReportValueType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ereportvaluetype: ReportValueTypeConstants) -> ::windows::core::HRESULT,
    pub ReportValueType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pereportvaluetype: *mut ReportValueTypeConstants) -> ::windows::core::HRESULT,
    pub SetMonitorDuplicateInstances: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT,
    pub MonitorDuplicateInstances: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub SetDisplayFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ivalue: i32) -> ::windows::core::HRESULT,
    pub DisplayFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pivalue: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub LogFiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppilogfiles: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    LogFiles: usize,
    pub SetDataSourceType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, edatasourcetype: DataSourceTypeConstants) -> ::windows::core::HRESULT,
    pub DataSourceType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pedatasourcetype: *mut DataSourceTypeConstants) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSqlDsnName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bssqldsnname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSqlDsnName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SqlDsnName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bssqldsnname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SqlDsnName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSqlLogSetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bssqllogsetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSqlLogSetName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SqlLogSetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bssqllogsetname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SqlLogSetName: usize,
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[repr(transparent)]
pub struct ISystemMonitor2(::windows::core::IUnknown);
impl ISystemMonitor2 {
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn Appearance(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Appearance)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetAppearance(&self, iappearance: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetAppearance)(::core::mem::transmute_copy(self), ::core::mem::transmute(iappearance)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn BackColor(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.BackColor)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetBackColor(&self, color: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetBackColor)(::core::mem::transmute_copy(self), ::core::mem::transmute(color)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn BorderStyle(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.BorderStyle)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetBorderStyle(&self, iborderstyle: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetBorderStyle)(::core::mem::transmute_copy(self), ::core::mem::transmute(iborderstyle)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn ForeColor(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.ForeColor)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetForeColor(&self, color: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetForeColor)(::core::mem::transmute_copy(self), ::core::mem::transmute(color)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Font(&self) -> ::windows::core::Result<super::Ole::IFontDisp> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Font)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Ole::IFontDisp>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn putref_Font<'a, Param0: ::windows::core::IntoParam<'a, super::Ole::IFontDisp>>(&self, pfont: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.putref_Font)(::core::mem::transmute_copy(self), pfont.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Counters(&self) -> ::windows::core::Result<ICounters> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Counters)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ICounters>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetShowVerticalGrid(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetShowVerticalGrid)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn ShowVerticalGrid(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.ShowVerticalGrid)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetShowHorizontalGrid(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetShowHorizontalGrid)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn ShowHorizontalGrid(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.ShowHorizontalGrid)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetShowLegend(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetShowLegend)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn ShowLegend(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.ShowLegend)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetShowScaleLabels(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetShowScaleLabels)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn ShowScaleLabels(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.ShowScaleLabels)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetShowValueBar(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetShowValueBar)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn ShowValueBar(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.ShowValueBar)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetMaximumScale(&self, ivalue: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetMaximumScale)(::core::mem::transmute_copy(self), ::core::mem::transmute(ivalue)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn MaximumScale(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.MaximumScale)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetMinimumScale(&self, ivalue: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetMinimumScale)(::core::mem::transmute_copy(self), ::core::mem::transmute(ivalue)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn MinimumScale(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.MinimumScale)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetUpdateInterval(&self, fvalue: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetUpdateInterval)(::core::mem::transmute_copy(self), ::core::mem::transmute(fvalue)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn UpdateInterval(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.UpdateInterval)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetDisplayType(&self, edisplaytype: DisplayTypeConstants) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetDisplayType)(::core::mem::transmute_copy(self), ::core::mem::transmute(edisplaytype)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn DisplayType(&self) -> ::windows::core::Result<DisplayTypeConstants> {
        let mut result__: DisplayTypeConstants = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.DisplayType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DisplayTypeConstants>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetManualUpdate(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetManualUpdate)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn ManualUpdate(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.ManualUpdate)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetGraphTitle<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstitle: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetGraphTitle)(::core::mem::transmute_copy(self), bstitle.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GraphTitle(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GraphTitle)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetYAxisLabel<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstitle: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetYAxisLabel)(::core::mem::transmute_copy(self), bstitle.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn YAxisLabel(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.YAxisLabel)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn CollectSample(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.CollectSample)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn UpdateGraph(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.UpdateGraph)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn BrowseCounters(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.BrowseCounters)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn DisplayProperties(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.DisplayProperties)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn Counter(&self, iindex: i32) -> ::windows::core::Result<ICounterItem> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Counter)(::core::mem::transmute_copy(self), ::core::mem::transmute(iindex), ::core::mem::transmute(&mut result__)).from_abi::<ICounterItem>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddCounter<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bspath: Param0) -> ::windows::core::Result<ICounterItem> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.AddCounter)(::core::mem::transmute_copy(self), bspath.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<ICounterItem>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn DeleteCounter<'a, Param0: ::windows::core::IntoParam<'a, ICounterItem>>(&self, pctr: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.DeleteCounter)(::core::mem::transmute_copy(self), pctr.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn BackColorCtl(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.BackColorCtl)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetBackColorCtl(&self, color: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetBackColorCtl)(::core::mem::transmute_copy(self), ::core::mem::transmute(color)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLogFileName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bsfilename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetLogFileName)(::core::mem::transmute_copy(self), bsfilename.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LogFileName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.LogFileName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetLogViewStart(&self, starttime: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetLogViewStart)(::core::mem::transmute_copy(self), ::core::mem::transmute(starttime)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn LogViewStart(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.LogViewStart)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetLogViewStop(&self, stoptime: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetLogViewStop)(::core::mem::transmute_copy(self), ::core::mem::transmute(stoptime)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn LogViewStop(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.LogViewStop)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn GridColor(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GridColor)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetGridColor(&self, color: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetGridColor)(::core::mem::transmute_copy(self), ::core::mem::transmute(color)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn TimeBarColor(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.TimeBarColor)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetTimeBarColor(&self, color: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetTimeBarColor)(::core::mem::transmute_copy(self), ::core::mem::transmute(color)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn Highlight(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Highlight)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetHighlight(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetHighlight)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn ShowToolbar(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.ShowToolbar)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetShowToolbar(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetShowToolbar)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn Paste(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Paste)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn Copy(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Copy)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Reset)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetReadOnly(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetReadOnly)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn ReadOnly(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.ReadOnly)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetReportValueType(&self, ereportvaluetype: ReportValueTypeConstants) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetReportValueType)(::core::mem::transmute_copy(self), ::core::mem::transmute(ereportvaluetype)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn ReportValueType(&self) -> ::windows::core::Result<ReportValueTypeConstants> {
        let mut result__: ReportValueTypeConstants = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.ReportValueType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ReportValueTypeConstants>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetMonitorDuplicateInstances(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetMonitorDuplicateInstances)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn MonitorDuplicateInstances(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.MonitorDuplicateInstances)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetDisplayFilter(&self, ivalue: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetDisplayFilter)(::core::mem::transmute_copy(self), ::core::mem::transmute(ivalue)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn DisplayFilter(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.DisplayFilter)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn LogFiles(&self) -> ::windows::core::Result<ILogFiles> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.LogFiles)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ILogFiles>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetDataSourceType(&self, edatasourcetype: DataSourceTypeConstants) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetDataSourceType)(::core::mem::transmute_copy(self), ::core::mem::transmute(edatasourcetype)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn DataSourceType(&self) -> ::windows::core::Result<DataSourceTypeConstants> {
        let mut result__: DataSourceTypeConstants = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.DataSourceType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DataSourceTypeConstants>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSqlDsnName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bssqldsnname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetSqlDsnName)(::core::mem::transmute_copy(self), bssqldsnname.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SqlDsnName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.SqlDsnName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSqlLogSetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bssqllogsetname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetSqlLogSetName)(::core::mem::transmute_copy(self), bssqllogsetname.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SqlLogSetName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.SqlLogSetName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetEnableDigitGrouping(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetEnableDigitGrouping)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn EnableDigitGrouping(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).EnableDigitGrouping)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetEnableToolTips(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetEnableToolTips)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn EnableToolTips(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).EnableToolTips)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetShowTimeAxisLabels(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetShowTimeAxisLabels)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn ShowTimeAxisLabels(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ShowTimeAxisLabels)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetChartScroll(&self, bscroll: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetChartScroll)(::core::mem::transmute_copy(self), ::core::mem::transmute(bscroll)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn ChartScroll(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ChartScroll)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetDataPointCount(&self, inewcount: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDataPointCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(inewcount)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn DataPointCount(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).DataPointCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn ScaleToFit(&self, bselectedcountersonly: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ScaleToFit)(::core::mem::transmute_copy(self), ::core::mem::transmute(bselectedcountersonly)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SaveAs<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrfilename: Param0, esysmonfiletype: SysmonFileType) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SaveAs)(::core::mem::transmute_copy(self), bstrfilename.into_param().abi(), ::core::mem::transmute(esysmonfiletype)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Relog<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrfilename: Param0, esysmonfiletype: SysmonFileType, ifilter: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Relog)(::core::mem::transmute_copy(self), bstrfilename.into_param().abi(), ::core::mem::transmute(esysmonfiletype), ::core::mem::transmute(ifilter)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn ClearData(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ClearData)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn LogSourceStartTime(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).LogSourceStartTime)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn LogSourceStopTime(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).LogSourceStopTime)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetLogViewRange(&self, starttime: f64, stoptime: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetLogViewRange)(::core::mem::transmute_copy(self), ::core::mem::transmute(starttime), ::core::mem::transmute(stoptime)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn GetLogViewRange(&self, starttime: *mut f64, stoptime: *mut f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetLogViewRange)(::core::mem::transmute_copy(self), ::core::mem::transmute(starttime), ::core::mem::transmute(stoptime)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn BatchingLock(&self, flock: i16, ebatchreason: SysmonBatchReason) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BatchingLock)(::core::mem::transmute_copy(self), ::core::mem::transmute(flock), ::core::mem::transmute(ebatchreason)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LoadSettings<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrsettingfilename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).LoadSettings)(::core::mem::transmute_copy(self), bstrsettingfilename.into_param().abi()).ok()
    }
}
impl ::core::convert::From<ISystemMonitor2> for ::windows::core::IUnknown {
    fn from(value: ISystemMonitor2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISystemMonitor2> for ::windows::core::IUnknown {
    fn from(value: &ISystemMonitor2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISystemMonitor2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISystemMonitor2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISystemMonitor2> for ISystemMonitor {
    fn from(value: ISystemMonitor2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISystemMonitor2> for ISystemMonitor {
    fn from(value: &ISystemMonitor2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISystemMonitor> for ISystemMonitor2 {
    fn into_param(self) -> ::windows::core::Param<'a, ISystemMonitor> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISystemMonitor> for &'a ISystemMonitor2 {
    fn into_param(self) -> ::windows::core::Param<'a, ISystemMonitor> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISystemMonitor2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISystemMonitor2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISystemMonitor2 {}
impl ::core::fmt::Debug for ISystemMonitor2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISystemMonitor2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISystemMonitor2 {
    type Vtable = ISystemMonitor2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x08e3206a_5fd2_4fde_a8a5_8cb3b63d2677);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemMonitor2_Vtbl {
    pub base: ISystemMonitor_Vtbl,
    pub SetEnableDigitGrouping: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT,
    pub EnableDigitGrouping: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub SetEnableToolTips: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT,
    pub EnableToolTips: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub SetShowTimeAxisLabels: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT,
    pub ShowTimeAxisLabels: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub SetChartScroll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bscroll: i16) -> ::windows::core::HRESULT,
    pub ChartScroll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbscroll: *mut i16) -> ::windows::core::HRESULT,
    pub SetDataPointCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inewcount: i32) -> ::windows::core::HRESULT,
    pub DataPointCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pidatapointcount: *mut i32) -> ::windows::core::HRESULT,
    pub ScaleToFit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bselectedcountersonly: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SaveAs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, esysmonfiletype: SysmonFileType) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SaveAs: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Relog: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, esysmonfiletype: SysmonFileType, ifilter: i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Relog: usize,
    pub ClearData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub LogSourceStartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdate: *mut f64) -> ::windows::core::HRESULT,
    pub LogSourceStopTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdate: *mut f64) -> ::windows::core::HRESULT,
    pub SetLogViewRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, starttime: f64, stoptime: f64) -> ::windows::core::HRESULT,
    pub GetLogViewRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, starttime: *mut f64, stoptime: *mut f64) -> ::windows::core::HRESULT,
    pub BatchingLock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flock: i16, ebatchreason: SysmonBatchReason) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub LoadSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrsettingfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LoadSettings: usize,
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[repr(transparent)]
pub struct ISystemMonitorEvents(::windows::core::IUnknown);
impl ISystemMonitorEvents {
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn OnCounterSelected(&self, index: i32) {
        (::windows::core::Interface::vtable(self).OnCounterSelected)(::core::mem::transmute_copy(self), ::core::mem::transmute(index))
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn OnCounterAdded(&self, index: i32) {
        (::windows::core::Interface::vtable(self).OnCounterAdded)(::core::mem::transmute_copy(self), ::core::mem::transmute(index))
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn OnCounterDeleted(&self, index: i32) {
        (::windows::core::Interface::vtable(self).OnCounterDeleted)(::core::mem::transmute_copy(self), ::core::mem::transmute(index))
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn OnSampleCollected(&self) {
        (::windows::core::Interface::vtable(self).OnSampleCollected)(::core::mem::transmute_copy(self))
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn OnDblClick(&self, index: i32) {
        (::windows::core::Interface::vtable(self).OnDblClick)(::core::mem::transmute_copy(self), ::core::mem::transmute(index))
    }
}
impl ::core::convert::From<ISystemMonitorEvents> for ::windows::core::IUnknown {
    fn from(value: ISystemMonitorEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISystemMonitorEvents> for ::windows::core::IUnknown {
    fn from(value: &ISystemMonitorEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISystemMonitorEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISystemMonitorEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISystemMonitorEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISystemMonitorEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISystemMonitorEvents {}
impl ::core::fmt::Debug for ISystemMonitorEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISystemMonitorEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISystemMonitorEvents {
    type Vtable = ISystemMonitorEvents_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xee660ea0_4abd_11cf_943a_008029004347);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemMonitorEvents_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub OnCounterSelected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32),
    pub OnCounterAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32),
    pub OnCounterDeleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32),
    pub OnSampleCollected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub OnDblClick: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32),
}
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITraceDataCollector(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITraceDataCollector {
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DataCollectorSet(&self) -> ::windows::core::Result<IDataCollectorSet> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.DataCollectorSet)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDataCollectorSet>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetDataCollectorSet<'a, Param0: ::windows::core::IntoParam<'a, IDataCollectorSet>>(&self, group: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetDataCollectorSet)(::core::mem::transmute_copy(self), group.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn DataCollectorType(&self) -> ::windows::core::Result<DataCollectorType> {
        let mut result__: DataCollectorType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.DataCollectorType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DataCollectorType>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FileName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.FileName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFileName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetFileName)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn FileNameFormat(&self) -> ::windows::core::Result<AutoPathFormat> {
        let mut result__: AutoPathFormat = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.FileNameFormat)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<AutoPathFormat>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetFileNameFormat(&self, format: AutoPathFormat) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetFileNameFormat)(::core::mem::transmute_copy(self), ::core::mem::transmute(format)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FileNameFormatPattern(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.FileNameFormatPattern)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFileNameFormatPattern<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, pattern: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetFileNameFormatPattern)(::core::mem::transmute_copy(self), pattern.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LatestOutputLocation(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.LatestOutputLocation)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLatestOutputLocation<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetLatestOutputLocation)(::core::mem::transmute_copy(self), path.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn LogAppend(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.LogAppend)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetLogAppend(&self, append: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetLogAppend)(::core::mem::transmute_copy(self), ::core::mem::transmute(append)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn LogCircular(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.LogCircular)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetLogCircular(&self, circular: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetLogCircular)(::core::mem::transmute_copy(self), ::core::mem::transmute(circular)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn LogOverwrite(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.LogOverwrite)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetLogOverwrite(&self, overwrite: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetLogOverwrite)(::core::mem::transmute_copy(self), ::core::mem::transmute(overwrite)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Name)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetName)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OutputLocation(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.OutputLocation)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn Index(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Index)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetIndex(&self, index: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetIndex)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Xml(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Xml)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn SetXml<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, xml: Param0) -> ::windows::core::Result<IValueMap> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.SetXml)(::core::mem::transmute_copy(self), xml.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IValueMap>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateOutputLocation(&self, latest: i16) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateOutputLocation)(::core::mem::transmute_copy(self), ::core::mem::transmute(latest), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn BufferSize(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).BufferSize)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetBufferSize(&self, size: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBufferSize)(::core::mem::transmute_copy(self), ::core::mem::transmute(size)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn BuffersLost(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).BuffersLost)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetBuffersLost(&self, buffers: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBuffersLost)(::core::mem::transmute_copy(self), ::core::mem::transmute(buffers)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn BuffersWritten(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).BuffersWritten)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetBuffersWritten(&self, buffers: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBuffersWritten)(::core::mem::transmute_copy(self), ::core::mem::transmute(buffers)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn ClockType(&self) -> ::windows::core::Result<ClockType> {
        let mut result__: ClockType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ClockType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ClockType>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetClockType(&self, clock: ClockType) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetClockType)(::core::mem::transmute_copy(self), ::core::mem::transmute(clock)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn EventsLost(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).EventsLost)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetEventsLost(&self, events: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetEventsLost)(::core::mem::transmute_copy(self), ::core::mem::transmute(events)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn ExtendedModes(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ExtendedModes)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetExtendedModes(&self, mode: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetExtendedModes)(::core::mem::transmute_copy(self), ::core::mem::transmute(mode)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn FlushTimer(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).FlushTimer)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetFlushTimer(&self, seconds: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetFlushTimer)(::core::mem::transmute_copy(self), ::core::mem::transmute(seconds)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn FreeBuffers(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).FreeBuffers)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetFreeBuffers(&self, buffers: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetFreeBuffers)(::core::mem::transmute_copy(self), ::core::mem::transmute(buffers)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn Guid(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Guid)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetGuid<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, guid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetGuid)(::core::mem::transmute_copy(self), guid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn IsKernelTrace(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).IsKernelTrace)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn MaximumBuffers(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).MaximumBuffers)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetMaximumBuffers(&self, buffers: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMaximumBuffers)(::core::mem::transmute_copy(self), ::core::mem::transmute(buffers)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn MinimumBuffers(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).MinimumBuffers)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetMinimumBuffers(&self, buffers: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMinimumBuffers)(::core::mem::transmute_copy(self), ::core::mem::transmute(buffers)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn NumberOfBuffers(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).NumberOfBuffers)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetNumberOfBuffers(&self, buffers: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetNumberOfBuffers)(::core::mem::transmute_copy(self), ::core::mem::transmute(buffers)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn PreallocateFile(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).PreallocateFile)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetPreallocateFile(&self, allocate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPreallocateFile)(::core::mem::transmute_copy(self), ::core::mem::transmute(allocate)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn ProcessMode(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ProcessMode)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetProcessMode(&self, process: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetProcessMode)(::core::mem::transmute_copy(self), ::core::mem::transmute(process)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn RealTimeBuffersLost(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).RealTimeBuffersLost)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetRealTimeBuffersLost(&self, buffers: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetRealTimeBuffersLost)(::core::mem::transmute_copy(self), ::core::mem::transmute(buffers)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SessionId(&self) -> ::windows::core::Result<u64> {
        let mut result__: u64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SessionId)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetSessionId(&self, id: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSessionId)(::core::mem::transmute_copy(self), ::core::mem::transmute(id)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SessionName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SessionName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSessionName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSessionName)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SessionThreadId(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SessionThreadId)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetSessionThreadId(&self, tid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSessionThreadId)(::core::mem::transmute_copy(self), ::core::mem::transmute(tid)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn StreamMode(&self) -> ::windows::core::Result<StreamMode> {
        let mut result__: StreamMode = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).StreamMode)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<StreamMode>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetStreamMode(&self, mode: StreamMode) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetStreamMode)(::core::mem::transmute_copy(self), ::core::mem::transmute(mode)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn TraceDataProviders(&self) -> ::windows::core::Result<ITraceDataProviderCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).TraceDataProviders)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITraceDataProviderCollection>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ITraceDataCollector> for ::windows::core::IUnknown {
    fn from(value: ITraceDataCollector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ITraceDataCollector> for ::windows::core::IUnknown {
    fn from(value: &ITraceDataCollector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITraceDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITraceDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ITraceDataCollector> for super::Com::IDispatch {
    fn from(value: ITraceDataCollector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ITraceDataCollector> for super::Com::IDispatch {
    fn from(value: &ITraceDataCollector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for ITraceDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a ITraceDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ITraceDataCollector> for IDataCollector {
    fn from(value: ITraceDataCollector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ITraceDataCollector> for IDataCollector {
    fn from(value: &ITraceDataCollector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IDataCollector> for ITraceDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, IDataCollector> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IDataCollector> for &'a ITraceDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, IDataCollector> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITraceDataCollector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITraceDataCollector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITraceDataCollector {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITraceDataCollector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITraceDataCollector").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITraceDataCollector {
    type Vtable = ITraceDataCollector_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0383750b_098b_11d8_9414_505054503030);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITraceDataCollector_Vtbl {
    pub base: IDataCollector_Vtbl,
    pub BufferSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, size: *mut u32) -> ::windows::core::HRESULT,
    pub SetBufferSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, size: u32) -> ::windows::core::HRESULT,
    pub BuffersLost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffers: *mut u32) -> ::windows::core::HRESULT,
    pub SetBuffersLost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffers: u32) -> ::windows::core::HRESULT,
    pub BuffersWritten: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffers: *mut u32) -> ::windows::core::HRESULT,
    pub SetBuffersWritten: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffers: u32) -> ::windows::core::HRESULT,
    pub ClockType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clock: *mut ClockType) -> ::windows::core::HRESULT,
    pub SetClockType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clock: ClockType) -> ::windows::core::HRESULT,
    pub EventsLost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, events: *mut u32) -> ::windows::core::HRESULT,
    pub SetEventsLost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, events: u32) -> ::windows::core::HRESULT,
    pub ExtendedModes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: *mut u32) -> ::windows::core::HRESULT,
    pub SetExtendedModes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: u32) -> ::windows::core::HRESULT,
    pub FlushTimer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, seconds: *mut u32) -> ::windows::core::HRESULT,
    pub SetFlushTimer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, seconds: u32) -> ::windows::core::HRESULT,
    pub FreeBuffers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffers: *mut u32) -> ::windows::core::HRESULT,
    pub SetFreeBuffers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffers: u32) -> ::windows::core::HRESULT,
    pub Guid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub SetGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub IsKernelTrace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, kernel: *mut i16) -> ::windows::core::HRESULT,
    pub MaximumBuffers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffers: *mut u32) -> ::windows::core::HRESULT,
    pub SetMaximumBuffers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffers: u32) -> ::windows::core::HRESULT,
    pub MinimumBuffers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffers: *mut u32) -> ::windows::core::HRESULT,
    pub SetMinimumBuffers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffers: u32) -> ::windows::core::HRESULT,
    pub NumberOfBuffers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffers: *mut u32) -> ::windows::core::HRESULT,
    pub SetNumberOfBuffers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffers: u32) -> ::windows::core::HRESULT,
    pub PreallocateFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allocate: *mut i16) -> ::windows::core::HRESULT,
    pub SetPreallocateFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allocate: i16) -> ::windows::core::HRESULT,
    pub ProcessMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, process: *mut i16) -> ::windows::core::HRESULT,
    pub SetProcessMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, process: i16) -> ::windows::core::HRESULT,
    pub RealTimeBuffersLost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffers: *mut u32) -> ::windows::core::HRESULT,
    pub SetRealTimeBuffersLost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffers: u32) -> ::windows::core::HRESULT,
    pub SessionId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: *mut u64) -> ::windows::core::HRESULT,
    pub SetSessionId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SessionName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SessionName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSessionName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSessionName: usize,
    pub SessionThreadId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tid: *mut u32) -> ::windows::core::HRESULT,
    pub SetSessionThreadId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tid: u32) -> ::windows::core::HRESULT,
    pub StreamMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: *mut StreamMode) -> ::windows::core::HRESULT,
    pub SetStreamMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: StreamMode) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub TraceDataProviders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, providers: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    TraceDataProviders: usize,
}
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITraceDataProvider(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITraceDataProvider {
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DisplayName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).DisplayName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDisplayName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDisplayName)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn Guid(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Guid)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetGuid<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, guid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetGuid)(::core::mem::transmute_copy(self), guid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Level(&self) -> ::windows::core::Result<IValueMap> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Level)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IValueMap>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn KeywordsAny(&self) -> ::windows::core::Result<IValueMap> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).KeywordsAny)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IValueMap>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn KeywordsAll(&self) -> ::windows::core::Result<IValueMap> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).KeywordsAll)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IValueMap>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> ::windows::core::Result<IValueMap> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Properties)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IValueMap>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn FilterEnabled(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).FilterEnabled)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetFilterEnabled(&self, filterenabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetFilterEnabled)(::core::mem::transmute_copy(self), ::core::mem::transmute(filterenabled)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn FilterType(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).FilterType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetFilterType(&self, ultype: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetFilterType)(::core::mem::transmute_copy(self), ::core::mem::transmute(ultype)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn FilterData(&self) -> ::windows::core::Result<*mut super::Com::SAFEARRAY> {
        let mut result__: *mut super::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).FilterData)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetFilterData(&self, pdata: *const super::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetFilterData)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Query<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrname: Param0, bstrserver: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Query)(::core::mem::transmute_copy(self), bstrname.into_param().abi(), bstrserver.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Resolve<'a, Param0: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, pfrom: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Resolve)(::core::mem::transmute_copy(self), pfrom.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSecurity<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, sddl: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSecurity)(::core::mem::transmute_copy(self), sddl.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSecurity(&self, securityinfo: u32) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSecurity)(::core::mem::transmute_copy(self), ::core::mem::transmute(securityinfo), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetRegisteredProcesses(&self) -> ::windows::core::Result<IValueMap> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetRegisteredProcesses)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IValueMap>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ITraceDataProvider> for ::windows::core::IUnknown {
    fn from(value: ITraceDataProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ITraceDataProvider> for ::windows::core::IUnknown {
    fn from(value: &ITraceDataProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITraceDataProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITraceDataProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ITraceDataProvider> for super::Com::IDispatch {
    fn from(value: ITraceDataProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ITraceDataProvider> for super::Com::IDispatch {
    fn from(value: &ITraceDataProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for ITraceDataProvider {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a ITraceDataProvider {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITraceDataProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITraceDataProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITraceDataProvider {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITraceDataProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITraceDataProvider").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITraceDataProvider {
    type Vtable = ITraceDataProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03837512_098b_11d8_9414_505054503030);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITraceDataProvider_Vtbl {
    pub base: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DisplayName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDisplayName: usize,
    pub Guid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub SetGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Level: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pplevel: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Level: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub KeywordsAny: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppkeywords: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    KeywordsAny: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub KeywordsAll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppkeywords: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    KeywordsAll: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
    pub FilterEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filterenabled: *mut i16) -> ::windows::core::HRESULT,
    pub SetFilterEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filterenabled: i16) -> ::windows::core::HRESULT,
    pub FilterType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pultype: *mut u32) -> ::windows::core::HRESULT,
    pub SetFilterType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ultype: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub FilterData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdata: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    FilterData: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetFilterData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetFilterData: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Query: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrserver: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Query: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Resolve: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfrom: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Resolve: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSecurity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sddl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSecurity: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSecurity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, securityinfo: u32, sddl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSecurity: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetRegisteredProcesses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, processes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetRegisteredProcesses: usize,
}
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITraceDataProviderCollection(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITraceDataProviderCollection {
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Count)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Item<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, index: Param0) -> ::windows::core::Result<ITraceDataProvider> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Item)(::core::mem::transmute_copy(self), index.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<ITraceDataProvider>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self)._NewEnum)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Add<'a, Param0: ::windows::core::IntoParam<'a, ITraceDataProvider>>(&self, pprovider: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Add)(::core::mem::transmute_copy(self), pprovider.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Remove<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, vprovider: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Remove)(::core::mem::transmute_copy(self), vprovider.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn Clear(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Clear)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddRange<'a, Param0: ::windows::core::IntoParam<'a, ITraceDataProviderCollection>>(&self, providers: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddRange)(::core::mem::transmute_copy(self), providers.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateTraceDataProvider(&self) -> ::windows::core::Result<ITraceDataProvider> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateTraceDataProvider)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITraceDataProvider>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTraceDataProviders<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, server: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetTraceDataProviders)(::core::mem::transmute_copy(self), server.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTraceDataProvidersByProcess<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, server: Param0, pid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetTraceDataProvidersByProcess)(::core::mem::transmute_copy(self), server.into_param().abi(), ::core::mem::transmute(pid)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ITraceDataProviderCollection> for ::windows::core::IUnknown {
    fn from(value: ITraceDataProviderCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ITraceDataProviderCollection> for ::windows::core::IUnknown {
    fn from(value: &ITraceDataProviderCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITraceDataProviderCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITraceDataProviderCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ITraceDataProviderCollection> for super::Com::IDispatch {
    fn from(value: ITraceDataProviderCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ITraceDataProviderCollection> for super::Com::IDispatch {
    fn from(value: &ITraceDataProviderCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for ITraceDataProviderCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a ITraceDataProviderCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITraceDataProviderCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITraceDataProviderCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITraceDataProviderCollection {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITraceDataProviderCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITraceDataProviderCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITraceDataProviderCollection {
    type Vtable = ITraceDataProviderCollection_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03837510_098b_11d8_9414_505054503030);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITraceDataProviderCollection_Vtbl {
    pub base: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppprovider: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprovider: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vprovider: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Remove: usize,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub AddRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, providers: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddRange: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateTraceDataProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, provider: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateTraceDataProvider: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetTraceDataProviders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, server: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetTraceDataProviders: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetTraceDataProvidersByProcess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, server: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pid: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetTraceDataProvidersByProcess: usize,
}
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IValueMap(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IValueMap {
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Count)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Item<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, index: Param0) -> ::windows::core::Result<IValueMapItem> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Item)(::core::mem::transmute_copy(self), index.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IValueMapItem>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self)._NewEnum)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Description)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, description: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDescription)(::core::mem::transmute_copy(self), description.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Value(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Value)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetValue<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, value: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetValue)(::core::mem::transmute_copy(self), value.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn ValueMapType(&self) -> ::windows::core::Result<ValueMapType> {
        let mut result__: ValueMapType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ValueMapType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ValueMapType>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetValueMapType(&self, r#type: ValueMapType) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetValueMapType)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Add<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, value: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Add)(::core::mem::transmute_copy(self), value.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Remove<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, value: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Remove)(::core::mem::transmute_copy(self), value.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn Clear(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Clear)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddRange<'a, Param0: ::windows::core::IntoParam<'a, IValueMap>>(&self, map: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddRange)(::core::mem::transmute_copy(self), map.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateValueMapItem(&self) -> ::windows::core::Result<IValueMapItem> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateValueMapItem)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IValueMapItem>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IValueMap> for ::windows::core::IUnknown {
    fn from(value: IValueMap) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IValueMap> for ::windows::core::IUnknown {
    fn from(value: &IValueMap) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IValueMap {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IValueMap {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IValueMap> for super::Com::IDispatch {
    fn from(value: IValueMap) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IValueMap> for super::Com::IDispatch {
    fn from(value: &IValueMap) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IValueMap {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a IValueMap {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IValueMap {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IValueMap {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IValueMap {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IValueMap {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IValueMap").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IValueMap {
    type Vtable = IValueMap_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03837534_098b_11d8_9414_505054503030);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IValueMap_Vtbl {
    pub base: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Description: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDescription: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Value: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetValue: usize,
    pub ValueMapType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: *mut ValueMapType) -> ::windows::core::HRESULT,
    pub SetValueMapType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: ValueMapType) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Add: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Remove: usize,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub AddRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, map: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddRange: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateValueMapItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateValueMapItem: usize,
}
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IValueMapItem(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IValueMapItem {
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Description)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, description: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDescription)(::core::mem::transmute_copy(self), description.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn Enabled(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Enabled)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetEnabled(&self, enabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetEnabled)(::core::mem::transmute_copy(self), ::core::mem::transmute(enabled)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Key(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Key)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetKey<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, key: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetKey)(::core::mem::transmute_copy(self), key.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Value(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Value)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetValue<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, value: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetValue)(::core::mem::transmute_copy(self), value.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn ValueMapType(&self) -> ::windows::core::Result<ValueMapType> {
        let mut result__: ValueMapType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ValueMapType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ValueMapType>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetValueMapType(&self, r#type: ValueMapType) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetValueMapType)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IValueMapItem> for ::windows::core::IUnknown {
    fn from(value: IValueMapItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IValueMapItem> for ::windows::core::IUnknown {
    fn from(value: &IValueMapItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IValueMapItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IValueMapItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IValueMapItem> for super::Com::IDispatch {
    fn from(value: IValueMapItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IValueMapItem> for super::Com::IDispatch {
    fn from(value: &IValueMapItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IValueMapItem {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a IValueMapItem {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IValueMapItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IValueMapItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IValueMapItem {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IValueMapItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IValueMapItem").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IValueMapItem {
    type Vtable = IValueMapItem_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03837533_098b_11d8_9414_505054503030);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IValueMapItem_Vtbl {
    pub base: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Description: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDescription: usize,
    pub Enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: *mut i16) -> ::windows::core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Key: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Key: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetKey: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Value: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetValue: usize,
    pub ValueMapType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: *mut ValueMapType) -> ::windows::core::HRESULT,
    pub SetValueMapType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: ValueMapType) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn InstallPerfDllA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(szcomputername: Param0, lpinifile: Param1, dwflags: usize) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InstallPerfDllA(szcomputername: ::windows::core::PCSTR, lpinifile: ::windows::core::PCSTR, dwflags: usize) -> u32;
        }
        ::core::mem::transmute(InstallPerfDllA(szcomputername.into_param().abi(), lpinifile.into_param().abi(), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn InstallPerfDllW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(szcomputername: Param0, lpinifile: Param1, dwflags: usize) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InstallPerfDllW(szcomputername: ::windows::core::PCWSTR, lpinifile: ::windows::core::PCWSTR, dwflags: usize) -> u32;
        }
        ::core::mem::transmute(InstallPerfDllW(szcomputername.into_param().abi(), lpinifile.into_param().abi(), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const LIBID_SystemMonitor: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1b773e42_2509_11cf_942f_008029004347);
pub const LegacyDataCollectorSet: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03837526_098b_11d8_9414_505054503030);
pub const LegacyDataCollectorSetCollection: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03837527_098b_11d8_9414_505054503030);
pub const LegacyTraceSession: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03837528_098b_11d8_9414_505054503030);
pub const LegacyTraceSessionCollection: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03837529_098b_11d8_9414_505054503030);
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LoadPerfCounterTextStringsA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(lpcommandline: Param0, bquietmodearg: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LoadPerfCounterTextStringsA(lpcommandline: ::windows::core::PCSTR, bquietmodearg: super::super::Foundation::BOOL) -> u32;
        }
        ::core::mem::transmute(LoadPerfCounterTextStringsA(lpcommandline.into_param().abi(), bquietmodearg.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LoadPerfCounterTextStringsW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(lpcommandline: Param0, bquietmodearg: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LoadPerfCounterTextStringsW(lpcommandline: ::windows::core::PCWSTR, bquietmodearg: super::super::Foundation::BOOL) -> u32;
        }
        ::core::mem::transmute(LoadPerfCounterTextStringsW(lpcommandline.into_param().abi(), bquietmodearg.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const LogFileItem: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x16ec5be8_df93_4237_94e4_9ee918111d71);
pub const LogFiles: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2735d9fd_f6b9_4f19_a5d9_e2d068584bc5);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const MAX_COUNTER_PATH: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const MAX_PERF_OBJECTS_IN_QUERY_FUNCTION: i32 = 64i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_ACCESS_DENIED: i32 = -1073738789i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_ASYNC_QUERY_TIMEOUT: i32 = -2147481637i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_BINARY_LOG_CORRUPT: i32 = -1073738761i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_BROWSE_DLG_CONFIG_A {
    pub _bitfield: u32,
    pub hWndOwner: super::super::Foundation::HWND,
    pub szDataSource: ::windows::core::PSTR,
    pub szReturnPathBuffer: ::windows::core::PSTR,
    pub cchReturnPathLength: u32,
    pub pCallBack: CounterPathCallBack,
    pub dwCallBackArg: usize,
    pub CallBackStatus: i32,
    pub dwDefaultDetailLevel: PERF_DETAIL,
    pub szDialogBoxCaption: ::windows::core::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PDH_BROWSE_DLG_CONFIG_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PDH_BROWSE_DLG_CONFIG_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PDH_BROWSE_DLG_CONFIG_A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PDH_BROWSE_DLG_CONFIG_A")
            .field("_bitfield", &self._bitfield)
            .field("hWndOwner", &self.hWndOwner)
            .field("szDataSource", &self.szDataSource)
            .field("szReturnPathBuffer", &self.szReturnPathBuffer)
            .field("cchReturnPathLength", &self.cchReturnPathLength)
            .field("pCallBack", &self.pCallBack.map(|f| f as usize))
            .field("dwCallBackArg", &self.dwCallBackArg)
            .field("CallBackStatus", &self.CallBackStatus)
            .field("dwDefaultDetailLevel", &self.dwDefaultDetailLevel)
            .field("szDialogBoxCaption", &self.szDialogBoxCaption)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PDH_BROWSE_DLG_CONFIG_A {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PDH_BROWSE_DLG_CONFIG_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PDH_BROWSE_DLG_CONFIG_A>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PDH_BROWSE_DLG_CONFIG_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PDH_BROWSE_DLG_CONFIG_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_BROWSE_DLG_CONFIG_HA {
    pub _bitfield: u32,
    pub hWndOwner: super::super::Foundation::HWND,
    pub hDataSource: isize,
    pub szReturnPathBuffer: ::windows::core::PSTR,
    pub cchReturnPathLength: u32,
    pub pCallBack: CounterPathCallBack,
    pub dwCallBackArg: usize,
    pub CallBackStatus: i32,
    pub dwDefaultDetailLevel: PERF_DETAIL,
    pub szDialogBoxCaption: ::windows::core::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PDH_BROWSE_DLG_CONFIG_HA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PDH_BROWSE_DLG_CONFIG_HA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PDH_BROWSE_DLG_CONFIG_HA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PDH_BROWSE_DLG_CONFIG_HA")
            .field("_bitfield", &self._bitfield)
            .field("hWndOwner", &self.hWndOwner)
            .field("hDataSource", &self.hDataSource)
            .field("szReturnPathBuffer", &self.szReturnPathBuffer)
            .field("cchReturnPathLength", &self.cchReturnPathLength)
            .field("pCallBack", &self.pCallBack.map(|f| f as usize))
            .field("dwCallBackArg", &self.dwCallBackArg)
            .field("CallBackStatus", &self.CallBackStatus)
            .field("dwDefaultDetailLevel", &self.dwDefaultDetailLevel)
            .field("szDialogBoxCaption", &self.szDialogBoxCaption)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PDH_BROWSE_DLG_CONFIG_HA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PDH_BROWSE_DLG_CONFIG_HA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PDH_BROWSE_DLG_CONFIG_HA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PDH_BROWSE_DLG_CONFIG_HA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PDH_BROWSE_DLG_CONFIG_HA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_BROWSE_DLG_CONFIG_HW {
    pub _bitfield: u32,
    pub hWndOwner: super::super::Foundation::HWND,
    pub hDataSource: isize,
    pub szReturnPathBuffer: ::windows::core::PWSTR,
    pub cchReturnPathLength: u32,
    pub pCallBack: CounterPathCallBack,
    pub dwCallBackArg: usize,
    pub CallBackStatus: i32,
    pub dwDefaultDetailLevel: PERF_DETAIL,
    pub szDialogBoxCaption: ::windows::core::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PDH_BROWSE_DLG_CONFIG_HW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PDH_BROWSE_DLG_CONFIG_HW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PDH_BROWSE_DLG_CONFIG_HW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PDH_BROWSE_DLG_CONFIG_HW")
            .field("_bitfield", &self._bitfield)
            .field("hWndOwner", &self.hWndOwner)
            .field("hDataSource", &self.hDataSource)
            .field("szReturnPathBuffer", &self.szReturnPathBuffer)
            .field("cchReturnPathLength", &self.cchReturnPathLength)
            .field("pCallBack", &self.pCallBack.map(|f| f as usize))
            .field("dwCallBackArg", &self.dwCallBackArg)
            .field("CallBackStatus", &self.CallBackStatus)
            .field("dwDefaultDetailLevel", &self.dwDefaultDetailLevel)
            .field("szDialogBoxCaption", &self.szDialogBoxCaption)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PDH_BROWSE_DLG_CONFIG_HW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PDH_BROWSE_DLG_CONFIG_HW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PDH_BROWSE_DLG_CONFIG_HW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PDH_BROWSE_DLG_CONFIG_HW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PDH_BROWSE_DLG_CONFIG_HW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_BROWSE_DLG_CONFIG_W {
    pub _bitfield: u32,
    pub hWndOwner: super::super::Foundation::HWND,
    pub szDataSource: ::windows::core::PWSTR,
    pub szReturnPathBuffer: ::windows::core::PWSTR,
    pub cchReturnPathLength: u32,
    pub pCallBack: CounterPathCallBack,
    pub dwCallBackArg: usize,
    pub CallBackStatus: i32,
    pub dwDefaultDetailLevel: PERF_DETAIL,
    pub szDialogBoxCaption: ::windows::core::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PDH_BROWSE_DLG_CONFIG_W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PDH_BROWSE_DLG_CONFIG_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PDH_BROWSE_DLG_CONFIG_W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PDH_BROWSE_DLG_CONFIG_W")
            .field("_bitfield", &self._bitfield)
            .field("hWndOwner", &self.hWndOwner)
            .field("szDataSource", &self.szDataSource)
            .field("szReturnPathBuffer", &self.szReturnPathBuffer)
            .field("cchReturnPathLength", &self.cchReturnPathLength)
            .field("pCallBack", &self.pCallBack.map(|f| f as usize))
            .field("dwCallBackArg", &self.dwCallBackArg)
            .field("CallBackStatus", &self.CallBackStatus)
            .field("dwDefaultDetailLevel", &self.dwDefaultDetailLevel)
            .field("szDialogBoxCaption", &self.szDialogBoxCaption)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PDH_BROWSE_DLG_CONFIG_W {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PDH_BROWSE_DLG_CONFIG_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PDH_BROWSE_DLG_CONFIG_W>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PDH_BROWSE_DLG_CONFIG_W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PDH_BROWSE_DLG_CONFIG_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_CALC_NEGATIVE_DENOMINATOR: i32 = -2147481642i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_CALC_NEGATIVE_TIMEBASE: i32 = -2147481641i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_CALC_NEGATIVE_VALUE: i32 = -2147481640i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_CANNOT_CONNECT_MACHINE: i32 = -1073738813i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_CANNOT_CONNECT_WMI_SERVER: i32 = -1073738776i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_CANNOT_READ_NAME_STRINGS: i32 = -1073738808i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_CANNOT_SET_DEFAULT_REALTIME_DATASOURCE: i32 = -2147481636i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_COUNTER_ALREADY_IN_QUERY: i32 = -1073738762i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub struct PDH_COUNTER_INFO_A {
    pub dwLength: u32,
    pub dwType: u32,
    pub CVersion: u32,
    pub CStatus: u32,
    pub lScale: i32,
    pub lDefaultScale: i32,
    pub dwUserData: usize,
    pub dwQueryUserData: usize,
    pub szFullPath: ::windows::core::PSTR,
    pub Anonymous: PDH_COUNTER_INFO_A_0,
    pub szExplainText: ::windows::core::PSTR,
    pub DataBuffer: [u32; 1],
}
impl ::core::marker::Copy for PDH_COUNTER_INFO_A {}
impl ::core::clone::Clone for PDH_COUNTER_INFO_A {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PDH_COUNTER_INFO_A {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PDH_COUNTER_INFO_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PDH_COUNTER_INFO_A>()) == 0 }
    }
}
impl ::core::cmp::Eq for PDH_COUNTER_INFO_A {}
impl ::core::default::Default for PDH_COUNTER_INFO_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub union PDH_COUNTER_INFO_A_0 {
    pub DataItemPath: PDH_DATA_ITEM_PATH_ELEMENTS_A,
    pub CounterPath: PDH_COUNTER_PATH_ELEMENTS_A,
    pub Anonymous: PDH_COUNTER_INFO_A_0_0,
}
impl ::core::marker::Copy for PDH_COUNTER_INFO_A_0 {}
impl ::core::clone::Clone for PDH_COUNTER_INFO_A_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PDH_COUNTER_INFO_A_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PDH_COUNTER_INFO_A_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PDH_COUNTER_INFO_A_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for PDH_COUNTER_INFO_A_0 {}
impl ::core::default::Default for PDH_COUNTER_INFO_A_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub struct PDH_COUNTER_INFO_A_0_0 {
    pub szMachineName: ::windows::core::PSTR,
    pub szObjectName: ::windows::core::PSTR,
    pub szInstanceName: ::windows::core::PSTR,
    pub szParentInstance: ::windows::core::PSTR,
    pub dwInstanceIndex: u32,
    pub szCounterName: ::windows::core::PSTR,
}
impl ::core::marker::Copy for PDH_COUNTER_INFO_A_0_0 {}
impl ::core::clone::Clone for PDH_COUNTER_INFO_A_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PDH_COUNTER_INFO_A_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PDH_COUNTER_INFO_A_0_0").field("szMachineName", &self.szMachineName).field("szObjectName", &self.szObjectName).field("szInstanceName", &self.szInstanceName).field("szParentInstance", &self.szParentInstance).field("dwInstanceIndex", &self.dwInstanceIndex).field("szCounterName", &self.szCounterName).finish()
    }
}
unsafe impl ::windows::core::Abi for PDH_COUNTER_INFO_A_0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PDH_COUNTER_INFO_A_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PDH_COUNTER_INFO_A_0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for PDH_COUNTER_INFO_A_0_0 {}
impl ::core::default::Default for PDH_COUNTER_INFO_A_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub struct PDH_COUNTER_INFO_W {
    pub dwLength: u32,
    pub dwType: u32,
    pub CVersion: u32,
    pub CStatus: u32,
    pub lScale: i32,
    pub lDefaultScale: i32,
    pub dwUserData: usize,
    pub dwQueryUserData: usize,
    pub szFullPath: ::windows::core::PWSTR,
    pub Anonymous: PDH_COUNTER_INFO_W_0,
    pub szExplainText: ::windows::core::PWSTR,
    pub DataBuffer: [u32; 1],
}
impl ::core::marker::Copy for PDH_COUNTER_INFO_W {}
impl ::core::clone::Clone for PDH_COUNTER_INFO_W {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PDH_COUNTER_INFO_W {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PDH_COUNTER_INFO_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PDH_COUNTER_INFO_W>()) == 0 }
    }
}
impl ::core::cmp::Eq for PDH_COUNTER_INFO_W {}
impl ::core::default::Default for PDH_COUNTER_INFO_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub union PDH_COUNTER_INFO_W_0 {
    pub DataItemPath: PDH_DATA_ITEM_PATH_ELEMENTS_W,
    pub CounterPath: PDH_COUNTER_PATH_ELEMENTS_W,
    pub Anonymous: PDH_COUNTER_INFO_W_0_0,
}
impl ::core::marker::Copy for PDH_COUNTER_INFO_W_0 {}
impl ::core::clone::Clone for PDH_COUNTER_INFO_W_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PDH_COUNTER_INFO_W_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PDH_COUNTER_INFO_W_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PDH_COUNTER_INFO_W_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for PDH_COUNTER_INFO_W_0 {}
impl ::core::default::Default for PDH_COUNTER_INFO_W_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub struct PDH_COUNTER_INFO_W_0_0 {
    pub szMachineName: ::windows::core::PWSTR,
    pub szObjectName: ::windows::core::PWSTR,
    pub szInstanceName: ::windows::core::PWSTR,
    pub szParentInstance: ::windows::core::PWSTR,
    pub dwInstanceIndex: u32,
    pub szCounterName: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for PDH_COUNTER_INFO_W_0_0 {}
impl ::core::clone::Clone for PDH_COUNTER_INFO_W_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PDH_COUNTER_INFO_W_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PDH_COUNTER_INFO_W_0_0").field("szMachineName", &self.szMachineName).field("szObjectName", &self.szObjectName).field("szInstanceName", &self.szInstanceName).field("szParentInstance", &self.szParentInstance).field("dwInstanceIndex", &self.dwInstanceIndex).field("szCounterName", &self.szCounterName).finish()
    }
}
unsafe impl ::windows::core::Abi for PDH_COUNTER_INFO_W_0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PDH_COUNTER_INFO_W_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PDH_COUNTER_INFO_W_0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for PDH_COUNTER_INFO_W_0_0 {}
impl ::core::default::Default for PDH_COUNTER_INFO_W_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub struct PDH_COUNTER_PATH_ELEMENTS_A {
    pub szMachineName: ::windows::core::PSTR,
    pub szObjectName: ::windows::core::PSTR,
    pub szInstanceName: ::windows::core::PSTR,
    pub szParentInstance: ::windows::core::PSTR,
    pub dwInstanceIndex: u32,
    pub szCounterName: ::windows::core::PSTR,
}
impl ::core::marker::Copy for PDH_COUNTER_PATH_ELEMENTS_A {}
impl ::core::clone::Clone for PDH_COUNTER_PATH_ELEMENTS_A {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PDH_COUNTER_PATH_ELEMENTS_A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PDH_COUNTER_PATH_ELEMENTS_A").field("szMachineName", &self.szMachineName).field("szObjectName", &self.szObjectName).field("szInstanceName", &self.szInstanceName).field("szParentInstance", &self.szParentInstance).field("dwInstanceIndex", &self.dwInstanceIndex).field("szCounterName", &self.szCounterName).finish()
    }
}
unsafe impl ::windows::core::Abi for PDH_COUNTER_PATH_ELEMENTS_A {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PDH_COUNTER_PATH_ELEMENTS_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PDH_COUNTER_PATH_ELEMENTS_A>()) == 0 }
    }
}
impl ::core::cmp::Eq for PDH_COUNTER_PATH_ELEMENTS_A {}
impl ::core::default::Default for PDH_COUNTER_PATH_ELEMENTS_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub struct PDH_COUNTER_PATH_ELEMENTS_W {
    pub szMachineName: ::windows::core::PWSTR,
    pub szObjectName: ::windows::core::PWSTR,
    pub szInstanceName: ::windows::core::PWSTR,
    pub szParentInstance: ::windows::core::PWSTR,
    pub dwInstanceIndex: u32,
    pub szCounterName: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for PDH_COUNTER_PATH_ELEMENTS_W {}
impl ::core::clone::Clone for PDH_COUNTER_PATH_ELEMENTS_W {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PDH_COUNTER_PATH_ELEMENTS_W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PDH_COUNTER_PATH_ELEMENTS_W").field("szMachineName", &self.szMachineName).field("szObjectName", &self.szObjectName).field("szInstanceName", &self.szInstanceName).field("szParentInstance", &self.szParentInstance).field("dwInstanceIndex", &self.dwInstanceIndex).field("szCounterName", &self.szCounterName).finish()
    }
}
unsafe impl ::windows::core::Abi for PDH_COUNTER_PATH_ELEMENTS_W {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PDH_COUNTER_PATH_ELEMENTS_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PDH_COUNTER_PATH_ELEMENTS_W>()) == 0 }
    }
}
impl ::core::cmp::Eq for PDH_COUNTER_PATH_ELEMENTS_W {}
impl ::core::default::Default for PDH_COUNTER_PATH_ELEMENTS_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_CSTATUS_BAD_COUNTERNAME: i32 = -1073738816i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_CSTATUS_INVALID_DATA: i32 = -1073738822i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_CSTATUS_ITEM_NOT_VALIDATED: i32 = -2147481645i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_CSTATUS_NEW_DATA: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_CSTATUS_NO_COUNTER: i32 = -1073738823i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_CSTATUS_NO_COUNTERNAME: i32 = -1073738817i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_CSTATUS_NO_INSTANCE: i32 = -2147481647i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_CSTATUS_NO_MACHINE: i32 = -2147481648i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_CSTATUS_NO_OBJECT: i32 = -1073738824i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_CSTATUS_VALID_DATA: i32 = 0i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub struct PDH_DATA_ITEM_PATH_ELEMENTS_A {
    pub szMachineName: ::windows::core::PSTR,
    pub ObjectGUID: ::windows::core::GUID,
    pub dwItemId: u32,
    pub szInstanceName: ::windows::core::PSTR,
}
impl ::core::marker::Copy for PDH_DATA_ITEM_PATH_ELEMENTS_A {}
impl ::core::clone::Clone for PDH_DATA_ITEM_PATH_ELEMENTS_A {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PDH_DATA_ITEM_PATH_ELEMENTS_A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PDH_DATA_ITEM_PATH_ELEMENTS_A").field("szMachineName", &self.szMachineName).field("ObjectGUID", &self.ObjectGUID).field("dwItemId", &self.dwItemId).field("szInstanceName", &self.szInstanceName).finish()
    }
}
unsafe impl ::windows::core::Abi for PDH_DATA_ITEM_PATH_ELEMENTS_A {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PDH_DATA_ITEM_PATH_ELEMENTS_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PDH_DATA_ITEM_PATH_ELEMENTS_A>()) == 0 }
    }
}
impl ::core::cmp::Eq for PDH_DATA_ITEM_PATH_ELEMENTS_A {}
impl ::core::default::Default for PDH_DATA_ITEM_PATH_ELEMENTS_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub struct PDH_DATA_ITEM_PATH_ELEMENTS_W {
    pub szMachineName: ::windows::core::PWSTR,
    pub ObjectGUID: ::windows::core::GUID,
    pub dwItemId: u32,
    pub szInstanceName: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for PDH_DATA_ITEM_PATH_ELEMENTS_W {}
impl ::core::clone::Clone for PDH_DATA_ITEM_PATH_ELEMENTS_W {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PDH_DATA_ITEM_PATH_ELEMENTS_W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PDH_DATA_ITEM_PATH_ELEMENTS_W").field("szMachineName", &self.szMachineName).field("ObjectGUID", &self.ObjectGUID).field("dwItemId", &self.dwItemId).field("szInstanceName", &self.szInstanceName).finish()
    }
}
unsafe impl ::windows::core::Abi for PDH_DATA_ITEM_PATH_ELEMENTS_W {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PDH_DATA_ITEM_PATH_ELEMENTS_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PDH_DATA_ITEM_PATH_ELEMENTS_W>()) == 0 }
    }
}
impl ::core::cmp::Eq for PDH_DATA_ITEM_PATH_ELEMENTS_W {}
impl ::core::default::Default for PDH_DATA_ITEM_PATH_ELEMENTS_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_DATA_SOURCE_IS_LOG_FILE: i32 = -1073738802i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_DATA_SOURCE_IS_REAL_TIME: i32 = -1073738801i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_DIALOG_CANCELLED: i32 = -2147481639i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PDH_DLL_VERSION(pub u32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_CVERSION_WIN50: PDH_DLL_VERSION = PDH_DLL_VERSION(1280u32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_VERSION: PDH_DLL_VERSION = PDH_DLL_VERSION(1283u32);
impl ::core::marker::Copy for PDH_DLL_VERSION {}
impl ::core::clone::Clone for PDH_DLL_VERSION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PDH_DLL_VERSION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PDH_DLL_VERSION {
    type Abi = Self;
}
impl ::core::fmt::Debug for PDH_DLL_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PDH_DLL_VERSION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_END_OF_LOG_FILE: i32 = -2147481638i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_ENTRY_NOT_IN_LOG_FILE: i32 = -1073738803i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_FILE_ALREADY_EXISTS: i32 = -1073738798i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_FILE_NOT_FOUND: i32 = -1073738799i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PDH_FMT(pub u32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_FMT_DOUBLE: PDH_FMT = PDH_FMT(512u32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_FMT_LARGE: PDH_FMT = PDH_FMT(1024u32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_FMT_LONG: PDH_FMT = PDH_FMT(256u32);
impl ::core::marker::Copy for PDH_FMT {}
impl ::core::clone::Clone for PDH_FMT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PDH_FMT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PDH_FMT {
    type Abi = Self;
}
impl ::core::fmt::Debug for PDH_FMT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PDH_FMT").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub struct PDH_FMT_COUNTERVALUE {
    pub CStatus: u32,
    pub Anonymous: PDH_FMT_COUNTERVALUE_0,
}
impl ::core::marker::Copy for PDH_FMT_COUNTERVALUE {}
impl ::core::clone::Clone for PDH_FMT_COUNTERVALUE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PDH_FMT_COUNTERVALUE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PDH_FMT_COUNTERVALUE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PDH_FMT_COUNTERVALUE>()) == 0 }
    }
}
impl ::core::cmp::Eq for PDH_FMT_COUNTERVALUE {}
impl ::core::default::Default for PDH_FMT_COUNTERVALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub union PDH_FMT_COUNTERVALUE_0 {
    pub longValue: i32,
    pub doubleValue: f64,
    pub largeValue: i64,
    pub AnsiStringValue: ::windows::core::PCSTR,
    pub WideStringValue: ::windows::core::PCWSTR,
}
impl ::core::marker::Copy for PDH_FMT_COUNTERVALUE_0 {}
impl ::core::clone::Clone for PDH_FMT_COUNTERVALUE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PDH_FMT_COUNTERVALUE_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PDH_FMT_COUNTERVALUE_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PDH_FMT_COUNTERVALUE_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for PDH_FMT_COUNTERVALUE_0 {}
impl ::core::default::Default for PDH_FMT_COUNTERVALUE_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub struct PDH_FMT_COUNTERVALUE_ITEM_A {
    pub szName: ::windows::core::PSTR,
    pub FmtValue: PDH_FMT_COUNTERVALUE,
}
impl ::core::marker::Copy for PDH_FMT_COUNTERVALUE_ITEM_A {}
impl ::core::clone::Clone for PDH_FMT_COUNTERVALUE_ITEM_A {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PDH_FMT_COUNTERVALUE_ITEM_A {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PDH_FMT_COUNTERVALUE_ITEM_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PDH_FMT_COUNTERVALUE_ITEM_A>()) == 0 }
    }
}
impl ::core::cmp::Eq for PDH_FMT_COUNTERVALUE_ITEM_A {}
impl ::core::default::Default for PDH_FMT_COUNTERVALUE_ITEM_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub struct PDH_FMT_COUNTERVALUE_ITEM_W {
    pub szName: ::windows::core::PWSTR,
    pub FmtValue: PDH_FMT_COUNTERVALUE,
}
impl ::core::marker::Copy for PDH_FMT_COUNTERVALUE_ITEM_W {}
impl ::core::clone::Clone for PDH_FMT_COUNTERVALUE_ITEM_W {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PDH_FMT_COUNTERVALUE_ITEM_W {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PDH_FMT_COUNTERVALUE_ITEM_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PDH_FMT_COUNTERVALUE_ITEM_W>()) == 0 }
    }
}
impl ::core::cmp::Eq for PDH_FMT_COUNTERVALUE_ITEM_W {}
impl ::core::default::Default for PDH_FMT_COUNTERVALUE_ITEM_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_FUNCTION_NOT_FOUND: i32 = -1073738818i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_INCORRECT_APPEND_TIME: i32 = -1073738757i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_INSUFFICIENT_BUFFER: i32 = -1073738814i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_INVALID_ARGUMENT: i32 = -1073738819i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_INVALID_BUFFER: i32 = -1073738815i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_INVALID_DATA: i32 = -1073738810i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_INVALID_DATASOURCE: i32 = -1073738787i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_INVALID_HANDLE: i32 = -1073738820i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_INVALID_INSTANCE: i32 = -1073738811i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_INVALID_PATH: i32 = -1073738812i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_INVALID_SQLDB: i32 = -1073738786i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_INVALID_SQL_LOG_FORMAT: i32 = -1073738763i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PDH_LOG(pub u32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_LOG_READ_ACCESS: PDH_LOG = PDH_LOG(65536u32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_LOG_WRITE_ACCESS: PDH_LOG = PDH_LOG(131072u32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_LOG_UPDATE_ACCESS: PDH_LOG = PDH_LOG(262144u32);
impl ::core::marker::Copy for PDH_LOG {}
impl ::core::clone::Clone for PDH_LOG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PDH_LOG {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PDH_LOG {
    type Abi = Self;
}
impl ::core::fmt::Debug for PDH_LOG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PDH_LOG").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_LOGSVC_NOT_OPENED: i32 = -1073738791i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_LOGSVC_QUERY_NOT_FOUND: i32 = -1073738792i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_LOG_FILE_CREATE_ERROR: i32 = -1073738807i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_LOG_FILE_OPEN_ERROR: i32 = -1073738806i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_LOG_FILE_TOO_SMALL: i32 = -1073738788i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_LOG_SAMPLE_TOO_SMALL: i32 = -1073738760i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_LOG_SERVICE_QUERY_INFO_A {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwLogQuota: u32,
    pub szLogFileCaption: ::windows::core::PSTR,
    pub szDefaultDir: ::windows::core::PSTR,
    pub szBaseFileName: ::windows::core::PSTR,
    pub dwFileType: u32,
    pub dwReserved: u32,
    pub Anonymous: PDH_LOG_SERVICE_QUERY_INFO_A_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PDH_LOG_SERVICE_QUERY_INFO_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PDH_LOG_SERVICE_QUERY_INFO_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PDH_LOG_SERVICE_QUERY_INFO_A {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PDH_LOG_SERVICE_QUERY_INFO_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PDH_LOG_SERVICE_QUERY_INFO_A>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PDH_LOG_SERVICE_QUERY_INFO_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PDH_LOG_SERVICE_QUERY_INFO_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union PDH_LOG_SERVICE_QUERY_INFO_A_0 {
    pub Anonymous1: PDH_LOG_SERVICE_QUERY_INFO_A_0_0,
    pub Anonymous2: PDH_LOG_SERVICE_QUERY_INFO_A_0_1,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PDH_LOG_SERVICE_QUERY_INFO_A_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PDH_LOG_SERVICE_QUERY_INFO_A_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PDH_LOG_SERVICE_QUERY_INFO_A_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PDH_LOG_SERVICE_QUERY_INFO_A_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PDH_LOG_SERVICE_QUERY_INFO_A_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PDH_LOG_SERVICE_QUERY_INFO_A_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PDH_LOG_SERVICE_QUERY_INFO_A_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_LOG_SERVICE_QUERY_INFO_A_0_0 {
    pub PdlAutoNameInterval: u32,
    pub PdlAutoNameUnits: u32,
    pub PdlCommandFilename: ::windows::core::PSTR,
    pub PdlCounterList: ::windows::core::PSTR,
    pub PdlAutoNameFormat: u32,
    pub PdlSampleInterval: u32,
    pub PdlLogStartTime: super::super::Foundation::FILETIME,
    pub PdlLogEndTime: super::super::Foundation::FILETIME,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PDH_LOG_SERVICE_QUERY_INFO_A_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PDH_LOG_SERVICE_QUERY_INFO_A_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PDH_LOG_SERVICE_QUERY_INFO_A_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PDH_LOG_SERVICE_QUERY_INFO_A_0_0").field("PdlAutoNameInterval", &self.PdlAutoNameInterval).field("PdlAutoNameUnits", &self.PdlAutoNameUnits).field("PdlCommandFilename", &self.PdlCommandFilename).field("PdlCounterList", &self.PdlCounterList).field("PdlAutoNameFormat", &self.PdlAutoNameFormat).field("PdlSampleInterval", &self.PdlSampleInterval).field("PdlLogStartTime", &self.PdlLogStartTime).field("PdlLogEndTime", &self.PdlLogEndTime).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PDH_LOG_SERVICE_QUERY_INFO_A_0_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PDH_LOG_SERVICE_QUERY_INFO_A_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PDH_LOG_SERVICE_QUERY_INFO_A_0_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PDH_LOG_SERVICE_QUERY_INFO_A_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PDH_LOG_SERVICE_QUERY_INFO_A_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_LOG_SERVICE_QUERY_INFO_A_0_1 {
    pub TlNumberOfBuffers: u32,
    pub TlMinimumBuffers: u32,
    pub TlMaximumBuffers: u32,
    pub TlFreeBuffers: u32,
    pub TlBufferSize: u32,
    pub TlEventsLost: u32,
    pub TlLoggerThreadId: u32,
    pub TlBuffersWritten: u32,
    pub TlLogHandle: u32,
    pub TlLogFileName: ::windows::core::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PDH_LOG_SERVICE_QUERY_INFO_A_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PDH_LOG_SERVICE_QUERY_INFO_A_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PDH_LOG_SERVICE_QUERY_INFO_A_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PDH_LOG_SERVICE_QUERY_INFO_A_0_1")
            .field("TlNumberOfBuffers", &self.TlNumberOfBuffers)
            .field("TlMinimumBuffers", &self.TlMinimumBuffers)
            .field("TlMaximumBuffers", &self.TlMaximumBuffers)
            .field("TlFreeBuffers", &self.TlFreeBuffers)
            .field("TlBufferSize", &self.TlBufferSize)
            .field("TlEventsLost", &self.TlEventsLost)
            .field("TlLoggerThreadId", &self.TlLoggerThreadId)
            .field("TlBuffersWritten", &self.TlBuffersWritten)
            .field("TlLogHandle", &self.TlLogHandle)
            .field("TlLogFileName", &self.TlLogFileName)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PDH_LOG_SERVICE_QUERY_INFO_A_0_1 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PDH_LOG_SERVICE_QUERY_INFO_A_0_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PDH_LOG_SERVICE_QUERY_INFO_A_0_1>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PDH_LOG_SERVICE_QUERY_INFO_A_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PDH_LOG_SERVICE_QUERY_INFO_A_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_LOG_SERVICE_QUERY_INFO_W {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwLogQuota: u32,
    pub szLogFileCaption: ::windows::core::PWSTR,
    pub szDefaultDir: ::windows::core::PWSTR,
    pub szBaseFileName: ::windows::core::PWSTR,
    pub dwFileType: u32,
    pub dwReserved: u32,
    pub Anonymous: PDH_LOG_SERVICE_QUERY_INFO_W_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PDH_LOG_SERVICE_QUERY_INFO_W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PDH_LOG_SERVICE_QUERY_INFO_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PDH_LOG_SERVICE_QUERY_INFO_W {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PDH_LOG_SERVICE_QUERY_INFO_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PDH_LOG_SERVICE_QUERY_INFO_W>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PDH_LOG_SERVICE_QUERY_INFO_W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PDH_LOG_SERVICE_QUERY_INFO_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union PDH_LOG_SERVICE_QUERY_INFO_W_0 {
    pub Anonymous1: PDH_LOG_SERVICE_QUERY_INFO_W_0_0,
    pub Anonymous2: PDH_LOG_SERVICE_QUERY_INFO_W_0_1,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PDH_LOG_SERVICE_QUERY_INFO_W_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PDH_LOG_SERVICE_QUERY_INFO_W_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PDH_LOG_SERVICE_QUERY_INFO_W_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PDH_LOG_SERVICE_QUERY_INFO_W_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PDH_LOG_SERVICE_QUERY_INFO_W_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PDH_LOG_SERVICE_QUERY_INFO_W_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PDH_LOG_SERVICE_QUERY_INFO_W_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_LOG_SERVICE_QUERY_INFO_W_0_0 {
    pub PdlAutoNameInterval: u32,
    pub PdlAutoNameUnits: u32,
    pub PdlCommandFilename: ::windows::core::PWSTR,
    pub PdlCounterList: ::windows::core::PWSTR,
    pub PdlAutoNameFormat: u32,
    pub PdlSampleInterval: u32,
    pub PdlLogStartTime: super::super::Foundation::FILETIME,
    pub PdlLogEndTime: super::super::Foundation::FILETIME,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PDH_LOG_SERVICE_QUERY_INFO_W_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PDH_LOG_SERVICE_QUERY_INFO_W_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PDH_LOG_SERVICE_QUERY_INFO_W_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PDH_LOG_SERVICE_QUERY_INFO_W_0_0").field("PdlAutoNameInterval", &self.PdlAutoNameInterval).field("PdlAutoNameUnits", &self.PdlAutoNameUnits).field("PdlCommandFilename", &self.PdlCommandFilename).field("PdlCounterList", &self.PdlCounterList).field("PdlAutoNameFormat", &self.PdlAutoNameFormat).field("PdlSampleInterval", &self.PdlSampleInterval).field("PdlLogStartTime", &self.PdlLogStartTime).field("PdlLogEndTime", &self.PdlLogEndTime).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PDH_LOG_SERVICE_QUERY_INFO_W_0_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PDH_LOG_SERVICE_QUERY_INFO_W_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PDH_LOG_SERVICE_QUERY_INFO_W_0_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PDH_LOG_SERVICE_QUERY_INFO_W_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PDH_LOG_SERVICE_QUERY_INFO_W_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_LOG_SERVICE_QUERY_INFO_W_0_1 {
    pub TlNumberOfBuffers: u32,
    pub TlMinimumBuffers: u32,
    pub TlMaximumBuffers: u32,
    pub TlFreeBuffers: u32,
    pub TlBufferSize: u32,
    pub TlEventsLost: u32,
    pub TlLoggerThreadId: u32,
    pub TlBuffersWritten: u32,
    pub TlLogHandle: u32,
    pub TlLogFileName: ::windows::core::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PDH_LOG_SERVICE_QUERY_INFO_W_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PDH_LOG_SERVICE_QUERY_INFO_W_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PDH_LOG_SERVICE_QUERY_INFO_W_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PDH_LOG_SERVICE_QUERY_INFO_W_0_1")
            .field("TlNumberOfBuffers", &self.TlNumberOfBuffers)
            .field("TlMinimumBuffers", &self.TlMinimumBuffers)
            .field("TlMaximumBuffers", &self.TlMaximumBuffers)
            .field("TlFreeBuffers", &self.TlFreeBuffers)
            .field("TlBufferSize", &self.TlBufferSize)
            .field("TlEventsLost", &self.TlEventsLost)
            .field("TlLoggerThreadId", &self.TlLoggerThreadId)
            .field("TlBuffersWritten", &self.TlBuffersWritten)
            .field("TlLogHandle", &self.TlLogHandle)
            .field("TlLogFileName", &self.TlLogFileName)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PDH_LOG_SERVICE_QUERY_INFO_W_0_1 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PDH_LOG_SERVICE_QUERY_INFO_W_0_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PDH_LOG_SERVICE_QUERY_INFO_W_0_1>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PDH_LOG_SERVICE_QUERY_INFO_W_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PDH_LOG_SERVICE_QUERY_INFO_W_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PDH_LOG_TYPE(pub u32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_LOG_TYPE_UNDEFINED: PDH_LOG_TYPE = PDH_LOG_TYPE(0u32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_LOG_TYPE_CSV: PDH_LOG_TYPE = PDH_LOG_TYPE(1u32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_LOG_TYPE_SQL: PDH_LOG_TYPE = PDH_LOG_TYPE(7u32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_LOG_TYPE_TSV: PDH_LOG_TYPE = PDH_LOG_TYPE(2u32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_LOG_TYPE_BINARY: PDH_LOG_TYPE = PDH_LOG_TYPE(8u32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_LOG_TYPE_PERFMON: PDH_LOG_TYPE = PDH_LOG_TYPE(6u32);
impl ::core::marker::Copy for PDH_LOG_TYPE {}
impl ::core::clone::Clone for PDH_LOG_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PDH_LOG_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PDH_LOG_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for PDH_LOG_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PDH_LOG_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_LOG_TYPE_NOT_FOUND: i32 = -1073738805i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_LOG_TYPE_RETIRED_BIN: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_LOG_TYPE_TRACE_GENERIC: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_LOG_TYPE_TRACE_KERNEL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_MAX_COUNTER_NAME: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_MAX_COUNTER_PATH: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_MAX_DATASOURCE_PATH: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_MAX_INSTANCE_NAME: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_MAX_SCALE: i32 = 7i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_MEMORY_ALLOCATION_FAILURE: i32 = -1073738821i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_MIN_SCALE: i32 = -7i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_MORE_DATA: i32 = -2147481646i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_NOEXPANDCOUNTERS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_NOEXPANDINSTANCES: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_NOT_IMPLEMENTED: i32 = -1073738797i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_NO_COUNTERS: i32 = -1073738785i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_NO_DATA: i32 = -2147481643i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_NO_DIALOG_DATA: i32 = -1073738809i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_NO_MORE_DATA: i32 = -1073738804i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_OS_EARLIER_VERSION: i32 = -1073738758i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_OS_LATER_VERSION: i32 = -1073738759i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PDH_PATH_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_PATH_WBEM_RESULT: PDH_PATH_FLAGS = PDH_PATH_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_PATH_WBEM_INPUT: PDH_PATH_FLAGS = PDH_PATH_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_PATH_WBEM_NONE: PDH_PATH_FLAGS = PDH_PATH_FLAGS(0u32);
impl ::core::marker::Copy for PDH_PATH_FLAGS {}
impl ::core::clone::Clone for PDH_PATH_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PDH_PATH_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PDH_PATH_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for PDH_PATH_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PDH_PATH_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_PLA_COLLECTION_ALREADY_RUNNING: i32 = -1073738775i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_PLA_COLLECTION_NOT_FOUND: i32 = -1073738773i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_PLA_ERROR_ALREADY_EXISTS: i32 = -1073738770i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_PLA_ERROR_FILEPATH: i32 = -1073738768i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_PLA_ERROR_NAME_TOO_LONG: i32 = -1073738764i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_PLA_ERROR_NOSTART: i32 = -1073738771i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_PLA_ERROR_SCHEDULE_ELAPSED: i32 = -1073738772i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_PLA_ERROR_SCHEDULE_OVERLAP: i32 = -1073738774i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_PLA_ERROR_TYPE_MISMATCH: i32 = -1073738769i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_PLA_SERVICE_ERROR: i32 = -1073738767i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_PLA_VALIDATION_ERROR: i32 = -1073738766i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_PLA_VALIDATION_WARNING: i32 = -2147480589i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_QUERY_PERF_DATA_TIMEOUT: i32 = -1073738754i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_RAW_COUNTER {
    pub CStatus: u32,
    pub TimeStamp: super::super::Foundation::FILETIME,
    pub FirstValue: i64,
    pub SecondValue: i64,
    pub MultiCount: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PDH_RAW_COUNTER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PDH_RAW_COUNTER {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PDH_RAW_COUNTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PDH_RAW_COUNTER").field("CStatus", &self.CStatus).field("TimeStamp", &self.TimeStamp).field("FirstValue", &self.FirstValue).field("SecondValue", &self.SecondValue).field("MultiCount", &self.MultiCount).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PDH_RAW_COUNTER {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PDH_RAW_COUNTER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PDH_RAW_COUNTER>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PDH_RAW_COUNTER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PDH_RAW_COUNTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_RAW_COUNTER_ITEM_A {
    pub szName: ::windows::core::PSTR,
    pub RawValue: PDH_RAW_COUNTER,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PDH_RAW_COUNTER_ITEM_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PDH_RAW_COUNTER_ITEM_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PDH_RAW_COUNTER_ITEM_A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PDH_RAW_COUNTER_ITEM_A").field("szName", &self.szName).field("RawValue", &self.RawValue).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PDH_RAW_COUNTER_ITEM_A {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PDH_RAW_COUNTER_ITEM_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PDH_RAW_COUNTER_ITEM_A>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PDH_RAW_COUNTER_ITEM_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PDH_RAW_COUNTER_ITEM_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_RAW_COUNTER_ITEM_W {
    pub szName: ::windows::core::PWSTR,
    pub RawValue: PDH_RAW_COUNTER,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PDH_RAW_COUNTER_ITEM_W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PDH_RAW_COUNTER_ITEM_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PDH_RAW_COUNTER_ITEM_W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PDH_RAW_COUNTER_ITEM_W").field("szName", &self.szName).field("RawValue", &self.RawValue).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PDH_RAW_COUNTER_ITEM_W {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PDH_RAW_COUNTER_ITEM_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PDH_RAW_COUNTER_ITEM_W>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PDH_RAW_COUNTER_ITEM_W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PDH_RAW_COUNTER_ITEM_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub struct PDH_RAW_LOG_RECORD {
    pub dwStructureSize: u32,
    pub dwRecordType: PDH_LOG_TYPE,
    pub dwItems: u32,
    pub RawBytes: [u8; 1],
}
impl ::core::marker::Copy for PDH_RAW_LOG_RECORD {}
impl ::core::clone::Clone for PDH_RAW_LOG_RECORD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PDH_RAW_LOG_RECORD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PDH_RAW_LOG_RECORD").field("dwStructureSize", &self.dwStructureSize).field("dwRecordType", &self.dwRecordType).field("dwItems", &self.dwItems).field("RawBytes", &self.RawBytes).finish()
    }
}
unsafe impl ::windows::core::Abi for PDH_RAW_LOG_RECORD {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PDH_RAW_LOG_RECORD {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PDH_RAW_LOG_RECORD>()) == 0 }
    }
}
impl ::core::cmp::Eq for PDH_RAW_LOG_RECORD {}
impl ::core::default::Default for PDH_RAW_LOG_RECORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_REFRESHCOUNTERS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_RETRY: i32 = -2147481644i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PDH_SELECT_DATA_SOURCE_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_FLAGS_FILE_BROWSER_ONLY: PDH_SELECT_DATA_SOURCE_FLAGS = PDH_SELECT_DATA_SOURCE_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_FLAGS_NONE: PDH_SELECT_DATA_SOURCE_FLAGS = PDH_SELECT_DATA_SOURCE_FLAGS(0u32);
impl ::core::marker::Copy for PDH_SELECT_DATA_SOURCE_FLAGS {}
impl ::core::clone::Clone for PDH_SELECT_DATA_SOURCE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PDH_SELECT_DATA_SOURCE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PDH_SELECT_DATA_SOURCE_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for PDH_SELECT_DATA_SOURCE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PDH_SELECT_DATA_SOURCE_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_SQL_ALLOCCON_FAILED: i32 = -1073738783i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_SQL_ALLOC_FAILED: i32 = -1073738784i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_SQL_ALTER_DETAIL_FAILED: i32 = -1073738755i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_SQL_BIND_FAILED: i32 = -1073738777i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_SQL_CONNECT_FAILED: i32 = -1073738778i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_SQL_EXEC_DIRECT_FAILED: i32 = -1073738782i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_SQL_FETCH_FAILED: i32 = -1073738781i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_SQL_MORE_RESULTS_FAILED: i32 = -1073738779i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_SQL_ROWCOUNT_FAILED: i32 = -1073738780i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub struct PDH_STATISTICS {
    pub dwFormat: u32,
    pub count: u32,
    pub min: PDH_FMT_COUNTERVALUE,
    pub max: PDH_FMT_COUNTERVALUE,
    pub mean: PDH_FMT_COUNTERVALUE,
}
impl ::core::marker::Copy for PDH_STATISTICS {}
impl ::core::clone::Clone for PDH_STATISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PDH_STATISTICS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PDH_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PDH_STATISTICS>()) == 0 }
    }
}
impl ::core::cmp::Eq for PDH_STATISTICS {}
impl ::core::default::Default for PDH_STATISTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_STRING_NOT_FOUND: i32 = -1073738796i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub struct PDH_TIME_INFO {
    pub StartTime: i64,
    pub EndTime: i64,
    pub SampleCount: u32,
}
impl ::core::marker::Copy for PDH_TIME_INFO {}
impl ::core::clone::Clone for PDH_TIME_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PDH_TIME_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PDH_TIME_INFO").field("StartTime", &self.StartTime).field("EndTime", &self.EndTime).field("SampleCount", &self.SampleCount).finish()
    }
}
unsafe impl ::windows::core::Abi for PDH_TIME_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PDH_TIME_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PDH_TIME_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for PDH_TIME_INFO {}
impl ::core::default::Default for PDH_TIME_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_UNABLE_MAP_NAME_FILES: i32 = -2147480619i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_UNABLE_READ_LOG_HEADER: i32 = -1073738800i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_UNKNOWN_LOGSVC_COMMAND: i32 = -1073738793i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_UNKNOWN_LOG_FORMAT: i32 = -1073738794i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_UNMATCHED_APPEND_COUNTER: i32 = -1073738756i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_WBEM_ERROR: i32 = -1073738790i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub type PERFLIBREQUEST = ::core::option::Option<unsafe extern "system" fn(requestcode: u32, buffer: *mut ::core::ffi::c_void, buffersize: u32) -> u32>;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_ADD_COUNTER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_AGGREGATE_INSTANCE: &'static str = "_Total";
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_AGGREGATE_MAX: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_ATTRIB_BY_REFERENCE: u64 = 1u64;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_ATTRIB_DISPLAY_AS_HEX: u64 = 16u64;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_ATTRIB_DISPLAY_AS_REAL: u64 = 8u64;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_ATTRIB_NO_DISPLAYABLE: u64 = 2u64;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_ATTRIB_NO_GROUP_SEPARATOR: u64 = 4u64;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_COLLECT_END: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_COLLECT_START: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_COUNTERSET_FLAG_AGGREGATE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_COUNTERSET_FLAG_HISTORY: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_COUNTERSET_FLAG_INSTANCE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_COUNTERSET_FLAG_MULTIPLE: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub struct PERF_COUNTERSET_INFO {
    pub CounterSetGuid: ::windows::core::GUID,
    pub ProviderGuid: ::windows::core::GUID,
    pub NumCounters: u32,
    pub InstanceType: u32,
}
impl ::core::marker::Copy for PERF_COUNTERSET_INFO {}
impl ::core::clone::Clone for PERF_COUNTERSET_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PERF_COUNTERSET_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERF_COUNTERSET_INFO").field("CounterSetGuid", &self.CounterSetGuid).field("ProviderGuid", &self.ProviderGuid).field("NumCounters", &self.NumCounters).field("InstanceType", &self.InstanceType).finish()
    }
}
unsafe impl ::windows::core::Abi for PERF_COUNTERSET_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PERF_COUNTERSET_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PERF_COUNTERSET_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for PERF_COUNTERSET_INFO {}
impl ::core::default::Default for PERF_COUNTERSET_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub struct PERF_COUNTERSET_INSTANCE {
    pub CounterSetGuid: ::windows::core::GUID,
    pub dwSize: u32,
    pub InstanceId: u32,
    pub InstanceNameOffset: u32,
    pub InstanceNameSize: u32,
}
impl ::core::marker::Copy for PERF_COUNTERSET_INSTANCE {}
impl ::core::clone::Clone for PERF_COUNTERSET_INSTANCE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PERF_COUNTERSET_INSTANCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERF_COUNTERSET_INSTANCE").field("CounterSetGuid", &self.CounterSetGuid).field("dwSize", &self.dwSize).field("InstanceId", &self.InstanceId).field("InstanceNameOffset", &self.InstanceNameOffset).field("InstanceNameSize", &self.InstanceNameSize).finish()
    }
}
unsafe impl ::windows::core::Abi for PERF_COUNTERSET_INSTANCE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PERF_COUNTERSET_INSTANCE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PERF_COUNTERSET_INSTANCE>()) == 0 }
    }
}
impl ::core::cmp::Eq for PERF_COUNTERSET_INSTANCE {}
impl ::core::default::Default for PERF_COUNTERSET_INSTANCE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_COUNTERSET_MULTI_INSTANCES: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub struct PERF_COUNTERSET_REG_INFO {
    pub CounterSetGuid: ::windows::core::GUID,
    pub CounterSetType: u32,
    pub DetailLevel: u32,
    pub NumCounters: u32,
    pub InstanceType: u32,
}
impl ::core::marker::Copy for PERF_COUNTERSET_REG_INFO {}
impl ::core::clone::Clone for PERF_COUNTERSET_REG_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PERF_COUNTERSET_REG_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERF_COUNTERSET_REG_INFO").field("CounterSetGuid", &self.CounterSetGuid).field("CounterSetType", &self.CounterSetType).field("DetailLevel", &self.DetailLevel).field("NumCounters", &self.NumCounters).field("InstanceType", &self.InstanceType).finish()
    }
}
unsafe impl ::windows::core::Abi for PERF_COUNTERSET_REG_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PERF_COUNTERSET_REG_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PERF_COUNTERSET_REG_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for PERF_COUNTERSET_REG_INFO {}
impl ::core::default::Default for PERF_COUNTERSET_REG_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_COUNTERSET_SINGLE_AGGREGATE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_COUNTERSET_SINGLE_INSTANCE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PERF_COUNTER_AGGREGATE_FUNC(pub u32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_AGGREGATE_UNDEFINED: PERF_COUNTER_AGGREGATE_FUNC = PERF_COUNTER_AGGREGATE_FUNC(0u32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_AGGREGATE_TOTAL: PERF_COUNTER_AGGREGATE_FUNC = PERF_COUNTER_AGGREGATE_FUNC(1u32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_AGGREGATE_AVG: PERF_COUNTER_AGGREGATE_FUNC = PERF_COUNTER_AGGREGATE_FUNC(2u32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_AGGREGATE_MIN: PERF_COUNTER_AGGREGATE_FUNC = PERF_COUNTER_AGGREGATE_FUNC(3u32);
impl ::core::marker::Copy for PERF_COUNTER_AGGREGATE_FUNC {}
impl ::core::clone::Clone for PERF_COUNTER_AGGREGATE_FUNC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PERF_COUNTER_AGGREGATE_FUNC {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PERF_COUNTER_AGGREGATE_FUNC {
    type Abi = Self;
}
impl ::core::fmt::Debug for PERF_COUNTER_AGGREGATE_FUNC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PERF_COUNTER_AGGREGATE_FUNC").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_COUNTER_BASE: u32 = 196608u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub struct PERF_COUNTER_BLOCK {
    pub ByteLength: u32,
}
impl ::core::marker::Copy for PERF_COUNTER_BLOCK {}
impl ::core::clone::Clone for PERF_COUNTER_BLOCK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PERF_COUNTER_BLOCK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERF_COUNTER_BLOCK").field("ByteLength", &self.ByteLength).finish()
    }
}
unsafe impl ::windows::core::Abi for PERF_COUNTER_BLOCK {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PERF_COUNTER_BLOCK {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PERF_COUNTER_BLOCK>()) == 0 }
    }
}
impl ::core::cmp::Eq for PERF_COUNTER_BLOCK {}
impl ::core::default::Default for PERF_COUNTER_BLOCK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub struct PERF_COUNTER_DATA {
    pub dwDataSize: u32,
    pub dwSize: u32,
}
impl ::core::marker::Copy for PERF_COUNTER_DATA {}
impl ::core::clone::Clone for PERF_COUNTER_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PERF_COUNTER_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERF_COUNTER_DATA").field("dwDataSize", &self.dwDataSize).field("dwSize", &self.dwSize).finish()
    }
}
unsafe impl ::windows::core::Abi for PERF_COUNTER_DATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PERF_COUNTER_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PERF_COUNTER_DATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for PERF_COUNTER_DATA {}
impl ::core::default::Default for PERF_COUNTER_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct PERF_COUNTER_DEFINITION {
    pub ByteLength: u32,
    pub CounterNameTitleIndex: u32,
    pub CounterNameTitle: u32,
    pub CounterHelpTitleIndex: u32,
    pub CounterHelpTitle: u32,
    pub DefaultScale: i32,
    pub DetailLevel: u32,
    pub CounterType: u32,
    pub CounterSize: u32,
    pub CounterOffset: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for PERF_COUNTER_DEFINITION {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for PERF_COUNTER_DEFINITION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::fmt::Debug for PERF_COUNTER_DEFINITION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERF_COUNTER_DEFINITION")
            .field("ByteLength", &self.ByteLength)
            .field("CounterNameTitleIndex", &self.CounterNameTitleIndex)
            .field("CounterNameTitle", &self.CounterNameTitle)
            .field("CounterHelpTitleIndex", &self.CounterHelpTitleIndex)
            .field("CounterHelpTitle", &self.CounterHelpTitle)
            .field("DefaultScale", &self.DefaultScale)
            .field("DetailLevel", &self.DetailLevel)
            .field("CounterType", &self.CounterType)
            .field("CounterSize", &self.CounterSize)
            .field("CounterOffset", &self.CounterOffset)
            .finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
unsafe impl ::windows::core::Abi for PERF_COUNTER_DEFINITION {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for PERF_COUNTER_DEFINITION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PERF_COUNTER_DEFINITION>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for PERF_COUNTER_DEFINITION {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for PERF_COUNTER_DEFINITION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[cfg(target_arch = "x86")]
pub struct PERF_COUNTER_DEFINITION {
    pub ByteLength: u32,
    pub CounterNameTitleIndex: u32,
    pub CounterNameTitle: ::windows::core::PWSTR,
    pub CounterHelpTitleIndex: u32,
    pub CounterHelpTitle: ::windows::core::PWSTR,
    pub DefaultScale: i32,
    pub DetailLevel: u32,
    pub CounterType: u32,
    pub CounterSize: u32,
    pub CounterOffset: u32,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for PERF_COUNTER_DEFINITION {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for PERF_COUNTER_DEFINITION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
impl ::core::fmt::Debug for PERF_COUNTER_DEFINITION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERF_COUNTER_DEFINITION")
            .field("ByteLength", &self.ByteLength)
            .field("CounterNameTitleIndex", &self.CounterNameTitleIndex)
            .field("CounterNameTitle", &self.CounterNameTitle)
            .field("CounterHelpTitleIndex", &self.CounterHelpTitleIndex)
            .field("CounterHelpTitle", &self.CounterHelpTitle)
            .field("DefaultScale", &self.DefaultScale)
            .field("DetailLevel", &self.DetailLevel)
            .field("CounterType", &self.CounterType)
            .field("CounterSize", &self.CounterSize)
            .field("CounterOffset", &self.CounterOffset)
            .finish()
    }
}
#[cfg(target_arch = "x86")]
unsafe impl ::windows::core::Abi for PERF_COUNTER_DEFINITION {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for PERF_COUNTER_DEFINITION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PERF_COUNTER_DEFINITION>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for PERF_COUNTER_DEFINITION {}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for PERF_COUNTER_DEFINITION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_COUNTER_ELAPSED: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_COUNTER_FRACTION: u32 = 131072u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub struct PERF_COUNTER_HEADER {
    pub dwStatus: u32,
    pub dwType: PerfCounterDataType,
    pub dwSize: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for PERF_COUNTER_HEADER {}
impl ::core::clone::Clone for PERF_COUNTER_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PERF_COUNTER_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERF_COUNTER_HEADER").field("dwStatus", &self.dwStatus).field("dwType", &self.dwType).field("dwSize", &self.dwSize).field("Reserved", &self.Reserved).finish()
    }
}
unsafe impl ::windows::core::Abi for PERF_COUNTER_HEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PERF_COUNTER_HEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PERF_COUNTER_HEADER>()) == 0 }
    }
}
impl ::core::cmp::Eq for PERF_COUNTER_HEADER {}
impl ::core::default::Default for PERF_COUNTER_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_COUNTER_HISTOGRAM: u32 = 393216u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_COUNTER_HISTOGRAM_TYPE: u32 = 2147483648u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub struct PERF_COUNTER_IDENTIFIER {
    pub CounterSetGuid: ::windows::core::GUID,
    pub Status: u32,
    pub Size: u32,
    pub CounterId: u32,
    pub InstanceId: u32,
    pub Index: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for PERF_COUNTER_IDENTIFIER {}
impl ::core::clone::Clone for PERF_COUNTER_IDENTIFIER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PERF_COUNTER_IDENTIFIER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERF_COUNTER_IDENTIFIER").field("CounterSetGuid", &self.CounterSetGuid).field("Status", &self.Status).field("Size", &self.Size).field("CounterId", &self.CounterId).field("InstanceId", &self.InstanceId).field("Index", &self.Index).field("Reserved", &self.Reserved).finish()
    }
}
unsafe impl ::windows::core::Abi for PERF_COUNTER_IDENTIFIER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PERF_COUNTER_IDENTIFIER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PERF_COUNTER_IDENTIFIER>()) == 0 }
    }
}
impl ::core::cmp::Eq for PERF_COUNTER_IDENTIFIER {}
impl ::core::default::Default for PERF_COUNTER_IDENTIFIER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub struct PERF_COUNTER_IDENTITY {
    pub CounterSetGuid: ::windows::core::GUID,
    pub BufferSize: u32,
    pub CounterId: u32,
    pub InstanceId: u32,
    pub MachineOffset: u32,
    pub NameOffset: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for PERF_COUNTER_IDENTITY {}
impl ::core::clone::Clone for PERF_COUNTER_IDENTITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PERF_COUNTER_IDENTITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERF_COUNTER_IDENTITY").field("CounterSetGuid", &self.CounterSetGuid).field("BufferSize", &self.BufferSize).field("CounterId", &self.CounterId).field("InstanceId", &self.InstanceId).field("MachineOffset", &self.MachineOffset).field("NameOffset", &self.NameOffset).field("Reserved", &self.Reserved).finish()
    }
}
unsafe impl ::windows::core::Abi for PERF_COUNTER_IDENTITY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PERF_COUNTER_IDENTITY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PERF_COUNTER_IDENTITY>()) == 0 }
    }
}
impl ::core::cmp::Eq for PERF_COUNTER_IDENTITY {}
impl ::core::default::Default for PERF_COUNTER_IDENTITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub struct PERF_COUNTER_INFO {
    pub CounterId: u32,
    pub Type: u32,
    pub Attrib: u64,
    pub Size: u32,
    pub DetailLevel: u32,
    pub Scale: i32,
    pub Offset: u32,
}
impl ::core::marker::Copy for PERF_COUNTER_INFO {}
impl ::core::clone::Clone for PERF_COUNTER_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PERF_COUNTER_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERF_COUNTER_INFO").field("CounterId", &self.CounterId).field("Type", &self.Type).field("Attrib", &self.Attrib).field("Size", &self.Size).field("DetailLevel", &self.DetailLevel).field("Scale", &self.Scale).field("Offset", &self.Offset).finish()
    }
}
unsafe impl ::windows::core::Abi for PERF_COUNTER_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PERF_COUNTER_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PERF_COUNTER_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for PERF_COUNTER_INFO {}
impl ::core::default::Default for PERF_COUNTER_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_COUNTER_PRECISION: u32 = 458752u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_COUNTER_QUEUELEN: u32 = 327680u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_COUNTER_RATE: u32 = 65536u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub struct PERF_COUNTER_REG_INFO {
    pub CounterId: u32,
    pub Type: u32,
    pub Attrib: u64,
    pub DetailLevel: u32,
    pub DefaultScale: i32,
    pub BaseCounterId: u32,
    pub PerfTimeId: u32,
    pub PerfFreqId: u32,
    pub MultiId: u32,
    pub AggregateFunc: PERF_COUNTER_AGGREGATE_FUNC,
    pub Reserved: u32,
}
impl ::core::marker::Copy for PERF_COUNTER_REG_INFO {}
impl ::core::clone::Clone for PERF_COUNTER_REG_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PERF_COUNTER_REG_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERF_COUNTER_REG_INFO").field("CounterId", &self.CounterId).field("Type", &self.Type).field("Attrib", &self.Attrib).field("DetailLevel", &self.DetailLevel).field("DefaultScale", &self.DefaultScale).field("BaseCounterId", &self.BaseCounterId).field("PerfTimeId", &self.PerfTimeId).field("PerfFreqId", &self.PerfFreqId).field("MultiId", &self.MultiId).field("AggregateFunc", &self.AggregateFunc).field("Reserved", &self.Reserved).finish()
    }
}
unsafe impl ::windows::core::Abi for PERF_COUNTER_REG_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PERF_COUNTER_REG_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PERF_COUNTER_REG_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for PERF_COUNTER_REG_INFO {}
impl ::core::default::Default for PERF_COUNTER_REG_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_COUNTER_VALUE: u32 = 0u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PERF_DATA_BLOCK {
    pub Signature: [u16; 4],
    pub LittleEndian: u32,
    pub Version: u32,
    pub Revision: u32,
    pub TotalByteLength: u32,
    pub HeaderLength: u32,
    pub NumObjectTypes: u32,
    pub DefaultObject: i32,
    pub SystemTime: super::super::Foundation::SYSTEMTIME,
    pub PerfTime: i64,
    pub PerfFreq: i64,
    pub PerfTime100nSec: i64,
    pub SystemNameLength: u32,
    pub SystemNameOffset: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PERF_DATA_BLOCK {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PERF_DATA_BLOCK {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PERF_DATA_BLOCK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERF_DATA_BLOCK")
            .field("Signature", &self.Signature)
            .field("LittleEndian", &self.LittleEndian)
            .field("Version", &self.Version)
            .field("Revision", &self.Revision)
            .field("TotalByteLength", &self.TotalByteLength)
            .field("HeaderLength", &self.HeaderLength)
            .field("NumObjectTypes", &self.NumObjectTypes)
            .field("DefaultObject", &self.DefaultObject)
            .field("SystemTime", &self.SystemTime)
            .field("PerfTime", &self.PerfTime)
            .field("PerfFreq", &self.PerfFreq)
            .field("PerfTime100nSec", &self.PerfTime100nSec)
            .field("SystemNameLength", &self.SystemNameLength)
            .field("SystemNameOffset", &self.SystemNameOffset)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PERF_DATA_BLOCK {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PERF_DATA_BLOCK {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PERF_DATA_BLOCK>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PERF_DATA_BLOCK {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PERF_DATA_BLOCK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PERF_DATA_HEADER {
    pub dwTotalSize: u32,
    pub dwNumCounters: u32,
    pub PerfTimeStamp: i64,
    pub PerfTime100NSec: i64,
    pub PerfFreq: i64,
    pub SystemTime: super::super::Foundation::SYSTEMTIME,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PERF_DATA_HEADER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PERF_DATA_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PERF_DATA_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERF_DATA_HEADER").field("dwTotalSize", &self.dwTotalSize).field("dwNumCounters", &self.dwNumCounters).field("PerfTimeStamp", &self.PerfTimeStamp).field("PerfTime100NSec", &self.PerfTime100NSec).field("PerfFreq", &self.PerfFreq).field("SystemTime", &self.SystemTime).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PERF_DATA_HEADER {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PERF_DATA_HEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PERF_DATA_HEADER>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PERF_DATA_HEADER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PERF_DATA_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_DATA_REVISION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_DATA_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_DELTA_BASE: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_DELTA_COUNTER: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PERF_DETAIL(pub u32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_DETAIL_NOVICE: PERF_DETAIL = PERF_DETAIL(100u32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_DETAIL_ADVANCED: PERF_DETAIL = PERF_DETAIL(200u32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_DETAIL_EXPERT: PERF_DETAIL = PERF_DETAIL(300u32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_DETAIL_WIZARD: PERF_DETAIL = PERF_DETAIL(400u32);
impl ::core::marker::Copy for PERF_DETAIL {}
impl ::core::clone::Clone for PERF_DETAIL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PERF_DETAIL {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PERF_DETAIL {
    type Abi = Self;
}
impl ::core::fmt::Debug for PERF_DETAIL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PERF_DETAIL").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_DISPLAY_NOSHOW: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_DISPLAY_NO_SUFFIX: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_DISPLAY_PERCENT: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_DISPLAY_PER_SEC: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_DISPLAY_SECONDS: u32 = 805306368u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_ENUM_INSTANCES: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_FILTER: u32 = 9u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub struct PERF_INSTANCE_DEFINITION {
    pub ByteLength: u32,
    pub ParentObjectTitleIndex: u32,
    pub ParentObjectInstance: u32,
    pub UniqueID: i32,
    pub NameOffset: u32,
    pub NameLength: u32,
}
impl ::core::marker::Copy for PERF_INSTANCE_DEFINITION {}
impl ::core::clone::Clone for PERF_INSTANCE_DEFINITION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PERF_INSTANCE_DEFINITION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERF_INSTANCE_DEFINITION").field("ByteLength", &self.ByteLength).field("ParentObjectTitleIndex", &self.ParentObjectTitleIndex).field("ParentObjectInstance", &self.ParentObjectInstance).field("UniqueID", &self.UniqueID).field("NameOffset", &self.NameOffset).field("NameLength", &self.NameLength).finish()
    }
}
unsafe impl ::windows::core::Abi for PERF_INSTANCE_DEFINITION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PERF_INSTANCE_DEFINITION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PERF_INSTANCE_DEFINITION>()) == 0 }
    }
}
impl ::core::cmp::Eq for PERF_INSTANCE_DEFINITION {}
impl ::core::default::Default for PERF_INSTANCE_DEFINITION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub struct PERF_INSTANCE_HEADER {
    pub Size: u32,
    pub InstanceId: u32,
}
impl ::core::marker::Copy for PERF_INSTANCE_HEADER {}
impl ::core::clone::Clone for PERF_INSTANCE_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PERF_INSTANCE_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERF_INSTANCE_HEADER").field("Size", &self.Size).field("InstanceId", &self.InstanceId).finish()
    }
}
unsafe impl ::windows::core::Abi for PERF_INSTANCE_HEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PERF_INSTANCE_HEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PERF_INSTANCE_HEADER>()) == 0 }
    }
}
impl ::core::cmp::Eq for PERF_INSTANCE_HEADER {}
impl ::core::default::Default for PERF_INSTANCE_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_INVERSE_COUNTER: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_MAX_INSTANCE_NAME: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub type PERF_MEM_ALLOC = ::core::option::Option<unsafe extern "system" fn(allocsize: usize, pcontext: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void>;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub type PERF_MEM_FREE = ::core::option::Option<unsafe extern "system" fn(pbuffer: *mut ::core::ffi::c_void, pcontext: *mut ::core::ffi::c_void)>;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_METADATA_MULTIPLE_INSTANCES: i32 = -2i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_METADATA_NO_INSTANCES: i32 = -3i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_MULTI_COUNTER: u32 = 33554432u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub struct PERF_MULTI_COUNTERS {
    pub dwSize: u32,
    pub dwCounters: u32,
}
impl ::core::marker::Copy for PERF_MULTI_COUNTERS {}
impl ::core::clone::Clone for PERF_MULTI_COUNTERS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PERF_MULTI_COUNTERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERF_MULTI_COUNTERS").field("dwSize", &self.dwSize).field("dwCounters", &self.dwCounters).finish()
    }
}
unsafe impl ::windows::core::Abi for PERF_MULTI_COUNTERS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PERF_MULTI_COUNTERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PERF_MULTI_COUNTERS>()) == 0 }
    }
}
impl ::core::cmp::Eq for PERF_MULTI_COUNTERS {}
impl ::core::default::Default for PERF_MULTI_COUNTERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub struct PERF_MULTI_INSTANCES {
    pub dwTotalSize: u32,
    pub dwInstances: u32,
}
impl ::core::marker::Copy for PERF_MULTI_INSTANCES {}
impl ::core::clone::Clone for PERF_MULTI_INSTANCES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PERF_MULTI_INSTANCES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERF_MULTI_INSTANCES").field("dwTotalSize", &self.dwTotalSize).field("dwInstances", &self.dwInstances).finish()
    }
}
unsafe impl ::windows::core::Abi for PERF_MULTI_INSTANCES {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PERF_MULTI_INSTANCES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PERF_MULTI_INSTANCES>()) == 0 }
    }
}
impl ::core::cmp::Eq for PERF_MULTI_INSTANCES {}
impl ::core::default::Default for PERF_MULTI_INSTANCES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_NO_INSTANCES: i32 = -1i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_NO_UNIQUE_ID: i32 = -1i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_NUMBER_DECIMAL: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_NUMBER_DEC_1000: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_NUMBER_HEX: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_OBJECT_TIMER: u32 = 2097152u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct PERF_OBJECT_TYPE {
    pub TotalByteLength: u32,
    pub DefinitionLength: u32,
    pub HeaderLength: u32,
    pub ObjectNameTitleIndex: u32,
    pub ObjectNameTitle: u32,
    pub ObjectHelpTitleIndex: u32,
    pub ObjectHelpTitle: u32,
    pub DetailLevel: u32,
    pub NumCounters: u32,
    pub DefaultCounter: i32,
    pub NumInstances: i32,
    pub CodePage: u32,
    pub PerfTime: i64,
    pub PerfFreq: i64,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for PERF_OBJECT_TYPE {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for PERF_OBJECT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::fmt::Debug for PERF_OBJECT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERF_OBJECT_TYPE")
            .field("TotalByteLength", &self.TotalByteLength)
            .field("DefinitionLength", &self.DefinitionLength)
            .field("HeaderLength", &self.HeaderLength)
            .field("ObjectNameTitleIndex", &self.ObjectNameTitleIndex)
            .field("ObjectNameTitle", &self.ObjectNameTitle)
            .field("ObjectHelpTitleIndex", &self.ObjectHelpTitleIndex)
            .field("ObjectHelpTitle", &self.ObjectHelpTitle)
            .field("DetailLevel", &self.DetailLevel)
            .field("NumCounters", &self.NumCounters)
            .field("DefaultCounter", &self.DefaultCounter)
            .field("NumInstances", &self.NumInstances)
            .field("CodePage", &self.CodePage)
            .field("PerfTime", &self.PerfTime)
            .field("PerfFreq", &self.PerfFreq)
            .finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
unsafe impl ::windows::core::Abi for PERF_OBJECT_TYPE {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for PERF_OBJECT_TYPE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PERF_OBJECT_TYPE>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for PERF_OBJECT_TYPE {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for PERF_OBJECT_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[cfg(target_arch = "x86")]
pub struct PERF_OBJECT_TYPE {
    pub TotalByteLength: u32,
    pub DefinitionLength: u32,
    pub HeaderLength: u32,
    pub ObjectNameTitleIndex: u32,
    pub ObjectNameTitle: ::windows::core::PWSTR,
    pub ObjectHelpTitleIndex: u32,
    pub ObjectHelpTitle: ::windows::core::PWSTR,
    pub DetailLevel: u32,
    pub NumCounters: u32,
    pub DefaultCounter: i32,
    pub NumInstances: i32,
    pub CodePage: u32,
    pub PerfTime: i64,
    pub PerfFreq: i64,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for PERF_OBJECT_TYPE {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for PERF_OBJECT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
impl ::core::fmt::Debug for PERF_OBJECT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERF_OBJECT_TYPE")
            .field("TotalByteLength", &self.TotalByteLength)
            .field("DefinitionLength", &self.DefinitionLength)
            .field("HeaderLength", &self.HeaderLength)
            .field("ObjectNameTitleIndex", &self.ObjectNameTitleIndex)
            .field("ObjectNameTitle", &self.ObjectNameTitle)
            .field("ObjectHelpTitleIndex", &self.ObjectHelpTitleIndex)
            .field("ObjectHelpTitle", &self.ObjectHelpTitle)
            .field("DetailLevel", &self.DetailLevel)
            .field("NumCounters", &self.NumCounters)
            .field("DefaultCounter", &self.DefaultCounter)
            .field("NumInstances", &self.NumInstances)
            .field("CodePage", &self.CodePage)
            .field("PerfTime", &self.PerfTime)
            .field("PerfFreq", &self.PerfFreq)
            .finish()
    }
}
#[cfg(target_arch = "x86")]
unsafe impl ::windows::core::Abi for PERF_OBJECT_TYPE {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for PERF_OBJECT_TYPE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PERF_OBJECT_TYPE>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for PERF_OBJECT_TYPE {}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for PERF_OBJECT_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub struct PERF_PROVIDER_CONTEXT {
    pub ContextSize: u32,
    pub Reserved: u32,
    pub ControlCallback: PERFLIBREQUEST,
    pub MemAllocRoutine: PERF_MEM_ALLOC,
    pub MemFreeRoutine: PERF_MEM_FREE,
    pub pMemContext: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for PERF_PROVIDER_CONTEXT {}
impl ::core::clone::Clone for PERF_PROVIDER_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PERF_PROVIDER_CONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERF_PROVIDER_CONTEXT").field("ContextSize", &self.ContextSize).field("Reserved", &self.Reserved).field("ControlCallback", &self.ControlCallback.map(|f| f as usize)).field("MemAllocRoutine", &self.MemAllocRoutine.map(|f| f as usize)).field("MemFreeRoutine", &self.MemFreeRoutine.map(|f| f as usize)).field("pMemContext", &self.pMemContext).finish()
    }
}
unsafe impl ::windows::core::Abi for PERF_PROVIDER_CONTEXT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PERF_PROVIDER_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PERF_PROVIDER_CONTEXT>()) == 0 }
    }
}
impl ::core::cmp::Eq for PERF_PROVIDER_CONTEXT {}
impl ::core::default::Default for PERF_PROVIDER_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_PROVIDER_DRIVER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_PROVIDER_KERNEL_MODE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_PROVIDER_USER_MODE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_REMOVE_COUNTER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_SIZE_DWORD: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_SIZE_LARGE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_SIZE_VARIABLE_LEN: u32 = 768u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_SIZE_ZERO: u32 = 512u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub struct PERF_STRING_BUFFER_HEADER {
    pub dwSize: u32,
    pub dwCounters: u32,
}
impl ::core::marker::Copy for PERF_STRING_BUFFER_HEADER {}
impl ::core::clone::Clone for PERF_STRING_BUFFER_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PERF_STRING_BUFFER_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERF_STRING_BUFFER_HEADER").field("dwSize", &self.dwSize).field("dwCounters", &self.dwCounters).finish()
    }
}
unsafe impl ::windows::core::Abi for PERF_STRING_BUFFER_HEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PERF_STRING_BUFFER_HEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PERF_STRING_BUFFER_HEADER>()) == 0 }
    }
}
impl ::core::cmp::Eq for PERF_STRING_BUFFER_HEADER {}
impl ::core::default::Default for PERF_STRING_BUFFER_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub struct PERF_STRING_COUNTER_HEADER {
    pub dwCounterId: u32,
    pub dwOffset: u32,
}
impl ::core::marker::Copy for PERF_STRING_COUNTER_HEADER {}
impl ::core::clone::Clone for PERF_STRING_COUNTER_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PERF_STRING_COUNTER_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERF_STRING_COUNTER_HEADER").field("dwCounterId", &self.dwCounterId).field("dwOffset", &self.dwOffset).finish()
    }
}
unsafe impl ::windows::core::Abi for PERF_STRING_COUNTER_HEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PERF_STRING_COUNTER_HEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PERF_STRING_COUNTER_HEADER>()) == 0 }
    }
}
impl ::core::cmp::Eq for PERF_STRING_COUNTER_HEADER {}
impl ::core::default::Default for PERF_STRING_COUNTER_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_TEXT_ASCII: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_TEXT_UNICODE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_TIMER_100NS: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_TIMER_TICK: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_TYPE_COUNTER: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_TYPE_NUMBER: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_TYPE_TEXT: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_TYPE_ZERO: u32 = 3072u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_WILDCARD_COUNTER: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_WILDCARD_INSTANCE: &'static str = "*";
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub type PLA_CABEXTRACT_CALLBACK = ::core::option::Option<unsafe extern "system" fn(filename: ::windows::core::PCWSTR, context: *mut ::core::ffi::c_void)>;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PLA_CAPABILITY_AUTOLOGGER: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PLA_CAPABILITY_LEGACY_SESSION: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PLA_CAPABILITY_LEGACY_SVC: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PLA_CAPABILITY_LOCAL: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PLA_CAPABILITY_V1_SESSION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PLA_CAPABILITY_V1_SVC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PLA_CAPABILITY_V1_SYSTEM: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub type PM_CLOSE_PROC = ::core::option::Option<unsafe extern "system" fn() -> u32>;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub type PM_COLLECT_PROC = ::core::option::Option<unsafe extern "system" fn(pvaluename: ::windows::core::PCWSTR, ppdata: *mut *mut ::core::ffi::c_void, pcbtotalbytes: *mut u32, pnumobjecttypes: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub type PM_OPEN_PROC = ::core::option::Option<unsafe extern "system" fn(pcontext: ::windows::core::PCWSTR) -> u32>;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn PdhAddCounterA<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(hquery: isize, szfullcounterpath: Param1, dwuserdata: usize, phcounter: *mut isize) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhAddCounterA(hquery: isize, szfullcounterpath: ::windows::core::PCSTR, dwuserdata: usize, phcounter: *mut isize) -> i32;
        }
        ::core::mem::transmute(PdhAddCounterA(::core::mem::transmute(hquery), szfullcounterpath.into_param().abi(), ::core::mem::transmute(dwuserdata), ::core::mem::transmute(phcounter)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn PdhAddCounterW<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(hquery: isize, szfullcounterpath: Param1, dwuserdata: usize, phcounter: *mut isize) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhAddCounterW(hquery: isize, szfullcounterpath: ::windows::core::PCWSTR, dwuserdata: usize, phcounter: *mut isize) -> i32;
        }
        ::core::mem::transmute(PdhAddCounterW(::core::mem::transmute(hquery), szfullcounterpath.into_param().abi(), ::core::mem::transmute(dwuserdata), ::core::mem::transmute(phcounter)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn PdhAddEnglishCounterA<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(hquery: isize, szfullcounterpath: Param1, dwuserdata: usize, phcounter: *mut isize) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhAddEnglishCounterA(hquery: isize, szfullcounterpath: ::windows::core::PCSTR, dwuserdata: usize, phcounter: *mut isize) -> i32;
        }
        ::core::mem::transmute(PdhAddEnglishCounterA(::core::mem::transmute(hquery), szfullcounterpath.into_param().abi(), ::core::mem::transmute(dwuserdata), ::core::mem::transmute(phcounter)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn PdhAddEnglishCounterW<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(hquery: isize, szfullcounterpath: Param1, dwuserdata: usize, phcounter: *mut isize) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhAddEnglishCounterW(hquery: isize, szfullcounterpath: ::windows::core::PCWSTR, dwuserdata: usize, phcounter: *mut isize) -> i32;
        }
        ::core::mem::transmute(PdhAddEnglishCounterW(::core::mem::transmute(hquery), szfullcounterpath.into_param().abi(), ::core::mem::transmute(dwuserdata), ::core::mem::transmute(phcounter)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn PdhBindInputDataSourceA<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(phdatasource: *mut isize, logfilenamelist: Param1) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhBindInputDataSourceA(phdatasource: *mut isize, logfilenamelist: ::windows::core::PCSTR) -> i32;
        }
        ::core::mem::transmute(PdhBindInputDataSourceA(::core::mem::transmute(phdatasource), logfilenamelist.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn PdhBindInputDataSourceW<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(phdatasource: *mut isize, logfilenamelist: Param1) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhBindInputDataSourceW(phdatasource: *mut isize, logfilenamelist: ::windows::core::PCWSTR) -> i32;
        }
        ::core::mem::transmute(PdhBindInputDataSourceW(::core::mem::transmute(phdatasource), logfilenamelist.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhBrowseCountersA(pbrowsedlgdata: *const PDH_BROWSE_DLG_CONFIG_A) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhBrowseCountersA(pbrowsedlgdata: *const PDH_BROWSE_DLG_CONFIG_A) -> i32;
        }
        ::core::mem::transmute(PdhBrowseCountersA(::core::mem::transmute(pbrowsedlgdata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhBrowseCountersHA(pbrowsedlgdata: *const PDH_BROWSE_DLG_CONFIG_HA) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhBrowseCountersHA(pbrowsedlgdata: *const PDH_BROWSE_DLG_CONFIG_HA) -> i32;
        }
        ::core::mem::transmute(PdhBrowseCountersHA(::core::mem::transmute(pbrowsedlgdata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhBrowseCountersHW(pbrowsedlgdata: *const PDH_BROWSE_DLG_CONFIG_HW) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhBrowseCountersHW(pbrowsedlgdata: *const PDH_BROWSE_DLG_CONFIG_HW) -> i32;
        }
        ::core::mem::transmute(PdhBrowseCountersHW(::core::mem::transmute(pbrowsedlgdata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhBrowseCountersW(pbrowsedlgdata: *const PDH_BROWSE_DLG_CONFIG_W) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhBrowseCountersW(pbrowsedlgdata: *const PDH_BROWSE_DLG_CONFIG_W) -> i32;
        }
        ::core::mem::transmute(PdhBrowseCountersW(::core::mem::transmute(pbrowsedlgdata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhCalculateCounterFromRawValue(hcounter: isize, dwformat: PDH_FMT, rawvalue1: *const PDH_RAW_COUNTER, rawvalue2: *const PDH_RAW_COUNTER, fmtvalue: *mut PDH_FMT_COUNTERVALUE) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhCalculateCounterFromRawValue(hcounter: isize, dwformat: PDH_FMT, rawvalue1: *const PDH_RAW_COUNTER, rawvalue2: *const PDH_RAW_COUNTER, fmtvalue: *mut PDH_FMT_COUNTERVALUE) -> i32;
        }
        ::core::mem::transmute(PdhCalculateCounterFromRawValue(::core::mem::transmute(hcounter), ::core::mem::transmute(dwformat), ::core::mem::transmute(rawvalue1), ::core::mem::transmute(rawvalue2), ::core::mem::transmute(fmtvalue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn PdhCloseLog(hlog: isize, dwflags: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhCloseLog(hlog: isize, dwflags: u32) -> i32;
        }
        ::core::mem::transmute(PdhCloseLog(::core::mem::transmute(hlog), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn PdhCloseQuery(hquery: isize) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhCloseQuery(hquery: isize) -> i32;
        }
        ::core::mem::transmute(PdhCloseQuery(::core::mem::transmute(hquery)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn PdhCollectQueryData(hquery: isize) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhCollectQueryData(hquery: isize) -> i32;
        }
        ::core::mem::transmute(PdhCollectQueryData(::core::mem::transmute(hquery)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhCollectQueryDataEx<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hquery: isize, dwintervaltime: u32, hnewdataevent: Param2) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhCollectQueryDataEx(hquery: isize, dwintervaltime: u32, hnewdataevent: super::super::Foundation::HANDLE) -> i32;
        }
        ::core::mem::transmute(PdhCollectQueryDataEx(::core::mem::transmute(hquery), ::core::mem::transmute(dwintervaltime), hnewdataevent.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn PdhCollectQueryDataWithTime(hquery: isize, plltimestamp: *mut i64) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhCollectQueryDataWithTime(hquery: isize, plltimestamp: *mut i64) -> i32;
        }
        ::core::mem::transmute(PdhCollectQueryDataWithTime(::core::mem::transmute(hquery), ::core::mem::transmute(plltimestamp)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhComputeCounterStatistics(hcounter: isize, dwformat: PDH_FMT, dwfirstentry: u32, dwnumentries: u32, lprawvaluearray: *const PDH_RAW_COUNTER, data: *mut PDH_STATISTICS) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhComputeCounterStatistics(hcounter: isize, dwformat: PDH_FMT, dwfirstentry: u32, dwnumentries: u32, lprawvaluearray: *const PDH_RAW_COUNTER, data: *mut PDH_STATISTICS) -> i32;
        }
        ::core::mem::transmute(PdhComputeCounterStatistics(::core::mem::transmute(hcounter), ::core::mem::transmute(dwformat), ::core::mem::transmute(dwfirstentry), ::core::mem::transmute(dwnumentries), ::core::mem::transmute(lprawvaluearray), ::core::mem::transmute(data)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn PdhConnectMachineA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(szmachinename: Param0) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhConnectMachineA(szmachinename: ::windows::core::PCSTR) -> i32;
        }
        ::core::mem::transmute(PdhConnectMachineA(szmachinename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn PdhConnectMachineW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(szmachinename: Param0) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhConnectMachineW(szmachinename: ::windows::core::PCWSTR) -> i32;
        }
        ::core::mem::transmute(PdhConnectMachineW(szmachinename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn PdhCreateSQLTablesA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(szdatasource: Param0) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhCreateSQLTablesA(szdatasource: ::windows::core::PCSTR) -> i32;
        }
        ::core::mem::transmute(PdhCreateSQLTablesA(szdatasource.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn PdhCreateSQLTablesW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(szdatasource: Param0) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhCreateSQLTablesW(szdatasource: ::windows::core::PCWSTR) -> i32;
        }
        ::core::mem::transmute(PdhCreateSQLTablesW(szdatasource.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn PdhEnumLogSetNamesA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(szdatasource: Param0, mszdatasetnamelist: ::windows::core::PSTR, pcchbufferlength: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhEnumLogSetNamesA(szdatasource: ::windows::core::PCSTR, mszdatasetnamelist: ::windows::core::PSTR, pcchbufferlength: *mut u32) -> i32;
        }
        ::core::mem::transmute(PdhEnumLogSetNamesA(szdatasource.into_param().abi(), ::core::mem::transmute(mszdatasetnamelist), ::core::mem::transmute(pcchbufferlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn PdhEnumLogSetNamesW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(szdatasource: Param0, mszdatasetnamelist: ::windows::core::PWSTR, pcchbufferlength: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhEnumLogSetNamesW(szdatasource: ::windows::core::PCWSTR, mszdatasetnamelist: ::windows::core::PWSTR, pcchbufferlength: *mut u32) -> i32;
        }
        ::core::mem::transmute(PdhEnumLogSetNamesW(szdatasource.into_param().abi(), ::core::mem::transmute(mszdatasetnamelist), ::core::mem::transmute(pcchbufferlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn PdhEnumMachinesA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(szdatasource: Param0, mszmachinelist: ::windows::core::PSTR, pcchbuffersize: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhEnumMachinesA(szdatasource: ::windows::core::PCSTR, mszmachinelist: ::windows::core::PSTR, pcchbuffersize: *mut u32) -> i32;
        }
        ::core::mem::transmute(PdhEnumMachinesA(szdatasource.into_param().abi(), ::core::mem::transmute(mszmachinelist), ::core::mem::transmute(pcchbuffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn PdhEnumMachinesHA(hdatasource: isize, mszmachinelist: ::windows::core::PSTR, pcchbuffersize: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhEnumMachinesHA(hdatasource: isize, mszmachinelist: ::windows::core::PSTR, pcchbuffersize: *mut u32) -> i32;
        }
        ::core::mem::transmute(PdhEnumMachinesHA(::core::mem::transmute(hdatasource), ::core::mem::transmute(mszmachinelist), ::core::mem::transmute(pcchbuffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn PdhEnumMachinesHW(hdatasource: isize, mszmachinelist: ::windows::core::PWSTR, pcchbuffersize: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhEnumMachinesHW(hdatasource: isize, mszmachinelist: ::windows::core::PWSTR, pcchbuffersize: *mut u32) -> i32;
        }
        ::core::mem::transmute(PdhEnumMachinesHW(::core::mem::transmute(hdatasource), ::core::mem::transmute(mszmachinelist), ::core::mem::transmute(pcchbuffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn PdhEnumMachinesW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(szdatasource: Param0, mszmachinelist: ::windows::core::PWSTR, pcchbuffersize: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhEnumMachinesW(szdatasource: ::windows::core::PCWSTR, mszmachinelist: ::windows::core::PWSTR, pcchbuffersize: *mut u32) -> i32;
        }
        ::core::mem::transmute(PdhEnumMachinesW(szdatasource.into_param().abi(), ::core::mem::transmute(mszmachinelist), ::core::mem::transmute(pcchbuffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn PdhEnumObjectItemsA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(szdatasource: Param0, szmachinename: Param1, szobjectname: Param2, mszcounterlist: ::windows::core::PSTR, pcchcounterlistlength: *mut u32, mszinstancelist: ::windows::core::PSTR, pcchinstancelistlength: *mut u32, dwdetaillevel: PERF_DETAIL, dwflags: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhEnumObjectItemsA(szdatasource: ::windows::core::PCSTR, szmachinename: ::windows::core::PCSTR, szobjectname: ::windows::core::PCSTR, mszcounterlist: ::windows::core::PSTR, pcchcounterlistlength: *mut u32, mszinstancelist: ::windows::core::PSTR, pcchinstancelistlength: *mut u32, dwdetaillevel: PERF_DETAIL, dwflags: u32) -> i32;
        }
        ::core::mem::transmute(PdhEnumObjectItemsA(szdatasource.into_param().abi(), szmachinename.into_param().abi(), szobjectname.into_param().abi(), ::core::mem::transmute(mszcounterlist), ::core::mem::transmute(pcchcounterlistlength), ::core::mem::transmute(mszinstancelist), ::core::mem::transmute(pcchinstancelistlength), ::core::mem::transmute(dwdetaillevel), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn PdhEnumObjectItemsHA<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(hdatasource: isize, szmachinename: Param1, szobjectname: Param2, mszcounterlist: ::windows::core::PSTR, pcchcounterlistlength: *mut u32, mszinstancelist: ::windows::core::PSTR, pcchinstancelistlength: *mut u32, dwdetaillevel: PERF_DETAIL, dwflags: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhEnumObjectItemsHA(hdatasource: isize, szmachinename: ::windows::core::PCSTR, szobjectname: ::windows::core::PCSTR, mszcounterlist: ::windows::core::PSTR, pcchcounterlistlength: *mut u32, mszinstancelist: ::windows::core::PSTR, pcchinstancelistlength: *mut u32, dwdetaillevel: PERF_DETAIL, dwflags: u32) -> i32;
        }
        ::core::mem::transmute(PdhEnumObjectItemsHA(::core::mem::transmute(hdatasource), szmachinename.into_param().abi(), szobjectname.into_param().abi(), ::core::mem::transmute(mszcounterlist), ::core::mem::transmute(pcchcounterlistlength), ::core::mem::transmute(mszinstancelist), ::core::mem::transmute(pcchinstancelistlength), ::core::mem::transmute(dwdetaillevel), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn PdhEnumObjectItemsHW<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(hdatasource: isize, szmachinename: Param1, szobjectname: Param2, mszcounterlist: ::windows::core::PWSTR, pcchcounterlistlength: *mut u32, mszinstancelist: ::windows::core::PWSTR, pcchinstancelistlength: *mut u32, dwdetaillevel: PERF_DETAIL, dwflags: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhEnumObjectItemsHW(hdatasource: isize, szmachinename: ::windows::core::PCWSTR, szobjectname: ::windows::core::PCWSTR, mszcounterlist: ::windows::core::PWSTR, pcchcounterlistlength: *mut u32, mszinstancelist: ::windows::core::PWSTR, pcchinstancelistlength: *mut u32, dwdetaillevel: PERF_DETAIL, dwflags: u32) -> i32;
        }
        ::core::mem::transmute(PdhEnumObjectItemsHW(::core::mem::transmute(hdatasource), szmachinename.into_param().abi(), szobjectname.into_param().abi(), ::core::mem::transmute(mszcounterlist), ::core::mem::transmute(pcchcounterlistlength), ::core::mem::transmute(mszinstancelist), ::core::mem::transmute(pcchinstancelistlength), ::core::mem::transmute(dwdetaillevel), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn PdhEnumObjectItemsW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(szdatasource: Param0, szmachinename: Param1, szobjectname: Param2, mszcounterlist: ::windows::core::PWSTR, pcchcounterlistlength: *mut u32, mszinstancelist: ::windows::core::PWSTR, pcchinstancelistlength: *mut u32, dwdetaillevel: PERF_DETAIL, dwflags: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhEnumObjectItemsW(szdatasource: ::windows::core::PCWSTR, szmachinename: ::windows::core::PCWSTR, szobjectname: ::windows::core::PCWSTR, mszcounterlist: ::windows::core::PWSTR, pcchcounterlistlength: *mut u32, mszinstancelist: ::windows::core::PWSTR, pcchinstancelistlength: *mut u32, dwdetaillevel: PERF_DETAIL, dwflags: u32) -> i32;
        }
        ::core::mem::transmute(PdhEnumObjectItemsW(szdatasource.into_param().abi(), szmachinename.into_param().abi(), szobjectname.into_param().abi(), ::core::mem::transmute(mszcounterlist), ::core::mem::transmute(pcchcounterlistlength), ::core::mem::transmute(mszinstancelist), ::core::mem::transmute(pcchinstancelistlength), ::core::mem::transmute(dwdetaillevel), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhEnumObjectsA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(szdatasource: Param0, szmachinename: Param1, mszobjectlist: ::windows::core::PSTR, pcchbuffersize: *mut u32, dwdetaillevel: PERF_DETAIL, brefresh: Param5) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhEnumObjectsA(szdatasource: ::windows::core::PCSTR, szmachinename: ::windows::core::PCSTR, mszobjectlist: ::windows::core::PSTR, pcchbuffersize: *mut u32, dwdetaillevel: PERF_DETAIL, brefresh: super::super::Foundation::BOOL) -> i32;
        }
        ::core::mem::transmute(PdhEnumObjectsA(szdatasource.into_param().abi(), szmachinename.into_param().abi(), ::core::mem::transmute(mszobjectlist), ::core::mem::transmute(pcchbuffersize), ::core::mem::transmute(dwdetaillevel), brefresh.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhEnumObjectsHA<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(hdatasource: isize, szmachinename: Param1, mszobjectlist: ::windows::core::PSTR, pcchbuffersize: *mut u32, dwdetaillevel: PERF_DETAIL, brefresh: Param5) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhEnumObjectsHA(hdatasource: isize, szmachinename: ::windows::core::PCSTR, mszobjectlist: ::windows::core::PSTR, pcchbuffersize: *mut u32, dwdetaillevel: PERF_DETAIL, brefresh: super::super::Foundation::BOOL) -> i32;
        }
        ::core::mem::transmute(PdhEnumObjectsHA(::core::mem::transmute(hdatasource), szmachinename.into_param().abi(), ::core::mem::transmute(mszobjectlist), ::core::mem::transmute(pcchbuffersize), ::core::mem::transmute(dwdetaillevel), brefresh.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhEnumObjectsHW<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(hdatasource: isize, szmachinename: Param1, mszobjectlist: ::windows::core::PWSTR, pcchbuffersize: *mut u32, dwdetaillevel: PERF_DETAIL, brefresh: Param5) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhEnumObjectsHW(hdatasource: isize, szmachinename: ::windows::core::PCWSTR, mszobjectlist: ::windows::core::PWSTR, pcchbuffersize: *mut u32, dwdetaillevel: PERF_DETAIL, brefresh: super::super::Foundation::BOOL) -> i32;
        }
        ::core::mem::transmute(PdhEnumObjectsHW(::core::mem::transmute(hdatasource), szmachinename.into_param().abi(), ::core::mem::transmute(mszobjectlist), ::core::mem::transmute(pcchbuffersize), ::core::mem::transmute(dwdetaillevel), brefresh.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhEnumObjectsW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(szdatasource: Param0, szmachinename: Param1, mszobjectlist: ::windows::core::PWSTR, pcchbuffersize: *mut u32, dwdetaillevel: PERF_DETAIL, brefresh: Param5) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhEnumObjectsW(szdatasource: ::windows::core::PCWSTR, szmachinename: ::windows::core::PCWSTR, mszobjectlist: ::windows::core::PWSTR, pcchbuffersize: *mut u32, dwdetaillevel: PERF_DETAIL, brefresh: super::super::Foundation::BOOL) -> i32;
        }
        ::core::mem::transmute(PdhEnumObjectsW(szdatasource.into_param().abi(), szmachinename.into_param().abi(), ::core::mem::transmute(mszobjectlist), ::core::mem::transmute(pcchbuffersize), ::core::mem::transmute(dwdetaillevel), brefresh.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn PdhExpandCounterPathA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(szwildcardpath: Param0, mszexpandedpathlist: ::windows::core::PSTR, pcchpathlistlength: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhExpandCounterPathA(szwildcardpath: ::windows::core::PCSTR, mszexpandedpathlist: ::windows::core::PSTR, pcchpathlistlength: *mut u32) -> i32;
        }
        ::core::mem::transmute(PdhExpandCounterPathA(szwildcardpath.into_param().abi(), ::core::mem::transmute(mszexpandedpathlist), ::core::mem::transmute(pcchpathlistlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn PdhExpandCounterPathW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(szwildcardpath: Param0, mszexpandedpathlist: ::windows::core::PWSTR, pcchpathlistlength: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhExpandCounterPathW(szwildcardpath: ::windows::core::PCWSTR, mszexpandedpathlist: ::windows::core::PWSTR, pcchpathlistlength: *mut u32) -> i32;
        }
        ::core::mem::transmute(PdhExpandCounterPathW(szwildcardpath.into_param().abi(), ::core::mem::transmute(mszexpandedpathlist), ::core::mem::transmute(pcchpathlistlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn PdhExpandWildCardPathA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(szdatasource: Param0, szwildcardpath: Param1, mszexpandedpathlist: ::windows::core::PSTR, pcchpathlistlength: *mut u32, dwflags: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhExpandWildCardPathA(szdatasource: ::windows::core::PCSTR, szwildcardpath: ::windows::core::PCSTR, mszexpandedpathlist: ::windows::core::PSTR, pcchpathlistlength: *mut u32, dwflags: u32) -> i32;
        }
        ::core::mem::transmute(PdhExpandWildCardPathA(szdatasource.into_param().abi(), szwildcardpath.into_param().abi(), ::core::mem::transmute(mszexpandedpathlist), ::core::mem::transmute(pcchpathlistlength), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn PdhExpandWildCardPathHA<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(hdatasource: isize, szwildcardpath: Param1, mszexpandedpathlist: ::windows::core::PSTR, pcchpathlistlength: *mut u32, dwflags: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhExpandWildCardPathHA(hdatasource: isize, szwildcardpath: ::windows::core::PCSTR, mszexpandedpathlist: ::windows::core::PSTR, pcchpathlistlength: *mut u32, dwflags: u32) -> i32;
        }
        ::core::mem::transmute(PdhExpandWildCardPathHA(::core::mem::transmute(hdatasource), szwildcardpath.into_param().abi(), ::core::mem::transmute(mszexpandedpathlist), ::core::mem::transmute(pcchpathlistlength), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn PdhExpandWildCardPathHW<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(hdatasource: isize, szwildcardpath: Param1, mszexpandedpathlist: ::windows::core::PWSTR, pcchpathlistlength: *mut u32, dwflags: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhExpandWildCardPathHW(hdatasource: isize, szwildcardpath: ::windows::core::PCWSTR, mszexpandedpathlist: ::windows::core::PWSTR, pcchpathlistlength: *mut u32, dwflags: u32) -> i32;
        }
        ::core::mem::transmute(PdhExpandWildCardPathHW(::core::mem::transmute(hdatasource), szwildcardpath.into_param().abi(), ::core::mem::transmute(mszexpandedpathlist), ::core::mem::transmute(pcchpathlistlength), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn PdhExpandWildCardPathW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(szdatasource: Param0, szwildcardpath: Param1, mszexpandedpathlist: ::windows::core::PWSTR, pcchpathlistlength: *mut u32, dwflags: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhExpandWildCardPathW(szdatasource: ::windows::core::PCWSTR, szwildcardpath: ::windows::core::PCWSTR, mszexpandedpathlist: ::windows::core::PWSTR, pcchpathlistlength: *mut u32, dwflags: u32) -> i32;
        }
        ::core::mem::transmute(PdhExpandWildCardPathW(szdatasource.into_param().abi(), szwildcardpath.into_param().abi(), ::core::mem::transmute(mszexpandedpathlist), ::core::mem::transmute(pcchpathlistlength), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhFormatFromRawValue(dwcountertype: u32, dwformat: PDH_FMT, ptimebase: *const i64, prawvalue1: *const PDH_RAW_COUNTER, prawvalue2: *const PDH_RAW_COUNTER, pfmtvalue: *mut PDH_FMT_COUNTERVALUE) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhFormatFromRawValue(dwcountertype: u32, dwformat: PDH_FMT, ptimebase: *const i64, prawvalue1: *const PDH_RAW_COUNTER, prawvalue2: *const PDH_RAW_COUNTER, pfmtvalue: *mut PDH_FMT_COUNTERVALUE) -> i32;
        }
        ::core::mem::transmute(PdhFormatFromRawValue(::core::mem::transmute(dwcountertype), ::core::mem::transmute(dwformat), ::core::mem::transmute(ptimebase), ::core::mem::transmute(prawvalue1), ::core::mem::transmute(prawvalue2), ::core::mem::transmute(pfmtvalue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhGetCounterInfoA<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOLEAN>>(hcounter: isize, bretrieveexplaintext: Param1, pdwbuffersize: *mut u32, lpbuffer: *mut PDH_COUNTER_INFO_A) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhGetCounterInfoA(hcounter: isize, bretrieveexplaintext: super::super::Foundation::BOOLEAN, pdwbuffersize: *mut u32, lpbuffer: *mut PDH_COUNTER_INFO_A) -> i32;
        }
        ::core::mem::transmute(PdhGetCounterInfoA(::core::mem::transmute(hcounter), bretrieveexplaintext.into_param().abi(), ::core::mem::transmute(pdwbuffersize), ::core::mem::transmute(lpbuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhGetCounterInfoW<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOLEAN>>(hcounter: isize, bretrieveexplaintext: Param1, pdwbuffersize: *mut u32, lpbuffer: *mut PDH_COUNTER_INFO_W) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhGetCounterInfoW(hcounter: isize, bretrieveexplaintext: super::super::Foundation::BOOLEAN, pdwbuffersize: *mut u32, lpbuffer: *mut PDH_COUNTER_INFO_W) -> i32;
        }
        ::core::mem::transmute(PdhGetCounterInfoW(::core::mem::transmute(hcounter), bretrieveexplaintext.into_param().abi(), ::core::mem::transmute(pdwbuffersize), ::core::mem::transmute(lpbuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn PdhGetCounterTimeBase(hcounter: isize, ptimebase: *mut i64) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhGetCounterTimeBase(hcounter: isize, ptimebase: *mut i64) -> i32;
        }
        ::core::mem::transmute(PdhGetCounterTimeBase(::core::mem::transmute(hcounter), ::core::mem::transmute(ptimebase)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn PdhGetDataSourceTimeRangeA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(szdatasource: Param0, pdwnumentries: *mut u32, pinfo: *mut PDH_TIME_INFO, pdwbuffersize: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhGetDataSourceTimeRangeA(szdatasource: ::windows::core::PCSTR, pdwnumentries: *mut u32, pinfo: *mut PDH_TIME_INFO, pdwbuffersize: *mut u32) -> i32;
        }
        ::core::mem::transmute(PdhGetDataSourceTimeRangeA(szdatasource.into_param().abi(), ::core::mem::transmute(pdwnumentries), ::core::mem::transmute(pinfo), ::core::mem::transmute(pdwbuffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn PdhGetDataSourceTimeRangeH(hdatasource: isize, pdwnumentries: *mut u32, pinfo: *mut PDH_TIME_INFO, pdwbuffersize: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhGetDataSourceTimeRangeH(hdatasource: isize, pdwnumentries: *mut u32, pinfo: *mut PDH_TIME_INFO, pdwbuffersize: *mut u32) -> i32;
        }
        ::core::mem::transmute(PdhGetDataSourceTimeRangeH(::core::mem::transmute(hdatasource), ::core::mem::transmute(pdwnumentries), ::core::mem::transmute(pinfo), ::core::mem::transmute(pdwbuffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn PdhGetDataSourceTimeRangeW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(szdatasource: Param0, pdwnumentries: *mut u32, pinfo: *mut PDH_TIME_INFO, pdwbuffersize: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhGetDataSourceTimeRangeW(szdatasource: ::windows::core::PCWSTR, pdwnumentries: *mut u32, pinfo: *mut PDH_TIME_INFO, pdwbuffersize: *mut u32) -> i32;
        }
        ::core::mem::transmute(PdhGetDataSourceTimeRangeW(szdatasource.into_param().abi(), ::core::mem::transmute(pdwnumentries), ::core::mem::transmute(pinfo), ::core::mem::transmute(pdwbuffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn PdhGetDefaultPerfCounterA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(szdatasource: Param0, szmachinename: Param1, szobjectname: Param2, szdefaultcountername: ::windows::core::PSTR, pcchbuffersize: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhGetDefaultPerfCounterA(szdatasource: ::windows::core::PCSTR, szmachinename: ::windows::core::PCSTR, szobjectname: ::windows::core::PCSTR, szdefaultcountername: ::windows::core::PSTR, pcchbuffersize: *mut u32) -> i32;
        }
        ::core::mem::transmute(PdhGetDefaultPerfCounterA(szdatasource.into_param().abi(), szmachinename.into_param().abi(), szobjectname.into_param().abi(), ::core::mem::transmute(szdefaultcountername), ::core::mem::transmute(pcchbuffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn PdhGetDefaultPerfCounterHA<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(hdatasource: isize, szmachinename: Param1, szobjectname: Param2, szdefaultcountername: ::windows::core::PSTR, pcchbuffersize: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhGetDefaultPerfCounterHA(hdatasource: isize, szmachinename: ::windows::core::PCSTR, szobjectname: ::windows::core::PCSTR, szdefaultcountername: ::windows::core::PSTR, pcchbuffersize: *mut u32) -> i32;
        }
        ::core::mem::transmute(PdhGetDefaultPerfCounterHA(::core::mem::transmute(hdatasource), szmachinename.into_param().abi(), szobjectname.into_param().abi(), ::core::mem::transmute(szdefaultcountername), ::core::mem::transmute(pcchbuffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn PdhGetDefaultPerfCounterHW<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(hdatasource: isize, szmachinename: Param1, szobjectname: Param2, szdefaultcountername: ::windows::core::PWSTR, pcchbuffersize: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhGetDefaultPerfCounterHW(hdatasource: isize, szmachinename: ::windows::core::PCWSTR, szobjectname: ::windows::core::PCWSTR, szdefaultcountername: ::windows::core::PWSTR, pcchbuffersize: *mut u32) -> i32;
        }
        ::core::mem::transmute(PdhGetDefaultPerfCounterHW(::core::mem::transmute(hdatasource), szmachinename.into_param().abi(), szobjectname.into_param().abi(), ::core::mem::transmute(szdefaultcountername), ::core::mem::transmute(pcchbuffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn PdhGetDefaultPerfCounterW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(szdatasource: Param0, szmachinename: Param1, szobjectname: Param2, szdefaultcountername: ::windows::core::PWSTR, pcchbuffersize: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhGetDefaultPerfCounterW(szdatasource: ::windows::core::PCWSTR, szmachinename: ::windows::core::PCWSTR, szobjectname: ::windows::core::PCWSTR, szdefaultcountername: ::windows::core::PWSTR, pcchbuffersize: *mut u32) -> i32;
        }
        ::core::mem::transmute(PdhGetDefaultPerfCounterW(szdatasource.into_param().abi(), szmachinename.into_param().abi(), szobjectname.into_param().abi(), ::core::mem::transmute(szdefaultcountername), ::core::mem::transmute(pcchbuffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn PdhGetDefaultPerfObjectA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(szdatasource: Param0, szmachinename: Param1, szdefaultobjectname: ::windows::core::PSTR, pcchbuffersize: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhGetDefaultPerfObjectA(szdatasource: ::windows::core::PCSTR, szmachinename: ::windows::core::PCSTR, szdefaultobjectname: ::windows::core::PSTR, pcchbuffersize: *mut u32) -> i32;
        }
        ::core::mem::transmute(PdhGetDefaultPerfObjectA(szdatasource.into_param().abi(), szmachinename.into_param().abi(), ::core::mem::transmute(szdefaultobjectname), ::core::mem::transmute(pcchbuffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn PdhGetDefaultPerfObjectHA<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(hdatasource: isize, szmachinename: Param1, szdefaultobjectname: ::windows::core::PSTR, pcchbuffersize: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhGetDefaultPerfObjectHA(hdatasource: isize, szmachinename: ::windows::core::PCSTR, szdefaultobjectname: ::windows::core::PSTR, pcchbuffersize: *mut u32) -> i32;
        }
        ::core::mem::transmute(PdhGetDefaultPerfObjectHA(::core::mem::transmute(hdatasource), szmachinename.into_param().abi(), ::core::mem::transmute(szdefaultobjectname), ::core::mem::transmute(pcchbuffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn PdhGetDefaultPerfObjectHW<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(hdatasource: isize, szmachinename: Param1, szdefaultobjectname: ::windows::core::PWSTR, pcchbuffersize: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhGetDefaultPerfObjectHW(hdatasource: isize, szmachinename: ::windows::core::PCWSTR, szdefaultobjectname: ::windows::core::PWSTR, pcchbuffersize: *mut u32) -> i32;
        }
        ::core::mem::transmute(PdhGetDefaultPerfObjectHW(::core::mem::transmute(hdatasource), szmachinename.into_param().abi(), ::core::mem::transmute(szdefaultobjectname), ::core::mem::transmute(pcchbuffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn PdhGetDefaultPerfObjectW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(szdatasource: Param0, szmachinename: Param1, szdefaultobjectname: ::windows::core::PWSTR, pcchbuffersize: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhGetDefaultPerfObjectW(szdatasource: ::windows::core::PCWSTR, szmachinename: ::windows::core::PCWSTR, szdefaultobjectname: ::windows::core::PWSTR, pcchbuffersize: *mut u32) -> i32;
        }
        ::core::mem::transmute(PdhGetDefaultPerfObjectW(szdatasource.into_param().abi(), szmachinename.into_param().abi(), ::core::mem::transmute(szdefaultobjectname), ::core::mem::transmute(pcchbuffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn PdhGetDllVersion(lpdwversion: *mut PDH_DLL_VERSION) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhGetDllVersion(lpdwversion: *mut PDH_DLL_VERSION) -> i32;
        }
        ::core::mem::transmute(PdhGetDllVersion(::core::mem::transmute(lpdwversion)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn PdhGetFormattedCounterArrayA(hcounter: isize, dwformat: PDH_FMT, lpdwbuffersize: *mut u32, lpdwitemcount: *mut u32, itembuffer: *mut PDH_FMT_COUNTERVALUE_ITEM_A) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhGetFormattedCounterArrayA(hcounter: isize, dwformat: PDH_FMT, lpdwbuffersize: *mut u32, lpdwitemcount: *mut u32, itembuffer: *mut PDH_FMT_COUNTERVALUE_ITEM_A) -> i32;
        }
        ::core::mem::transmute(PdhGetFormattedCounterArrayA(::core::mem::transmute(hcounter), ::core::mem::transmute(dwformat), ::core::mem::transmute(lpdwbuffersize), ::core::mem::transmute(lpdwitemcount), ::core::mem::transmute(itembuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn PdhGetFormattedCounterArrayW(hcounter: isize, dwformat: PDH_FMT, lpdwbuffersize: *mut u32, lpdwitemcount: *mut u32, itembuffer: *mut PDH_FMT_COUNTERVALUE_ITEM_W) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhGetFormattedCounterArrayW(hcounter: isize, dwformat: PDH_FMT, lpdwbuffersize: *mut u32, lpdwitemcount: *mut u32, itembuffer: *mut PDH_FMT_COUNTERVALUE_ITEM_W) -> i32;
        }
        ::core::mem::transmute(PdhGetFormattedCounterArrayW(::core::mem::transmute(hcounter), ::core::mem::transmute(dwformat), ::core::mem::transmute(lpdwbuffersize), ::core::mem::transmute(lpdwitemcount), ::core::mem::transmute(itembuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn PdhGetFormattedCounterValue(hcounter: isize, dwformat: PDH_FMT, lpdwtype: *mut u32, pvalue: *mut PDH_FMT_COUNTERVALUE) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhGetFormattedCounterValue(hcounter: isize, dwformat: PDH_FMT, lpdwtype: *mut u32, pvalue: *mut PDH_FMT_COUNTERVALUE) -> i32;
        }
        ::core::mem::transmute(PdhGetFormattedCounterValue(::core::mem::transmute(hcounter), ::core::mem::transmute(dwformat), ::core::mem::transmute(lpdwtype), ::core::mem::transmute(pvalue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn PdhGetLogFileSize(hlog: isize, llsize: *mut i64) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhGetLogFileSize(hlog: isize, llsize: *mut i64) -> i32;
        }
        ::core::mem::transmute(PdhGetLogFileSize(::core::mem::transmute(hlog), ::core::mem::transmute(llsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn PdhGetLogSetGUID(hlog: isize, pguid: *mut ::windows::core::GUID, prunid: *mut i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhGetLogSetGUID(hlog: isize, pguid: *mut ::windows::core::GUID, prunid: *mut i32) -> i32;
        }
        ::core::mem::transmute(PdhGetLogSetGUID(::core::mem::transmute(hlog), ::core::mem::transmute(pguid), ::core::mem::transmute(prunid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhGetRawCounterArrayA(hcounter: isize, lpdwbuffersize: *mut u32, lpdwitemcount: *mut u32, itembuffer: *mut PDH_RAW_COUNTER_ITEM_A) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhGetRawCounterArrayA(hcounter: isize, lpdwbuffersize: *mut u32, lpdwitemcount: *mut u32, itembuffer: *mut PDH_RAW_COUNTER_ITEM_A) -> i32;
        }
        ::core::mem::transmute(PdhGetRawCounterArrayA(::core::mem::transmute(hcounter), ::core::mem::transmute(lpdwbuffersize), ::core::mem::transmute(lpdwitemcount), ::core::mem::transmute(itembuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhGetRawCounterArrayW(hcounter: isize, lpdwbuffersize: *mut u32, lpdwitemcount: *mut u32, itembuffer: *mut PDH_RAW_COUNTER_ITEM_W) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhGetRawCounterArrayW(hcounter: isize, lpdwbuffersize: *mut u32, lpdwitemcount: *mut u32, itembuffer: *mut PDH_RAW_COUNTER_ITEM_W) -> i32;
        }
        ::core::mem::transmute(PdhGetRawCounterArrayW(::core::mem::transmute(hcounter), ::core::mem::transmute(lpdwbuffersize), ::core::mem::transmute(lpdwitemcount), ::core::mem::transmute(itembuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhGetRawCounterValue(hcounter: isize, lpdwtype: *mut u32, pvalue: *mut PDH_RAW_COUNTER) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhGetRawCounterValue(hcounter: isize, lpdwtype: *mut u32, pvalue: *mut PDH_RAW_COUNTER) -> i32;
        }
        ::core::mem::transmute(PdhGetRawCounterValue(::core::mem::transmute(hcounter), ::core::mem::transmute(lpdwtype), ::core::mem::transmute(pvalue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhIsRealTimeQuery(hquery: isize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhIsRealTimeQuery(hquery: isize) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(PdhIsRealTimeQuery(::core::mem::transmute(hquery)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn PdhLookupPerfIndexByNameA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(szmachinename: Param0, sznamebuffer: Param1, pdwindex: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhLookupPerfIndexByNameA(szmachinename: ::windows::core::PCSTR, sznamebuffer: ::windows::core::PCSTR, pdwindex: *mut u32) -> i32;
        }
        ::core::mem::transmute(PdhLookupPerfIndexByNameA(szmachinename.into_param().abi(), sznamebuffer.into_param().abi(), ::core::mem::transmute(pdwindex)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn PdhLookupPerfIndexByNameW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(szmachinename: Param0, sznamebuffer: Param1, pdwindex: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhLookupPerfIndexByNameW(szmachinename: ::windows::core::PCWSTR, sznamebuffer: ::windows::core::PCWSTR, pdwindex: *mut u32) -> i32;
        }
        ::core::mem::transmute(PdhLookupPerfIndexByNameW(szmachinename.into_param().abi(), sznamebuffer.into_param().abi(), ::core::mem::transmute(pdwindex)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn PdhLookupPerfNameByIndexA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(szmachinename: Param0, dwnameindex: u32, sznamebuffer: ::windows::core::PSTR, pcchnamebuffersize: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhLookupPerfNameByIndexA(szmachinename: ::windows::core::PCSTR, dwnameindex: u32, sznamebuffer: ::windows::core::PSTR, pcchnamebuffersize: *mut u32) -> i32;
        }
        ::core::mem::transmute(PdhLookupPerfNameByIndexA(szmachinename.into_param().abi(), ::core::mem::transmute(dwnameindex), ::core::mem::transmute(sznamebuffer), ::core::mem::transmute(pcchnamebuffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn PdhLookupPerfNameByIndexW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(szmachinename: Param0, dwnameindex: u32, sznamebuffer: ::windows::core::PWSTR, pcchnamebuffersize: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhLookupPerfNameByIndexW(szmachinename: ::windows::core::PCWSTR, dwnameindex: u32, sznamebuffer: ::windows::core::PWSTR, pcchnamebuffersize: *mut u32) -> i32;
        }
        ::core::mem::transmute(PdhLookupPerfNameByIndexW(szmachinename.into_param().abi(), ::core::mem::transmute(dwnameindex), ::core::mem::transmute(sznamebuffer), ::core::mem::transmute(pcchnamebuffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn PdhMakeCounterPathA(pcounterpathelements: *const PDH_COUNTER_PATH_ELEMENTS_A, szfullpathbuffer: ::windows::core::PSTR, pcchbuffersize: *mut u32, dwflags: PDH_PATH_FLAGS) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhMakeCounterPathA(pcounterpathelements: *const PDH_COUNTER_PATH_ELEMENTS_A, szfullpathbuffer: ::windows::core::PSTR, pcchbuffersize: *mut u32, dwflags: PDH_PATH_FLAGS) -> i32;
        }
        ::core::mem::transmute(PdhMakeCounterPathA(::core::mem::transmute(pcounterpathelements), ::core::mem::transmute(szfullpathbuffer), ::core::mem::transmute(pcchbuffersize), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn PdhMakeCounterPathW(pcounterpathelements: *const PDH_COUNTER_PATH_ELEMENTS_W, szfullpathbuffer: ::windows::core::PWSTR, pcchbuffersize: *mut u32, dwflags: PDH_PATH_FLAGS) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhMakeCounterPathW(pcounterpathelements: *const PDH_COUNTER_PATH_ELEMENTS_W, szfullpathbuffer: ::windows::core::PWSTR, pcchbuffersize: *mut u32, dwflags: PDH_PATH_FLAGS) -> i32;
        }
        ::core::mem::transmute(PdhMakeCounterPathW(::core::mem::transmute(pcounterpathelements), ::core::mem::transmute(szfullpathbuffer), ::core::mem::transmute(pcchbuffersize), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn PdhOpenLogA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param5: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(szlogfilename: Param0, dwaccessflags: PDH_LOG, lpdwlogtype: *mut PDH_LOG_TYPE, hquery: isize, dwmaxsize: u32, szusercaption: Param5, phlog: *mut isize) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhOpenLogA(szlogfilename: ::windows::core::PCSTR, dwaccessflags: PDH_LOG, lpdwlogtype: *mut PDH_LOG_TYPE, hquery: isize, dwmaxsize: u32, szusercaption: ::windows::core::PCSTR, phlog: *mut isize) -> i32;
        }
        ::core::mem::transmute(PdhOpenLogA(szlogfilename.into_param().abi(), ::core::mem::transmute(dwaccessflags), ::core::mem::transmute(lpdwlogtype), ::core::mem::transmute(hquery), ::core::mem::transmute(dwmaxsize), szusercaption.into_param().abi(), ::core::mem::transmute(phlog)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn PdhOpenLogW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param5: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(szlogfilename: Param0, dwaccessflags: PDH_LOG, lpdwlogtype: *mut PDH_LOG_TYPE, hquery: isize, dwmaxsize: u32, szusercaption: Param5, phlog: *mut isize) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhOpenLogW(szlogfilename: ::windows::core::PCWSTR, dwaccessflags: PDH_LOG, lpdwlogtype: *mut PDH_LOG_TYPE, hquery: isize, dwmaxsize: u32, szusercaption: ::windows::core::PCWSTR, phlog: *mut isize) -> i32;
        }
        ::core::mem::transmute(PdhOpenLogW(szlogfilename.into_param().abi(), ::core::mem::transmute(dwaccessflags), ::core::mem::transmute(lpdwlogtype), ::core::mem::transmute(hquery), ::core::mem::transmute(dwmaxsize), szusercaption.into_param().abi(), ::core::mem::transmute(phlog)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn PdhOpenQueryA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(szdatasource: Param0, dwuserdata: usize, phquery: *mut isize) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhOpenQueryA(szdatasource: ::windows::core::PCSTR, dwuserdata: usize, phquery: *mut isize) -> i32;
        }
        ::core::mem::transmute(PdhOpenQueryA(szdatasource.into_param().abi(), ::core::mem::transmute(dwuserdata), ::core::mem::transmute(phquery)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn PdhOpenQueryH(hdatasource: isize, dwuserdata: usize, phquery: *mut isize) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhOpenQueryH(hdatasource: isize, dwuserdata: usize, phquery: *mut isize) -> i32;
        }
        ::core::mem::transmute(PdhOpenQueryH(::core::mem::transmute(hdatasource), ::core::mem::transmute(dwuserdata), ::core::mem::transmute(phquery)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn PdhOpenQueryW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(szdatasource: Param0, dwuserdata: usize, phquery: *mut isize) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhOpenQueryW(szdatasource: ::windows::core::PCWSTR, dwuserdata: usize, phquery: *mut isize) -> i32;
        }
        ::core::mem::transmute(PdhOpenQueryW(szdatasource.into_param().abi(), ::core::mem::transmute(dwuserdata), ::core::mem::transmute(phquery)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn PdhParseCounterPathA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(szfullpathbuffer: Param0, pcounterpathelements: *mut PDH_COUNTER_PATH_ELEMENTS_A, pdwbuffersize: *mut u32, dwflags: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhParseCounterPathA(szfullpathbuffer: ::windows::core::PCSTR, pcounterpathelements: *mut PDH_COUNTER_PATH_ELEMENTS_A, pdwbuffersize: *mut u32, dwflags: u32) -> i32;
        }
        ::core::mem::transmute(PdhParseCounterPathA(szfullpathbuffer.into_param().abi(), ::core::mem::transmute(pcounterpathelements), ::core::mem::transmute(pdwbuffersize), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn PdhParseCounterPathW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(szfullpathbuffer: Param0, pcounterpathelements: *mut PDH_COUNTER_PATH_ELEMENTS_W, pdwbuffersize: *mut u32, dwflags: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhParseCounterPathW(szfullpathbuffer: ::windows::core::PCWSTR, pcounterpathelements: *mut PDH_COUNTER_PATH_ELEMENTS_W, pdwbuffersize: *mut u32, dwflags: u32) -> i32;
        }
        ::core::mem::transmute(PdhParseCounterPathW(szfullpathbuffer.into_param().abi(), ::core::mem::transmute(pcounterpathelements), ::core::mem::transmute(pdwbuffersize), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn PdhParseInstanceNameA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(szinstancestring: Param0, szinstancename: ::windows::core::PSTR, pcchinstancenamelength: *mut u32, szparentname: ::windows::core::PSTR, pcchparentnamelength: *mut u32, lpindex: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhParseInstanceNameA(szinstancestring: ::windows::core::PCSTR, szinstancename: ::windows::core::PSTR, pcchinstancenamelength: *mut u32, szparentname: ::windows::core::PSTR, pcchparentnamelength: *mut u32, lpindex: *mut u32) -> i32;
        }
        ::core::mem::transmute(PdhParseInstanceNameA(szinstancestring.into_param().abi(), ::core::mem::transmute(szinstancename), ::core::mem::transmute(pcchinstancenamelength), ::core::mem::transmute(szparentname), ::core::mem::transmute(pcchparentnamelength), ::core::mem::transmute(lpindex)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn PdhParseInstanceNameW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(szinstancestring: Param0, szinstancename: ::windows::core::PWSTR, pcchinstancenamelength: *mut u32, szparentname: ::windows::core::PWSTR, pcchparentnamelength: *mut u32, lpindex: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhParseInstanceNameW(szinstancestring: ::windows::core::PCWSTR, szinstancename: ::windows::core::PWSTR, pcchinstancenamelength: *mut u32, szparentname: ::windows::core::PWSTR, pcchparentnamelength: *mut u32, lpindex: *mut u32) -> i32;
        }
        ::core::mem::transmute(PdhParseInstanceNameW(szinstancestring.into_param().abi(), ::core::mem::transmute(szinstancename), ::core::mem::transmute(pcchinstancenamelength), ::core::mem::transmute(szparentname), ::core::mem::transmute(pcchparentnamelength), ::core::mem::transmute(lpindex)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhReadRawLogRecord<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::FILETIME>>(hlog: isize, ftrecord: Param1, prawlogrecord: *mut PDH_RAW_LOG_RECORD, pdwbufferlength: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhReadRawLogRecord(hlog: isize, ftrecord: super::super::Foundation::FILETIME, prawlogrecord: *mut PDH_RAW_LOG_RECORD, pdwbufferlength: *mut u32) -> i32;
        }
        ::core::mem::transmute(PdhReadRawLogRecord(::core::mem::transmute(hlog), ftrecord.into_param().abi(), ::core::mem::transmute(prawlogrecord), ::core::mem::transmute(pdwbufferlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn PdhRemoveCounter(hcounter: isize) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhRemoveCounter(hcounter: isize) -> i32;
        }
        ::core::mem::transmute(PdhRemoveCounter(::core::mem::transmute(hcounter)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhSelectDataSourceA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(hwndowner: Param0, dwflags: PDH_SELECT_DATA_SOURCE_FLAGS, szdatasource: ::windows::core::PSTR, pcchbufferlength: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhSelectDataSourceA(hwndowner: super::super::Foundation::HWND, dwflags: PDH_SELECT_DATA_SOURCE_FLAGS, szdatasource: ::windows::core::PSTR, pcchbufferlength: *mut u32) -> i32;
        }
        ::core::mem::transmute(PdhSelectDataSourceA(hwndowner.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(szdatasource), ::core::mem::transmute(pcchbufferlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhSelectDataSourceW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(hwndowner: Param0, dwflags: PDH_SELECT_DATA_SOURCE_FLAGS, szdatasource: ::windows::core::PWSTR, pcchbufferlength: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhSelectDataSourceW(hwndowner: super::super::Foundation::HWND, dwflags: PDH_SELECT_DATA_SOURCE_FLAGS, szdatasource: ::windows::core::PWSTR, pcchbufferlength: *mut u32) -> i32;
        }
        ::core::mem::transmute(PdhSelectDataSourceW(hwndowner.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(szdatasource), ::core::mem::transmute(pcchbufferlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn PdhSetCounterScaleFactor(hcounter: isize, lfactor: i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhSetCounterScaleFactor(hcounter: isize, lfactor: i32) -> i32;
        }
        ::core::mem::transmute(PdhSetCounterScaleFactor(::core::mem::transmute(hcounter), ::core::mem::transmute(lfactor)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn PdhSetDefaultRealTimeDataSource(dwdatasourceid: REAL_TIME_DATA_SOURCE_ID_FLAGS) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhSetDefaultRealTimeDataSource(dwdatasourceid: REAL_TIME_DATA_SOURCE_ID_FLAGS) -> i32;
        }
        ::core::mem::transmute(PdhSetDefaultRealTimeDataSource(::core::mem::transmute(dwdatasourceid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn PdhSetLogSetRunID(hlog: isize, runid: i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhSetLogSetRunID(hlog: isize, runid: i32) -> i32;
        }
        ::core::mem::transmute(PdhSetLogSetRunID(::core::mem::transmute(hlog), ::core::mem::transmute(runid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn PdhSetQueryTimeRange(hquery: isize, pinfo: *const PDH_TIME_INFO) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhSetQueryTimeRange(hquery: isize, pinfo: *const PDH_TIME_INFO) -> i32;
        }
        ::core::mem::transmute(PdhSetQueryTimeRange(::core::mem::transmute(hquery), ::core::mem::transmute(pinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn PdhUpdateLogA<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(hlog: isize, szuserstring: Param1) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhUpdateLogA(hlog: isize, szuserstring: ::windows::core::PCSTR) -> i32;
        }
        ::core::mem::transmute(PdhUpdateLogA(::core::mem::transmute(hlog), szuserstring.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn PdhUpdateLogFileCatalog(hlog: isize) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhUpdateLogFileCatalog(hlog: isize) -> i32;
        }
        ::core::mem::transmute(PdhUpdateLogFileCatalog(::core::mem::transmute(hlog)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn PdhUpdateLogW<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(hlog: isize, szuserstring: Param1) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhUpdateLogW(hlog: isize, szuserstring: ::windows::core::PCWSTR) -> i32;
        }
        ::core::mem::transmute(PdhUpdateLogW(::core::mem::transmute(hlog), szuserstring.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn PdhValidatePathA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(szfullpathbuffer: Param0) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhValidatePathA(szfullpathbuffer: ::windows::core::PCSTR) -> i32;
        }
        ::core::mem::transmute(PdhValidatePathA(szfullpathbuffer.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn PdhValidatePathExA<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(hdatasource: isize, szfullpathbuffer: Param1) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhValidatePathExA(hdatasource: isize, szfullpathbuffer: ::windows::core::PCSTR) -> i32;
        }
        ::core::mem::transmute(PdhValidatePathExA(::core::mem::transmute(hdatasource), szfullpathbuffer.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn PdhValidatePathExW<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(hdatasource: isize, szfullpathbuffer: Param1) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhValidatePathExW(hdatasource: isize, szfullpathbuffer: ::windows::core::PCWSTR) -> i32;
        }
        ::core::mem::transmute(PdhValidatePathExW(::core::mem::transmute(hdatasource), szfullpathbuffer.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn PdhValidatePathW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(szfullpathbuffer: Param0) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhValidatePathW(szfullpathbuffer: ::windows::core::PCWSTR) -> i32;
        }
        ::core::mem::transmute(PdhValidatePathW(szfullpathbuffer.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn PdhVerifySQLDBA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(szdatasource: Param0) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhVerifySQLDBA(szdatasource: ::windows::core::PCSTR) -> i32;
        }
        ::core::mem::transmute(PdhVerifySQLDBA(szdatasource.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn PdhVerifySQLDBW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(szdatasource: Param0) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhVerifySQLDBW(szdatasource: ::windows::core::PCWSTR) -> i32;
        }
        ::core::mem::transmute(PdhVerifySQLDBW(szdatasource.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn PerfAddCounters<'a, Param0: ::windows::core::IntoParam<'a, PerfQueryHandle>>(hquery: Param0, pcounters: *const PERF_COUNTER_IDENTIFIER, cbcounters: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PerfAddCounters(hquery: PerfQueryHandle, pcounters: *const PERF_COUNTER_IDENTIFIER, cbcounters: u32) -> u32;
        }
        ::core::mem::transmute(PerfAddCounters(hquery.into_param().abi(), ::core::mem::transmute(pcounters), ::core::mem::transmute(cbcounters)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PerfCloseQueryHandle<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hquery: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PerfCloseQueryHandle(hquery: super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(PerfCloseQueryHandle(hquery.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PerfCounterDataType(pub i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_ERROR_RETURN: PerfCounterDataType = PerfCounterDataType(0i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_SINGLE_COUNTER: PerfCounterDataType = PerfCounterDataType(1i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_MULTIPLE_COUNTERS: PerfCounterDataType = PerfCounterDataType(2i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_MULTIPLE_INSTANCES: PerfCounterDataType = PerfCounterDataType(4i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_COUNTERSET: PerfCounterDataType = PerfCounterDataType(6i32);
impl ::core::marker::Copy for PerfCounterDataType {}
impl ::core::clone::Clone for PerfCounterDataType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PerfCounterDataType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PerfCounterDataType {
    type Abi = Self;
}
impl ::core::fmt::Debug for PerfCounterDataType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PerfCounterDataType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn PerfCreateInstance<'a, Param0: ::windows::core::IntoParam<'a, PerfProviderHandle>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(providerhandle: Param0, countersetguid: *const ::windows::core::GUID, name: Param2, id: u32) -> *mut PERF_COUNTERSET_INSTANCE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PerfCreateInstance(providerhandle: PerfProviderHandle, countersetguid: *const ::windows::core::GUID, name: ::windows::core::PCWSTR, id: u32) -> *mut PERF_COUNTERSET_INSTANCE;
        }
        ::core::mem::transmute(PerfCreateInstance(providerhandle.into_param().abi(), ::core::mem::transmute(countersetguid), name.into_param().abi(), ::core::mem::transmute(id)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PerfDecrementULongCounterValue<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(provider: Param0, instance: *mut PERF_COUNTERSET_INSTANCE, counterid: u32, value: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PerfDecrementULongCounterValue(provider: super::super::Foundation::HANDLE, instance: *mut PERF_COUNTERSET_INSTANCE, counterid: u32, value: u32) -> u32;
        }
        ::core::mem::transmute(PerfDecrementULongCounterValue(provider.into_param().abi(), ::core::mem::transmute(instance), ::core::mem::transmute(counterid), ::core::mem::transmute(value)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PerfDecrementULongLongCounterValue<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(provider: Param0, instance: *mut PERF_COUNTERSET_INSTANCE, counterid: u32, value: u64) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PerfDecrementULongLongCounterValue(provider: super::super::Foundation::HANDLE, instance: *mut PERF_COUNTERSET_INSTANCE, counterid: u32, value: u64) -> u32;
        }
        ::core::mem::transmute(PerfDecrementULongLongCounterValue(provider.into_param().abi(), ::core::mem::transmute(instance), ::core::mem::transmute(counterid), ::core::mem::transmute(value)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn PerfDeleteCounters<'a, Param0: ::windows::core::IntoParam<'a, PerfQueryHandle>>(hquery: Param0, pcounters: *const PERF_COUNTER_IDENTIFIER, cbcounters: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PerfDeleteCounters(hquery: PerfQueryHandle, pcounters: *const PERF_COUNTER_IDENTIFIER, cbcounters: u32) -> u32;
        }
        ::core::mem::transmute(PerfDeleteCounters(hquery.into_param().abi(), ::core::mem::transmute(pcounters), ::core::mem::transmute(cbcounters)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn PerfDeleteInstance<'a, Param0: ::windows::core::IntoParam<'a, PerfProviderHandle>>(provider: Param0, instanceblock: *const PERF_COUNTERSET_INSTANCE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PerfDeleteInstance(provider: PerfProviderHandle, instanceblock: *const PERF_COUNTERSET_INSTANCE) -> u32;
        }
        ::core::mem::transmute(PerfDeleteInstance(provider.into_param().abi(), ::core::mem::transmute(instanceblock)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn PerfEnumerateCounterSet<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(szmachine: Param0, pcountersetids: &mut [::windows::core::GUID], pccountersetidsactual: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PerfEnumerateCounterSet(szmachine: ::windows::core::PCWSTR, pcountersetids: *mut ::windows::core::GUID, ccountersetids: u32, pccountersetidsactual: *mut u32) -> u32;
        }
        ::core::mem::transmute(PerfEnumerateCounterSet(szmachine.into_param().abi(), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pcountersetids)), pcountersetids.len() as _, ::core::mem::transmute(pccountersetidsactual)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn PerfEnumerateCounterSetInstances<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(szmachine: Param0, pcountersetid: *const ::windows::core::GUID, pinstances: *mut PERF_INSTANCE_HEADER, cbinstances: u32, pcbinstancesactual: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PerfEnumerateCounterSetInstances(szmachine: ::windows::core::PCWSTR, pcountersetid: *const ::windows::core::GUID, pinstances: *mut PERF_INSTANCE_HEADER, cbinstances: u32, pcbinstancesactual: *mut u32) -> u32;
        }
        ::core::mem::transmute(PerfEnumerateCounterSetInstances(szmachine.into_param().abi(), ::core::mem::transmute(pcountersetid), ::core::mem::transmute(pinstances), ::core::mem::transmute(cbinstances), ::core::mem::transmute(pcbinstancesactual)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PerfIncrementULongCounterValue<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(provider: Param0, instance: *mut PERF_COUNTERSET_INSTANCE, counterid: u32, value: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PerfIncrementULongCounterValue(provider: super::super::Foundation::HANDLE, instance: *mut PERF_COUNTERSET_INSTANCE, counterid: u32, value: u32) -> u32;
        }
        ::core::mem::transmute(PerfIncrementULongCounterValue(provider.into_param().abi(), ::core::mem::transmute(instance), ::core::mem::transmute(counterid), ::core::mem::transmute(value)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PerfIncrementULongLongCounterValue<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(provider: Param0, instance: *mut PERF_COUNTERSET_INSTANCE, counterid: u32, value: u64) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PerfIncrementULongLongCounterValue(provider: super::super::Foundation::HANDLE, instance: *mut PERF_COUNTERSET_INSTANCE, counterid: u32, value: u64) -> u32;
        }
        ::core::mem::transmute(PerfIncrementULongLongCounterValue(provider.into_param().abi(), ::core::mem::transmute(instance), ::core::mem::transmute(counterid), ::core::mem::transmute(value)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn PerfOpenQueryHandle<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(szmachine: Param0, phquery: *mut PerfQueryHandle) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PerfOpenQueryHandle(szmachine: ::windows::core::PCWSTR, phquery: *mut PerfQueryHandle) -> u32;
        }
        ::core::mem::transmute(PerfOpenQueryHandle(szmachine.into_param().abi(), ::core::mem::transmute(phquery)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PerfProviderHandle(pub isize);
impl PerfProviderHandle {
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
impl ::core::default::Default for PerfProviderHandle {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for PerfProviderHandle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for PerfProviderHandle {}
impl ::core::fmt::Debug for PerfProviderHandle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PerfProviderHandle").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Abi for PerfProviderHandle {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PerfQueryCounterData<'a, Param0: ::windows::core::IntoParam<'a, PerfQueryHandle>>(hquery: Param0, pcounterblock: *mut PERF_DATA_HEADER, cbcounterblock: u32, pcbcounterblockactual: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PerfQueryCounterData(hquery: PerfQueryHandle, pcounterblock: *mut PERF_DATA_HEADER, cbcounterblock: u32, pcbcounterblockactual: *mut u32) -> u32;
        }
        ::core::mem::transmute(PerfQueryCounterData(hquery.into_param().abi(), ::core::mem::transmute(pcounterblock), ::core::mem::transmute(cbcounterblock), ::core::mem::transmute(pcbcounterblockactual)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn PerfQueryCounterInfo<'a, Param0: ::windows::core::IntoParam<'a, PerfQueryHandle>>(hquery: Param0, pcounters: *mut PERF_COUNTER_IDENTIFIER, cbcounters: u32, pcbcountersactual: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PerfQueryCounterInfo(hquery: PerfQueryHandle, pcounters: *mut PERF_COUNTER_IDENTIFIER, cbcounters: u32, pcbcountersactual: *mut u32) -> u32;
        }
        ::core::mem::transmute(PerfQueryCounterInfo(hquery.into_param().abi(), ::core::mem::transmute(pcounters), ::core::mem::transmute(cbcounters), ::core::mem::transmute(pcbcountersactual)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn PerfQueryCounterSetRegistrationInfo<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(szmachine: Param0, pcountersetid: *const ::windows::core::GUID, requestcode: PerfRegInfoType, requestlangid: u32, pbreginfo: *mut u8, cbreginfo: u32, pcbreginfoactual: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PerfQueryCounterSetRegistrationInfo(szmachine: ::windows::core::PCWSTR, pcountersetid: *const ::windows::core::GUID, requestcode: PerfRegInfoType, requestlangid: u32, pbreginfo: *mut u8, cbreginfo: u32, pcbreginfoactual: *mut u32) -> u32;
        }
        ::core::mem::transmute(PerfQueryCounterSetRegistrationInfo(szmachine.into_param().abi(), ::core::mem::transmute(pcountersetid), ::core::mem::transmute(requestcode), ::core::mem::transmute(requestlangid), ::core::mem::transmute(pbreginfo), ::core::mem::transmute(cbreginfo), ::core::mem::transmute(pcbreginfoactual)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PerfQueryHandle(pub isize);
impl PerfQueryHandle {
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
impl ::core::default::Default for PerfQueryHandle {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for PerfQueryHandle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for PerfQueryHandle {}
impl ::core::fmt::Debug for PerfQueryHandle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PerfQueryHandle").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Abi for PerfQueryHandle {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PerfQueryInstance<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(providerhandle: Param0, countersetguid: *const ::windows::core::GUID, name: Param2, id: u32) -> *mut PERF_COUNTERSET_INSTANCE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PerfQueryInstance(providerhandle: super::super::Foundation::HANDLE, countersetguid: *const ::windows::core::GUID, name: ::windows::core::PCWSTR, id: u32) -> *mut PERF_COUNTERSET_INSTANCE;
        }
        ::core::mem::transmute(PerfQueryInstance(providerhandle.into_param().abi(), ::core::mem::transmute(countersetguid), name.into_param().abi(), ::core::mem::transmute(id)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PerfRegInfoType(pub i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_REG_COUNTERSET_STRUCT: PerfRegInfoType = PerfRegInfoType(1i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_REG_COUNTER_STRUCT: PerfRegInfoType = PerfRegInfoType(2i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_REG_COUNTERSET_NAME_STRING: PerfRegInfoType = PerfRegInfoType(3i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_REG_COUNTERSET_HELP_STRING: PerfRegInfoType = PerfRegInfoType(4i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_REG_COUNTER_NAME_STRINGS: PerfRegInfoType = PerfRegInfoType(5i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_REG_COUNTER_HELP_STRINGS: PerfRegInfoType = PerfRegInfoType(6i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_REG_PROVIDER_NAME: PerfRegInfoType = PerfRegInfoType(7i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_REG_PROVIDER_GUID: PerfRegInfoType = PerfRegInfoType(8i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_REG_COUNTERSET_ENGLISH_NAME: PerfRegInfoType = PerfRegInfoType(9i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_REG_COUNTER_ENGLISH_NAMES: PerfRegInfoType = PerfRegInfoType(10i32);
impl ::core::marker::Copy for PerfRegInfoType {}
impl ::core::clone::Clone for PerfRegInfoType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PerfRegInfoType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PerfRegInfoType {
    type Abi = Self;
}
impl ::core::fmt::Debug for PerfRegInfoType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PerfRegInfoType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PerfSetCounterRefValue<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(provider: Param0, instance: *mut PERF_COUNTERSET_INSTANCE, counterid: u32, address: *const ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PerfSetCounterRefValue(provider: super::super::Foundation::HANDLE, instance: *mut PERF_COUNTERSET_INSTANCE, counterid: u32, address: *const ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(PerfSetCounterRefValue(provider.into_param().abi(), ::core::mem::transmute(instance), ::core::mem::transmute(counterid), ::core::mem::transmute(address)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PerfSetCounterSetInfo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(providerhandle: Param0, template: *mut PERF_COUNTERSET_INFO, templatesize: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PerfSetCounterSetInfo(providerhandle: super::super::Foundation::HANDLE, template: *mut PERF_COUNTERSET_INFO, templatesize: u32) -> u32;
        }
        ::core::mem::transmute(PerfSetCounterSetInfo(providerhandle.into_param().abi(), ::core::mem::transmute(template), ::core::mem::transmute(templatesize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PerfSetULongCounterValue<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(provider: Param0, instance: *mut PERF_COUNTERSET_INSTANCE, counterid: u32, value: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PerfSetULongCounterValue(provider: super::super::Foundation::HANDLE, instance: *mut PERF_COUNTERSET_INSTANCE, counterid: u32, value: u32) -> u32;
        }
        ::core::mem::transmute(PerfSetULongCounterValue(provider.into_param().abi(), ::core::mem::transmute(instance), ::core::mem::transmute(counterid), ::core::mem::transmute(value)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PerfSetULongLongCounterValue<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(provider: Param0, instance: *mut PERF_COUNTERSET_INSTANCE, counterid: u32, value: u64) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PerfSetULongLongCounterValue(provider: super::super::Foundation::HANDLE, instance: *mut PERF_COUNTERSET_INSTANCE, counterid: u32, value: u64) -> u32;
        }
        ::core::mem::transmute(PerfSetULongLongCounterValue(provider.into_param().abi(), ::core::mem::transmute(instance), ::core::mem::transmute(counterid), ::core::mem::transmute(value)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn PerfStartProvider(providerguid: *const ::windows::core::GUID, controlcallback: PERFLIBREQUEST, phprovider: *mut PerfProviderHandle) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PerfStartProvider(providerguid: *const ::windows::core::GUID, controlcallback: ::windows::core::RawPtr, phprovider: *mut PerfProviderHandle) -> u32;
        }
        ::core::mem::transmute(PerfStartProvider(::core::mem::transmute(providerguid), ::core::mem::transmute(controlcallback), ::core::mem::transmute(phprovider)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn PerfStartProviderEx(providerguid: *const ::windows::core::GUID, providercontext: *const PERF_PROVIDER_CONTEXT, provider: *mut PerfProviderHandle) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PerfStartProviderEx(providerguid: *const ::windows::core::GUID, providercontext: *const PERF_PROVIDER_CONTEXT, provider: *mut PerfProviderHandle) -> u32;
        }
        ::core::mem::transmute(PerfStartProviderEx(::core::mem::transmute(providerguid), ::core::mem::transmute(providercontext), ::core::mem::transmute(provider)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn PerfStopProvider<'a, Param0: ::windows::core::IntoParam<'a, PerfProviderHandle>>(providerhandle: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PerfStopProvider(providerhandle: PerfProviderHandle) -> u32;
        }
        ::core::mem::transmute(PerfStopProvider(providerhandle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QueryPerformanceCounter(lpperformancecount: *mut i64) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryPerformanceCounter(lpperformancecount: *mut i64) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(QueryPerformanceCounter(::core::mem::transmute(lpperformancecount)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QueryPerformanceFrequency(lpfrequency: *mut i64) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryPerformanceFrequency(lpfrequency: *mut i64) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(QueryPerformanceFrequency(::core::mem::transmute(lpfrequency)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct REAL_TIME_DATA_SOURCE_ID_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const DATA_SOURCE_REGISTRY: REAL_TIME_DATA_SOURCE_ID_FLAGS = REAL_TIME_DATA_SOURCE_ID_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const DATA_SOURCE_WBEM: REAL_TIME_DATA_SOURCE_ID_FLAGS = REAL_TIME_DATA_SOURCE_ID_FLAGS(4u32);
impl ::core::marker::Copy for REAL_TIME_DATA_SOURCE_ID_FLAGS {}
impl ::core::clone::Clone for REAL_TIME_DATA_SOURCE_ID_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for REAL_TIME_DATA_SOURCE_ID_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for REAL_TIME_DATA_SOURCE_ID_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for REAL_TIME_DATA_SOURCE_ID_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("REAL_TIME_DATA_SOURCE_ID_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ReportValueTypeConstants(pub i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const sysmonDefaultValue: ReportValueTypeConstants = ReportValueTypeConstants(0i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const sysmonCurrentValue: ReportValueTypeConstants = ReportValueTypeConstants(1i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const sysmonAverage: ReportValueTypeConstants = ReportValueTypeConstants(2i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const sysmonMinimum: ReportValueTypeConstants = ReportValueTypeConstants(3i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const sysmonMaximum: ReportValueTypeConstants = ReportValueTypeConstants(4i32);
impl ::core::marker::Copy for ReportValueTypeConstants {}
impl ::core::clone::Clone for ReportValueTypeConstants {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ReportValueTypeConstants {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ReportValueTypeConstants {
    type Abi = Self;
}
impl ::core::fmt::Debug for ReportValueTypeConstants {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ReportValueTypeConstants").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ResourcePolicy(pub i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaDeleteLargest: ResourcePolicy = ResourcePolicy(0i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaDeleteOldest: ResourcePolicy = ResourcePolicy(1i32);
impl ::core::marker::Copy for ResourcePolicy {}
impl ::core::clone::Clone for ResourcePolicy {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ResourcePolicy {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ResourcePolicy {
    type Abi = Self;
}
impl ::core::fmt::Debug for ResourcePolicy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ResourcePolicy").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn RestorePerfRegistryFromFileW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(szfilename: Param0, szlangid: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RestorePerfRegistryFromFileW(szfilename: ::windows::core::PCWSTR, szlangid: ::windows::core::PCWSTR) -> u32;
        }
        ::core::mem::transmute(RestorePerfRegistryFromFileW(szfilename.into_param().abi(), szlangid.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const S_PDH: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x04d66358_c4a1_419b_8023_23b73902de2c);
pub const ServerDataCollectorSet: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03837531_098b_11d8_9414_505054503030);
pub const ServerDataCollectorSetCollection: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03837532_098b_11d8_9414_505054503030);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn SetServiceAsTrustedA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(szreserved: Param0, szservicename: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetServiceAsTrustedA(szreserved: ::windows::core::PCSTR, szservicename: ::windows::core::PCSTR) -> u32;
        }
        ::core::mem::transmute(SetServiceAsTrustedA(szreserved.into_param().abi(), szservicename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn SetServiceAsTrustedW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(szreserved: Param0, szservicename: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetServiceAsTrustedW(szreserved: ::windows::core::PCWSTR, szservicename: ::windows::core::PCWSTR) -> u32;
        }
        ::core::mem::transmute(SetServiceAsTrustedW(szreserved.into_param().abi(), szservicename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const SourcePropPage: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0cf32aa1_7571_11d0_93c4_00aa00a3ddea);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct StreamMode(pub i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaFile: StreamMode = StreamMode(1i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaRealTime: StreamMode = StreamMode(2i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaBoth: StreamMode = StreamMode(3i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaBuffering: StreamMode = StreamMode(4i32);
impl ::core::marker::Copy for StreamMode {}
impl ::core::clone::Clone for StreamMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for StreamMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for StreamMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for StreamMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StreamMode").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SysmonBatchReason(pub i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const sysmonBatchNone: SysmonBatchReason = SysmonBatchReason(0i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const sysmonBatchAddFiles: SysmonBatchReason = SysmonBatchReason(1i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const sysmonBatchAddCounters: SysmonBatchReason = SysmonBatchReason(2i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const sysmonBatchAddFilesAutoCounters: SysmonBatchReason = SysmonBatchReason(3i32);
impl ::core::marker::Copy for SysmonBatchReason {}
impl ::core::clone::Clone for SysmonBatchReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SysmonBatchReason {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SysmonBatchReason {
    type Abi = Self;
}
impl ::core::fmt::Debug for SysmonBatchReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SysmonBatchReason").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SysmonDataType(pub i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const sysmonDataAvg: SysmonDataType = SysmonDataType(1i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const sysmonDataMin: SysmonDataType = SysmonDataType(2i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const sysmonDataMax: SysmonDataType = SysmonDataType(3i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const sysmonDataTime: SysmonDataType = SysmonDataType(4i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const sysmonDataCount: SysmonDataType = SysmonDataType(5i32);
impl ::core::marker::Copy for SysmonDataType {}
impl ::core::clone::Clone for SysmonDataType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SysmonDataType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SysmonDataType {
    type Abi = Self;
}
impl ::core::fmt::Debug for SysmonDataType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SysmonDataType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SysmonFileType(pub i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const sysmonFileHtml: SysmonFileType = SysmonFileType(1i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const sysmonFileReport: SysmonFileType = SysmonFileType(2i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const sysmonFileCsv: SysmonFileType = SysmonFileType(3i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const sysmonFileTsv: SysmonFileType = SysmonFileType(4i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const sysmonFileBlg: SysmonFileType = SysmonFileType(5i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const sysmonFileRetiredBlg: SysmonFileType = SysmonFileType(6i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const sysmonFileGif: SysmonFileType = SysmonFileType(7i32);
impl ::core::marker::Copy for SysmonFileType {}
impl ::core::clone::Clone for SysmonFileType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SysmonFileType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SysmonFileType {
    type Abi = Self;
}
impl ::core::fmt::Debug for SysmonFileType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SysmonFileType").field(&self.0).finish()
    }
}
pub const SystemDataCollectorSet: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03837546_098b_11d8_9414_505054503030);
pub const SystemDataCollectorSetCollection: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03837547_098b_11d8_9414_505054503030);
pub const SystemMonitor: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc4d2d8e0_d1dd_11ce_940f_008029004347);
pub const SystemMonitor2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7f30578c_5f38_4612_acfe_6ed04c7b7af8);
pub const TraceDataProvider: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03837513_098b_11d8_9414_505054503030);
pub const TraceDataProviderCollection: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03837511_098b_11d8_9414_505054503030);
pub const TraceSession: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0383751c_098b_11d8_9414_505054503030);
pub const TraceSessionCollection: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03837530_098b_11d8_9414_505054503030);
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UnloadPerfCounterTextStringsA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(lpcommandline: Param0, bquietmodearg: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UnloadPerfCounterTextStringsA(lpcommandline: ::windows::core::PCSTR, bquietmodearg: super::super::Foundation::BOOL) -> u32;
        }
        ::core::mem::transmute(UnloadPerfCounterTextStringsA(lpcommandline.into_param().abi(), bquietmodearg.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UnloadPerfCounterTextStringsW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(lpcommandline: Param0, bquietmodearg: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UnloadPerfCounterTextStringsW(lpcommandline: ::windows::core::PCWSTR, bquietmodearg: super::super::Foundation::BOOL) -> u32;
        }
        ::core::mem::transmute(UnloadPerfCounterTextStringsW(lpcommandline.into_param().abi(), bquietmodearg.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn UpdatePerfNameFilesA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(sznewctrfilepath: Param0, sznewhlpfilepath: Param1, szlanguageid: Param2, dwflags: usize) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UpdatePerfNameFilesA(sznewctrfilepath: ::windows::core::PCSTR, sznewhlpfilepath: ::windows::core::PCSTR, szlanguageid: ::windows::core::PCSTR, dwflags: usize) -> u32;
        }
        ::core::mem::transmute(UpdatePerfNameFilesA(sznewctrfilepath.into_param().abi(), sznewhlpfilepath.into_param().abi(), szlanguageid.into_param().abi(), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[inline]
pub unsafe fn UpdatePerfNameFilesW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(sznewctrfilepath: Param0, sznewhlpfilepath: Param1, szlanguageid: Param2, dwflags: usize) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UpdatePerfNameFilesW(sznewctrfilepath: ::windows::core::PCWSTR, sznewhlpfilepath: ::windows::core::PCWSTR, szlanguageid: ::windows::core::PCWSTR, dwflags: usize) -> u32;
        }
        ::core::mem::transmute(UpdatePerfNameFilesW(sznewctrfilepath.into_param().abi(), sznewhlpfilepath.into_param().abi(), szlanguageid.into_param().abi(), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ValueMapType(pub i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaIndex: ValueMapType = ValueMapType(1i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaFlag: ValueMapType = ValueMapType(2i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaFlagArray: ValueMapType = ValueMapType(3i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaValidation: ValueMapType = ValueMapType(4i32);
impl ::core::marker::Copy for ValueMapType {}
impl ::core::clone::Clone for ValueMapType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ValueMapType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ValueMapType {
    type Abi = Self;
}
impl ::core::fmt::Debug for ValueMapType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ValueMapType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const WINPERF_LOG_DEBUG: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const WINPERF_LOG_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const WINPERF_LOG_USER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const WINPERF_LOG_VERBOSE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WeekDays(pub i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaRunOnce: WeekDays = WeekDays(0i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaSunday: WeekDays = WeekDays(1i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaMonday: WeekDays = WeekDays(2i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaTuesday: WeekDays = WeekDays(4i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaWednesday: WeekDays = WeekDays(8i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaThursday: WeekDays = WeekDays(16i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaFriday: WeekDays = WeekDays(32i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaSaturday: WeekDays = WeekDays(64i32);
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaEveryday: WeekDays = WeekDays(127i32);
impl ::core::marker::Copy for WeekDays {}
impl ::core::clone::Clone for WeekDays {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WeekDays {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WeekDays {
    type Abi = Self;
}
impl ::core::fmt::Debug for WeekDays {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WeekDays").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[repr(transparent)]
pub struct _ICounterItemUnion(::windows::core::IUnknown);
impl _ICounterItemUnion {
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn Value(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Value)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetColor(&self, color: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetColor)(::core::mem::transmute_copy(self), ::core::mem::transmute(color)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn Color(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Color)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetWidth(&self, iwidth: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetWidth)(::core::mem::transmute_copy(self), ::core::mem::transmute(iwidth)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn Width(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Width)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetLineStyle(&self, ilinestyle: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetLineStyle)(::core::mem::transmute_copy(self), ::core::mem::transmute(ilinestyle)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn LineStyle(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).LineStyle)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetScaleFactor(&self, iscale: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetScaleFactor)(::core::mem::transmute_copy(self), ::core::mem::transmute(iscale)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn ScaleFactor(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ScaleFactor)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Path(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Path)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn GetValue(&self, value: *mut f64, status: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetValue)(::core::mem::transmute_copy(self), ::core::mem::transmute(value), ::core::mem::transmute(status)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn GetStatistics(&self, max: *mut f64, min: *mut f64, avg: *mut f64, status: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetStatistics)(::core::mem::transmute_copy(self), ::core::mem::transmute(max), ::core::mem::transmute(min), ::core::mem::transmute(avg), ::core::mem::transmute(status)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetSelected(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSelected)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn Selected(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Selected)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetVisible(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetVisible)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn Visible(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Visible)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetDataAt(&self, iindex: i32, iwhich: SysmonDataType) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDataAt)(::core::mem::transmute_copy(self), ::core::mem::transmute(iindex), ::core::mem::transmute(iwhich), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::VARIANT>(result__)
    }
}
impl ::core::convert::From<_ICounterItemUnion> for ::windows::core::IUnknown {
    fn from(value: _ICounterItemUnion) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&_ICounterItemUnion> for ::windows::core::IUnknown {
    fn from(value: &_ICounterItemUnion) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for _ICounterItemUnion {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a _ICounterItemUnion {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for _ICounterItemUnion {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for _ICounterItemUnion {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for _ICounterItemUnion {}
impl ::core::fmt::Debug for _ICounterItemUnion {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_ICounterItemUnion").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for _ICounterItemUnion {
    type Vtable = _ICounterItemUnion_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xde1a6b74_9182_4c41_8e2c_24c2cd30ee83);
}
#[repr(C)]
#[doc(hidden)]
pub struct _ICounterItemUnion_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdblvalue: *mut f64) -> ::windows::core::HRESULT,
    pub SetColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, color: u32) -> ::windows::core::HRESULT,
    pub Color: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcolor: *mut u32) -> ::windows::core::HRESULT,
    pub SetWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iwidth: i32) -> ::windows::core::HRESULT,
    pub Width: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pivalue: *mut i32) -> ::windows::core::HRESULT,
    pub SetLineStyle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ilinestyle: i32) -> ::windows::core::HRESULT,
    pub LineStyle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pivalue: *mut i32) -> ::windows::core::HRESULT,
    pub SetScaleFactor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iscale: i32) -> ::windows::core::HRESULT,
    pub ScaleFactor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pivalue: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Path: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstrvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Path: usize,
    pub GetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut f64, status: *mut i32) -> ::windows::core::HRESULT,
    pub GetStatistics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, max: *mut f64, min: *mut f64, avg: *mut f64, status: *mut i32) -> ::windows::core::HRESULT,
    pub SetSelected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT,
    pub Selected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub SetVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT,
    pub Visible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetDataAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iindex: i32, iwhich: SysmonDataType, pvariant: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetDataAt: usize,
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[repr(transparent)]
pub struct _ISystemMonitorUnion(::windows::core::IUnknown);
impl _ISystemMonitorUnion {
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn Appearance(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Appearance)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetAppearance(&self, iappearance: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAppearance)(::core::mem::transmute_copy(self), ::core::mem::transmute(iappearance)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn BackColor(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).BackColor)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetBackColor(&self, color: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBackColor)(::core::mem::transmute_copy(self), ::core::mem::transmute(color)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn BorderStyle(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).BorderStyle)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetBorderStyle(&self, iborderstyle: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBorderStyle)(::core::mem::transmute_copy(self), ::core::mem::transmute(iborderstyle)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn ForeColor(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ForeColor)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetForeColor(&self, color: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetForeColor)(::core::mem::transmute_copy(self), ::core::mem::transmute(color)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Font(&self) -> ::windows::core::Result<super::Ole::IFontDisp> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Font)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Ole::IFontDisp>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn putref_Font<'a, Param0: ::windows::core::IntoParam<'a, super::Ole::IFontDisp>>(&self, pfont: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).putref_Font)(::core::mem::transmute_copy(self), pfont.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Counters(&self) -> ::windows::core::Result<ICounters> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Counters)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ICounters>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetShowVerticalGrid(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetShowVerticalGrid)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn ShowVerticalGrid(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ShowVerticalGrid)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetShowHorizontalGrid(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetShowHorizontalGrid)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn ShowHorizontalGrid(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ShowHorizontalGrid)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetShowLegend(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetShowLegend)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn ShowLegend(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ShowLegend)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetShowScaleLabels(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetShowScaleLabels)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn ShowScaleLabels(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ShowScaleLabels)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetShowValueBar(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetShowValueBar)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn ShowValueBar(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ShowValueBar)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetMaximumScale(&self, ivalue: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMaximumScale)(::core::mem::transmute_copy(self), ::core::mem::transmute(ivalue)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn MaximumScale(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).MaximumScale)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetMinimumScale(&self, ivalue: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMinimumScale)(::core::mem::transmute_copy(self), ::core::mem::transmute(ivalue)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn MinimumScale(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).MinimumScale)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetUpdateInterval(&self, fvalue: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetUpdateInterval)(::core::mem::transmute_copy(self), ::core::mem::transmute(fvalue)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn UpdateInterval(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).UpdateInterval)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetDisplayType(&self, edisplaytype: DisplayTypeConstants) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDisplayType)(::core::mem::transmute_copy(self), ::core::mem::transmute(edisplaytype)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn DisplayType(&self) -> ::windows::core::Result<DisplayTypeConstants> {
        let mut result__: DisplayTypeConstants = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).DisplayType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DisplayTypeConstants>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetManualUpdate(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetManualUpdate)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn ManualUpdate(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ManualUpdate)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetGraphTitle<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstitle: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetGraphTitle)(::core::mem::transmute_copy(self), bstitle.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GraphTitle(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GraphTitle)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetYAxisLabel<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstitle: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetYAxisLabel)(::core::mem::transmute_copy(self), bstitle.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn YAxisLabel(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).YAxisLabel)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn CollectSample(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CollectSample)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn UpdateGraph(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UpdateGraph)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn BrowseCounters(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BrowseCounters)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn DisplayProperties(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DisplayProperties)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn Counter(&self, iindex: i32) -> ::windows::core::Result<ICounterItem> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Counter)(::core::mem::transmute_copy(self), ::core::mem::transmute(iindex), ::core::mem::transmute(&mut result__)).from_abi::<ICounterItem>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddCounter<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bspath: Param0) -> ::windows::core::Result<ICounterItem> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).AddCounter)(::core::mem::transmute_copy(self), bspath.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<ICounterItem>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn DeleteCounter<'a, Param0: ::windows::core::IntoParam<'a, ICounterItem>>(&self, pctr: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DeleteCounter)(::core::mem::transmute_copy(self), pctr.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn BackColorCtl(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).BackColorCtl)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetBackColorCtl(&self, color: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBackColorCtl)(::core::mem::transmute_copy(self), ::core::mem::transmute(color)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLogFileName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bsfilename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetLogFileName)(::core::mem::transmute_copy(self), bsfilename.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LogFileName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).LogFileName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetLogViewStart(&self, starttime: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetLogViewStart)(::core::mem::transmute_copy(self), ::core::mem::transmute(starttime)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn LogViewStart(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).LogViewStart)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetLogViewStop(&self, stoptime: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetLogViewStop)(::core::mem::transmute_copy(self), ::core::mem::transmute(stoptime)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn LogViewStop(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).LogViewStop)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn GridColor(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GridColor)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetGridColor(&self, color: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetGridColor)(::core::mem::transmute_copy(self), ::core::mem::transmute(color)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn TimeBarColor(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).TimeBarColor)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetTimeBarColor(&self, color: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTimeBarColor)(::core::mem::transmute_copy(self), ::core::mem::transmute(color)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn Highlight(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Highlight)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetHighlight(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetHighlight)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn ShowToolbar(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ShowToolbar)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetShowToolbar(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetShowToolbar)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn Paste(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Paste)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn Copy(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Copy)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetReadOnly(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetReadOnly)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn ReadOnly(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ReadOnly)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetReportValueType(&self, ereportvaluetype: ReportValueTypeConstants) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetReportValueType)(::core::mem::transmute_copy(self), ::core::mem::transmute(ereportvaluetype)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn ReportValueType(&self) -> ::windows::core::Result<ReportValueTypeConstants> {
        let mut result__: ReportValueTypeConstants = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ReportValueType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ReportValueTypeConstants>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetMonitorDuplicateInstances(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMonitorDuplicateInstances)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn MonitorDuplicateInstances(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).MonitorDuplicateInstances)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetDisplayFilter(&self, ivalue: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDisplayFilter)(::core::mem::transmute_copy(self), ::core::mem::transmute(ivalue)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn DisplayFilter(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).DisplayFilter)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn LogFiles(&self) -> ::windows::core::Result<ILogFiles> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).LogFiles)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ILogFiles>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetDataSourceType(&self, edatasourcetype: DataSourceTypeConstants) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDataSourceType)(::core::mem::transmute_copy(self), ::core::mem::transmute(edatasourcetype)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn DataSourceType(&self) -> ::windows::core::Result<DataSourceTypeConstants> {
        let mut result__: DataSourceTypeConstants = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).DataSourceType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DataSourceTypeConstants>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSqlDsnName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bssqldsnname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSqlDsnName)(::core::mem::transmute_copy(self), bssqldsnname.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SqlDsnName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SqlDsnName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSqlLogSetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bssqllogsetname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSqlLogSetName)(::core::mem::transmute_copy(self), bssqllogsetname.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SqlLogSetName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SqlLogSetName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetEnableDigitGrouping(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetEnableDigitGrouping)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn EnableDigitGrouping(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).EnableDigitGrouping)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetEnableToolTips(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetEnableToolTips)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn EnableToolTips(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).EnableToolTips)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetShowTimeAxisLabels(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetShowTimeAxisLabels)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn ShowTimeAxisLabels(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ShowTimeAxisLabels)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetChartScroll(&self, bscroll: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetChartScroll)(::core::mem::transmute_copy(self), ::core::mem::transmute(bscroll)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn ChartScroll(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ChartScroll)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetDataPointCount(&self, inewcount: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDataPointCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(inewcount)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn DataPointCount(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).DataPointCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn ScaleToFit(&self, bselectedcountersonly: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ScaleToFit)(::core::mem::transmute_copy(self), ::core::mem::transmute(bselectedcountersonly)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SaveAs<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrfilename: Param0, esysmonfiletype: SysmonFileType) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SaveAs)(::core::mem::transmute_copy(self), bstrfilename.into_param().abi(), ::core::mem::transmute(esysmonfiletype)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Relog<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrfilename: Param0, esysmonfiletype: SysmonFileType, ifilter: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Relog)(::core::mem::transmute_copy(self), bstrfilename.into_param().abi(), ::core::mem::transmute(esysmonfiletype), ::core::mem::transmute(ifilter)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn ClearData(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ClearData)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn LogSourceStartTime(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).LogSourceStartTime)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn LogSourceStopTime(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).LogSourceStopTime)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn SetLogViewRange(&self, starttime: f64, stoptime: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetLogViewRange)(::core::mem::transmute_copy(self), ::core::mem::transmute(starttime), ::core::mem::transmute(stoptime)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn GetLogViewRange(&self, starttime: *mut f64, stoptime: *mut f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetLogViewRange)(::core::mem::transmute_copy(self), ::core::mem::transmute(starttime), ::core::mem::transmute(stoptime)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub unsafe fn BatchingLock(&self, flock: i16, ebatchreason: SysmonBatchReason) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BatchingLock)(::core::mem::transmute_copy(self), ::core::mem::transmute(flock), ::core::mem::transmute(ebatchreason)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LoadSettings<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrsettingfilename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).LoadSettings)(::core::mem::transmute_copy(self), bstrsettingfilename.into_param().abi()).ok()
    }
}
impl ::core::convert::From<_ISystemMonitorUnion> for ::windows::core::IUnknown {
    fn from(value: _ISystemMonitorUnion) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&_ISystemMonitorUnion> for ::windows::core::IUnknown {
    fn from(value: &_ISystemMonitorUnion) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for _ISystemMonitorUnion {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a _ISystemMonitorUnion {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for _ISystemMonitorUnion {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for _ISystemMonitorUnion {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for _ISystemMonitorUnion {}
impl ::core::fmt::Debug for _ISystemMonitorUnion {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_ISystemMonitorUnion").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for _ISystemMonitorUnion {
    type Vtable = _ISystemMonitorUnion_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc8a77338_265f_4de5_aa25_c7da1ce5a8f4);
}
#[repr(C)]
#[doc(hidden)]
pub struct _ISystemMonitorUnion_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Appearance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iappearance: *mut i32) -> ::windows::core::HRESULT,
    pub SetAppearance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iappearance: i32) -> ::windows::core::HRESULT,
    pub BackColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcolor: *mut u32) -> ::windows::core::HRESULT,
    pub SetBackColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, color: u32) -> ::windows::core::HRESULT,
    pub BorderStyle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iborderstyle: *mut i32) -> ::windows::core::HRESULT,
    pub SetBorderStyle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iborderstyle: i32) -> ::windows::core::HRESULT,
    pub ForeColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcolor: *mut u32) -> ::windows::core::HRESULT,
    pub SetForeColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, color: u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Font: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppfont: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Font: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub putref_Font: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfont: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    putref_Font: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Counters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppicounters: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Counters: usize,
    pub SetShowVerticalGrid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT,
    pub ShowVerticalGrid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub SetShowHorizontalGrid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT,
    pub ShowHorizontalGrid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub SetShowLegend: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT,
    pub ShowLegend: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub SetShowScaleLabels: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT,
    pub ShowScaleLabels: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub SetShowValueBar: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT,
    pub ShowValueBar: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub SetMaximumScale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ivalue: i32) -> ::windows::core::HRESULT,
    pub MaximumScale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pivalue: *mut i32) -> ::windows::core::HRESULT,
    pub SetMinimumScale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ivalue: i32) -> ::windows::core::HRESULT,
    pub MinimumScale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pivalue: *mut i32) -> ::windows::core::HRESULT,
    pub SetUpdateInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fvalue: f32) -> ::windows::core::HRESULT,
    pub UpdateInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfvalue: *mut f32) -> ::windows::core::HRESULT,
    pub SetDisplayType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, edisplaytype: DisplayTypeConstants) -> ::windows::core::HRESULT,
    pub DisplayType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pedisplaytype: *mut DisplayTypeConstants) -> ::windows::core::HRESULT,
    pub SetManualUpdate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT,
    pub ManualUpdate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetGraphTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstitle: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetGraphTitle: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GraphTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstitle: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GraphTitle: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetYAxisLabel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstitle: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetYAxisLabel: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub YAxisLabel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstitle: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    YAxisLabel: usize,
    pub CollectSample: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub UpdateGraph: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub BrowseCounters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DisplayProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Counter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iindex: i32, ppicounter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub AddCounter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bspath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppicounter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddCounter: usize,
    pub DeleteCounter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctr: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub BackColorCtl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcolor: *mut u32) -> ::windows::core::HRESULT,
    pub SetBackColorCtl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, color: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetLogFileName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bsfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetLogFileName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub LogFileName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bsfilename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LogFileName: usize,
    pub SetLogViewStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, starttime: f64) -> ::windows::core::HRESULT,
    pub LogViewStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, starttime: *mut f64) -> ::windows::core::HRESULT,
    pub SetLogViewStop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stoptime: f64) -> ::windows::core::HRESULT,
    pub LogViewStop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stoptime: *mut f64) -> ::windows::core::HRESULT,
    pub GridColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcolor: *mut u32) -> ::windows::core::HRESULT,
    pub SetGridColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, color: u32) -> ::windows::core::HRESULT,
    pub TimeBarColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcolor: *mut u32) -> ::windows::core::HRESULT,
    pub SetTimeBarColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, color: u32) -> ::windows::core::HRESULT,
    pub Highlight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub SetHighlight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT,
    pub ShowToolbar: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub SetShowToolbar: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT,
    pub Paste: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Copy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetReadOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT,
    pub ReadOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub SetReportValueType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ereportvaluetype: ReportValueTypeConstants) -> ::windows::core::HRESULT,
    pub ReportValueType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pereportvaluetype: *mut ReportValueTypeConstants) -> ::windows::core::HRESULT,
    pub SetMonitorDuplicateInstances: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT,
    pub MonitorDuplicateInstances: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub SetDisplayFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ivalue: i32) -> ::windows::core::HRESULT,
    pub DisplayFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pivalue: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub LogFiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppilogfiles: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    LogFiles: usize,
    pub SetDataSourceType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, edatasourcetype: DataSourceTypeConstants) -> ::windows::core::HRESULT,
    pub DataSourceType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pedatasourcetype: *mut DataSourceTypeConstants) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSqlDsnName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bssqldsnname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSqlDsnName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SqlDsnName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bssqldsnname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SqlDsnName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSqlLogSetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bssqllogsetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSqlLogSetName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SqlLogSetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bssqllogsetname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SqlLogSetName: usize,
    pub SetEnableDigitGrouping: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT,
    pub EnableDigitGrouping: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub SetEnableToolTips: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT,
    pub EnableToolTips: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub SetShowTimeAxisLabels: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT,
    pub ShowTimeAxisLabels: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub SetChartScroll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bscroll: i16) -> ::windows::core::HRESULT,
    pub ChartScroll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbscroll: *mut i16) -> ::windows::core::HRESULT,
    pub SetDataPointCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inewcount: i32) -> ::windows::core::HRESULT,
    pub DataPointCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pidatapointcount: *mut i32) -> ::windows::core::HRESULT,
    pub ScaleToFit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bselectedcountersonly: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SaveAs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, esysmonfiletype: SysmonFileType) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SaveAs: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Relog: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, esysmonfiletype: SysmonFileType, ifilter: i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Relog: usize,
    pub ClearData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub LogSourceStartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdate: *mut f64) -> ::windows::core::HRESULT,
    pub LogSourceStopTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdate: *mut f64) -> ::windows::core::HRESULT,
    pub SetLogViewRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, starttime: f64, stoptime: f64) -> ::windows::core::HRESULT,
    pub GetLogViewRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, starttime: *mut f64, stoptime: *mut f64) -> ::windows::core::HRESULT,
    pub BatchingLock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flock: i16, ebatchreason: SysmonBatchReason) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub LoadSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrsettingfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LoadSettings: usize,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
