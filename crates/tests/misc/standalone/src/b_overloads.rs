#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct AE_RESACCESS {
    pub ae_ra_compname: u32,
    pub ae_ra_username: u32,
    pub ae_ra_resname: u32,
    pub ae_ra_operation: u32,
    pub ae_ra_returncode: u32,
    pub ae_ra_restype: u32,
    pub ae_ra_fileid: u32,
}
pub const AE_RESACCESS: u32 = 7u32;
