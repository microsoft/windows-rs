pub const PERCEPTIONFIELD_StateStream_TimeStamps: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xaa886119_f32f_49bf_92ca_f9ddf784d297);
#[repr(C)]
pub struct PERCEPTION_PAYLOAD_FIELD {
    pub FieldId: windows_sys::core::GUID,
    pub OffsetInBytes: u32,
    pub SizeInBytes: u32,
}
impl Copy for PERCEPTION_PAYLOAD_FIELD {}
impl Clone for PERCEPTION_PAYLOAD_FIELD {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct PERCEPTION_STATE_STREAM_TIMESTAMPS {
    pub InputTimestampInQpcCounts: i64,
    pub AvailableTimestampInQpcCounts: i64,
}
impl Copy for PERCEPTION_STATE_STREAM_TIMESTAMPS {}
impl Clone for PERCEPTION_STATE_STREAM_TIMESTAMPS {
    fn clone(&self) -> Self {
        *self
    }
}
