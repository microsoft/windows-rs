#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct CLASSIC_EVENT_ID {
    pub EventGuid: ::windows::core::GUID,
    pub Type: u8,
    pub Reserved: [u8; 7],
}
impl ::core::marker::Copy for CLASSIC_EVENT_ID {}
impl ::core::clone::Clone for CLASSIC_EVENT_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CLASSIC_EVENT_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLASSIC_EVENT_ID").field("EventGuid", &self.EventGuid).field("Type", &self.Type).field("Reserved", &self.Reserved).finish()
    }
}
unsafe impl ::windows::core::Abi for CLASSIC_EVENT_ID {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CLASSIC_EVENT_ID {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CLASSIC_EVENT_ID>()) == 0 }
    }
}
impl ::core::cmp::Eq for CLASSIC_EVENT_ID {}
impl ::core::default::Default for CLASSIC_EVENT_ID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const CLSID_TraceRelogger: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7b40792d_05ff_44c4_9058_f440c71f17d4);
pub const CTraceRelogger: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7b40792d_05ff_44c4_9058_f440c71f17d4);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
#[inline]
pub unsafe fn CloseTrace(tracehandle: u64) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CloseTrace(tracehandle: u64) -> u32;
    }
    CloseTrace(tracehandle)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ControlTraceA<'a, P0>(tracehandle: u64, instancename: P0, properties: *mut EVENT_TRACE_PROPERTIES, controlcode: EVENT_TRACE_CONTROL) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ControlTraceA(tracehandle: u64, instancename: ::windows::core::PCSTR, properties: *mut EVENT_TRACE_PROPERTIES, controlcode: EVENT_TRACE_CONTROL) -> u32;
    }
    ControlTraceA(tracehandle, instancename.into(), ::core::mem::transmute(properties), controlcode)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ControlTraceW<'a, P0>(tracehandle: u64, instancename: P0, properties: *mut EVENT_TRACE_PROPERTIES, controlcode: EVENT_TRACE_CONTROL) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ControlTraceW(tracehandle: u64, instancename: ::windows::core::PCWSTR, properties: *mut EVENT_TRACE_PROPERTIES, controlcode: EVENT_TRACE_CONTROL) -> u32;
    }
    ControlTraceW(tracehandle, instancename.into(), ::core::mem::transmute(properties), controlcode)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateTraceInstanceId<'a, P0>(reghandle: P0, instinfo: *mut EVENT_INSTANCE_INFO) -> u32
where
    P0: ::std::convert::Into<super::super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreateTraceInstanceId(reghandle: super::super::super::Foundation::HANDLE, instinfo: *mut EVENT_INSTANCE_INFO) -> u32;
    }
    CreateTraceInstanceId(reghandle.into(), ::core::mem::transmute(instinfo))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
#[inline]
pub unsafe fn CveEventWrite<'a, P0, P1>(cveid: P0, additionaldetails: P1) -> i32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CveEventWrite(cveid: ::windows::core::PCWSTR, additionaldetails: ::windows::core::PCWSTR) -> i32;
    }
    CveEventWrite(cveid.into(), additionaldetails.into())
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DECODING_SOURCE(pub i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const DecodingSourceXMLFile: DECODING_SOURCE = DECODING_SOURCE(0i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const DecodingSourceWbem: DECODING_SOURCE = DECODING_SOURCE(1i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const DecodingSourceWPP: DECODING_SOURCE = DECODING_SOURCE(2i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const DecodingSourceTlg: DECODING_SOURCE = DECODING_SOURCE(3i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const DecodingSourceMax: DECODING_SOURCE = DECODING_SOURCE(4i32);
impl ::core::marker::Copy for DECODING_SOURCE {}
impl ::core::clone::Clone for DECODING_SOURCE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DECODING_SOURCE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DECODING_SOURCE {
    type Abi = Self;
}
impl ::core::fmt::Debug for DECODING_SOURCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DECODING_SOURCE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const DIAG_LOGGER_NAMEA: &str = "DiagLog";
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const DIAG_LOGGER_NAMEW: &str = "DiagLog";
pub const DefaultTraceSecurityGuid: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0811c1af_7a07_4a06_82ed_869455cdf713);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ENABLECALLBACK_ENABLED_STATE(pub u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_CONTROL_CODE_DISABLE_PROVIDER: ENABLECALLBACK_ENABLED_STATE = ENABLECALLBACK_ENABLED_STATE(0u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_CONTROL_CODE_ENABLE_PROVIDER: ENABLECALLBACK_ENABLED_STATE = ENABLECALLBACK_ENABLED_STATE(1u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_CONTROL_CODE_CAPTURE_STATE: ENABLECALLBACK_ENABLED_STATE = ENABLECALLBACK_ENABLED_STATE(2u32);
impl ::core::marker::Copy for ENABLECALLBACK_ENABLED_STATE {}
impl ::core::clone::Clone for ENABLECALLBACK_ENABLED_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ENABLECALLBACK_ENABLED_STATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ENABLECALLBACK_ENABLED_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for ENABLECALLBACK_ENABLED_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ENABLECALLBACK_ENABLED_STATE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct ENABLE_TRACE_PARAMETERS {
    pub Version: u32,
    pub EnableProperty: u32,
    pub ControlFlags: u32,
    pub SourceId: ::windows::core::GUID,
    pub EnableFilterDesc: *mut EVENT_FILTER_DESCRIPTOR,
    pub FilterDescCount: u32,
}
impl ::core::marker::Copy for ENABLE_TRACE_PARAMETERS {}
impl ::core::clone::Clone for ENABLE_TRACE_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ENABLE_TRACE_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENABLE_TRACE_PARAMETERS").field("Version", &self.Version).field("EnableProperty", &self.EnableProperty).field("ControlFlags", &self.ControlFlags).field("SourceId", &self.SourceId).field("EnableFilterDesc", &self.EnableFilterDesc).field("FilterDescCount", &self.FilterDescCount).finish()
    }
}
unsafe impl ::windows::core::Abi for ENABLE_TRACE_PARAMETERS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ENABLE_TRACE_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ENABLE_TRACE_PARAMETERS>()) == 0 }
    }
}
impl ::core::cmp::Eq for ENABLE_TRACE_PARAMETERS {}
impl ::core::default::Default for ENABLE_TRACE_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct ENABLE_TRACE_PARAMETERS_V1 {
    pub Version: u32,
    pub EnableProperty: u32,
    pub ControlFlags: u32,
    pub SourceId: ::windows::core::GUID,
    pub EnableFilterDesc: *mut EVENT_FILTER_DESCRIPTOR,
}
impl ::core::marker::Copy for ENABLE_TRACE_PARAMETERS_V1 {}
impl ::core::clone::Clone for ENABLE_TRACE_PARAMETERS_V1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ENABLE_TRACE_PARAMETERS_V1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENABLE_TRACE_PARAMETERS_V1").field("Version", &self.Version).field("EnableProperty", &self.EnableProperty).field("ControlFlags", &self.ControlFlags).field("SourceId", &self.SourceId).field("EnableFilterDesc", &self.EnableFilterDesc).finish()
    }
}
unsafe impl ::windows::core::Abi for ENABLE_TRACE_PARAMETERS_V1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ENABLE_TRACE_PARAMETERS_V1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ENABLE_TRACE_PARAMETERS_V1>()) == 0 }
    }
}
impl ::core::cmp::Eq for ENABLE_TRACE_PARAMETERS_V1 {}
impl ::core::default::Default for ENABLE_TRACE_PARAMETERS_V1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const ENABLE_TRACE_PARAMETERS_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const ENABLE_TRACE_PARAMETERS_VERSION_2: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const ETW_ASCIICHAR_TYPE_VALUE: u32 = 102u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const ETW_ASCIISTRING_TYPE_VALUE: u32 = 103u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const ETW_BOOLEAN_TYPE_VALUE: u32 = 14u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const ETW_BOOL_TYPE_VALUE: u32 = 108u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct ETW_BUFFER_CONTEXT {
    pub Anonymous: ETW_BUFFER_CONTEXT_0,
    pub LoggerId: u16,
}
impl ::core::marker::Copy for ETW_BUFFER_CONTEXT {}
impl ::core::clone::Clone for ETW_BUFFER_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ETW_BUFFER_CONTEXT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ETW_BUFFER_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ETW_BUFFER_CONTEXT>()) == 0 }
    }
}
impl ::core::cmp::Eq for ETW_BUFFER_CONTEXT {}
impl ::core::default::Default for ETW_BUFFER_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub union ETW_BUFFER_CONTEXT_0 {
    pub Anonymous: ETW_BUFFER_CONTEXT_0_0,
    pub ProcessorIndex: u16,
}
impl ::core::marker::Copy for ETW_BUFFER_CONTEXT_0 {}
impl ::core::clone::Clone for ETW_BUFFER_CONTEXT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ETW_BUFFER_CONTEXT_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ETW_BUFFER_CONTEXT_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ETW_BUFFER_CONTEXT_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for ETW_BUFFER_CONTEXT_0 {}
impl ::core::default::Default for ETW_BUFFER_CONTEXT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct ETW_BUFFER_CONTEXT_0_0 {
    pub ProcessorNumber: u8,
    pub Alignment: u8,
}
impl ::core::marker::Copy for ETW_BUFFER_CONTEXT_0_0 {}
impl ::core::clone::Clone for ETW_BUFFER_CONTEXT_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ETW_BUFFER_CONTEXT_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ETW_BUFFER_CONTEXT_0_0").field("ProcessorNumber", &self.ProcessorNumber).field("Alignment", &self.Alignment).finish()
    }
}
unsafe impl ::windows::core::Abi for ETW_BUFFER_CONTEXT_0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ETW_BUFFER_CONTEXT_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ETW_BUFFER_CONTEXT_0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for ETW_BUFFER_CONTEXT_0_0 {}
impl ::core::default::Default for ETW_BUFFER_CONTEXT_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const ETW_BYTE_TYPE_VALUE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const ETW_CHAR_TYPE_VALUE: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ETW_COMPRESSION_RESUMPTION_MODE(pub i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EtwCompressionModeRestart: ETW_COMPRESSION_RESUMPTION_MODE = ETW_COMPRESSION_RESUMPTION_MODE(0i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EtwCompressionModeNoDisable: ETW_COMPRESSION_RESUMPTION_MODE = ETW_COMPRESSION_RESUMPTION_MODE(1i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EtwCompressionModeNoRestart: ETW_COMPRESSION_RESUMPTION_MODE = ETW_COMPRESSION_RESUMPTION_MODE(2i32);
impl ::core::marker::Copy for ETW_COMPRESSION_RESUMPTION_MODE {}
impl ::core::clone::Clone for ETW_COMPRESSION_RESUMPTION_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ETW_COMPRESSION_RESUMPTION_MODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ETW_COMPRESSION_RESUMPTION_MODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for ETW_COMPRESSION_RESUMPTION_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ETW_COMPRESSION_RESUMPTION_MODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const ETW_COUNTED_ANSISTRING_TYPE_VALUE: u32 = 109u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const ETW_COUNTED_STRING_TYPE_VALUE: u32 = 104u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const ETW_DATETIME_TYPE_VALUE: u32 = 119u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const ETW_DECIMAL_TYPE_VALUE: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const ETW_DOUBLE_TYPE_VALUE: u32 = 13u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const ETW_GUID_TYPE_VALUE: u32 = 101u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const ETW_HIDDEN_TYPE_VALUE: u32 = 107u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const ETW_INT16_TYPE_VALUE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const ETW_INT32_TYPE_VALUE: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const ETW_INT64_TYPE_VALUE: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const ETW_NON_NULL_TERMINATED_STRING_TYPE_VALUE: u32 = 112u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const ETW_NULL_TYPE_VALUE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const ETW_OBJECT_TYPE_VALUE: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct ETW_PMC_COUNTER_OWNER {
    pub OwnerType: ETW_PMC_COUNTER_OWNER_TYPE,
    pub ProfileSource: u32,
    pub OwnerTag: u32,
}
impl ::core::marker::Copy for ETW_PMC_COUNTER_OWNER {}
impl ::core::clone::Clone for ETW_PMC_COUNTER_OWNER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ETW_PMC_COUNTER_OWNER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ETW_PMC_COUNTER_OWNER").field("OwnerType", &self.OwnerType).field("ProfileSource", &self.ProfileSource).field("OwnerTag", &self.OwnerTag).finish()
    }
}
unsafe impl ::windows::core::Abi for ETW_PMC_COUNTER_OWNER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ETW_PMC_COUNTER_OWNER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ETW_PMC_COUNTER_OWNER>()) == 0 }
    }
}
impl ::core::cmp::Eq for ETW_PMC_COUNTER_OWNER {}
impl ::core::default::Default for ETW_PMC_COUNTER_OWNER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct ETW_PMC_COUNTER_OWNERSHIP_STATUS {
    pub ProcessorNumber: u32,
    pub NumberOfCounters: u32,
    pub CounterOwners: [ETW_PMC_COUNTER_OWNER; 1],
}
impl ::core::marker::Copy for ETW_PMC_COUNTER_OWNERSHIP_STATUS {}
impl ::core::clone::Clone for ETW_PMC_COUNTER_OWNERSHIP_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ETW_PMC_COUNTER_OWNERSHIP_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ETW_PMC_COUNTER_OWNERSHIP_STATUS").field("ProcessorNumber", &self.ProcessorNumber).field("NumberOfCounters", &self.NumberOfCounters).field("CounterOwners", &self.CounterOwners).finish()
    }
}
unsafe impl ::windows::core::Abi for ETW_PMC_COUNTER_OWNERSHIP_STATUS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ETW_PMC_COUNTER_OWNERSHIP_STATUS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ETW_PMC_COUNTER_OWNERSHIP_STATUS>()) == 0 }
    }
}
impl ::core::cmp::Eq for ETW_PMC_COUNTER_OWNERSHIP_STATUS {}
impl ::core::default::Default for ETW_PMC_COUNTER_OWNERSHIP_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ETW_PMC_COUNTER_OWNER_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EtwPmcOwnerFree: ETW_PMC_COUNTER_OWNER_TYPE = ETW_PMC_COUNTER_OWNER_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EtwPmcOwnerUntagged: ETW_PMC_COUNTER_OWNER_TYPE = ETW_PMC_COUNTER_OWNER_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EtwPmcOwnerTagged: ETW_PMC_COUNTER_OWNER_TYPE = ETW_PMC_COUNTER_OWNER_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EtwPmcOwnerTaggedWithSource: ETW_PMC_COUNTER_OWNER_TYPE = ETW_PMC_COUNTER_OWNER_TYPE(3i32);
impl ::core::marker::Copy for ETW_PMC_COUNTER_OWNER_TYPE {}
impl ::core::clone::Clone for ETW_PMC_COUNTER_OWNER_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ETW_PMC_COUNTER_OWNER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ETW_PMC_COUNTER_OWNER_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for ETW_PMC_COUNTER_OWNER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ETW_PMC_COUNTER_OWNER_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const ETW_POINTER_TYPE_VALUE: u32 = 105u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ETW_PROCESS_HANDLE_INFO_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EtwQueryPartitionInformation: ETW_PROCESS_HANDLE_INFO_TYPE = ETW_PROCESS_HANDLE_INFO_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EtwQueryPartitionInformationV2: ETW_PROCESS_HANDLE_INFO_TYPE = ETW_PROCESS_HANDLE_INFO_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EtwQueryLastDroppedTimes: ETW_PROCESS_HANDLE_INFO_TYPE = ETW_PROCESS_HANDLE_INFO_TYPE(3i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EtwQueryProcessHandleInfoMax: ETW_PROCESS_HANDLE_INFO_TYPE = ETW_PROCESS_HANDLE_INFO_TYPE(4i32);
impl ::core::marker::Copy for ETW_PROCESS_HANDLE_INFO_TYPE {}
impl ::core::clone::Clone for ETW_PROCESS_HANDLE_INFO_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ETW_PROCESS_HANDLE_INFO_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ETW_PROCESS_HANDLE_INFO_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for ETW_PROCESS_HANDLE_INFO_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ETW_PROCESS_HANDLE_INFO_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ETW_PROVIDER_TRAIT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EtwProviderTraitTypeGroup: ETW_PROVIDER_TRAIT_TYPE = ETW_PROVIDER_TRAIT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EtwProviderTraitDecodeGuid: ETW_PROVIDER_TRAIT_TYPE = ETW_PROVIDER_TRAIT_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EtwProviderTraitTypeMax: ETW_PROVIDER_TRAIT_TYPE = ETW_PROVIDER_TRAIT_TYPE(3i32);
impl ::core::marker::Copy for ETW_PROVIDER_TRAIT_TYPE {}
impl ::core::clone::Clone for ETW_PROVIDER_TRAIT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ETW_PROVIDER_TRAIT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ETW_PROVIDER_TRAIT_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for ETW_PROVIDER_TRAIT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ETW_PROVIDER_TRAIT_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const ETW_PTVECTOR_TYPE_VALUE: u32 = 117u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const ETW_REDUCED_ANSISTRING_TYPE_VALUE: u32 = 113u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const ETW_REDUCED_STRING_TYPE_VALUE: u32 = 114u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const ETW_REFRENCE_TYPE_VALUE: u32 = 120u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const ETW_REVERSED_COUNTED_ANSISTRING_TYPE_VALUE: u32 = 111u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const ETW_REVERSED_COUNTED_STRING_TYPE_VALUE: u32 = 110u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const ETW_SBYTE_TYPE_VALUE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const ETW_SID_TYPE_VALUE: u32 = 115u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const ETW_SINGLE_TYPE_VALUE: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const ETW_SIZET_TYPE_VALUE: u32 = 106u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const ETW_STRING_TYPE_VALUE: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct ETW_TRACE_PARTITION_INFORMATION {
    pub PartitionId: ::windows::core::GUID,
    pub ParentId: ::windows::core::GUID,
    pub QpcOffsetFromRoot: i64,
    pub PartitionType: u32,
}
impl ::core::marker::Copy for ETW_TRACE_PARTITION_INFORMATION {}
impl ::core::clone::Clone for ETW_TRACE_PARTITION_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ETW_TRACE_PARTITION_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ETW_TRACE_PARTITION_INFORMATION").field("PartitionId", &self.PartitionId).field("ParentId", &self.ParentId).field("QpcOffsetFromRoot", &self.QpcOffsetFromRoot).field("PartitionType", &self.PartitionType).finish()
    }
}
unsafe impl ::windows::core::Abi for ETW_TRACE_PARTITION_INFORMATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ETW_TRACE_PARTITION_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ETW_TRACE_PARTITION_INFORMATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for ETW_TRACE_PARTITION_INFORMATION {}
impl ::core::default::Default for ETW_TRACE_PARTITION_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct ETW_TRACE_PARTITION_INFORMATION_V2 {
    pub QpcOffsetFromRoot: i64,
    pub PartitionType: u32,
    pub PartitionId: ::windows::core::PWSTR,
    pub ParentId: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for ETW_TRACE_PARTITION_INFORMATION_V2 {}
impl ::core::clone::Clone for ETW_TRACE_PARTITION_INFORMATION_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ETW_TRACE_PARTITION_INFORMATION_V2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ETW_TRACE_PARTITION_INFORMATION_V2").field("QpcOffsetFromRoot", &self.QpcOffsetFromRoot).field("PartitionType", &self.PartitionType).field("PartitionId", &self.PartitionId).field("ParentId", &self.ParentId).finish()
    }
}
unsafe impl ::windows::core::Abi for ETW_TRACE_PARTITION_INFORMATION_V2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ETW_TRACE_PARTITION_INFORMATION_V2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ETW_TRACE_PARTITION_INFORMATION_V2>()) == 0 }
    }
}
impl ::core::cmp::Eq for ETW_TRACE_PARTITION_INFORMATION_V2 {}
impl ::core::default::Default for ETW_TRACE_PARTITION_INFORMATION_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const ETW_UINT16_TYPE_VALUE: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const ETW_UINT32_TYPE_VALUE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const ETW_UINT64_TYPE_VALUE: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const ETW_VARIANT_TYPE_VALUE: u32 = 116u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const ETW_WMITIME_TYPE_VALUE: u32 = 118u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EVENTSECURITYOPERATION(pub i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EventSecuritySetDACL: EVENTSECURITYOPERATION = EVENTSECURITYOPERATION(0i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EventSecuritySetSACL: EVENTSECURITYOPERATION = EVENTSECURITYOPERATION(1i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EventSecurityAddDACL: EVENTSECURITYOPERATION = EVENTSECURITYOPERATION(2i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EventSecurityAddSACL: EVENTSECURITYOPERATION = EVENTSECURITYOPERATION(3i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EventSecurityMax: EVENTSECURITYOPERATION = EVENTSECURITYOPERATION(4i32);
impl ::core::marker::Copy for EVENTSECURITYOPERATION {}
impl ::core::clone::Clone for EVENTSECURITYOPERATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EVENTSECURITYOPERATION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for EVENTSECURITYOPERATION {
    type Abi = Self;
}
impl ::core::fmt::Debug for EVENTSECURITYOPERATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EVENTSECURITYOPERATION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_ACTIVITY_CTRL_CREATE_ID: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_ACTIVITY_CTRL_CREATE_SET_ID: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_ACTIVITY_CTRL_GET_ID: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_ACTIVITY_CTRL_GET_SET_ID: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_ACTIVITY_CTRL_SET_ID: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct EVENT_DATA_DESCRIPTOR {
    pub Ptr: u64,
    pub Size: u32,
    pub Anonymous: EVENT_DATA_DESCRIPTOR_0,
}
impl ::core::marker::Copy for EVENT_DATA_DESCRIPTOR {}
impl ::core::clone::Clone for EVENT_DATA_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for EVENT_DATA_DESCRIPTOR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EVENT_DATA_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EVENT_DATA_DESCRIPTOR>()) == 0 }
    }
}
impl ::core::cmp::Eq for EVENT_DATA_DESCRIPTOR {}
impl ::core::default::Default for EVENT_DATA_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub union EVENT_DATA_DESCRIPTOR_0 {
    pub Reserved: u32,
    pub Anonymous: EVENT_DATA_DESCRIPTOR_0_0,
}
impl ::core::marker::Copy for EVENT_DATA_DESCRIPTOR_0 {}
impl ::core::clone::Clone for EVENT_DATA_DESCRIPTOR_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for EVENT_DATA_DESCRIPTOR_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EVENT_DATA_DESCRIPTOR_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EVENT_DATA_DESCRIPTOR_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for EVENT_DATA_DESCRIPTOR_0 {}
impl ::core::default::Default for EVENT_DATA_DESCRIPTOR_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct EVENT_DATA_DESCRIPTOR_0_0 {
    pub Type: u8,
    pub Reserved1: u8,
    pub Reserved2: u16,
}
impl ::core::marker::Copy for EVENT_DATA_DESCRIPTOR_0_0 {}
impl ::core::clone::Clone for EVENT_DATA_DESCRIPTOR_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EVENT_DATA_DESCRIPTOR_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EVENT_DATA_DESCRIPTOR_0_0").field("Type", &self.Type).field("Reserved1", &self.Reserved1).field("Reserved2", &self.Reserved2).finish()
    }
}
unsafe impl ::windows::core::Abi for EVENT_DATA_DESCRIPTOR_0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EVENT_DATA_DESCRIPTOR_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EVENT_DATA_DESCRIPTOR_0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for EVENT_DATA_DESCRIPTOR_0_0 {}
impl ::core::default::Default for EVENT_DATA_DESCRIPTOR_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_DATA_DESCRIPTOR_TYPE_EVENT_METADATA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_DATA_DESCRIPTOR_TYPE_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_DATA_DESCRIPTOR_TYPE_PROVIDER_METADATA: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_DATA_DESCRIPTOR_TYPE_TIMESTAMP_OVERRIDE: u32 = 3u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct EVENT_DESCRIPTOR {
    pub Id: u16,
    pub Version: u8,
    pub Channel: u8,
    pub Level: u8,
    pub Opcode: u8,
    pub Task: u16,
    pub Keyword: u64,
}
impl ::core::marker::Copy for EVENT_DESCRIPTOR {}
impl ::core::clone::Clone for EVENT_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EVENT_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EVENT_DESCRIPTOR").field("Id", &self.Id).field("Version", &self.Version).field("Channel", &self.Channel).field("Level", &self.Level).field("Opcode", &self.Opcode).field("Task", &self.Task).field("Keyword", &self.Keyword).finish()
    }
}
unsafe impl ::windows::core::Abi for EVENT_DESCRIPTOR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EVENT_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EVENT_DESCRIPTOR>()) == 0 }
    }
}
impl ::core::cmp::Eq for EVENT_DESCRIPTOR {}
impl ::core::default::Default for EVENT_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_ENABLE_PROPERTY_ENABLE_KEYWORD_0: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_ENABLE_PROPERTY_ENABLE_SILOS: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_ENABLE_PROPERTY_EVENT_KEY: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_ENABLE_PROPERTY_EXCLUDE_INPRIVATE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_ENABLE_PROPERTY_IGNORE_KEYWORD_0: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_ENABLE_PROPERTY_PROCESS_START_KEY: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_ENABLE_PROPERTY_PROVIDER_GROUP: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_ENABLE_PROPERTY_PSM_KEY: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_ENABLE_PROPERTY_SID: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_ENABLE_PROPERTY_SOURCE_CONTAINER_TRACKING: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_ENABLE_PROPERTY_STACK_TRACE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_ENABLE_PROPERTY_TS_ID: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct EVENT_EXTENDED_ITEM_EVENT_KEY {
    pub Key: u64,
}
impl ::core::marker::Copy for EVENT_EXTENDED_ITEM_EVENT_KEY {}
impl ::core::clone::Clone for EVENT_EXTENDED_ITEM_EVENT_KEY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EVENT_EXTENDED_ITEM_EVENT_KEY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EVENT_EXTENDED_ITEM_EVENT_KEY").field("Key", &self.Key).finish()
    }
}
unsafe impl ::windows::core::Abi for EVENT_EXTENDED_ITEM_EVENT_KEY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EVENT_EXTENDED_ITEM_EVENT_KEY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EVENT_EXTENDED_ITEM_EVENT_KEY>()) == 0 }
    }
}
impl ::core::cmp::Eq for EVENT_EXTENDED_ITEM_EVENT_KEY {}
impl ::core::default::Default for EVENT_EXTENDED_ITEM_EVENT_KEY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct EVENT_EXTENDED_ITEM_INSTANCE {
    pub InstanceId: u32,
    pub ParentInstanceId: u32,
    pub ParentGuid: ::windows::core::GUID,
}
impl ::core::marker::Copy for EVENT_EXTENDED_ITEM_INSTANCE {}
impl ::core::clone::Clone for EVENT_EXTENDED_ITEM_INSTANCE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EVENT_EXTENDED_ITEM_INSTANCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EVENT_EXTENDED_ITEM_INSTANCE").field("InstanceId", &self.InstanceId).field("ParentInstanceId", &self.ParentInstanceId).field("ParentGuid", &self.ParentGuid).finish()
    }
}
unsafe impl ::windows::core::Abi for EVENT_EXTENDED_ITEM_INSTANCE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EVENT_EXTENDED_ITEM_INSTANCE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EVENT_EXTENDED_ITEM_INSTANCE>()) == 0 }
    }
}
impl ::core::cmp::Eq for EVENT_EXTENDED_ITEM_INSTANCE {}
impl ::core::default::Default for EVENT_EXTENDED_ITEM_INSTANCE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct EVENT_EXTENDED_ITEM_PEBS_INDEX {
    pub PebsIndex: u64,
}
impl ::core::marker::Copy for EVENT_EXTENDED_ITEM_PEBS_INDEX {}
impl ::core::clone::Clone for EVENT_EXTENDED_ITEM_PEBS_INDEX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EVENT_EXTENDED_ITEM_PEBS_INDEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EVENT_EXTENDED_ITEM_PEBS_INDEX").field("PebsIndex", &self.PebsIndex).finish()
    }
}
unsafe impl ::windows::core::Abi for EVENT_EXTENDED_ITEM_PEBS_INDEX {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EVENT_EXTENDED_ITEM_PEBS_INDEX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EVENT_EXTENDED_ITEM_PEBS_INDEX>()) == 0 }
    }
}
impl ::core::cmp::Eq for EVENT_EXTENDED_ITEM_PEBS_INDEX {}
impl ::core::default::Default for EVENT_EXTENDED_ITEM_PEBS_INDEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct EVENT_EXTENDED_ITEM_PMC_COUNTERS {
    pub Counter: [u64; 1],
}
impl ::core::marker::Copy for EVENT_EXTENDED_ITEM_PMC_COUNTERS {}
impl ::core::clone::Clone for EVENT_EXTENDED_ITEM_PMC_COUNTERS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EVENT_EXTENDED_ITEM_PMC_COUNTERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EVENT_EXTENDED_ITEM_PMC_COUNTERS").field("Counter", &self.Counter).finish()
    }
}
unsafe impl ::windows::core::Abi for EVENT_EXTENDED_ITEM_PMC_COUNTERS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EVENT_EXTENDED_ITEM_PMC_COUNTERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EVENT_EXTENDED_ITEM_PMC_COUNTERS>()) == 0 }
    }
}
impl ::core::cmp::Eq for EVENT_EXTENDED_ITEM_PMC_COUNTERS {}
impl ::core::default::Default for EVENT_EXTENDED_ITEM_PMC_COUNTERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct EVENT_EXTENDED_ITEM_PROCESS_START_KEY {
    pub ProcessStartKey: u64,
}
impl ::core::marker::Copy for EVENT_EXTENDED_ITEM_PROCESS_START_KEY {}
impl ::core::clone::Clone for EVENT_EXTENDED_ITEM_PROCESS_START_KEY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EVENT_EXTENDED_ITEM_PROCESS_START_KEY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EVENT_EXTENDED_ITEM_PROCESS_START_KEY").field("ProcessStartKey", &self.ProcessStartKey).finish()
    }
}
unsafe impl ::windows::core::Abi for EVENT_EXTENDED_ITEM_PROCESS_START_KEY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EVENT_EXTENDED_ITEM_PROCESS_START_KEY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EVENT_EXTENDED_ITEM_PROCESS_START_KEY>()) == 0 }
    }
}
impl ::core::cmp::Eq for EVENT_EXTENDED_ITEM_PROCESS_START_KEY {}
impl ::core::default::Default for EVENT_EXTENDED_ITEM_PROCESS_START_KEY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct EVENT_EXTENDED_ITEM_RELATED_ACTIVITYID {
    pub RelatedActivityId: ::windows::core::GUID,
}
impl ::core::marker::Copy for EVENT_EXTENDED_ITEM_RELATED_ACTIVITYID {}
impl ::core::clone::Clone for EVENT_EXTENDED_ITEM_RELATED_ACTIVITYID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EVENT_EXTENDED_ITEM_RELATED_ACTIVITYID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EVENT_EXTENDED_ITEM_RELATED_ACTIVITYID").field("RelatedActivityId", &self.RelatedActivityId).finish()
    }
}
unsafe impl ::windows::core::Abi for EVENT_EXTENDED_ITEM_RELATED_ACTIVITYID {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EVENT_EXTENDED_ITEM_RELATED_ACTIVITYID {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EVENT_EXTENDED_ITEM_RELATED_ACTIVITYID>()) == 0 }
    }
}
impl ::core::cmp::Eq for EVENT_EXTENDED_ITEM_RELATED_ACTIVITYID {}
impl ::core::default::Default for EVENT_EXTENDED_ITEM_RELATED_ACTIVITYID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct EVENT_EXTENDED_ITEM_STACK_KEY32 {
    pub MatchId: u64,
    pub StackKey: u32,
    pub Padding: u32,
}
impl ::core::marker::Copy for EVENT_EXTENDED_ITEM_STACK_KEY32 {}
impl ::core::clone::Clone for EVENT_EXTENDED_ITEM_STACK_KEY32 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EVENT_EXTENDED_ITEM_STACK_KEY32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EVENT_EXTENDED_ITEM_STACK_KEY32").field("MatchId", &self.MatchId).field("StackKey", &self.StackKey).field("Padding", &self.Padding).finish()
    }
}
unsafe impl ::windows::core::Abi for EVENT_EXTENDED_ITEM_STACK_KEY32 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EVENT_EXTENDED_ITEM_STACK_KEY32 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EVENT_EXTENDED_ITEM_STACK_KEY32>()) == 0 }
    }
}
impl ::core::cmp::Eq for EVENT_EXTENDED_ITEM_STACK_KEY32 {}
impl ::core::default::Default for EVENT_EXTENDED_ITEM_STACK_KEY32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct EVENT_EXTENDED_ITEM_STACK_KEY64 {
    pub MatchId: u64,
    pub StackKey: u64,
}
impl ::core::marker::Copy for EVENT_EXTENDED_ITEM_STACK_KEY64 {}
impl ::core::clone::Clone for EVENT_EXTENDED_ITEM_STACK_KEY64 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EVENT_EXTENDED_ITEM_STACK_KEY64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EVENT_EXTENDED_ITEM_STACK_KEY64").field("MatchId", &self.MatchId).field("StackKey", &self.StackKey).finish()
    }
}
unsafe impl ::windows::core::Abi for EVENT_EXTENDED_ITEM_STACK_KEY64 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EVENT_EXTENDED_ITEM_STACK_KEY64 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EVENT_EXTENDED_ITEM_STACK_KEY64>()) == 0 }
    }
}
impl ::core::cmp::Eq for EVENT_EXTENDED_ITEM_STACK_KEY64 {}
impl ::core::default::Default for EVENT_EXTENDED_ITEM_STACK_KEY64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct EVENT_EXTENDED_ITEM_STACK_TRACE32 {
    pub MatchId: u64,
    pub Address: [u32; 1],
}
impl ::core::marker::Copy for EVENT_EXTENDED_ITEM_STACK_TRACE32 {}
impl ::core::clone::Clone for EVENT_EXTENDED_ITEM_STACK_TRACE32 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EVENT_EXTENDED_ITEM_STACK_TRACE32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EVENT_EXTENDED_ITEM_STACK_TRACE32").field("MatchId", &self.MatchId).field("Address", &self.Address).finish()
    }
}
unsafe impl ::windows::core::Abi for EVENT_EXTENDED_ITEM_STACK_TRACE32 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EVENT_EXTENDED_ITEM_STACK_TRACE32 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EVENT_EXTENDED_ITEM_STACK_TRACE32>()) == 0 }
    }
}
impl ::core::cmp::Eq for EVENT_EXTENDED_ITEM_STACK_TRACE32 {}
impl ::core::default::Default for EVENT_EXTENDED_ITEM_STACK_TRACE32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct EVENT_EXTENDED_ITEM_STACK_TRACE64 {
    pub MatchId: u64,
    pub Address: [u64; 1],
}
impl ::core::marker::Copy for EVENT_EXTENDED_ITEM_STACK_TRACE64 {}
impl ::core::clone::Clone for EVENT_EXTENDED_ITEM_STACK_TRACE64 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EVENT_EXTENDED_ITEM_STACK_TRACE64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EVENT_EXTENDED_ITEM_STACK_TRACE64").field("MatchId", &self.MatchId).field("Address", &self.Address).finish()
    }
}
unsafe impl ::windows::core::Abi for EVENT_EXTENDED_ITEM_STACK_TRACE64 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EVENT_EXTENDED_ITEM_STACK_TRACE64 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EVENT_EXTENDED_ITEM_STACK_TRACE64>()) == 0 }
    }
}
impl ::core::cmp::Eq for EVENT_EXTENDED_ITEM_STACK_TRACE64 {}
impl ::core::default::Default for EVENT_EXTENDED_ITEM_STACK_TRACE64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct EVENT_EXTENDED_ITEM_TS_ID {
    pub SessionId: u32,
}
impl ::core::marker::Copy for EVENT_EXTENDED_ITEM_TS_ID {}
impl ::core::clone::Clone for EVENT_EXTENDED_ITEM_TS_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EVENT_EXTENDED_ITEM_TS_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EVENT_EXTENDED_ITEM_TS_ID").field("SessionId", &self.SessionId).finish()
    }
}
unsafe impl ::windows::core::Abi for EVENT_EXTENDED_ITEM_TS_ID {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EVENT_EXTENDED_ITEM_TS_ID {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EVENT_EXTENDED_ITEM_TS_ID>()) == 0 }
    }
}
impl ::core::cmp::Eq for EVENT_EXTENDED_ITEM_TS_ID {}
impl ::core::default::Default for EVENT_EXTENDED_ITEM_TS_ID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EVENT_FIELD_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EventKeywordInformation: EVENT_FIELD_TYPE = EVENT_FIELD_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EventLevelInformation: EVENT_FIELD_TYPE = EVENT_FIELD_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EventChannelInformation: EVENT_FIELD_TYPE = EVENT_FIELD_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EventTaskInformation: EVENT_FIELD_TYPE = EVENT_FIELD_TYPE(3i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EventOpcodeInformation: EVENT_FIELD_TYPE = EVENT_FIELD_TYPE(4i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EventInformationMax: EVENT_FIELD_TYPE = EVENT_FIELD_TYPE(5i32);
impl ::core::marker::Copy for EVENT_FIELD_TYPE {}
impl ::core::clone::Clone for EVENT_FIELD_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EVENT_FIELD_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for EVENT_FIELD_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for EVENT_FIELD_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EVENT_FIELD_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct EVENT_FILTER_DESCRIPTOR {
    pub Ptr: u64,
    pub Size: u32,
    pub Type: u32,
}
impl ::core::marker::Copy for EVENT_FILTER_DESCRIPTOR {}
impl ::core::clone::Clone for EVENT_FILTER_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EVENT_FILTER_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EVENT_FILTER_DESCRIPTOR").field("Ptr", &self.Ptr).field("Size", &self.Size).field("Type", &self.Type).finish()
    }
}
unsafe impl ::windows::core::Abi for EVENT_FILTER_DESCRIPTOR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EVENT_FILTER_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EVENT_FILTER_DESCRIPTOR>()) == 0 }
    }
}
impl ::core::cmp::Eq for EVENT_FILTER_DESCRIPTOR {}
impl ::core::default::Default for EVENT_FILTER_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EVENT_FILTER_EVENT_ID {
    pub FilterIn: super::super::super::Foundation::BOOLEAN,
    pub Reserved: u8,
    pub Count: u16,
    pub Events: [u16; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EVENT_FILTER_EVENT_ID {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EVENT_FILTER_EVENT_ID {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EVENT_FILTER_EVENT_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EVENT_FILTER_EVENT_ID").field("FilterIn", &self.FilterIn).field("Reserved", &self.Reserved).field("Count", &self.Count).field("Events", &self.Events).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for EVENT_FILTER_EVENT_ID {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EVENT_FILTER_EVENT_ID {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EVENT_FILTER_EVENT_ID>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EVENT_FILTER_EVENT_ID {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EVENT_FILTER_EVENT_ID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EVENT_FILTER_EVENT_NAME {
    pub MatchAnyKeyword: u64,
    pub MatchAllKeyword: u64,
    pub Level: u8,
    pub FilterIn: super::super::super::Foundation::BOOLEAN,
    pub NameCount: u16,
    pub Names: [u8; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EVENT_FILTER_EVENT_NAME {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EVENT_FILTER_EVENT_NAME {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EVENT_FILTER_EVENT_NAME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EVENT_FILTER_EVENT_NAME").field("MatchAnyKeyword", &self.MatchAnyKeyword).field("MatchAllKeyword", &self.MatchAllKeyword).field("Level", &self.Level).field("FilterIn", &self.FilterIn).field("NameCount", &self.NameCount).field("Names", &self.Names).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for EVENT_FILTER_EVENT_NAME {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EVENT_FILTER_EVENT_NAME {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EVENT_FILTER_EVENT_NAME>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EVENT_FILTER_EVENT_NAME {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EVENT_FILTER_EVENT_NAME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct EVENT_FILTER_HEADER {
    pub Id: u16,
    pub Version: u8,
    pub Reserved: [u8; 5],
    pub InstanceId: u64,
    pub Size: u32,
    pub NextOffset: u32,
}
impl ::core::marker::Copy for EVENT_FILTER_HEADER {}
impl ::core::clone::Clone for EVENT_FILTER_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EVENT_FILTER_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EVENT_FILTER_HEADER").field("Id", &self.Id).field("Version", &self.Version).field("Reserved", &self.Reserved).field("InstanceId", &self.InstanceId).field("Size", &self.Size).field("NextOffset", &self.NextOffset).finish()
    }
}
unsafe impl ::windows::core::Abi for EVENT_FILTER_HEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EVENT_FILTER_HEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EVENT_FILTER_HEADER>()) == 0 }
    }
}
impl ::core::cmp::Eq for EVENT_FILTER_HEADER {}
impl ::core::default::Default for EVENT_FILTER_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EVENT_FILTER_LEVEL_KW {
    pub MatchAnyKeyword: u64,
    pub MatchAllKeyword: u64,
    pub Level: u8,
    pub FilterIn: super::super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EVENT_FILTER_LEVEL_KW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EVENT_FILTER_LEVEL_KW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EVENT_FILTER_LEVEL_KW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EVENT_FILTER_LEVEL_KW").field("MatchAnyKeyword", &self.MatchAnyKeyword).field("MatchAllKeyword", &self.MatchAllKeyword).field("Level", &self.Level).field("FilterIn", &self.FilterIn).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for EVENT_FILTER_LEVEL_KW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EVENT_FILTER_LEVEL_KW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EVENT_FILTER_LEVEL_KW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EVENT_FILTER_LEVEL_KW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EVENT_FILTER_LEVEL_KW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_FILTER_TYPE_CONTAINER: u32 = 2147516416u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_FILTER_TYPE_EVENT_ID: u32 = 2147484160u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_FILTER_TYPE_EVENT_NAME: u32 = 2147484672u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_FILTER_TYPE_EXECUTABLE_NAME: u32 = 2147483656u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_FILTER_TYPE_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_FILTER_TYPE_PACKAGE_APP_ID: u32 = 2147483680u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_FILTER_TYPE_PACKAGE_ID: u32 = 2147483664u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_FILTER_TYPE_PAYLOAD: u32 = 2147483904u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_FILTER_TYPE_PID: u32 = 2147483652u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_FILTER_TYPE_SCHEMATIZED: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_FILTER_TYPE_STACKWALK: u32 = 2147487744u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_FILTER_TYPE_STACKWALK_LEVEL_KW: u32 = 2147500032u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_FILTER_TYPE_STACKWALK_NAME: u32 = 2147491840u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_FILTER_TYPE_SYSTEM_FLAGS: u32 = 2147483649u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_FILTER_TYPE_TRACEHANDLE: u32 = 2147483650u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct EVENT_HEADER {
    pub Size: u16,
    pub HeaderType: u16,
    pub Flags: u16,
    pub EventProperty: u16,
    pub ThreadId: u32,
    pub ProcessId: u32,
    pub TimeStamp: i64,
    pub ProviderId: ::windows::core::GUID,
    pub EventDescriptor: EVENT_DESCRIPTOR,
    pub Anonymous: EVENT_HEADER_0,
    pub ActivityId: ::windows::core::GUID,
}
impl ::core::marker::Copy for EVENT_HEADER {}
impl ::core::clone::Clone for EVENT_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for EVENT_HEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EVENT_HEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EVENT_HEADER>()) == 0 }
    }
}
impl ::core::cmp::Eq for EVENT_HEADER {}
impl ::core::default::Default for EVENT_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub union EVENT_HEADER_0 {
    pub Anonymous: EVENT_HEADER_0_0,
    pub ProcessorTime: u64,
}
impl ::core::marker::Copy for EVENT_HEADER_0 {}
impl ::core::clone::Clone for EVENT_HEADER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for EVENT_HEADER_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EVENT_HEADER_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EVENT_HEADER_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for EVENT_HEADER_0 {}
impl ::core::default::Default for EVENT_HEADER_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct EVENT_HEADER_0_0 {
    pub KernelTime: u32,
    pub UserTime: u32,
}
impl ::core::marker::Copy for EVENT_HEADER_0_0 {}
impl ::core::clone::Clone for EVENT_HEADER_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EVENT_HEADER_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EVENT_HEADER_0_0").field("KernelTime", &self.KernelTime).field("UserTime", &self.UserTime).finish()
    }
}
unsafe impl ::windows::core::Abi for EVENT_HEADER_0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EVENT_HEADER_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EVENT_HEADER_0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for EVENT_HEADER_0_0 {}
impl ::core::default::Default for EVENT_HEADER_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct EVENT_HEADER_EXTENDED_DATA_ITEM {
    pub Reserved1: u16,
    pub ExtType: u16,
    pub Anonymous: EVENT_HEADER_EXTENDED_DATA_ITEM_0,
    pub DataSize: u16,
    pub DataPtr: u64,
}
impl ::core::marker::Copy for EVENT_HEADER_EXTENDED_DATA_ITEM {}
impl ::core::clone::Clone for EVENT_HEADER_EXTENDED_DATA_ITEM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EVENT_HEADER_EXTENDED_DATA_ITEM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EVENT_HEADER_EXTENDED_DATA_ITEM").field("Reserved1", &self.Reserved1).field("ExtType", &self.ExtType).field("Anonymous", &self.Anonymous).field("DataSize", &self.DataSize).field("DataPtr", &self.DataPtr).finish()
    }
}
unsafe impl ::windows::core::Abi for EVENT_HEADER_EXTENDED_DATA_ITEM {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EVENT_HEADER_EXTENDED_DATA_ITEM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EVENT_HEADER_EXTENDED_DATA_ITEM>()) == 0 }
    }
}
impl ::core::cmp::Eq for EVENT_HEADER_EXTENDED_DATA_ITEM {}
impl ::core::default::Default for EVENT_HEADER_EXTENDED_DATA_ITEM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct EVENT_HEADER_EXTENDED_DATA_ITEM_0 {
    pub _bitfield: u16,
}
impl ::core::marker::Copy for EVENT_HEADER_EXTENDED_DATA_ITEM_0 {}
impl ::core::clone::Clone for EVENT_HEADER_EXTENDED_DATA_ITEM_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EVENT_HEADER_EXTENDED_DATA_ITEM_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EVENT_HEADER_EXTENDED_DATA_ITEM_0").field("_bitfield", &self._bitfield).finish()
    }
}
unsafe impl ::windows::core::Abi for EVENT_HEADER_EXTENDED_DATA_ITEM_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EVENT_HEADER_EXTENDED_DATA_ITEM_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EVENT_HEADER_EXTENDED_DATA_ITEM_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for EVENT_HEADER_EXTENDED_DATA_ITEM_0 {}
impl ::core::default::Default for EVENT_HEADER_EXTENDED_DATA_ITEM_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_HEADER_EXT_TYPE_CONTAINER_ID: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_HEADER_EXT_TYPE_CONTROL_GUID: u32 = 14u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_HEADER_EXT_TYPE_EVENT_KEY: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_HEADER_EXT_TYPE_EVENT_SCHEMA_TL: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_HEADER_EXT_TYPE_INSTANCE_INFO: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_HEADER_EXT_TYPE_MAX: u32 = 19u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_HEADER_EXT_TYPE_PEBS_INDEX: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_HEADER_EXT_TYPE_PMC_COUNTERS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_HEADER_EXT_TYPE_PROCESS_START_KEY: u32 = 13u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_HEADER_EXT_TYPE_PROV_TRAITS: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_HEADER_EXT_TYPE_PSM_KEY: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_HEADER_EXT_TYPE_QPC_DELTA: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_HEADER_EXT_TYPE_RELATED_ACTIVITYID: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_HEADER_EXT_TYPE_SID: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_HEADER_EXT_TYPE_STACK_KEY32: u32 = 17u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_HEADER_EXT_TYPE_STACK_KEY64: u32 = 18u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_HEADER_EXT_TYPE_STACK_TRACE32: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_HEADER_EXT_TYPE_STACK_TRACE64: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_HEADER_EXT_TYPE_TS_ID: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_HEADER_FLAG_32_BIT_HEADER: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_HEADER_FLAG_64_BIT_HEADER: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_HEADER_FLAG_CLASSIC_HEADER: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_HEADER_FLAG_DECODE_GUID: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_HEADER_FLAG_EXTENDED_INFO: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_HEADER_FLAG_NO_CPUTIME: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_HEADER_FLAG_PRIVATE_SESSION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_HEADER_FLAG_PROCESSOR_INDEX: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_HEADER_FLAG_STRING_ONLY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_HEADER_FLAG_TRACE_MESSAGE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_HEADER_PROPERTY_FORWARDED_XML: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_HEADER_PROPERTY_LEGACY_EVENTLOG: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_HEADER_PROPERTY_RELOGGABLE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_HEADER_PROPERTY_XML: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EVENT_INFO_CLASS(pub i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EventProviderBinaryTrackInfo: EVENT_INFO_CLASS = EVENT_INFO_CLASS(0i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EventProviderSetReserved1: EVENT_INFO_CLASS = EVENT_INFO_CLASS(1i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EventProviderSetTraits: EVENT_INFO_CLASS = EVENT_INFO_CLASS(2i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EventProviderUseDescriptorType: EVENT_INFO_CLASS = EVENT_INFO_CLASS(3i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const MaxEventInfo: EVENT_INFO_CLASS = EVENT_INFO_CLASS(4i32);
impl ::core::marker::Copy for EVENT_INFO_CLASS {}
impl ::core::clone::Clone for EVENT_INFO_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EVENT_INFO_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for EVENT_INFO_CLASS {
    type Abi = Self;
}
impl ::core::fmt::Debug for EVENT_INFO_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EVENT_INFO_CLASS").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
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
impl ::core::marker::Copy for EVENT_INSTANCE_HEADER {}
impl ::core::clone::Clone for EVENT_INSTANCE_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for EVENT_INSTANCE_HEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EVENT_INSTANCE_HEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EVENT_INSTANCE_HEADER>()) == 0 }
    }
}
impl ::core::cmp::Eq for EVENT_INSTANCE_HEADER {}
impl ::core::default::Default for EVENT_INSTANCE_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub union EVENT_INSTANCE_HEADER_0 {
    pub FieldTypeFlags: u16,
    pub Anonymous: EVENT_INSTANCE_HEADER_0_0,
}
impl ::core::marker::Copy for EVENT_INSTANCE_HEADER_0 {}
impl ::core::clone::Clone for EVENT_INSTANCE_HEADER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for EVENT_INSTANCE_HEADER_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EVENT_INSTANCE_HEADER_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EVENT_INSTANCE_HEADER_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for EVENT_INSTANCE_HEADER_0 {}
impl ::core::default::Default for EVENT_INSTANCE_HEADER_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct EVENT_INSTANCE_HEADER_0_0 {
    pub HeaderType: u8,
    pub MarkerFlags: u8,
}
impl ::core::marker::Copy for EVENT_INSTANCE_HEADER_0_0 {}
impl ::core::clone::Clone for EVENT_INSTANCE_HEADER_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EVENT_INSTANCE_HEADER_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EVENT_INSTANCE_HEADER_0_0").field("HeaderType", &self.HeaderType).field("MarkerFlags", &self.MarkerFlags).finish()
    }
}
unsafe impl ::windows::core::Abi for EVENT_INSTANCE_HEADER_0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EVENT_INSTANCE_HEADER_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EVENT_INSTANCE_HEADER_0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for EVENT_INSTANCE_HEADER_0_0 {}
impl ::core::default::Default for EVENT_INSTANCE_HEADER_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub union EVENT_INSTANCE_HEADER_1 {
    pub Version: u32,
    pub Class: EVENT_INSTANCE_HEADER_1_0,
}
impl ::core::marker::Copy for EVENT_INSTANCE_HEADER_1 {}
impl ::core::clone::Clone for EVENT_INSTANCE_HEADER_1 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for EVENT_INSTANCE_HEADER_1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EVENT_INSTANCE_HEADER_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EVENT_INSTANCE_HEADER_1>()) == 0 }
    }
}
impl ::core::cmp::Eq for EVENT_INSTANCE_HEADER_1 {}
impl ::core::default::Default for EVENT_INSTANCE_HEADER_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct EVENT_INSTANCE_HEADER_1_0 {
    pub Type: u8,
    pub Level: u8,
    pub Version: u16,
}
impl ::core::marker::Copy for EVENT_INSTANCE_HEADER_1_0 {}
impl ::core::clone::Clone for EVENT_INSTANCE_HEADER_1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EVENT_INSTANCE_HEADER_1_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EVENT_INSTANCE_HEADER_1_0").field("Type", &self.Type).field("Level", &self.Level).field("Version", &self.Version).finish()
    }
}
unsafe impl ::windows::core::Abi for EVENT_INSTANCE_HEADER_1_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EVENT_INSTANCE_HEADER_1_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EVENT_INSTANCE_HEADER_1_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for EVENT_INSTANCE_HEADER_1_0 {}
impl ::core::default::Default for EVENT_INSTANCE_HEADER_1_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub union EVENT_INSTANCE_HEADER_2 {
    pub Anonymous1: EVENT_INSTANCE_HEADER_2_0,
    pub ProcessorTime: u64,
    pub Anonymous2: EVENT_INSTANCE_HEADER_2_1,
}
impl ::core::marker::Copy for EVENT_INSTANCE_HEADER_2 {}
impl ::core::clone::Clone for EVENT_INSTANCE_HEADER_2 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for EVENT_INSTANCE_HEADER_2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EVENT_INSTANCE_HEADER_2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EVENT_INSTANCE_HEADER_2>()) == 0 }
    }
}
impl ::core::cmp::Eq for EVENT_INSTANCE_HEADER_2 {}
impl ::core::default::Default for EVENT_INSTANCE_HEADER_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct EVENT_INSTANCE_HEADER_2_0 {
    pub KernelTime: u32,
    pub UserTime: u32,
}
impl ::core::marker::Copy for EVENT_INSTANCE_HEADER_2_0 {}
impl ::core::clone::Clone for EVENT_INSTANCE_HEADER_2_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EVENT_INSTANCE_HEADER_2_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EVENT_INSTANCE_HEADER_2_0").field("KernelTime", &self.KernelTime).field("UserTime", &self.UserTime).finish()
    }
}
unsafe impl ::windows::core::Abi for EVENT_INSTANCE_HEADER_2_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EVENT_INSTANCE_HEADER_2_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EVENT_INSTANCE_HEADER_2_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for EVENT_INSTANCE_HEADER_2_0 {}
impl ::core::default::Default for EVENT_INSTANCE_HEADER_2_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct EVENT_INSTANCE_HEADER_2_1 {
    pub EventId: u32,
    pub Flags: u32,
}
impl ::core::marker::Copy for EVENT_INSTANCE_HEADER_2_1 {}
impl ::core::clone::Clone for EVENT_INSTANCE_HEADER_2_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EVENT_INSTANCE_HEADER_2_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EVENT_INSTANCE_HEADER_2_1").field("EventId", &self.EventId).field("Flags", &self.Flags).finish()
    }
}
unsafe impl ::windows::core::Abi for EVENT_INSTANCE_HEADER_2_1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EVENT_INSTANCE_HEADER_2_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EVENT_INSTANCE_HEADER_2_1>()) == 0 }
    }
}
impl ::core::cmp::Eq for EVENT_INSTANCE_HEADER_2_1 {}
impl ::core::default::Default for EVENT_INSTANCE_HEADER_2_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EVENT_INSTANCE_INFO {
    pub RegHandle: super::super::super::Foundation::HANDLE,
    pub InstanceId: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EVENT_INSTANCE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EVENT_INSTANCE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EVENT_INSTANCE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EVENT_INSTANCE_INFO").field("RegHandle", &self.RegHandle).field("InstanceId", &self.InstanceId).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for EVENT_INSTANCE_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EVENT_INSTANCE_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EVENT_INSTANCE_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EVENT_INSTANCE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EVENT_INSTANCE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_LOGGER_NAME: &str = "EventLog";
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_LOGGER_NAMEA: &str = "EventLog";
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_LOGGER_NAMEW: &str = "EventLog";
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct EVENT_MAP_ENTRY {
    pub OutputOffset: u32,
    pub Anonymous: EVENT_MAP_ENTRY_0,
}
impl ::core::marker::Copy for EVENT_MAP_ENTRY {}
impl ::core::clone::Clone for EVENT_MAP_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for EVENT_MAP_ENTRY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EVENT_MAP_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EVENT_MAP_ENTRY>()) == 0 }
    }
}
impl ::core::cmp::Eq for EVENT_MAP_ENTRY {}
impl ::core::default::Default for EVENT_MAP_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub union EVENT_MAP_ENTRY_0 {
    pub Value: u32,
    pub InputOffset: u32,
}
impl ::core::marker::Copy for EVENT_MAP_ENTRY_0 {}
impl ::core::clone::Clone for EVENT_MAP_ENTRY_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for EVENT_MAP_ENTRY_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EVENT_MAP_ENTRY_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EVENT_MAP_ENTRY_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for EVENT_MAP_ENTRY_0 {}
impl ::core::default::Default for EVENT_MAP_ENTRY_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct EVENT_MAP_INFO {
    pub NameOffset: u32,
    pub Flag: MAP_FLAGS,
    pub EntryCount: u32,
    pub Anonymous: EVENT_MAP_INFO_0,
    pub MapEntryArray: [EVENT_MAP_ENTRY; 1],
}
impl ::core::marker::Copy for EVENT_MAP_INFO {}
impl ::core::clone::Clone for EVENT_MAP_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for EVENT_MAP_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EVENT_MAP_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EVENT_MAP_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for EVENT_MAP_INFO {}
impl ::core::default::Default for EVENT_MAP_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub union EVENT_MAP_INFO_0 {
    pub MapEntryValueType: MAP_VALUETYPE,
    pub FormatStringOffset: u32,
}
impl ::core::marker::Copy for EVENT_MAP_INFO_0 {}
impl ::core::clone::Clone for EVENT_MAP_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for EVENT_MAP_INFO_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EVENT_MAP_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EVENT_MAP_INFO_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for EVENT_MAP_INFO_0 {}
impl ::core::default::Default for EVENT_MAP_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_MAX_LEVEL: u32 = 255u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_MIN_LEVEL: u32 = 0u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct EVENT_PROPERTY_INFO {
    pub Flags: PROPERTY_FLAGS,
    pub NameOffset: u32,
    pub Anonymous1: EVENT_PROPERTY_INFO_0,
    pub Anonymous2: EVENT_PROPERTY_INFO_1,
    pub Anonymous3: EVENT_PROPERTY_INFO_2,
    pub Anonymous4: EVENT_PROPERTY_INFO_3,
}
impl ::core::marker::Copy for EVENT_PROPERTY_INFO {}
impl ::core::clone::Clone for EVENT_PROPERTY_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for EVENT_PROPERTY_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EVENT_PROPERTY_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EVENT_PROPERTY_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for EVENT_PROPERTY_INFO {}
impl ::core::default::Default for EVENT_PROPERTY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub union EVENT_PROPERTY_INFO_0 {
    pub nonStructType: EVENT_PROPERTY_INFO_0_1,
    pub structType: EVENT_PROPERTY_INFO_0_2,
    pub customSchemaType: EVENT_PROPERTY_INFO_0_0,
}
impl ::core::marker::Copy for EVENT_PROPERTY_INFO_0 {}
impl ::core::clone::Clone for EVENT_PROPERTY_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for EVENT_PROPERTY_INFO_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EVENT_PROPERTY_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EVENT_PROPERTY_INFO_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for EVENT_PROPERTY_INFO_0 {}
impl ::core::default::Default for EVENT_PROPERTY_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct EVENT_PROPERTY_INFO_0_0 {
    pub InType: u16,
    pub OutType: u16,
    pub CustomSchemaOffset: u32,
}
impl ::core::marker::Copy for EVENT_PROPERTY_INFO_0_0 {}
impl ::core::clone::Clone for EVENT_PROPERTY_INFO_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EVENT_PROPERTY_INFO_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EVENT_PROPERTY_INFO_0_0").field("InType", &self.InType).field("OutType", &self.OutType).field("CustomSchemaOffset", &self.CustomSchemaOffset).finish()
    }
}
unsafe impl ::windows::core::Abi for EVENT_PROPERTY_INFO_0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EVENT_PROPERTY_INFO_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EVENT_PROPERTY_INFO_0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for EVENT_PROPERTY_INFO_0_0 {}
impl ::core::default::Default for EVENT_PROPERTY_INFO_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct EVENT_PROPERTY_INFO_0_1 {
    pub InType: u16,
    pub OutType: u16,
    pub MapNameOffset: u32,
}
impl ::core::marker::Copy for EVENT_PROPERTY_INFO_0_1 {}
impl ::core::clone::Clone for EVENT_PROPERTY_INFO_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EVENT_PROPERTY_INFO_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EVENT_PROPERTY_INFO_0_1").field("InType", &self.InType).field("OutType", &self.OutType).field("MapNameOffset", &self.MapNameOffset).finish()
    }
}
unsafe impl ::windows::core::Abi for EVENT_PROPERTY_INFO_0_1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EVENT_PROPERTY_INFO_0_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EVENT_PROPERTY_INFO_0_1>()) == 0 }
    }
}
impl ::core::cmp::Eq for EVENT_PROPERTY_INFO_0_1 {}
impl ::core::default::Default for EVENT_PROPERTY_INFO_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct EVENT_PROPERTY_INFO_0_2 {
    pub StructStartIndex: u16,
    pub NumOfStructMembers: u16,
    pub padding: u32,
}
impl ::core::marker::Copy for EVENT_PROPERTY_INFO_0_2 {}
impl ::core::clone::Clone for EVENT_PROPERTY_INFO_0_2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EVENT_PROPERTY_INFO_0_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EVENT_PROPERTY_INFO_0_2").field("StructStartIndex", &self.StructStartIndex).field("NumOfStructMembers", &self.NumOfStructMembers).field("padding", &self.padding).finish()
    }
}
unsafe impl ::windows::core::Abi for EVENT_PROPERTY_INFO_0_2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EVENT_PROPERTY_INFO_0_2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EVENT_PROPERTY_INFO_0_2>()) == 0 }
    }
}
impl ::core::cmp::Eq for EVENT_PROPERTY_INFO_0_2 {}
impl ::core::default::Default for EVENT_PROPERTY_INFO_0_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub union EVENT_PROPERTY_INFO_1 {
    pub count: u16,
    pub countPropertyIndex: u16,
}
impl ::core::marker::Copy for EVENT_PROPERTY_INFO_1 {}
impl ::core::clone::Clone for EVENT_PROPERTY_INFO_1 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for EVENT_PROPERTY_INFO_1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EVENT_PROPERTY_INFO_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EVENT_PROPERTY_INFO_1>()) == 0 }
    }
}
impl ::core::cmp::Eq for EVENT_PROPERTY_INFO_1 {}
impl ::core::default::Default for EVENT_PROPERTY_INFO_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub union EVENT_PROPERTY_INFO_2 {
    pub length: u16,
    pub lengthPropertyIndex: u16,
}
impl ::core::marker::Copy for EVENT_PROPERTY_INFO_2 {}
impl ::core::clone::Clone for EVENT_PROPERTY_INFO_2 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for EVENT_PROPERTY_INFO_2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EVENT_PROPERTY_INFO_2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EVENT_PROPERTY_INFO_2>()) == 0 }
    }
}
impl ::core::cmp::Eq for EVENT_PROPERTY_INFO_2 {}
impl ::core::default::Default for EVENT_PROPERTY_INFO_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub union EVENT_PROPERTY_INFO_3 {
    pub Reserved: u32,
    pub Anonymous: EVENT_PROPERTY_INFO_3_0,
}
impl ::core::marker::Copy for EVENT_PROPERTY_INFO_3 {}
impl ::core::clone::Clone for EVENT_PROPERTY_INFO_3 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for EVENT_PROPERTY_INFO_3 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EVENT_PROPERTY_INFO_3 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EVENT_PROPERTY_INFO_3>()) == 0 }
    }
}
impl ::core::cmp::Eq for EVENT_PROPERTY_INFO_3 {}
impl ::core::default::Default for EVENT_PROPERTY_INFO_3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct EVENT_PROPERTY_INFO_3_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for EVENT_PROPERTY_INFO_3_0 {}
impl ::core::clone::Clone for EVENT_PROPERTY_INFO_3_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EVENT_PROPERTY_INFO_3_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EVENT_PROPERTY_INFO_3_0").field("_bitfield", &self._bitfield).finish()
    }
}
unsafe impl ::windows::core::Abi for EVENT_PROPERTY_INFO_3_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EVENT_PROPERTY_INFO_3_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EVENT_PROPERTY_INFO_3_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for EVENT_PROPERTY_INFO_3_0 {}
impl ::core::default::Default for EVENT_PROPERTY_INFO_3_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct EVENT_RECORD {
    pub EventHeader: EVENT_HEADER,
    pub BufferContext: ETW_BUFFER_CONTEXT,
    pub ExtendedDataCount: u16,
    pub UserDataLength: u16,
    pub ExtendedData: *mut EVENT_HEADER_EXTENDED_DATA_ITEM,
    pub UserData: *mut ::core::ffi::c_void,
    pub UserContext: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for EVENT_RECORD {}
impl ::core::clone::Clone for EVENT_RECORD {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for EVENT_RECORD {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EVENT_RECORD {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EVENT_RECORD>()) == 0 }
    }
}
impl ::core::cmp::Eq for EVENT_RECORD {}
impl ::core::default::Default for EVENT_RECORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct EVENT_TRACE {
    pub Header: EVENT_TRACE_HEADER,
    pub InstanceId: u32,
    pub ParentInstanceId: u32,
    pub ParentGuid: ::windows::core::GUID,
    pub MofData: *mut ::core::ffi::c_void,
    pub MofLength: u32,
    pub Anonymous: EVENT_TRACE_0,
}
impl ::core::marker::Copy for EVENT_TRACE {}
impl ::core::clone::Clone for EVENT_TRACE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for EVENT_TRACE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EVENT_TRACE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EVENT_TRACE>()) == 0 }
    }
}
impl ::core::cmp::Eq for EVENT_TRACE {}
impl ::core::default::Default for EVENT_TRACE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub union EVENT_TRACE_0 {
    pub ClientContext: u32,
    pub BufferContext: ETW_BUFFER_CONTEXT,
}
impl ::core::marker::Copy for EVENT_TRACE_0 {}
impl ::core::clone::Clone for EVENT_TRACE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for EVENT_TRACE_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EVENT_TRACE_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EVENT_TRACE_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for EVENT_TRACE_0 {}
impl ::core::default::Default for EVENT_TRACE_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_ADDTO_TRIAGE_DUMP: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_ADD_HEADER_MODE: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_BUFFERING_MODE: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_COMPRESSED_MODE: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EVENT_TRACE_CONTROL(pub u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_CONTROL_FLUSH: EVENT_TRACE_CONTROL = EVENT_TRACE_CONTROL(3u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_CONTROL_QUERY: EVENT_TRACE_CONTROL = EVENT_TRACE_CONTROL(0u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_CONTROL_STOP: EVENT_TRACE_CONTROL = EVENT_TRACE_CONTROL(1u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_CONTROL_UPDATE: EVENT_TRACE_CONTROL = EVENT_TRACE_CONTROL(2u32);
impl ::core::marker::Copy for EVENT_TRACE_CONTROL {}
impl ::core::clone::Clone for EVENT_TRACE_CONTROL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EVENT_TRACE_CONTROL {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for EVENT_TRACE_CONTROL {
    type Abi = Self;
}
impl ::core::fmt::Debug for EVENT_TRACE_CONTROL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EVENT_TRACE_CONTROL").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_CONTROL_CONVERT_TO_REALTIME: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_CONTROL_INCREMENT_FILE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_DELAY_OPEN_FILE_MODE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_FILE_MODE_APPEND: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_FILE_MODE_CIRCULAR: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_FILE_MODE_NEWFILE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_FILE_MODE_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_FILE_MODE_PREALLOCATE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_FILE_MODE_SEQUENTIAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EVENT_TRACE_FLAG(pub u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_FLAG_ALPC: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(1048576u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_FLAG_CSWITCH: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(16u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_FLAG_DBGPRINT: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(262144u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_FLAG_DISK_FILE_IO: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(512u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_FLAG_DISK_IO: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(256u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_FLAG_DISK_IO_INIT: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(1024u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_FLAG_DISPATCHER: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(2048u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_FLAG_DPC: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(32u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_FLAG_DRIVER: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(8388608u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_FLAG_FILE_IO: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(33554432u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_FLAG_FILE_IO_INIT: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(67108864u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_FLAG_IMAGE_LOAD: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(4u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_FLAG_INTERRUPT: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(64u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_FLAG_JOB: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(524288u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_FLAG_MEMORY_HARD_FAULTS: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(8192u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_FLAG_MEMORY_PAGE_FAULTS: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(4096u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_FLAG_NETWORK_TCPIP: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(65536u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_FLAG_NO_SYSCONFIG: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(268435456u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_FLAG_PROCESS: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(1u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_FLAG_PROCESS_COUNTERS: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(8u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_FLAG_PROFILE: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(16777216u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_FLAG_REGISTRY: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(131072u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_FLAG_SPLIT_IO: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(2097152u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_FLAG_SYSTEMCALL: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(128u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_FLAG_THREAD: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(2u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_FLAG_VAMAP: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(32768u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_FLAG_VIRTUAL_ALLOC: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(16384u32);
impl ::core::marker::Copy for EVENT_TRACE_FLAG {}
impl ::core::clone::Clone for EVENT_TRACE_FLAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EVENT_TRACE_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for EVENT_TRACE_FLAG {
    type Abi = Self;
}
impl ::core::fmt::Debug for EVENT_TRACE_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EVENT_TRACE_FLAG").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for EVENT_TRACE_FLAG {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for EVENT_TRACE_FLAG {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for EVENT_TRACE_FLAG {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for EVENT_TRACE_FLAG {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for EVENT_TRACE_FLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_FLAG_DEBUG_EVENTS: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_FLAG_ENABLE_RESERVE: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_FLAG_EXTENSION: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_FLAG_FORWARD_WMI: u32 = 1073741824u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
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
impl ::core::marker::Copy for EVENT_TRACE_HEADER {}
impl ::core::clone::Clone for EVENT_TRACE_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for EVENT_TRACE_HEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EVENT_TRACE_HEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EVENT_TRACE_HEADER>()) == 0 }
    }
}
impl ::core::cmp::Eq for EVENT_TRACE_HEADER {}
impl ::core::default::Default for EVENT_TRACE_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub union EVENT_TRACE_HEADER_0 {
    pub FieldTypeFlags: u16,
    pub Anonymous: EVENT_TRACE_HEADER_0_0,
}
impl ::core::marker::Copy for EVENT_TRACE_HEADER_0 {}
impl ::core::clone::Clone for EVENT_TRACE_HEADER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for EVENT_TRACE_HEADER_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EVENT_TRACE_HEADER_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EVENT_TRACE_HEADER_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for EVENT_TRACE_HEADER_0 {}
impl ::core::default::Default for EVENT_TRACE_HEADER_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct EVENT_TRACE_HEADER_0_0 {
    pub HeaderType: u8,
    pub MarkerFlags: u8,
}
impl ::core::marker::Copy for EVENT_TRACE_HEADER_0_0 {}
impl ::core::clone::Clone for EVENT_TRACE_HEADER_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EVENT_TRACE_HEADER_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EVENT_TRACE_HEADER_0_0").field("HeaderType", &self.HeaderType).field("MarkerFlags", &self.MarkerFlags).finish()
    }
}
unsafe impl ::windows::core::Abi for EVENT_TRACE_HEADER_0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EVENT_TRACE_HEADER_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EVENT_TRACE_HEADER_0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for EVENT_TRACE_HEADER_0_0 {}
impl ::core::default::Default for EVENT_TRACE_HEADER_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub union EVENT_TRACE_HEADER_1 {
    pub Version: u32,
    pub Class: EVENT_TRACE_HEADER_1_0,
}
impl ::core::marker::Copy for EVENT_TRACE_HEADER_1 {}
impl ::core::clone::Clone for EVENT_TRACE_HEADER_1 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for EVENT_TRACE_HEADER_1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EVENT_TRACE_HEADER_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EVENT_TRACE_HEADER_1>()) == 0 }
    }
}
impl ::core::cmp::Eq for EVENT_TRACE_HEADER_1 {}
impl ::core::default::Default for EVENT_TRACE_HEADER_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct EVENT_TRACE_HEADER_1_0 {
    pub Type: u8,
    pub Level: u8,
    pub Version: u16,
}
impl ::core::marker::Copy for EVENT_TRACE_HEADER_1_0 {}
impl ::core::clone::Clone for EVENT_TRACE_HEADER_1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EVENT_TRACE_HEADER_1_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EVENT_TRACE_HEADER_1_0").field("Type", &self.Type).field("Level", &self.Level).field("Version", &self.Version).finish()
    }
}
unsafe impl ::windows::core::Abi for EVENT_TRACE_HEADER_1_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EVENT_TRACE_HEADER_1_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EVENT_TRACE_HEADER_1_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for EVENT_TRACE_HEADER_1_0 {}
impl ::core::default::Default for EVENT_TRACE_HEADER_1_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub union EVENT_TRACE_HEADER_2 {
    pub Guid: ::windows::core::GUID,
    pub GuidPtr: u64,
}
impl ::core::marker::Copy for EVENT_TRACE_HEADER_2 {}
impl ::core::clone::Clone for EVENT_TRACE_HEADER_2 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for EVENT_TRACE_HEADER_2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EVENT_TRACE_HEADER_2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EVENT_TRACE_HEADER_2>()) == 0 }
    }
}
impl ::core::cmp::Eq for EVENT_TRACE_HEADER_2 {}
impl ::core::default::Default for EVENT_TRACE_HEADER_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub union EVENT_TRACE_HEADER_3 {
    pub Anonymous1: EVENT_TRACE_HEADER_3_0,
    pub ProcessorTime: u64,
    pub Anonymous2: EVENT_TRACE_HEADER_3_1,
}
impl ::core::marker::Copy for EVENT_TRACE_HEADER_3 {}
impl ::core::clone::Clone for EVENT_TRACE_HEADER_3 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for EVENT_TRACE_HEADER_3 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EVENT_TRACE_HEADER_3 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EVENT_TRACE_HEADER_3>()) == 0 }
    }
}
impl ::core::cmp::Eq for EVENT_TRACE_HEADER_3 {}
impl ::core::default::Default for EVENT_TRACE_HEADER_3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct EVENT_TRACE_HEADER_3_0 {
    pub KernelTime: u32,
    pub UserTime: u32,
}
impl ::core::marker::Copy for EVENT_TRACE_HEADER_3_0 {}
impl ::core::clone::Clone for EVENT_TRACE_HEADER_3_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EVENT_TRACE_HEADER_3_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EVENT_TRACE_HEADER_3_0").field("KernelTime", &self.KernelTime).field("UserTime", &self.UserTime).finish()
    }
}
unsafe impl ::windows::core::Abi for EVENT_TRACE_HEADER_3_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EVENT_TRACE_HEADER_3_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EVENT_TRACE_HEADER_3_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for EVENT_TRACE_HEADER_3_0 {}
impl ::core::default::Default for EVENT_TRACE_HEADER_3_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct EVENT_TRACE_HEADER_3_1 {
    pub ClientContext: u32,
    pub Flags: u32,
}
impl ::core::marker::Copy for EVENT_TRACE_HEADER_3_1 {}
impl ::core::clone::Clone for EVENT_TRACE_HEADER_3_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EVENT_TRACE_HEADER_3_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EVENT_TRACE_HEADER_3_1").field("ClientContext", &self.ClientContext).field("Flags", &self.Flags).finish()
    }
}
unsafe impl ::windows::core::Abi for EVENT_TRACE_HEADER_3_1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EVENT_TRACE_HEADER_3_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EVENT_TRACE_HEADER_3_1>()) == 0 }
    }
}
impl ::core::cmp::Eq for EVENT_TRACE_HEADER_3_1 {}
impl ::core::default::Default for EVENT_TRACE_HEADER_3_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_INDEPENDENT_SESSION_MODE: u32 = 134217728u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`, `\"Win32_System_Time\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
pub struct EVENT_TRACE_LOGFILEA {
    pub LogFileName: ::windows::core::PSTR,
    pub LoggerName: ::windows::core::PSTR,
    pub CurrentTime: i64,
    pub BuffersRead: u32,
    pub Anonymous1: EVENT_TRACE_LOGFILEA_0,
    pub CurrentEvent: EVENT_TRACE,
    pub LogfileHeader: TRACE_LOGFILE_HEADER,
    pub BufferCallback: PEVENT_TRACE_BUFFER_CALLBACKA,
    pub BufferSize: u32,
    pub Filled: u32,
    pub EventsLost: u32,
    pub Anonymous2: EVENT_TRACE_LOGFILEA_1,
    pub IsKernelTrace: u32,
    pub Context: *mut ::core::ffi::c_void,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::marker::Copy for EVENT_TRACE_LOGFILEA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::clone::Clone for EVENT_TRACE_LOGFILEA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
unsafe impl ::windows::core::Abi for EVENT_TRACE_LOGFILEA {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::cmp::PartialEq for EVENT_TRACE_LOGFILEA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EVENT_TRACE_LOGFILEA>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::cmp::Eq for EVENT_TRACE_LOGFILEA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::default::Default for EVENT_TRACE_LOGFILEA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`, `\"Win32_System_Time\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
pub union EVENT_TRACE_LOGFILEA_0 {
    pub LogFileMode: u32,
    pub ProcessTraceMode: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::marker::Copy for EVENT_TRACE_LOGFILEA_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::clone::Clone for EVENT_TRACE_LOGFILEA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
unsafe impl ::windows::core::Abi for EVENT_TRACE_LOGFILEA_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::cmp::PartialEq for EVENT_TRACE_LOGFILEA_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EVENT_TRACE_LOGFILEA_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::cmp::Eq for EVENT_TRACE_LOGFILEA_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::default::Default for EVENT_TRACE_LOGFILEA_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`, `\"Win32_System_Time\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
pub union EVENT_TRACE_LOGFILEA_1 {
    pub EventCallback: PEVENT_CALLBACK,
    pub EventRecordCallback: PEVENT_RECORD_CALLBACK,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::marker::Copy for EVENT_TRACE_LOGFILEA_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::clone::Clone for EVENT_TRACE_LOGFILEA_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
unsafe impl ::windows::core::Abi for EVENT_TRACE_LOGFILEA_1 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::cmp::PartialEq for EVENT_TRACE_LOGFILEA_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EVENT_TRACE_LOGFILEA_1>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::cmp::Eq for EVENT_TRACE_LOGFILEA_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::default::Default for EVENT_TRACE_LOGFILEA_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`, `\"Win32_System_Time\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
pub struct EVENT_TRACE_LOGFILEW {
    pub LogFileName: ::windows::core::PWSTR,
    pub LoggerName: ::windows::core::PWSTR,
    pub CurrentTime: i64,
    pub BuffersRead: u32,
    pub Anonymous1: EVENT_TRACE_LOGFILEW_0,
    pub CurrentEvent: EVENT_TRACE,
    pub LogfileHeader: TRACE_LOGFILE_HEADER,
    pub BufferCallback: PEVENT_TRACE_BUFFER_CALLBACKW,
    pub BufferSize: u32,
    pub Filled: u32,
    pub EventsLost: u32,
    pub Anonymous2: EVENT_TRACE_LOGFILEW_1,
    pub IsKernelTrace: u32,
    pub Context: *mut ::core::ffi::c_void,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::marker::Copy for EVENT_TRACE_LOGFILEW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::clone::Clone for EVENT_TRACE_LOGFILEW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
unsafe impl ::windows::core::Abi for EVENT_TRACE_LOGFILEW {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::cmp::PartialEq for EVENT_TRACE_LOGFILEW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EVENT_TRACE_LOGFILEW>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::cmp::Eq for EVENT_TRACE_LOGFILEW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::default::Default for EVENT_TRACE_LOGFILEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`, `\"Win32_System_Time\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
pub union EVENT_TRACE_LOGFILEW_0 {
    pub LogFileMode: u32,
    pub ProcessTraceMode: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::marker::Copy for EVENT_TRACE_LOGFILEW_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::clone::Clone for EVENT_TRACE_LOGFILEW_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
unsafe impl ::windows::core::Abi for EVENT_TRACE_LOGFILEW_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::cmp::PartialEq for EVENT_TRACE_LOGFILEW_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EVENT_TRACE_LOGFILEW_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::cmp::Eq for EVENT_TRACE_LOGFILEW_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::default::Default for EVENT_TRACE_LOGFILEW_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`, `\"Win32_System_Time\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
pub union EVENT_TRACE_LOGFILEW_1 {
    pub EventCallback: PEVENT_CALLBACK,
    pub EventRecordCallback: PEVENT_RECORD_CALLBACK,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::marker::Copy for EVENT_TRACE_LOGFILEW_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::clone::Clone for EVENT_TRACE_LOGFILEW_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
unsafe impl ::windows::core::Abi for EVENT_TRACE_LOGFILEW_1 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::cmp::PartialEq for EVENT_TRACE_LOGFILEW_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EVENT_TRACE_LOGFILEW_1>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::cmp::Eq for EVENT_TRACE_LOGFILEW_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::default::Default for EVENT_TRACE_LOGFILEW_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_MODE_RESERVED: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_NONSTOPPABLE_MODE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_NO_PER_PROCESSOR_BUFFERING: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_PERSIST_ON_HYBRID_SHUTDOWN: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_PRIVATE_IN_PROC: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_PRIVATE_LOGGER_MODE: u32 = 2048u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
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
impl ::core::marker::Copy for EVENT_TRACE_PROPERTIES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EVENT_TRACE_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for EVENT_TRACE_PROPERTIES {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EVENT_TRACE_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EVENT_TRACE_PROPERTIES>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EVENT_TRACE_PROPERTIES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EVENT_TRACE_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union EVENT_TRACE_PROPERTIES_0 {
    pub AgeLimit: i32,
    pub FlushThreshold: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EVENT_TRACE_PROPERTIES_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EVENT_TRACE_PROPERTIES_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for EVENT_TRACE_PROPERTIES_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EVENT_TRACE_PROPERTIES_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EVENT_TRACE_PROPERTIES_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EVENT_TRACE_PROPERTIES_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EVENT_TRACE_PROPERTIES_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
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
impl ::core::marker::Copy for EVENT_TRACE_PROPERTIES_V2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EVENT_TRACE_PROPERTIES_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for EVENT_TRACE_PROPERTIES_V2 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EVENT_TRACE_PROPERTIES_V2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EVENT_TRACE_PROPERTIES_V2>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EVENT_TRACE_PROPERTIES_V2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EVENT_TRACE_PROPERTIES_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union EVENT_TRACE_PROPERTIES_V2_0 {
    pub AgeLimit: i32,
    pub FlushThreshold: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EVENT_TRACE_PROPERTIES_V2_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EVENT_TRACE_PROPERTIES_V2_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for EVENT_TRACE_PROPERTIES_V2_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EVENT_TRACE_PROPERTIES_V2_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EVENT_TRACE_PROPERTIES_V2_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EVENT_TRACE_PROPERTIES_V2_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EVENT_TRACE_PROPERTIES_V2_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union EVENT_TRACE_PROPERTIES_V2_1 {
    pub Anonymous: EVENT_TRACE_PROPERTIES_V2_1_0,
    pub V2Control: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EVENT_TRACE_PROPERTIES_V2_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EVENT_TRACE_PROPERTIES_V2_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for EVENT_TRACE_PROPERTIES_V2_1 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EVENT_TRACE_PROPERTIES_V2_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EVENT_TRACE_PROPERTIES_V2_1>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EVENT_TRACE_PROPERTIES_V2_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EVENT_TRACE_PROPERTIES_V2_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EVENT_TRACE_PROPERTIES_V2_1_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EVENT_TRACE_PROPERTIES_V2_1_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EVENT_TRACE_PROPERTIES_V2_1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EVENT_TRACE_PROPERTIES_V2_1_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EVENT_TRACE_PROPERTIES_V2_1_0").field("_bitfield", &self._bitfield).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for EVENT_TRACE_PROPERTIES_V2_1_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EVENT_TRACE_PROPERTIES_V2_1_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EVENT_TRACE_PROPERTIES_V2_1_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EVENT_TRACE_PROPERTIES_V2_1_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EVENT_TRACE_PROPERTIES_V2_1_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union EVENT_TRACE_PROPERTIES_V2_2 {
    pub Anonymous: EVENT_TRACE_PROPERTIES_V2_2_0,
    pub V2Options: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EVENT_TRACE_PROPERTIES_V2_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EVENT_TRACE_PROPERTIES_V2_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for EVENT_TRACE_PROPERTIES_V2_2 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EVENT_TRACE_PROPERTIES_V2_2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EVENT_TRACE_PROPERTIES_V2_2>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EVENT_TRACE_PROPERTIES_V2_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EVENT_TRACE_PROPERTIES_V2_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EVENT_TRACE_PROPERTIES_V2_2_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EVENT_TRACE_PROPERTIES_V2_2_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EVENT_TRACE_PROPERTIES_V2_2_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EVENT_TRACE_PROPERTIES_V2_2_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EVENT_TRACE_PROPERTIES_V2_2_0").field("_bitfield", &self._bitfield).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for EVENT_TRACE_PROPERTIES_V2_2_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EVENT_TRACE_PROPERTIES_V2_2_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EVENT_TRACE_PROPERTIES_V2_2_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EVENT_TRACE_PROPERTIES_V2_2_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EVENT_TRACE_PROPERTIES_V2_2_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_REAL_TIME_MODE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_RELOG_MODE: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_SECURE_MODE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_STOP_ON_HYBRID_SHUTDOWN: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_SYSTEM_LOGGER_MODE: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_ACCEPT: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_ACKDUP: u32 = 22u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_ACKFULL: u32 = 20u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_ACKPART: u32 = 21u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_CHECKPOINT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_CONFIG: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_CONFIG_BOOT: u32 = 37u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_CONFIG_CI_INFO: u32 = 29u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_CONFIG_CPU: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_CONFIG_DEFRAG: u32 = 31u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_CONFIG_DEVICEFAMILY: u32 = 33u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_CONFIG_DPI: u32 = 28u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_CONFIG_FLIGHTID: u32 = 34u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_CONFIG_IDECHANNEL: u32 = 23u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_CONFIG_IRQ: u32 = 21u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_CONFIG_LOGICALDISK: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_CONFIG_MACHINEID: u32 = 30u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_CONFIG_MOBILEPLATFORM: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_CONFIG_NETINFO: u32 = 17u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_CONFIG_NIC: u32 = 13u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_CONFIG_NUMANODE: u32 = 24u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_CONFIG_OPTICALMEDIA: u32 = 18u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_CONFIG_PHYSICALDISK: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_CONFIG_PLATFORM: u32 = 25u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_CONFIG_PNP: u32 = 22u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_CONFIG_POWER: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_CONFIG_PROCESSOR: u32 = 35u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_CONFIG_PROCESSORGROUP: u32 = 26u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_CONFIG_PROCESSORNUMBER: u32 = 27u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_CONFIG_SERVICES: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_CONFIG_VIDEO: u32 = 14u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_CONFIG_VIRTUALIZATION: u32 = 36u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_CONNECT: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_CONNFAIL: u32 = 17u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_COPY_ARP: u32 = 19u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_COPY_TCP: u32 = 18u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_DBGID_RSDS: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_DC_END: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_DC_START: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_DEQUEUE: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_DISCONNECT: u32 = 13u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_END: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_EXTENSION: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_FLT_POSTOP_COMPLETION: u32 = 99u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_FLT_POSTOP_FAILURE: u32 = 101u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_FLT_POSTOP_INIT: u32 = 97u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_FLT_PREOP_COMPLETION: u32 = 98u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_FLT_PREOP_FAILURE: u32 = 100u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_FLT_PREOP_INIT: u32 = 96u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_GUIDMAP: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_INFO: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_IO_FLUSH: u32 = 14u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_IO_FLUSH_INIT: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_IO_READ: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_IO_READ_INIT: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_IO_REDIRECTED_INIT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_IO_WRITE: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_IO_WRITE_INIT: u32 = 13u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_LOAD: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_MM_AV: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_MM_COW: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_MM_DZF: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_MM_GPF: u32 = 13u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_MM_HPF: u32 = 14u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_MM_TF: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_OPTICAL_IO_FLUSH: u32 = 57u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_OPTICAL_IO_FLUSH_INIT: u32 = 60u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_OPTICAL_IO_READ: u32 = 55u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_OPTICAL_IO_READ_INIT: u32 = 58u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_OPTICAL_IO_WRITE: u32 = 56u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_OPTICAL_IO_WRITE_INIT: u32 = 59u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_RECEIVE: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_RECONNECT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_REGCLOSE: u32 = 27u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_REGCOMMIT: u32 = 30u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_REGCREATE: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_REGDELETE: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_REGDELETEVALUE: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_REGENUMERATEKEY: u32 = 17u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_REGENUMERATEVALUEKEY: u32 = 18u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_REGFLUSH: u32 = 21u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_REGKCBCREATE: u32 = 22u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_REGKCBDELETE: u32 = 23u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_REGKCBRUNDOWNBEGIN: u32 = 24u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_REGKCBRUNDOWNEND: u32 = 25u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_REGMOUNTHIVE: u32 = 33u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_REGOPEN: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_REGPREPARE: u32 = 31u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_REGQUERY: u32 = 13u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_REGQUERYMULTIPLEVALUE: u32 = 19u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_REGQUERYSECURITY: u32 = 29u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_REGQUERYVALUE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_REGROLLBACK: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_REGSETINFORMATION: u32 = 20u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_REGSETSECURITY: u32 = 28u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_REGSETVALUE: u32 = 14u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_REGVIRTUALIZE: u32 = 26u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_REPLY: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_RESUME: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_RETRANSMIT: u32 = 14u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_SECURITY: u32 = 13u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_SEND: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_SIDINFO: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_START: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_STOP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_SUSPEND: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_TERMINATE: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_WINEVT_RECEIVE: u32 = 240u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_TYPE_WINEVT_SEND: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_USE_GLOBAL_SEQUENCE: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_USE_KBYTES_FOR_SIZE: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_USE_LOCAL_SEQUENCE: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_USE_NOCPUTIME: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_USE_PAGED_MEMORY: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_TRACE_USE_PROCTIME: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_WRITE_FLAG_INPRIVATE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENT_WRITE_FLAG_NO_FAULTING: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
#[inline]
pub unsafe fn EnableTrace(enable: u32, enableflag: u32, enablelevel: u32, controlguid: *const ::windows::core::GUID, tracehandle: u64) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EnableTrace(enable: u32, enableflag: u32, enablelevel: u32, controlguid: *const ::windows::core::GUID, tracehandle: u64) -> u32;
    }
    EnableTrace(enable, enableflag, enablelevel, ::core::mem::transmute(controlguid), tracehandle)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
#[inline]
pub unsafe fn EnableTraceEx(providerid: *const ::windows::core::GUID, sourceid: *const ::windows::core::GUID, tracehandle: u64, isenabled: u32, level: u8, matchanykeyword: u64, matchallkeyword: u64, enableproperty: u32, enablefilterdesc: *const EVENT_FILTER_DESCRIPTOR) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EnableTraceEx(providerid: *const ::windows::core::GUID, sourceid: *const ::windows::core::GUID, tracehandle: u64, isenabled: u32, level: u8, matchanykeyword: u64, matchallkeyword: u64, enableproperty: u32, enablefilterdesc: *const EVENT_FILTER_DESCRIPTOR) -> u32;
    }
    EnableTraceEx(::core::mem::transmute(providerid), ::core::mem::transmute(sourceid), tracehandle, isenabled, level, matchanykeyword, matchallkeyword, enableproperty, ::core::mem::transmute(enablefilterdesc))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
#[inline]
pub unsafe fn EnableTraceEx2(tracehandle: u64, providerid: *const ::windows::core::GUID, controlcode: u32, level: u8, matchanykeyword: u64, matchallkeyword: u64, timeout: u32, enableparameters: *const ENABLE_TRACE_PARAMETERS) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EnableTraceEx2(tracehandle: u64, providerid: *const ::windows::core::GUID, controlcode: u32, level: u8, matchanykeyword: u64, matchallkeyword: u64, timeout: u32, enableparameters: *const ENABLE_TRACE_PARAMETERS) -> u32;
    }
    EnableTraceEx2(tracehandle, ::core::mem::transmute(providerid), controlcode, level, matchanykeyword, matchallkeyword, timeout, ::core::mem::transmute(enableparameters))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumerateTraceGuids(guidpropertiesarray: &mut [*mut TRACE_GUID_PROPERTIES], guidcount: *mut u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EnumerateTraceGuids(guidpropertiesarray: *mut *mut TRACE_GUID_PROPERTIES, propertyarraycount: u32, guidcount: *mut u32) -> u32;
    }
    EnumerateTraceGuids(::core::mem::transmute(::windows::core::as_mut_ptr_or_null(guidpropertiesarray)), guidpropertiesarray.len() as _, ::core::mem::transmute(guidcount))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
#[inline]
pub unsafe fn EnumerateTraceGuidsEx(tracequeryinfoclass: TRACE_QUERY_INFO_CLASS, inbuffer: *const ::core::ffi::c_void, inbuffersize: u32, outbuffer: *mut ::core::ffi::c_void, outbuffersize: u32, returnlength: *mut u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EnumerateTraceGuidsEx(tracequeryinfoclass: TRACE_QUERY_INFO_CLASS, inbuffer: *const ::core::ffi::c_void, inbuffersize: u32, outbuffer: *mut ::core::ffi::c_void, outbuffersize: u32, returnlength: *mut u32) -> u32;
    }
    EnumerateTraceGuidsEx(tracequeryinfoclass, ::core::mem::transmute(inbuffer), inbuffersize, ::core::mem::transmute(outbuffer), outbuffersize, ::core::mem::transmute(returnlength))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EventAccessControl<'a, P0, P1>(guid: *const ::windows::core::GUID, operation: u32, sid: P0, rights: u32, allowordeny: P1) -> u32
where
    P0: ::std::convert::Into<super::super::super::Foundation::PSID>,
    P1: ::std::convert::Into<super::super::super::Foundation::BOOLEAN>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EventAccessControl(guid: *const ::windows::core::GUID, operation: u32, sid: super::super::super::Foundation::PSID, rights: u32, allowordeny: super::super::super::Foundation::BOOLEAN) -> u32;
    }
    EventAccessControl(::core::mem::transmute(guid), operation, sid.into(), rights, allowordeny.into())
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Security\"`*"]
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn EventAccessQuery(guid: *const ::windows::core::GUID, buffer: super::super::super::Security::PSECURITY_DESCRIPTOR, buffersize: *mut u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EventAccessQuery(guid: *const ::windows::core::GUID, buffer: super::super::super::Security::PSECURITY_DESCRIPTOR, buffersize: *mut u32) -> u32;
    }
    EventAccessQuery(::core::mem::transmute(guid), ::core::mem::transmute(buffer), ::core::mem::transmute(buffersize))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
#[inline]
pub unsafe fn EventAccessRemove(guid: *const ::windows::core::GUID) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EventAccessRemove(guid: *const ::windows::core::GUID) -> u32;
    }
    EventAccessRemove(::core::mem::transmute(guid))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
#[inline]
pub unsafe fn EventActivityIdControl(controlcode: u32, activityid: *mut ::windows::core::GUID) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EventActivityIdControl(controlcode: u32, activityid: *mut ::windows::core::GUID) -> u32;
    }
    EventActivityIdControl(controlcode, ::core::mem::transmute(activityid))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EventEnabled(reghandle: u64, eventdescriptor: *const EVENT_DESCRIPTOR) -> super::super::super::Foundation::BOOLEAN {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EventEnabled(reghandle: u64, eventdescriptor: *const EVENT_DESCRIPTOR) -> super::super::super::Foundation::BOOLEAN;
    }
    EventEnabled(reghandle, ::core::mem::transmute(eventdescriptor))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EventProviderEnabled(reghandle: u64, level: u8, keyword: u64) -> super::super::super::Foundation::BOOLEAN {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EventProviderEnabled(reghandle: u64, level: u8, keyword: u64) -> super::super::super::Foundation::BOOLEAN;
    }
    EventProviderEnabled(reghandle, level, keyword)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
#[inline]
pub unsafe fn EventRegister(providerid: *const ::windows::core::GUID, enablecallback: PENABLECALLBACK, callbackcontext: *const ::core::ffi::c_void, reghandle: *mut u64) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EventRegister(providerid: *const ::windows::core::GUID, enablecallback: *mut ::core::ffi::c_void, callbackcontext: *const ::core::ffi::c_void, reghandle: *mut u64) -> u32;
    }
    EventRegister(::core::mem::transmute(providerid), ::core::mem::transmute(enablecallback), ::core::mem::transmute(callbackcontext), ::core::mem::transmute(reghandle))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
#[inline]
pub unsafe fn EventSetInformation(reghandle: u64, informationclass: EVENT_INFO_CLASS, eventinformation: *const ::core::ffi::c_void, informationlength: u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EventSetInformation(reghandle: u64, informationclass: EVENT_INFO_CLASS, eventinformation: *const ::core::ffi::c_void, informationlength: u32) -> u32;
    }
    EventSetInformation(reghandle, informationclass, ::core::mem::transmute(eventinformation), informationlength)
}
pub const EventTraceConfigGuid: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x01853a65_418f_4f36_aefc_dc0f1d2fd235);
pub const EventTraceGuid: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x68fdd900_4a3e_11d1_84f4_0000f80464e3);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
#[inline]
pub unsafe fn EventUnregister(reghandle: u64) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EventUnregister(reghandle: u64) -> u32;
    }
    EventUnregister(reghandle)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
#[inline]
pub unsafe fn EventWrite(reghandle: u64, eventdescriptor: *const EVENT_DESCRIPTOR, userdata: &[EVENT_DATA_DESCRIPTOR]) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EventWrite(reghandle: u64, eventdescriptor: *const EVENT_DESCRIPTOR, userdatacount: u32, userdata: *const EVENT_DATA_DESCRIPTOR) -> u32;
    }
    EventWrite(reghandle, ::core::mem::transmute(eventdescriptor), userdata.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(userdata)))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
#[inline]
pub unsafe fn EventWriteEx(reghandle: u64, eventdescriptor: *const EVENT_DESCRIPTOR, filter: u64, flags: u32, activityid: *const ::windows::core::GUID, relatedactivityid: *const ::windows::core::GUID, userdata: &[EVENT_DATA_DESCRIPTOR]) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EventWriteEx(reghandle: u64, eventdescriptor: *const EVENT_DESCRIPTOR, filter: u64, flags: u32, activityid: *const ::windows::core::GUID, relatedactivityid: *const ::windows::core::GUID, userdatacount: u32, userdata: *const EVENT_DATA_DESCRIPTOR) -> u32;
    }
    EventWriteEx(reghandle, ::core::mem::transmute(eventdescriptor), filter, flags, ::core::mem::transmute(activityid), ::core::mem::transmute(relatedactivityid), userdata.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(userdata)))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
#[inline]
pub unsafe fn EventWriteString<'a, P0>(reghandle: u64, level: u8, keyword: u64, string: P0) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EventWriteString(reghandle: u64, level: u8, keyword: u64, string: ::windows::core::PCWSTR) -> u32;
    }
    EventWriteString(reghandle, level, keyword, string.into())
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
#[inline]
pub unsafe fn EventWriteTransfer(reghandle: u64, eventdescriptor: *const EVENT_DESCRIPTOR, activityid: *const ::windows::core::GUID, relatedactivityid: *const ::windows::core::GUID, userdata: &[EVENT_DATA_DESCRIPTOR]) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EventWriteTransfer(reghandle: u64, eventdescriptor: *const EVENT_DESCRIPTOR, activityid: *const ::windows::core::GUID, relatedactivityid: *const ::windows::core::GUID, userdatacount: u32, userdata: *const EVENT_DATA_DESCRIPTOR) -> u32;
    }
    EventWriteTransfer(reghandle, ::core::mem::transmute(eventdescriptor), ::core::mem::transmute(activityid), ::core::mem::transmute(relatedactivityid), userdata.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(userdata)))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FlushTraceA<'a, P0>(tracehandle: u64, instancename: P0, properties: *mut EVENT_TRACE_PROPERTIES) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn FlushTraceA(tracehandle: u64, instancename: ::windows::core::PCSTR, properties: *mut EVENT_TRACE_PROPERTIES) -> u32;
    }
    FlushTraceA(tracehandle, instancename.into(), ::core::mem::transmute(properties))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FlushTraceW<'a, P0>(tracehandle: u64, instancename: P0, properties: *mut EVENT_TRACE_PROPERTIES) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn FlushTraceW(tracehandle: u64, instancename: ::windows::core::PCWSTR, properties: *mut EVENT_TRACE_PROPERTIES) -> u32;
    }
    FlushTraceW(tracehandle, instancename.into(), ::core::mem::transmute(properties))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const GLOBAL_LOGGER_NAME: &str = "GlobalLogger";
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const GLOBAL_LOGGER_NAMEA: &str = "GlobalLogger";
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const GLOBAL_LOGGER_NAMEW: &str = "GlobalLogger";
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
#[inline]
pub unsafe fn GetTraceEnableFlags(tracehandle: u64) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetTraceEnableFlags(tracehandle: u64) -> u32;
    }
    GetTraceEnableFlags(tracehandle)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
#[inline]
pub unsafe fn GetTraceEnableLevel(tracehandle: u64) -> u8 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetTraceEnableLevel(tracehandle: u64) -> u8;
    }
    GetTraceEnableLevel(tracehandle)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
#[inline]
pub unsafe fn GetTraceLoggerHandle(buffer: *const ::core::ffi::c_void) -> u64 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetTraceLoggerHandle(buffer: *const ::core::ffi::c_void) -> u64;
    }
    GetTraceLoggerHandle(::core::mem::transmute(buffer))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
#[repr(transparent)]
pub struct ITraceEvent(::windows::core::IUnknown);
impl ITraceEvent {
    pub unsafe fn Clone(&self) -> ::windows::core::Result<ITraceEvent> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ITraceEvent>(result__)
    }
    pub unsafe fn GetUserContext(&self) -> ::windows::core::Result<*mut ::core::ffi::c_void> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetUserContext)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut ::core::ffi::c_void>(result__)
    }
    pub unsafe fn GetEventRecord(&self) -> ::windows::core::Result<*mut EVENT_RECORD> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetEventRecord)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut EVENT_RECORD>(result__)
    }
    pub unsafe fn SetPayload(&self, payload: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPayload)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(payload)), payload.len() as _).ok()
    }
    pub unsafe fn SetEventDescriptor(&self, eventdescriptor: *const EVENT_DESCRIPTOR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetEventDescriptor)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(eventdescriptor)).ok()
    }
    pub unsafe fn SetProcessId(&self, processid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetProcessId)(::windows::core::Interface::as_raw(self), processid).ok()
    }
    pub unsafe fn SetProcessorIndex(&self, processorindex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetProcessorIndex)(::windows::core::Interface::as_raw(self), processorindex).ok()
    }
    pub unsafe fn SetThreadId(&self, threadid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetThreadId)(::windows::core::Interface::as_raw(self), threadid).ok()
    }
    pub unsafe fn SetThreadTimes(&self, kerneltime: u32, usertime: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetThreadTimes)(::windows::core::Interface::as_raw(self), kerneltime, usertime).ok()
    }
    pub unsafe fn SetActivityId(&self, activityid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetActivityId)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(activityid)).ok()
    }
    pub unsafe fn SetTimeStamp(&self, timestamp: *const i64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTimeStamp)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(timestamp)).ok()
    }
    pub unsafe fn SetProviderId(&self, providerid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetProviderId)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(providerid)).ok()
    }
}
impl ::core::convert::From<ITraceEvent> for ::windows::core::IUnknown {
    fn from(value: ITraceEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ITraceEvent> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ITraceEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITraceEvent> for ::windows::core::IUnknown {
    fn from(value: &ITraceEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ITraceEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITraceEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITraceEvent {}
impl ::core::fmt::Debug for ITraceEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITraceEvent").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITraceEvent {
    type Vtable = ITraceEvent_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8cc97f40_9028_4ff3_9b62_7d1f79ca7bcb);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITraceEvent_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newevent: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetUserContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usercontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetEventRecord: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventrecord: *mut *mut EVENT_RECORD) -> ::windows::core::HRESULT,
    pub SetPayload: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, payload: *const u8, payloadsize: u32) -> ::windows::core::HRESULT,
    pub SetEventDescriptor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventdescriptor: *const EVENT_DESCRIPTOR) -> ::windows::core::HRESULT,
    pub SetProcessId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, processid: u32) -> ::windows::core::HRESULT,
    pub SetProcessorIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, processorindex: u32) -> ::windows::core::HRESULT,
    pub SetThreadId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, threadid: u32) -> ::windows::core::HRESULT,
    pub SetThreadTimes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, kerneltime: u32, usertime: u32) -> ::windows::core::HRESULT,
    pub SetActivityId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activityid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub SetTimeStamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timestamp: *const i64) -> ::windows::core::HRESULT,
    pub SetProviderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, providerid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
#[repr(transparent)]
pub struct ITraceEventCallback(::windows::core::IUnknown);
impl ITraceEventCallback {
    pub unsafe fn OnBeginProcessTrace<'a, P0, P1>(&self, headerevent: P0, relogger: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ITraceEvent>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, ITraceRelogger>>,
    {
        (::windows::core::Interface::vtable(self).OnBeginProcessTrace)(::windows::core::Interface::as_raw(self), headerevent.into().abi(), relogger.into().abi()).ok()
    }
    pub unsafe fn OnFinalizeProcessTrace<'a, P0>(&self, relogger: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ITraceRelogger>>,
    {
        (::windows::core::Interface::vtable(self).OnFinalizeProcessTrace)(::windows::core::Interface::as_raw(self), relogger.into().abi()).ok()
    }
    pub unsafe fn OnEvent<'a, P0, P1>(&self, event: P0, relogger: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ITraceEvent>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, ITraceRelogger>>,
    {
        (::windows::core::Interface::vtable(self).OnEvent)(::windows::core::Interface::as_raw(self), event.into().abi(), relogger.into().abi()).ok()
    }
}
impl ::core::convert::From<ITraceEventCallback> for ::windows::core::IUnknown {
    fn from(value: ITraceEventCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ITraceEventCallback> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ITraceEventCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITraceEventCallback> for ::windows::core::IUnknown {
    fn from(value: &ITraceEventCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ITraceEventCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITraceEventCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITraceEventCallback {}
impl ::core::fmt::Debug for ITraceEventCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITraceEventCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITraceEventCallback {
    type Vtable = ITraceEventCallback_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3ed25501_593f_43e9_8f38_3ab46f5a4a52);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITraceEventCallback_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub OnBeginProcessTrace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, headerevent: *mut ::core::ffi::c_void, relogger: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OnFinalizeProcessTrace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, relogger: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OnEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, event: *mut ::core::ffi::c_void, relogger: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
#[repr(transparent)]
pub struct ITraceRelogger(::windows::core::IUnknown);
impl ITraceRelogger {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddLogfileTraceStream<'a, P0>(&self, logfilename: P0, usercontext: *const ::core::ffi::c_void) -> ::windows::core::Result<u64>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Foundation::BSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).AddLogfileTraceStream)(::windows::core::Interface::as_raw(self), logfilename.into().abi(), ::core::mem::transmute(usercontext), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddRealtimeTraceStream<'a, P0>(&self, loggername: P0, usercontext: *const ::core::ffi::c_void) -> ::windows::core::Result<u64>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Foundation::BSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).AddRealtimeTraceStream)(::windows::core::Interface::as_raw(self), loggername.into().abi(), ::core::mem::transmute(usercontext), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u64>(result__)
    }
    pub unsafe fn RegisterCallback<'a, P0>(&self, callback: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ITraceEventCallback>>,
    {
        (::windows::core::Interface::vtable(self).RegisterCallback)(::windows::core::Interface::as_raw(self), callback.into().abi()).ok()
    }
    pub unsafe fn Inject<'a, P0>(&self, event: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ITraceEvent>>,
    {
        (::windows::core::Interface::vtable(self).Inject)(::windows::core::Interface::as_raw(self), event.into().abi()).ok()
    }
    pub unsafe fn CreateEventInstance(&self, tracehandle: u64, flags: u32) -> ::windows::core::Result<ITraceEvent> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CreateEventInstance)(::windows::core::Interface::as_raw(self), tracehandle, flags, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ITraceEvent>(result__)
    }
    pub unsafe fn ProcessTrace(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ProcessTrace)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetOutputFilename<'a, P0>(&self, logfilename: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetOutputFilename)(::windows::core::Interface::as_raw(self), logfilename.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCompressionMode<'a, P0>(&self, compressionmode: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::BOOLEAN>,
    {
        (::windows::core::Interface::vtable(self).SetCompressionMode)(::windows::core::Interface::as_raw(self), compressionmode.into()).ok()
    }
    pub unsafe fn Cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Cancel)(::windows::core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<ITraceRelogger> for ::windows::core::IUnknown {
    fn from(value: ITraceRelogger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ITraceRelogger> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ITraceRelogger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITraceRelogger> for ::windows::core::IUnknown {
    fn from(value: &ITraceRelogger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ITraceRelogger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITraceRelogger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITraceRelogger {}
impl ::core::fmt::Debug for ITraceRelogger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITraceRelogger").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITraceRelogger {
    type Vtable = ITraceRelogger_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf754ad43_3bcc_4286_8009_9c5da214e84e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITraceRelogger_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub AddLogfileTraceStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, logfilename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, usercontext: *const ::core::ffi::c_void, tracehandle: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddLogfileTraceStream: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub AddRealtimeTraceStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, loggername: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, usercontext: *const ::core::ffi::c_void, tracehandle: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddRealtimeTraceStream: usize,
    pub RegisterCallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, callback: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Inject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, event: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateEventInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tracehandle: u64, flags: u32, event: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ProcessTrace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetOutputFilename: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, logfilename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetOutputFilename: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetCompressionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, compressionmode: super::super::super::Foundation::BOOLEAN) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetCompressionMode: usize,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const KERNEL_LOGGER_NAME: &str = "NT Kernel Logger";
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const KERNEL_LOGGER_NAMEA: &str = "NT Kernel Logger";
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const KERNEL_LOGGER_NAMEW: &str = "NT Kernel Logger";
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MAP_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENTMAP_INFO_FLAG_MANIFEST_VALUEMAP: MAP_FLAGS = MAP_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENTMAP_INFO_FLAG_MANIFEST_BITMAP: MAP_FLAGS = MAP_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENTMAP_INFO_FLAG_MANIFEST_PATTERNMAP: MAP_FLAGS = MAP_FLAGS(4i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENTMAP_INFO_FLAG_WBEM_VALUEMAP: MAP_FLAGS = MAP_FLAGS(8i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENTMAP_INFO_FLAG_WBEM_BITMAP: MAP_FLAGS = MAP_FLAGS(16i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENTMAP_INFO_FLAG_WBEM_FLAG: MAP_FLAGS = MAP_FLAGS(32i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENTMAP_INFO_FLAG_WBEM_NO_MAP: MAP_FLAGS = MAP_FLAGS(64i32);
impl ::core::marker::Copy for MAP_FLAGS {}
impl ::core::clone::Clone for MAP_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MAP_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MAP_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for MAP_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MAP_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MAP_VALUETYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENTMAP_ENTRY_VALUETYPE_ULONG: MAP_VALUETYPE = MAP_VALUETYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const EVENTMAP_ENTRY_VALUETYPE_STRING: MAP_VALUETYPE = MAP_VALUETYPE(1i32);
impl ::core::marker::Copy for MAP_VALUETYPE {}
impl ::core::clone::Clone for MAP_VALUETYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MAP_VALUETYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MAP_VALUETYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for MAP_VALUETYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MAP_VALUETYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const MAX_EVENT_DATA_DESCRIPTORS: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const MAX_EVENT_FILTERS_COUNT: u32 = 13u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const MAX_EVENT_FILTER_DATA_SIZE: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const MAX_EVENT_FILTER_EVENT_ID_COUNT: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const MAX_EVENT_FILTER_EVENT_NAME_SIZE: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const MAX_EVENT_FILTER_PAYLOAD_SIZE: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const MAX_EVENT_FILTER_PID_COUNT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const MAX_MOF_FIELDS: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const MAX_PAYLOAD_PREDICATES: u32 = 8u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct MOF_FIELD {
    pub DataPtr: u64,
    pub Length: u32,
    pub DataType: u32,
}
impl ::core::marker::Copy for MOF_FIELD {}
impl ::core::clone::Clone for MOF_FIELD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MOF_FIELD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MOF_FIELD").field("DataPtr", &self.DataPtr).field("Length", &self.Length).field("DataType", &self.DataType).finish()
    }
}
unsafe impl ::windows::core::Abi for MOF_FIELD {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MOF_FIELD {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MOF_FIELD>()) == 0 }
    }
}
impl ::core::cmp::Eq for MOF_FIELD {}
impl ::core::default::Default for MOF_FIELD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct OFFSETINSTANCEDATAANDLENGTH {
    pub OffsetInstanceData: u32,
    pub LengthInstanceData: u32,
}
impl ::core::marker::Copy for OFFSETINSTANCEDATAANDLENGTH {}
impl ::core::clone::Clone for OFFSETINSTANCEDATAANDLENGTH {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for OFFSETINSTANCEDATAANDLENGTH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OFFSETINSTANCEDATAANDLENGTH").field("OffsetInstanceData", &self.OffsetInstanceData).field("LengthInstanceData", &self.LengthInstanceData).finish()
    }
}
unsafe impl ::windows::core::Abi for OFFSETINSTANCEDATAANDLENGTH {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for OFFSETINSTANCEDATAANDLENGTH {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<OFFSETINSTANCEDATAANDLENGTH>()) == 0 }
    }
}
impl ::core::cmp::Eq for OFFSETINSTANCEDATAANDLENGTH {}
impl ::core::default::Default for OFFSETINSTANCEDATAANDLENGTH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`, `\"Win32_System_Time\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
#[inline]
pub unsafe fn OpenTraceA(logfile: *mut EVENT_TRACE_LOGFILEA) -> u64 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn OpenTraceA(logfile: *mut EVENT_TRACE_LOGFILEA) -> u64;
    }
    OpenTraceA(::core::mem::transmute(logfile))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`, `\"Win32_System_Time\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
#[inline]
pub unsafe fn OpenTraceW(logfile: *mut EVENT_TRACE_LOGFILEW) -> u64 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn OpenTraceW(logfile: *mut EVENT_TRACE_LOGFILEW) -> u64;
    }
    OpenTraceW(::core::mem::transmute(logfile))
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct PAYLOAD_FILTER_PREDICATE {
    pub FieldName: ::windows::core::PWSTR,
    pub CompareOp: u16,
    pub Value: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for PAYLOAD_FILTER_PREDICATE {}
impl ::core::clone::Clone for PAYLOAD_FILTER_PREDICATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PAYLOAD_FILTER_PREDICATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PAYLOAD_FILTER_PREDICATE").field("FieldName", &self.FieldName).field("CompareOp", &self.CompareOp).field("Value", &self.Value).finish()
    }
}
unsafe impl ::windows::core::Abi for PAYLOAD_FILTER_PREDICATE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PAYLOAD_FILTER_PREDICATE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PAYLOAD_FILTER_PREDICATE>()) == 0 }
    }
}
impl ::core::cmp::Eq for PAYLOAD_FILTER_PREDICATE {}
impl ::core::default::Default for PAYLOAD_FILTER_PREDICATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PAYLOAD_OPERATOR(pub i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const PAYLOADFIELD_EQ: PAYLOAD_OPERATOR = PAYLOAD_OPERATOR(0i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const PAYLOADFIELD_NE: PAYLOAD_OPERATOR = PAYLOAD_OPERATOR(1i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const PAYLOADFIELD_LE: PAYLOAD_OPERATOR = PAYLOAD_OPERATOR(2i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const PAYLOADFIELD_GT: PAYLOAD_OPERATOR = PAYLOAD_OPERATOR(3i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const PAYLOADFIELD_LT: PAYLOAD_OPERATOR = PAYLOAD_OPERATOR(4i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const PAYLOADFIELD_GE: PAYLOAD_OPERATOR = PAYLOAD_OPERATOR(5i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const PAYLOADFIELD_BETWEEN: PAYLOAD_OPERATOR = PAYLOAD_OPERATOR(6i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const PAYLOADFIELD_NOTBETWEEN: PAYLOAD_OPERATOR = PAYLOAD_OPERATOR(7i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const PAYLOADFIELD_MODULO: PAYLOAD_OPERATOR = PAYLOAD_OPERATOR(8i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const PAYLOADFIELD_CONTAINS: PAYLOAD_OPERATOR = PAYLOAD_OPERATOR(20i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const PAYLOADFIELD_DOESNTCONTAIN: PAYLOAD_OPERATOR = PAYLOAD_OPERATOR(21i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const PAYLOADFIELD_IS: PAYLOAD_OPERATOR = PAYLOAD_OPERATOR(30i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const PAYLOADFIELD_ISNOT: PAYLOAD_OPERATOR = PAYLOAD_OPERATOR(31i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const PAYLOADFIELD_INVALID: PAYLOAD_OPERATOR = PAYLOAD_OPERATOR(32i32);
impl ::core::marker::Copy for PAYLOAD_OPERATOR {}
impl ::core::clone::Clone for PAYLOAD_OPERATOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PAYLOAD_OPERATOR {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PAYLOAD_OPERATOR {
    type Abi = Self;
}
impl ::core::fmt::Debug for PAYLOAD_OPERATOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PAYLOAD_OPERATOR").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub type PENABLECALLBACK = ::core::option::Option<unsafe extern "system" fn(sourceid: *const ::windows::core::GUID, isenabled: ENABLECALLBACK_ENABLED_STATE, level: u8, matchanykeyword: u64, matchallkeyword: u64, filterdata: *const EVENT_FILTER_DESCRIPTOR, callbackcontext: *mut ::core::ffi::c_void)>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub type PEVENT_CALLBACK = ::core::option::Option<unsafe extern "system" fn(pevent: *mut EVENT_TRACE)>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub type PEVENT_RECORD_CALLBACK = ::core::option::Option<unsafe extern "system" fn(eventrecord: *mut EVENT_RECORD)>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`, `\"Win32_System_Time\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
pub type PEVENT_TRACE_BUFFER_CALLBACKA = ::core::option::Option<unsafe extern "system" fn(logfile: *mut EVENT_TRACE_LOGFILEA) -> u32>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`, `\"Win32_System_Time\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
pub type PEVENT_TRACE_BUFFER_CALLBACKW = ::core::option::Option<unsafe extern "system" fn(logfile: *mut EVENT_TRACE_LOGFILEW) -> u32>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const PROCESS_TRACE_MODE_EVENT_RECORD: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const PROCESS_TRACE_MODE_RAW_TIMESTAMP: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const PROCESS_TRACE_MODE_REAL_TIME: u32 = 256u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct PROFILE_SOURCE_INFO {
    pub NextEntryOffset: u32,
    pub Source: u32,
    pub MinInterval: u32,
    pub MaxInterval: u32,
    pub Reserved: u64,
    pub Description: [u16; 1],
}
impl ::core::marker::Copy for PROFILE_SOURCE_INFO {}
impl ::core::clone::Clone for PROFILE_SOURCE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PROFILE_SOURCE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROFILE_SOURCE_INFO").field("NextEntryOffset", &self.NextEntryOffset).field("Source", &self.Source).field("MinInterval", &self.MinInterval).field("MaxInterval", &self.MaxInterval).field("Reserved", &self.Reserved).field("Description", &self.Description).finish()
    }
}
unsafe impl ::windows::core::Abi for PROFILE_SOURCE_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PROFILE_SOURCE_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PROFILE_SOURCE_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for PROFILE_SOURCE_INFO {}
impl ::core::default::Default for PROFILE_SOURCE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct PROPERTY_DATA_DESCRIPTOR {
    pub PropertyName: u64,
    pub ArrayIndex: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for PROPERTY_DATA_DESCRIPTOR {}
impl ::core::clone::Clone for PROPERTY_DATA_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PROPERTY_DATA_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROPERTY_DATA_DESCRIPTOR").field("PropertyName", &self.PropertyName).field("ArrayIndex", &self.ArrayIndex).field("Reserved", &self.Reserved).finish()
    }
}
unsafe impl ::windows::core::Abi for PROPERTY_DATA_DESCRIPTOR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PROPERTY_DATA_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PROPERTY_DATA_DESCRIPTOR>()) == 0 }
    }
}
impl ::core::cmp::Eq for PROPERTY_DATA_DESCRIPTOR {}
impl ::core::default::Default for PROPERTY_DATA_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PROPERTY_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const PropertyStruct: PROPERTY_FLAGS = PROPERTY_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const PropertyParamLength: PROPERTY_FLAGS = PROPERTY_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const PropertyParamCount: PROPERTY_FLAGS = PROPERTY_FLAGS(4i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const PropertyWBEMXmlFragment: PROPERTY_FLAGS = PROPERTY_FLAGS(8i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const PropertyParamFixedLength: PROPERTY_FLAGS = PROPERTY_FLAGS(16i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const PropertyParamFixedCount: PROPERTY_FLAGS = PROPERTY_FLAGS(32i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const PropertyHasTags: PROPERTY_FLAGS = PROPERTY_FLAGS(64i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const PropertyHasCustomSchema: PROPERTY_FLAGS = PROPERTY_FLAGS(128i32);
impl ::core::marker::Copy for PROPERTY_FLAGS {}
impl ::core::clone::Clone for PROPERTY_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROPERTY_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PROPERTY_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for PROPERTY_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROPERTY_FLAGS").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct PROVIDER_ENUMERATION_INFO {
    pub NumberOfProviders: u32,
    pub Reserved: u32,
    pub TraceProviderInfoArray: [TRACE_PROVIDER_INFO; 1],
}
impl ::core::marker::Copy for PROVIDER_ENUMERATION_INFO {}
impl ::core::clone::Clone for PROVIDER_ENUMERATION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PROVIDER_ENUMERATION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROVIDER_ENUMERATION_INFO").field("NumberOfProviders", &self.NumberOfProviders).field("Reserved", &self.Reserved).field("TraceProviderInfoArray", &self.TraceProviderInfoArray).finish()
    }
}
unsafe impl ::windows::core::Abi for PROVIDER_ENUMERATION_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PROVIDER_ENUMERATION_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PROVIDER_ENUMERATION_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for PROVIDER_ENUMERATION_INFO {}
impl ::core::default::Default for PROVIDER_ENUMERATION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct PROVIDER_EVENT_INFO {
    pub NumberOfEvents: u32,
    pub Reserved: u32,
    pub EventDescriptorsArray: [EVENT_DESCRIPTOR; 1],
}
impl ::core::marker::Copy for PROVIDER_EVENT_INFO {}
impl ::core::clone::Clone for PROVIDER_EVENT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PROVIDER_EVENT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROVIDER_EVENT_INFO").field("NumberOfEvents", &self.NumberOfEvents).field("Reserved", &self.Reserved).field("EventDescriptorsArray", &self.EventDescriptorsArray).finish()
    }
}
unsafe impl ::windows::core::Abi for PROVIDER_EVENT_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PROVIDER_EVENT_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PROVIDER_EVENT_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for PROVIDER_EVENT_INFO {}
impl ::core::default::Default for PROVIDER_EVENT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct PROVIDER_FIELD_INFO {
    pub NameOffset: u32,
    pub DescriptionOffset: u32,
    pub Value: u64,
}
impl ::core::marker::Copy for PROVIDER_FIELD_INFO {}
impl ::core::clone::Clone for PROVIDER_FIELD_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PROVIDER_FIELD_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROVIDER_FIELD_INFO").field("NameOffset", &self.NameOffset).field("DescriptionOffset", &self.DescriptionOffset).field("Value", &self.Value).finish()
    }
}
unsafe impl ::windows::core::Abi for PROVIDER_FIELD_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PROVIDER_FIELD_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PROVIDER_FIELD_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for PROVIDER_FIELD_INFO {}
impl ::core::default::Default for PROVIDER_FIELD_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct PROVIDER_FIELD_INFOARRAY {
    pub NumberOfElements: u32,
    pub FieldType: EVENT_FIELD_TYPE,
    pub FieldInfoArray: [PROVIDER_FIELD_INFO; 1],
}
impl ::core::marker::Copy for PROVIDER_FIELD_INFOARRAY {}
impl ::core::clone::Clone for PROVIDER_FIELD_INFOARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PROVIDER_FIELD_INFOARRAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROVIDER_FIELD_INFOARRAY").field("NumberOfElements", &self.NumberOfElements).field("FieldType", &self.FieldType).field("FieldInfoArray", &self.FieldInfoArray).finish()
    }
}
unsafe impl ::windows::core::Abi for PROVIDER_FIELD_INFOARRAY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PROVIDER_FIELD_INFOARRAY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PROVIDER_FIELD_INFOARRAY>()) == 0 }
    }
}
impl ::core::cmp::Eq for PROVIDER_FIELD_INFOARRAY {}
impl ::core::default::Default for PROVIDER_FIELD_INFOARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct PROVIDER_FILTER_INFO {
    pub Id: u8,
    pub Version: u8,
    pub MessageOffset: u32,
    pub Reserved: u32,
    pub PropertyCount: u32,
    pub EventPropertyInfoArray: [EVENT_PROPERTY_INFO; 1],
}
impl ::core::marker::Copy for PROVIDER_FILTER_INFO {}
impl ::core::clone::Clone for PROVIDER_FILTER_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PROVIDER_FILTER_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PROVIDER_FILTER_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PROVIDER_FILTER_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for PROVIDER_FILTER_INFO {}
impl ::core::default::Default for PROVIDER_FILTER_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const PrivateLoggerNotificationGuid: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3595ab5c_042a_4c8e_b942_2d059bfeb1b1);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ProcessTrace(handlearray: &[u64], starttime: *const super::super::super::Foundation::FILETIME, endtime: *const super::super::super::Foundation::FILETIME) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ProcessTrace(handlearray: *const u64, handlecount: u32, starttime: *const super::super::super::Foundation::FILETIME, endtime: *const super::super::super::Foundation::FILETIME) -> u32;
    }
    ProcessTrace(::core::mem::transmute(::windows::core::as_ptr_or_null(handlearray)), handlearray.len() as _, ::core::mem::transmute(starttime), ::core::mem::transmute(endtime))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QueryAllTracesA(propertyarray: &mut [*mut EVENT_TRACE_PROPERTIES], loggercount: *mut u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn QueryAllTracesA(propertyarray: *mut *mut EVENT_TRACE_PROPERTIES, propertyarraycount: u32, loggercount: *mut u32) -> u32;
    }
    QueryAllTracesA(::core::mem::transmute(::windows::core::as_mut_ptr_or_null(propertyarray)), propertyarray.len() as _, ::core::mem::transmute(loggercount))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QueryAllTracesW(propertyarray: &mut [*mut EVENT_TRACE_PROPERTIES], loggercount: *mut u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn QueryAllTracesW(propertyarray: *mut *mut EVENT_TRACE_PROPERTIES, propertyarraycount: u32, loggercount: *mut u32) -> u32;
    }
    QueryAllTracesW(::core::mem::transmute(::windows::core::as_mut_ptr_or_null(propertyarray)), propertyarray.len() as _, ::core::mem::transmute(loggercount))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QueryTraceA<'a, P0>(tracehandle: u64, instancename: P0, properties: *mut EVENT_TRACE_PROPERTIES) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn QueryTraceA(tracehandle: u64, instancename: ::windows::core::PCSTR, properties: *mut EVENT_TRACE_PROPERTIES) -> u32;
    }
    QueryTraceA(tracehandle, instancename.into(), ::core::mem::transmute(properties))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
#[inline]
pub unsafe fn QueryTraceProcessingHandle(processinghandle: u64, informationclass: ETW_PROCESS_HANDLE_INFO_TYPE, inbuffer: *const ::core::ffi::c_void, inbuffersize: u32, outbuffer: *mut ::core::ffi::c_void, outbuffersize: u32, returnlength: *mut u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn QueryTraceProcessingHandle(processinghandle: u64, informationclass: ETW_PROCESS_HANDLE_INFO_TYPE, inbuffer: *const ::core::ffi::c_void, inbuffersize: u32, outbuffer: *mut ::core::ffi::c_void, outbuffersize: u32, returnlength: *mut u32) -> u32;
    }
    QueryTraceProcessingHandle(processinghandle, informationclass, ::core::mem::transmute(inbuffer), inbuffersize, ::core::mem::transmute(outbuffer), outbuffersize, ::core::mem::transmute(returnlength))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QueryTraceW<'a, P0>(tracehandle: u64, instancename: P0, properties: *mut EVENT_TRACE_PROPERTIES) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn QueryTraceW(tracehandle: u64, instancename: ::windows::core::PCWSTR, properties: *mut EVENT_TRACE_PROPERTIES) -> u32;
    }
    QueryTraceW(tracehandle, instancename.into(), ::core::mem::transmute(properties))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegisterTraceGuidsA<'a, P0, P1>(requestaddress: WMIDPREQUEST, requestcontext: *const ::core::ffi::c_void, controlguid: *const ::windows::core::GUID, traceguidreg: &[TRACE_GUID_REGISTRATION], mofimagepath: P0, mofresourcename: P1, registrationhandle: *mut u64) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegisterTraceGuidsA(requestaddress: *mut ::core::ffi::c_void, requestcontext: *const ::core::ffi::c_void, controlguid: *const ::windows::core::GUID, guidcount: u32, traceguidreg: *const TRACE_GUID_REGISTRATION, mofimagepath: ::windows::core::PCSTR, mofresourcename: ::windows::core::PCSTR, registrationhandle: *mut u64) -> u32;
    }
    RegisterTraceGuidsA(::core::mem::transmute(requestaddress), ::core::mem::transmute(requestcontext), ::core::mem::transmute(controlguid), traceguidreg.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(traceguidreg)), mofimagepath.into(), mofresourcename.into(), ::core::mem::transmute(registrationhandle))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegisterTraceGuidsW<'a, P0, P1>(requestaddress: WMIDPREQUEST, requestcontext: *const ::core::ffi::c_void, controlguid: *const ::windows::core::GUID, traceguidreg: &[TRACE_GUID_REGISTRATION], mofimagepath: P0, mofresourcename: P1, registrationhandle: *mut u64) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegisterTraceGuidsW(requestaddress: *mut ::core::ffi::c_void, requestcontext: *const ::core::ffi::c_void, controlguid: *const ::windows::core::GUID, guidcount: u32, traceguidreg: *const TRACE_GUID_REGISTRATION, mofimagepath: ::windows::core::PCWSTR, mofresourcename: ::windows::core::PCWSTR, registrationhandle: *mut u64) -> u32;
    }
    RegisterTraceGuidsW(::core::mem::transmute(requestaddress), ::core::mem::transmute(requestcontext), ::core::mem::transmute(controlguid), traceguidreg.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(traceguidreg)), mofimagepath.into(), mofresourcename.into(), ::core::mem::transmute(registrationhandle))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
#[inline]
pub unsafe fn RemoveTraceCallback(pguid: *const ::windows::core::GUID) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RemoveTraceCallback(pguid: *const ::windows::core::GUID) -> u32;
    }
    RemoveTraceCallback(::core::mem::transmute(pguid))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_ALPC_KW_GENERAL: u64 = 1u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_CONFIG_KW_GRAPHICS: u64 = 2u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_CONFIG_KW_NETWORK: u64 = 8u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_CONFIG_KW_OPTICAL: u64 = 64u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_CONFIG_KW_PNP: u64 = 32u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_CONFIG_KW_SERVICES: u64 = 16u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_CONFIG_KW_STORAGE: u64 = 4u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_CONFIG_KW_SYSTEM: u64 = 1u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_CPU_KW_CACHE_FLUSH: u64 = 2u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_CPU_KW_CONFIG: u64 = 1u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_CPU_KW_SPEC_CONTROL: u64 = 4u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_EVENT_TYPE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_HYPERVISOR_KW_CALLOUTS: u64 = 2u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_HYPERVISOR_KW_PROFILE: u64 = 1u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_HYPERVISOR_KW_VTL_CHANGE: u64 = 4u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_INTERRUPT_KW_CLOCK_INTERRUPT: u64 = 2u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_INTERRUPT_KW_DPC: u64 = 4u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_INTERRUPT_KW_DPC_QUEUE: u64 = 8u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_INTERRUPT_KW_GENERAL: u64 = 1u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_INTERRUPT_KW_IPI: u64 = 64u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_INTERRUPT_KW_WDF_DPC: u64 = 16u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_INTERRUPT_KW_WDF_INTERRUPT: u64 = 32u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_IOFILTER_KW_FAILURE: u64 = 8u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_IOFILTER_KW_FASTIO: u64 = 4u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_IOFILTER_KW_GENERAL: u64 = 1u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_IOFILTER_KW_INIT: u64 = 2u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_IO_KW_CC: u64 = 256u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_IO_KW_DISK: u64 = 1u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_IO_KW_DISK_INIT: u64 = 2u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_IO_KW_DRIVERS: u64 = 128u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_IO_KW_FILE: u64 = 16u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_IO_KW_FILENAME: u64 = 4u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_IO_KW_NETWORK: u64 = 512u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_IO_KW_OPTICAL: u64 = 32u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_IO_KW_OPTICAL_INIT: u64 = 64u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_IO_KW_SPLIT: u64 = 8u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_LOCK_KW_SPINLOCK: u64 = 1u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_LOCK_KW_SPINLOCK_COUNTERS: u64 = 2u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_LOCK_KW_SYNC_OBJECTS: u64 = 4u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_MEMORY_KW_ALL_FAULTS: u64 = 4u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_MEMORY_KW_CONTMEM_GEN: u64 = 512u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_MEMORY_KW_FOOTPRINT: u64 = 2048u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_MEMORY_KW_GENERAL: u64 = 1u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_MEMORY_KW_HARD_FAULTS: u64 = 2u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_MEMORY_KW_HEAP: u64 = 128u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_MEMORY_KW_MEMINFO: u64 = 16u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_MEMORY_KW_MEMINFO_WS: u64 = 64u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_MEMORY_KW_NONTRADEABLE: u64 = 32768u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_MEMORY_KW_PFSECTION: u64 = 32u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_MEMORY_KW_POOL: u64 = 8u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_MEMORY_KW_REFSET: u64 = 8192u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_MEMORY_KW_SESSION: u64 = 4096u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_MEMORY_KW_VAMAP: u64 = 16384u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_MEMORY_KW_VIRTUAL_ALLOC: u64 = 1024u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_MEMORY_KW_WS: u64 = 256u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_MEMORY_POOL_FILTER_ID: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_OBJECT_KW_GENERAL: u64 = 1u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_OBJECT_KW_HANDLE: u64 = 2u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_POWER_KW_GENERAL: u64 = 1u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_POWER_KW_HIBER_RUNDOWN: u64 = 2u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_POWER_KW_IDLE_SELECTION: u64 = 8u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_POWER_KW_PPM_EXIT_LATENCY: u64 = 16u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_POWER_KW_PROCESSOR_IDLE: u64 = 4u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_PROCESS_KW_DBGPRINT: u64 = 256u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_PROCESS_KW_DEBUG_EVENTS: u64 = 128u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_PROCESS_KW_FREEZE: u64 = 4u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_PROCESS_KW_GENERAL: u64 = 1u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_PROCESS_KW_INSWAP: u64 = 2u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_PROCESS_KW_JOB: u64 = 512u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_PROCESS_KW_LOADER: u64 = 4096u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_PROCESS_KW_PERF_COUNTER: u64 = 8u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_PROCESS_KW_THREAD: u64 = 2048u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_PROCESS_KW_WAKE_COUNTER: u64 = 16u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_PROCESS_KW_WAKE_DROP: u64 = 32u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_PROCESS_KW_WAKE_EVENT: u64 = 64u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_PROCESS_KW_WORKER_THREAD: u64 = 1024u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_PROFILE_KW_GENERAL: u64 = 1u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_PROFILE_KW_PMC_PROFILE: u64 = 2u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_REGISTRY_KW_GENERAL: u64 = 1u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_REGISTRY_KW_HIVE: u64 = 2u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_REGISTRY_KW_NOTIFICATION: u64 = 4u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_SCHEDULER_KW_AFFINITY: u64 = 64u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_SCHEDULER_KW_ANTI_STARVATION: u64 = 16u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_SCHEDULER_KW_COMPACT_CSWITCH: u64 = 1024u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_SCHEDULER_KW_CONTEXT_SWITCH: u64 = 512u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_SCHEDULER_KW_DISPATCHER: u64 = 2u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_SCHEDULER_KW_IDEAL_PROCESSOR: u64 = 256u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_SCHEDULER_KW_KERNEL_QUEUE: u64 = 4u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_SCHEDULER_KW_LOAD_BALANCER: u64 = 32u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_SCHEDULER_KW_PRIORITY: u64 = 128u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_SCHEDULER_KW_SHOULD_YIELD: u64 = 8u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_SCHEDULER_KW_XSCHEDULER: u64 = 1u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_SYSCALL_KW_GENERAL: u64 = 1u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_TIMER_KW_CLOCK_TIMER: u64 = 2u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const SYSTEM_TIMER_KW_GENERAL: u64 = 1u64;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
#[inline]
pub unsafe fn SetTraceCallback(pguid: *const ::windows::core::GUID, eventcallback: PEVENT_CALLBACK) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetTraceCallback(pguid: *const ::windows::core::GUID, eventcallback: *mut ::core::ffi::c_void) -> u32;
    }
    SetTraceCallback(::core::mem::transmute(pguid), ::core::mem::transmute(eventcallback))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn StartTraceA<'a, P0>(tracehandle: *mut u64, instancename: P0, properties: *mut EVENT_TRACE_PROPERTIES) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn StartTraceA(tracehandle: *mut u64, instancename: ::windows::core::PCSTR, properties: *mut EVENT_TRACE_PROPERTIES) -> u32;
    }
    StartTraceA(::core::mem::transmute(tracehandle), instancename.into(), ::core::mem::transmute(properties))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn StartTraceW<'a, P0>(tracehandle: *mut u64, instancename: P0, properties: *mut EVENT_TRACE_PROPERTIES) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn StartTraceW(tracehandle: *mut u64, instancename: ::windows::core::PCWSTR, properties: *mut EVENT_TRACE_PROPERTIES) -> u32;
    }
    StartTraceW(::core::mem::transmute(tracehandle), instancename.into(), ::core::mem::transmute(properties))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn StopTraceA<'a, P0>(tracehandle: u64, instancename: P0, properties: *mut EVENT_TRACE_PROPERTIES) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn StopTraceA(tracehandle: u64, instancename: ::windows::core::PCSTR, properties: *mut EVENT_TRACE_PROPERTIES) -> u32;
    }
    StopTraceA(tracehandle, instancename.into(), ::core::mem::transmute(properties))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn StopTraceW<'a, P0>(tracehandle: u64, instancename: P0, properties: *mut EVENT_TRACE_PROPERTIES) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn StopTraceW(tracehandle: u64, instancename: ::windows::core::PCWSTR, properties: *mut EVENT_TRACE_PROPERTIES) -> u32;
    }
    StopTraceW(tracehandle, instancename.into(), ::core::mem::transmute(properties))
}
pub const SystemAlpcProviderGuid: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfcb9baaf_e529_4980_92e9_ced1a6aadfdf);
pub const SystemConfigProviderGuid: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfef3a8b6_318d_4b67_a96a_3b0f6b8f18fe);
pub const SystemCpuProviderGuid: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc6c5265f_eae8_4650_aae4_9d48603d8510);
pub const SystemHypervisorProviderGuid: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbafa072a_918a_4bed_b622_bc152097098f);
pub const SystemInterruptProviderGuid: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd4bbee17_b545_4888_858b_744169015b25);
pub const SystemIoFilterProviderGuid: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfbd09363_9e22_4661_b8bf_e7a34b535b8c);
pub const SystemIoProviderGuid: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3d5c43e3_0f1c_4202_b817_174c0070dc79);
pub const SystemLockProviderGuid: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x721ddfd3_dacc_4e1e_b26a_a2cb31d4705a);
pub const SystemMemoryProviderGuid: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x82958ca9_b6cd_47f8_a3a8_03ae85a4bc24);
pub const SystemObjectProviderGuid: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfebd7460_3d1d_47eb_af49_c9eeb1e146f2);
pub const SystemPowerProviderGuid: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc134884a_32d5_4488_80e5_14ed7abb8269);
pub const SystemProcessProviderGuid: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x151f55dc_467d_471f_83b5_5f889d46ff66);
pub const SystemProfileProviderGuid: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbfeb0324_1cee_496f_a409_2ac2b48a6322);
pub const SystemRegistryProviderGuid: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x16156bd9_fab4_4cfa_a232_89d1099058e3);
pub const SystemSchedulerProviderGuid: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x599a2a76_4d91_4910_9ac7_7d33f2e97a6c);
pub const SystemSyscallProviderGuid: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x434286f7_6f1b_45bb_b37e_95f623046c7c);
pub const SystemTimerProviderGuid: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4f061568_e215_499f_ab2e_eda0ae890a5b);
pub const SystemTraceControlGuid: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9e814aad_3204_11d2_9a82_006008a86939);
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct TDH_CONTEXT {
    pub ParameterValue: u64,
    pub ParameterType: TDH_CONTEXT_TYPE,
    pub ParameterSize: u32,
}
impl ::core::marker::Copy for TDH_CONTEXT {}
impl ::core::clone::Clone for TDH_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TDH_CONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TDH_CONTEXT").field("ParameterValue", &self.ParameterValue).field("ParameterType", &self.ParameterType).field("ParameterSize", &self.ParameterSize).finish()
    }
}
unsafe impl ::windows::core::Abi for TDH_CONTEXT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TDH_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TDH_CONTEXT>()) == 0 }
    }
}
impl ::core::cmp::Eq for TDH_CONTEXT {}
impl ::core::default::Default for TDH_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TDH_CONTEXT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_CONTEXT_WPP_TMFFILE: TDH_CONTEXT_TYPE = TDH_CONTEXT_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_CONTEXT_WPP_TMFSEARCHPATH: TDH_CONTEXT_TYPE = TDH_CONTEXT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_CONTEXT_WPP_GMT: TDH_CONTEXT_TYPE = TDH_CONTEXT_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_CONTEXT_POINTERSIZE: TDH_CONTEXT_TYPE = TDH_CONTEXT_TYPE(3i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_CONTEXT_PDB_PATH: TDH_CONTEXT_TYPE = TDH_CONTEXT_TYPE(4i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_CONTEXT_MAXIMUM: TDH_CONTEXT_TYPE = TDH_CONTEXT_TYPE(5i32);
impl ::core::marker::Copy for TDH_CONTEXT_TYPE {}
impl ::core::clone::Clone for TDH_CONTEXT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TDH_CONTEXT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TDH_CONTEXT_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for TDH_CONTEXT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TDH_CONTEXT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TDH_HANDLE(pub isize);
impl TDH_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for TDH_HANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for TDH_HANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for TDH_HANDLE {}
impl ::core::fmt::Debug for TDH_HANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TDH_HANDLE").field(&self.0).finish()
    }
}
impl ::core::convert::From<::core::option::Option<TDH_HANDLE>> for TDH_HANDLE {
    fn from(optional: ::core::option::Option<TDH_HANDLE>) -> TDH_HANDLE {
        optional.unwrap_or_default()
    }
}
unsafe impl ::windows::core::Abi for TDH_HANDLE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TEMPLATE_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TEMPLATE_EVENT_DATA: TEMPLATE_FLAGS = TEMPLATE_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TEMPLATE_USER_DATA: TEMPLATE_FLAGS = TEMPLATE_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TEMPLATE_CONTROL_GUID: TEMPLATE_FLAGS = TEMPLATE_FLAGS(4i32);
impl ::core::marker::Copy for TEMPLATE_FLAGS {}
impl ::core::clone::Clone for TEMPLATE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TEMPLATE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TEMPLATE_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for TEMPLATE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TEMPLATE_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TRACELOG_ACCESS_KERNEL_LOGGER: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TRACELOG_ACCESS_REALTIME: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TRACELOG_CREATE_INPROC: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TRACELOG_CREATE_ONDISK: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TRACELOG_CREATE_REALTIME: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TRACELOG_GUID_ENABLE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TRACELOG_JOIN_GROUP: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TRACELOG_LOG_EVENT: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TRACELOG_REGISTER_GUIDS: u32 = 2048u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
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
impl ::core::marker::Copy for TRACE_ENABLE_INFO {}
impl ::core::clone::Clone for TRACE_ENABLE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TRACE_ENABLE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRACE_ENABLE_INFO").field("IsEnabled", &self.IsEnabled).field("Level", &self.Level).field("Reserved1", &self.Reserved1).field("LoggerId", &self.LoggerId).field("EnableProperty", &self.EnableProperty).field("Reserved2", &self.Reserved2).field("MatchAnyKeyword", &self.MatchAnyKeyword).field("MatchAllKeyword", &self.MatchAllKeyword).finish()
    }
}
unsafe impl ::windows::core::Abi for TRACE_ENABLE_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TRACE_ENABLE_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TRACE_ENABLE_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for TRACE_ENABLE_INFO {}
impl ::core::default::Default for TRACE_ENABLE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct TRACE_EVENT_INFO {
    pub ProviderGuid: ::windows::core::GUID,
    pub EventGuid: ::windows::core::GUID,
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
impl ::core::marker::Copy for TRACE_EVENT_INFO {}
impl ::core::clone::Clone for TRACE_EVENT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TRACE_EVENT_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TRACE_EVENT_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TRACE_EVENT_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for TRACE_EVENT_INFO {}
impl ::core::default::Default for TRACE_EVENT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub union TRACE_EVENT_INFO_0 {
    pub EventNameOffset: u32,
    pub ActivityIDNameOffset: u32,
}
impl ::core::marker::Copy for TRACE_EVENT_INFO_0 {}
impl ::core::clone::Clone for TRACE_EVENT_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TRACE_EVENT_INFO_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TRACE_EVENT_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TRACE_EVENT_INFO_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for TRACE_EVENT_INFO_0 {}
impl ::core::default::Default for TRACE_EVENT_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub union TRACE_EVENT_INFO_1 {
    pub EventAttributesOffset: u32,
    pub RelatedActivityIDNameOffset: u32,
}
impl ::core::marker::Copy for TRACE_EVENT_INFO_1 {}
impl ::core::clone::Clone for TRACE_EVENT_INFO_1 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TRACE_EVENT_INFO_1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TRACE_EVENT_INFO_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TRACE_EVENT_INFO_1>()) == 0 }
    }
}
impl ::core::cmp::Eq for TRACE_EVENT_INFO_1 {}
impl ::core::default::Default for TRACE_EVENT_INFO_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub union TRACE_EVENT_INFO_2 {
    pub Flags: TEMPLATE_FLAGS,
    pub Anonymous: TRACE_EVENT_INFO_2_0,
}
impl ::core::marker::Copy for TRACE_EVENT_INFO_2 {}
impl ::core::clone::Clone for TRACE_EVENT_INFO_2 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TRACE_EVENT_INFO_2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TRACE_EVENT_INFO_2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TRACE_EVENT_INFO_2>()) == 0 }
    }
}
impl ::core::cmp::Eq for TRACE_EVENT_INFO_2 {}
impl ::core::default::Default for TRACE_EVENT_INFO_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct TRACE_EVENT_INFO_2_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for TRACE_EVENT_INFO_2_0 {}
impl ::core::clone::Clone for TRACE_EVENT_INFO_2_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TRACE_EVENT_INFO_2_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRACE_EVENT_INFO_2_0").field("_bitfield", &self._bitfield).finish()
    }
}
unsafe impl ::windows::core::Abi for TRACE_EVENT_INFO_2_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TRACE_EVENT_INFO_2_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TRACE_EVENT_INFO_2_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for TRACE_EVENT_INFO_2_0 {}
impl ::core::default::Default for TRACE_EVENT_INFO_2_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct TRACE_GUID_INFO {
    pub InstanceCount: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for TRACE_GUID_INFO {}
impl ::core::clone::Clone for TRACE_GUID_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TRACE_GUID_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRACE_GUID_INFO").field("InstanceCount", &self.InstanceCount).field("Reserved", &self.Reserved).finish()
    }
}
unsafe impl ::windows::core::Abi for TRACE_GUID_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TRACE_GUID_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TRACE_GUID_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for TRACE_GUID_INFO {}
impl ::core::default::Default for TRACE_GUID_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TRACE_GUID_PROPERTIES {
    pub Guid: ::windows::core::GUID,
    pub GuidType: u32,
    pub LoggerId: u32,
    pub EnableLevel: u32,
    pub EnableFlags: u32,
    pub IsEnable: super::super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TRACE_GUID_PROPERTIES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TRACE_GUID_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TRACE_GUID_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRACE_GUID_PROPERTIES").field("Guid", &self.Guid).field("GuidType", &self.GuidType).field("LoggerId", &self.LoggerId).field("EnableLevel", &self.EnableLevel).field("EnableFlags", &self.EnableFlags).field("IsEnable", &self.IsEnable).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for TRACE_GUID_PROPERTIES {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TRACE_GUID_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TRACE_GUID_PROPERTIES>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TRACE_GUID_PROPERTIES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TRACE_GUID_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TRACE_GUID_REGISTRATION {
    pub Guid: *const ::windows::core::GUID,
    pub RegHandle: super::super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TRACE_GUID_REGISTRATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TRACE_GUID_REGISTRATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TRACE_GUID_REGISTRATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRACE_GUID_REGISTRATION").field("Guid", &self.Guid).field("RegHandle", &self.RegHandle).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for TRACE_GUID_REGISTRATION {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TRACE_GUID_REGISTRATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TRACE_GUID_REGISTRATION>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TRACE_GUID_REGISTRATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TRACE_GUID_REGISTRATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TRACE_HEADER_FLAG_LOG_WNODE: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TRACE_HEADER_FLAG_TRACED_GUID: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TRACE_HEADER_FLAG_USE_GUID_PTR: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TRACE_HEADER_FLAG_USE_MOF_PTR: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TRACE_HEADER_FLAG_USE_TIMESTAMP: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TRACE_LEVEL_CRITICAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TRACE_LEVEL_ERROR: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TRACE_LEVEL_FATAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TRACE_LEVEL_INFORMATION: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TRACE_LEVEL_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TRACE_LEVEL_RESERVED6: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TRACE_LEVEL_RESERVED7: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TRACE_LEVEL_RESERVED8: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TRACE_LEVEL_RESERVED9: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TRACE_LEVEL_VERBOSE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TRACE_LEVEL_WARNING: u32 = 3u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`, `\"Win32_System_Time\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
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
    pub LoggerName: ::windows::core::PWSTR,
    pub LogFileName: ::windows::core::PWSTR,
    pub TimeZone: super::super::Time::TIME_ZONE_INFORMATION,
    pub BootTime: i64,
    pub PerfFreq: i64,
    pub StartTime: i64,
    pub ReservedFlags: u32,
    pub BuffersLost: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::marker::Copy for TRACE_LOGFILE_HEADER {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::clone::Clone for TRACE_LOGFILE_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
unsafe impl ::windows::core::Abi for TRACE_LOGFILE_HEADER {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::cmp::PartialEq for TRACE_LOGFILE_HEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TRACE_LOGFILE_HEADER>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::cmp::Eq for TRACE_LOGFILE_HEADER {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::default::Default for TRACE_LOGFILE_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`, `\"Win32_System_Time\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
pub union TRACE_LOGFILE_HEADER_0 {
    pub Version: u32,
    pub VersionDetail: TRACE_LOGFILE_HEADER_0_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::marker::Copy for TRACE_LOGFILE_HEADER_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::clone::Clone for TRACE_LOGFILE_HEADER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
unsafe impl ::windows::core::Abi for TRACE_LOGFILE_HEADER_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::cmp::PartialEq for TRACE_LOGFILE_HEADER_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TRACE_LOGFILE_HEADER_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::cmp::Eq for TRACE_LOGFILE_HEADER_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::default::Default for TRACE_LOGFILE_HEADER_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`, `\"Win32_System_Time\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
pub struct TRACE_LOGFILE_HEADER_0_0 {
    pub MajorVersion: u8,
    pub MinorVersion: u8,
    pub SubVersion: u8,
    pub SubMinorVersion: u8,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::marker::Copy for TRACE_LOGFILE_HEADER_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::clone::Clone for TRACE_LOGFILE_HEADER_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::fmt::Debug for TRACE_LOGFILE_HEADER_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRACE_LOGFILE_HEADER_0_0").field("MajorVersion", &self.MajorVersion).field("MinorVersion", &self.MinorVersion).field("SubVersion", &self.SubVersion).field("SubMinorVersion", &self.SubMinorVersion).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
unsafe impl ::windows::core::Abi for TRACE_LOGFILE_HEADER_0_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::cmp::PartialEq for TRACE_LOGFILE_HEADER_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TRACE_LOGFILE_HEADER_0_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::cmp::Eq for TRACE_LOGFILE_HEADER_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::default::Default for TRACE_LOGFILE_HEADER_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`, `\"Win32_System_Time\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
pub union TRACE_LOGFILE_HEADER_1 {
    pub LogInstanceGuid: ::windows::core::GUID,
    pub Anonymous: TRACE_LOGFILE_HEADER_1_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::marker::Copy for TRACE_LOGFILE_HEADER_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::clone::Clone for TRACE_LOGFILE_HEADER_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
unsafe impl ::windows::core::Abi for TRACE_LOGFILE_HEADER_1 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::cmp::PartialEq for TRACE_LOGFILE_HEADER_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TRACE_LOGFILE_HEADER_1>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::cmp::Eq for TRACE_LOGFILE_HEADER_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::default::Default for TRACE_LOGFILE_HEADER_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`, `\"Win32_System_Time\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
pub struct TRACE_LOGFILE_HEADER_1_0 {
    pub StartBuffers: u32,
    pub PointerSize: u32,
    pub EventsLost: u32,
    pub CpuSpeedInMHz: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::marker::Copy for TRACE_LOGFILE_HEADER_1_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::clone::Clone for TRACE_LOGFILE_HEADER_1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::fmt::Debug for TRACE_LOGFILE_HEADER_1_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRACE_LOGFILE_HEADER_1_0").field("StartBuffers", &self.StartBuffers).field("PointerSize", &self.PointerSize).field("EventsLost", &self.EventsLost).field("CpuSpeedInMHz", &self.CpuSpeedInMHz).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
unsafe impl ::windows::core::Abi for TRACE_LOGFILE_HEADER_1_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::cmp::PartialEq for TRACE_LOGFILE_HEADER_1_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TRACE_LOGFILE_HEADER_1_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::cmp::Eq for TRACE_LOGFILE_HEADER_1_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::default::Default for TRACE_LOGFILE_HEADER_1_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`, `\"Win32_System_Time\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
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
impl ::core::marker::Copy for TRACE_LOGFILE_HEADER32 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::clone::Clone for TRACE_LOGFILE_HEADER32 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
unsafe impl ::windows::core::Abi for TRACE_LOGFILE_HEADER32 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::cmp::PartialEq for TRACE_LOGFILE_HEADER32 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TRACE_LOGFILE_HEADER32>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::cmp::Eq for TRACE_LOGFILE_HEADER32 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::default::Default for TRACE_LOGFILE_HEADER32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`, `\"Win32_System_Time\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
pub union TRACE_LOGFILE_HEADER32_0 {
    pub Version: u32,
    pub VersionDetail: TRACE_LOGFILE_HEADER32_0_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::marker::Copy for TRACE_LOGFILE_HEADER32_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::clone::Clone for TRACE_LOGFILE_HEADER32_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
unsafe impl ::windows::core::Abi for TRACE_LOGFILE_HEADER32_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::cmp::PartialEq for TRACE_LOGFILE_HEADER32_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TRACE_LOGFILE_HEADER32_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::cmp::Eq for TRACE_LOGFILE_HEADER32_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::default::Default for TRACE_LOGFILE_HEADER32_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`, `\"Win32_System_Time\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
pub struct TRACE_LOGFILE_HEADER32_0_0 {
    pub MajorVersion: u8,
    pub MinorVersion: u8,
    pub SubVersion: u8,
    pub SubMinorVersion: u8,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::marker::Copy for TRACE_LOGFILE_HEADER32_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::clone::Clone for TRACE_LOGFILE_HEADER32_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::fmt::Debug for TRACE_LOGFILE_HEADER32_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRACE_LOGFILE_HEADER32_0_0").field("MajorVersion", &self.MajorVersion).field("MinorVersion", &self.MinorVersion).field("SubVersion", &self.SubVersion).field("SubMinorVersion", &self.SubMinorVersion).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
unsafe impl ::windows::core::Abi for TRACE_LOGFILE_HEADER32_0_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::cmp::PartialEq for TRACE_LOGFILE_HEADER32_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TRACE_LOGFILE_HEADER32_0_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::cmp::Eq for TRACE_LOGFILE_HEADER32_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::default::Default for TRACE_LOGFILE_HEADER32_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`, `\"Win32_System_Time\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
pub union TRACE_LOGFILE_HEADER32_1 {
    pub LogInstanceGuid: ::windows::core::GUID,
    pub Anonymous: TRACE_LOGFILE_HEADER32_1_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::marker::Copy for TRACE_LOGFILE_HEADER32_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::clone::Clone for TRACE_LOGFILE_HEADER32_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
unsafe impl ::windows::core::Abi for TRACE_LOGFILE_HEADER32_1 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::cmp::PartialEq for TRACE_LOGFILE_HEADER32_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TRACE_LOGFILE_HEADER32_1>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::cmp::Eq for TRACE_LOGFILE_HEADER32_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::default::Default for TRACE_LOGFILE_HEADER32_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`, `\"Win32_System_Time\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
pub struct TRACE_LOGFILE_HEADER32_1_0 {
    pub StartBuffers: u32,
    pub PointerSize: u32,
    pub EventsLost: u32,
    pub CpuSpeedInMHz: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::marker::Copy for TRACE_LOGFILE_HEADER32_1_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::clone::Clone for TRACE_LOGFILE_HEADER32_1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::fmt::Debug for TRACE_LOGFILE_HEADER32_1_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRACE_LOGFILE_HEADER32_1_0").field("StartBuffers", &self.StartBuffers).field("PointerSize", &self.PointerSize).field("EventsLost", &self.EventsLost).field("CpuSpeedInMHz", &self.CpuSpeedInMHz).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
unsafe impl ::windows::core::Abi for TRACE_LOGFILE_HEADER32_1_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::cmp::PartialEq for TRACE_LOGFILE_HEADER32_1_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TRACE_LOGFILE_HEADER32_1_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::cmp::Eq for TRACE_LOGFILE_HEADER32_1_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::default::Default for TRACE_LOGFILE_HEADER32_1_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`, `\"Win32_System_Time\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
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
impl ::core::marker::Copy for TRACE_LOGFILE_HEADER64 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::clone::Clone for TRACE_LOGFILE_HEADER64 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
unsafe impl ::windows::core::Abi for TRACE_LOGFILE_HEADER64 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::cmp::PartialEq for TRACE_LOGFILE_HEADER64 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TRACE_LOGFILE_HEADER64>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::cmp::Eq for TRACE_LOGFILE_HEADER64 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::default::Default for TRACE_LOGFILE_HEADER64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`, `\"Win32_System_Time\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
pub union TRACE_LOGFILE_HEADER64_0 {
    pub Version: u32,
    pub VersionDetail: TRACE_LOGFILE_HEADER64_0_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::marker::Copy for TRACE_LOGFILE_HEADER64_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::clone::Clone for TRACE_LOGFILE_HEADER64_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
unsafe impl ::windows::core::Abi for TRACE_LOGFILE_HEADER64_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::cmp::PartialEq for TRACE_LOGFILE_HEADER64_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TRACE_LOGFILE_HEADER64_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::cmp::Eq for TRACE_LOGFILE_HEADER64_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::default::Default for TRACE_LOGFILE_HEADER64_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`, `\"Win32_System_Time\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
pub struct TRACE_LOGFILE_HEADER64_0_0 {
    pub MajorVersion: u8,
    pub MinorVersion: u8,
    pub SubVersion: u8,
    pub SubMinorVersion: u8,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::marker::Copy for TRACE_LOGFILE_HEADER64_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::clone::Clone for TRACE_LOGFILE_HEADER64_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::fmt::Debug for TRACE_LOGFILE_HEADER64_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRACE_LOGFILE_HEADER64_0_0").field("MajorVersion", &self.MajorVersion).field("MinorVersion", &self.MinorVersion).field("SubVersion", &self.SubVersion).field("SubMinorVersion", &self.SubMinorVersion).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
unsafe impl ::windows::core::Abi for TRACE_LOGFILE_HEADER64_0_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::cmp::PartialEq for TRACE_LOGFILE_HEADER64_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TRACE_LOGFILE_HEADER64_0_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::cmp::Eq for TRACE_LOGFILE_HEADER64_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::default::Default for TRACE_LOGFILE_HEADER64_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`, `\"Win32_System_Time\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
pub union TRACE_LOGFILE_HEADER64_1 {
    pub LogInstanceGuid: ::windows::core::GUID,
    pub Anonymous: TRACE_LOGFILE_HEADER64_1_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::marker::Copy for TRACE_LOGFILE_HEADER64_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::clone::Clone for TRACE_LOGFILE_HEADER64_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
unsafe impl ::windows::core::Abi for TRACE_LOGFILE_HEADER64_1 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::cmp::PartialEq for TRACE_LOGFILE_HEADER64_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TRACE_LOGFILE_HEADER64_1>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::cmp::Eq for TRACE_LOGFILE_HEADER64_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::default::Default for TRACE_LOGFILE_HEADER64_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`, `\"Win32_System_Time\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
pub struct TRACE_LOGFILE_HEADER64_1_0 {
    pub StartBuffers: u32,
    pub PointerSize: u32,
    pub EventsLost: u32,
    pub CpuSpeedInMHz: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::marker::Copy for TRACE_LOGFILE_HEADER64_1_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::clone::Clone for TRACE_LOGFILE_HEADER64_1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::fmt::Debug for TRACE_LOGFILE_HEADER64_1_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRACE_LOGFILE_HEADER64_1_0").field("StartBuffers", &self.StartBuffers).field("PointerSize", &self.PointerSize).field("EventsLost", &self.EventsLost).field("CpuSpeedInMHz", &self.CpuSpeedInMHz).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
unsafe impl ::windows::core::Abi for TRACE_LOGFILE_HEADER64_1_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::cmp::PartialEq for TRACE_LOGFILE_HEADER64_1_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TRACE_LOGFILE_HEADER64_1_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::cmp::Eq for TRACE_LOGFILE_HEADER64_1_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::default::Default for TRACE_LOGFILE_HEADER64_1_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TRACE_MESSAGE_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TRACE_MESSAGE_COMPONENTID: TRACE_MESSAGE_FLAGS = TRACE_MESSAGE_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TRACE_MESSAGE_GUID: TRACE_MESSAGE_FLAGS = TRACE_MESSAGE_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TRACE_MESSAGE_SEQUENCE: TRACE_MESSAGE_FLAGS = TRACE_MESSAGE_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TRACE_MESSAGE_SYSTEMINFO: TRACE_MESSAGE_FLAGS = TRACE_MESSAGE_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TRACE_MESSAGE_TIMESTAMP: TRACE_MESSAGE_FLAGS = TRACE_MESSAGE_FLAGS(8u32);
impl ::core::marker::Copy for TRACE_MESSAGE_FLAGS {}
impl ::core::clone::Clone for TRACE_MESSAGE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TRACE_MESSAGE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TRACE_MESSAGE_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for TRACE_MESSAGE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TRACE_MESSAGE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for TRACE_MESSAGE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for TRACE_MESSAGE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for TRACE_MESSAGE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for TRACE_MESSAGE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for TRACE_MESSAGE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TRACE_MESSAGE_FLAG_MASK: u32 = 65535u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TRACE_MESSAGE_PERFORMANCE_TIMESTAMP: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TRACE_MESSAGE_POINTER32: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TRACE_MESSAGE_POINTER64: u32 = 128u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct TRACE_PERIODIC_CAPTURE_STATE_INFO {
    pub CaptureStateFrequencyInSeconds: u32,
    pub ProviderCount: u16,
    pub Reserved: u16,
}
impl ::core::marker::Copy for TRACE_PERIODIC_CAPTURE_STATE_INFO {}
impl ::core::clone::Clone for TRACE_PERIODIC_CAPTURE_STATE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TRACE_PERIODIC_CAPTURE_STATE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRACE_PERIODIC_CAPTURE_STATE_INFO").field("CaptureStateFrequencyInSeconds", &self.CaptureStateFrequencyInSeconds).field("ProviderCount", &self.ProviderCount).field("Reserved", &self.Reserved).finish()
    }
}
unsafe impl ::windows::core::Abi for TRACE_PERIODIC_CAPTURE_STATE_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TRACE_PERIODIC_CAPTURE_STATE_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TRACE_PERIODIC_CAPTURE_STATE_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for TRACE_PERIODIC_CAPTURE_STATE_INFO {}
impl ::core::default::Default for TRACE_PERIODIC_CAPTURE_STATE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct TRACE_PROFILE_INTERVAL {
    pub Source: u32,
    pub Interval: u32,
}
impl ::core::marker::Copy for TRACE_PROFILE_INTERVAL {}
impl ::core::clone::Clone for TRACE_PROFILE_INTERVAL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TRACE_PROFILE_INTERVAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRACE_PROFILE_INTERVAL").field("Source", &self.Source).field("Interval", &self.Interval).finish()
    }
}
unsafe impl ::windows::core::Abi for TRACE_PROFILE_INTERVAL {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TRACE_PROFILE_INTERVAL {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TRACE_PROFILE_INTERVAL>()) == 0 }
    }
}
impl ::core::cmp::Eq for TRACE_PROFILE_INTERVAL {}
impl ::core::default::Default for TRACE_PROFILE_INTERVAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TRACE_PROVIDER_FLAG_LEGACY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TRACE_PROVIDER_FLAG_PRE_ENABLE: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct TRACE_PROVIDER_INFO {
    pub ProviderGuid: ::windows::core::GUID,
    pub SchemaSource: u32,
    pub ProviderNameOffset: u32,
}
impl ::core::marker::Copy for TRACE_PROVIDER_INFO {}
impl ::core::clone::Clone for TRACE_PROVIDER_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TRACE_PROVIDER_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRACE_PROVIDER_INFO").field("ProviderGuid", &self.ProviderGuid).field("SchemaSource", &self.SchemaSource).field("ProviderNameOffset", &self.ProviderNameOffset).finish()
    }
}
unsafe impl ::windows::core::Abi for TRACE_PROVIDER_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TRACE_PROVIDER_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TRACE_PROVIDER_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for TRACE_PROVIDER_INFO {}
impl ::core::default::Default for TRACE_PROVIDER_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct TRACE_PROVIDER_INSTANCE_INFO {
    pub NextOffset: u32,
    pub EnableCount: u32,
    pub Pid: u32,
    pub Flags: u32,
}
impl ::core::marker::Copy for TRACE_PROVIDER_INSTANCE_INFO {}
impl ::core::clone::Clone for TRACE_PROVIDER_INSTANCE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TRACE_PROVIDER_INSTANCE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRACE_PROVIDER_INSTANCE_INFO").field("NextOffset", &self.NextOffset).field("EnableCount", &self.EnableCount).field("Pid", &self.Pid).field("Flags", &self.Flags).finish()
    }
}
unsafe impl ::windows::core::Abi for TRACE_PROVIDER_INSTANCE_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TRACE_PROVIDER_INSTANCE_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TRACE_PROVIDER_INSTANCE_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for TRACE_PROVIDER_INSTANCE_INFO {}
impl ::core::default::Default for TRACE_PROVIDER_INSTANCE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TRACE_QUERY_INFO_CLASS(pub i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TraceGuidQueryList: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(0i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TraceGuidQueryInfo: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(1i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TraceGuidQueryProcess: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(2i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TraceStackTracingInfo: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(3i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TraceSystemTraceEnableFlagsInfo: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(4i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TraceSampledProfileIntervalInfo: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(5i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TraceProfileSourceConfigInfo: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(6i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TraceProfileSourceListInfo: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(7i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TracePmcEventListInfo: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(8i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TracePmcCounterListInfo: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(9i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TraceSetDisallowList: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(10i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TraceVersionInfo: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(11i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TraceGroupQueryList: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(12i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TraceGroupQueryInfo: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(13i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TraceDisallowListQuery: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(14i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TraceInfoReserved15: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(15i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TracePeriodicCaptureStateListInfo: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(16i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TracePeriodicCaptureStateInfo: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(17i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TraceProviderBinaryTracking: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(18i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TraceMaxLoggersQuery: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(19i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TraceLbrConfigurationInfo: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(20i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TraceLbrEventListInfo: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(21i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TraceMaxPmcCounterQuery: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(22i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TraceStreamCount: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(23i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TraceStackCachingInfo: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(24i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TracePmcCounterOwners: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(25i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TraceUnifiedStackCachingInfo: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(26i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const MaxTraceSetInfoClass: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(27i32);
impl ::core::marker::Copy for TRACE_QUERY_INFO_CLASS {}
impl ::core::clone::Clone for TRACE_QUERY_INFO_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TRACE_QUERY_INFO_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TRACE_QUERY_INFO_CLASS {
    type Abi = Self;
}
impl ::core::fmt::Debug for TRACE_QUERY_INFO_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TRACE_QUERY_INFO_CLASS").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TRACE_STACK_CACHING_INFO {
    pub Enabled: super::super::super::Foundation::BOOLEAN,
    pub CacheSize: u32,
    pub BucketCount: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TRACE_STACK_CACHING_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TRACE_STACK_CACHING_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TRACE_STACK_CACHING_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRACE_STACK_CACHING_INFO").field("Enabled", &self.Enabled).field("CacheSize", &self.CacheSize).field("BucketCount", &self.BucketCount).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for TRACE_STACK_CACHING_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TRACE_STACK_CACHING_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TRACE_STACK_CACHING_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TRACE_STACK_CACHING_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TRACE_STACK_CACHING_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct TRACE_VERSION_INFO {
    pub EtwTraceProcessingVersion: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for TRACE_VERSION_INFO {}
impl ::core::clone::Clone for TRACE_VERSION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TRACE_VERSION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRACE_VERSION_INFO").field("EtwTraceProcessingVersion", &self.EtwTraceProcessingVersion).field("Reserved", &self.Reserved).finish()
    }
}
unsafe impl ::windows::core::Abi for TRACE_VERSION_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TRACE_VERSION_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TRACE_VERSION_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for TRACE_VERSION_INFO {}
impl ::core::default::Default for TRACE_VERSION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TdhAggregatePayloadFilters(payloadfiltercount: u32, payloadfilterptrs: *const *const ::core::ffi::c_void, eventmatchallflags: *const super::super::super::Foundation::BOOLEAN, eventfilterdescriptor: *mut EVENT_FILTER_DESCRIPTOR) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn TdhAggregatePayloadFilters(payloadfiltercount: u32, payloadfilterptrs: *const *const ::core::ffi::c_void, eventmatchallflags: *const super::super::super::Foundation::BOOLEAN, eventfilterdescriptor: *mut EVENT_FILTER_DESCRIPTOR) -> u32;
    }
    TdhAggregatePayloadFilters(::core::mem::transmute(payloadfiltercount), ::core::mem::transmute(payloadfilterptrs), ::core::mem::transmute(eventmatchallflags), ::core::mem::transmute(eventfilterdescriptor))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
#[inline]
pub unsafe fn TdhCleanupPayloadEventFilterDescriptor(eventfilterdescriptor: *mut EVENT_FILTER_DESCRIPTOR) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn TdhCleanupPayloadEventFilterDescriptor(eventfilterdescriptor: *mut EVENT_FILTER_DESCRIPTOR) -> u32;
    }
    TdhCleanupPayloadEventFilterDescriptor(::core::mem::transmute(eventfilterdescriptor))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
#[inline]
pub unsafe fn TdhCloseDecodingHandle<'a, P0>(handle: P0) -> u32
where
    P0: ::std::convert::Into<TDH_HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn TdhCloseDecodingHandle(handle: TDH_HANDLE) -> u32;
    }
    TdhCloseDecodingHandle(handle.into())
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TdhCreatePayloadFilter<'a, P0>(providerguid: *const ::windows::core::GUID, eventdescriptor: *const EVENT_DESCRIPTOR, eventmatchany: P0, payloadpredicates: &[PAYLOAD_FILTER_PREDICATE], payloadfilter: *mut *mut ::core::ffi::c_void) -> u32
where
    P0: ::std::convert::Into<super::super::super::Foundation::BOOLEAN>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn TdhCreatePayloadFilter(providerguid: *const ::windows::core::GUID, eventdescriptor: *const EVENT_DESCRIPTOR, eventmatchany: super::super::super::Foundation::BOOLEAN, payloadpredicatecount: u32, payloadpredicates: *const PAYLOAD_FILTER_PREDICATE, payloadfilter: *mut *mut ::core::ffi::c_void) -> u32;
    }
    TdhCreatePayloadFilter(::core::mem::transmute(providerguid), ::core::mem::transmute(eventdescriptor), eventmatchany.into(), payloadpredicates.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(payloadpredicates)), ::core::mem::transmute(payloadfilter))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
#[inline]
pub unsafe fn TdhDeletePayloadFilter(payloadfilter: *mut *mut ::core::ffi::c_void) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn TdhDeletePayloadFilter(payloadfilter: *mut *mut ::core::ffi::c_void) -> u32;
    }
    TdhDeletePayloadFilter(::core::mem::transmute(payloadfilter))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
#[inline]
pub unsafe fn TdhEnumerateManifestProviderEvents(providerguid: *const ::windows::core::GUID, buffer: *mut PROVIDER_EVENT_INFO, buffersize: *mut u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn TdhEnumerateManifestProviderEvents(providerguid: *const ::windows::core::GUID, buffer: *mut PROVIDER_EVENT_INFO, buffersize: *mut u32) -> u32;
    }
    TdhEnumerateManifestProviderEvents(::core::mem::transmute(providerguid), ::core::mem::transmute(buffer), ::core::mem::transmute(buffersize))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
#[inline]
pub unsafe fn TdhEnumerateProviderFieldInformation(pguid: *const ::windows::core::GUID, eventfieldtype: EVENT_FIELD_TYPE, pbuffer: *mut PROVIDER_FIELD_INFOARRAY, pbuffersize: *mut u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn TdhEnumerateProviderFieldInformation(pguid: *const ::windows::core::GUID, eventfieldtype: EVENT_FIELD_TYPE, pbuffer: *mut PROVIDER_FIELD_INFOARRAY, pbuffersize: *mut u32) -> u32;
    }
    TdhEnumerateProviderFieldInformation(::core::mem::transmute(pguid), eventfieldtype, ::core::mem::transmute(pbuffer), ::core::mem::transmute(pbuffersize))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
#[inline]
pub unsafe fn TdhEnumerateProviderFilters(guid: *const ::windows::core::GUID, tdhcontext: &[TDH_CONTEXT], filtercount: *mut u32, buffer: *mut *mut PROVIDER_FILTER_INFO, buffersize: *mut u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn TdhEnumerateProviderFilters(guid: *const ::windows::core::GUID, tdhcontextcount: u32, tdhcontext: *const TDH_CONTEXT, filtercount: *mut u32, buffer: *mut *mut PROVIDER_FILTER_INFO, buffersize: *mut u32) -> u32;
    }
    TdhEnumerateProviderFilters(::core::mem::transmute(guid), tdhcontext.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(tdhcontext)), ::core::mem::transmute(filtercount), ::core::mem::transmute(buffer), ::core::mem::transmute(buffersize))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
#[inline]
pub unsafe fn TdhEnumerateProviders(pbuffer: *mut PROVIDER_ENUMERATION_INFO, pbuffersize: *mut u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn TdhEnumerateProviders(pbuffer: *mut PROVIDER_ENUMERATION_INFO, pbuffersize: *mut u32) -> u32;
    }
    TdhEnumerateProviders(::core::mem::transmute(pbuffer), ::core::mem::transmute(pbuffersize))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
#[inline]
pub unsafe fn TdhEnumerateProvidersForDecodingSource(filter: DECODING_SOURCE, buffer: *mut PROVIDER_ENUMERATION_INFO, buffersize: u32, bufferrequired: *mut u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn TdhEnumerateProvidersForDecodingSource(filter: DECODING_SOURCE, buffer: *mut PROVIDER_ENUMERATION_INFO, buffersize: u32, bufferrequired: *mut u32) -> u32;
    }
    TdhEnumerateProvidersForDecodingSource(filter, ::core::mem::transmute(buffer), buffersize, ::core::mem::transmute(bufferrequired))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
#[inline]
pub unsafe fn TdhFormatProperty(eventinfo: *const TRACE_EVENT_INFO, mapinfo: *const EVENT_MAP_INFO, pointersize: u32, propertyintype: u16, propertyouttype: u16, propertylength: u16, userdatalength: u16, userdata: *const u8, buffersize: *mut u32, buffer: ::windows::core::PWSTR, userdataconsumed: *mut u16) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn TdhFormatProperty(eventinfo: *const TRACE_EVENT_INFO, mapinfo: *const EVENT_MAP_INFO, pointersize: u32, propertyintype: u16, propertyouttype: u16, propertylength: u16, userdatalength: u16, userdata: *const u8, buffersize: *mut u32, buffer: ::windows::core::PWSTR, userdataconsumed: *mut u16) -> u32;
    }
    TdhFormatProperty(::core::mem::transmute(eventinfo), ::core::mem::transmute(mapinfo), pointersize, propertyintype, propertyouttype, propertylength, userdatalength, ::core::mem::transmute(userdata), ::core::mem::transmute(buffersize), ::core::mem::transmute(buffer), ::core::mem::transmute(userdataconsumed))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
#[inline]
pub unsafe fn TdhGetDecodingParameter<'a, P0>(handle: P0, tdhcontext: *mut TDH_CONTEXT) -> u32
where
    P0: ::std::convert::Into<TDH_HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn TdhGetDecodingParameter(handle: TDH_HANDLE, tdhcontext: *mut TDH_CONTEXT) -> u32;
    }
    TdhGetDecodingParameter(handle.into(), ::core::mem::transmute(tdhcontext))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
#[inline]
pub unsafe fn TdhGetEventInformation(event: *const EVENT_RECORD, tdhcontext: &[TDH_CONTEXT], buffer: *mut TRACE_EVENT_INFO, buffersize: *mut u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn TdhGetEventInformation(event: *const EVENT_RECORD, tdhcontextcount: u32, tdhcontext: *const TDH_CONTEXT, buffer: *mut TRACE_EVENT_INFO, buffersize: *mut u32) -> u32;
    }
    TdhGetEventInformation(::core::mem::transmute(event), tdhcontext.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(tdhcontext)), ::core::mem::transmute(buffer), ::core::mem::transmute(buffersize))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
#[inline]
pub unsafe fn TdhGetEventMapInformation<'a, P0>(pevent: *const EVENT_RECORD, pmapname: P0, pbuffer: *mut EVENT_MAP_INFO, pbuffersize: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn TdhGetEventMapInformation(pevent: *const EVENT_RECORD, pmapname: ::windows::core::PCWSTR, pbuffer: *mut EVENT_MAP_INFO, pbuffersize: *mut u32) -> u32;
    }
    TdhGetEventMapInformation(::core::mem::transmute(pevent), pmapname.into(), ::core::mem::transmute(pbuffer), ::core::mem::transmute(pbuffersize))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
#[inline]
pub unsafe fn TdhGetManifestEventInformation(providerguid: *const ::windows::core::GUID, eventdescriptor: *const EVENT_DESCRIPTOR, buffer: *mut TRACE_EVENT_INFO, buffersize: *mut u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn TdhGetManifestEventInformation(providerguid: *const ::windows::core::GUID, eventdescriptor: *const EVENT_DESCRIPTOR, buffer: *mut TRACE_EVENT_INFO, buffersize: *mut u32) -> u32;
    }
    TdhGetManifestEventInformation(::core::mem::transmute(providerguid), ::core::mem::transmute(eventdescriptor), ::core::mem::transmute(buffer), ::core::mem::transmute(buffersize))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
#[inline]
pub unsafe fn TdhGetProperty(pevent: *const EVENT_RECORD, ptdhcontext: &[TDH_CONTEXT], ppropertydata: &[PROPERTY_DATA_DESCRIPTOR], buffersize: u32, pbuffer: *mut u8) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn TdhGetProperty(pevent: *const EVENT_RECORD, tdhcontextcount: u32, ptdhcontext: *const TDH_CONTEXT, propertydatacount: u32, ppropertydata: *const PROPERTY_DATA_DESCRIPTOR, buffersize: u32, pbuffer: *mut u8) -> u32;
    }
    TdhGetProperty(::core::mem::transmute(pevent), ptdhcontext.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(ptdhcontext)), ppropertydata.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(ppropertydata)), buffersize, ::core::mem::transmute(pbuffer))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
#[inline]
pub unsafe fn TdhGetPropertySize(pevent: *const EVENT_RECORD, ptdhcontext: &[TDH_CONTEXT], ppropertydata: &[PROPERTY_DATA_DESCRIPTOR], ppropertysize: *mut u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn TdhGetPropertySize(pevent: *const EVENT_RECORD, tdhcontextcount: u32, ptdhcontext: *const TDH_CONTEXT, propertydatacount: u32, ppropertydata: *const PROPERTY_DATA_DESCRIPTOR, ppropertysize: *mut u32) -> u32;
    }
    TdhGetPropertySize(::core::mem::transmute(pevent), ptdhcontext.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(ptdhcontext)), ppropertydata.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(ppropertydata)), ::core::mem::transmute(ppropertysize))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
#[inline]
pub unsafe fn TdhGetWppMessage<'a, P0>(handle: P0, eventrecord: *const EVENT_RECORD, buffersize: *mut u32, buffer: *mut u8) -> u32
where
    P0: ::std::convert::Into<TDH_HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn TdhGetWppMessage(handle: TDH_HANDLE, eventrecord: *const EVENT_RECORD, buffersize: *mut u32, buffer: *mut u8) -> u32;
    }
    TdhGetWppMessage(handle.into(), ::core::mem::transmute(eventrecord), ::core::mem::transmute(buffersize), ::core::mem::transmute(buffer))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
#[inline]
pub unsafe fn TdhGetWppProperty<'a, P0, P1>(handle: P0, eventrecord: *const EVENT_RECORD, propertyname: P1, buffersize: *mut u32, buffer: *mut u8) -> u32
where
    P0: ::std::convert::Into<TDH_HANDLE>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn TdhGetWppProperty(handle: TDH_HANDLE, eventrecord: *const EVENT_RECORD, propertyname: ::windows::core::PCWSTR, buffersize: *mut u32, buffer: *mut u8) -> u32;
    }
    TdhGetWppProperty(handle.into(), ::core::mem::transmute(eventrecord), propertyname.into(), ::core::mem::transmute(buffersize), ::core::mem::transmute(buffer))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
#[inline]
pub unsafe fn TdhLoadManifest<'a, P0>(manifest: P0) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn TdhLoadManifest(manifest: ::windows::core::PCWSTR) -> u32;
    }
    TdhLoadManifest(manifest.into())
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
#[inline]
pub unsafe fn TdhLoadManifestFromBinary<'a, P0>(binarypath: P0) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn TdhLoadManifestFromBinary(binarypath: ::windows::core::PCWSTR) -> u32;
    }
    TdhLoadManifestFromBinary(binarypath.into())
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
#[inline]
pub unsafe fn TdhLoadManifestFromMemory(pdata: *const ::core::ffi::c_void, cbdata: u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn TdhLoadManifestFromMemory(pdata: *const ::core::ffi::c_void, cbdata: u32) -> u32;
    }
    TdhLoadManifestFromMemory(::core::mem::transmute(pdata), cbdata)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
#[inline]
pub unsafe fn TdhOpenDecodingHandle(handle: *mut TDH_HANDLE) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn TdhOpenDecodingHandle(handle: *mut TDH_HANDLE) -> u32;
    }
    TdhOpenDecodingHandle(::core::mem::transmute(handle))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
#[inline]
pub unsafe fn TdhQueryProviderFieldInformation(pguid: *const ::windows::core::GUID, eventfieldvalue: u64, eventfieldtype: EVENT_FIELD_TYPE, pbuffer: *mut PROVIDER_FIELD_INFOARRAY, pbuffersize: *mut u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn TdhQueryProviderFieldInformation(pguid: *const ::windows::core::GUID, eventfieldvalue: u64, eventfieldtype: EVENT_FIELD_TYPE, pbuffer: *mut PROVIDER_FIELD_INFOARRAY, pbuffersize: *mut u32) -> u32;
    }
    TdhQueryProviderFieldInformation(::core::mem::transmute(pguid), eventfieldvalue, eventfieldtype, ::core::mem::transmute(pbuffer), ::core::mem::transmute(pbuffersize))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
#[inline]
pub unsafe fn TdhSetDecodingParameter<'a, P0>(handle: P0, tdhcontext: *const TDH_CONTEXT) -> u32
where
    P0: ::std::convert::Into<TDH_HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn TdhSetDecodingParameter(handle: TDH_HANDLE, tdhcontext: *const TDH_CONTEXT) -> u32;
    }
    TdhSetDecodingParameter(handle.into(), ::core::mem::transmute(tdhcontext))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
#[inline]
pub unsafe fn TdhUnloadManifest<'a, P0>(manifest: P0) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn TdhUnloadManifest(manifest: ::windows::core::PCWSTR) -> u32;
    }
    TdhUnloadManifest(manifest.into())
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
#[inline]
pub unsafe fn TdhUnloadManifestFromMemory(pdata: *const ::core::ffi::c_void, cbdata: u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn TdhUnloadManifestFromMemory(pdata: *const ::core::ffi::c_void, cbdata: u32) -> u32;
    }
    TdhUnloadManifestFromMemory(::core::mem::transmute(pdata), cbdata)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
#[inline]
pub unsafe fn TraceEvent(tracehandle: u64, eventtrace: *const EVENT_TRACE_HEADER) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn TraceEvent(tracehandle: u64, eventtrace: *const EVENT_TRACE_HEADER) -> u32;
    }
    TraceEvent(tracehandle, ::core::mem::transmute(eventtrace))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TraceEventInstance(tracehandle: u64, eventtrace: *const EVENT_INSTANCE_HEADER, instinfo: *const EVENT_INSTANCE_INFO, parentinstinfo: *const EVENT_INSTANCE_INFO) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn TraceEventInstance(tracehandle: u64, eventtrace: *const EVENT_INSTANCE_HEADER, instinfo: *const EVENT_INSTANCE_INFO, parentinstinfo: *const EVENT_INSTANCE_INFO) -> u32;
    }
    TraceEventInstance(tracehandle, ::core::mem::transmute(eventtrace), ::core::mem::transmute(instinfo), ::core::mem::transmute(parentinstinfo))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
#[inline]
pub unsafe fn TraceMessage(loggerhandle: u64, messageflags: TRACE_MESSAGE_FLAGS, messageguid: *const ::windows::core::GUID, messagenumber: u16) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn TraceMessage(loggerhandle: u64, messageflags: TRACE_MESSAGE_FLAGS, messageguid: *const ::windows::core::GUID, messagenumber: u16) -> u32;
    }
    TraceMessage(loggerhandle, messageflags, ::core::mem::transmute(messageguid), messagenumber)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
#[inline]
pub unsafe fn TraceMessageVa(loggerhandle: u64, messageflags: TRACE_MESSAGE_FLAGS, messageguid: *const ::windows::core::GUID, messagenumber: u16, messagearglist: *const i8) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn TraceMessageVa(loggerhandle: u64, messageflags: TRACE_MESSAGE_FLAGS, messageguid: *const ::windows::core::GUID, messagenumber: u16, messagearglist: *const i8) -> u32;
    }
    TraceMessageVa(loggerhandle, messageflags, ::core::mem::transmute(messageguid), messagenumber, ::core::mem::transmute(messagearglist))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
#[inline]
pub unsafe fn TraceQueryInformation(sessionhandle: u64, informationclass: TRACE_QUERY_INFO_CLASS, traceinformation: *mut ::core::ffi::c_void, informationlength: u32, returnlength: *mut u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn TraceQueryInformation(sessionhandle: u64, informationclass: TRACE_QUERY_INFO_CLASS, traceinformation: *mut ::core::ffi::c_void, informationlength: u32, returnlength: *mut u32) -> u32;
    }
    TraceQueryInformation(sessionhandle, informationclass, ::core::mem::transmute(traceinformation), informationlength, ::core::mem::transmute(returnlength))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
#[inline]
pub unsafe fn TraceSetInformation(sessionhandle: u64, informationclass: TRACE_QUERY_INFO_CLASS, traceinformation: *const ::core::ffi::c_void, informationlength: u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn TraceSetInformation(sessionhandle: u64, informationclass: TRACE_QUERY_INFO_CLASS, traceinformation: *const ::core::ffi::c_void, informationlength: u32) -> u32;
    }
    TraceSetInformation(sessionhandle, informationclass, ::core::mem::transmute(traceinformation), informationlength)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
#[inline]
pub unsafe fn UnregisterTraceGuids(registrationhandle: u64) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn UnregisterTraceGuids(registrationhandle: u64) -> u32;
    }
    UnregisterTraceGuids(registrationhandle)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UpdateTraceA<'a, P0>(tracehandle: u64, instancename: P0, properties: *mut EVENT_TRACE_PROPERTIES) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn UpdateTraceA(tracehandle: u64, instancename: ::windows::core::PCSTR, properties: *mut EVENT_TRACE_PROPERTIES) -> u32;
    }
    UpdateTraceA(tracehandle, instancename.into(), ::core::mem::transmute(properties))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UpdateTraceW<'a, P0>(tracehandle: u64, instancename: P0, properties: *mut EVENT_TRACE_PROPERTIES) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn UpdateTraceW(tracehandle: u64, instancename: ::windows::core::PCWSTR, properties: *mut EVENT_TRACE_PROPERTIES) -> u32;
    }
    UpdateTraceW(tracehandle, instancename.into(), ::core::mem::transmute(properties))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub type WMIDPREQUEST = ::core::option::Option<unsafe extern "system" fn(requestcode: WMIDPREQUESTCODE, requestcontext: *const ::core::ffi::c_void, buffersize: *mut u32, buffer: *mut ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WMIDPREQUESTCODE(pub i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WMI_GET_ALL_DATA: WMIDPREQUESTCODE = WMIDPREQUESTCODE(0i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WMI_GET_SINGLE_INSTANCE: WMIDPREQUESTCODE = WMIDPREQUESTCODE(1i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WMI_SET_SINGLE_INSTANCE: WMIDPREQUESTCODE = WMIDPREQUESTCODE(2i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WMI_SET_SINGLE_ITEM: WMIDPREQUESTCODE = WMIDPREQUESTCODE(3i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WMI_ENABLE_EVENTS: WMIDPREQUESTCODE = WMIDPREQUESTCODE(4i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WMI_DISABLE_EVENTS: WMIDPREQUESTCODE = WMIDPREQUESTCODE(5i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WMI_ENABLE_COLLECTION: WMIDPREQUESTCODE = WMIDPREQUESTCODE(6i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WMI_DISABLE_COLLECTION: WMIDPREQUESTCODE = WMIDPREQUESTCODE(7i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WMI_REGINFO: WMIDPREQUESTCODE = WMIDPREQUESTCODE(8i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WMI_EXECUTE_METHOD: WMIDPREQUESTCODE = WMIDPREQUESTCODE(9i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WMI_CAPTURE_STATE: WMIDPREQUESTCODE = WMIDPREQUESTCODE(10i32);
impl ::core::marker::Copy for WMIDPREQUESTCODE {}
impl ::core::clone::Clone for WMIDPREQUESTCODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMIDPREQUESTCODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WMIDPREQUESTCODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WMIDPREQUESTCODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMIDPREQUESTCODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WMIGUID_EXECUTE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WMIGUID_NOTIFICATION: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WMIGUID_QUERY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WMIGUID_READ_DESCRIPTION: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WMIGUID_SET: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct WMIREGGUIDW {
    pub Guid: ::windows::core::GUID,
    pub Flags: u32,
    pub InstanceCount: u32,
    pub Anonymous: WMIREGGUIDW_0,
}
impl ::core::marker::Copy for WMIREGGUIDW {}
impl ::core::clone::Clone for WMIREGGUIDW {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WMIREGGUIDW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WMIREGGUIDW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WMIREGGUIDW>()) == 0 }
    }
}
impl ::core::cmp::Eq for WMIREGGUIDW {}
impl ::core::default::Default for WMIREGGUIDW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub union WMIREGGUIDW_0 {
    pub InstanceNameList: u32,
    pub BaseNameOffset: u32,
    pub Pdo: usize,
    pub InstanceInfo: usize,
}
impl ::core::marker::Copy for WMIREGGUIDW_0 {}
impl ::core::clone::Clone for WMIREGGUIDW_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WMIREGGUIDW_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WMIREGGUIDW_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WMIREGGUIDW_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for WMIREGGUIDW_0 {}
impl ::core::default::Default for WMIREGGUIDW_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub struct WMIREGINFOW {
    pub BufferSize: u32,
    pub NextWmiRegInfo: u32,
    pub RegistryPath: u32,
    pub MofResourceName: u32,
    pub GuidCount: u32,
    pub WmiRegGuid: [WMIREGGUIDW; 1],
}
impl ::core::marker::Copy for WMIREGINFOW {}
impl ::core::clone::Clone for WMIREGINFOW {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WMIREGINFOW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WMIREGINFOW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WMIREGINFOW>()) == 0 }
    }
}
impl ::core::cmp::Eq for WMIREGINFOW {}
impl ::core::default::Default for WMIREGINFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WMIREG_FLAG_EVENT_ONLY_GUID: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WMIREG_FLAG_EXPENSIVE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WMIREG_FLAG_INSTANCE_BASENAME: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WMIREG_FLAG_INSTANCE_LIST: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WMIREG_FLAG_INSTANCE_PDO: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WMIREG_FLAG_REMOVE_GUID: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WMIREG_FLAG_RESERVED1: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WMIREG_FLAG_RESERVED2: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WMIREG_FLAG_TRACED_GUID: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WMIREG_FLAG_TRACE_CONTROL_GUID: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WMI_GLOBAL_LOGGER_ID: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WMI_GUIDTYPE_DATA: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WMI_GUIDTYPE_EVENT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WMI_GUIDTYPE_TRACE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WMI_GUIDTYPE_TRACECONTROL: u32 = 0u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WNODE_ALL_DATA {
    pub WnodeHeader: WNODE_HEADER,
    pub DataBlockOffset: u32,
    pub InstanceCount: u32,
    pub OffsetInstanceNameOffsets: u32,
    pub Anonymous: WNODE_ALL_DATA_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WNODE_ALL_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WNODE_ALL_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WNODE_ALL_DATA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WNODE_ALL_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WNODE_ALL_DATA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WNODE_ALL_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WNODE_ALL_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union WNODE_ALL_DATA_0 {
    pub FixedInstanceSize: u32,
    pub OffsetInstanceDataAndLength: [OFFSETINSTANCEDATAANDLENGTH; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WNODE_ALL_DATA_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WNODE_ALL_DATA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WNODE_ALL_DATA_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WNODE_ALL_DATA_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WNODE_ALL_DATA_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WNODE_ALL_DATA_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WNODE_ALL_DATA_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WNODE_EVENT_ITEM {
    pub WnodeHeader: WNODE_HEADER,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WNODE_EVENT_ITEM {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WNODE_EVENT_ITEM {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WNODE_EVENT_ITEM {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WNODE_EVENT_ITEM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WNODE_EVENT_ITEM>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WNODE_EVENT_ITEM {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WNODE_EVENT_ITEM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WNODE_EVENT_REFERENCE {
    pub WnodeHeader: WNODE_HEADER,
    pub TargetGuid: ::windows::core::GUID,
    pub TargetDataBlockSize: u32,
    pub Anonymous: WNODE_EVENT_REFERENCE_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WNODE_EVENT_REFERENCE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WNODE_EVENT_REFERENCE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WNODE_EVENT_REFERENCE {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WNODE_EVENT_REFERENCE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WNODE_EVENT_REFERENCE>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WNODE_EVENT_REFERENCE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WNODE_EVENT_REFERENCE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union WNODE_EVENT_REFERENCE_0 {
    pub TargetInstanceIndex: u32,
    pub TargetInstanceName: [u16; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WNODE_EVENT_REFERENCE_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WNODE_EVENT_REFERENCE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WNODE_EVENT_REFERENCE_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WNODE_EVENT_REFERENCE_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WNODE_EVENT_REFERENCE_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WNODE_EVENT_REFERENCE_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WNODE_EVENT_REFERENCE_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WNODE_FLAG_ALL_DATA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WNODE_FLAG_ANSI_INSTANCENAMES: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WNODE_FLAG_EVENT_ITEM: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WNODE_FLAG_EVENT_REFERENCE: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WNODE_FLAG_FIXED_INSTANCE_SIZE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WNODE_FLAG_INSTANCES_SAME: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WNODE_FLAG_INTERNAL: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WNODE_FLAG_LOG_WNODE: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WNODE_FLAG_METHOD_ITEM: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WNODE_FLAG_NO_HEADER: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WNODE_FLAG_PDO_INSTANCE_NAMES: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WNODE_FLAG_PERSIST_EVENT: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WNODE_FLAG_SEND_DATA_BLOCK: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WNODE_FLAG_SEVERITY_MASK: u32 = 4278190080u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WNODE_FLAG_SINGLE_INSTANCE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WNODE_FLAG_SINGLE_ITEM: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WNODE_FLAG_STATIC_INSTANCE_NAMES: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WNODE_FLAG_TOO_SMALL: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WNODE_FLAG_TRACED_GUID: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WNODE_FLAG_USE_GUID_PTR: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WNODE_FLAG_USE_MOF_PTR: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WNODE_FLAG_USE_TIMESTAMP: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const WNODE_FLAG_VERSIONED_PROPERTIES: u32 = 8388608u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WNODE_HEADER {
    pub BufferSize: u32,
    pub ProviderId: u32,
    pub Anonymous1: WNODE_HEADER_0,
    pub Anonymous2: WNODE_HEADER_1,
    pub Guid: ::windows::core::GUID,
    pub ClientContext: u32,
    pub Flags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WNODE_HEADER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WNODE_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WNODE_HEADER {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WNODE_HEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WNODE_HEADER>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WNODE_HEADER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WNODE_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union WNODE_HEADER_0 {
    pub HistoricalContext: u64,
    pub Anonymous: WNODE_HEADER_0_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WNODE_HEADER_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WNODE_HEADER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WNODE_HEADER_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WNODE_HEADER_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WNODE_HEADER_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WNODE_HEADER_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WNODE_HEADER_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WNODE_HEADER_0_0 {
    pub Version: u32,
    pub Linkage: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WNODE_HEADER_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WNODE_HEADER_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WNODE_HEADER_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WNODE_HEADER_0_0").field("Version", &self.Version).field("Linkage", &self.Linkage).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WNODE_HEADER_0_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WNODE_HEADER_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WNODE_HEADER_0_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WNODE_HEADER_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WNODE_HEADER_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union WNODE_HEADER_1 {
    pub CountLost: u32,
    pub KernelHandle: super::super::super::Foundation::HANDLE,
    pub TimeStamp: i64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WNODE_HEADER_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WNODE_HEADER_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WNODE_HEADER_1 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WNODE_HEADER_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WNODE_HEADER_1>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WNODE_HEADER_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WNODE_HEADER_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
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
impl ::core::marker::Copy for WNODE_METHOD_ITEM {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WNODE_METHOD_ITEM {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WNODE_METHOD_ITEM {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WNODE_METHOD_ITEM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WNODE_METHOD_ITEM>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WNODE_METHOD_ITEM {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WNODE_METHOD_ITEM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WNODE_SINGLE_INSTANCE {
    pub WnodeHeader: WNODE_HEADER,
    pub OffsetInstanceName: u32,
    pub InstanceIndex: u32,
    pub DataBlockOffset: u32,
    pub SizeDataBlock: u32,
    pub VariableData: [u8; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WNODE_SINGLE_INSTANCE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WNODE_SINGLE_INSTANCE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WNODE_SINGLE_INSTANCE {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WNODE_SINGLE_INSTANCE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WNODE_SINGLE_INSTANCE>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WNODE_SINGLE_INSTANCE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WNODE_SINGLE_INSTANCE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
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
impl ::core::marker::Copy for WNODE_SINGLE_ITEM {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WNODE_SINGLE_ITEM {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WNODE_SINGLE_ITEM {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WNODE_SINGLE_ITEM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WNODE_SINGLE_ITEM>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WNODE_SINGLE_ITEM {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WNODE_SINGLE_ITEM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WNODE_TOO_SMALL {
    pub WnodeHeader: WNODE_HEADER,
    pub SizeNeeded: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WNODE_TOO_SMALL {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WNODE_TOO_SMALL {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WNODE_TOO_SMALL {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WNODE_TOO_SMALL {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WNODE_TOO_SMALL>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WNODE_TOO_SMALL {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WNODE_TOO_SMALL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct _TDH_IN_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_INTYPE_NULL: _TDH_IN_TYPE = _TDH_IN_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_INTYPE_UNICODESTRING: _TDH_IN_TYPE = _TDH_IN_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_INTYPE_ANSISTRING: _TDH_IN_TYPE = _TDH_IN_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_INTYPE_INT8: _TDH_IN_TYPE = _TDH_IN_TYPE(3i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_INTYPE_UINT8: _TDH_IN_TYPE = _TDH_IN_TYPE(4i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_INTYPE_INT16: _TDH_IN_TYPE = _TDH_IN_TYPE(5i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_INTYPE_UINT16: _TDH_IN_TYPE = _TDH_IN_TYPE(6i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_INTYPE_INT32: _TDH_IN_TYPE = _TDH_IN_TYPE(7i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_INTYPE_UINT32: _TDH_IN_TYPE = _TDH_IN_TYPE(8i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_INTYPE_INT64: _TDH_IN_TYPE = _TDH_IN_TYPE(9i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_INTYPE_UINT64: _TDH_IN_TYPE = _TDH_IN_TYPE(10i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_INTYPE_FLOAT: _TDH_IN_TYPE = _TDH_IN_TYPE(11i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_INTYPE_DOUBLE: _TDH_IN_TYPE = _TDH_IN_TYPE(12i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_INTYPE_BOOLEAN: _TDH_IN_TYPE = _TDH_IN_TYPE(13i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_INTYPE_BINARY: _TDH_IN_TYPE = _TDH_IN_TYPE(14i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_INTYPE_GUID: _TDH_IN_TYPE = _TDH_IN_TYPE(15i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_INTYPE_POINTER: _TDH_IN_TYPE = _TDH_IN_TYPE(16i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_INTYPE_FILETIME: _TDH_IN_TYPE = _TDH_IN_TYPE(17i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_INTYPE_SYSTEMTIME: _TDH_IN_TYPE = _TDH_IN_TYPE(18i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_INTYPE_SID: _TDH_IN_TYPE = _TDH_IN_TYPE(19i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_INTYPE_HEXINT32: _TDH_IN_TYPE = _TDH_IN_TYPE(20i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_INTYPE_HEXINT64: _TDH_IN_TYPE = _TDH_IN_TYPE(21i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_INTYPE_MANIFEST_COUNTEDSTRING: _TDH_IN_TYPE = _TDH_IN_TYPE(22i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_INTYPE_MANIFEST_COUNTEDANSISTRING: _TDH_IN_TYPE = _TDH_IN_TYPE(23i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_INTYPE_RESERVED24: _TDH_IN_TYPE = _TDH_IN_TYPE(24i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_INTYPE_MANIFEST_COUNTEDBINARY: _TDH_IN_TYPE = _TDH_IN_TYPE(25i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_INTYPE_COUNTEDSTRING: _TDH_IN_TYPE = _TDH_IN_TYPE(300i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_INTYPE_COUNTEDANSISTRING: _TDH_IN_TYPE = _TDH_IN_TYPE(301i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_INTYPE_REVERSEDCOUNTEDSTRING: _TDH_IN_TYPE = _TDH_IN_TYPE(302i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_INTYPE_REVERSEDCOUNTEDANSISTRING: _TDH_IN_TYPE = _TDH_IN_TYPE(303i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_INTYPE_NONNULLTERMINATEDSTRING: _TDH_IN_TYPE = _TDH_IN_TYPE(304i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_INTYPE_NONNULLTERMINATEDANSISTRING: _TDH_IN_TYPE = _TDH_IN_TYPE(305i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_INTYPE_UNICODECHAR: _TDH_IN_TYPE = _TDH_IN_TYPE(306i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_INTYPE_ANSICHAR: _TDH_IN_TYPE = _TDH_IN_TYPE(307i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_INTYPE_SIZET: _TDH_IN_TYPE = _TDH_IN_TYPE(308i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_INTYPE_HEXDUMP: _TDH_IN_TYPE = _TDH_IN_TYPE(309i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_INTYPE_WBEMSID: _TDH_IN_TYPE = _TDH_IN_TYPE(310i32);
impl ::core::marker::Copy for _TDH_IN_TYPE {}
impl ::core::clone::Clone for _TDH_IN_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for _TDH_IN_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for _TDH_IN_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for _TDH_IN_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_TDH_IN_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct _TDH_OUT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_OUTTYPE_NULL: _TDH_OUT_TYPE = _TDH_OUT_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_OUTTYPE_STRING: _TDH_OUT_TYPE = _TDH_OUT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_OUTTYPE_DATETIME: _TDH_OUT_TYPE = _TDH_OUT_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_OUTTYPE_BYTE: _TDH_OUT_TYPE = _TDH_OUT_TYPE(3i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_OUTTYPE_UNSIGNEDBYTE: _TDH_OUT_TYPE = _TDH_OUT_TYPE(4i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_OUTTYPE_SHORT: _TDH_OUT_TYPE = _TDH_OUT_TYPE(5i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_OUTTYPE_UNSIGNEDSHORT: _TDH_OUT_TYPE = _TDH_OUT_TYPE(6i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_OUTTYPE_INT: _TDH_OUT_TYPE = _TDH_OUT_TYPE(7i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_OUTTYPE_UNSIGNEDINT: _TDH_OUT_TYPE = _TDH_OUT_TYPE(8i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_OUTTYPE_LONG: _TDH_OUT_TYPE = _TDH_OUT_TYPE(9i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_OUTTYPE_UNSIGNEDLONG: _TDH_OUT_TYPE = _TDH_OUT_TYPE(10i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_OUTTYPE_FLOAT: _TDH_OUT_TYPE = _TDH_OUT_TYPE(11i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_OUTTYPE_DOUBLE: _TDH_OUT_TYPE = _TDH_OUT_TYPE(12i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_OUTTYPE_BOOLEAN: _TDH_OUT_TYPE = _TDH_OUT_TYPE(13i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_OUTTYPE_GUID: _TDH_OUT_TYPE = _TDH_OUT_TYPE(14i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_OUTTYPE_HEXBINARY: _TDH_OUT_TYPE = _TDH_OUT_TYPE(15i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_OUTTYPE_HEXINT8: _TDH_OUT_TYPE = _TDH_OUT_TYPE(16i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_OUTTYPE_HEXINT16: _TDH_OUT_TYPE = _TDH_OUT_TYPE(17i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_OUTTYPE_HEXINT32: _TDH_OUT_TYPE = _TDH_OUT_TYPE(18i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_OUTTYPE_HEXINT64: _TDH_OUT_TYPE = _TDH_OUT_TYPE(19i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_OUTTYPE_PID: _TDH_OUT_TYPE = _TDH_OUT_TYPE(20i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_OUTTYPE_TID: _TDH_OUT_TYPE = _TDH_OUT_TYPE(21i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_OUTTYPE_PORT: _TDH_OUT_TYPE = _TDH_OUT_TYPE(22i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_OUTTYPE_IPV4: _TDH_OUT_TYPE = _TDH_OUT_TYPE(23i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_OUTTYPE_IPV6: _TDH_OUT_TYPE = _TDH_OUT_TYPE(24i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_OUTTYPE_SOCKETADDRESS: _TDH_OUT_TYPE = _TDH_OUT_TYPE(25i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_OUTTYPE_CIMDATETIME: _TDH_OUT_TYPE = _TDH_OUT_TYPE(26i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_OUTTYPE_ETWTIME: _TDH_OUT_TYPE = _TDH_OUT_TYPE(27i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_OUTTYPE_XML: _TDH_OUT_TYPE = _TDH_OUT_TYPE(28i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_OUTTYPE_ERRORCODE: _TDH_OUT_TYPE = _TDH_OUT_TYPE(29i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_OUTTYPE_WIN32ERROR: _TDH_OUT_TYPE = _TDH_OUT_TYPE(30i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_OUTTYPE_NTSTATUS: _TDH_OUT_TYPE = _TDH_OUT_TYPE(31i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_OUTTYPE_HRESULT: _TDH_OUT_TYPE = _TDH_OUT_TYPE(32i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_OUTTYPE_CULTURE_INSENSITIVE_DATETIME: _TDH_OUT_TYPE = _TDH_OUT_TYPE(33i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_OUTTYPE_JSON: _TDH_OUT_TYPE = _TDH_OUT_TYPE(34i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_OUTTYPE_UTF8: _TDH_OUT_TYPE = _TDH_OUT_TYPE(35i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_OUTTYPE_PKCS7_WITH_TYPE_INFO: _TDH_OUT_TYPE = _TDH_OUT_TYPE(36i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_OUTTYPE_CODE_POINTER: _TDH_OUT_TYPE = _TDH_OUT_TYPE(37i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_OUTTYPE_DATETIME_UTC: _TDH_OUT_TYPE = _TDH_OUT_TYPE(38i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_OUTTYPE_REDUCEDSTRING: _TDH_OUT_TYPE = _TDH_OUT_TYPE(300i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`*"]
pub const TDH_OUTTYPE_NOPRINT: _TDH_OUT_TYPE = _TDH_OUT_TYPE(301i32);
impl ::core::marker::Copy for _TDH_OUT_TYPE {}
impl ::core::clone::Clone for _TDH_OUT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for _TDH_OUT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for _TDH_OUT_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for _TDH_OUT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_TDH_OUT_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
