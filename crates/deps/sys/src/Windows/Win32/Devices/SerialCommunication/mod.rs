#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {
    pub fn ComDBClaimNextFreePort(hcomdb: HCOMDB, comnumber: *mut u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ComDBClaimPort(hcomdb: HCOMDB, comnumber: u32, forceclaim: super::super::Foundation::BOOL, forced: *mut super::super::Foundation::BOOL) -> i32;
    pub fn ComDBClose(hcomdb: HCOMDB) -> i32;
    pub fn ComDBGetCurrentPortUsage(hcomdb: HCOMDB, buffer: *mut u8, buffersize: u32, reporttype: u32, maxportsreported: *mut u32) -> i32;
    pub fn ComDBOpen(phcomdb: *mut isize) -> i32;
    pub fn ComDBReleasePort(hcomdb: HCOMDB, comnumber: u32) -> i32;
    pub fn ComDBResizeDatabase(hcomdb: HCOMDB, newsize: u32) -> i32;
}
pub const CDB_REPORT_BITS: u32 = 0u32;
pub const CDB_REPORT_BYTES: u32 = 1u32;
pub const COMDB_MAX_PORTS_ARBITRATED: u32 = 4096u32;
pub const COMDB_MIN_PORTS_ARBITRATED: u32 = 256u32;
#[repr(transparent)]
pub struct HCOMDB(pub isize);
impl ::core::marker::Copy for HCOMDB {}
impl ::core::clone::Clone for HCOMDB {
    fn clone(&self) -> Self {
        *self
    }
}
