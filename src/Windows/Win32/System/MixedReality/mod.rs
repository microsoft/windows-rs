#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
pub const PERCEPTIONFIELD_StateStream_TimeStamps: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaa886119_f32f_49bf_92ca_f9ddf784d297);
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_MixedReality`*"]
pub struct PERCEPTION_PAYLOAD_FIELD {
    pub FieldId: ::windows::core::GUID,
    pub OffsetInBytes: u32,
    pub SizeInBytes: u32,
}
impl PERCEPTION_PAYLOAD_FIELD {}
impl ::core::default::Default for PERCEPTION_PAYLOAD_FIELD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for PERCEPTION_PAYLOAD_FIELD {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("PERCEPTION_PAYLOAD_FIELD").field("FieldId", &self.FieldId).field("OffsetInBytes", &self.OffsetInBytes).field("SizeInBytes", &self.SizeInBytes).finish()
    }
}
impl ::core::cmp::PartialEq for PERCEPTION_PAYLOAD_FIELD {
    fn eq(&self, other: &Self) -> bool {
        self.FieldId == other.FieldId && self.OffsetInBytes == other.OffsetInBytes && self.SizeInBytes == other.SizeInBytes
    }
}
impl ::core::cmp::Eq for PERCEPTION_PAYLOAD_FIELD {}
unsafe impl ::windows::core::Abi for PERCEPTION_PAYLOAD_FIELD {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_MixedReality`*"]
pub struct PERCEPTION_STATE_STREAM_TIMESTAMPS {
    pub InputTimestampInQpcCounts: i64,
    pub AvailableTimestampInQpcCounts: i64,
}
impl PERCEPTION_STATE_STREAM_TIMESTAMPS {}
impl ::core::default::Default for PERCEPTION_STATE_STREAM_TIMESTAMPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for PERCEPTION_STATE_STREAM_TIMESTAMPS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("PERCEPTION_STATE_STREAM_TIMESTAMPS").field("InputTimestampInQpcCounts", &self.InputTimestampInQpcCounts).field("AvailableTimestampInQpcCounts", &self.AvailableTimestampInQpcCounts).finish()
    }
}
impl ::core::cmp::PartialEq for PERCEPTION_STATE_STREAM_TIMESTAMPS {
    fn eq(&self, other: &Self) -> bool {
        self.InputTimestampInQpcCounts == other.InputTimestampInQpcCounts && self.AvailableTimestampInQpcCounts == other.AvailableTimestampInQpcCounts
    }
}
impl ::core::cmp::Eq for PERCEPTION_STATE_STREAM_TIMESTAMPS {}
unsafe impl ::windows::core::Abi for PERCEPTION_STATE_STREAM_TIMESTAMPS {
    type Abi = Self;
}
