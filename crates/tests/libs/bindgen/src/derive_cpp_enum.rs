#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd)]
pub struct WAIT_EVENT(pub u32);
#[must_use]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WIN32_ERROR(pub u32);
