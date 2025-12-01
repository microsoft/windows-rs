#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

extern "system" {
    pub fn GetTickCount() -> u32;
}
