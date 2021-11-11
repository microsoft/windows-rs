#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn PERCEPTIONFIELD_StateStream_TimeStamps();
    fn PERCEPTION_PAYLOAD_FIELD();
    fn PERCEPTION_STATE_STREAM_TIMESTAMPS();
}
