#[cfg(target_arch = "x86")]
pub type BOOKMARK = SQLUINTEGER;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type BOOKMARK = SQLULEN;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DATE_STRUCT {
    pub year: SQLSMALLINT,
    pub month: SQLUSMALLINT,
    pub day: SQLUSMALLINT,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HDBC(pub *mut core::ffi::c_void);
impl HDBC {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for HDBC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HENV(pub *mut core::ffi::c_void);
impl HENV {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for HENV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HSTMT(pub *mut core::ffi::c_void);
impl HSTMT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for HSTMT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PTR(pub *mut core::ffi::c_void);
impl PTR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct RETCODE(pub i16);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct SCHAR(pub i8);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct SDWORD(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct SLONG(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct SQLBIGINT(pub i64);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct SQLCHAR(pub u8);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct SQLDATE(pub u8);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct SQLDECIMAL(pub u8);
pub type SQLGUID = windows_core::GUID;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SQLHANDLE(pub *mut core::ffi::c_void);
impl SQLHANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for SQLHANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type SQLHDBC = SQLHANDLE;
pub type SQLHDESC = SQLHANDLE;
pub type SQLHENV = SQLHANDLE;
pub type SQLHSTMT = SQLHANDLE;
#[cfg(feature = "windef")]
pub type SQLHWND = super::windef::HWND;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct SQLINTEGER(pub i32);
pub type SQLINTERVAL = i32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct SQLLEN(pub i64);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct SQLNUMERIC(pub u8);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SQLPOINTER(pub *mut core::ffi::c_void);
impl SQLPOINTER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for SQLPOINTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct SQLSCHAR(pub i8);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct SQLSETPOSIROW(pub u64);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct SQLSMALLINT(pub i16);
pub type SQLTCHAR = SQLCHAR;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct SQLTIME(pub u8);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct SQLTIMESTAMP(pub u8);
#[cfg(target_arch = "x86")]
pub type SQLTRANSID = SQLUINTEGER;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type SQLTRANSID = SQLULEN;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct SQLUBIGINT(pub u64);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct SQLUINTEGER(pub u32);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct SQLULEN(pub u64);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct SQLUSMALLINT(pub u16);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct SQLVARCHAR(pub u8);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct SQLWCHAR(pub u16);
pub type SQL_DATE_STRUCT = DATE_STRUCT;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SQL_YEAR_MONTH_STRUCT {
    pub year: SQLUINTEGER,
    pub month: SQLUINTEGER,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct SSHORT(pub i16);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct SWORD(pub i16);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TIME_STRUCT {
    pub hour: SQLUSMALLINT,
    pub minute: SQLUSMALLINT,
    pub second: SQLUSMALLINT,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct UDWORD(pub u32);
