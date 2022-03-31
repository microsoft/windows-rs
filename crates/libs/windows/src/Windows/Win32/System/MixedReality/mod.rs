#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
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
impl ::core::fmt::Debug for PERCEPTION_PAYLOAD_FIELD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERCEPTION_PAYLOAD_FIELD").field("FieldId", &self.FieldId).field("OffsetInBytes", &self.OffsetInBytes).field("SizeInBytes", &self.SizeInBytes).finish()
    }
}
unsafe impl ::windows::core::Abi for PERCEPTION_PAYLOAD_FIELD {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PERCEPTION_PAYLOAD_FIELD {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PERCEPTION_PAYLOAD_FIELD>()) == 0 }
    }
}
impl ::core::cmp::Eq for PERCEPTION_PAYLOAD_FIELD {}
impl ::core::default::Default for PERCEPTION_PAYLOAD_FIELD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
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
impl ::core::fmt::Debug for PERCEPTION_STATE_STREAM_TIMESTAMPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERCEPTION_STATE_STREAM_TIMESTAMPS").field("InputTimestampInQpcCounts", &self.InputTimestampInQpcCounts).field("AvailableTimestampInQpcCounts", &self.AvailableTimestampInQpcCounts).finish()
    }
}
unsafe impl ::windows::core::Abi for PERCEPTION_STATE_STREAM_TIMESTAMPS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PERCEPTION_STATE_STREAM_TIMESTAMPS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PERCEPTION_STATE_STREAM_TIMESTAMPS>()) == 0 }
    }
}
impl ::core::cmp::Eq for PERCEPTION_STATE_STREAM_TIMESTAMPS {}
impl ::core::default::Default for PERCEPTION_STATE_STREAM_TIMESTAMPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
