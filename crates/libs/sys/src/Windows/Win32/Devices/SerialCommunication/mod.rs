::windows_targets::link!("msports.dll" "system" fn ComDBClaimNextFreePort(hcomdb : HCOMDB, comnumber : *mut u32) -> i32);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("msports.dll" "system" #[doc = "Required features: `Win32_Foundation`"] fn ComDBClaimPort(hcomdb : HCOMDB, comnumber : u32, forceclaim : super::super::Foundation:: BOOL, forced : *mut super::super::Foundation:: BOOL) -> i32);
::windows_targets::link!("msports.dll" "system" fn ComDBClose(hcomdb : HCOMDB) -> i32);
::windows_targets::link!("msports.dll" "system" fn ComDBGetCurrentPortUsage(hcomdb : HCOMDB, buffer : *mut u8, buffersize : u32, reporttype : u32, maxportsreported : *mut u32) -> i32);
::windows_targets::link!("msports.dll" "system" fn ComDBOpen(phcomdb : *mut HCOMDB) -> i32);
::windows_targets::link!("msports.dll" "system" fn ComDBReleasePort(hcomdb : HCOMDB, comnumber : u32) -> i32);
::windows_targets::link!("msports.dll" "system" fn ComDBResizeDatabase(hcomdb : HCOMDB, newsize : u32) -> i32);
pub const CDB_REPORT_BITS: u32 = 0u32;
pub const CDB_REPORT_BYTES: u32 = 1u32;
pub const COMDB_MAX_PORTS_ARBITRATED: u32 = 4096u32;
pub const COMDB_MIN_PORTS_ARBITRATED: u32 = 256u32;
pub type HCOMDB = isize;
