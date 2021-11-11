#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn CDB_REPORT_BITS();
    fn CDB_REPORT_BYTES();
    fn COMDB_MAX_PORTS_ARBITRATED();
    fn COMDB_MIN_PORTS_ARBITRATED();
    fn ComDBClaimNextFreePort();
    fn ComDBClaimPort();
    fn ComDBClose();
    fn ComDBGetCurrentPortUsage();
    fn ComDBOpen();
    fn ComDBReleasePort();
    fn ComDBResizeDatabase();
    fn HCOMDB();
}
