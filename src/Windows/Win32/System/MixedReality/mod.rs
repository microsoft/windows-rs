#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
pub const PERCEPTIONFIELD_StateStream_TimeStamps: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2861064473,
        62255,
        18879,
        [146, 202, 249, 221, 247, 132, 210, 151],
    );
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct PERCEPTION_PAYLOAD_FIELD {
    pub FieldId: ::windows::runtime::GUID,
    pub OffsetInBytes: u32,
    pub SizeInBytes: u32,
}
impl PERCEPTION_PAYLOAD_FIELD {}
impl ::std::default::Default for PERCEPTION_PAYLOAD_FIELD {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PERCEPTION_PAYLOAD_FIELD {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PERCEPTION_PAYLOAD_FIELD")
            .field("FieldId", &self.FieldId)
            .field("OffsetInBytes", &self.OffsetInBytes)
            .field("SizeInBytes", &self.SizeInBytes)
            .finish()
    }
}
impl ::std::cmp::PartialEq for PERCEPTION_PAYLOAD_FIELD {
    fn eq(&self, other: &Self) -> bool {
        self.FieldId == other.FieldId
            && self.OffsetInBytes == other.OffsetInBytes
            && self.SizeInBytes == other.SizeInBytes
    }
}
impl ::std::cmp::Eq for PERCEPTION_PAYLOAD_FIELD {}
unsafe impl ::windows::runtime::Abi for PERCEPTION_PAYLOAD_FIELD {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct PERCEPTION_STATE_STREAM_TIMESTAMPS {
    pub InputTimestampInQpcCounts: i64,
    pub AvailableTimestampInQpcCounts: i64,
}
impl PERCEPTION_STATE_STREAM_TIMESTAMPS {}
impl ::std::default::Default for PERCEPTION_STATE_STREAM_TIMESTAMPS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PERCEPTION_STATE_STREAM_TIMESTAMPS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PERCEPTION_STATE_STREAM_TIMESTAMPS")
            .field("InputTimestampInQpcCounts", &self.InputTimestampInQpcCounts)
            .field(
                "AvailableTimestampInQpcCounts",
                &self.AvailableTimestampInQpcCounts,
            )
            .finish()
    }
}
impl ::std::cmp::PartialEq for PERCEPTION_STATE_STREAM_TIMESTAMPS {
    fn eq(&self, other: &Self) -> bool {
        self.InputTimestampInQpcCounts == other.InputTimestampInQpcCounts
            && self.AvailableTimestampInQpcCounts == other.AvailableTimestampInQpcCounts
    }
}
impl ::std::cmp::Eq for PERCEPTION_STATE_STREAM_TIMESTAMPS {}
unsafe impl ::windows::runtime::Abi for PERCEPTION_STATE_STREAM_TIMESTAMPS {
    type Abi = Self;
    type DefaultType = Self;
}
