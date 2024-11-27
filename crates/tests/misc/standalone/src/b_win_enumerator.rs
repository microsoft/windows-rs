#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WAIT_EVENT(pub u32);
pub const WAIT_IO_COMPLETION: WAIT_EVENT = WAIT_EVENT(192u32);
pub const WAIT_TIMEOUT: WAIT_EVENT = WAIT_EVENT(258u32);
