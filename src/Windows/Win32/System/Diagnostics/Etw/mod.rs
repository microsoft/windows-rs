#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub struct CLASSIC_EVENT_ID {
    pub EventGuid: ::windows::runtime::GUID,
    pub Type: u8,
    pub Reserved: [u8; 7],
}
impl CLASSIC_EVENT_ID {}
impl ::std::default::Default for CLASSIC_EVENT_ID {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CLASSIC_EVENT_ID {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CLASSIC_EVENT_ID").field("EventGuid", &self.EventGuid).field("Type", &self.Type).field("Reserved", &self.Reserved).finish()
    }
}
impl ::std::cmp::PartialEq for CLASSIC_EVENT_ID {
    fn eq(&self, other: &Self) -> bool {
        self.EventGuid == other.EventGuid && self.Type == other.Type && self.Reserved == other.Reserved
    }
}
impl ::std::cmp::Eq for CLASSIC_EVENT_ID {}
unsafe impl ::windows::runtime::Abi for CLASSIC_EVENT_ID {
    type Abi = Self;
}
pub const CLSID_TraceRelogger: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2067822893, 1535, 17604, [144, 88, 244, 64, 199, 31, 23, 212]);
pub const CTraceRelogger: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2067822893, 1535, 17604, [144, 88, 244, 64, 199, 31, 23, 212]);
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
#[inline]
pub unsafe fn CloseTrace(tracehandle: u64) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CloseTrace(tracehandle: u64) -> u32;
        }
        ::std::mem::transmute(CloseTrace(::std::mem::transmute(tracehandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ControlTraceA<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PSTR>>(tracehandle: u64, instancename: Param1, properties: *mut EVENT_TRACE_PROPERTIES, controlcode: EVENT_TRACE_CONTROL) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ControlTraceA(tracehandle: u64, instancename: super::super::super::Foundation::PSTR, properties: *mut EVENT_TRACE_PROPERTIES, controlcode: EVENT_TRACE_CONTROL) -> u32;
        }
        ::std::mem::transmute(ControlTraceA(::std::mem::transmute(tracehandle), instancename.into_param().abi(), ::std::mem::transmute(properties), ::std::mem::transmute(controlcode)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ControlTraceW<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(tracehandle: u64, instancename: Param1, properties: *mut EVENT_TRACE_PROPERTIES, controlcode: EVENT_TRACE_CONTROL) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ControlTraceW(tracehandle: u64, instancename: super::super::super::Foundation::PWSTR, properties: *mut EVENT_TRACE_PROPERTIES, controlcode: EVENT_TRACE_CONTROL) -> u32;
        }
        ::std::mem::transmute(ControlTraceW(::std::mem::transmute(tracehandle), instancename.into_param().abi(), ::std::mem::transmute(properties), ::std::mem::transmute(controlcode)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateTraceInstanceId<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HANDLE>>(reghandle: Param0, instinfo: *mut EVENT_INSTANCE_INFO) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateTraceInstanceId(reghandle: super::super::super::Foundation::HANDLE, instinfo: *mut EVENT_INSTANCE_INFO) -> u32;
        }
        ::std::mem::transmute(CreateTraceInstanceId(reghandle.into_param().abi(), ::std::mem::transmute(instinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CveEventWrite<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(cveid: Param0, additionaldetails: Param1) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CveEventWrite(cveid: super::super::super::Foundation::PWSTR, additionaldetails: super::super::super::Foundation::PWSTR) -> i32;
        }
        ::std::mem::transmute(CveEventWrite(cveid.into_param().abi(), additionaldetails.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DECODING_SOURCE(pub i32);
pub const DecodingSourceXMLFile: DECODING_SOURCE = DECODING_SOURCE(0i32);
pub const DecodingSourceWbem: DECODING_SOURCE = DECODING_SOURCE(1i32);
pub const DecodingSourceWPP: DECODING_SOURCE = DECODING_SOURCE(2i32);
pub const DecodingSourceTlg: DECODING_SOURCE = DECODING_SOURCE(3i32);
pub const DecodingSourceMax: DECODING_SOURCE = DECODING_SOURCE(4i32);
impl ::std::convert::From<i32> for DECODING_SOURCE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DECODING_SOURCE {
    type Abi = Self;
}
pub const DefaultTraceSecurityGuid: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(135381423, 31239, 18950, [130, 237, 134, 148, 85, 205, 247, 19]);
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ENABLECALLBACK_ENABLED_STATE(pub u32);
pub const EVENT_CONTROL_CODE_DISABLE_PROVIDER: ENABLECALLBACK_ENABLED_STATE = ENABLECALLBACK_ENABLED_STATE(0u32);
pub const EVENT_CONTROL_CODE_ENABLE_PROVIDER: ENABLECALLBACK_ENABLED_STATE = ENABLECALLBACK_ENABLED_STATE(1u32);
pub const EVENT_CONTROL_CODE_CAPTURE_STATE: ENABLECALLBACK_ENABLED_STATE = ENABLECALLBACK_ENABLED_STATE(2u32);
impl ::std::convert::From<u32> for ENABLECALLBACK_ENABLED_STATE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ENABLECALLBACK_ENABLED_STATE {
    type Abi = Self;
}
impl ::std::ops::BitOr for ENABLECALLBACK_ENABLED_STATE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for ENABLECALLBACK_ENABLED_STATE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for ENABLECALLBACK_ENABLED_STATE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for ENABLECALLBACK_ENABLED_STATE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for ENABLECALLBACK_ENABLED_STATE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub struct ENABLE_TRACE_PARAMETERS {
    pub Version: u32,
    pub EnableProperty: u32,
    pub ControlFlags: u32,
    pub SourceId: ::windows::runtime::GUID,
    pub EnableFilterDesc: *mut EVENT_FILTER_DESCRIPTOR,
    pub FilterDescCount: u32,
}
impl ENABLE_TRACE_PARAMETERS {}
impl ::std::default::Default for ENABLE_TRACE_PARAMETERS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for ENABLE_TRACE_PARAMETERS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ENABLE_TRACE_PARAMETERS")
            .field("Version", &self.Version)
            .field("EnableProperty", &self.EnableProperty)
            .field("ControlFlags", &self.ControlFlags)
            .field("SourceId", &self.SourceId)
            .field("EnableFilterDesc", &self.EnableFilterDesc)
            .field("FilterDescCount", &self.FilterDescCount)
            .finish()
    }
}
impl ::std::cmp::PartialEq for ENABLE_TRACE_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.EnableProperty == other.EnableProperty && self.ControlFlags == other.ControlFlags && self.SourceId == other.SourceId && self.EnableFilterDesc == other.EnableFilterDesc && self.FilterDescCount == other.FilterDescCount
    }
}
impl ::std::cmp::Eq for ENABLE_TRACE_PARAMETERS {}
unsafe impl ::windows::runtime::Abi for ENABLE_TRACE_PARAMETERS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub struct ENABLE_TRACE_PARAMETERS_V1 {
    pub Version: u32,
    pub EnableProperty: u32,
    pub ControlFlags: u32,
    pub SourceId: ::windows::runtime::GUID,
    pub EnableFilterDesc: *mut EVENT_FILTER_DESCRIPTOR,
}
impl ENABLE_TRACE_PARAMETERS_V1 {}
impl ::std::default::Default for ENABLE_TRACE_PARAMETERS_V1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for ENABLE_TRACE_PARAMETERS_V1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ENABLE_TRACE_PARAMETERS_V1").field("Version", &self.Version).field("EnableProperty", &self.EnableProperty).field("ControlFlags", &self.ControlFlags).field("SourceId", &self.SourceId).field("EnableFilterDesc", &self.EnableFilterDesc).finish()
    }
}
impl ::std::cmp::PartialEq for ENABLE_TRACE_PARAMETERS_V1 {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.EnableProperty == other.EnableProperty && self.ControlFlags == other.ControlFlags && self.SourceId == other.SourceId && self.EnableFilterDesc == other.EnableFilterDesc
    }
}
impl ::std::cmp::Eq for ENABLE_TRACE_PARAMETERS_V1 {}
unsafe impl ::windows::runtime::Abi for ENABLE_TRACE_PARAMETERS_V1 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const ENABLE_TRACE_PARAMETERS_VERSION: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const ENABLE_TRACE_PARAMETERS_VERSION_2: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const ETW_ASCIICHAR_TYPE_VALUE: u32 = 102u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const ETW_ASCIISTRING_TYPE_VALUE: u32 = 103u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const ETW_BOOLEAN_TYPE_VALUE: u32 = 14u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const ETW_BOOL_TYPE_VALUE: u32 = 108u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub struct ETW_BUFFER_CONTEXT {
    pub Anonymous: ETW_BUFFER_CONTEXT_0,
    pub LoggerId: u16,
}
impl ETW_BUFFER_CONTEXT {}
impl ::std::default::Default for ETW_BUFFER_CONTEXT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for ETW_BUFFER_CONTEXT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for ETW_BUFFER_CONTEXT {}
unsafe impl ::windows::runtime::Abi for ETW_BUFFER_CONTEXT {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub union ETW_BUFFER_CONTEXT_0 {
    pub Anonymous: ETW_BUFFER_CONTEXT_0_0,
    pub ProcessorIndex: u16,
}
impl ETW_BUFFER_CONTEXT_0 {}
impl ::std::default::Default for ETW_BUFFER_CONTEXT_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for ETW_BUFFER_CONTEXT_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for ETW_BUFFER_CONTEXT_0 {}
unsafe impl ::windows::runtime::Abi for ETW_BUFFER_CONTEXT_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub struct ETW_BUFFER_CONTEXT_0_0 {
    pub ProcessorNumber: u8,
    pub Alignment: u8,
}
impl ETW_BUFFER_CONTEXT_0_0 {}
impl ::std::default::Default for ETW_BUFFER_CONTEXT_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for ETW_BUFFER_CONTEXT_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("ProcessorNumber", &self.ProcessorNumber).field("Alignment", &self.Alignment).finish()
    }
}
impl ::std::cmp::PartialEq for ETW_BUFFER_CONTEXT_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.ProcessorNumber == other.ProcessorNumber && self.Alignment == other.Alignment
    }
}
impl ::std::cmp::Eq for ETW_BUFFER_CONTEXT_0_0 {}
unsafe impl ::windows::runtime::Abi for ETW_BUFFER_CONTEXT_0_0 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const ETW_BYTE_TYPE_VALUE: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const ETW_CHAR_TYPE_VALUE: u32 = 11u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ETW_COMPRESSION_RESUMPTION_MODE(pub i32);
pub const EtwCompressionModeRestart: ETW_COMPRESSION_RESUMPTION_MODE = ETW_COMPRESSION_RESUMPTION_MODE(0i32);
pub const EtwCompressionModeNoDisable: ETW_COMPRESSION_RESUMPTION_MODE = ETW_COMPRESSION_RESUMPTION_MODE(1i32);
pub const EtwCompressionModeNoRestart: ETW_COMPRESSION_RESUMPTION_MODE = ETW_COMPRESSION_RESUMPTION_MODE(2i32);
impl ::std::convert::From<i32> for ETW_COMPRESSION_RESUMPTION_MODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ETW_COMPRESSION_RESUMPTION_MODE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const ETW_COUNTED_ANSISTRING_TYPE_VALUE: u32 = 109u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const ETW_COUNTED_STRING_TYPE_VALUE: u32 = 104u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const ETW_DATETIME_TYPE_VALUE: u32 = 119u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const ETW_DECIMAL_TYPE_VALUE: u32 = 15u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const ETW_DOUBLE_TYPE_VALUE: u32 = 13u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const ETW_GUID_TYPE_VALUE: u32 = 101u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const ETW_HIDDEN_TYPE_VALUE: u32 = 107u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const ETW_INT16_TYPE_VALUE: u32 = 5u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const ETW_INT32_TYPE_VALUE: u32 = 7u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const ETW_INT64_TYPE_VALUE: u32 = 9u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const ETW_NON_NULL_TERMINATED_STRING_TYPE_VALUE: u32 = 112u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const ETW_NULL_TYPE_VALUE: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const ETW_OBJECT_TYPE_VALUE: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub struct ETW_PMC_COUNTER_OWNER {
    pub OwnerType: ETW_PMC_COUNTER_OWNER_TYPE,
    pub ProfileSource: u32,
    pub OwnerTag: u32,
}
impl ETW_PMC_COUNTER_OWNER {}
impl ::std::default::Default for ETW_PMC_COUNTER_OWNER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for ETW_PMC_COUNTER_OWNER {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ETW_PMC_COUNTER_OWNER").field("OwnerType", &self.OwnerType).field("ProfileSource", &self.ProfileSource).field("OwnerTag", &self.OwnerTag).finish()
    }
}
impl ::std::cmp::PartialEq for ETW_PMC_COUNTER_OWNER {
    fn eq(&self, other: &Self) -> bool {
        self.OwnerType == other.OwnerType && self.ProfileSource == other.ProfileSource && self.OwnerTag == other.OwnerTag
    }
}
impl ::std::cmp::Eq for ETW_PMC_COUNTER_OWNER {}
unsafe impl ::windows::runtime::Abi for ETW_PMC_COUNTER_OWNER {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub struct ETW_PMC_COUNTER_OWNERSHIP_STATUS {
    pub ProcessorNumber: u32,
    pub NumberOfCounters: u32,
    pub CounterOwners: [ETW_PMC_COUNTER_OWNER; 1],
}
impl ETW_PMC_COUNTER_OWNERSHIP_STATUS {}
impl ::std::default::Default for ETW_PMC_COUNTER_OWNERSHIP_STATUS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for ETW_PMC_COUNTER_OWNERSHIP_STATUS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ETW_PMC_COUNTER_OWNERSHIP_STATUS").field("ProcessorNumber", &self.ProcessorNumber).field("NumberOfCounters", &self.NumberOfCounters).field("CounterOwners", &self.CounterOwners).finish()
    }
}
impl ::std::cmp::PartialEq for ETW_PMC_COUNTER_OWNERSHIP_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.ProcessorNumber == other.ProcessorNumber && self.NumberOfCounters == other.NumberOfCounters && self.CounterOwners == other.CounterOwners
    }
}
impl ::std::cmp::Eq for ETW_PMC_COUNTER_OWNERSHIP_STATUS {}
unsafe impl ::windows::runtime::Abi for ETW_PMC_COUNTER_OWNERSHIP_STATUS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ETW_PMC_COUNTER_OWNER_TYPE(pub i32);
pub const EtwPmcOwnerFree: ETW_PMC_COUNTER_OWNER_TYPE = ETW_PMC_COUNTER_OWNER_TYPE(0i32);
pub const EtwPmcOwnerUntagged: ETW_PMC_COUNTER_OWNER_TYPE = ETW_PMC_COUNTER_OWNER_TYPE(1i32);
pub const EtwPmcOwnerTagged: ETW_PMC_COUNTER_OWNER_TYPE = ETW_PMC_COUNTER_OWNER_TYPE(2i32);
pub const EtwPmcOwnerTaggedWithSource: ETW_PMC_COUNTER_OWNER_TYPE = ETW_PMC_COUNTER_OWNER_TYPE(3i32);
impl ::std::convert::From<i32> for ETW_PMC_COUNTER_OWNER_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ETW_PMC_COUNTER_OWNER_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const ETW_POINTER_TYPE_VALUE: u32 = 105u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ETW_PROCESS_HANDLE_INFO_TYPE(pub i32);
pub const EtwQueryPartitionInformation: ETW_PROCESS_HANDLE_INFO_TYPE = ETW_PROCESS_HANDLE_INFO_TYPE(1i32);
pub const EtwQueryPartitionInformationV2: ETW_PROCESS_HANDLE_INFO_TYPE = ETW_PROCESS_HANDLE_INFO_TYPE(2i32);
pub const EtwQueryLastDroppedTimes: ETW_PROCESS_HANDLE_INFO_TYPE = ETW_PROCESS_HANDLE_INFO_TYPE(3i32);
pub const EtwQueryProcessHandleInfoMax: ETW_PROCESS_HANDLE_INFO_TYPE = ETW_PROCESS_HANDLE_INFO_TYPE(4i32);
impl ::std::convert::From<i32> for ETW_PROCESS_HANDLE_INFO_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ETW_PROCESS_HANDLE_INFO_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ETW_PROVIDER_TRAIT_TYPE(pub i32);
pub const EtwProviderTraitTypeGroup: ETW_PROVIDER_TRAIT_TYPE = ETW_PROVIDER_TRAIT_TYPE(1i32);
pub const EtwProviderTraitDecodeGuid: ETW_PROVIDER_TRAIT_TYPE = ETW_PROVIDER_TRAIT_TYPE(2i32);
pub const EtwProviderTraitTypeMax: ETW_PROVIDER_TRAIT_TYPE = ETW_PROVIDER_TRAIT_TYPE(3i32);
impl ::std::convert::From<i32> for ETW_PROVIDER_TRAIT_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ETW_PROVIDER_TRAIT_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const ETW_PTVECTOR_TYPE_VALUE: u32 = 117u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const ETW_REDUCED_ANSISTRING_TYPE_VALUE: u32 = 113u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const ETW_REDUCED_STRING_TYPE_VALUE: u32 = 114u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const ETW_REFRENCE_TYPE_VALUE: u32 = 120u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const ETW_REVERSED_COUNTED_ANSISTRING_TYPE_VALUE: u32 = 111u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const ETW_REVERSED_COUNTED_STRING_TYPE_VALUE: u32 = 110u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const ETW_SBYTE_TYPE_VALUE: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const ETW_SID_TYPE_VALUE: u32 = 115u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const ETW_SINGLE_TYPE_VALUE: u32 = 12u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const ETW_SIZET_TYPE_VALUE: u32 = 106u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const ETW_STRING_TYPE_VALUE: u32 = 2u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub struct ETW_TRACE_PARTITION_INFORMATION {
    pub PartitionId: ::windows::runtime::GUID,
    pub ParentId: ::windows::runtime::GUID,
    pub QpcOffsetFromRoot: i64,
    pub PartitionType: u32,
}
impl ETW_TRACE_PARTITION_INFORMATION {}
impl ::std::default::Default for ETW_TRACE_PARTITION_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for ETW_TRACE_PARTITION_INFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ETW_TRACE_PARTITION_INFORMATION").field("PartitionId", &self.PartitionId).field("ParentId", &self.ParentId).field("QpcOffsetFromRoot", &self.QpcOffsetFromRoot).field("PartitionType", &self.PartitionType).finish()
    }
}
impl ::std::cmp::PartialEq for ETW_TRACE_PARTITION_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.PartitionId == other.PartitionId && self.ParentId == other.ParentId && self.QpcOffsetFromRoot == other.QpcOffsetFromRoot && self.PartitionType == other.PartitionType
    }
}
impl ::std::cmp::Eq for ETW_TRACE_PARTITION_INFORMATION {}
unsafe impl ::windows::runtime::Abi for ETW_TRACE_PARTITION_INFORMATION {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
pub struct ETW_TRACE_PARTITION_INFORMATION_V2 {
    pub QpcOffsetFromRoot: i64,
    pub PartitionType: u32,
    pub PartitionId: super::super::super::Foundation::PWSTR,
    pub ParentId: super::super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ETW_TRACE_PARTITION_INFORMATION_V2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for ETW_TRACE_PARTITION_INFORMATION_V2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for ETW_TRACE_PARTITION_INFORMATION_V2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ETW_TRACE_PARTITION_INFORMATION_V2").field("QpcOffsetFromRoot", &self.QpcOffsetFromRoot).field("PartitionType", &self.PartitionType).field("PartitionId", &self.PartitionId).field("ParentId", &self.ParentId).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for ETW_TRACE_PARTITION_INFORMATION_V2 {
    fn eq(&self, other: &Self) -> bool {
        self.QpcOffsetFromRoot == other.QpcOffsetFromRoot && self.PartitionType == other.PartitionType && self.PartitionId == other.PartitionId && self.ParentId == other.ParentId
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for ETW_TRACE_PARTITION_INFORMATION_V2 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for ETW_TRACE_PARTITION_INFORMATION_V2 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const ETW_UINT16_TYPE_VALUE: u32 = 6u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const ETW_UINT32_TYPE_VALUE: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const ETW_UINT64_TYPE_VALUE: u32 = 10u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const ETW_VARIANT_TYPE_VALUE: u32 = 116u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const ETW_WMITIME_TYPE_VALUE: u32 = 118u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct EVENTSECURITYOPERATION(pub i32);
pub const EventSecuritySetDACL: EVENTSECURITYOPERATION = EVENTSECURITYOPERATION(0i32);
pub const EventSecuritySetSACL: EVENTSECURITYOPERATION = EVENTSECURITYOPERATION(1i32);
pub const EventSecurityAddDACL: EVENTSECURITYOPERATION = EVENTSECURITYOPERATION(2i32);
pub const EventSecurityAddSACL: EVENTSECURITYOPERATION = EVENTSECURITYOPERATION(3i32);
pub const EventSecurityMax: EVENTSECURITYOPERATION = EVENTSECURITYOPERATION(4i32);
impl ::std::convert::From<i32> for EVENTSECURITYOPERATION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for EVENTSECURITYOPERATION {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_ACTIVITY_CTRL_CREATE_ID: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_ACTIVITY_CTRL_CREATE_SET_ID: u32 = 5u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_ACTIVITY_CTRL_GET_ID: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_ACTIVITY_CTRL_GET_SET_ID: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_ACTIVITY_CTRL_SET_ID: u32 = 2u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub struct EVENT_DATA_DESCRIPTOR {
    pub Ptr: u64,
    pub Size: u32,
    pub Anonymous: EVENT_DATA_DESCRIPTOR_0,
}
impl EVENT_DATA_DESCRIPTOR {}
impl ::std::default::Default for EVENT_DATA_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for EVENT_DATA_DESCRIPTOR {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for EVENT_DATA_DESCRIPTOR {}
unsafe impl ::windows::runtime::Abi for EVENT_DATA_DESCRIPTOR {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub union EVENT_DATA_DESCRIPTOR_0 {
    pub Reserved: u32,
    pub Anonymous: EVENT_DATA_DESCRIPTOR_0_0,
}
impl EVENT_DATA_DESCRIPTOR_0 {}
impl ::std::default::Default for EVENT_DATA_DESCRIPTOR_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for EVENT_DATA_DESCRIPTOR_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for EVENT_DATA_DESCRIPTOR_0 {}
unsafe impl ::windows::runtime::Abi for EVENT_DATA_DESCRIPTOR_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub struct EVENT_DATA_DESCRIPTOR_0_0 {
    pub Type: u8,
    pub Reserved1: u8,
    pub Reserved2: u16,
}
impl EVENT_DATA_DESCRIPTOR_0_0 {}
impl ::std::default::Default for EVENT_DATA_DESCRIPTOR_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for EVENT_DATA_DESCRIPTOR_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("Type", &self.Type).field("Reserved1", &self.Reserved1).field("Reserved2", &self.Reserved2).finish()
    }
}
impl ::std::cmp::PartialEq for EVENT_DATA_DESCRIPTOR_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Reserved1 == other.Reserved1 && self.Reserved2 == other.Reserved2
    }
}
impl ::std::cmp::Eq for EVENT_DATA_DESCRIPTOR_0_0 {}
unsafe impl ::windows::runtime::Abi for EVENT_DATA_DESCRIPTOR_0_0 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_DATA_DESCRIPTOR_TYPE_EVENT_METADATA: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_DATA_DESCRIPTOR_TYPE_NONE: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_DATA_DESCRIPTOR_TYPE_PROVIDER_METADATA: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_DATA_DESCRIPTOR_TYPE_TIMESTAMP_OVERRIDE: u32 = 3u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub struct EVENT_DESCRIPTOR {
    pub Id: u16,
    pub Version: u8,
    pub Channel: u8,
    pub Level: u8,
    pub Opcode: u8,
    pub Task: u16,
    pub Keyword: u64,
}
impl EVENT_DESCRIPTOR {}
impl ::std::default::Default for EVENT_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for EVENT_DESCRIPTOR {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("EVENT_DESCRIPTOR").field("Id", &self.Id).field("Version", &self.Version).field("Channel", &self.Channel).field("Level", &self.Level).field("Opcode", &self.Opcode).field("Task", &self.Task).field("Keyword", &self.Keyword).finish()
    }
}
impl ::std::cmp::PartialEq for EVENT_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.Id == other.Id && self.Version == other.Version && self.Channel == other.Channel && self.Level == other.Level && self.Opcode == other.Opcode && self.Task == other.Task && self.Keyword == other.Keyword
    }
}
impl ::std::cmp::Eq for EVENT_DESCRIPTOR {}
unsafe impl ::windows::runtime::Abi for EVENT_DESCRIPTOR {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_ENABLE_PROPERTY_ENABLE_KEYWORD_0: u32 = 64u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_ENABLE_PROPERTY_ENABLE_SILOS: u32 = 1024u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_ENABLE_PROPERTY_EVENT_KEY: u32 = 256u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_ENABLE_PROPERTY_EXCLUDE_INPRIVATE: u32 = 512u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_ENABLE_PROPERTY_IGNORE_KEYWORD_0: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_ENABLE_PROPERTY_PROCESS_START_KEY: u32 = 128u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_ENABLE_PROPERTY_PROVIDER_GROUP: u32 = 32u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_ENABLE_PROPERTY_PSM_KEY: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_ENABLE_PROPERTY_SID: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_ENABLE_PROPERTY_SOURCE_CONTAINER_TRACKING: u32 = 2048u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_ENABLE_PROPERTY_STACK_TRACE: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_ENABLE_PROPERTY_TS_ID: u32 = 2u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub struct EVENT_EXTENDED_ITEM_EVENT_KEY {
    pub Key: u64,
}
impl EVENT_EXTENDED_ITEM_EVENT_KEY {}
impl ::std::default::Default for EVENT_EXTENDED_ITEM_EVENT_KEY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for EVENT_EXTENDED_ITEM_EVENT_KEY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("EVENT_EXTENDED_ITEM_EVENT_KEY").field("Key", &self.Key).finish()
    }
}
impl ::std::cmp::PartialEq for EVENT_EXTENDED_ITEM_EVENT_KEY {
    fn eq(&self, other: &Self) -> bool {
        self.Key == other.Key
    }
}
impl ::std::cmp::Eq for EVENT_EXTENDED_ITEM_EVENT_KEY {}
unsafe impl ::windows::runtime::Abi for EVENT_EXTENDED_ITEM_EVENT_KEY {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub struct EVENT_EXTENDED_ITEM_INSTANCE {
    pub InstanceId: u32,
    pub ParentInstanceId: u32,
    pub ParentGuid: ::windows::runtime::GUID,
}
impl EVENT_EXTENDED_ITEM_INSTANCE {}
impl ::std::default::Default for EVENT_EXTENDED_ITEM_INSTANCE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for EVENT_EXTENDED_ITEM_INSTANCE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("EVENT_EXTENDED_ITEM_INSTANCE").field("InstanceId", &self.InstanceId).field("ParentInstanceId", &self.ParentInstanceId).field("ParentGuid", &self.ParentGuid).finish()
    }
}
impl ::std::cmp::PartialEq for EVENT_EXTENDED_ITEM_INSTANCE {
    fn eq(&self, other: &Self) -> bool {
        self.InstanceId == other.InstanceId && self.ParentInstanceId == other.ParentInstanceId && self.ParentGuid == other.ParentGuid
    }
}
impl ::std::cmp::Eq for EVENT_EXTENDED_ITEM_INSTANCE {}
unsafe impl ::windows::runtime::Abi for EVENT_EXTENDED_ITEM_INSTANCE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub struct EVENT_EXTENDED_ITEM_PEBS_INDEX {
    pub PebsIndex: u64,
}
impl EVENT_EXTENDED_ITEM_PEBS_INDEX {}
impl ::std::default::Default for EVENT_EXTENDED_ITEM_PEBS_INDEX {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for EVENT_EXTENDED_ITEM_PEBS_INDEX {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("EVENT_EXTENDED_ITEM_PEBS_INDEX").field("PebsIndex", &self.PebsIndex).finish()
    }
}
impl ::std::cmp::PartialEq for EVENT_EXTENDED_ITEM_PEBS_INDEX {
    fn eq(&self, other: &Self) -> bool {
        self.PebsIndex == other.PebsIndex
    }
}
impl ::std::cmp::Eq for EVENT_EXTENDED_ITEM_PEBS_INDEX {}
unsafe impl ::windows::runtime::Abi for EVENT_EXTENDED_ITEM_PEBS_INDEX {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub struct EVENT_EXTENDED_ITEM_PMC_COUNTERS {
    pub Counter: [u64; 1],
}
impl EVENT_EXTENDED_ITEM_PMC_COUNTERS {}
impl ::std::default::Default for EVENT_EXTENDED_ITEM_PMC_COUNTERS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for EVENT_EXTENDED_ITEM_PMC_COUNTERS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("EVENT_EXTENDED_ITEM_PMC_COUNTERS").field("Counter", &self.Counter).finish()
    }
}
impl ::std::cmp::PartialEq for EVENT_EXTENDED_ITEM_PMC_COUNTERS {
    fn eq(&self, other: &Self) -> bool {
        self.Counter == other.Counter
    }
}
impl ::std::cmp::Eq for EVENT_EXTENDED_ITEM_PMC_COUNTERS {}
unsafe impl ::windows::runtime::Abi for EVENT_EXTENDED_ITEM_PMC_COUNTERS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub struct EVENT_EXTENDED_ITEM_PROCESS_START_KEY {
    pub ProcessStartKey: u64,
}
impl EVENT_EXTENDED_ITEM_PROCESS_START_KEY {}
impl ::std::default::Default for EVENT_EXTENDED_ITEM_PROCESS_START_KEY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for EVENT_EXTENDED_ITEM_PROCESS_START_KEY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("EVENT_EXTENDED_ITEM_PROCESS_START_KEY").field("ProcessStartKey", &self.ProcessStartKey).finish()
    }
}
impl ::std::cmp::PartialEq for EVENT_EXTENDED_ITEM_PROCESS_START_KEY {
    fn eq(&self, other: &Self) -> bool {
        self.ProcessStartKey == other.ProcessStartKey
    }
}
impl ::std::cmp::Eq for EVENT_EXTENDED_ITEM_PROCESS_START_KEY {}
unsafe impl ::windows::runtime::Abi for EVENT_EXTENDED_ITEM_PROCESS_START_KEY {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub struct EVENT_EXTENDED_ITEM_RELATED_ACTIVITYID {
    pub RelatedActivityId: ::windows::runtime::GUID,
}
impl EVENT_EXTENDED_ITEM_RELATED_ACTIVITYID {}
impl ::std::default::Default for EVENT_EXTENDED_ITEM_RELATED_ACTIVITYID {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for EVENT_EXTENDED_ITEM_RELATED_ACTIVITYID {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("EVENT_EXTENDED_ITEM_RELATED_ACTIVITYID").field("RelatedActivityId", &self.RelatedActivityId).finish()
    }
}
impl ::std::cmp::PartialEq for EVENT_EXTENDED_ITEM_RELATED_ACTIVITYID {
    fn eq(&self, other: &Self) -> bool {
        self.RelatedActivityId == other.RelatedActivityId
    }
}
impl ::std::cmp::Eq for EVENT_EXTENDED_ITEM_RELATED_ACTIVITYID {}
unsafe impl ::windows::runtime::Abi for EVENT_EXTENDED_ITEM_RELATED_ACTIVITYID {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub struct EVENT_EXTENDED_ITEM_STACK_KEY32 {
    pub MatchId: u64,
    pub StackKey: u32,
    pub Padding: u32,
}
impl EVENT_EXTENDED_ITEM_STACK_KEY32 {}
impl ::std::default::Default for EVENT_EXTENDED_ITEM_STACK_KEY32 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for EVENT_EXTENDED_ITEM_STACK_KEY32 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("EVENT_EXTENDED_ITEM_STACK_KEY32").field("MatchId", &self.MatchId).field("StackKey", &self.StackKey).field("Padding", &self.Padding).finish()
    }
}
impl ::std::cmp::PartialEq for EVENT_EXTENDED_ITEM_STACK_KEY32 {
    fn eq(&self, other: &Self) -> bool {
        self.MatchId == other.MatchId && self.StackKey == other.StackKey && self.Padding == other.Padding
    }
}
impl ::std::cmp::Eq for EVENT_EXTENDED_ITEM_STACK_KEY32 {}
unsafe impl ::windows::runtime::Abi for EVENT_EXTENDED_ITEM_STACK_KEY32 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub struct EVENT_EXTENDED_ITEM_STACK_KEY64 {
    pub MatchId: u64,
    pub StackKey: u64,
}
impl EVENT_EXTENDED_ITEM_STACK_KEY64 {}
impl ::std::default::Default for EVENT_EXTENDED_ITEM_STACK_KEY64 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for EVENT_EXTENDED_ITEM_STACK_KEY64 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("EVENT_EXTENDED_ITEM_STACK_KEY64").field("MatchId", &self.MatchId).field("StackKey", &self.StackKey).finish()
    }
}
impl ::std::cmp::PartialEq for EVENT_EXTENDED_ITEM_STACK_KEY64 {
    fn eq(&self, other: &Self) -> bool {
        self.MatchId == other.MatchId && self.StackKey == other.StackKey
    }
}
impl ::std::cmp::Eq for EVENT_EXTENDED_ITEM_STACK_KEY64 {}
unsafe impl ::windows::runtime::Abi for EVENT_EXTENDED_ITEM_STACK_KEY64 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub struct EVENT_EXTENDED_ITEM_STACK_TRACE32 {
    pub MatchId: u64,
    pub Address: [u32; 1],
}
impl EVENT_EXTENDED_ITEM_STACK_TRACE32 {}
impl ::std::default::Default for EVENT_EXTENDED_ITEM_STACK_TRACE32 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for EVENT_EXTENDED_ITEM_STACK_TRACE32 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("EVENT_EXTENDED_ITEM_STACK_TRACE32").field("MatchId", &self.MatchId).field("Address", &self.Address).finish()
    }
}
impl ::std::cmp::PartialEq for EVENT_EXTENDED_ITEM_STACK_TRACE32 {
    fn eq(&self, other: &Self) -> bool {
        self.MatchId == other.MatchId && self.Address == other.Address
    }
}
impl ::std::cmp::Eq for EVENT_EXTENDED_ITEM_STACK_TRACE32 {}
unsafe impl ::windows::runtime::Abi for EVENT_EXTENDED_ITEM_STACK_TRACE32 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub struct EVENT_EXTENDED_ITEM_STACK_TRACE64 {
    pub MatchId: u64,
    pub Address: [u64; 1],
}
impl EVENT_EXTENDED_ITEM_STACK_TRACE64 {}
impl ::std::default::Default for EVENT_EXTENDED_ITEM_STACK_TRACE64 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for EVENT_EXTENDED_ITEM_STACK_TRACE64 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("EVENT_EXTENDED_ITEM_STACK_TRACE64").field("MatchId", &self.MatchId).field("Address", &self.Address).finish()
    }
}
impl ::std::cmp::PartialEq for EVENT_EXTENDED_ITEM_STACK_TRACE64 {
    fn eq(&self, other: &Self) -> bool {
        self.MatchId == other.MatchId && self.Address == other.Address
    }
}
impl ::std::cmp::Eq for EVENT_EXTENDED_ITEM_STACK_TRACE64 {}
unsafe impl ::windows::runtime::Abi for EVENT_EXTENDED_ITEM_STACK_TRACE64 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub struct EVENT_EXTENDED_ITEM_TS_ID {
    pub SessionId: u32,
}
impl EVENT_EXTENDED_ITEM_TS_ID {}
impl ::std::default::Default for EVENT_EXTENDED_ITEM_TS_ID {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for EVENT_EXTENDED_ITEM_TS_ID {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("EVENT_EXTENDED_ITEM_TS_ID").field("SessionId", &self.SessionId).finish()
    }
}
impl ::std::cmp::PartialEq for EVENT_EXTENDED_ITEM_TS_ID {
    fn eq(&self, other: &Self) -> bool {
        self.SessionId == other.SessionId
    }
}
impl ::std::cmp::Eq for EVENT_EXTENDED_ITEM_TS_ID {}
unsafe impl ::windows::runtime::Abi for EVENT_EXTENDED_ITEM_TS_ID {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct EVENT_FIELD_TYPE(pub i32);
pub const EventKeywordInformation: EVENT_FIELD_TYPE = EVENT_FIELD_TYPE(0i32);
pub const EventLevelInformation: EVENT_FIELD_TYPE = EVENT_FIELD_TYPE(1i32);
pub const EventChannelInformation: EVENT_FIELD_TYPE = EVENT_FIELD_TYPE(2i32);
pub const EventTaskInformation: EVENT_FIELD_TYPE = EVENT_FIELD_TYPE(3i32);
pub const EventOpcodeInformation: EVENT_FIELD_TYPE = EVENT_FIELD_TYPE(4i32);
pub const EventInformationMax: EVENT_FIELD_TYPE = EVENT_FIELD_TYPE(5i32);
impl ::std::convert::From<i32> for EVENT_FIELD_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for EVENT_FIELD_TYPE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub struct EVENT_FILTER_DESCRIPTOR {
    pub Ptr: u64,
    pub Size: u32,
    pub Type: u32,
}
impl EVENT_FILTER_DESCRIPTOR {}
impl ::std::default::Default for EVENT_FILTER_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for EVENT_FILTER_DESCRIPTOR {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("EVENT_FILTER_DESCRIPTOR").field("Ptr", &self.Ptr).field("Size", &self.Size).field("Type", &self.Type).finish()
    }
}
impl ::std::cmp::PartialEq for EVENT_FILTER_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.Ptr == other.Ptr && self.Size == other.Size && self.Type == other.Type
    }
}
impl ::std::cmp::Eq for EVENT_FILTER_DESCRIPTOR {}
unsafe impl ::windows::runtime::Abi for EVENT_FILTER_DESCRIPTOR {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
pub struct EVENT_FILTER_EVENT_ID {
    pub FilterIn: super::super::super::Foundation::BOOLEAN,
    pub Reserved: u8,
    pub Count: u16,
    pub Events: [u16; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl EVENT_FILTER_EVENT_ID {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for EVENT_FILTER_EVENT_ID {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for EVENT_FILTER_EVENT_ID {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("EVENT_FILTER_EVENT_ID").field("FilterIn", &self.FilterIn).field("Reserved", &self.Reserved).field("Count", &self.Count).field("Events", &self.Events).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for EVENT_FILTER_EVENT_ID {
    fn eq(&self, other: &Self) -> bool {
        self.FilterIn == other.FilterIn && self.Reserved == other.Reserved && self.Count == other.Count && self.Events == other.Events
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for EVENT_FILTER_EVENT_ID {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for EVENT_FILTER_EVENT_ID {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
pub struct EVENT_FILTER_EVENT_NAME {
    pub MatchAnyKeyword: u64,
    pub MatchAllKeyword: u64,
    pub Level: u8,
    pub FilterIn: super::super::super::Foundation::BOOLEAN,
    pub NameCount: u16,
    pub Names: [u8; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl EVENT_FILTER_EVENT_NAME {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for EVENT_FILTER_EVENT_NAME {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for EVENT_FILTER_EVENT_NAME {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("EVENT_FILTER_EVENT_NAME").field("MatchAnyKeyword", &self.MatchAnyKeyword).field("MatchAllKeyword", &self.MatchAllKeyword).field("Level", &self.Level).field("FilterIn", &self.FilterIn).field("NameCount", &self.NameCount).field("Names", &self.Names).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for EVENT_FILTER_EVENT_NAME {
    fn eq(&self, other: &Self) -> bool {
        self.MatchAnyKeyword == other.MatchAnyKeyword && self.MatchAllKeyword == other.MatchAllKeyword && self.Level == other.Level && self.FilterIn == other.FilterIn && self.NameCount == other.NameCount && self.Names == other.Names
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for EVENT_FILTER_EVENT_NAME {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for EVENT_FILTER_EVENT_NAME {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub struct EVENT_FILTER_HEADER {
    pub Id: u16,
    pub Version: u8,
    pub Reserved: [u8; 5],
    pub InstanceId: u64,
    pub Size: u32,
    pub NextOffset: u32,
}
impl EVENT_FILTER_HEADER {}
impl ::std::default::Default for EVENT_FILTER_HEADER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for EVENT_FILTER_HEADER {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("EVENT_FILTER_HEADER").field("Id", &self.Id).field("Version", &self.Version).field("Reserved", &self.Reserved).field("InstanceId", &self.InstanceId).field("Size", &self.Size).field("NextOffset", &self.NextOffset).finish()
    }
}
impl ::std::cmp::PartialEq for EVENT_FILTER_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.Id == other.Id && self.Version == other.Version && self.Reserved == other.Reserved && self.InstanceId == other.InstanceId && self.Size == other.Size && self.NextOffset == other.NextOffset
    }
}
impl ::std::cmp::Eq for EVENT_FILTER_HEADER {}
unsafe impl ::windows::runtime::Abi for EVENT_FILTER_HEADER {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
pub struct EVENT_FILTER_LEVEL_KW {
    pub MatchAnyKeyword: u64,
    pub MatchAllKeyword: u64,
    pub Level: u8,
    pub FilterIn: super::super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl EVENT_FILTER_LEVEL_KW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for EVENT_FILTER_LEVEL_KW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for EVENT_FILTER_LEVEL_KW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("EVENT_FILTER_LEVEL_KW").field("MatchAnyKeyword", &self.MatchAnyKeyword).field("MatchAllKeyword", &self.MatchAllKeyword).field("Level", &self.Level).field("FilterIn", &self.FilterIn).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for EVENT_FILTER_LEVEL_KW {
    fn eq(&self, other: &Self) -> bool {
        self.MatchAnyKeyword == other.MatchAnyKeyword && self.MatchAllKeyword == other.MatchAllKeyword && self.Level == other.Level && self.FilterIn == other.FilterIn
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for EVENT_FILTER_LEVEL_KW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for EVENT_FILTER_LEVEL_KW {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_FILTER_TYPE_CONTAINER: u32 = 2147516416u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_FILTER_TYPE_EVENT_ID: u32 = 2147484160u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_FILTER_TYPE_EVENT_NAME: u32 = 2147484672u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_FILTER_TYPE_EXECUTABLE_NAME: u32 = 2147483656u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_FILTER_TYPE_NONE: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_FILTER_TYPE_PACKAGE_APP_ID: u32 = 2147483680u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_FILTER_TYPE_PACKAGE_ID: u32 = 2147483664u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_FILTER_TYPE_PAYLOAD: u32 = 2147483904u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_FILTER_TYPE_PID: u32 = 2147483652u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_FILTER_TYPE_SCHEMATIZED: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_FILTER_TYPE_STACKWALK: u32 = 2147487744u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_FILTER_TYPE_STACKWALK_LEVEL_KW: u32 = 2147500032u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_FILTER_TYPE_STACKWALK_NAME: u32 = 2147491840u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_FILTER_TYPE_SYSTEM_FLAGS: u32 = 2147483649u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_FILTER_TYPE_TRACEHANDLE: u32 = 2147483650u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub struct EVENT_HEADER {
    pub Size: u16,
    pub HeaderType: u16,
    pub Flags: u16,
    pub EventProperty: u16,
    pub ThreadId: u32,
    pub ProcessId: u32,
    pub TimeStamp: i64,
    pub ProviderId: ::windows::runtime::GUID,
    pub EventDescriptor: EVENT_DESCRIPTOR,
    pub Anonymous: EVENT_HEADER_0,
    pub ActivityId: ::windows::runtime::GUID,
}
impl EVENT_HEADER {}
impl ::std::default::Default for EVENT_HEADER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for EVENT_HEADER {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for EVENT_HEADER {}
unsafe impl ::windows::runtime::Abi for EVENT_HEADER {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub union EVENT_HEADER_0 {
    pub Anonymous: EVENT_HEADER_0_0,
    pub ProcessorTime: u64,
}
impl EVENT_HEADER_0 {}
impl ::std::default::Default for EVENT_HEADER_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for EVENT_HEADER_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for EVENT_HEADER_0 {}
unsafe impl ::windows::runtime::Abi for EVENT_HEADER_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub struct EVENT_HEADER_0_0 {
    pub KernelTime: u32,
    pub UserTime: u32,
}
impl EVENT_HEADER_0_0 {}
impl ::std::default::Default for EVENT_HEADER_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for EVENT_HEADER_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("KernelTime", &self.KernelTime).field("UserTime", &self.UserTime).finish()
    }
}
impl ::std::cmp::PartialEq for EVENT_HEADER_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.KernelTime == other.KernelTime && self.UserTime == other.UserTime
    }
}
impl ::std::cmp::Eq for EVENT_HEADER_0_0 {}
unsafe impl ::windows::runtime::Abi for EVENT_HEADER_0_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub struct EVENT_HEADER_EXTENDED_DATA_ITEM {
    pub Reserved1: u16,
    pub ExtType: u16,
    pub Anonymous: EVENT_HEADER_EXTENDED_DATA_ITEM_0,
    pub DataSize: u16,
    pub DataPtr: u64,
}
impl EVENT_HEADER_EXTENDED_DATA_ITEM {}
impl ::std::default::Default for EVENT_HEADER_EXTENDED_DATA_ITEM {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for EVENT_HEADER_EXTENDED_DATA_ITEM {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("EVENT_HEADER_EXTENDED_DATA_ITEM").field("Reserved1", &self.Reserved1).field("ExtType", &self.ExtType).field("Anonymous", &self.Anonymous).field("DataSize", &self.DataSize).field("DataPtr", &self.DataPtr).finish()
    }
}
impl ::std::cmp::PartialEq for EVENT_HEADER_EXTENDED_DATA_ITEM {
    fn eq(&self, other: &Self) -> bool {
        self.Reserved1 == other.Reserved1 && self.ExtType == other.ExtType && self.Anonymous == other.Anonymous && self.DataSize == other.DataSize && self.DataPtr == other.DataPtr
    }
}
impl ::std::cmp::Eq for EVENT_HEADER_EXTENDED_DATA_ITEM {}
unsafe impl ::windows::runtime::Abi for EVENT_HEADER_EXTENDED_DATA_ITEM {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub struct EVENT_HEADER_EXTENDED_DATA_ITEM_0 {
    pub _bitfield: u16,
}
impl EVENT_HEADER_EXTENDED_DATA_ITEM_0 {}
impl ::std::default::Default for EVENT_HEADER_EXTENDED_DATA_ITEM_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for EVENT_HEADER_EXTENDED_DATA_ITEM_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::std::cmp::PartialEq for EVENT_HEADER_EXTENDED_DATA_ITEM_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for EVENT_HEADER_EXTENDED_DATA_ITEM_0 {}
unsafe impl ::windows::runtime::Abi for EVENT_HEADER_EXTENDED_DATA_ITEM_0 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_HEADER_EXT_TYPE_CONTAINER_ID: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_HEADER_EXT_TYPE_CONTROL_GUID: u32 = 14u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_HEADER_EXT_TYPE_EVENT_KEY: u32 = 10u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_HEADER_EXT_TYPE_EVENT_SCHEMA_TL: u32 = 11u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_HEADER_EXT_TYPE_INSTANCE_INFO: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_HEADER_EXT_TYPE_MAX: u32 = 19u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_HEADER_EXT_TYPE_PEBS_INDEX: u32 = 7u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_HEADER_EXT_TYPE_PMC_COUNTERS: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_HEADER_EXT_TYPE_PROCESS_START_KEY: u32 = 13u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_HEADER_EXT_TYPE_PROV_TRAITS: u32 = 12u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_HEADER_EXT_TYPE_PSM_KEY: u32 = 9u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_HEADER_EXT_TYPE_QPC_DELTA: u32 = 15u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_HEADER_EXT_TYPE_RELATED_ACTIVITYID: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_HEADER_EXT_TYPE_SID: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_HEADER_EXT_TYPE_STACK_KEY32: u32 = 17u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_HEADER_EXT_TYPE_STACK_KEY64: u32 = 18u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_HEADER_EXT_TYPE_STACK_TRACE32: u32 = 5u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_HEADER_EXT_TYPE_STACK_TRACE64: u32 = 6u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_HEADER_EXT_TYPE_TS_ID: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_HEADER_FLAG_32_BIT_HEADER: u32 = 32u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_HEADER_FLAG_64_BIT_HEADER: u32 = 64u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_HEADER_FLAG_CLASSIC_HEADER: u32 = 256u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_HEADER_FLAG_DECODE_GUID: u32 = 128u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_HEADER_FLAG_EXTENDED_INFO: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_HEADER_FLAG_NO_CPUTIME: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_HEADER_FLAG_PRIVATE_SESSION: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_HEADER_FLAG_PROCESSOR_INDEX: u32 = 512u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_HEADER_FLAG_STRING_ONLY: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_HEADER_FLAG_TRACE_MESSAGE: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_HEADER_PROPERTY_FORWARDED_XML: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_HEADER_PROPERTY_LEGACY_EVENTLOG: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_HEADER_PROPERTY_RELOGGABLE: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_HEADER_PROPERTY_XML: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct EVENT_INFO_CLASS(pub i32);
pub const EventProviderBinaryTrackInfo: EVENT_INFO_CLASS = EVENT_INFO_CLASS(0i32);
pub const EventProviderSetReserved1: EVENT_INFO_CLASS = EVENT_INFO_CLASS(1i32);
pub const EventProviderSetTraits: EVENT_INFO_CLASS = EVENT_INFO_CLASS(2i32);
pub const EventProviderUseDescriptorType: EVENT_INFO_CLASS = EVENT_INFO_CLASS(3i32);
pub const MaxEventInfo: EVENT_INFO_CLASS = EVENT_INFO_CLASS(4i32);
impl ::std::convert::From<i32> for EVENT_INFO_CLASS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for EVENT_INFO_CLASS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub struct EVENT_INSTANCE_HEADER {
    pub Size: u16,
    pub Anonymous1: EVENT_INSTANCE_HEADER_0,
    pub Anonymous2: EVENT_INSTANCE_HEADER_1,
    pub ThreadId: u32,
    pub ProcessId: u32,
    pub TimeStamp: i64,
    pub RegHandle: u64,
    pub InstanceId: u32,
    pub ParentInstanceId: u32,
    pub Anonymous3: EVENT_INSTANCE_HEADER_2,
    pub ParentRegHandle: u64,
}
impl EVENT_INSTANCE_HEADER {}
impl ::std::default::Default for EVENT_INSTANCE_HEADER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for EVENT_INSTANCE_HEADER {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for EVENT_INSTANCE_HEADER {}
unsafe impl ::windows::runtime::Abi for EVENT_INSTANCE_HEADER {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub union EVENT_INSTANCE_HEADER_0 {
    pub FieldTypeFlags: u16,
    pub Anonymous: EVENT_INSTANCE_HEADER_0_0,
}
impl EVENT_INSTANCE_HEADER_0 {}
impl ::std::default::Default for EVENT_INSTANCE_HEADER_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for EVENT_INSTANCE_HEADER_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for EVENT_INSTANCE_HEADER_0 {}
unsafe impl ::windows::runtime::Abi for EVENT_INSTANCE_HEADER_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub struct EVENT_INSTANCE_HEADER_0_0 {
    pub HeaderType: u8,
    pub MarkerFlags: u8,
}
impl EVENT_INSTANCE_HEADER_0_0 {}
impl ::std::default::Default for EVENT_INSTANCE_HEADER_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for EVENT_INSTANCE_HEADER_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("HeaderType", &self.HeaderType).field("MarkerFlags", &self.MarkerFlags).finish()
    }
}
impl ::std::cmp::PartialEq for EVENT_INSTANCE_HEADER_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.HeaderType == other.HeaderType && self.MarkerFlags == other.MarkerFlags
    }
}
impl ::std::cmp::Eq for EVENT_INSTANCE_HEADER_0_0 {}
unsafe impl ::windows::runtime::Abi for EVENT_INSTANCE_HEADER_0_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub union EVENT_INSTANCE_HEADER_1 {
    pub Version: u32,
    pub Class: EVENT_INSTANCE_HEADER_1_0,
}
impl EVENT_INSTANCE_HEADER_1 {}
impl ::std::default::Default for EVENT_INSTANCE_HEADER_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for EVENT_INSTANCE_HEADER_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for EVENT_INSTANCE_HEADER_1 {}
unsafe impl ::windows::runtime::Abi for EVENT_INSTANCE_HEADER_1 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub struct EVENT_INSTANCE_HEADER_1_0 {
    pub Type: u8,
    pub Level: u8,
    pub Version: u16,
}
impl EVENT_INSTANCE_HEADER_1_0 {}
impl ::std::default::Default for EVENT_INSTANCE_HEADER_1_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for EVENT_INSTANCE_HEADER_1_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Class_e__Struct").field("Type", &self.Type).field("Level", &self.Level).field("Version", &self.Version).finish()
    }
}
impl ::std::cmp::PartialEq for EVENT_INSTANCE_HEADER_1_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Level == other.Level && self.Version == other.Version
    }
}
impl ::std::cmp::Eq for EVENT_INSTANCE_HEADER_1_0 {}
unsafe impl ::windows::runtime::Abi for EVENT_INSTANCE_HEADER_1_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub union EVENT_INSTANCE_HEADER_2 {
    pub Anonymous1: EVENT_INSTANCE_HEADER_2_0,
    pub ProcessorTime: u64,
    pub Anonymous2: EVENT_INSTANCE_HEADER_2_1,
}
impl EVENT_INSTANCE_HEADER_2 {}
impl ::std::default::Default for EVENT_INSTANCE_HEADER_2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for EVENT_INSTANCE_HEADER_2 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for EVENT_INSTANCE_HEADER_2 {}
unsafe impl ::windows::runtime::Abi for EVENT_INSTANCE_HEADER_2 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub struct EVENT_INSTANCE_HEADER_2_0 {
    pub KernelTime: u32,
    pub UserTime: u32,
}
impl EVENT_INSTANCE_HEADER_2_0 {}
impl ::std::default::Default for EVENT_INSTANCE_HEADER_2_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for EVENT_INSTANCE_HEADER_2_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous1_e__Struct").field("KernelTime", &self.KernelTime).field("UserTime", &self.UserTime).finish()
    }
}
impl ::std::cmp::PartialEq for EVENT_INSTANCE_HEADER_2_0 {
    fn eq(&self, other: &Self) -> bool {
        self.KernelTime == other.KernelTime && self.UserTime == other.UserTime
    }
}
impl ::std::cmp::Eq for EVENT_INSTANCE_HEADER_2_0 {}
unsafe impl ::windows::runtime::Abi for EVENT_INSTANCE_HEADER_2_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub struct EVENT_INSTANCE_HEADER_2_1 {
    pub EventId: u32,
    pub Flags: u32,
}
impl EVENT_INSTANCE_HEADER_2_1 {}
impl ::std::default::Default for EVENT_INSTANCE_HEADER_2_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for EVENT_INSTANCE_HEADER_2_1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous2_e__Struct").field("EventId", &self.EventId).field("Flags", &self.Flags).finish()
    }
}
impl ::std::cmp::PartialEq for EVENT_INSTANCE_HEADER_2_1 {
    fn eq(&self, other: &Self) -> bool {
        self.EventId == other.EventId && self.Flags == other.Flags
    }
}
impl ::std::cmp::Eq for EVENT_INSTANCE_HEADER_2_1 {}
unsafe impl ::windows::runtime::Abi for EVENT_INSTANCE_HEADER_2_1 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
pub struct EVENT_INSTANCE_INFO {
    pub RegHandle: super::super::super::Foundation::HANDLE,
    pub InstanceId: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl EVENT_INSTANCE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for EVENT_INSTANCE_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for EVENT_INSTANCE_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("EVENT_INSTANCE_INFO").field("RegHandle", &self.RegHandle).field("InstanceId", &self.InstanceId).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for EVENT_INSTANCE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.RegHandle == other.RegHandle && self.InstanceId == other.InstanceId
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for EVENT_INSTANCE_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for EVENT_INSTANCE_INFO {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub struct EVENT_MAP_ENTRY {
    pub OutputOffset: u32,
    pub Anonymous: EVENT_MAP_ENTRY_0,
}
impl EVENT_MAP_ENTRY {}
impl ::std::default::Default for EVENT_MAP_ENTRY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for EVENT_MAP_ENTRY {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for EVENT_MAP_ENTRY {}
unsafe impl ::windows::runtime::Abi for EVENT_MAP_ENTRY {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub union EVENT_MAP_ENTRY_0 {
    pub Value: u32,
    pub InputOffset: u32,
}
impl EVENT_MAP_ENTRY_0 {}
impl ::std::default::Default for EVENT_MAP_ENTRY_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for EVENT_MAP_ENTRY_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for EVENT_MAP_ENTRY_0 {}
unsafe impl ::windows::runtime::Abi for EVENT_MAP_ENTRY_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub struct EVENT_MAP_INFO {
    pub NameOffset: u32,
    pub Flag: MAP_FLAGS,
    pub EntryCount: u32,
    pub Anonymous: EVENT_MAP_INFO_0,
    pub MapEntryArray: [EVENT_MAP_ENTRY; 1],
}
impl EVENT_MAP_INFO {}
impl ::std::default::Default for EVENT_MAP_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for EVENT_MAP_INFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for EVENT_MAP_INFO {}
unsafe impl ::windows::runtime::Abi for EVENT_MAP_INFO {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub union EVENT_MAP_INFO_0 {
    pub MapEntryValueType: MAP_VALUETYPE,
    pub FormatStringOffset: u32,
}
impl EVENT_MAP_INFO_0 {}
impl ::std::default::Default for EVENT_MAP_INFO_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for EVENT_MAP_INFO_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for EVENT_MAP_INFO_0 {}
unsafe impl ::windows::runtime::Abi for EVENT_MAP_INFO_0 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_MAX_LEVEL: u32 = 255u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_MIN_LEVEL: u32 = 0u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub struct EVENT_PROPERTY_INFO {
    pub Flags: PROPERTY_FLAGS,
    pub NameOffset: u32,
    pub Anonymous1: EVENT_PROPERTY_INFO_0,
    pub Anonymous2: EVENT_PROPERTY_INFO_1,
    pub Anonymous3: EVENT_PROPERTY_INFO_2,
    pub Anonymous4: EVENT_PROPERTY_INFO_3,
}
impl EVENT_PROPERTY_INFO {}
impl ::std::default::Default for EVENT_PROPERTY_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for EVENT_PROPERTY_INFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for EVENT_PROPERTY_INFO {}
unsafe impl ::windows::runtime::Abi for EVENT_PROPERTY_INFO {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub union EVENT_PROPERTY_INFO_0 {
    pub nonStructType: EVENT_PROPERTY_INFO_0_1,
    pub structType: EVENT_PROPERTY_INFO_0_2,
    pub customSchemaType: EVENT_PROPERTY_INFO_0_0,
}
impl EVENT_PROPERTY_INFO_0 {}
impl ::std::default::Default for EVENT_PROPERTY_INFO_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for EVENT_PROPERTY_INFO_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for EVENT_PROPERTY_INFO_0 {}
unsafe impl ::windows::runtime::Abi for EVENT_PROPERTY_INFO_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub struct EVENT_PROPERTY_INFO_0_0 {
    pub InType: u16,
    pub OutType: u16,
    pub CustomSchemaOffset: u32,
}
impl EVENT_PROPERTY_INFO_0_0 {}
impl ::std::default::Default for EVENT_PROPERTY_INFO_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for EVENT_PROPERTY_INFO_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_customSchemaType").field("InType", &self.InType).field("OutType", &self.OutType).field("CustomSchemaOffset", &self.CustomSchemaOffset).finish()
    }
}
impl ::std::cmp::PartialEq for EVENT_PROPERTY_INFO_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.InType == other.InType && self.OutType == other.OutType && self.CustomSchemaOffset == other.CustomSchemaOffset
    }
}
impl ::std::cmp::Eq for EVENT_PROPERTY_INFO_0_0 {}
unsafe impl ::windows::runtime::Abi for EVENT_PROPERTY_INFO_0_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub struct EVENT_PROPERTY_INFO_0_1 {
    pub InType: u16,
    pub OutType: u16,
    pub MapNameOffset: u32,
}
impl EVENT_PROPERTY_INFO_0_1 {}
impl ::std::default::Default for EVENT_PROPERTY_INFO_0_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for EVENT_PROPERTY_INFO_0_1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_nonStructType").field("InType", &self.InType).field("OutType", &self.OutType).field("MapNameOffset", &self.MapNameOffset).finish()
    }
}
impl ::std::cmp::PartialEq for EVENT_PROPERTY_INFO_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.InType == other.InType && self.OutType == other.OutType && self.MapNameOffset == other.MapNameOffset
    }
}
impl ::std::cmp::Eq for EVENT_PROPERTY_INFO_0_1 {}
unsafe impl ::windows::runtime::Abi for EVENT_PROPERTY_INFO_0_1 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub struct EVENT_PROPERTY_INFO_0_2 {
    pub StructStartIndex: u16,
    pub NumOfStructMembers: u16,
    pub padding: u32,
}
impl EVENT_PROPERTY_INFO_0_2 {}
impl ::std::default::Default for EVENT_PROPERTY_INFO_0_2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for EVENT_PROPERTY_INFO_0_2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_structType").field("StructStartIndex", &self.StructStartIndex).field("NumOfStructMembers", &self.NumOfStructMembers).field("padding", &self.padding).finish()
    }
}
impl ::std::cmp::PartialEq for EVENT_PROPERTY_INFO_0_2 {
    fn eq(&self, other: &Self) -> bool {
        self.StructStartIndex == other.StructStartIndex && self.NumOfStructMembers == other.NumOfStructMembers && self.padding == other.padding
    }
}
impl ::std::cmp::Eq for EVENT_PROPERTY_INFO_0_2 {}
unsafe impl ::windows::runtime::Abi for EVENT_PROPERTY_INFO_0_2 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub union EVENT_PROPERTY_INFO_1 {
    pub count: u16,
    pub countPropertyIndex: u16,
}
impl EVENT_PROPERTY_INFO_1 {}
impl ::std::default::Default for EVENT_PROPERTY_INFO_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for EVENT_PROPERTY_INFO_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for EVENT_PROPERTY_INFO_1 {}
unsafe impl ::windows::runtime::Abi for EVENT_PROPERTY_INFO_1 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub union EVENT_PROPERTY_INFO_2 {
    pub length: u16,
    pub lengthPropertyIndex: u16,
}
impl EVENT_PROPERTY_INFO_2 {}
impl ::std::default::Default for EVENT_PROPERTY_INFO_2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for EVENT_PROPERTY_INFO_2 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for EVENT_PROPERTY_INFO_2 {}
unsafe impl ::windows::runtime::Abi for EVENT_PROPERTY_INFO_2 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub union EVENT_PROPERTY_INFO_3 {
    pub Reserved: u32,
    pub Anonymous: EVENT_PROPERTY_INFO_3_0,
}
impl EVENT_PROPERTY_INFO_3 {}
impl ::std::default::Default for EVENT_PROPERTY_INFO_3 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for EVENT_PROPERTY_INFO_3 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for EVENT_PROPERTY_INFO_3 {}
unsafe impl ::windows::runtime::Abi for EVENT_PROPERTY_INFO_3 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub struct EVENT_PROPERTY_INFO_3_0 {
    pub _bitfield: u32,
}
impl EVENT_PROPERTY_INFO_3_0 {}
impl ::std::default::Default for EVENT_PROPERTY_INFO_3_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for EVENT_PROPERTY_INFO_3_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::std::cmp::PartialEq for EVENT_PROPERTY_INFO_3_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for EVENT_PROPERTY_INFO_3_0 {}
unsafe impl ::windows::runtime::Abi for EVENT_PROPERTY_INFO_3_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub struct EVENT_RECORD {
    pub EventHeader: EVENT_HEADER,
    pub BufferContext: ETW_BUFFER_CONTEXT,
    pub ExtendedDataCount: u16,
    pub UserDataLength: u16,
    pub ExtendedData: *mut EVENT_HEADER_EXTENDED_DATA_ITEM,
    pub UserData: *mut ::std::ffi::c_void,
    pub UserContext: *mut ::std::ffi::c_void,
}
impl EVENT_RECORD {}
impl ::std::default::Default for EVENT_RECORD {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for EVENT_RECORD {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for EVENT_RECORD {}
unsafe impl ::windows::runtime::Abi for EVENT_RECORD {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub struct EVENT_TRACE {
    pub Header: EVENT_TRACE_HEADER,
    pub InstanceId: u32,
    pub ParentInstanceId: u32,
    pub ParentGuid: ::windows::runtime::GUID,
    pub MofData: *mut ::std::ffi::c_void,
    pub MofLength: u32,
    pub Anonymous: EVENT_TRACE_0,
}
impl EVENT_TRACE {}
impl ::std::default::Default for EVENT_TRACE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for EVENT_TRACE {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for EVENT_TRACE {}
unsafe impl ::windows::runtime::Abi for EVENT_TRACE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub union EVENT_TRACE_0 {
    pub ClientContext: u32,
    pub BufferContext: ETW_BUFFER_CONTEXT,
}
impl EVENT_TRACE_0 {}
impl ::std::default::Default for EVENT_TRACE_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for EVENT_TRACE_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for EVENT_TRACE_0 {}
unsafe impl ::windows::runtime::Abi for EVENT_TRACE_0 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_ADDTO_TRIAGE_DUMP: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_ADD_HEADER_MODE: u32 = 4096u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_BUFFERING_MODE: u32 = 1024u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_COMPRESSED_MODE: u32 = 67108864u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct EVENT_TRACE_CONTROL(pub u32);
pub const EVENT_TRACE_CONTROL_FLUSH: EVENT_TRACE_CONTROL = EVENT_TRACE_CONTROL(3u32);
pub const EVENT_TRACE_CONTROL_QUERY: EVENT_TRACE_CONTROL = EVENT_TRACE_CONTROL(0u32);
pub const EVENT_TRACE_CONTROL_STOP: EVENT_TRACE_CONTROL = EVENT_TRACE_CONTROL(1u32);
pub const EVENT_TRACE_CONTROL_UPDATE: EVENT_TRACE_CONTROL = EVENT_TRACE_CONTROL(2u32);
impl ::std::convert::From<u32> for EVENT_TRACE_CONTROL {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for EVENT_TRACE_CONTROL {
    type Abi = Self;
}
impl ::std::ops::BitOr for EVENT_TRACE_CONTROL {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for EVENT_TRACE_CONTROL {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for EVENT_TRACE_CONTROL {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for EVENT_TRACE_CONTROL {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for EVENT_TRACE_CONTROL {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_CONTROL_CONVERT_TO_REALTIME: u32 = 5u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_CONTROL_INCREMENT_FILE: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_DELAY_OPEN_FILE_MODE: u32 = 512u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_FILE_MODE_APPEND: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_FILE_MODE_CIRCULAR: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_FILE_MODE_NEWFILE: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_FILE_MODE_NONE: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_FILE_MODE_PREALLOCATE: u32 = 32u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_FILE_MODE_SEQUENTIAL: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct EVENT_TRACE_FLAG(pub u32);
pub const EVENT_TRACE_FLAG_ALPC: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(1048576u32);
pub const EVENT_TRACE_FLAG_CSWITCH: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(16u32);
pub const EVENT_TRACE_FLAG_DBGPRINT: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(262144u32);
pub const EVENT_TRACE_FLAG_DISK_FILE_IO: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(512u32);
pub const EVENT_TRACE_FLAG_DISK_IO: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(256u32);
pub const EVENT_TRACE_FLAG_DISK_IO_INIT: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(1024u32);
pub const EVENT_TRACE_FLAG_DISPATCHER: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(2048u32);
pub const EVENT_TRACE_FLAG_DPC: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(32u32);
pub const EVENT_TRACE_FLAG_DRIVER: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(8388608u32);
pub const EVENT_TRACE_FLAG_FILE_IO: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(33554432u32);
pub const EVENT_TRACE_FLAG_FILE_IO_INIT: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(67108864u32);
pub const EVENT_TRACE_FLAG_IMAGE_LOAD: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(4u32);
pub const EVENT_TRACE_FLAG_INTERRUPT: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(64u32);
pub const EVENT_TRACE_FLAG_JOB: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(524288u32);
pub const EVENT_TRACE_FLAG_MEMORY_HARD_FAULTS: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(8192u32);
pub const EVENT_TRACE_FLAG_MEMORY_PAGE_FAULTS: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(4096u32);
pub const EVENT_TRACE_FLAG_NETWORK_TCPIP: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(65536u32);
pub const EVENT_TRACE_FLAG_NO_SYSCONFIG: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(268435456u32);
pub const EVENT_TRACE_FLAG_PROCESS: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(1u32);
pub const EVENT_TRACE_FLAG_PROCESS_COUNTERS: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(8u32);
pub const EVENT_TRACE_FLAG_PROFILE: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(16777216u32);
pub const EVENT_TRACE_FLAG_REGISTRY: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(131072u32);
pub const EVENT_TRACE_FLAG_SPLIT_IO: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(2097152u32);
pub const EVENT_TRACE_FLAG_SYSTEMCALL: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(128u32);
pub const EVENT_TRACE_FLAG_THREAD: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(2u32);
pub const EVENT_TRACE_FLAG_VAMAP: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(32768u32);
pub const EVENT_TRACE_FLAG_VIRTUAL_ALLOC: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(16384u32);
impl ::std::convert::From<u32> for EVENT_TRACE_FLAG {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for EVENT_TRACE_FLAG {
    type Abi = Self;
}
impl ::std::ops::BitOr for EVENT_TRACE_FLAG {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for EVENT_TRACE_FLAG {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for EVENT_TRACE_FLAG {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for EVENT_TRACE_FLAG {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for EVENT_TRACE_FLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_FLAG_DEBUG_EVENTS: u32 = 4194304u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_FLAG_ENABLE_RESERVE: u32 = 536870912u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_FLAG_EXTENSION: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_FLAG_FORWARD_WMI: u32 = 1073741824u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub struct EVENT_TRACE_HEADER {
    pub Size: u16,
    pub Anonymous1: EVENT_TRACE_HEADER_0,
    pub Anonymous2: EVENT_TRACE_HEADER_1,
    pub ThreadId: u32,
    pub ProcessId: u32,
    pub TimeStamp: i64,
    pub Anonymous3: EVENT_TRACE_HEADER_2,
    pub Anonymous4: EVENT_TRACE_HEADER_3,
}
impl EVENT_TRACE_HEADER {}
impl ::std::default::Default for EVENT_TRACE_HEADER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for EVENT_TRACE_HEADER {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for EVENT_TRACE_HEADER {}
unsafe impl ::windows::runtime::Abi for EVENT_TRACE_HEADER {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub union EVENT_TRACE_HEADER_0 {
    pub FieldTypeFlags: u16,
    pub Anonymous: EVENT_TRACE_HEADER_0_0,
}
impl EVENT_TRACE_HEADER_0 {}
impl ::std::default::Default for EVENT_TRACE_HEADER_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for EVENT_TRACE_HEADER_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for EVENT_TRACE_HEADER_0 {}
unsafe impl ::windows::runtime::Abi for EVENT_TRACE_HEADER_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub struct EVENT_TRACE_HEADER_0_0 {
    pub HeaderType: u8,
    pub MarkerFlags: u8,
}
impl EVENT_TRACE_HEADER_0_0 {}
impl ::std::default::Default for EVENT_TRACE_HEADER_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for EVENT_TRACE_HEADER_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("HeaderType", &self.HeaderType).field("MarkerFlags", &self.MarkerFlags).finish()
    }
}
impl ::std::cmp::PartialEq for EVENT_TRACE_HEADER_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.HeaderType == other.HeaderType && self.MarkerFlags == other.MarkerFlags
    }
}
impl ::std::cmp::Eq for EVENT_TRACE_HEADER_0_0 {}
unsafe impl ::windows::runtime::Abi for EVENT_TRACE_HEADER_0_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub union EVENT_TRACE_HEADER_1 {
    pub Version: u32,
    pub Class: EVENT_TRACE_HEADER_1_0,
}
impl EVENT_TRACE_HEADER_1 {}
impl ::std::default::Default for EVENT_TRACE_HEADER_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for EVENT_TRACE_HEADER_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for EVENT_TRACE_HEADER_1 {}
unsafe impl ::windows::runtime::Abi for EVENT_TRACE_HEADER_1 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub struct EVENT_TRACE_HEADER_1_0 {
    pub Type: u8,
    pub Level: u8,
    pub Version: u16,
}
impl EVENT_TRACE_HEADER_1_0 {}
impl ::std::default::Default for EVENT_TRACE_HEADER_1_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for EVENT_TRACE_HEADER_1_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Class_e__Struct").field("Type", &self.Type).field("Level", &self.Level).field("Version", &self.Version).finish()
    }
}
impl ::std::cmp::PartialEq for EVENT_TRACE_HEADER_1_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Level == other.Level && self.Version == other.Version
    }
}
impl ::std::cmp::Eq for EVENT_TRACE_HEADER_1_0 {}
unsafe impl ::windows::runtime::Abi for EVENT_TRACE_HEADER_1_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub union EVENT_TRACE_HEADER_2 {
    pub Guid: ::windows::runtime::GUID,
    pub GuidPtr: u64,
}
impl EVENT_TRACE_HEADER_2 {}
impl ::std::default::Default for EVENT_TRACE_HEADER_2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for EVENT_TRACE_HEADER_2 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for EVENT_TRACE_HEADER_2 {}
unsafe impl ::windows::runtime::Abi for EVENT_TRACE_HEADER_2 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub union EVENT_TRACE_HEADER_3 {
    pub Anonymous1: EVENT_TRACE_HEADER_3_0,
    pub ProcessorTime: u64,
    pub Anonymous2: EVENT_TRACE_HEADER_3_1,
}
impl EVENT_TRACE_HEADER_3 {}
impl ::std::default::Default for EVENT_TRACE_HEADER_3 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for EVENT_TRACE_HEADER_3 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for EVENT_TRACE_HEADER_3 {}
unsafe impl ::windows::runtime::Abi for EVENT_TRACE_HEADER_3 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub struct EVENT_TRACE_HEADER_3_0 {
    pub KernelTime: u32,
    pub UserTime: u32,
}
impl EVENT_TRACE_HEADER_3_0 {}
impl ::std::default::Default for EVENT_TRACE_HEADER_3_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for EVENT_TRACE_HEADER_3_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous1_e__Struct").field("KernelTime", &self.KernelTime).field("UserTime", &self.UserTime).finish()
    }
}
impl ::std::cmp::PartialEq for EVENT_TRACE_HEADER_3_0 {
    fn eq(&self, other: &Self) -> bool {
        self.KernelTime == other.KernelTime && self.UserTime == other.UserTime
    }
}
impl ::std::cmp::Eq for EVENT_TRACE_HEADER_3_0 {}
unsafe impl ::windows::runtime::Abi for EVENT_TRACE_HEADER_3_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub struct EVENT_TRACE_HEADER_3_1 {
    pub ClientContext: u32,
    pub Flags: u32,
}
impl EVENT_TRACE_HEADER_3_1 {}
impl ::std::default::Default for EVENT_TRACE_HEADER_3_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for EVENT_TRACE_HEADER_3_1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous2_e__Struct").field("ClientContext", &self.ClientContext).field("Flags", &self.Flags).finish()
    }
}
impl ::std::cmp::PartialEq for EVENT_TRACE_HEADER_3_1 {
    fn eq(&self, other: &Self) -> bool {
        self.ClientContext == other.ClientContext && self.Flags == other.Flags
    }
}
impl ::std::cmp::Eq for EVENT_TRACE_HEADER_3_1 {}
unsafe impl ::windows::runtime::Abi for EVENT_TRACE_HEADER_3_1 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_INDEPENDENT_SESSION_MODE: u32 = 134217728u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::std::clone::Clone for EVENT_TRACE_LOGFILEA {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`, `Win32_System_Time`*"]
pub struct EVENT_TRACE_LOGFILEA {
    pub LogFileName: super::super::super::Foundation::PSTR,
    pub LoggerName: super::super::super::Foundation::PSTR,
    pub CurrentTime: i64,
    pub BuffersRead: u32,
    pub Anonymous1: EVENT_TRACE_LOGFILEA_0,
    pub CurrentEvent: EVENT_TRACE,
    pub LogfileHeader: TRACE_LOGFILE_HEADER,
    pub BufferCallback: ::std::option::Option<PEVENT_TRACE_BUFFER_CALLBACKA>,
    pub BufferSize: u32,
    pub Filled: u32,
    pub EventsLost: u32,
    pub Anonymous2: EVENT_TRACE_LOGFILEA_1,
    pub IsKernelTrace: u32,
    pub Context: *mut ::std::ffi::c_void,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl EVENT_TRACE_LOGFILEA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::std::default::Default for EVENT_TRACE_LOGFILEA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::std::cmp::PartialEq for EVENT_TRACE_LOGFILEA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::std::cmp::Eq for EVENT_TRACE_LOGFILEA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
unsafe impl ::windows::runtime::Abi for EVENT_TRACE_LOGFILEA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub union EVENT_TRACE_LOGFILEA_0 {
    pub LogFileMode: u32,
    pub ProcessTraceMode: u32,
}
impl EVENT_TRACE_LOGFILEA_0 {}
impl ::std::default::Default for EVENT_TRACE_LOGFILEA_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for EVENT_TRACE_LOGFILEA_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for EVENT_TRACE_LOGFILEA_0 {}
unsafe impl ::windows::runtime::Abi for EVENT_TRACE_LOGFILEA_0 {
    type Abi = Self;
}
impl ::std::clone::Clone for EVENT_TRACE_LOGFILEA_1 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub union EVENT_TRACE_LOGFILEA_1 {
    pub EventCallback: ::windows::runtime::RawPtr,
    pub EventRecordCallback: ::windows::runtime::RawPtr,
}
impl EVENT_TRACE_LOGFILEA_1 {}
impl ::std::default::Default for EVENT_TRACE_LOGFILEA_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for EVENT_TRACE_LOGFILEA_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for EVENT_TRACE_LOGFILEA_1 {}
unsafe impl ::windows::runtime::Abi for EVENT_TRACE_LOGFILEA_1 {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::std::clone::Clone for EVENT_TRACE_LOGFILEW {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`, `Win32_System_Time`*"]
pub struct EVENT_TRACE_LOGFILEW {
    pub LogFileName: super::super::super::Foundation::PWSTR,
    pub LoggerName: super::super::super::Foundation::PWSTR,
    pub CurrentTime: i64,
    pub BuffersRead: u32,
    pub Anonymous1: EVENT_TRACE_LOGFILEW_0,
    pub CurrentEvent: EVENT_TRACE,
    pub LogfileHeader: TRACE_LOGFILE_HEADER,
    pub BufferCallback: ::std::option::Option<PEVENT_TRACE_BUFFER_CALLBACKW>,
    pub BufferSize: u32,
    pub Filled: u32,
    pub EventsLost: u32,
    pub Anonymous2: EVENT_TRACE_LOGFILEW_1,
    pub IsKernelTrace: u32,
    pub Context: *mut ::std::ffi::c_void,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl EVENT_TRACE_LOGFILEW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::std::default::Default for EVENT_TRACE_LOGFILEW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::std::cmp::PartialEq for EVENT_TRACE_LOGFILEW {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::std::cmp::Eq for EVENT_TRACE_LOGFILEW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
unsafe impl ::windows::runtime::Abi for EVENT_TRACE_LOGFILEW {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub union EVENT_TRACE_LOGFILEW_0 {
    pub LogFileMode: u32,
    pub ProcessTraceMode: u32,
}
impl EVENT_TRACE_LOGFILEW_0 {}
impl ::std::default::Default for EVENT_TRACE_LOGFILEW_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for EVENT_TRACE_LOGFILEW_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for EVENT_TRACE_LOGFILEW_0 {}
unsafe impl ::windows::runtime::Abi for EVENT_TRACE_LOGFILEW_0 {
    type Abi = Self;
}
impl ::std::clone::Clone for EVENT_TRACE_LOGFILEW_1 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub union EVENT_TRACE_LOGFILEW_1 {
    pub EventCallback: ::windows::runtime::RawPtr,
    pub EventRecordCallback: ::windows::runtime::RawPtr,
}
impl EVENT_TRACE_LOGFILEW_1 {}
impl ::std::default::Default for EVENT_TRACE_LOGFILEW_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for EVENT_TRACE_LOGFILEW_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for EVENT_TRACE_LOGFILEW_1 {}
unsafe impl ::windows::runtime::Abi for EVENT_TRACE_LOGFILEW_1 {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_MODE_RESERVED: u32 = 1048576u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_NONSTOPPABLE_MODE: u32 = 64u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_NO_PER_PROCESSOR_BUFFERING: u32 = 268435456u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_PERSIST_ON_HYBRID_SHUTDOWN: u32 = 8388608u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_PRIVATE_IN_PROC: u32 = 131072u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_PRIVATE_LOGGER_MODE: u32 = 2048u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
pub struct EVENT_TRACE_PROPERTIES {
    pub Wnode: WNODE_HEADER,
    pub BufferSize: u32,
    pub MinimumBuffers: u32,
    pub MaximumBuffers: u32,
    pub MaximumFileSize: u32,
    pub LogFileMode: u32,
    pub FlushTimer: u32,
    pub EnableFlags: EVENT_TRACE_FLAG,
    pub Anonymous: EVENT_TRACE_PROPERTIES_0,
    pub NumberOfBuffers: u32,
    pub FreeBuffers: u32,
    pub EventsLost: u32,
    pub BuffersWritten: u32,
    pub LogBuffersLost: u32,
    pub RealTimeBuffersLost: u32,
    pub LoggerThreadId: super::super::super::Foundation::HANDLE,
    pub LogFileNameOffset: u32,
    pub LoggerNameOffset: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl EVENT_TRACE_PROPERTIES {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for EVENT_TRACE_PROPERTIES {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for EVENT_TRACE_PROPERTIES {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for EVENT_TRACE_PROPERTIES {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for EVENT_TRACE_PROPERTIES {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub union EVENT_TRACE_PROPERTIES_0 {
    pub AgeLimit: i32,
    pub FlushThreshold: i32,
}
impl EVENT_TRACE_PROPERTIES_0 {}
impl ::std::default::Default for EVENT_TRACE_PROPERTIES_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for EVENT_TRACE_PROPERTIES_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for EVENT_TRACE_PROPERTIES_0 {}
unsafe impl ::windows::runtime::Abi for EVENT_TRACE_PROPERTIES_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
pub struct EVENT_TRACE_PROPERTIES_V2 {
    pub Wnode: WNODE_HEADER,
    pub BufferSize: u32,
    pub MinimumBuffers: u32,
    pub MaximumBuffers: u32,
    pub MaximumFileSize: u32,
    pub LogFileMode: u32,
    pub FlushTimer: u32,
    pub EnableFlags: EVENT_TRACE_FLAG,
    pub Anonymous1: EVENT_TRACE_PROPERTIES_V2_0,
    pub NumberOfBuffers: u32,
    pub FreeBuffers: u32,
    pub EventsLost: u32,
    pub BuffersWritten: u32,
    pub LogBuffersLost: u32,
    pub RealTimeBuffersLost: u32,
    pub LoggerThreadId: super::super::super::Foundation::HANDLE,
    pub LogFileNameOffset: u32,
    pub LoggerNameOffset: u32,
    pub Anonymous2: EVENT_TRACE_PROPERTIES_V2_1,
    pub FilterDescCount: u32,
    pub FilterDesc: *mut EVENT_FILTER_DESCRIPTOR,
    pub Anonymous3: EVENT_TRACE_PROPERTIES_V2_2,
}
#[cfg(feature = "Win32_Foundation")]
impl EVENT_TRACE_PROPERTIES_V2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for EVENT_TRACE_PROPERTIES_V2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for EVENT_TRACE_PROPERTIES_V2 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for EVENT_TRACE_PROPERTIES_V2 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for EVENT_TRACE_PROPERTIES_V2 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub union EVENT_TRACE_PROPERTIES_V2_0 {
    pub AgeLimit: i32,
    pub FlushThreshold: i32,
}
impl EVENT_TRACE_PROPERTIES_V2_0 {}
impl ::std::default::Default for EVENT_TRACE_PROPERTIES_V2_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for EVENT_TRACE_PROPERTIES_V2_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for EVENT_TRACE_PROPERTIES_V2_0 {}
unsafe impl ::windows::runtime::Abi for EVENT_TRACE_PROPERTIES_V2_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub union EVENT_TRACE_PROPERTIES_V2_1 {
    pub Anonymous: EVENT_TRACE_PROPERTIES_V2_1_0,
    pub V2Control: u32,
}
impl EVENT_TRACE_PROPERTIES_V2_1 {}
impl ::std::default::Default for EVENT_TRACE_PROPERTIES_V2_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for EVENT_TRACE_PROPERTIES_V2_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for EVENT_TRACE_PROPERTIES_V2_1 {}
unsafe impl ::windows::runtime::Abi for EVENT_TRACE_PROPERTIES_V2_1 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub struct EVENT_TRACE_PROPERTIES_V2_1_0 {
    pub _bitfield: u32,
}
impl EVENT_TRACE_PROPERTIES_V2_1_0 {}
impl ::std::default::Default for EVENT_TRACE_PROPERTIES_V2_1_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for EVENT_TRACE_PROPERTIES_V2_1_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::std::cmp::PartialEq for EVENT_TRACE_PROPERTIES_V2_1_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for EVENT_TRACE_PROPERTIES_V2_1_0 {}
unsafe impl ::windows::runtime::Abi for EVENT_TRACE_PROPERTIES_V2_1_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub union EVENT_TRACE_PROPERTIES_V2_2 {
    pub Anonymous: EVENT_TRACE_PROPERTIES_V2_2_0,
    pub V2Options: u64,
}
impl EVENT_TRACE_PROPERTIES_V2_2 {}
impl ::std::default::Default for EVENT_TRACE_PROPERTIES_V2_2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for EVENT_TRACE_PROPERTIES_V2_2 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for EVENT_TRACE_PROPERTIES_V2_2 {}
unsafe impl ::windows::runtime::Abi for EVENT_TRACE_PROPERTIES_V2_2 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub struct EVENT_TRACE_PROPERTIES_V2_2_0 {
    pub _bitfield: u32,
}
impl EVENT_TRACE_PROPERTIES_V2_2_0 {}
impl ::std::default::Default for EVENT_TRACE_PROPERTIES_V2_2_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for EVENT_TRACE_PROPERTIES_V2_2_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::std::cmp::PartialEq for EVENT_TRACE_PROPERTIES_V2_2_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for EVENT_TRACE_PROPERTIES_V2_2_0 {}
unsafe impl ::windows::runtime::Abi for EVENT_TRACE_PROPERTIES_V2_2_0 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_REAL_TIME_MODE: u32 = 256u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_RELOG_MODE: u32 = 65536u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_SECURE_MODE: u32 = 128u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_STOP_ON_HYBRID_SHUTDOWN: u32 = 4194304u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_SYSTEM_LOGGER_MODE: u32 = 33554432u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_ACCEPT: u32 = 15u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_ACKDUP: u32 = 22u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_ACKFULL: u32 = 20u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_ACKPART: u32 = 21u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_CHECKPOINT: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_CONFIG: u32 = 11u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_CONFIG_BOOT: u32 = 37u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_CONFIG_CI_INFO: u32 = 29u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_CONFIG_CPU: u32 = 10u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_CONFIG_DEFRAG: u32 = 31u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_CONFIG_DEVICEFAMILY: u32 = 33u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_CONFIG_DPI: u32 = 28u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_CONFIG_FLIGHTID: u32 = 34u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_CONFIG_IDECHANNEL: u32 = 23u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_CONFIG_IRQ: u32 = 21u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_CONFIG_LOGICALDISK: u32 = 12u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_CONFIG_MACHINEID: u32 = 30u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_CONFIG_MOBILEPLATFORM: u32 = 32u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_CONFIG_NETINFO: u32 = 17u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_CONFIG_NIC: u32 = 13u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_CONFIG_NUMANODE: u32 = 24u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_CONFIG_OPTICALMEDIA: u32 = 18u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_CONFIG_PHYSICALDISK: u32 = 11u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_CONFIG_PLATFORM: u32 = 25u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_CONFIG_PNP: u32 = 22u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_CONFIG_POWER: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_CONFIG_PROCESSOR: u32 = 35u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_CONFIG_PROCESSORGROUP: u32 = 26u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_CONFIG_PROCESSORNUMBER: u32 = 27u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_CONFIG_SERVICES: u32 = 15u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_CONFIG_VIDEO: u32 = 14u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_CONFIG_VIRTUALIZATION: u32 = 36u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_CONNECT: u32 = 12u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_CONNFAIL: u32 = 17u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_COPY_ARP: u32 = 19u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_COPY_TCP: u32 = 18u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_DBGID_RSDS: u32 = 64u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_DC_END: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_DC_START: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_DEQUEUE: u32 = 7u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_DISCONNECT: u32 = 13u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_END: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_EXTENSION: u32 = 5u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_FLT_POSTOP_COMPLETION: u32 = 99u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_FLT_POSTOP_FAILURE: u32 = 101u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_FLT_POSTOP_INIT: u32 = 97u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_FLT_PREOP_COMPLETION: u32 = 98u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_FLT_PREOP_FAILURE: u32 = 100u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_FLT_PREOP_INIT: u32 = 96u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_GUIDMAP: u32 = 10u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_INFO: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_IO_FLUSH: u32 = 14u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_IO_FLUSH_INIT: u32 = 15u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_IO_READ: u32 = 10u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_IO_READ_INIT: u32 = 12u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_IO_REDIRECTED_INIT: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_IO_WRITE: u32 = 11u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_IO_WRITE_INIT: u32 = 13u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_LOAD: u32 = 10u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_MM_AV: u32 = 15u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_MM_COW: u32 = 12u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_MM_DZF: u32 = 11u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_MM_GPF: u32 = 13u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_MM_HPF: u32 = 14u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_MM_TF: u32 = 10u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_OPTICAL_IO_FLUSH: u32 = 57u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_OPTICAL_IO_FLUSH_INIT: u32 = 60u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_OPTICAL_IO_READ: u32 = 55u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_OPTICAL_IO_READ_INIT: u32 = 58u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_OPTICAL_IO_WRITE: u32 = 56u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_OPTICAL_IO_WRITE_INIT: u32 = 59u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_RECEIVE: u32 = 11u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_RECONNECT: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_REGCLOSE: u32 = 27u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_REGCOMMIT: u32 = 30u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_REGCREATE: u32 = 10u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_REGDELETE: u32 = 12u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_REGDELETEVALUE: u32 = 15u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_REGENUMERATEKEY: u32 = 17u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_REGENUMERATEVALUEKEY: u32 = 18u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_REGFLUSH: u32 = 21u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_REGKCBCREATE: u32 = 22u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_REGKCBDELETE: u32 = 23u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_REGKCBRUNDOWNBEGIN: u32 = 24u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_REGKCBRUNDOWNEND: u32 = 25u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_REGMOUNTHIVE: u32 = 33u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_REGOPEN: u32 = 11u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_REGPREPARE: u32 = 31u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_REGQUERY: u32 = 13u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_REGQUERYMULTIPLEVALUE: u32 = 19u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_REGQUERYSECURITY: u32 = 29u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_REGQUERYVALUE: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_REGROLLBACK: u32 = 32u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_REGSETINFORMATION: u32 = 20u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_REGSETSECURITY: u32 = 28u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_REGSETVALUE: u32 = 14u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_REGVIRTUALIZE: u32 = 26u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_REPLY: u32 = 6u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_RESUME: u32 = 7u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_RETRANSMIT: u32 = 14u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_SECURITY: u32 = 13u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_SEND: u32 = 10u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_SIDINFO: u32 = 12u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_START: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_STOP: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_SUSPEND: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_TERMINATE: u32 = 11u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_WINEVT_RECEIVE: u32 = 240u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_TYPE_WINEVT_SEND: u32 = 9u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_USE_GLOBAL_SEQUENCE: u32 = 16384u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_USE_KBYTES_FOR_SIZE: u32 = 8192u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_USE_LOCAL_SEQUENCE: u32 = 32768u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_USE_NOCPUTIME: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_USE_PAGED_MEMORY: u32 = 16777216u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_TRACE_USE_PROCTIME: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_WRITE_FLAG_INPRIVATE: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const EVENT_WRITE_FLAG_NO_FAULTING: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
#[inline]
pub unsafe fn EnableTrace(enable: u32, enableflag: u32, enablelevel: u32, controlguid: *const ::windows::runtime::GUID, tracehandle: u64) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnableTrace(enable: u32, enableflag: u32, enablelevel: u32, controlguid: *const ::windows::runtime::GUID, tracehandle: u64) -> u32;
        }
        ::std::mem::transmute(EnableTrace(::std::mem::transmute(enable), ::std::mem::transmute(enableflag), ::std::mem::transmute(enablelevel), ::std::mem::transmute(controlguid), ::std::mem::transmute(tracehandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
#[inline]
pub unsafe fn EnableTraceEx(providerid: *const ::windows::runtime::GUID, sourceid: *const ::windows::runtime::GUID, tracehandle: u64, isenabled: u32, level: u8, matchanykeyword: u64, matchallkeyword: u64, enableproperty: u32, enablefilterdesc: *const EVENT_FILTER_DESCRIPTOR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnableTraceEx(providerid: *const ::windows::runtime::GUID, sourceid: *const ::windows::runtime::GUID, tracehandle: u64, isenabled: u32, level: u8, matchanykeyword: u64, matchallkeyword: u64, enableproperty: u32, enablefilterdesc: *const EVENT_FILTER_DESCRIPTOR) -> u32;
        }
        ::std::mem::transmute(EnableTraceEx(
            ::std::mem::transmute(providerid),
            ::std::mem::transmute(sourceid),
            ::std::mem::transmute(tracehandle),
            ::std::mem::transmute(isenabled),
            ::std::mem::transmute(level),
            ::std::mem::transmute(matchanykeyword),
            ::std::mem::transmute(matchallkeyword),
            ::std::mem::transmute(enableproperty),
            ::std::mem::transmute(enablefilterdesc),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
#[inline]
pub unsafe fn EnableTraceEx2(tracehandle: u64, providerid: *const ::windows::runtime::GUID, controlcode: u32, level: u8, matchanykeyword: u64, matchallkeyword: u64, timeout: u32, enableparameters: *const ENABLE_TRACE_PARAMETERS) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnableTraceEx2(tracehandle: u64, providerid: *const ::windows::runtime::GUID, controlcode: u32, level: u8, matchanykeyword: u64, matchallkeyword: u64, timeout: u32, enableparameters: *const ENABLE_TRACE_PARAMETERS) -> u32;
        }
        ::std::mem::transmute(EnableTraceEx2(
            ::std::mem::transmute(tracehandle),
            ::std::mem::transmute(providerid),
            ::std::mem::transmute(controlcode),
            ::std::mem::transmute(level),
            ::std::mem::transmute(matchanykeyword),
            ::std::mem::transmute(matchallkeyword),
            ::std::mem::transmute(timeout),
            ::std::mem::transmute(enableparameters),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumerateTraceGuids(guidpropertiesarray: *mut *mut TRACE_GUID_PROPERTIES, propertyarraycount: u32, guidcount: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnumerateTraceGuids(guidpropertiesarray: *mut *mut TRACE_GUID_PROPERTIES, propertyarraycount: u32, guidcount: *mut u32) -> u32;
        }
        ::std::mem::transmute(EnumerateTraceGuids(::std::mem::transmute(guidpropertiesarray), ::std::mem::transmute(propertyarraycount), ::std::mem::transmute(guidcount)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
#[inline]
pub unsafe fn EnumerateTraceGuidsEx(tracequeryinfoclass: TRACE_QUERY_INFO_CLASS, inbuffer: *const ::std::ffi::c_void, inbuffersize: u32, outbuffer: *mut ::std::ffi::c_void, outbuffersize: u32, returnlength: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnumerateTraceGuidsEx(tracequeryinfoclass: TRACE_QUERY_INFO_CLASS, inbuffer: *const ::std::ffi::c_void, inbuffersize: u32, outbuffer: *mut ::std::ffi::c_void, outbuffersize: u32, returnlength: *mut u32) -> u32;
        }
        ::std::mem::transmute(EnumerateTraceGuidsEx(::std::mem::transmute(tracequeryinfoclass), ::std::mem::transmute(inbuffer), ::std::mem::transmute(inbuffersize), ::std::mem::transmute(outbuffer), ::std::mem::transmute(outbuffersize), ::std::mem::transmute(returnlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EventAccessControl<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PSID>, Param4: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BOOLEAN>>(guid: *const ::windows::runtime::GUID, operation: u32, sid: Param2, rights: u32, allowordeny: Param4) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EventAccessControl(guid: *const ::windows::runtime::GUID, operation: u32, sid: super::super::super::Foundation::PSID, rights: u32, allowordeny: super::super::super::Foundation::BOOLEAN) -> u32;
        }
        ::std::mem::transmute(EventAccessControl(::std::mem::transmute(guid), ::std::mem::transmute(operation), sid.into_param().abi(), ::std::mem::transmute(rights), allowordeny.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`, `Win32_Security`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn EventAccessQuery(guid: *const ::windows::runtime::GUID, buffer: *mut super::super::super::Security::SECURITY_DESCRIPTOR, buffersize: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EventAccessQuery(guid: *const ::windows::runtime::GUID, buffer: *mut super::super::super::Security::SECURITY_DESCRIPTOR, buffersize: *mut u32) -> u32;
        }
        ::std::mem::transmute(EventAccessQuery(::std::mem::transmute(guid), ::std::mem::transmute(buffer), ::std::mem::transmute(buffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
#[inline]
pub unsafe fn EventAccessRemove(guid: *const ::windows::runtime::GUID) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EventAccessRemove(guid: *const ::windows::runtime::GUID) -> u32;
        }
        ::std::mem::transmute(EventAccessRemove(::std::mem::transmute(guid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
#[inline]
pub unsafe fn EventActivityIdControl(controlcode: u32, activityid: *mut ::windows::runtime::GUID) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EventActivityIdControl(controlcode: u32, activityid: *mut ::windows::runtime::GUID) -> u32;
        }
        ::std::mem::transmute(EventActivityIdControl(::std::mem::transmute(controlcode), ::std::mem::transmute(activityid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EventEnabled(reghandle: u64, eventdescriptor: *const EVENT_DESCRIPTOR) -> super::super::super::Foundation::BOOLEAN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EventEnabled(reghandle: u64, eventdescriptor: *const EVENT_DESCRIPTOR) -> super::super::super::Foundation::BOOLEAN;
        }
        ::std::mem::transmute(EventEnabled(::std::mem::transmute(reghandle), ::std::mem::transmute(eventdescriptor)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EventProviderEnabled(reghandle: u64, level: u8, keyword: u64) -> super::super::super::Foundation::BOOLEAN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EventProviderEnabled(reghandle: u64, level: u8, keyword: u64) -> super::super::super::Foundation::BOOLEAN;
        }
        ::std::mem::transmute(EventProviderEnabled(::std::mem::transmute(reghandle), ::std::mem::transmute(level), ::std::mem::transmute(keyword)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
#[inline]
pub unsafe fn EventRegister(providerid: *const ::windows::runtime::GUID, enablecallback: ::std::option::Option<PENABLECALLBACK>, callbackcontext: *const ::std::ffi::c_void, reghandle: *mut u64) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EventRegister(providerid: *const ::windows::runtime::GUID, enablecallback: ::windows::runtime::RawPtr, callbackcontext: *const ::std::ffi::c_void, reghandle: *mut u64) -> u32;
        }
        ::std::mem::transmute(EventRegister(::std::mem::transmute(providerid), ::std::mem::transmute(enablecallback), ::std::mem::transmute(callbackcontext), ::std::mem::transmute(reghandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
#[inline]
pub unsafe fn EventSetInformation(reghandle: u64, informationclass: EVENT_INFO_CLASS, eventinformation: *const ::std::ffi::c_void, informationlength: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EventSetInformation(reghandle: u64, informationclass: EVENT_INFO_CLASS, eventinformation: *const ::std::ffi::c_void, informationlength: u32) -> u32;
        }
        ::std::mem::transmute(EventSetInformation(::std::mem::transmute(reghandle), ::std::mem::transmute(informationclass), ::std::mem::transmute(eventinformation), ::std::mem::transmute(informationlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const EventTraceConfigGuid: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(25508453, 16783, 20278, [174, 252, 220, 15, 29, 47, 210, 53]);
pub const EventTraceGuid: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1761466624, 19006, 4561, [132, 244, 0, 0, 248, 4, 100, 227]);
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
#[inline]
pub unsafe fn EventUnregister(reghandle: u64) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EventUnregister(reghandle: u64) -> u32;
        }
        ::std::mem::transmute(EventUnregister(::std::mem::transmute(reghandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
#[inline]
pub unsafe fn EventWrite(reghandle: u64, eventdescriptor: *const EVENT_DESCRIPTOR, userdatacount: u32, userdata: *const EVENT_DATA_DESCRIPTOR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EventWrite(reghandle: u64, eventdescriptor: *const EVENT_DESCRIPTOR, userdatacount: u32, userdata: *const EVENT_DATA_DESCRIPTOR) -> u32;
        }
        ::std::mem::transmute(EventWrite(::std::mem::transmute(reghandle), ::std::mem::transmute(eventdescriptor), ::std::mem::transmute(userdatacount), ::std::mem::transmute(userdata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
#[inline]
pub unsafe fn EventWriteEx(reghandle: u64, eventdescriptor: *const EVENT_DESCRIPTOR, filter: u64, flags: u32, activityid: *const ::windows::runtime::GUID, relatedactivityid: *const ::windows::runtime::GUID, userdatacount: u32, userdata: *const EVENT_DATA_DESCRIPTOR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EventWriteEx(reghandle: u64, eventdescriptor: *const EVENT_DESCRIPTOR, filter: u64, flags: u32, activityid: *const ::windows::runtime::GUID, relatedactivityid: *const ::windows::runtime::GUID, userdatacount: u32, userdata: *const EVENT_DATA_DESCRIPTOR) -> u32;
        }
        ::std::mem::transmute(EventWriteEx(::std::mem::transmute(reghandle), ::std::mem::transmute(eventdescriptor), ::std::mem::transmute(filter), ::std::mem::transmute(flags), ::std::mem::transmute(activityid), ::std::mem::transmute(relatedactivityid), ::std::mem::transmute(userdatacount), ::std::mem::transmute(userdata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EventWriteString<'a, Param3: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(reghandle: u64, level: u8, keyword: u64, string: Param3) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EventWriteString(reghandle: u64, level: u8, keyword: u64, string: super::super::super::Foundation::PWSTR) -> u32;
        }
        ::std::mem::transmute(EventWriteString(::std::mem::transmute(reghandle), ::std::mem::transmute(level), ::std::mem::transmute(keyword), string.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
#[inline]
pub unsafe fn EventWriteTransfer(reghandle: u64, eventdescriptor: *const EVENT_DESCRIPTOR, activityid: *const ::windows::runtime::GUID, relatedactivityid: *const ::windows::runtime::GUID, userdatacount: u32, userdata: *const EVENT_DATA_DESCRIPTOR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EventWriteTransfer(reghandle: u64, eventdescriptor: *const EVENT_DESCRIPTOR, activityid: *const ::windows::runtime::GUID, relatedactivityid: *const ::windows::runtime::GUID, userdatacount: u32, userdata: *const EVENT_DATA_DESCRIPTOR) -> u32;
        }
        ::std::mem::transmute(EventWriteTransfer(::std::mem::transmute(reghandle), ::std::mem::transmute(eventdescriptor), ::std::mem::transmute(activityid), ::std::mem::transmute(relatedactivityid), ::std::mem::transmute(userdatacount), ::std::mem::transmute(userdata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FlushTraceA<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PSTR>>(tracehandle: u64, instancename: Param1, properties: *mut EVENT_TRACE_PROPERTIES) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FlushTraceA(tracehandle: u64, instancename: super::super::super::Foundation::PSTR, properties: *mut EVENT_TRACE_PROPERTIES) -> u32;
        }
        ::std::mem::transmute(FlushTraceA(::std::mem::transmute(tracehandle), instancename.into_param().abi(), ::std::mem::transmute(properties)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FlushTraceW<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(tracehandle: u64, instancename: Param1, properties: *mut EVENT_TRACE_PROPERTIES) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FlushTraceW(tracehandle: u64, instancename: super::super::super::Foundation::PWSTR, properties: *mut EVENT_TRACE_PROPERTIES) -> u32;
        }
        ::std::mem::transmute(FlushTraceW(::std::mem::transmute(tracehandle), instancename.into_param().abi(), ::std::mem::transmute(properties)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
#[inline]
pub unsafe fn GetTraceEnableFlags(tracehandle: u64) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetTraceEnableFlags(tracehandle: u64) -> u32;
        }
        ::std::mem::transmute(GetTraceEnableFlags(::std::mem::transmute(tracehandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
#[inline]
pub unsafe fn GetTraceEnableLevel(tracehandle: u64) -> u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetTraceEnableLevel(tracehandle: u64) -> u8;
        }
        ::std::mem::transmute(GetTraceEnableLevel(::std::mem::transmute(tracehandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
#[inline]
pub unsafe fn GetTraceLoggerHandle(buffer: *const ::std::ffi::c_void) -> u64 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetTraceLoggerHandle(buffer: *const ::std::ffi::c_void) -> u64;
        }
        ::std::mem::transmute(GetTraceLoggerHandle(::std::mem::transmute(buffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ITraceEvent(pub ::windows::runtime::IUnknown);
impl ITraceEvent {
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<ITraceEvent> {
        let mut result__: <ITraceEvent as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<ITraceEvent>(result__)
    }
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub unsafe fn GetUserContext(&self, usercontext: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(usercontext)).ok()
    }
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub unsafe fn GetEventRecord(&self) -> ::windows::runtime::Result<*mut EVENT_RECORD> {
        let mut result__: <*mut EVENT_RECORD as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<*mut EVENT_RECORD>(result__)
    }
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub unsafe fn SetPayload(&self, payload: *const u8, payloadsize: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(payload), ::std::mem::transmute(payloadsize)).ok()
    }
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub unsafe fn SetEventDescriptor(&self, eventdescriptor: *const EVENT_DESCRIPTOR) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(eventdescriptor)).ok()
    }
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub unsafe fn SetProcessId(&self, processid: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(processid)).ok()
    }
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub unsafe fn SetProcessorIndex(&self, processorindex: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(processorindex)).ok()
    }
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub unsafe fn SetThreadId(&self, threadid: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(threadid)).ok()
    }
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub unsafe fn SetThreadTimes(&self, kerneltime: u32, usertime: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(kerneltime), ::std::mem::transmute(usertime)).ok()
    }
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub unsafe fn SetActivityId(&self, activityid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(activityid)).ok()
    }
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub unsafe fn SetTimeStamp(&self, timestamp: *const i64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(timestamp)).ok()
    }
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub unsafe fn SetProviderId(&self, providerid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(providerid)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITraceEvent {
    type Vtable = ITraceEvent_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2362015552, 36904, 20467, [155, 98, 125, 31, 121, 202, 123, 203]);
}
impl ::std::convert::From<ITraceEvent> for ::windows::runtime::IUnknown {
    fn from(value: ITraceEvent) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ITraceEvent> for ::windows::runtime::IUnknown {
    fn from(value: &ITraceEvent) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITraceEvent {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITraceEvent {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITraceEvent_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newevent: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, usercontext: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, eventrecord: *mut *mut EVENT_RECORD) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, payload: *const u8, payloadsize: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, eventdescriptor: *const EVENT_DESCRIPTOR) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, processid: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, processorindex: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, threadid: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, kerneltime: u32, usertime: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, activityid: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, timestamp: *const i64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, providerid: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ITraceEventCallback(pub ::windows::runtime::IUnknown);
impl ITraceEventCallback {
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub unsafe fn OnBeginProcessTrace<'a, Param0: ::windows::runtime::IntoParam<'a, ITraceEvent>, Param1: ::windows::runtime::IntoParam<'a, ITraceRelogger>>(&self, headerevent: Param0, relogger: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), headerevent.into_param().abi(), relogger.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub unsafe fn OnFinalizeProcessTrace<'a, Param0: ::windows::runtime::IntoParam<'a, ITraceRelogger>>(&self, relogger: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), relogger.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub unsafe fn OnEvent<'a, Param0: ::windows::runtime::IntoParam<'a, ITraceEvent>, Param1: ::windows::runtime::IntoParam<'a, ITraceRelogger>>(&self, event: Param0, relogger: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), event.into_param().abi(), relogger.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITraceEventCallback {
    type Vtable = ITraceEventCallback_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1053971713, 22847, 17385, [143, 56, 58, 180, 111, 90, 74, 82]);
}
impl ::std::convert::From<ITraceEventCallback> for ::windows::runtime::IUnknown {
    fn from(value: ITraceEventCallback) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ITraceEventCallback> for ::windows::runtime::IUnknown {
    fn from(value: &ITraceEventCallback) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITraceEventCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITraceEventCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITraceEventCallback_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, headerevent: ::windows::runtime::RawPtr, relogger: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, relogger: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, event: ::windows::runtime::RawPtr, relogger: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ITraceRelogger(pub ::windows::runtime::IUnknown);
impl ITraceRelogger {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
    pub unsafe fn AddLogfileTraceStream<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, logfilename: Param0, usercontext: *const ::std::ffi::c_void) -> ::windows::runtime::Result<u64> {
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), logfilename.into_param().abi(), ::std::mem::transmute(usercontext), &mut result__).from_abi::<u64>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
    pub unsafe fn AddRealtimeTraceStream<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, loggername: Param0, usercontext: *const ::std::ffi::c_void) -> ::windows::runtime::Result<u64> {
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), loggername.into_param().abi(), ::std::mem::transmute(usercontext), &mut result__).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub unsafe fn RegisterCallback<'a, Param0: ::windows::runtime::IntoParam<'a, ITraceEventCallback>>(&self, callback: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), callback.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub unsafe fn Inject<'a, Param0: ::windows::runtime::IntoParam<'a, ITraceEvent>>(&self, event: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), event.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub unsafe fn CreateEventInstance(&self, tracehandle: u64, flags: u32) -> ::windows::runtime::Result<ITraceEvent> {
        let mut result__: <ITraceEvent as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(tracehandle), ::std::mem::transmute(flags), &mut result__).from_abi::<ITraceEvent>(result__)
    }
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub unsafe fn ProcessTrace(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
    pub unsafe fn SetOutputFilename<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, logfilename: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), logfilename.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
    pub unsafe fn SetCompressionMode<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BOOLEAN>>(&self, compressionmode: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), compressionmode.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub unsafe fn Cancel(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITraceRelogger {
    type Vtable = ITraceRelogger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4149521731, 15308, 17030, [128, 9, 156, 93, 162, 20, 232, 78]);
}
impl ::std::convert::From<ITraceRelogger> for ::windows::runtime::IUnknown {
    fn from(value: ITraceRelogger) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ITraceRelogger> for ::windows::runtime::IUnknown {
    fn from(value: &ITraceRelogger) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITraceRelogger {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITraceRelogger {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITraceRelogger_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, logfilename: ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, usercontext: *const ::std::ffi::c_void, tracehandle: *mut u64) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, loggername: ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, usercontext: *const ::std::ffi::c_void, tracehandle: *mut u64) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, callback: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, event: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, tracehandle: u64, flags: u32, event: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, logfilename: ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, compressionmode: super::super::super::Foundation::BOOLEAN) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct MAP_FLAGS(pub i32);
pub const EVENTMAP_INFO_FLAG_MANIFEST_VALUEMAP: MAP_FLAGS = MAP_FLAGS(1i32);
pub const EVENTMAP_INFO_FLAG_MANIFEST_BITMAP: MAP_FLAGS = MAP_FLAGS(2i32);
pub const EVENTMAP_INFO_FLAG_MANIFEST_PATTERNMAP: MAP_FLAGS = MAP_FLAGS(4i32);
pub const EVENTMAP_INFO_FLAG_WBEM_VALUEMAP: MAP_FLAGS = MAP_FLAGS(8i32);
pub const EVENTMAP_INFO_FLAG_WBEM_BITMAP: MAP_FLAGS = MAP_FLAGS(16i32);
pub const EVENTMAP_INFO_FLAG_WBEM_FLAG: MAP_FLAGS = MAP_FLAGS(32i32);
pub const EVENTMAP_INFO_FLAG_WBEM_NO_MAP: MAP_FLAGS = MAP_FLAGS(64i32);
impl ::std::convert::From<i32> for MAP_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MAP_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct MAP_VALUETYPE(pub i32);
pub const EVENTMAP_ENTRY_VALUETYPE_ULONG: MAP_VALUETYPE = MAP_VALUETYPE(0i32);
pub const EVENTMAP_ENTRY_VALUETYPE_STRING: MAP_VALUETYPE = MAP_VALUETYPE(1i32);
impl ::std::convert::From<i32> for MAP_VALUETYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MAP_VALUETYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const MAX_EVENT_DATA_DESCRIPTORS: u32 = 128u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const MAX_EVENT_FILTERS_COUNT: u32 = 13u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const MAX_EVENT_FILTER_DATA_SIZE: u32 = 1024u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const MAX_EVENT_FILTER_EVENT_ID_COUNT: u32 = 64u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const MAX_EVENT_FILTER_EVENT_NAME_SIZE: u32 = 4096u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const MAX_EVENT_FILTER_PAYLOAD_SIZE: u32 = 4096u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const MAX_EVENT_FILTER_PID_COUNT: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const MAX_MOF_FIELDS: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const MAX_PAYLOAD_PREDICATES: u32 = 8u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub struct MOF_FIELD {
    pub DataPtr: u64,
    pub Length: u32,
    pub DataType: u32,
}
impl MOF_FIELD {}
impl ::std::default::Default for MOF_FIELD {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for MOF_FIELD {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MOF_FIELD").field("DataPtr", &self.DataPtr).field("Length", &self.Length).field("DataType", &self.DataType).finish()
    }
}
impl ::std::cmp::PartialEq for MOF_FIELD {
    fn eq(&self, other: &Self) -> bool {
        self.DataPtr == other.DataPtr && self.Length == other.Length && self.DataType == other.DataType
    }
}
impl ::std::cmp::Eq for MOF_FIELD {}
unsafe impl ::windows::runtime::Abi for MOF_FIELD {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub struct OFFSETINSTANCEDATAANDLENGTH {
    pub OffsetInstanceData: u32,
    pub LengthInstanceData: u32,
}
impl OFFSETINSTANCEDATAANDLENGTH {}
impl ::std::default::Default for OFFSETINSTANCEDATAANDLENGTH {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for OFFSETINSTANCEDATAANDLENGTH {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("OFFSETINSTANCEDATAANDLENGTH").field("OffsetInstanceData", &self.OffsetInstanceData).field("LengthInstanceData", &self.LengthInstanceData).finish()
    }
}
impl ::std::cmp::PartialEq for OFFSETINSTANCEDATAANDLENGTH {
    fn eq(&self, other: &Self) -> bool {
        self.OffsetInstanceData == other.OffsetInstanceData && self.LengthInstanceData == other.LengthInstanceData
    }
}
impl ::std::cmp::Eq for OFFSETINSTANCEDATAANDLENGTH {}
unsafe impl ::windows::runtime::Abi for OFFSETINSTANCEDATAANDLENGTH {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`, `Win32_System_Time`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
#[inline]
pub unsafe fn OpenTraceA(logfile: *mut EVENT_TRACE_LOGFILEA) -> u64 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenTraceA(logfile: *mut ::std::mem::ManuallyDrop<EVENT_TRACE_LOGFILEA>) -> u64;
        }
        ::std::mem::transmute(OpenTraceA(::std::mem::transmute(logfile)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`, `Win32_System_Time`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
#[inline]
pub unsafe fn OpenTraceW(logfile: *mut EVENT_TRACE_LOGFILEW) -> u64 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenTraceW(logfile: *mut ::std::mem::ManuallyDrop<EVENT_TRACE_LOGFILEW>) -> u64;
        }
        ::std::mem::transmute(OpenTraceW(::std::mem::transmute(logfile)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
pub struct PAYLOAD_FILTER_PREDICATE {
    pub FieldName: super::super::super::Foundation::PWSTR,
    pub CompareOp: u16,
    pub Value: super::super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl PAYLOAD_FILTER_PREDICATE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for PAYLOAD_FILTER_PREDICATE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for PAYLOAD_FILTER_PREDICATE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PAYLOAD_FILTER_PREDICATE").field("FieldName", &self.FieldName).field("CompareOp", &self.CompareOp).field("Value", &self.Value).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for PAYLOAD_FILTER_PREDICATE {
    fn eq(&self, other: &Self) -> bool {
        self.FieldName == other.FieldName && self.CompareOp == other.CompareOp && self.Value == other.Value
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for PAYLOAD_FILTER_PREDICATE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for PAYLOAD_FILTER_PREDICATE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PAYLOAD_OPERATOR(pub i32);
pub const PAYLOADFIELD_EQ: PAYLOAD_OPERATOR = PAYLOAD_OPERATOR(0i32);
pub const PAYLOADFIELD_NE: PAYLOAD_OPERATOR = PAYLOAD_OPERATOR(1i32);
pub const PAYLOADFIELD_LE: PAYLOAD_OPERATOR = PAYLOAD_OPERATOR(2i32);
pub const PAYLOADFIELD_GT: PAYLOAD_OPERATOR = PAYLOAD_OPERATOR(3i32);
pub const PAYLOADFIELD_LT: PAYLOAD_OPERATOR = PAYLOAD_OPERATOR(4i32);
pub const PAYLOADFIELD_GE: PAYLOAD_OPERATOR = PAYLOAD_OPERATOR(5i32);
pub const PAYLOADFIELD_BETWEEN: PAYLOAD_OPERATOR = PAYLOAD_OPERATOR(6i32);
pub const PAYLOADFIELD_NOTBETWEEN: PAYLOAD_OPERATOR = PAYLOAD_OPERATOR(7i32);
pub const PAYLOADFIELD_MODULO: PAYLOAD_OPERATOR = PAYLOAD_OPERATOR(8i32);
pub const PAYLOADFIELD_CONTAINS: PAYLOAD_OPERATOR = PAYLOAD_OPERATOR(20i32);
pub const PAYLOADFIELD_DOESNTCONTAIN: PAYLOAD_OPERATOR = PAYLOAD_OPERATOR(21i32);
pub const PAYLOADFIELD_IS: PAYLOAD_OPERATOR = PAYLOAD_OPERATOR(30i32);
pub const PAYLOADFIELD_ISNOT: PAYLOAD_OPERATOR = PAYLOAD_OPERATOR(31i32);
pub const PAYLOADFIELD_INVALID: PAYLOAD_OPERATOR = PAYLOAD_OPERATOR(32i32);
impl ::std::convert::From<i32> for PAYLOAD_OPERATOR {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PAYLOAD_OPERATOR {
    type Abi = Self;
}
pub type PENABLECALLBACK = unsafe extern "system" fn(sourceid: *const ::windows::runtime::GUID, isenabled: ENABLECALLBACK_ENABLED_STATE, level: u8, matchanykeyword: u64, matchallkeyword: u64, filterdata: *const EVENT_FILTER_DESCRIPTOR, callbackcontext: *mut ::std::ffi::c_void);
pub type PEVENT_CALLBACK = unsafe extern "system" fn(pevent: *mut EVENT_TRACE);
pub type PEVENT_RECORD_CALLBACK = unsafe extern "system" fn(eventrecord: *mut EVENT_RECORD);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
pub type PEVENT_TRACE_BUFFER_CALLBACKA = unsafe extern "system" fn(logfile: *mut ::std::mem::ManuallyDrop<EVENT_TRACE_LOGFILEA>) -> u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
pub type PEVENT_TRACE_BUFFER_CALLBACKW = unsafe extern "system" fn(logfile: *mut ::std::mem::ManuallyDrop<EVENT_TRACE_LOGFILEW>) -> u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const PROCESS_TRACE_MODE_EVENT_RECORD: u32 = 268435456u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const PROCESS_TRACE_MODE_RAW_TIMESTAMP: u32 = 4096u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const PROCESS_TRACE_MODE_REAL_TIME: u32 = 256u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub struct PROFILE_SOURCE_INFO {
    pub NextEntryOffset: u32,
    pub Source: u32,
    pub MinInterval: u32,
    pub MaxInterval: u32,
    pub Reserved: u64,
    pub Description: [u16; 1],
}
impl PROFILE_SOURCE_INFO {}
impl ::std::default::Default for PROFILE_SOURCE_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PROFILE_SOURCE_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PROFILE_SOURCE_INFO").field("NextEntryOffset", &self.NextEntryOffset).field("Source", &self.Source).field("MinInterval", &self.MinInterval).field("MaxInterval", &self.MaxInterval).field("Reserved", &self.Reserved).field("Description", &self.Description).finish()
    }
}
impl ::std::cmp::PartialEq for PROFILE_SOURCE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.NextEntryOffset == other.NextEntryOffset && self.Source == other.Source && self.MinInterval == other.MinInterval && self.MaxInterval == other.MaxInterval && self.Reserved == other.Reserved && self.Description == other.Description
    }
}
impl ::std::cmp::Eq for PROFILE_SOURCE_INFO {}
unsafe impl ::windows::runtime::Abi for PROFILE_SOURCE_INFO {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub struct PROPERTY_DATA_DESCRIPTOR {
    pub PropertyName: u64,
    pub ArrayIndex: u32,
    pub Reserved: u32,
}
impl PROPERTY_DATA_DESCRIPTOR {}
impl ::std::default::Default for PROPERTY_DATA_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PROPERTY_DATA_DESCRIPTOR {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PROPERTY_DATA_DESCRIPTOR").field("PropertyName", &self.PropertyName).field("ArrayIndex", &self.ArrayIndex).field("Reserved", &self.Reserved).finish()
    }
}
impl ::std::cmp::PartialEq for PROPERTY_DATA_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.PropertyName == other.PropertyName && self.ArrayIndex == other.ArrayIndex && self.Reserved == other.Reserved
    }
}
impl ::std::cmp::Eq for PROPERTY_DATA_DESCRIPTOR {}
unsafe impl ::windows::runtime::Abi for PROPERTY_DATA_DESCRIPTOR {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PROPERTY_FLAGS(pub i32);
pub const PropertyStruct: PROPERTY_FLAGS = PROPERTY_FLAGS(1i32);
pub const PropertyParamLength: PROPERTY_FLAGS = PROPERTY_FLAGS(2i32);
pub const PropertyParamCount: PROPERTY_FLAGS = PROPERTY_FLAGS(4i32);
pub const PropertyWBEMXmlFragment: PROPERTY_FLAGS = PROPERTY_FLAGS(8i32);
pub const PropertyParamFixedLength: PROPERTY_FLAGS = PROPERTY_FLAGS(16i32);
pub const PropertyParamFixedCount: PROPERTY_FLAGS = PROPERTY_FLAGS(32i32);
pub const PropertyHasTags: PROPERTY_FLAGS = PROPERTY_FLAGS(64i32);
pub const PropertyHasCustomSchema: PROPERTY_FLAGS = PROPERTY_FLAGS(128i32);
impl ::std::convert::From<i32> for PROPERTY_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PROPERTY_FLAGS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub struct PROVIDER_ENUMERATION_INFO {
    pub NumberOfProviders: u32,
    pub Reserved: u32,
    pub TraceProviderInfoArray: [TRACE_PROVIDER_INFO; 1],
}
impl PROVIDER_ENUMERATION_INFO {}
impl ::std::default::Default for PROVIDER_ENUMERATION_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PROVIDER_ENUMERATION_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PROVIDER_ENUMERATION_INFO").field("NumberOfProviders", &self.NumberOfProviders).field("Reserved", &self.Reserved).field("TraceProviderInfoArray", &self.TraceProviderInfoArray).finish()
    }
}
impl ::std::cmp::PartialEq for PROVIDER_ENUMERATION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.NumberOfProviders == other.NumberOfProviders && self.Reserved == other.Reserved && self.TraceProviderInfoArray == other.TraceProviderInfoArray
    }
}
impl ::std::cmp::Eq for PROVIDER_ENUMERATION_INFO {}
unsafe impl ::windows::runtime::Abi for PROVIDER_ENUMERATION_INFO {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub struct PROVIDER_EVENT_INFO {
    pub NumberOfEvents: u32,
    pub Reserved: u32,
    pub EventDescriptorsArray: [EVENT_DESCRIPTOR; 1],
}
impl PROVIDER_EVENT_INFO {}
impl ::std::default::Default for PROVIDER_EVENT_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PROVIDER_EVENT_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PROVIDER_EVENT_INFO").field("NumberOfEvents", &self.NumberOfEvents).field("Reserved", &self.Reserved).field("EventDescriptorsArray", &self.EventDescriptorsArray).finish()
    }
}
impl ::std::cmp::PartialEq for PROVIDER_EVENT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.NumberOfEvents == other.NumberOfEvents && self.Reserved == other.Reserved && self.EventDescriptorsArray == other.EventDescriptorsArray
    }
}
impl ::std::cmp::Eq for PROVIDER_EVENT_INFO {}
unsafe impl ::windows::runtime::Abi for PROVIDER_EVENT_INFO {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub struct PROVIDER_FIELD_INFO {
    pub NameOffset: u32,
    pub DescriptionOffset: u32,
    pub Value: u64,
}
impl PROVIDER_FIELD_INFO {}
impl ::std::default::Default for PROVIDER_FIELD_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PROVIDER_FIELD_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PROVIDER_FIELD_INFO").field("NameOffset", &self.NameOffset).field("DescriptionOffset", &self.DescriptionOffset).field("Value", &self.Value).finish()
    }
}
impl ::std::cmp::PartialEq for PROVIDER_FIELD_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.NameOffset == other.NameOffset && self.DescriptionOffset == other.DescriptionOffset && self.Value == other.Value
    }
}
impl ::std::cmp::Eq for PROVIDER_FIELD_INFO {}
unsafe impl ::windows::runtime::Abi for PROVIDER_FIELD_INFO {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub struct PROVIDER_FIELD_INFOARRAY {
    pub NumberOfElements: u32,
    pub FieldType: EVENT_FIELD_TYPE,
    pub FieldInfoArray: [PROVIDER_FIELD_INFO; 1],
}
impl PROVIDER_FIELD_INFOARRAY {}
impl ::std::default::Default for PROVIDER_FIELD_INFOARRAY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PROVIDER_FIELD_INFOARRAY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PROVIDER_FIELD_INFOARRAY").field("NumberOfElements", &self.NumberOfElements).field("FieldType", &self.FieldType).field("FieldInfoArray", &self.FieldInfoArray).finish()
    }
}
impl ::std::cmp::PartialEq for PROVIDER_FIELD_INFOARRAY {
    fn eq(&self, other: &Self) -> bool {
        self.NumberOfElements == other.NumberOfElements && self.FieldType == other.FieldType && self.FieldInfoArray == other.FieldInfoArray
    }
}
impl ::std::cmp::Eq for PROVIDER_FIELD_INFOARRAY {}
unsafe impl ::windows::runtime::Abi for PROVIDER_FIELD_INFOARRAY {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub struct PROVIDER_FILTER_INFO {
    pub Id: u8,
    pub Version: u8,
    pub MessageOffset: u32,
    pub Reserved: u32,
    pub PropertyCount: u32,
    pub EventPropertyInfoArray: [EVENT_PROPERTY_INFO; 1],
}
impl PROVIDER_FILTER_INFO {}
impl ::std::default::Default for PROVIDER_FILTER_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for PROVIDER_FILTER_INFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for PROVIDER_FILTER_INFO {}
unsafe impl ::windows::runtime::Abi for PROVIDER_FILTER_INFO {
    type Abi = Self;
}
pub const PrivateLoggerNotificationGuid: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(899001180, 1066, 19598, [185, 66, 45, 5, 155, 254, 177, 177]);
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ProcessTrace(handlearray: *const u64, handlecount: u32, starttime: *const super::super::super::Foundation::FILETIME, endtime: *const super::super::super::Foundation::FILETIME) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ProcessTrace(handlearray: *const u64, handlecount: u32, starttime: *const super::super::super::Foundation::FILETIME, endtime: *const super::super::super::Foundation::FILETIME) -> u32;
        }
        ::std::mem::transmute(ProcessTrace(::std::mem::transmute(handlearray), ::std::mem::transmute(handlecount), ::std::mem::transmute(starttime), ::std::mem::transmute(endtime)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QueryAllTracesA(propertyarray: *mut *mut EVENT_TRACE_PROPERTIES, propertyarraycount: u32, loggercount: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryAllTracesA(propertyarray: *mut *mut EVENT_TRACE_PROPERTIES, propertyarraycount: u32, loggercount: *mut u32) -> u32;
        }
        ::std::mem::transmute(QueryAllTracesA(::std::mem::transmute(propertyarray), ::std::mem::transmute(propertyarraycount), ::std::mem::transmute(loggercount)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QueryAllTracesW(propertyarray: *mut *mut EVENT_TRACE_PROPERTIES, propertyarraycount: u32, loggercount: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryAllTracesW(propertyarray: *mut *mut EVENT_TRACE_PROPERTIES, propertyarraycount: u32, loggercount: *mut u32) -> u32;
        }
        ::std::mem::transmute(QueryAllTracesW(::std::mem::transmute(propertyarray), ::std::mem::transmute(propertyarraycount), ::std::mem::transmute(loggercount)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QueryTraceA<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PSTR>>(tracehandle: u64, instancename: Param1, properties: *mut EVENT_TRACE_PROPERTIES) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryTraceA(tracehandle: u64, instancename: super::super::super::Foundation::PSTR, properties: *mut EVENT_TRACE_PROPERTIES) -> u32;
        }
        ::std::mem::transmute(QueryTraceA(::std::mem::transmute(tracehandle), instancename.into_param().abi(), ::std::mem::transmute(properties)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
#[inline]
pub unsafe fn QueryTraceProcessingHandle(processinghandle: u64, informationclass: ETW_PROCESS_HANDLE_INFO_TYPE, inbuffer: *const ::std::ffi::c_void, inbuffersize: u32, outbuffer: *mut ::std::ffi::c_void, outbuffersize: u32, returnlength: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryTraceProcessingHandle(processinghandle: u64, informationclass: ETW_PROCESS_HANDLE_INFO_TYPE, inbuffer: *const ::std::ffi::c_void, inbuffersize: u32, outbuffer: *mut ::std::ffi::c_void, outbuffersize: u32, returnlength: *mut u32) -> u32;
        }
        ::std::mem::transmute(QueryTraceProcessingHandle(::std::mem::transmute(processinghandle), ::std::mem::transmute(informationclass), ::std::mem::transmute(inbuffer), ::std::mem::transmute(inbuffersize), ::std::mem::transmute(outbuffer), ::std::mem::transmute(outbuffersize), ::std::mem::transmute(returnlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QueryTraceW<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(tracehandle: u64, instancename: Param1, properties: *mut EVENT_TRACE_PROPERTIES) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryTraceW(tracehandle: u64, instancename: super::super::super::Foundation::PWSTR, properties: *mut EVENT_TRACE_PROPERTIES) -> u32;
        }
        ::std::mem::transmute(QueryTraceW(::std::mem::transmute(tracehandle), instancename.into_param().abi(), ::std::mem::transmute(properties)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegisterTraceGuidsA<'a, Param5: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PSTR>, Param6: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PSTR>>(requestaddress: ::std::option::Option<WMIDPREQUEST>, requestcontext: *const ::std::ffi::c_void, controlguid: *const ::windows::runtime::GUID, guidcount: u32, traceguidreg: *const TRACE_GUID_REGISTRATION, mofimagepath: Param5, mofresourcename: Param6, registrationhandle: *mut u64) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegisterTraceGuidsA(requestaddress: ::windows::runtime::RawPtr, requestcontext: *const ::std::ffi::c_void, controlguid: *const ::windows::runtime::GUID, guidcount: u32, traceguidreg: *const TRACE_GUID_REGISTRATION, mofimagepath: super::super::super::Foundation::PSTR, mofresourcename: super::super::super::Foundation::PSTR, registrationhandle: *mut u64) -> u32;
        }
        ::std::mem::transmute(RegisterTraceGuidsA(
            ::std::mem::transmute(requestaddress),
            ::std::mem::transmute(requestcontext),
            ::std::mem::transmute(controlguid),
            ::std::mem::transmute(guidcount),
            ::std::mem::transmute(traceguidreg),
            mofimagepath.into_param().abi(),
            mofresourcename.into_param().abi(),
            ::std::mem::transmute(registrationhandle),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegisterTraceGuidsW<'a, Param5: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param6: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(requestaddress: ::std::option::Option<WMIDPREQUEST>, requestcontext: *const ::std::ffi::c_void, controlguid: *const ::windows::runtime::GUID, guidcount: u32, traceguidreg: *const TRACE_GUID_REGISTRATION, mofimagepath: Param5, mofresourcename: Param6, registrationhandle: *mut u64) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegisterTraceGuidsW(requestaddress: ::windows::runtime::RawPtr, requestcontext: *const ::std::ffi::c_void, controlguid: *const ::windows::runtime::GUID, guidcount: u32, traceguidreg: *const TRACE_GUID_REGISTRATION, mofimagepath: super::super::super::Foundation::PWSTR, mofresourcename: super::super::super::Foundation::PWSTR, registrationhandle: *mut u64) -> u32;
        }
        ::std::mem::transmute(RegisterTraceGuidsW(
            ::std::mem::transmute(requestaddress),
            ::std::mem::transmute(requestcontext),
            ::std::mem::transmute(controlguid),
            ::std::mem::transmute(guidcount),
            ::std::mem::transmute(traceguidreg),
            mofimagepath.into_param().abi(),
            mofresourcename.into_param().abi(),
            ::std::mem::transmute(registrationhandle),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
#[inline]
pub unsafe fn RemoveTraceCallback(pguid: *const ::windows::runtime::GUID) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RemoveTraceCallback(pguid: *const ::windows::runtime::GUID) -> u32;
        }
        ::std::mem::transmute(RemoveTraceCallback(::std::mem::transmute(pguid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_ALPC_KW_GENERAL: u64 = 1u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_CONFIG_KW_GRAPHICS: u64 = 2u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_CONFIG_KW_NETWORK: u64 = 8u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_CONFIG_KW_OPTICAL: u64 = 64u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_CONFIG_KW_PNP: u64 = 32u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_CONFIG_KW_SERVICES: u64 = 16u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_CONFIG_KW_STORAGE: u64 = 4u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_CONFIG_KW_SYSTEM: u64 = 1u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_CPU_KW_CACHE_FLUSH: u64 = 2u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_CPU_KW_CONFIG: u64 = 1u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_CPU_KW_SPEC_CONTROL: u64 = 4u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_EVENT_TYPE: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_HYPERVISOR_KW_CALLOUTS: u64 = 2u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_HYPERVISOR_KW_PROFILE: u64 = 1u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_HYPERVISOR_KW_VTL_CHANGE: u64 = 4u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_INTERRUPT_KW_CLOCK_INTERRUPT: u64 = 2u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_INTERRUPT_KW_DPC: u64 = 4u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_INTERRUPT_KW_DPC_QUEUE: u64 = 8u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_INTERRUPT_KW_GENERAL: u64 = 1u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_INTERRUPT_KW_IPI: u64 = 64u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_INTERRUPT_KW_WDF_DPC: u64 = 16u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_INTERRUPT_KW_WDF_INTERRUPT: u64 = 32u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_IOFILTER_KW_FAILURE: u64 = 8u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_IOFILTER_KW_FASTIO: u64 = 4u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_IOFILTER_KW_GENERAL: u64 = 1u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_IOFILTER_KW_INIT: u64 = 2u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_IO_KW_CC: u64 = 256u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_IO_KW_DISK: u64 = 1u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_IO_KW_DISK_INIT: u64 = 2u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_IO_KW_DRIVERS: u64 = 128u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_IO_KW_FILE: u64 = 16u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_IO_KW_FILENAME: u64 = 4u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_IO_KW_NETWORK: u64 = 512u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_IO_KW_OPTICAL: u64 = 32u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_IO_KW_OPTICAL_INIT: u64 = 64u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_IO_KW_SPLIT: u64 = 8u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_LOCK_KW_SPINLOCK: u64 = 1u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_LOCK_KW_SPINLOCK_COUNTERS: u64 = 2u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_LOCK_KW_SYNC_OBJECTS: u64 = 4u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_MEMORY_KW_ALL_FAULTS: u64 = 4u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_MEMORY_KW_CONTMEM_GEN: u64 = 512u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_MEMORY_KW_FOOTPRINT: u64 = 2048u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_MEMORY_KW_GENERAL: u64 = 1u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_MEMORY_KW_HARD_FAULTS: u64 = 2u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_MEMORY_KW_HEAP: u64 = 128u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_MEMORY_KW_MEMINFO: u64 = 16u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_MEMORY_KW_MEMINFO_WS: u64 = 64u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_MEMORY_KW_NONTRADEABLE: u64 = 32768u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_MEMORY_KW_PFSECTION: u64 = 32u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_MEMORY_KW_POOL: u64 = 8u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_MEMORY_KW_REFSET: u64 = 8192u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_MEMORY_KW_SESSION: u64 = 4096u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_MEMORY_KW_VAMAP: u64 = 16384u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_MEMORY_KW_VIRTUAL_ALLOC: u64 = 1024u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_MEMORY_KW_WS: u64 = 256u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_MEMORY_POOL_FILTER_ID: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_OBJECT_KW_GENERAL: u64 = 1u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_OBJECT_KW_HANDLE: u64 = 2u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_POWER_KW_GENERAL: u64 = 1u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_POWER_KW_HIBER_RUNDOWN: u64 = 2u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_POWER_KW_IDLE_SELECTION: u64 = 8u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_POWER_KW_PPM_EXIT_LATENCY: u64 = 16u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_POWER_KW_PROCESSOR_IDLE: u64 = 4u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_PROCESS_KW_DBGPRINT: u64 = 256u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_PROCESS_KW_DEBUG_EVENTS: u64 = 128u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_PROCESS_KW_FREEZE: u64 = 4u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_PROCESS_KW_GENERAL: u64 = 1u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_PROCESS_KW_INSWAP: u64 = 2u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_PROCESS_KW_JOB: u64 = 512u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_PROCESS_KW_LOADER: u64 = 4096u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_PROCESS_KW_PERF_COUNTER: u64 = 8u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_PROCESS_KW_THREAD: u64 = 2048u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_PROCESS_KW_WAKE_COUNTER: u64 = 16u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_PROCESS_KW_WAKE_DROP: u64 = 32u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_PROCESS_KW_WAKE_EVENT: u64 = 64u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_PROCESS_KW_WORKER_THREAD: u64 = 1024u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_PROFILE_KW_GENERAL: u64 = 1u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_PROFILE_KW_PMC_PROFILE: u64 = 2u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_REGISTRY_KW_GENERAL: u64 = 1u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_REGISTRY_KW_HIVE: u64 = 2u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_REGISTRY_KW_NOTIFICATION: u64 = 4u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_SCHEDULER_KW_AFFINITY: u64 = 64u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_SCHEDULER_KW_ANTI_STARVATION: u64 = 16u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_SCHEDULER_KW_COMPACT_CSWITCH: u64 = 1024u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_SCHEDULER_KW_CONTEXT_SWITCH: u64 = 512u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_SCHEDULER_KW_DISPATCHER: u64 = 2u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_SCHEDULER_KW_IDEAL_PROCESSOR: u64 = 256u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_SCHEDULER_KW_KERNEL_QUEUE: u64 = 4u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_SCHEDULER_KW_LOAD_BALANCER: u64 = 32u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_SCHEDULER_KW_PRIORITY: u64 = 128u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_SCHEDULER_KW_SHOULD_YIELD: u64 = 8u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_SCHEDULER_KW_XSCHEDULER: u64 = 1u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_SYSCALL_KW_GENERAL: u64 = 1u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_TIMER_KW_CLOCK_TIMER: u64 = 2u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const SYSTEM_TIMER_KW_GENERAL: u64 = 1u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
#[inline]
pub unsafe fn SetTraceCallback(pguid: *const ::windows::runtime::GUID, eventcallback: ::std::option::Option<PEVENT_CALLBACK>) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetTraceCallback(pguid: *const ::windows::runtime::GUID, eventcallback: ::windows::runtime::RawPtr) -> u32;
        }
        ::std::mem::transmute(SetTraceCallback(::std::mem::transmute(pguid), ::std::mem::transmute(eventcallback)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn StartTraceA<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PSTR>>(tracehandle: *mut u64, instancename: Param1, properties: *mut EVENT_TRACE_PROPERTIES) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StartTraceA(tracehandle: *mut u64, instancename: super::super::super::Foundation::PSTR, properties: *mut EVENT_TRACE_PROPERTIES) -> u32;
        }
        ::std::mem::transmute(StartTraceA(::std::mem::transmute(tracehandle), instancename.into_param().abi(), ::std::mem::transmute(properties)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn StartTraceW<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(tracehandle: *mut u64, instancename: Param1, properties: *mut EVENT_TRACE_PROPERTIES) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StartTraceW(tracehandle: *mut u64, instancename: super::super::super::Foundation::PWSTR, properties: *mut EVENT_TRACE_PROPERTIES) -> u32;
        }
        ::std::mem::transmute(StartTraceW(::std::mem::transmute(tracehandle), instancename.into_param().abi(), ::std::mem::transmute(properties)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn StopTraceA<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PSTR>>(tracehandle: u64, instancename: Param1, properties: *mut EVENT_TRACE_PROPERTIES) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StopTraceA(tracehandle: u64, instancename: super::super::super::Foundation::PSTR, properties: *mut EVENT_TRACE_PROPERTIES) -> u32;
        }
        ::std::mem::transmute(StopTraceA(::std::mem::transmute(tracehandle), instancename.into_param().abi(), ::std::mem::transmute(properties)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn StopTraceW<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(tracehandle: u64, instancename: Param1, properties: *mut EVENT_TRACE_PROPERTIES) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StopTraceW(tracehandle: u64, instancename: super::super::super::Foundation::PWSTR, properties: *mut EVENT_TRACE_PROPERTIES) -> u32;
        }
        ::std::mem::transmute(StopTraceW(::std::mem::transmute(tracehandle), instancename.into_param().abi(), ::std::mem::transmute(properties)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const SystemAlpcProviderGuid: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4240030383, 58665, 18816, [146, 233, 206, 209, 166, 170, 223, 223]);
pub const SystemConfigProviderGuid: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4277381302, 12685, 19303, [169, 106, 59, 15, 107, 143, 24, 254]);
pub const SystemCpuProviderGuid: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3334809183, 60136, 18000, [170, 228, 157, 72, 96, 61, 133, 16]);
pub const SystemHypervisorProviderGuid: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3136948010, 37258, 19437, [182, 34, 188, 21, 32, 151, 9, 143]);
pub const SystemInterruptProviderGuid: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3569085975, 46405, 18568, [133, 139, 116, 65, 105, 1, 91, 37]);
pub const SystemIoFilterProviderGuid: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4224750435, 40482, 18017, [184, 191, 231, 163, 75, 83, 91, 140]);
pub const SystemIoProviderGuid: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1029456867, 3868, 16898, [184, 23, 23, 76, 0, 112, 220, 121]);
pub const SystemLockProviderGuid: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1914560467, 56012, 19998, [178, 106, 162, 203, 49, 212, 112, 90]);
pub const SystemMemoryProviderGuid: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2190838953, 46797, 18424, [163, 168, 3, 174, 133, 164, 188, 36]);
pub const SystemObjectProviderGuid: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4273828960, 15645, 18411, [175, 73, 201, 238, 177, 225, 70, 242]);
pub const SystemPowerProviderGuid: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3241445450, 13013, 17544, [128, 229, 20, 237, 122, 187, 130, 105]);
pub const SystemProcessProviderGuid: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(354375132, 18045, 18207, [131, 181, 95, 136, 157, 70, 255, 102]);
pub const SystemProfileProviderGuid: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3219850020, 7406, 18799, [164, 9, 42, 194, 180, 138, 99, 34]);
pub const SystemRegistryProviderGuid: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(370502617, 64180, 19706, [162, 50, 137, 209, 9, 144, 88, 227]);
pub const SystemSchedulerProviderGuid: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1503275638, 19857, 18704, [154, 199, 125, 51, 242, 233, 122, 108]);
pub const SystemSyscallProviderGuid: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1128433399, 28443, 17851, [179, 126, 149, 246, 35, 4, 108, 124]);
pub const SystemTimerProviderGuid: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1325798760, 57877, 18847, [171, 46, 237, 160, 174, 137, 10, 91]);
pub const SystemTraceControlGuid: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2659273389, 12804, 4562, [154, 130, 0, 96, 8, 168, 105, 57]);
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub struct TDH_CONTEXT {
    pub ParameterValue: u64,
    pub ParameterType: TDH_CONTEXT_TYPE,
    pub ParameterSize: u32,
}
impl TDH_CONTEXT {}
impl ::std::default::Default for TDH_CONTEXT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for TDH_CONTEXT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TDH_CONTEXT").field("ParameterValue", &self.ParameterValue).field("ParameterType", &self.ParameterType).field("ParameterSize", &self.ParameterSize).finish()
    }
}
impl ::std::cmp::PartialEq for TDH_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.ParameterValue == other.ParameterValue && self.ParameterType == other.ParameterType && self.ParameterSize == other.ParameterSize
    }
}
impl ::std::cmp::Eq for TDH_CONTEXT {}
unsafe impl ::windows::runtime::Abi for TDH_CONTEXT {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct TDH_CONTEXT_TYPE(pub i32);
pub const TDH_CONTEXT_WPP_TMFFILE: TDH_CONTEXT_TYPE = TDH_CONTEXT_TYPE(0i32);
pub const TDH_CONTEXT_WPP_TMFSEARCHPATH: TDH_CONTEXT_TYPE = TDH_CONTEXT_TYPE(1i32);
pub const TDH_CONTEXT_WPP_GMT: TDH_CONTEXT_TYPE = TDH_CONTEXT_TYPE(2i32);
pub const TDH_CONTEXT_POINTERSIZE: TDH_CONTEXT_TYPE = TDH_CONTEXT_TYPE(3i32);
pub const TDH_CONTEXT_PDB_PATH: TDH_CONTEXT_TYPE = TDH_CONTEXT_TYPE(4i32);
pub const TDH_CONTEXT_MAXIMUM: TDH_CONTEXT_TYPE = TDH_CONTEXT_TYPE(5i32);
impl ::std::convert::From<i32> for TDH_CONTEXT_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TDH_CONTEXT_TYPE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq)]
#[repr(transparent)]
pub struct TDH_HANDLE(pub isize);
impl ::std::default::Default for TDH_HANDLE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for TDH_HANDLE {}
unsafe impl ::windows::runtime::Abi for TDH_HANDLE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct TEMPLATE_FLAGS(pub i32);
pub const TEMPLATE_EVENT_DATA: TEMPLATE_FLAGS = TEMPLATE_FLAGS(1i32);
pub const TEMPLATE_USER_DATA: TEMPLATE_FLAGS = TEMPLATE_FLAGS(2i32);
pub const TEMPLATE_CONTROL_GUID: TEMPLATE_FLAGS = TEMPLATE_FLAGS(4i32);
impl ::std::convert::From<i32> for TEMPLATE_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TEMPLATE_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const TRACELOG_ACCESS_KERNEL_LOGGER: u32 = 256u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const TRACELOG_ACCESS_REALTIME: u32 = 1024u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const TRACELOG_CREATE_INPROC: u32 = 512u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const TRACELOG_CREATE_ONDISK: u32 = 64u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const TRACELOG_CREATE_REALTIME: u32 = 32u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const TRACELOG_GUID_ENABLE: u32 = 128u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const TRACELOG_JOIN_GROUP: u32 = 4096u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const TRACELOG_LOG_EVENT: u32 = 512u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const TRACELOG_REGISTER_GUIDS: u32 = 2048u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub struct TRACE_ENABLE_INFO {
    pub IsEnabled: u32,
    pub Level: u8,
    pub Reserved1: u8,
    pub LoggerId: u16,
    pub EnableProperty: u32,
    pub Reserved2: u32,
    pub MatchAnyKeyword: u64,
    pub MatchAllKeyword: u64,
}
impl TRACE_ENABLE_INFO {}
impl ::std::default::Default for TRACE_ENABLE_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for TRACE_ENABLE_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TRACE_ENABLE_INFO")
            .field("IsEnabled", &self.IsEnabled)
            .field("Level", &self.Level)
            .field("Reserved1", &self.Reserved1)
            .field("LoggerId", &self.LoggerId)
            .field("EnableProperty", &self.EnableProperty)
            .field("Reserved2", &self.Reserved2)
            .field("MatchAnyKeyword", &self.MatchAnyKeyword)
            .field("MatchAllKeyword", &self.MatchAllKeyword)
            .finish()
    }
}
impl ::std::cmp::PartialEq for TRACE_ENABLE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.IsEnabled == other.IsEnabled && self.Level == other.Level && self.Reserved1 == other.Reserved1 && self.LoggerId == other.LoggerId && self.EnableProperty == other.EnableProperty && self.Reserved2 == other.Reserved2 && self.MatchAnyKeyword == other.MatchAnyKeyword && self.MatchAllKeyword == other.MatchAllKeyword
    }
}
impl ::std::cmp::Eq for TRACE_ENABLE_INFO {}
unsafe impl ::windows::runtime::Abi for TRACE_ENABLE_INFO {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub struct TRACE_EVENT_INFO {
    pub ProviderGuid: ::windows::runtime::GUID,
    pub EventGuid: ::windows::runtime::GUID,
    pub EventDescriptor: EVENT_DESCRIPTOR,
    pub DecodingSource: DECODING_SOURCE,
    pub ProviderNameOffset: u32,
    pub LevelNameOffset: u32,
    pub ChannelNameOffset: u32,
    pub KeywordsNameOffset: u32,
    pub TaskNameOffset: u32,
    pub OpcodeNameOffset: u32,
    pub EventMessageOffset: u32,
    pub ProviderMessageOffset: u32,
    pub BinaryXMLOffset: u32,
    pub BinaryXMLSize: u32,
    pub Anonymous1: TRACE_EVENT_INFO_0,
    pub Anonymous2: TRACE_EVENT_INFO_1,
    pub PropertyCount: u32,
    pub TopLevelPropertyCount: u32,
    pub Anonymous3: TRACE_EVENT_INFO_2,
    pub EventPropertyInfoArray: [EVENT_PROPERTY_INFO; 1],
}
impl TRACE_EVENT_INFO {}
impl ::std::default::Default for TRACE_EVENT_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for TRACE_EVENT_INFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for TRACE_EVENT_INFO {}
unsafe impl ::windows::runtime::Abi for TRACE_EVENT_INFO {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub union TRACE_EVENT_INFO_0 {
    pub EventNameOffset: u32,
    pub ActivityIDNameOffset: u32,
}
impl TRACE_EVENT_INFO_0 {}
impl ::std::default::Default for TRACE_EVENT_INFO_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for TRACE_EVENT_INFO_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for TRACE_EVENT_INFO_0 {}
unsafe impl ::windows::runtime::Abi for TRACE_EVENT_INFO_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub union TRACE_EVENT_INFO_1 {
    pub EventAttributesOffset: u32,
    pub RelatedActivityIDNameOffset: u32,
}
impl TRACE_EVENT_INFO_1 {}
impl ::std::default::Default for TRACE_EVENT_INFO_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for TRACE_EVENT_INFO_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for TRACE_EVENT_INFO_1 {}
unsafe impl ::windows::runtime::Abi for TRACE_EVENT_INFO_1 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub union TRACE_EVENT_INFO_2 {
    pub Flags: TEMPLATE_FLAGS,
    pub Anonymous: TRACE_EVENT_INFO_2_0,
}
impl TRACE_EVENT_INFO_2 {}
impl ::std::default::Default for TRACE_EVENT_INFO_2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for TRACE_EVENT_INFO_2 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for TRACE_EVENT_INFO_2 {}
unsafe impl ::windows::runtime::Abi for TRACE_EVENT_INFO_2 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub struct TRACE_EVENT_INFO_2_0 {
    pub _bitfield: u32,
}
impl TRACE_EVENT_INFO_2_0 {}
impl ::std::default::Default for TRACE_EVENT_INFO_2_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for TRACE_EVENT_INFO_2_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::std::cmp::PartialEq for TRACE_EVENT_INFO_2_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for TRACE_EVENT_INFO_2_0 {}
unsafe impl ::windows::runtime::Abi for TRACE_EVENT_INFO_2_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub struct TRACE_GUID_INFO {
    pub InstanceCount: u32,
    pub Reserved: u32,
}
impl TRACE_GUID_INFO {}
impl ::std::default::Default for TRACE_GUID_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for TRACE_GUID_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TRACE_GUID_INFO").field("InstanceCount", &self.InstanceCount).field("Reserved", &self.Reserved).finish()
    }
}
impl ::std::cmp::PartialEq for TRACE_GUID_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.InstanceCount == other.InstanceCount && self.Reserved == other.Reserved
    }
}
impl ::std::cmp::Eq for TRACE_GUID_INFO {}
unsafe impl ::windows::runtime::Abi for TRACE_GUID_INFO {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
pub struct TRACE_GUID_PROPERTIES {
    pub Guid: ::windows::runtime::GUID,
    pub GuidType: u32,
    pub LoggerId: u32,
    pub EnableLevel: u32,
    pub EnableFlags: u32,
    pub IsEnable: super::super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl TRACE_GUID_PROPERTIES {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for TRACE_GUID_PROPERTIES {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for TRACE_GUID_PROPERTIES {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TRACE_GUID_PROPERTIES").field("Guid", &self.Guid).field("GuidType", &self.GuidType).field("LoggerId", &self.LoggerId).field("EnableLevel", &self.EnableLevel).field("EnableFlags", &self.EnableFlags).field("IsEnable", &self.IsEnable).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for TRACE_GUID_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        self.Guid == other.Guid && self.GuidType == other.GuidType && self.LoggerId == other.LoggerId && self.EnableLevel == other.EnableLevel && self.EnableFlags == other.EnableFlags && self.IsEnable == other.IsEnable
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for TRACE_GUID_PROPERTIES {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for TRACE_GUID_PROPERTIES {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
pub struct TRACE_GUID_REGISTRATION {
    pub Guid: *mut ::windows::runtime::GUID,
    pub RegHandle: super::super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl TRACE_GUID_REGISTRATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for TRACE_GUID_REGISTRATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for TRACE_GUID_REGISTRATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TRACE_GUID_REGISTRATION").field("Guid", &self.Guid).field("RegHandle", &self.RegHandle).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for TRACE_GUID_REGISTRATION {
    fn eq(&self, other: &Self) -> bool {
        self.Guid == other.Guid && self.RegHandle == other.RegHandle
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for TRACE_GUID_REGISTRATION {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for TRACE_GUID_REGISTRATION {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const TRACE_HEADER_FLAG_LOG_WNODE: u32 = 262144u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const TRACE_HEADER_FLAG_TRACED_GUID: u32 = 131072u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const TRACE_HEADER_FLAG_USE_GUID_PTR: u32 = 524288u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const TRACE_HEADER_FLAG_USE_MOF_PTR: u32 = 1048576u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const TRACE_HEADER_FLAG_USE_TIMESTAMP: u32 = 512u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const TRACE_LEVEL_CRITICAL: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const TRACE_LEVEL_ERROR: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const TRACE_LEVEL_FATAL: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const TRACE_LEVEL_INFORMATION: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const TRACE_LEVEL_NONE: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const TRACE_LEVEL_RESERVED6: u32 = 6u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const TRACE_LEVEL_RESERVED7: u32 = 7u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const TRACE_LEVEL_RESERVED8: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const TRACE_LEVEL_RESERVED9: u32 = 9u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const TRACE_LEVEL_VERBOSE: u32 = 5u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const TRACE_LEVEL_WARNING: u32 = 3u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`, `Win32_System_Time`*"]
pub struct TRACE_LOGFILE_HEADER {
    pub BufferSize: u32,
    pub Anonymous1: TRACE_LOGFILE_HEADER_0,
    pub ProviderVersion: u32,
    pub NumberOfProcessors: u32,
    pub EndTime: i64,
    pub TimerResolution: u32,
    pub MaximumFileSize: u32,
    pub LogFileMode: u32,
    pub BuffersWritten: u32,
    pub Anonymous2: TRACE_LOGFILE_HEADER_1,
    pub LoggerName: super::super::super::Foundation::PWSTR,
    pub LogFileName: super::super::super::Foundation::PWSTR,
    pub TimeZone: super::super::Time::TIME_ZONE_INFORMATION,
    pub BootTime: i64,
    pub PerfFreq: i64,
    pub StartTime: i64,
    pub ReservedFlags: u32,
    pub BuffersLost: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl TRACE_LOGFILE_HEADER {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::std::default::Default for TRACE_LOGFILE_HEADER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::std::cmp::PartialEq for TRACE_LOGFILE_HEADER {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::std::cmp::Eq for TRACE_LOGFILE_HEADER {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
unsafe impl ::windows::runtime::Abi for TRACE_LOGFILE_HEADER {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub union TRACE_LOGFILE_HEADER_0 {
    pub Version: u32,
    pub VersionDetail: TRACE_LOGFILE_HEADER_0_0,
}
impl TRACE_LOGFILE_HEADER_0 {}
impl ::std::default::Default for TRACE_LOGFILE_HEADER_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for TRACE_LOGFILE_HEADER_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for TRACE_LOGFILE_HEADER_0 {}
unsafe impl ::windows::runtime::Abi for TRACE_LOGFILE_HEADER_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub struct TRACE_LOGFILE_HEADER_0_0 {
    pub MajorVersion: u8,
    pub MinorVersion: u8,
    pub SubVersion: u8,
    pub SubMinorVersion: u8,
}
impl TRACE_LOGFILE_HEADER_0_0 {}
impl ::std::default::Default for TRACE_LOGFILE_HEADER_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for TRACE_LOGFILE_HEADER_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_VersionDetail_e__Struct").field("MajorVersion", &self.MajorVersion).field("MinorVersion", &self.MinorVersion).field("SubVersion", &self.SubVersion).field("SubMinorVersion", &self.SubMinorVersion).finish()
    }
}
impl ::std::cmp::PartialEq for TRACE_LOGFILE_HEADER_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.MajorVersion == other.MajorVersion && self.MinorVersion == other.MinorVersion && self.SubVersion == other.SubVersion && self.SubMinorVersion == other.SubMinorVersion
    }
}
impl ::std::cmp::Eq for TRACE_LOGFILE_HEADER_0_0 {}
unsafe impl ::windows::runtime::Abi for TRACE_LOGFILE_HEADER_0_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub union TRACE_LOGFILE_HEADER_1 {
    pub LogInstanceGuid: ::windows::runtime::GUID,
    pub Anonymous: TRACE_LOGFILE_HEADER_1_0,
}
impl TRACE_LOGFILE_HEADER_1 {}
impl ::std::default::Default for TRACE_LOGFILE_HEADER_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for TRACE_LOGFILE_HEADER_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for TRACE_LOGFILE_HEADER_1 {}
unsafe impl ::windows::runtime::Abi for TRACE_LOGFILE_HEADER_1 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub struct TRACE_LOGFILE_HEADER_1_0 {
    pub StartBuffers: u32,
    pub PointerSize: u32,
    pub EventsLost: u32,
    pub CpuSpeedInMHz: u32,
}
impl TRACE_LOGFILE_HEADER_1_0 {}
impl ::std::default::Default for TRACE_LOGFILE_HEADER_1_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for TRACE_LOGFILE_HEADER_1_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("StartBuffers", &self.StartBuffers).field("PointerSize", &self.PointerSize).field("EventsLost", &self.EventsLost).field("CpuSpeedInMHz", &self.CpuSpeedInMHz).finish()
    }
}
impl ::std::cmp::PartialEq for TRACE_LOGFILE_HEADER_1_0 {
    fn eq(&self, other: &Self) -> bool {
        self.StartBuffers == other.StartBuffers && self.PointerSize == other.PointerSize && self.EventsLost == other.EventsLost && self.CpuSpeedInMHz == other.CpuSpeedInMHz
    }
}
impl ::std::cmp::Eq for TRACE_LOGFILE_HEADER_1_0 {}
unsafe impl ::windows::runtime::Abi for TRACE_LOGFILE_HEADER_1_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`, `Win32_System_Time`*"]
pub struct TRACE_LOGFILE_HEADER32 {
    pub BufferSize: u32,
    pub Anonymous1: TRACE_LOGFILE_HEADER32_0,
    pub ProviderVersion: u32,
    pub NumberOfProcessors: u32,
    pub EndTime: i64,
    pub TimerResolution: u32,
    pub MaximumFileSize: u32,
    pub LogFileMode: u32,
    pub BuffersWritten: u32,
    pub Anonymous2: TRACE_LOGFILE_HEADER32_1,
    pub LoggerName: u32,
    pub LogFileName: u32,
    pub TimeZone: super::super::Time::TIME_ZONE_INFORMATION,
    pub BootTime: i64,
    pub PerfFreq: i64,
    pub StartTime: i64,
    pub ReservedFlags: u32,
    pub BuffersLost: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl TRACE_LOGFILE_HEADER32 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::std::default::Default for TRACE_LOGFILE_HEADER32 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::std::cmp::PartialEq for TRACE_LOGFILE_HEADER32 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::std::cmp::Eq for TRACE_LOGFILE_HEADER32 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
unsafe impl ::windows::runtime::Abi for TRACE_LOGFILE_HEADER32 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub union TRACE_LOGFILE_HEADER32_0 {
    pub Version: u32,
    pub VersionDetail: TRACE_LOGFILE_HEADER32_0_0,
}
impl TRACE_LOGFILE_HEADER32_0 {}
impl ::std::default::Default for TRACE_LOGFILE_HEADER32_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for TRACE_LOGFILE_HEADER32_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for TRACE_LOGFILE_HEADER32_0 {}
unsafe impl ::windows::runtime::Abi for TRACE_LOGFILE_HEADER32_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub struct TRACE_LOGFILE_HEADER32_0_0 {
    pub MajorVersion: u8,
    pub MinorVersion: u8,
    pub SubVersion: u8,
    pub SubMinorVersion: u8,
}
impl TRACE_LOGFILE_HEADER32_0_0 {}
impl ::std::default::Default for TRACE_LOGFILE_HEADER32_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for TRACE_LOGFILE_HEADER32_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_VersionDetail_e__Struct").field("MajorVersion", &self.MajorVersion).field("MinorVersion", &self.MinorVersion).field("SubVersion", &self.SubVersion).field("SubMinorVersion", &self.SubMinorVersion).finish()
    }
}
impl ::std::cmp::PartialEq for TRACE_LOGFILE_HEADER32_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.MajorVersion == other.MajorVersion && self.MinorVersion == other.MinorVersion && self.SubVersion == other.SubVersion && self.SubMinorVersion == other.SubMinorVersion
    }
}
impl ::std::cmp::Eq for TRACE_LOGFILE_HEADER32_0_0 {}
unsafe impl ::windows::runtime::Abi for TRACE_LOGFILE_HEADER32_0_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub union TRACE_LOGFILE_HEADER32_1 {
    pub LogInstanceGuid: ::windows::runtime::GUID,
    pub Anonymous: TRACE_LOGFILE_HEADER32_1_0,
}
impl TRACE_LOGFILE_HEADER32_1 {}
impl ::std::default::Default for TRACE_LOGFILE_HEADER32_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for TRACE_LOGFILE_HEADER32_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for TRACE_LOGFILE_HEADER32_1 {}
unsafe impl ::windows::runtime::Abi for TRACE_LOGFILE_HEADER32_1 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub struct TRACE_LOGFILE_HEADER32_1_0 {
    pub StartBuffers: u32,
    pub PointerSize: u32,
    pub EventsLost: u32,
    pub CpuSpeedInMHz: u32,
}
impl TRACE_LOGFILE_HEADER32_1_0 {}
impl ::std::default::Default for TRACE_LOGFILE_HEADER32_1_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for TRACE_LOGFILE_HEADER32_1_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("StartBuffers", &self.StartBuffers).field("PointerSize", &self.PointerSize).field("EventsLost", &self.EventsLost).field("CpuSpeedInMHz", &self.CpuSpeedInMHz).finish()
    }
}
impl ::std::cmp::PartialEq for TRACE_LOGFILE_HEADER32_1_0 {
    fn eq(&self, other: &Self) -> bool {
        self.StartBuffers == other.StartBuffers && self.PointerSize == other.PointerSize && self.EventsLost == other.EventsLost && self.CpuSpeedInMHz == other.CpuSpeedInMHz
    }
}
impl ::std::cmp::Eq for TRACE_LOGFILE_HEADER32_1_0 {}
unsafe impl ::windows::runtime::Abi for TRACE_LOGFILE_HEADER32_1_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`, `Win32_System_Time`*"]
pub struct TRACE_LOGFILE_HEADER64 {
    pub BufferSize: u32,
    pub Anonymous1: TRACE_LOGFILE_HEADER64_0,
    pub ProviderVersion: u32,
    pub NumberOfProcessors: u32,
    pub EndTime: i64,
    pub TimerResolution: u32,
    pub MaximumFileSize: u32,
    pub LogFileMode: u32,
    pub BuffersWritten: u32,
    pub Anonymous2: TRACE_LOGFILE_HEADER64_1,
    pub LoggerName: u64,
    pub LogFileName: u64,
    pub TimeZone: super::super::Time::TIME_ZONE_INFORMATION,
    pub BootTime: i64,
    pub PerfFreq: i64,
    pub StartTime: i64,
    pub ReservedFlags: u32,
    pub BuffersLost: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl TRACE_LOGFILE_HEADER64 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::std::default::Default for TRACE_LOGFILE_HEADER64 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::std::cmp::PartialEq for TRACE_LOGFILE_HEADER64 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::std::cmp::Eq for TRACE_LOGFILE_HEADER64 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
unsafe impl ::windows::runtime::Abi for TRACE_LOGFILE_HEADER64 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub union TRACE_LOGFILE_HEADER64_0 {
    pub Version: u32,
    pub VersionDetail: TRACE_LOGFILE_HEADER64_0_0,
}
impl TRACE_LOGFILE_HEADER64_0 {}
impl ::std::default::Default for TRACE_LOGFILE_HEADER64_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for TRACE_LOGFILE_HEADER64_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for TRACE_LOGFILE_HEADER64_0 {}
unsafe impl ::windows::runtime::Abi for TRACE_LOGFILE_HEADER64_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub struct TRACE_LOGFILE_HEADER64_0_0 {
    pub MajorVersion: u8,
    pub MinorVersion: u8,
    pub SubVersion: u8,
    pub SubMinorVersion: u8,
}
impl TRACE_LOGFILE_HEADER64_0_0 {}
impl ::std::default::Default for TRACE_LOGFILE_HEADER64_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for TRACE_LOGFILE_HEADER64_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_VersionDetail_e__Struct").field("MajorVersion", &self.MajorVersion).field("MinorVersion", &self.MinorVersion).field("SubVersion", &self.SubVersion).field("SubMinorVersion", &self.SubMinorVersion).finish()
    }
}
impl ::std::cmp::PartialEq for TRACE_LOGFILE_HEADER64_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.MajorVersion == other.MajorVersion && self.MinorVersion == other.MinorVersion && self.SubVersion == other.SubVersion && self.SubMinorVersion == other.SubMinorVersion
    }
}
impl ::std::cmp::Eq for TRACE_LOGFILE_HEADER64_0_0 {}
unsafe impl ::windows::runtime::Abi for TRACE_LOGFILE_HEADER64_0_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub union TRACE_LOGFILE_HEADER64_1 {
    pub LogInstanceGuid: ::windows::runtime::GUID,
    pub Anonymous: TRACE_LOGFILE_HEADER64_1_0,
}
impl TRACE_LOGFILE_HEADER64_1 {}
impl ::std::default::Default for TRACE_LOGFILE_HEADER64_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for TRACE_LOGFILE_HEADER64_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for TRACE_LOGFILE_HEADER64_1 {}
unsafe impl ::windows::runtime::Abi for TRACE_LOGFILE_HEADER64_1 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub struct TRACE_LOGFILE_HEADER64_1_0 {
    pub StartBuffers: u32,
    pub PointerSize: u32,
    pub EventsLost: u32,
    pub CpuSpeedInMHz: u32,
}
impl TRACE_LOGFILE_HEADER64_1_0 {}
impl ::std::default::Default for TRACE_LOGFILE_HEADER64_1_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for TRACE_LOGFILE_HEADER64_1_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("StartBuffers", &self.StartBuffers).field("PointerSize", &self.PointerSize).field("EventsLost", &self.EventsLost).field("CpuSpeedInMHz", &self.CpuSpeedInMHz).finish()
    }
}
impl ::std::cmp::PartialEq for TRACE_LOGFILE_HEADER64_1_0 {
    fn eq(&self, other: &Self) -> bool {
        self.StartBuffers == other.StartBuffers && self.PointerSize == other.PointerSize && self.EventsLost == other.EventsLost && self.CpuSpeedInMHz == other.CpuSpeedInMHz
    }
}
impl ::std::cmp::Eq for TRACE_LOGFILE_HEADER64_1_0 {}
unsafe impl ::windows::runtime::Abi for TRACE_LOGFILE_HEADER64_1_0 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct TRACE_MESSAGE_FLAGS(pub u32);
pub const TRACE_MESSAGE_COMPONENTID: TRACE_MESSAGE_FLAGS = TRACE_MESSAGE_FLAGS(4u32);
pub const TRACE_MESSAGE_GUID: TRACE_MESSAGE_FLAGS = TRACE_MESSAGE_FLAGS(2u32);
pub const TRACE_MESSAGE_SEQUENCE: TRACE_MESSAGE_FLAGS = TRACE_MESSAGE_FLAGS(1u32);
pub const TRACE_MESSAGE_SYSTEMINFO: TRACE_MESSAGE_FLAGS = TRACE_MESSAGE_FLAGS(32u32);
pub const TRACE_MESSAGE_TIMESTAMP: TRACE_MESSAGE_FLAGS = TRACE_MESSAGE_FLAGS(8u32);
impl ::std::convert::From<u32> for TRACE_MESSAGE_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TRACE_MESSAGE_FLAGS {
    type Abi = Self;
}
impl ::std::ops::BitOr for TRACE_MESSAGE_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for TRACE_MESSAGE_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for TRACE_MESSAGE_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for TRACE_MESSAGE_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for TRACE_MESSAGE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const TRACE_MESSAGE_FLAG_MASK: u32 = 65535u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const TRACE_MESSAGE_PERFORMANCE_TIMESTAMP: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const TRACE_MESSAGE_POINTER32: u32 = 64u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const TRACE_MESSAGE_POINTER64: u32 = 128u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub struct TRACE_PERIODIC_CAPTURE_STATE_INFO {
    pub CaptureStateFrequencyInSeconds: u32,
    pub ProviderCount: u16,
    pub Reserved: u16,
}
impl TRACE_PERIODIC_CAPTURE_STATE_INFO {}
impl ::std::default::Default for TRACE_PERIODIC_CAPTURE_STATE_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for TRACE_PERIODIC_CAPTURE_STATE_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TRACE_PERIODIC_CAPTURE_STATE_INFO").field("CaptureStateFrequencyInSeconds", &self.CaptureStateFrequencyInSeconds).field("ProviderCount", &self.ProviderCount).field("Reserved", &self.Reserved).finish()
    }
}
impl ::std::cmp::PartialEq for TRACE_PERIODIC_CAPTURE_STATE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.CaptureStateFrequencyInSeconds == other.CaptureStateFrequencyInSeconds && self.ProviderCount == other.ProviderCount && self.Reserved == other.Reserved
    }
}
impl ::std::cmp::Eq for TRACE_PERIODIC_CAPTURE_STATE_INFO {}
unsafe impl ::windows::runtime::Abi for TRACE_PERIODIC_CAPTURE_STATE_INFO {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub struct TRACE_PROFILE_INTERVAL {
    pub Source: u32,
    pub Interval: u32,
}
impl TRACE_PROFILE_INTERVAL {}
impl ::std::default::Default for TRACE_PROFILE_INTERVAL {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for TRACE_PROFILE_INTERVAL {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TRACE_PROFILE_INTERVAL").field("Source", &self.Source).field("Interval", &self.Interval).finish()
    }
}
impl ::std::cmp::PartialEq for TRACE_PROFILE_INTERVAL {
    fn eq(&self, other: &Self) -> bool {
        self.Source == other.Source && self.Interval == other.Interval
    }
}
impl ::std::cmp::Eq for TRACE_PROFILE_INTERVAL {}
unsafe impl ::windows::runtime::Abi for TRACE_PROFILE_INTERVAL {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const TRACE_PROVIDER_FLAG_LEGACY: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const TRACE_PROVIDER_FLAG_PRE_ENABLE: u32 = 2u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub struct TRACE_PROVIDER_INFO {
    pub ProviderGuid: ::windows::runtime::GUID,
    pub SchemaSource: u32,
    pub ProviderNameOffset: u32,
}
impl TRACE_PROVIDER_INFO {}
impl ::std::default::Default for TRACE_PROVIDER_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for TRACE_PROVIDER_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TRACE_PROVIDER_INFO").field("ProviderGuid", &self.ProviderGuid).field("SchemaSource", &self.SchemaSource).field("ProviderNameOffset", &self.ProviderNameOffset).finish()
    }
}
impl ::std::cmp::PartialEq for TRACE_PROVIDER_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.ProviderGuid == other.ProviderGuid && self.SchemaSource == other.SchemaSource && self.ProviderNameOffset == other.ProviderNameOffset
    }
}
impl ::std::cmp::Eq for TRACE_PROVIDER_INFO {}
unsafe impl ::windows::runtime::Abi for TRACE_PROVIDER_INFO {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub struct TRACE_PROVIDER_INSTANCE_INFO {
    pub NextOffset: u32,
    pub EnableCount: u32,
    pub Pid: u32,
    pub Flags: u32,
}
impl TRACE_PROVIDER_INSTANCE_INFO {}
impl ::std::default::Default for TRACE_PROVIDER_INSTANCE_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for TRACE_PROVIDER_INSTANCE_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TRACE_PROVIDER_INSTANCE_INFO").field("NextOffset", &self.NextOffset).field("EnableCount", &self.EnableCount).field("Pid", &self.Pid).field("Flags", &self.Flags).finish()
    }
}
impl ::std::cmp::PartialEq for TRACE_PROVIDER_INSTANCE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.NextOffset == other.NextOffset && self.EnableCount == other.EnableCount && self.Pid == other.Pid && self.Flags == other.Flags
    }
}
impl ::std::cmp::Eq for TRACE_PROVIDER_INSTANCE_INFO {}
unsafe impl ::windows::runtime::Abi for TRACE_PROVIDER_INSTANCE_INFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct TRACE_QUERY_INFO_CLASS(pub i32);
pub const TraceGuidQueryList: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(0i32);
pub const TraceGuidQueryInfo: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(1i32);
pub const TraceGuidQueryProcess: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(2i32);
pub const TraceStackTracingInfo: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(3i32);
pub const TraceSystemTraceEnableFlagsInfo: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(4i32);
pub const TraceSampledProfileIntervalInfo: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(5i32);
pub const TraceProfileSourceConfigInfo: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(6i32);
pub const TraceProfileSourceListInfo: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(7i32);
pub const TracePmcEventListInfo: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(8i32);
pub const TracePmcCounterListInfo: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(9i32);
pub const TraceSetDisallowList: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(10i32);
pub const TraceVersionInfo: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(11i32);
pub const TraceGroupQueryList: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(12i32);
pub const TraceGroupQueryInfo: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(13i32);
pub const TraceDisallowListQuery: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(14i32);
pub const TraceInfoReserved15: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(15i32);
pub const TracePeriodicCaptureStateListInfo: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(16i32);
pub const TracePeriodicCaptureStateInfo: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(17i32);
pub const TraceProviderBinaryTracking: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(18i32);
pub const TraceMaxLoggersQuery: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(19i32);
pub const TraceLbrConfigurationInfo: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(20i32);
pub const TraceLbrEventListInfo: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(21i32);
pub const TraceMaxPmcCounterQuery: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(22i32);
pub const TraceStreamCount: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(23i32);
pub const TraceStackCachingInfo: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(24i32);
pub const TracePmcCounterOwners: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(25i32);
pub const TraceUnifiedStackCachingInfo: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(26i32);
pub const MaxTraceSetInfoClass: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(27i32);
impl ::std::convert::From<i32> for TRACE_QUERY_INFO_CLASS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TRACE_QUERY_INFO_CLASS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
pub struct TRACE_STACK_CACHING_INFO {
    pub Enabled: super::super::super::Foundation::BOOLEAN,
    pub CacheSize: u32,
    pub BucketCount: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl TRACE_STACK_CACHING_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for TRACE_STACK_CACHING_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for TRACE_STACK_CACHING_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TRACE_STACK_CACHING_INFO").field("Enabled", &self.Enabled).field("CacheSize", &self.CacheSize).field("BucketCount", &self.BucketCount).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for TRACE_STACK_CACHING_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Enabled == other.Enabled && self.CacheSize == other.CacheSize && self.BucketCount == other.BucketCount
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for TRACE_STACK_CACHING_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for TRACE_STACK_CACHING_INFO {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub struct TRACE_VERSION_INFO {
    pub EtwTraceProcessingVersion: u32,
    pub Reserved: u32,
}
impl TRACE_VERSION_INFO {}
impl ::std::default::Default for TRACE_VERSION_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for TRACE_VERSION_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TRACE_VERSION_INFO").field("EtwTraceProcessingVersion", &self.EtwTraceProcessingVersion).field("Reserved", &self.Reserved).finish()
    }
}
impl ::std::cmp::PartialEq for TRACE_VERSION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.EtwTraceProcessingVersion == other.EtwTraceProcessingVersion && self.Reserved == other.Reserved
    }
}
impl ::std::cmp::Eq for TRACE_VERSION_INFO {}
unsafe impl ::windows::runtime::Abi for TRACE_VERSION_INFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TdhAggregatePayloadFilters(payloadfiltercount: u32, payloadfilterptrs: *const *const ::std::ffi::c_void, eventmatchallflags: *const super::super::super::Foundation::BOOLEAN, eventfilterdescriptor: *mut EVENT_FILTER_DESCRIPTOR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TdhAggregatePayloadFilters(payloadfiltercount: u32, payloadfilterptrs: *const *const ::std::ffi::c_void, eventmatchallflags: *const super::super::super::Foundation::BOOLEAN, eventfilterdescriptor: *mut EVENT_FILTER_DESCRIPTOR) -> u32;
        }
        ::std::mem::transmute(TdhAggregatePayloadFilters(::std::mem::transmute(payloadfiltercount), ::std::mem::transmute(payloadfilterptrs), ::std::mem::transmute(eventmatchallflags), ::std::mem::transmute(eventfilterdescriptor)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
#[inline]
pub unsafe fn TdhCleanupPayloadEventFilterDescriptor(eventfilterdescriptor: *mut EVENT_FILTER_DESCRIPTOR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TdhCleanupPayloadEventFilterDescriptor(eventfilterdescriptor: *mut EVENT_FILTER_DESCRIPTOR) -> u32;
        }
        ::std::mem::transmute(TdhCleanupPayloadEventFilterDescriptor(::std::mem::transmute(eventfilterdescriptor)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
#[inline]
pub unsafe fn TdhCloseDecodingHandle<'a, Param0: ::windows::runtime::IntoParam<'a, TDH_HANDLE>>(handle: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TdhCloseDecodingHandle(handle: TDH_HANDLE) -> u32;
        }
        ::std::mem::transmute(TdhCloseDecodingHandle(handle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TdhCreatePayloadFilter<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BOOLEAN>>(providerguid: *const ::windows::runtime::GUID, eventdescriptor: *const EVENT_DESCRIPTOR, eventmatchany: Param2, payloadpredicatecount: u32, payloadpredicates: *const PAYLOAD_FILTER_PREDICATE, payloadfilter: *mut *mut ::std::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TdhCreatePayloadFilter(providerguid: *const ::windows::runtime::GUID, eventdescriptor: *const EVENT_DESCRIPTOR, eventmatchany: super::super::super::Foundation::BOOLEAN, payloadpredicatecount: u32, payloadpredicates: *const PAYLOAD_FILTER_PREDICATE, payloadfilter: *mut *mut ::std::ffi::c_void) -> u32;
        }
        ::std::mem::transmute(TdhCreatePayloadFilter(::std::mem::transmute(providerguid), ::std::mem::transmute(eventdescriptor), eventmatchany.into_param().abi(), ::std::mem::transmute(payloadpredicatecount), ::std::mem::transmute(payloadpredicates), ::std::mem::transmute(payloadfilter)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
#[inline]
pub unsafe fn TdhDeletePayloadFilter(payloadfilter: *mut *mut ::std::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TdhDeletePayloadFilter(payloadfilter: *mut *mut ::std::ffi::c_void) -> u32;
        }
        ::std::mem::transmute(TdhDeletePayloadFilter(::std::mem::transmute(payloadfilter)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
#[inline]
pub unsafe fn TdhEnumerateManifestProviderEvents(providerguid: *const ::windows::runtime::GUID, buffer: *mut PROVIDER_EVENT_INFO, buffersize: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TdhEnumerateManifestProviderEvents(providerguid: *const ::windows::runtime::GUID, buffer: *mut PROVIDER_EVENT_INFO, buffersize: *mut u32) -> u32;
        }
        ::std::mem::transmute(TdhEnumerateManifestProviderEvents(::std::mem::transmute(providerguid), ::std::mem::transmute(buffer), ::std::mem::transmute(buffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
#[inline]
pub unsafe fn TdhEnumerateProviderFieldInformation(pguid: *const ::windows::runtime::GUID, eventfieldtype: EVENT_FIELD_TYPE, pbuffer: *mut PROVIDER_FIELD_INFOARRAY, pbuffersize: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TdhEnumerateProviderFieldInformation(pguid: *const ::windows::runtime::GUID, eventfieldtype: EVENT_FIELD_TYPE, pbuffer: *mut PROVIDER_FIELD_INFOARRAY, pbuffersize: *mut u32) -> u32;
        }
        ::std::mem::transmute(TdhEnumerateProviderFieldInformation(::std::mem::transmute(pguid), ::std::mem::transmute(eventfieldtype), ::std::mem::transmute(pbuffer), ::std::mem::transmute(pbuffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
#[inline]
pub unsafe fn TdhEnumerateProviderFilters(guid: *const ::windows::runtime::GUID, tdhcontextcount: u32, tdhcontext: *const TDH_CONTEXT, filtercount: *mut u32, buffer: *mut *mut PROVIDER_FILTER_INFO, buffersize: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TdhEnumerateProviderFilters(guid: *const ::windows::runtime::GUID, tdhcontextcount: u32, tdhcontext: *const TDH_CONTEXT, filtercount: *mut u32, buffer: *mut *mut PROVIDER_FILTER_INFO, buffersize: *mut u32) -> u32;
        }
        ::std::mem::transmute(TdhEnumerateProviderFilters(::std::mem::transmute(guid), ::std::mem::transmute(tdhcontextcount), ::std::mem::transmute(tdhcontext), ::std::mem::transmute(filtercount), ::std::mem::transmute(buffer), ::std::mem::transmute(buffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
#[inline]
pub unsafe fn TdhEnumerateProviders(pbuffer: *mut PROVIDER_ENUMERATION_INFO, pbuffersize: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TdhEnumerateProviders(pbuffer: *mut PROVIDER_ENUMERATION_INFO, pbuffersize: *mut u32) -> u32;
        }
        ::std::mem::transmute(TdhEnumerateProviders(::std::mem::transmute(pbuffer), ::std::mem::transmute(pbuffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
#[inline]
pub unsafe fn TdhEnumerateProvidersForDecodingSource(filter: DECODING_SOURCE, buffer: *mut PROVIDER_ENUMERATION_INFO, buffersize: u32, bufferrequired: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TdhEnumerateProvidersForDecodingSource(filter: DECODING_SOURCE, buffer: *mut PROVIDER_ENUMERATION_INFO, buffersize: u32, bufferrequired: *mut u32) -> u32;
        }
        ::std::mem::transmute(TdhEnumerateProvidersForDecodingSource(::std::mem::transmute(filter), ::std::mem::transmute(buffer), ::std::mem::transmute(buffersize), ::std::mem::transmute(bufferrequired)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TdhFormatProperty(eventinfo: *const TRACE_EVENT_INFO, mapinfo: *const EVENT_MAP_INFO, pointersize: u32, propertyintype: u16, propertyouttype: u16, propertylength: u16, userdatalength: u16, userdata: *const u8, buffersize: *mut u32, buffer: super::super::super::Foundation::PWSTR, userdataconsumed: *mut u16) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TdhFormatProperty(eventinfo: *const TRACE_EVENT_INFO, mapinfo: *const EVENT_MAP_INFO, pointersize: u32, propertyintype: u16, propertyouttype: u16, propertylength: u16, userdatalength: u16, userdata: *const u8, buffersize: *mut u32, buffer: super::super::super::Foundation::PWSTR, userdataconsumed: *mut u16) -> u32;
        }
        ::std::mem::transmute(TdhFormatProperty(
            ::std::mem::transmute(eventinfo),
            ::std::mem::transmute(mapinfo),
            ::std::mem::transmute(pointersize),
            ::std::mem::transmute(propertyintype),
            ::std::mem::transmute(propertyouttype),
            ::std::mem::transmute(propertylength),
            ::std::mem::transmute(userdatalength),
            ::std::mem::transmute(userdata),
            ::std::mem::transmute(buffersize),
            ::std::mem::transmute(buffer),
            ::std::mem::transmute(userdataconsumed),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
#[inline]
pub unsafe fn TdhGetDecodingParameter<'a, Param0: ::windows::runtime::IntoParam<'a, TDH_HANDLE>>(handle: Param0, tdhcontext: *mut TDH_CONTEXT) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TdhGetDecodingParameter(handle: TDH_HANDLE, tdhcontext: *mut TDH_CONTEXT) -> u32;
        }
        ::std::mem::transmute(TdhGetDecodingParameter(handle.into_param().abi(), ::std::mem::transmute(tdhcontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
#[inline]
pub unsafe fn TdhGetEventInformation(event: *const EVENT_RECORD, tdhcontextcount: u32, tdhcontext: *const TDH_CONTEXT, buffer: *mut TRACE_EVENT_INFO, buffersize: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TdhGetEventInformation(event: *const EVENT_RECORD, tdhcontextcount: u32, tdhcontext: *const TDH_CONTEXT, buffer: *mut TRACE_EVENT_INFO, buffersize: *mut u32) -> u32;
        }
        ::std::mem::transmute(TdhGetEventInformation(::std::mem::transmute(event), ::std::mem::transmute(tdhcontextcount), ::std::mem::transmute(tdhcontext), ::std::mem::transmute(buffer), ::std::mem::transmute(buffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TdhGetEventMapInformation<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(pevent: *const EVENT_RECORD, pmapname: Param1, pbuffer: *mut EVENT_MAP_INFO, pbuffersize: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TdhGetEventMapInformation(pevent: *const EVENT_RECORD, pmapname: super::super::super::Foundation::PWSTR, pbuffer: *mut EVENT_MAP_INFO, pbuffersize: *mut u32) -> u32;
        }
        ::std::mem::transmute(TdhGetEventMapInformation(::std::mem::transmute(pevent), pmapname.into_param().abi(), ::std::mem::transmute(pbuffer), ::std::mem::transmute(pbuffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
#[inline]
pub unsafe fn TdhGetManifestEventInformation(providerguid: *const ::windows::runtime::GUID, eventdescriptor: *const EVENT_DESCRIPTOR, buffer: *mut TRACE_EVENT_INFO, buffersize: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TdhGetManifestEventInformation(providerguid: *const ::windows::runtime::GUID, eventdescriptor: *const EVENT_DESCRIPTOR, buffer: *mut TRACE_EVENT_INFO, buffersize: *mut u32) -> u32;
        }
        ::std::mem::transmute(TdhGetManifestEventInformation(::std::mem::transmute(providerguid), ::std::mem::transmute(eventdescriptor), ::std::mem::transmute(buffer), ::std::mem::transmute(buffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
#[inline]
pub unsafe fn TdhGetProperty(pevent: *const EVENT_RECORD, tdhcontextcount: u32, ptdhcontext: *const TDH_CONTEXT, propertydatacount: u32, ppropertydata: *const PROPERTY_DATA_DESCRIPTOR, buffersize: u32, pbuffer: *mut u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TdhGetProperty(pevent: *const EVENT_RECORD, tdhcontextcount: u32, ptdhcontext: *const TDH_CONTEXT, propertydatacount: u32, ppropertydata: *const PROPERTY_DATA_DESCRIPTOR, buffersize: u32, pbuffer: *mut u8) -> u32;
        }
        ::std::mem::transmute(TdhGetProperty(::std::mem::transmute(pevent), ::std::mem::transmute(tdhcontextcount), ::std::mem::transmute(ptdhcontext), ::std::mem::transmute(propertydatacount), ::std::mem::transmute(ppropertydata), ::std::mem::transmute(buffersize), ::std::mem::transmute(pbuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
#[inline]
pub unsafe fn TdhGetPropertySize(pevent: *const EVENT_RECORD, tdhcontextcount: u32, ptdhcontext: *const TDH_CONTEXT, propertydatacount: u32, ppropertydata: *const PROPERTY_DATA_DESCRIPTOR, ppropertysize: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TdhGetPropertySize(pevent: *const EVENT_RECORD, tdhcontextcount: u32, ptdhcontext: *const TDH_CONTEXT, propertydatacount: u32, ppropertydata: *const PROPERTY_DATA_DESCRIPTOR, ppropertysize: *mut u32) -> u32;
        }
        ::std::mem::transmute(TdhGetPropertySize(::std::mem::transmute(pevent), ::std::mem::transmute(tdhcontextcount), ::std::mem::transmute(ptdhcontext), ::std::mem::transmute(propertydatacount), ::std::mem::transmute(ppropertydata), ::std::mem::transmute(ppropertysize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
#[inline]
pub unsafe fn TdhGetWppMessage<'a, Param0: ::windows::runtime::IntoParam<'a, TDH_HANDLE>>(handle: Param0, eventrecord: *const EVENT_RECORD, buffersize: *mut u32, buffer: *mut u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TdhGetWppMessage(handle: TDH_HANDLE, eventrecord: *const EVENT_RECORD, buffersize: *mut u32, buffer: *mut u8) -> u32;
        }
        ::std::mem::transmute(TdhGetWppMessage(handle.into_param().abi(), ::std::mem::transmute(eventrecord), ::std::mem::transmute(buffersize), ::std::mem::transmute(buffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TdhGetWppProperty<'a, Param0: ::windows::runtime::IntoParam<'a, TDH_HANDLE>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(handle: Param0, eventrecord: *const EVENT_RECORD, propertyname: Param2, buffersize: *mut u32, buffer: *mut u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TdhGetWppProperty(handle: TDH_HANDLE, eventrecord: *const EVENT_RECORD, propertyname: super::super::super::Foundation::PWSTR, buffersize: *mut u32, buffer: *mut u8) -> u32;
        }
        ::std::mem::transmute(TdhGetWppProperty(handle.into_param().abi(), ::std::mem::transmute(eventrecord), propertyname.into_param().abi(), ::std::mem::transmute(buffersize), ::std::mem::transmute(buffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TdhLoadManifest<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(manifest: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TdhLoadManifest(manifest: super::super::super::Foundation::PWSTR) -> u32;
        }
        ::std::mem::transmute(TdhLoadManifest(manifest.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TdhLoadManifestFromBinary<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(binarypath: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TdhLoadManifestFromBinary(binarypath: super::super::super::Foundation::PWSTR) -> u32;
        }
        ::std::mem::transmute(TdhLoadManifestFromBinary(binarypath.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
#[inline]
pub unsafe fn TdhLoadManifestFromMemory(pdata: *const ::std::ffi::c_void, cbdata: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TdhLoadManifestFromMemory(pdata: *const ::std::ffi::c_void, cbdata: u32) -> u32;
        }
        ::std::mem::transmute(TdhLoadManifestFromMemory(::std::mem::transmute(pdata), ::std::mem::transmute(cbdata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
#[inline]
pub unsafe fn TdhOpenDecodingHandle(handle: *mut TDH_HANDLE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TdhOpenDecodingHandle(handle: *mut TDH_HANDLE) -> u32;
        }
        ::std::mem::transmute(TdhOpenDecodingHandle(::std::mem::transmute(handle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
#[inline]
pub unsafe fn TdhQueryProviderFieldInformation(pguid: *const ::windows::runtime::GUID, eventfieldvalue: u64, eventfieldtype: EVENT_FIELD_TYPE, pbuffer: *mut PROVIDER_FIELD_INFOARRAY, pbuffersize: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TdhQueryProviderFieldInformation(pguid: *const ::windows::runtime::GUID, eventfieldvalue: u64, eventfieldtype: EVENT_FIELD_TYPE, pbuffer: *mut PROVIDER_FIELD_INFOARRAY, pbuffersize: *mut u32) -> u32;
        }
        ::std::mem::transmute(TdhQueryProviderFieldInformation(::std::mem::transmute(pguid), ::std::mem::transmute(eventfieldvalue), ::std::mem::transmute(eventfieldtype), ::std::mem::transmute(pbuffer), ::std::mem::transmute(pbuffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
#[inline]
pub unsafe fn TdhSetDecodingParameter<'a, Param0: ::windows::runtime::IntoParam<'a, TDH_HANDLE>>(handle: Param0, tdhcontext: *const TDH_CONTEXT) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TdhSetDecodingParameter(handle: TDH_HANDLE, tdhcontext: *const TDH_CONTEXT) -> u32;
        }
        ::std::mem::transmute(TdhSetDecodingParameter(handle.into_param().abi(), ::std::mem::transmute(tdhcontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TdhUnloadManifest<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(manifest: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TdhUnloadManifest(manifest: super::super::super::Foundation::PWSTR) -> u32;
        }
        ::std::mem::transmute(TdhUnloadManifest(manifest.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
#[inline]
pub unsafe fn TdhUnloadManifestFromMemory(pdata: *const ::std::ffi::c_void, cbdata: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TdhUnloadManifestFromMemory(pdata: *const ::std::ffi::c_void, cbdata: u32) -> u32;
        }
        ::std::mem::transmute(TdhUnloadManifestFromMemory(::std::mem::transmute(pdata), ::std::mem::transmute(cbdata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
#[inline]
pub unsafe fn TraceEvent(tracehandle: u64, eventtrace: *const EVENT_TRACE_HEADER) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TraceEvent(tracehandle: u64, eventtrace: *const EVENT_TRACE_HEADER) -> u32;
        }
        ::std::mem::transmute(TraceEvent(::std::mem::transmute(tracehandle), ::std::mem::transmute(eventtrace)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TraceEventInstance(tracehandle: u64, eventtrace: *const EVENT_INSTANCE_HEADER, instinfo: *const EVENT_INSTANCE_INFO, parentinstinfo: *const EVENT_INSTANCE_INFO) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TraceEventInstance(tracehandle: u64, eventtrace: *const EVENT_INSTANCE_HEADER, instinfo: *const EVENT_INSTANCE_INFO, parentinstinfo: *const EVENT_INSTANCE_INFO) -> u32;
        }
        ::std::mem::transmute(TraceEventInstance(::std::mem::transmute(tracehandle), ::std::mem::transmute(eventtrace), ::std::mem::transmute(instinfo), ::std::mem::transmute(parentinstinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
#[inline]
pub unsafe fn TraceMessage(loggerhandle: u64, messageflags: TRACE_MESSAGE_FLAGS, messageguid: *const ::windows::runtime::GUID, messagenumber: u16) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TraceMessage(loggerhandle: u64, messageflags: TRACE_MESSAGE_FLAGS, messageguid: *const ::windows::runtime::GUID, messagenumber: u16) -> u32;
        }
        ::std::mem::transmute(TraceMessage(::std::mem::transmute(loggerhandle), ::std::mem::transmute(messageflags), ::std::mem::transmute(messageguid), ::std::mem::transmute(messagenumber)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
#[inline]
pub unsafe fn TraceMessageVa(loggerhandle: u64, messageflags: TRACE_MESSAGE_FLAGS, messageguid: *const ::windows::runtime::GUID, messagenumber: u16, messagearglist: *const i8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TraceMessageVa(loggerhandle: u64, messageflags: TRACE_MESSAGE_FLAGS, messageguid: *const ::windows::runtime::GUID, messagenumber: u16, messagearglist: *const i8) -> u32;
        }
        ::std::mem::transmute(TraceMessageVa(::std::mem::transmute(loggerhandle), ::std::mem::transmute(messageflags), ::std::mem::transmute(messageguid), ::std::mem::transmute(messagenumber), ::std::mem::transmute(messagearglist)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
#[inline]
pub unsafe fn TraceQueryInformation(sessionhandle: u64, informationclass: TRACE_QUERY_INFO_CLASS, traceinformation: *mut ::std::ffi::c_void, informationlength: u32, returnlength: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TraceQueryInformation(sessionhandle: u64, informationclass: TRACE_QUERY_INFO_CLASS, traceinformation: *mut ::std::ffi::c_void, informationlength: u32, returnlength: *mut u32) -> u32;
        }
        ::std::mem::transmute(TraceQueryInformation(::std::mem::transmute(sessionhandle), ::std::mem::transmute(informationclass), ::std::mem::transmute(traceinformation), ::std::mem::transmute(informationlength), ::std::mem::transmute(returnlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
#[inline]
pub unsafe fn TraceSetInformation(sessionhandle: u64, informationclass: TRACE_QUERY_INFO_CLASS, traceinformation: *const ::std::ffi::c_void, informationlength: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TraceSetInformation(sessionhandle: u64, informationclass: TRACE_QUERY_INFO_CLASS, traceinformation: *const ::std::ffi::c_void, informationlength: u32) -> u32;
        }
        ::std::mem::transmute(TraceSetInformation(::std::mem::transmute(sessionhandle), ::std::mem::transmute(informationclass), ::std::mem::transmute(traceinformation), ::std::mem::transmute(informationlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
#[inline]
pub unsafe fn UnregisterTraceGuids(registrationhandle: u64) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UnregisterTraceGuids(registrationhandle: u64) -> u32;
        }
        ::std::mem::transmute(UnregisterTraceGuids(::std::mem::transmute(registrationhandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UpdateTraceA<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PSTR>>(tracehandle: u64, instancename: Param1, properties: *mut EVENT_TRACE_PROPERTIES) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UpdateTraceA(tracehandle: u64, instancename: super::super::super::Foundation::PSTR, properties: *mut EVENT_TRACE_PROPERTIES) -> u32;
        }
        ::std::mem::transmute(UpdateTraceA(::std::mem::transmute(tracehandle), instancename.into_param().abi(), ::std::mem::transmute(properties)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UpdateTraceW<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(tracehandle: u64, instancename: Param1, properties: *mut EVENT_TRACE_PROPERTIES) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UpdateTraceW(tracehandle: u64, instancename: super::super::super::Foundation::PWSTR, properties: *mut EVENT_TRACE_PROPERTIES) -> u32;
        }
        ::std::mem::transmute(UpdateTraceW(::std::mem::transmute(tracehandle), instancename.into_param().abi(), ::std::mem::transmute(properties)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub type WMIDPREQUEST = unsafe extern "system" fn(requestcode: WMIDPREQUESTCODE, requestcontext: *const ::std::ffi::c_void, buffersize: *mut u32, buffer: *mut ::std::ffi::c_void) -> u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WMIDPREQUESTCODE(pub i32);
pub const WMI_GET_ALL_DATA: WMIDPREQUESTCODE = WMIDPREQUESTCODE(0i32);
pub const WMI_GET_SINGLE_INSTANCE: WMIDPREQUESTCODE = WMIDPREQUESTCODE(1i32);
pub const WMI_SET_SINGLE_INSTANCE: WMIDPREQUESTCODE = WMIDPREQUESTCODE(2i32);
pub const WMI_SET_SINGLE_ITEM: WMIDPREQUESTCODE = WMIDPREQUESTCODE(3i32);
pub const WMI_ENABLE_EVENTS: WMIDPREQUESTCODE = WMIDPREQUESTCODE(4i32);
pub const WMI_DISABLE_EVENTS: WMIDPREQUESTCODE = WMIDPREQUESTCODE(5i32);
pub const WMI_ENABLE_COLLECTION: WMIDPREQUESTCODE = WMIDPREQUESTCODE(6i32);
pub const WMI_DISABLE_COLLECTION: WMIDPREQUESTCODE = WMIDPREQUESTCODE(7i32);
pub const WMI_REGINFO: WMIDPREQUESTCODE = WMIDPREQUESTCODE(8i32);
pub const WMI_EXECUTE_METHOD: WMIDPREQUESTCODE = WMIDPREQUESTCODE(9i32);
pub const WMI_CAPTURE_STATE: WMIDPREQUESTCODE = WMIDPREQUESTCODE(10i32);
impl ::std::convert::From<i32> for WMIDPREQUESTCODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WMIDPREQUESTCODE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const WMIGUID_EXECUTE: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const WMIGUID_NOTIFICATION: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const WMIGUID_QUERY: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const WMIGUID_READ_DESCRIPTION: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const WMIGUID_SET: u32 = 2u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub struct WMIREGGUIDW {
    pub Guid: ::windows::runtime::GUID,
    pub Flags: u32,
    pub InstanceCount: u32,
    pub Anonymous: WMIREGGUIDW_0,
}
impl WMIREGGUIDW {}
impl ::std::default::Default for WMIREGGUIDW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WMIREGGUIDW {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WMIREGGUIDW {}
unsafe impl ::windows::runtime::Abi for WMIREGGUIDW {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub union WMIREGGUIDW_0 {
    pub InstanceNameList: u32,
    pub BaseNameOffset: u32,
    pub Pdo: usize,
    pub InstanceInfo: usize,
}
impl WMIREGGUIDW_0 {}
impl ::std::default::Default for WMIREGGUIDW_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WMIREGGUIDW_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WMIREGGUIDW_0 {}
unsafe impl ::windows::runtime::Abi for WMIREGGUIDW_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub struct WMIREGINFOW {
    pub BufferSize: u32,
    pub NextWmiRegInfo: u32,
    pub RegistryPath: u32,
    pub MofResourceName: u32,
    pub GuidCount: u32,
    pub WmiRegGuid: [WMIREGGUIDW; 1],
}
impl WMIREGINFOW {}
impl ::std::default::Default for WMIREGINFOW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WMIREGINFOW {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WMIREGINFOW {}
unsafe impl ::windows::runtime::Abi for WMIREGINFOW {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const WMIREG_FLAG_EVENT_ONLY_GUID: u32 = 64u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const WMIREG_FLAG_EXPENSIVE: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const WMIREG_FLAG_INSTANCE_BASENAME: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const WMIREG_FLAG_INSTANCE_LIST: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const WMIREG_FLAG_INSTANCE_PDO: u32 = 32u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const WMIREG_FLAG_REMOVE_GUID: u32 = 65536u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const WMIREG_FLAG_RESERVED1: u32 = 131072u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const WMIREG_FLAG_RESERVED2: u32 = 262144u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const WMIREG_FLAG_TRACED_GUID: u32 = 524288u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const WMIREG_FLAG_TRACE_CONTROL_GUID: u32 = 4096u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const WMI_GLOBAL_LOGGER_ID: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const WMI_GUIDTYPE_DATA: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const WMI_GUIDTYPE_EVENT: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const WMI_GUIDTYPE_TRACE: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const WMI_GUIDTYPE_TRACECONTROL: u32 = 0u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
pub struct WNODE_ALL_DATA {
    pub WnodeHeader: WNODE_HEADER,
    pub DataBlockOffset: u32,
    pub InstanceCount: u32,
    pub OffsetInstanceNameOffsets: u32,
    pub Anonymous: WNODE_ALL_DATA_0,
}
#[cfg(feature = "Win32_Foundation")]
impl WNODE_ALL_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WNODE_ALL_DATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WNODE_ALL_DATA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WNODE_ALL_DATA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WNODE_ALL_DATA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub union WNODE_ALL_DATA_0 {
    pub FixedInstanceSize: u32,
    pub OffsetInstanceDataAndLength: [OFFSETINSTANCEDATAANDLENGTH; 1],
}
impl WNODE_ALL_DATA_0 {}
impl ::std::default::Default for WNODE_ALL_DATA_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WNODE_ALL_DATA_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WNODE_ALL_DATA_0 {}
unsafe impl ::windows::runtime::Abi for WNODE_ALL_DATA_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
pub struct WNODE_EVENT_ITEM {
    pub WnodeHeader: WNODE_HEADER,
}
#[cfg(feature = "Win32_Foundation")]
impl WNODE_EVENT_ITEM {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WNODE_EVENT_ITEM {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WNODE_EVENT_ITEM {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WNODE_EVENT_ITEM {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WNODE_EVENT_ITEM {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
pub struct WNODE_EVENT_REFERENCE {
    pub WnodeHeader: WNODE_HEADER,
    pub TargetGuid: ::windows::runtime::GUID,
    pub TargetDataBlockSize: u32,
    pub Anonymous: WNODE_EVENT_REFERENCE_0,
}
#[cfg(feature = "Win32_Foundation")]
impl WNODE_EVENT_REFERENCE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WNODE_EVENT_REFERENCE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WNODE_EVENT_REFERENCE {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WNODE_EVENT_REFERENCE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WNODE_EVENT_REFERENCE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub union WNODE_EVENT_REFERENCE_0 {
    pub TargetInstanceIndex: u32,
    pub TargetInstanceName: [u16; 1],
}
impl WNODE_EVENT_REFERENCE_0 {}
impl ::std::default::Default for WNODE_EVENT_REFERENCE_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WNODE_EVENT_REFERENCE_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WNODE_EVENT_REFERENCE_0 {}
unsafe impl ::windows::runtime::Abi for WNODE_EVENT_REFERENCE_0 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const WNODE_FLAG_ALL_DATA: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const WNODE_FLAG_ANSI_INSTANCENAMES: u32 = 16384u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const WNODE_FLAG_EVENT_ITEM: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const WNODE_FLAG_EVENT_REFERENCE: u32 = 8192u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const WNODE_FLAG_FIXED_INSTANCE_SIZE: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const WNODE_FLAG_INSTANCES_SAME: u32 = 64u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const WNODE_FLAG_INTERNAL: u32 = 256u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const WNODE_FLAG_LOG_WNODE: u32 = 262144u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const WNODE_FLAG_METHOD_ITEM: u32 = 32768u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const WNODE_FLAG_NO_HEADER: u32 = 2097152u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const WNODE_FLAG_PDO_INSTANCE_NAMES: u32 = 65536u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const WNODE_FLAG_PERSIST_EVENT: u32 = 1024u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const WNODE_FLAG_SEND_DATA_BLOCK: u32 = 4194304u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const WNODE_FLAG_SEVERITY_MASK: u32 = 4278190080u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const WNODE_FLAG_SINGLE_INSTANCE: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const WNODE_FLAG_SINGLE_ITEM: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const WNODE_FLAG_STATIC_INSTANCE_NAMES: u32 = 128u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const WNODE_FLAG_TOO_SMALL: u32 = 32u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const WNODE_FLAG_TRACED_GUID: u32 = 131072u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const WNODE_FLAG_USE_GUID_PTR: u32 = 524288u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const WNODE_FLAG_USE_MOF_PTR: u32 = 1048576u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const WNODE_FLAG_USE_TIMESTAMP: u32 = 512u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub const WNODE_FLAG_VERSIONED_PROPERTIES: u32 = 8388608u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
pub struct WNODE_HEADER {
    pub BufferSize: u32,
    pub ProviderId: u32,
    pub Anonymous1: WNODE_HEADER_0,
    pub Anonymous2: WNODE_HEADER_1,
    pub Guid: ::windows::runtime::GUID,
    pub ClientContext: u32,
    pub Flags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl WNODE_HEADER {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WNODE_HEADER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WNODE_HEADER {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WNODE_HEADER {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WNODE_HEADER {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub union WNODE_HEADER_0 {
    pub HistoricalContext: u64,
    pub Anonymous: WNODE_HEADER_0_0,
}
impl WNODE_HEADER_0 {}
impl ::std::default::Default for WNODE_HEADER_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WNODE_HEADER_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WNODE_HEADER_0 {}
unsafe impl ::windows::runtime::Abi for WNODE_HEADER_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
pub struct WNODE_HEADER_0_0 {
    pub Version: u32,
    pub Linkage: u32,
}
impl WNODE_HEADER_0_0 {}
impl ::std::default::Default for WNODE_HEADER_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WNODE_HEADER_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("Version", &self.Version).field("Linkage", &self.Linkage).finish()
    }
}
impl ::std::cmp::PartialEq for WNODE_HEADER_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Linkage == other.Linkage
    }
}
impl ::std::cmp::Eq for WNODE_HEADER_0_0 {}
unsafe impl ::windows::runtime::Abi for WNODE_HEADER_0_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
pub union WNODE_HEADER_1 {
    pub CountLost: u32,
    pub KernelHandle: super::super::super::Foundation::HANDLE,
    pub TimeStamp: i64,
}
#[cfg(feature = "Win32_Foundation")]
impl WNODE_HEADER_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WNODE_HEADER_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WNODE_HEADER_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WNODE_HEADER_1 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WNODE_HEADER_1 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
pub struct WNODE_METHOD_ITEM {
    pub WnodeHeader: WNODE_HEADER,
    pub OffsetInstanceName: u32,
    pub InstanceIndex: u32,
    pub MethodId: u32,
    pub DataBlockOffset: u32,
    pub SizeDataBlock: u32,
    pub VariableData: [u8; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl WNODE_METHOD_ITEM {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WNODE_METHOD_ITEM {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WNODE_METHOD_ITEM {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WNODE_METHOD_ITEM {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WNODE_METHOD_ITEM {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
pub struct WNODE_SINGLE_INSTANCE {
    pub WnodeHeader: WNODE_HEADER,
    pub OffsetInstanceName: u32,
    pub InstanceIndex: u32,
    pub DataBlockOffset: u32,
    pub SizeDataBlock: u32,
    pub VariableData: [u8; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl WNODE_SINGLE_INSTANCE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WNODE_SINGLE_INSTANCE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WNODE_SINGLE_INSTANCE {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WNODE_SINGLE_INSTANCE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WNODE_SINGLE_INSTANCE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
pub struct WNODE_SINGLE_ITEM {
    pub WnodeHeader: WNODE_HEADER,
    pub OffsetInstanceName: u32,
    pub InstanceIndex: u32,
    pub ItemId: u32,
    pub DataBlockOffset: u32,
    pub SizeDataItem: u32,
    pub VariableData: [u8; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl WNODE_SINGLE_ITEM {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WNODE_SINGLE_ITEM {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WNODE_SINGLE_ITEM {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WNODE_SINGLE_ITEM {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WNODE_SINGLE_ITEM {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
pub struct WNODE_TOO_SMALL {
    pub WnodeHeader: WNODE_HEADER,
    pub SizeNeeded: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl WNODE_TOO_SMALL {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WNODE_TOO_SMALL {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WNODE_TOO_SMALL {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WNODE_TOO_SMALL {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WNODE_TOO_SMALL {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct _TDH_IN_TYPE(pub i32);
pub const TDH_INTYPE_NULL: _TDH_IN_TYPE = _TDH_IN_TYPE(0i32);
pub const TDH_INTYPE_UNICODESTRING: _TDH_IN_TYPE = _TDH_IN_TYPE(1i32);
pub const TDH_INTYPE_ANSISTRING: _TDH_IN_TYPE = _TDH_IN_TYPE(2i32);
pub const TDH_INTYPE_INT8: _TDH_IN_TYPE = _TDH_IN_TYPE(3i32);
pub const TDH_INTYPE_UINT8: _TDH_IN_TYPE = _TDH_IN_TYPE(4i32);
pub const TDH_INTYPE_INT16: _TDH_IN_TYPE = _TDH_IN_TYPE(5i32);
pub const TDH_INTYPE_UINT16: _TDH_IN_TYPE = _TDH_IN_TYPE(6i32);
pub const TDH_INTYPE_INT32: _TDH_IN_TYPE = _TDH_IN_TYPE(7i32);
pub const TDH_INTYPE_UINT32: _TDH_IN_TYPE = _TDH_IN_TYPE(8i32);
pub const TDH_INTYPE_INT64: _TDH_IN_TYPE = _TDH_IN_TYPE(9i32);
pub const TDH_INTYPE_UINT64: _TDH_IN_TYPE = _TDH_IN_TYPE(10i32);
pub const TDH_INTYPE_FLOAT: _TDH_IN_TYPE = _TDH_IN_TYPE(11i32);
pub const TDH_INTYPE_DOUBLE: _TDH_IN_TYPE = _TDH_IN_TYPE(12i32);
pub const TDH_INTYPE_BOOLEAN: _TDH_IN_TYPE = _TDH_IN_TYPE(13i32);
pub const TDH_INTYPE_BINARY: _TDH_IN_TYPE = _TDH_IN_TYPE(14i32);
pub const TDH_INTYPE_GUID: _TDH_IN_TYPE = _TDH_IN_TYPE(15i32);
pub const TDH_INTYPE_POINTER: _TDH_IN_TYPE = _TDH_IN_TYPE(16i32);
pub const TDH_INTYPE_FILETIME: _TDH_IN_TYPE = _TDH_IN_TYPE(17i32);
pub const TDH_INTYPE_SYSTEMTIME: _TDH_IN_TYPE = _TDH_IN_TYPE(18i32);
pub const TDH_INTYPE_SID: _TDH_IN_TYPE = _TDH_IN_TYPE(19i32);
pub const TDH_INTYPE_HEXINT32: _TDH_IN_TYPE = _TDH_IN_TYPE(20i32);
pub const TDH_INTYPE_HEXINT64: _TDH_IN_TYPE = _TDH_IN_TYPE(21i32);
pub const TDH_INTYPE_MANIFEST_COUNTEDSTRING: _TDH_IN_TYPE = _TDH_IN_TYPE(22i32);
pub const TDH_INTYPE_MANIFEST_COUNTEDANSISTRING: _TDH_IN_TYPE = _TDH_IN_TYPE(23i32);
pub const TDH_INTYPE_RESERVED24: _TDH_IN_TYPE = _TDH_IN_TYPE(24i32);
pub const TDH_INTYPE_MANIFEST_COUNTEDBINARY: _TDH_IN_TYPE = _TDH_IN_TYPE(25i32);
pub const TDH_INTYPE_COUNTEDSTRING: _TDH_IN_TYPE = _TDH_IN_TYPE(300i32);
pub const TDH_INTYPE_COUNTEDANSISTRING: _TDH_IN_TYPE = _TDH_IN_TYPE(301i32);
pub const TDH_INTYPE_REVERSEDCOUNTEDSTRING: _TDH_IN_TYPE = _TDH_IN_TYPE(302i32);
pub const TDH_INTYPE_REVERSEDCOUNTEDANSISTRING: _TDH_IN_TYPE = _TDH_IN_TYPE(303i32);
pub const TDH_INTYPE_NONNULLTERMINATEDSTRING: _TDH_IN_TYPE = _TDH_IN_TYPE(304i32);
pub const TDH_INTYPE_NONNULLTERMINATEDANSISTRING: _TDH_IN_TYPE = _TDH_IN_TYPE(305i32);
pub const TDH_INTYPE_UNICODECHAR: _TDH_IN_TYPE = _TDH_IN_TYPE(306i32);
pub const TDH_INTYPE_ANSICHAR: _TDH_IN_TYPE = _TDH_IN_TYPE(307i32);
pub const TDH_INTYPE_SIZET: _TDH_IN_TYPE = _TDH_IN_TYPE(308i32);
pub const TDH_INTYPE_HEXDUMP: _TDH_IN_TYPE = _TDH_IN_TYPE(309i32);
pub const TDH_INTYPE_WBEMSID: _TDH_IN_TYPE = _TDH_IN_TYPE(310i32);
impl ::std::convert::From<i32> for _TDH_IN_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for _TDH_IN_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct _TDH_OUT_TYPE(pub i32);
pub const TDH_OUTTYPE_NULL: _TDH_OUT_TYPE = _TDH_OUT_TYPE(0i32);
pub const TDH_OUTTYPE_STRING: _TDH_OUT_TYPE = _TDH_OUT_TYPE(1i32);
pub const TDH_OUTTYPE_DATETIME: _TDH_OUT_TYPE = _TDH_OUT_TYPE(2i32);
pub const TDH_OUTTYPE_BYTE: _TDH_OUT_TYPE = _TDH_OUT_TYPE(3i32);
pub const TDH_OUTTYPE_UNSIGNEDBYTE: _TDH_OUT_TYPE = _TDH_OUT_TYPE(4i32);
pub const TDH_OUTTYPE_SHORT: _TDH_OUT_TYPE = _TDH_OUT_TYPE(5i32);
pub const TDH_OUTTYPE_UNSIGNEDSHORT: _TDH_OUT_TYPE = _TDH_OUT_TYPE(6i32);
pub const TDH_OUTTYPE_INT: _TDH_OUT_TYPE = _TDH_OUT_TYPE(7i32);
pub const TDH_OUTTYPE_UNSIGNEDINT: _TDH_OUT_TYPE = _TDH_OUT_TYPE(8i32);
pub const TDH_OUTTYPE_LONG: _TDH_OUT_TYPE = _TDH_OUT_TYPE(9i32);
pub const TDH_OUTTYPE_UNSIGNEDLONG: _TDH_OUT_TYPE = _TDH_OUT_TYPE(10i32);
pub const TDH_OUTTYPE_FLOAT: _TDH_OUT_TYPE = _TDH_OUT_TYPE(11i32);
pub const TDH_OUTTYPE_DOUBLE: _TDH_OUT_TYPE = _TDH_OUT_TYPE(12i32);
pub const TDH_OUTTYPE_BOOLEAN: _TDH_OUT_TYPE = _TDH_OUT_TYPE(13i32);
pub const TDH_OUTTYPE_GUID: _TDH_OUT_TYPE = _TDH_OUT_TYPE(14i32);
pub const TDH_OUTTYPE_HEXBINARY: _TDH_OUT_TYPE = _TDH_OUT_TYPE(15i32);
pub const TDH_OUTTYPE_HEXINT8: _TDH_OUT_TYPE = _TDH_OUT_TYPE(16i32);
pub const TDH_OUTTYPE_HEXINT16: _TDH_OUT_TYPE = _TDH_OUT_TYPE(17i32);
pub const TDH_OUTTYPE_HEXINT32: _TDH_OUT_TYPE = _TDH_OUT_TYPE(18i32);
pub const TDH_OUTTYPE_HEXINT64: _TDH_OUT_TYPE = _TDH_OUT_TYPE(19i32);
pub const TDH_OUTTYPE_PID: _TDH_OUT_TYPE = _TDH_OUT_TYPE(20i32);
pub const TDH_OUTTYPE_TID: _TDH_OUT_TYPE = _TDH_OUT_TYPE(21i32);
pub const TDH_OUTTYPE_PORT: _TDH_OUT_TYPE = _TDH_OUT_TYPE(22i32);
pub const TDH_OUTTYPE_IPV4: _TDH_OUT_TYPE = _TDH_OUT_TYPE(23i32);
pub const TDH_OUTTYPE_IPV6: _TDH_OUT_TYPE = _TDH_OUT_TYPE(24i32);
pub const TDH_OUTTYPE_SOCKETADDRESS: _TDH_OUT_TYPE = _TDH_OUT_TYPE(25i32);
pub const TDH_OUTTYPE_CIMDATETIME: _TDH_OUT_TYPE = _TDH_OUT_TYPE(26i32);
pub const TDH_OUTTYPE_ETWTIME: _TDH_OUT_TYPE = _TDH_OUT_TYPE(27i32);
pub const TDH_OUTTYPE_XML: _TDH_OUT_TYPE = _TDH_OUT_TYPE(28i32);
pub const TDH_OUTTYPE_ERRORCODE: _TDH_OUT_TYPE = _TDH_OUT_TYPE(29i32);
pub const TDH_OUTTYPE_WIN32ERROR: _TDH_OUT_TYPE = _TDH_OUT_TYPE(30i32);
pub const TDH_OUTTYPE_NTSTATUS: _TDH_OUT_TYPE = _TDH_OUT_TYPE(31i32);
pub const TDH_OUTTYPE_HRESULT: _TDH_OUT_TYPE = _TDH_OUT_TYPE(32i32);
pub const TDH_OUTTYPE_CULTURE_INSENSITIVE_DATETIME: _TDH_OUT_TYPE = _TDH_OUT_TYPE(33i32);
pub const TDH_OUTTYPE_JSON: _TDH_OUT_TYPE = _TDH_OUT_TYPE(34i32);
pub const TDH_OUTTYPE_UTF8: _TDH_OUT_TYPE = _TDH_OUT_TYPE(35i32);
pub const TDH_OUTTYPE_PKCS7_WITH_TYPE_INFO: _TDH_OUT_TYPE = _TDH_OUT_TYPE(36i32);
pub const TDH_OUTTYPE_CODE_POINTER: _TDH_OUT_TYPE = _TDH_OUT_TYPE(37i32);
pub const TDH_OUTTYPE_DATETIME_UTC: _TDH_OUT_TYPE = _TDH_OUT_TYPE(38i32);
pub const TDH_OUTTYPE_REDUCEDSTRING: _TDH_OUT_TYPE = _TDH_OUT_TYPE(300i32);
pub const TDH_OUTTYPE_NOPRINT: _TDH_OUT_TYPE = _TDH_OUT_TYPE(301i32);
impl ::std::convert::From<i32> for _TDH_OUT_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for _TDH_OUT_TYPE {
    type Abi = Self;
}
