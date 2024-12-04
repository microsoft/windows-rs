#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

pub type WAIT_EVENT = u32;
pub const WAIT_IO_COMPLETION: WAIT_EVENT = 192u32;
pub const WAIT_TIMEOUT: WAIT_EVENT = 258u32;
