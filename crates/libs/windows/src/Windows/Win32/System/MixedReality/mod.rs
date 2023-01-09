#[doc = "*Required features: `\"Win32_System_MixedReality\"`*"]
pub const PERCEPTIONFIELD_StateStream_TimeStamps: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaa886119_f32f_49bf_92ca_f9ddf784d297);
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_MixedReality\"`*"]
pub struct PERCEPTION_PAYLOAD_FIELD {
    pub FieldId: ::windows::core::GUID,
    pub OffsetInBytes: u32,
    pub SizeInBytes: u32,
}
impl ::core::marker::Copy for PERCEPTION_PAYLOAD_FIELD {}
impl ::core::clone::Clone for PERCEPTION_PAYLOAD_FIELD {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PERCEPTION_PAYLOAD_FIELD {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_MixedReality\"`*"]
pub struct PERCEPTION_STATE_STREAM_TIMESTAMPS {
    pub InputTimestampInQpcCounts: i64,
    pub AvailableTimestampInQpcCounts: i64,
}
impl ::core::marker::Copy for PERCEPTION_STATE_STREAM_TIMESTAMPS {}
impl ::core::clone::Clone for PERCEPTION_STATE_STREAM_TIMESTAMPS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PERCEPTION_STATE_STREAM_TIMESTAMPS {
    type Abi = Self;
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
#[cfg(feature = "default")]
::core::include!("default.rs");
