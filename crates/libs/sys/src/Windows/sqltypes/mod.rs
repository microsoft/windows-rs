#[cfg(target_arch = "x86")]
pub type BOOKMARK = SQLUINTEGER;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type BOOKMARK = SQLULEN;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DATE_STRUCT {
    pub year: SQLSMALLINT,
    pub month: SQLUSMALLINT,
    pub day: SQLUSMALLINT,
}
pub type HDBC = *mut core::ffi::c_void;
pub type HENV = *mut core::ffi::c_void;
pub type HSTMT = *mut core::ffi::c_void;
pub type PTR = *mut core::ffi::c_void;
pub type RETCODE = i16;
pub type SCHAR = i8;
pub type SDWORD = i32;
pub type SLONG = i32;
pub type SQLBIGINT = i64;
pub type SQLCHAR = u8;
pub type SQLDATE = u8;
pub type SQLDECIMAL = u8;
pub type SQLGUID = windows_sys::core::GUID;
pub type SQLHANDLE = *mut core::ffi::c_void;
pub type SQLHDBC = SQLHANDLE;
pub type SQLHDESC = SQLHANDLE;
pub type SQLHENV = SQLHANDLE;
pub type SQLHSTMT = SQLHANDLE;
#[cfg(feature = "windef")]
pub type SQLHWND = super::windef::HWND;
pub type SQLINTEGER = i32;
pub type SQLINTERVAL = i32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type SQLLEN = i64;
pub type SQLNUMERIC = u8;
pub type SQLPOINTER = *mut core::ffi::c_void;
pub type SQLRETURN = SQLSMALLINT;
#[cfg(target_arch = "x86")]
pub type SQLROWCOUNT = SQLUINTEGER;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type SQLROWCOUNT = SQLULEN;
#[cfg(target_arch = "x86")]
pub type SQLROWOFFSET = SQLINTEGER;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type SQLROWOFFSET = SQLLEN;
#[cfg(target_arch = "x86")]
pub type SQLROWSETSIZE = SQLUINTEGER;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type SQLROWSETSIZE = SQLULEN;
pub type SQLSCHAR = i8;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type SQLSETPOSIROW = u64;
pub type SQLSMALLINT = i16;
pub type SQLTCHAR = SQLCHAR;
pub type SQLTIME = u8;
pub type SQLTIMESTAMP = u8;
#[cfg(target_arch = "x86")]
pub type SQLTRANSID = SQLUINTEGER;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type SQLTRANSID = SQLULEN;
pub type SQLUBIGINT = u64;
pub type SQLUINTEGER = u32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type SQLULEN = u64;
pub type SQLUSMALLINT = u16;
pub type SQLVARCHAR = u8;
pub type SQLWCHAR = u16;
pub type SQL_DATE_STRUCT = DATE_STRUCT;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SQL_DAY_SECOND_STRUCT {
    pub day: SQLUINTEGER,
    pub hour: SQLUINTEGER,
    pub minute: SQLUINTEGER,
    pub second: SQLUINTEGER,
    pub fraction: SQLUINTEGER,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SQL_INTERVAL_STRUCT {
    pub interval_type: SQLINTERVAL,
    pub interval_sign: SQLSMALLINT,
    pub intval: SQL_INTERVAL_STRUCT_0,
}
impl Default for SQL_INTERVAL_STRUCT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union SQL_INTERVAL_STRUCT_0 {
    pub year_month: SQL_YEAR_MONTH_STRUCT,
    pub day_second: SQL_DAY_SECOND_STRUCT,
}
impl Default for SQL_INTERVAL_STRUCT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SQL_IS_DAY: SQLINTERVAL = 3;
pub const SQL_IS_DAY_TO_HOUR: SQLINTERVAL = 8;
pub const SQL_IS_DAY_TO_MINUTE: SQLINTERVAL = 9;
pub const SQL_IS_DAY_TO_SECOND: SQLINTERVAL = 10;
pub const SQL_IS_HOUR: SQLINTERVAL = 4;
pub const SQL_IS_HOUR_TO_MINUTE: SQLINTERVAL = 11;
pub const SQL_IS_HOUR_TO_SECOND: SQLINTERVAL = 12;
pub const SQL_IS_MINUTE: SQLINTERVAL = 5;
pub const SQL_IS_MINUTE_TO_SECOND: SQLINTERVAL = 13;
pub const SQL_IS_MONTH: SQLINTERVAL = 2;
pub const SQL_IS_SECOND: SQLINTERVAL = 6;
pub const SQL_IS_YEAR: SQLINTERVAL = 1;
pub const SQL_IS_YEAR_TO_MONTH: SQLINTERVAL = 7;
pub const SQL_MAX_NUMERIC_LEN: u32 = 16;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SQL_NUMERIC_STRUCT {
    pub precision: SQLCHAR,
    pub scale: SQLSCHAR,
    pub sign: SQLCHAR,
    pub val: [SQLCHAR; 16],
}
impl Default for SQL_NUMERIC_STRUCT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type SQL_TIMESTAMP_STRUCT = TIMESTAMP_STRUCT;
pub type SQL_TIME_STRUCT = TIME_STRUCT;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SQL_YEAR_MONTH_STRUCT {
    pub year: SQLUINTEGER,
    pub month: SQLUINTEGER,
}
pub type SSHORT = i16;
pub type SWORD = i16;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct TIMESTAMP_STRUCT {
    pub year: SQLSMALLINT,
    pub month: SQLUSMALLINT,
    pub day: SQLUSMALLINT,
    pub hour: SQLUSMALLINT,
    pub minute: SQLUSMALLINT,
    pub second: SQLUSMALLINT,
    pub fraction: SQLUINTEGER,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct TIME_STRUCT {
    pub hour: SQLUSMALLINT,
    pub minute: SQLUSMALLINT,
    pub second: SQLUSMALLINT,
}
pub type UDWORD = u32;
