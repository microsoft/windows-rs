#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

pub const CMC_ADD_ATTRIBUTES: PCSTR = 63i32 as _;
pub const E_ACCESSDENIED: HRESULT = 0x80070005_u32 as _;
pub type HRESULT = i32;
pub const IDC_UPARROW: PCWSTR = 32516u16 as _;
pub type NTSTATUS = i32;
pub type PCSTR = *const u8;
pub type PCWSTR = *const u16;
pub const STATUS_NOT_FOUND: NTSTATUS = 0xC0000225_u32 as _;
