#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
pub const PERCEPTIONFIELD_StateStream_TimeStamps: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2861064473,
    data2: 62255,
    data3: 18879,
    data4: [146, 202, 249, 221, 247, 132, 210, 151],
};
#[repr(C)]
pub struct PERCEPTION_PAYLOAD_FIELD(i32);
#[repr(C)]
pub struct PERCEPTION_STATE_STREAM_TIMESTAMPS(i32);
