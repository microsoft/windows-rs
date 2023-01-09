impl ::core::default::Default for PERCEPTION_PAYLOAD_FIELD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PERCEPTION_PAYLOAD_FIELD {
    fn eq(&self, other: &Self) -> bool {
        self.FieldId == other.FieldId && self.OffsetInBytes == other.OffsetInBytes && self.SizeInBytes == other.SizeInBytes
    }
}
impl ::core::cmp::Eq for PERCEPTION_PAYLOAD_FIELD {}
impl ::core::fmt::Debug for PERCEPTION_PAYLOAD_FIELD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERCEPTION_PAYLOAD_FIELD").field("FieldId", &self.FieldId).field("OffsetInBytes", &self.OffsetInBytes).field("SizeInBytes", &self.SizeInBytes).finish()
    }
}
impl ::core::default::Default for PERCEPTION_STATE_STREAM_TIMESTAMPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PERCEPTION_STATE_STREAM_TIMESTAMPS {
    fn eq(&self, other: &Self) -> bool {
        self.InputTimestampInQpcCounts == other.InputTimestampInQpcCounts && self.AvailableTimestampInQpcCounts == other.AvailableTimestampInQpcCounts
    }
}
impl ::core::cmp::Eq for PERCEPTION_STATE_STREAM_TIMESTAMPS {}
impl ::core::fmt::Debug for PERCEPTION_STATE_STREAM_TIMESTAMPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERCEPTION_STATE_STREAM_TIMESTAMPS").field("InputTimestampInQpcCounts", &self.InputTimestampInQpcCounts).field("AvailableTimestampInQpcCounts", &self.AvailableTimestampInQpcCounts).finish()
    }
}
