#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
pub const ICW_ALREADYRUN: u32 = 4u32;
pub const ICW_CHECKSTATUS: u32 = 1u32;
pub const ICW_FULLPRESENT: u32 = 1u32;
pub const ICW_FULL_SMARTSTART: u32 = 2048u32;
pub const ICW_LAUNCHEDFULL: u32 = 256u32;
pub const ICW_LAUNCHEDMANUAL: u32 = 512u32;
pub const ICW_LAUNCHFULL: u32 = 256u32;
pub const ICW_LAUNCHMANUAL: u32 = 512u32;
pub const ICW_MANUALPRESENT: u32 = 2u32;
pub const ICW_MAX_ACCTNAME: u32 = 256u32;
pub const ICW_MAX_EMAILADDR: u32 = 128u32;
pub const ICW_MAX_EMAILNAME: u32 = 64u32;
pub const ICW_MAX_LOGONNAME: u32 = 256u32;
pub const ICW_MAX_PASSWORD: u32 = 256u32;
pub const ICW_MAX_RASNAME: u32 = 256u32;
pub const ICW_MAX_SERVERNAME: u32 = 64u32;
pub const ICW_USEDEFAULTS: u32 = 1u32;
pub const ICW_USE_SHELLNEXT: u32 = 1024u32;
pub type PFNCHECKCONNECTIONWIZARD = unsafe extern "system" fn(param0: u32, param1: *mut u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PFNSETSHELLNEXT = unsafe extern "system" fn(param0: super::super::Foundation::PSTR) -> u32;
