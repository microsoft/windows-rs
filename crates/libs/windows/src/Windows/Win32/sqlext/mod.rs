#[inline]
pub unsafe fn ODBCGetTryWaitValue() -> u32 {
    windows_core::link!("odbc32.dll" "system" fn ODBCGetTryWaitValue() -> u32);
    unsafe { ODBCGetTryWaitValue() }
}
#[inline]
pub unsafe fn ODBCSetTryWaitValue(dwvalue: u32) -> windows_core::BOOL {
    windows_core::link!("odbc32.dll" "system" fn ODBCSetTryWaitValue(dwvalue : u32) -> windows_core::BOOL);
    unsafe { ODBCSetTryWaitValue(dwvalue) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLAllocHandleStd(fhandletype: super::SQLSMALLINT, hinput: super::SQLHANDLE, phoutput: *mut super::SQLHANDLE) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLAllocHandleStd(fhandletype : super::SQLSMALLINT, hinput : super::SQLHANDLE, phoutput : *mut super::SQLHANDLE) -> super::SQLRETURN);
    unsafe { SQLAllocHandleStd(fhandletype, hinput, phoutput as _) }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLBindParameter(hstmt: super::SQLHSTMT, ipar: super::SQLUSMALLINT, fparamtype: super::SQLSMALLINT, fctype: super::SQLSMALLINT, fsqltype: super::SQLSMALLINT, cbcoldef: super::SQLUINTEGER, ibscale: super::SQLSMALLINT, rgbvalue: super::SQLPOINTER, cbvaluemax: super::SQLINTEGER, pcbvalue: *mut super::SQLINTEGER) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLBindParameter(hstmt : super::SQLHSTMT, ipar : super::SQLUSMALLINT, fparamtype : super::SQLSMALLINT, fctype : super::SQLSMALLINT, fsqltype : super::SQLSMALLINT, cbcoldef : super::SQLUINTEGER, ibscale : super::SQLSMALLINT, rgbvalue : super::SQLPOINTER, cbvaluemax : super::SQLINTEGER, pcbvalue : *mut super::SQLINTEGER) -> super::SQLRETURN);
    unsafe { SQLBindParameter(hstmt, ipar, fparamtype, fctype, fsqltype, cbcoldef, ibscale, rgbvalue, cbvaluemax, pcbvalue as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLBindParameter(hstmt: super::SQLHSTMT, ipar: super::SQLUSMALLINT, fparamtype: super::SQLSMALLINT, fctype: super::SQLSMALLINT, fsqltype: super::SQLSMALLINT, cbcoldef: super::SQLULEN, ibscale: super::SQLSMALLINT, rgbvalue: super::SQLPOINTER, cbvaluemax: super::SQLLEN, pcbvalue: *mut super::SQLLEN) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLBindParameter(hstmt : super::SQLHSTMT, ipar : super::SQLUSMALLINT, fparamtype : super::SQLSMALLINT, fctype : super::SQLSMALLINT, fsqltype : super::SQLSMALLINT, cbcoldef : super::SQLULEN, ibscale : super::SQLSMALLINT, rgbvalue : super::SQLPOINTER, cbvaluemax : super::SQLLEN, pcbvalue : *mut super::SQLLEN) -> super::SQLRETURN);
    unsafe { SQLBindParameter(hstmt, ipar, fparamtype, fctype, fsqltype, cbcoldef, ibscale, rgbvalue, cbvaluemax, pcbvalue as _) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLBrowseConnect(hdbc: super::SQLHDBC, szconnstrin: *const super::SQLCHAR, cchconnstrin: super::SQLSMALLINT, szconnstrout: Option<*mut super::SQLCHAR>, cchconnstroutmax: super::SQLSMALLINT, pcchconnstrout: Option<*mut super::SQLSMALLINT>) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLBrowseConnect(hdbc : super::SQLHDBC, szconnstrin : *const super::SQLCHAR, cchconnstrin : super::SQLSMALLINT, szconnstrout : *mut super::SQLCHAR, cchconnstroutmax : super::SQLSMALLINT, pcchconnstrout : *mut super::SQLSMALLINT) -> super::SQLRETURN);
    unsafe { SQLBrowseConnect(hdbc, szconnstrin, cchconnstrin, szconnstrout.unwrap_or(core::mem::zeroed()) as _, cchconnstroutmax, pcchconnstrout.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLBulkOperations(statementhandle: super::SQLHSTMT, operation: super::SQLSMALLINT) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLBulkOperations(statementhandle : super::SQLHSTMT, operation : super::SQLSMALLINT) -> super::SQLRETURN);
    unsafe { SQLBulkOperations(statementhandle, operation) }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLColAttributes(hstmt: super::SQLHSTMT, icol: super::SQLUSMALLINT, fdesctype: super::SQLUSMALLINT, rgbdesc: super::SQLPOINTER, cbdescmax: super::SQLSMALLINT, pcbdesc: *mut super::SQLSMALLINT, pfdesc: *mut super::SQLINTEGER) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLColAttributes(hstmt : super::SQLHSTMT, icol : super::SQLUSMALLINT, fdesctype : super::SQLUSMALLINT, rgbdesc : super::SQLPOINTER, cbdescmax : super::SQLSMALLINT, pcbdesc : *mut super::SQLSMALLINT, pfdesc : *mut super::SQLINTEGER) -> super::SQLRETURN);
    unsafe { SQLColAttributes(hstmt, icol, fdesctype, rgbdesc, cbdescmax, pcbdesc as _, pfdesc as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLColAttributes(hstmt: super::SQLHSTMT, icol: super::SQLUSMALLINT, fdesctype: super::SQLUSMALLINT, rgbdesc: super::SQLPOINTER, cbdescmax: super::SQLSMALLINT, pcbdesc: *mut super::SQLSMALLINT, pfdesc: *mut super::SQLLEN) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLColAttributes(hstmt : super::SQLHSTMT, icol : super::SQLUSMALLINT, fdesctype : super::SQLUSMALLINT, rgbdesc : super::SQLPOINTER, cbdescmax : super::SQLSMALLINT, pcbdesc : *mut super::SQLSMALLINT, pfdesc : *mut super::SQLLEN) -> super::SQLRETURN);
    unsafe { SQLColAttributes(hstmt, icol, fdesctype, rgbdesc, cbdescmax, pcbdesc as _, pfdesc as _) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLColumnPrivileges(hstmt: super::SQLHSTMT, szcatalogname: Option<*const super::SQLCHAR>, cchcatalogname: super::SQLSMALLINT, szschemaname: Option<*const super::SQLCHAR>, cchschemaname: super::SQLSMALLINT, sztablename: Option<*const super::SQLCHAR>, cchtablename: super::SQLSMALLINT, szcolumnname: Option<*const super::SQLCHAR>, cchcolumnname: super::SQLSMALLINT) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLColumnPrivileges(hstmt : super::SQLHSTMT, szcatalogname : *const super::SQLCHAR, cchcatalogname : super::SQLSMALLINT, szschemaname : *const super::SQLCHAR, cchschemaname : super::SQLSMALLINT, sztablename : *const super::SQLCHAR, cchtablename : super::SQLSMALLINT, szcolumnname : *const super::SQLCHAR, cchcolumnname : super::SQLSMALLINT) -> super::SQLRETURN);
    unsafe { SQLColumnPrivileges(hstmt, szcatalogname.unwrap_or(core::mem::zeroed()) as _, cchcatalogname, szschemaname.unwrap_or(core::mem::zeroed()) as _, cchschemaname, sztablename.unwrap_or(core::mem::zeroed()) as _, cchtablename, szcolumnname.unwrap_or(core::mem::zeroed()) as _, cchcolumnname) }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLDescribeParam(hstmt: super::SQLHSTMT, ipar: super::SQLUSMALLINT, pfsqltype: Option<*mut super::SQLSMALLINT>, pcbparamdef: Option<*mut super::SQLUINTEGER>, pibscale: Option<*mut super::SQLSMALLINT>, pfnullable: Option<*mut super::SQLSMALLINT>) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLDescribeParam(hstmt : super::SQLHSTMT, ipar : super::SQLUSMALLINT, pfsqltype : *mut super::SQLSMALLINT, pcbparamdef : *mut super::SQLUINTEGER, pibscale : *mut super::SQLSMALLINT, pfnullable : *mut super::SQLSMALLINT) -> super::SQLRETURN);
    unsafe { SQLDescribeParam(hstmt, ipar, pfsqltype.unwrap_or(core::mem::zeroed()) as _, pcbparamdef.unwrap_or(core::mem::zeroed()) as _, pibscale.unwrap_or(core::mem::zeroed()) as _, pfnullable.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLDescribeParam(hstmt: super::SQLHSTMT, ipar: super::SQLUSMALLINT, pfsqltype: Option<*mut super::SQLSMALLINT>, pcbparamdef: Option<*mut super::SQLULEN>, pibscale: Option<*mut super::SQLSMALLINT>, pfnullable: Option<*mut super::SQLSMALLINT>) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLDescribeParam(hstmt : super::SQLHSTMT, ipar : super::SQLUSMALLINT, pfsqltype : *mut super::SQLSMALLINT, pcbparamdef : *mut super::SQLULEN, pibscale : *mut super::SQLSMALLINT, pfnullable : *mut super::SQLSMALLINT) -> super::SQLRETURN);
    unsafe { SQLDescribeParam(hstmt, ipar, pfsqltype.unwrap_or(core::mem::zeroed()) as _, pcbparamdef.unwrap_or(core::mem::zeroed()) as _, pibscale.unwrap_or(core::mem::zeroed()) as _, pfnullable.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "sqltypes", feature = "windef"))]
#[inline]
pub unsafe fn SQLDriverConnect(hdbc: super::SQLHDBC, hwnd: super::SQLHWND, szconnstrin: *const super::SQLCHAR, cchconnstrin: super::SQLSMALLINT, szconnstrout: Option<*mut super::SQLCHAR>, cchconnstroutmax: super::SQLSMALLINT, pcchconnstrout: Option<*mut super::SQLSMALLINT>, fdrivercompletion: super::SQLUSMALLINT) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLDriverConnect(hdbc : super::SQLHDBC, hwnd : super::SQLHWND, szconnstrin : *const super::SQLCHAR, cchconnstrin : super::SQLSMALLINT, szconnstrout : *mut super::SQLCHAR, cchconnstroutmax : super::SQLSMALLINT, pcchconnstrout : *mut super::SQLSMALLINT, fdrivercompletion : super::SQLUSMALLINT) -> super::SQLRETURN);
    unsafe { SQLDriverConnect(hdbc, hwnd, szconnstrin, cchconnstrin, szconnstrout.unwrap_or(core::mem::zeroed()) as _, cchconnstroutmax, pcchconnstrout.unwrap_or(core::mem::zeroed()) as _, fdrivercompletion) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLDrivers(henv: super::SQLHENV, fdirection: super::SQLUSMALLINT, szdriverdesc: Option<*mut super::SQLCHAR>, cchdriverdescmax: super::SQLSMALLINT, pcchdriverdesc: Option<*mut super::SQLSMALLINT>, szdriverattributes: Option<*mut super::SQLCHAR>, cchdrvrattrmax: super::SQLSMALLINT, pcchdrvrattr: Option<*mut super::SQLSMALLINT>) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLDrivers(henv : super::SQLHENV, fdirection : super::SQLUSMALLINT, szdriverdesc : *mut super::SQLCHAR, cchdriverdescmax : super::SQLSMALLINT, pcchdriverdesc : *mut super::SQLSMALLINT, szdriverattributes : *mut super::SQLCHAR, cchdrvrattrmax : super::SQLSMALLINT, pcchdrvrattr : *mut super::SQLSMALLINT) -> super::SQLRETURN);
    unsafe { SQLDrivers(henv, fdirection, szdriverdesc.unwrap_or(core::mem::zeroed()) as _, cchdriverdescmax, pcchdriverdesc.unwrap_or(core::mem::zeroed()) as _, szdriverattributes.unwrap_or(core::mem::zeroed()) as _, cchdrvrattrmax, pcchdrvrattr.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLExtendedFetch(hstmt: super::SQLHSTMT, ffetchtype: super::SQLUSMALLINT, irow: super::SQLINTEGER, pcrow: Option<*mut super::SQLUINTEGER>, rgfrowstatus: Option<*mut super::SQLUSMALLINT>) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLExtendedFetch(hstmt : super::SQLHSTMT, ffetchtype : super::SQLUSMALLINT, irow : super::SQLINTEGER, pcrow : *mut super::SQLUINTEGER, rgfrowstatus : *mut super::SQLUSMALLINT) -> super::SQLRETURN);
    unsafe { SQLExtendedFetch(hstmt, ffetchtype, irow, pcrow.unwrap_or(core::mem::zeroed()) as _, rgfrowstatus.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLExtendedFetch(hstmt: super::SQLHSTMT, ffetchtype: super::SQLUSMALLINT, irow: super::SQLLEN, pcrow: Option<*mut super::SQLULEN>, rgfrowstatus: Option<*mut super::SQLUSMALLINT>) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLExtendedFetch(hstmt : super::SQLHSTMT, ffetchtype : super::SQLUSMALLINT, irow : super::SQLLEN, pcrow : *mut super::SQLULEN, rgfrowstatus : *mut super::SQLUSMALLINT) -> super::SQLRETURN);
    unsafe { SQLExtendedFetch(hstmt, ffetchtype, irow, pcrow.unwrap_or(core::mem::zeroed()) as _, rgfrowstatus.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLForeignKeys(hstmt: super::SQLHSTMT, szpkcatalogname: Option<*const super::SQLCHAR>, cchpkcatalogname: super::SQLSMALLINT, szpkschemaname: Option<*const super::SQLCHAR>, cchpkschemaname: super::SQLSMALLINT, szpktablename: Option<*const super::SQLCHAR>, cchpktablename: super::SQLSMALLINT, szfkcatalogname: Option<*const super::SQLCHAR>, cchfkcatalogname: super::SQLSMALLINT, szfkschemaname: Option<*const super::SQLCHAR>, cchfkschemaname: super::SQLSMALLINT, szfktablename: Option<*const super::SQLCHAR>, cchfktablename: super::SQLSMALLINT) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLForeignKeys(hstmt : super::SQLHSTMT, szpkcatalogname : *const super::SQLCHAR, cchpkcatalogname : super::SQLSMALLINT, szpkschemaname : *const super::SQLCHAR, cchpkschemaname : super::SQLSMALLINT, szpktablename : *const super::SQLCHAR, cchpktablename : super::SQLSMALLINT, szfkcatalogname : *const super::SQLCHAR, cchfkcatalogname : super::SQLSMALLINT, szfkschemaname : *const super::SQLCHAR, cchfkschemaname : super::SQLSMALLINT, szfktablename : *const super::SQLCHAR, cchfktablename : super::SQLSMALLINT) -> super::SQLRETURN);
    unsafe { SQLForeignKeys(hstmt, szpkcatalogname.unwrap_or(core::mem::zeroed()) as _, cchpkcatalogname, szpkschemaname.unwrap_or(core::mem::zeroed()) as _, cchpkschemaname, szpktablename.unwrap_or(core::mem::zeroed()) as _, cchpktablename, szfkcatalogname.unwrap_or(core::mem::zeroed()) as _, cchfkcatalogname, szfkschemaname.unwrap_or(core::mem::zeroed()) as _, cchfkschemaname, szfktablename.unwrap_or(core::mem::zeroed()) as _, cchfktablename) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLMoreResults(hstmt: super::SQLHSTMT) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLMoreResults(hstmt : super::SQLHSTMT) -> super::SQLRETURN);
    unsafe { SQLMoreResults(hstmt) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLNativeSql(hdbc: super::SQLHDBC, szsqlstrin: *const super::SQLCHAR, cchsqlstrin: super::SQLINTEGER, szsqlstr: Option<*mut super::SQLCHAR>, cchsqlstrmax: super::SQLINTEGER, pcbsqlstr: *mut super::SQLINTEGER) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLNativeSql(hdbc : super::SQLHDBC, szsqlstrin : *const super::SQLCHAR, cchsqlstrin : super::SQLINTEGER, szsqlstr : *mut super::SQLCHAR, cchsqlstrmax : super::SQLINTEGER, pcbsqlstr : *mut super::SQLINTEGER) -> super::SQLRETURN);
    unsafe { SQLNativeSql(hdbc, szsqlstrin, cchsqlstrin, szsqlstr.unwrap_or(core::mem::zeroed()) as _, cchsqlstrmax, pcbsqlstr as _) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLNumParams(hstmt: super::SQLHSTMT, pcpar: Option<*mut super::SQLSMALLINT>) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLNumParams(hstmt : super::SQLHSTMT, pcpar : *mut super::SQLSMALLINT) -> super::SQLRETURN);
    unsafe { SQLNumParams(hstmt, pcpar.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLParamOptions(hstmt: super::SQLHSTMT, crow: super::SQLUINTEGER, pirow: *mut super::SQLUINTEGER) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLParamOptions(hstmt : super::SQLHSTMT, crow : super::SQLUINTEGER, pirow : *mut super::SQLUINTEGER) -> super::SQLRETURN);
    unsafe { SQLParamOptions(hstmt, crow, pirow as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLParamOptions(hstmt: super::SQLHSTMT, crow: super::SQLULEN, pirow: *mut super::SQLULEN) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLParamOptions(hstmt : super::SQLHSTMT, crow : super::SQLULEN, pirow : *mut super::SQLULEN) -> super::SQLRETURN);
    unsafe { SQLParamOptions(hstmt, crow, pirow as _) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLPrimaryKeys(hstmt: super::SQLHSTMT, szcatalogname: Option<*const super::SQLCHAR>, cchcatalogname: super::SQLSMALLINT, szschemaname: Option<*const super::SQLCHAR>, cchschemaname: super::SQLSMALLINT, sztablename: Option<*const super::SQLCHAR>, cchtablename: super::SQLSMALLINT) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLPrimaryKeys(hstmt : super::SQLHSTMT, szcatalogname : *const super::SQLCHAR, cchcatalogname : super::SQLSMALLINT, szschemaname : *const super::SQLCHAR, cchschemaname : super::SQLSMALLINT, sztablename : *const super::SQLCHAR, cchtablename : super::SQLSMALLINT) -> super::SQLRETURN);
    unsafe { SQLPrimaryKeys(hstmt, szcatalogname.unwrap_or(core::mem::zeroed()) as _, cchcatalogname, szschemaname.unwrap_or(core::mem::zeroed()) as _, cchschemaname, sztablename.unwrap_or(core::mem::zeroed()) as _, cchtablename) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLProcedureColumns(hstmt: super::SQLHSTMT, szcatalogname: Option<*const super::SQLCHAR>, cchcatalogname: super::SQLSMALLINT, szschemaname: Option<*const super::SQLCHAR>, cchschemaname: super::SQLSMALLINT, szprocname: Option<*const super::SQLCHAR>, cchprocname: super::SQLSMALLINT, szcolumnname: Option<*const super::SQLCHAR>, cchcolumnname: super::SQLSMALLINT) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLProcedureColumns(hstmt : super::SQLHSTMT, szcatalogname : *const super::SQLCHAR, cchcatalogname : super::SQLSMALLINT, szschemaname : *const super::SQLCHAR, cchschemaname : super::SQLSMALLINT, szprocname : *const super::SQLCHAR, cchprocname : super::SQLSMALLINT, szcolumnname : *const super::SQLCHAR, cchcolumnname : super::SQLSMALLINT) -> super::SQLRETURN);
    unsafe { SQLProcedureColumns(hstmt, szcatalogname.unwrap_or(core::mem::zeroed()) as _, cchcatalogname, szschemaname.unwrap_or(core::mem::zeroed()) as _, cchschemaname, szprocname.unwrap_or(core::mem::zeroed()) as _, cchprocname, szcolumnname.unwrap_or(core::mem::zeroed()) as _, cchcolumnname) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLProcedures(hstmt: super::SQLHSTMT, szcatalogname: Option<*const super::SQLCHAR>, cchcatalogname: super::SQLSMALLINT, szschemaname: Option<*const super::SQLCHAR>, cchschemaname: super::SQLSMALLINT, szprocname: Option<*const super::SQLCHAR>, cchprocname: super::SQLSMALLINT) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLProcedures(hstmt : super::SQLHSTMT, szcatalogname : *const super::SQLCHAR, cchcatalogname : super::SQLSMALLINT, szschemaname : *const super::SQLCHAR, cchschemaname : super::SQLSMALLINT, szprocname : *const super::SQLCHAR, cchprocname : super::SQLSMALLINT) -> super::SQLRETURN);
    unsafe { SQLProcedures(hstmt, szcatalogname.unwrap_or(core::mem::zeroed()) as _, cchcatalogname, szschemaname.unwrap_or(core::mem::zeroed()) as _, cchschemaname, szprocname.unwrap_or(core::mem::zeroed()) as _, cchprocname) }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLSetPos(hstmt: super::SQLHSTMT, irow: super::SQLUSMALLINT, foption: super::SQLUSMALLINT, flock: super::SQLUSMALLINT) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLSetPos(hstmt : super::SQLHSTMT, irow : super::SQLUSMALLINT, foption : super::SQLUSMALLINT, flock : super::SQLUSMALLINT) -> super::SQLRETURN);
    unsafe { SQLSetPos(hstmt, irow, foption, flock) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLSetPos(hstmt: super::SQLHSTMT, irow: super::SQLSETPOSIROW, foption: super::SQLUSMALLINT, flock: super::SQLUSMALLINT) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLSetPos(hstmt : super::SQLHSTMT, irow : super::SQLSETPOSIROW, foption : super::SQLUSMALLINT, flock : super::SQLUSMALLINT) -> super::SQLRETURN);
    unsafe { SQLSetPos(hstmt, irow, foption, flock) }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLSetScrollOptions(hstmt: super::SQLHSTMT, fconcurrency: super::SQLUSMALLINT, crowkeyset: super::SQLINTEGER, crowrowset: super::SQLUSMALLINT) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLSetScrollOptions(hstmt : super::SQLHSTMT, fconcurrency : super::SQLUSMALLINT, crowkeyset : super::SQLINTEGER, crowrowset : super::SQLUSMALLINT) -> super::SQLRETURN);
    unsafe { SQLSetScrollOptions(hstmt, fconcurrency, crowkeyset, crowrowset) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLSetScrollOptions(hstmt: super::SQLHSTMT, fconcurrency: super::SQLUSMALLINT, crowkeyset: super::SQLLEN, crowrowset: super::SQLUSMALLINT) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLSetScrollOptions(hstmt : super::SQLHSTMT, fconcurrency : super::SQLUSMALLINT, crowkeyset : super::SQLLEN, crowrowset : super::SQLUSMALLINT) -> super::SQLRETURN);
    unsafe { SQLSetScrollOptions(hstmt, fconcurrency, crowkeyset, crowrowset) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLTablePrivileges(hstmt: super::SQLHSTMT, szcatalogname: Option<*const super::SQLCHAR>, cchcatalogname: super::SQLSMALLINT, szschemaname: Option<*const super::SQLCHAR>, cchschemaname: super::SQLSMALLINT, sztablename: Option<*const super::SQLCHAR>, cchtablename: super::SQLSMALLINT) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLTablePrivileges(hstmt : super::SQLHSTMT, szcatalogname : *const super::SQLCHAR, cchcatalogname : super::SQLSMALLINT, szschemaname : *const super::SQLCHAR, cchschemaname : super::SQLSMALLINT, sztablename : *const super::SQLCHAR, cchtablename : super::SQLSMALLINT) -> super::SQLRETURN);
    unsafe { SQLTablePrivileges(hstmt, szcatalogname.unwrap_or(core::mem::zeroed()) as _, cchcatalogname, szschemaname.unwrap_or(core::mem::zeroed()) as _, cchschemaname, sztablename.unwrap_or(core::mem::zeroed()) as _, cchtablename) }
}
#[repr(C)]
#[cfg(feature = "sqltypes")]
#[derive(Clone, Copy)]
pub struct ODBC_VS_ARGS {
    pub pguidEvent: *const windows_core::GUID,
    pub dwFlags: u32,
    pub Anonymous: ODBC_VS_ARGS_0,
    pub Anonymous2: ODBC_VS_ARGS_1,
    pub RetCode: super::RETCODE,
}
#[cfg(feature = "sqltypes")]
impl Default for ODBC_VS_ARGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "sqltypes")]
#[derive(Clone, Copy)]
pub union ODBC_VS_ARGS_0 {
    pub wszArg: *mut u16,
    pub szArg: *mut i8,
}
#[cfg(feature = "sqltypes")]
impl Default for ODBC_VS_ARGS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "sqltypes")]
#[derive(Clone, Copy)]
pub union ODBC_VS_ARGS_1 {
    pub wszCorrelation: *mut u16,
    pub szCorrelation: *mut i8,
}
#[cfg(feature = "sqltypes")]
impl Default for ODBC_VS_ARGS_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ODBC_VS_FLAG_RETCODE: i32 = 4;
pub const ODBC_VS_FLAG_STOP: i32 = 8;
pub const ODBC_VS_FLAG_UNICODE_ARG: i32 = 1;
pub const ODBC_VS_FLAG_UNICODE_COR: i32 = 2;
#[cfg(feature = "sqltypes")]
pub type PODBC_VS_ARGS = *mut ODBC_VS_ARGS;
#[cfg(feature = "sqltypes")]
pub type SQLSTATE = [super::SQLTCHAR; 6];
pub const SQL_AA_FALSE: i32 = 0;
pub const SQL_AA_TRUE: i32 = 1;
pub const SQL_ACCESS_MODE: i32 = 101;
pub const SQL_ACTIVE_CONNECTIONS: i32 = 0;
pub const SQL_ACTIVE_ENVIRONMENTS: i32 = 116;
pub const SQL_ACTIVE_STATEMENTS: i32 = 1;
pub const SQL_ADD: i32 = 4;
pub const SQL_AD_ADD_CONSTRAINT_DEFERRABLE: i32 = 128;
pub const SQL_AD_ADD_CONSTRAINT_INITIALLY_DEFERRED: i32 = 32;
pub const SQL_AD_ADD_CONSTRAINT_INITIALLY_IMMEDIATE: i32 = 64;
pub const SQL_AD_ADD_CONSTRAINT_NON_DEFERRABLE: i32 = 256;
pub const SQL_AD_ADD_DOMAIN_CONSTRAINT: i32 = 2;
pub const SQL_AD_ADD_DOMAIN_DEFAULT: i32 = 8;
pub const SQL_AD_CONSTRAINT_NAME_DEFINITION: i32 = 1;
pub const SQL_AD_DROP_DOMAIN_CONSTRAINT: i32 = 4;
pub const SQL_AD_DROP_DOMAIN_DEFAULT: i32 = 16;
pub const SQL_AF_ALL: i32 = 64;
pub const SQL_AF_AVG: i32 = 1;
pub const SQL_AF_COUNT: i32 = 2;
pub const SQL_AF_DISTINCT: i32 = 32;
pub const SQL_AF_MAX: i32 = 4;
pub const SQL_AF_MIN: i32 = 8;
pub const SQL_AF_SUM: i32 = 16;
pub const SQL_AGGREGATE_FUNCTIONS: i32 = 169;
pub const SQL_ALL_CATALOGS: windows_core::PCSTR = windows_core::s!("%");
pub const SQL_ALL_EXCEPT_LIKE: i32 = 2;
pub const SQL_ALL_SCHEMAS: windows_core::PCSTR = windows_core::s!("%");
pub const SQL_ALL_TABLE_TYPES: windows_core::PCSTR = windows_core::s!("%");
pub const SQL_ALTER_DOMAIN: i32 = 117;
pub const SQL_API_ALL_FUNCTIONS: i32 = 0;
pub const SQL_API_LOADBYORDINAL: i32 = 199;
pub const SQL_API_ODBC3_ALL_FUNCTIONS: i32 = 999;
pub const SQL_API_ODBC3_ALL_FUNCTIONS_SIZE: i32 = 250;
pub const SQL_API_SQLALLOCHANDLESTD: i32 = 73;
pub const SQL_API_SQLBINDPARAMETER: i32 = 72;
pub const SQL_API_SQLBROWSECONNECT: i32 = 55;
pub const SQL_API_SQLBULKOPERATIONS: i32 = 24;
pub const SQL_API_SQLCOLATTRIBUTES: i32 = 6;
pub const SQL_API_SQLCOLUMNPRIVILEGES: i32 = 56;
pub const SQL_API_SQLDESCRIBEPARAM: i32 = 58;
pub const SQL_API_SQLDRIVERCONNECT: i32 = 41;
pub const SQL_API_SQLDRIVERS: i32 = 71;
pub const SQL_API_SQLEXTENDEDFETCH: i32 = 59;
pub const SQL_API_SQLFOREIGNKEYS: i32 = 60;
pub const SQL_API_SQLMORERESULTS: i32 = 61;
pub const SQL_API_SQLNATIVESQL: i32 = 62;
pub const SQL_API_SQLNUMPARAMS: i32 = 63;
pub const SQL_API_SQLPARAMOPTIONS: i32 = 64;
pub const SQL_API_SQLPRIMARYKEYS: i32 = 65;
pub const SQL_API_SQLPRIVATEDRIVERS: i32 = 79;
pub const SQL_API_SQLPROCEDURECOLUMNS: i32 = 66;
pub const SQL_API_SQLPROCEDURES: i32 = 67;
pub const SQL_API_SQLSETPOS: i32 = 68;
pub const SQL_API_SQLSETSCROLLOPTIONS: i32 = 69;
pub const SQL_API_SQLTABLEPRIVILEGES: i32 = 70;
pub const SQL_ASYNC_DBC_CAPABLE: i32 = 1;
pub const SQL_ASYNC_DBC_ENABLE_DEFAULT: u32 = 0;
pub const SQL_ASYNC_DBC_ENABLE_OFF: u32 = 0;
pub const SQL_ASYNC_DBC_ENABLE_ON: u32 = 1;
pub const SQL_ASYNC_DBC_FUNCTIONS: i32 = 10023;
pub const SQL_ASYNC_DBC_NOT_CAPABLE: i32 = 0;
pub const SQL_ASYNC_ENABLE: i32 = 4;
pub const SQL_ASYNC_ENABLE_DEFAULT: u32 = 0;
pub const SQL_ASYNC_ENABLE_OFF: u32 = 0;
pub const SQL_ASYNC_ENABLE_ON: u32 = 1;
pub const SQL_ASYNC_MODE: i32 = 10021;
pub const SQL_ASYNC_NOTIFICATION: i32 = 10025;
pub const SQL_ASYNC_NOTIFICATION_CAPABLE: i32 = 1;
pub const SQL_ASYNC_NOTIFICATION_NOT_CAPABLE: i32 = 0;
pub const SQL_ATTR_ACCESS_MODE: i32 = 101;
pub const SQL_ATTR_ANSI_APP: i32 = 115;
pub const SQL_ATTR_APPLICATION_KEY: i32 = 203;
pub const SQL_ATTR_ASYNC_DBC_EVENT: i32 = 119;
pub const SQL_ATTR_ASYNC_DBC_FUNCTIONS_ENABLE: i32 = 117;
pub const SQL_ATTR_ASYNC_ENABLE: i32 = 4;
pub const SQL_ATTR_ASYNC_STMT_EVENT: i32 = 29;
pub const SQL_ATTR_AUTOCOMMIT: i32 = 102;
pub const SQL_ATTR_CONCURRENCY: i32 = 7;
pub const SQL_ATTR_CONNECTION_DEAD: i32 = 1209;
pub const SQL_ATTR_CONNECTION_POOLING: i32 = 201;
pub const SQL_ATTR_CONNECTION_TIMEOUT: i32 = 113;
pub const SQL_ATTR_CP_MATCH: i32 = 202;
pub const SQL_ATTR_CURRENT_CATALOG: i32 = 109;
pub const SQL_ATTR_CURSOR_TYPE: i32 = 6;
pub const SQL_ATTR_DISCONNECT_BEHAVIOR: i32 = 114;
pub const SQL_ATTR_ENABLE_AUTO_IPD: i32 = 15;
pub const SQL_ATTR_ENLIST_IN_DTC: i32 = 1207;
pub const SQL_ATTR_ENLIST_IN_XA: i32 = 1208;
pub const SQL_ATTR_FETCH_BOOKMARK_PTR: i32 = 16;
pub const SQL_ATTR_KEYSET_SIZE: i32 = 8;
pub const SQL_ATTR_LOGIN_TIMEOUT: i32 = 103;
pub const SQL_ATTR_MAX_LENGTH: i32 = 3;
pub const SQL_ATTR_MAX_ROWS: i32 = 1;
pub const SQL_ATTR_NOSCAN: i32 = 2;
pub const SQL_ATTR_ODBC_CURSORS: i32 = 110;
pub const SQL_ATTR_ODBC_VERSION: i32 = 200;
pub const SQL_ATTR_PACKET_SIZE: i32 = 112;
pub const SQL_ATTR_PARAMSET_SIZE: i32 = 22;
pub const SQL_ATTR_PARAMS_PROCESSED_PTR: i32 = 21;
pub const SQL_ATTR_PARAM_BIND_OFFSET_PTR: i32 = 17;
pub const SQL_ATTR_PARAM_BIND_TYPE: i32 = 18;
pub const SQL_ATTR_PARAM_OPERATION_PTR: i32 = 19;
pub const SQL_ATTR_PARAM_STATUS_PTR: i32 = 20;
pub const SQL_ATTR_QUERY_TIMEOUT: i32 = 0;
pub const SQL_ATTR_QUIET_MODE: i32 = 111;
pub const SQL_ATTR_READONLY: i32 = 0;
pub const SQL_ATTR_READWRITE_UNKNOWN: i32 = 2;
pub const SQL_ATTR_RESET_CONNECTION: i32 = 116;
pub const SQL_ATTR_RETRIEVE_DATA: i32 = 11;
pub const SQL_ATTR_ROWS_FETCHED_PTR: i32 = 26;
pub const SQL_ATTR_ROW_ARRAY_SIZE: i32 = 27;
pub const SQL_ATTR_ROW_BIND_OFFSET_PTR: i32 = 23;
pub const SQL_ATTR_ROW_BIND_TYPE: i32 = 5;
pub const SQL_ATTR_ROW_NUMBER: i32 = 14;
pub const SQL_ATTR_ROW_OPERATION_PTR: i32 = 24;
pub const SQL_ATTR_ROW_STATUS_PTR: i32 = 25;
pub const SQL_ATTR_SIMULATE_CURSOR: i32 = 10;
pub const SQL_ATTR_TRACE: i32 = 104;
pub const SQL_ATTR_TRACEFILE: i32 = 105;
pub const SQL_ATTR_TRANSLATE_LIB: i32 = 106;
pub const SQL_ATTR_TRANSLATE_OPTION: i32 = 107;
pub const SQL_ATTR_TXN_ISOLATION: i32 = 108;
pub const SQL_ATTR_USE_BOOKMARKS: i32 = 12;
pub const SQL_ATTR_WRITE: i32 = 1;
pub const SQL_AT_ADD_COLUMN_COLLATION: i32 = 128;
pub const SQL_AT_ADD_COLUMN_DEFAULT: i32 = 64;
pub const SQL_AT_ADD_COLUMN_SINGLE: i32 = 32;
pub const SQL_AT_ADD_TABLE_CONSTRAINT: i32 = 4096;
pub const SQL_AT_CONSTRAINT_DEFERRABLE: i32 = 262144;
pub const SQL_AT_CONSTRAINT_INITIALLY_DEFERRED: i32 = 65536;
pub const SQL_AT_CONSTRAINT_INITIALLY_IMMEDIATE: i32 = 131072;
pub const SQL_AT_CONSTRAINT_NAME_DEFINITION: i32 = 32768;
pub const SQL_AT_CONSTRAINT_NON_DEFERRABLE: i32 = 524288;
pub const SQL_AT_DROP_COLUMN_CASCADE: i32 = 1024;
pub const SQL_AT_DROP_COLUMN_DEFAULT: i32 = 512;
pub const SQL_AT_DROP_COLUMN_RESTRICT: i32 = 2048;
pub const SQL_AT_DROP_TABLE_CONSTRAINT_CASCADE: i32 = 8192;
pub const SQL_AT_DROP_TABLE_CONSTRAINT_RESTRICT: i32 = 16384;
pub const SQL_AT_SET_COLUMN_DEFAULT: i32 = 256;
pub const SQL_AUTOCOMMIT: i32 = 102;
pub const SQL_AUTOCOMMIT_DEFAULT: u32 = 1;
pub const SQL_AUTOCOMMIT_OFF: u32 = 0;
pub const SQL_AUTOCOMMIT_ON: u32 = 1;
pub const SQL_BATCH_ROW_COUNT: i32 = 120;
pub const SQL_BATCH_SUPPORT: i32 = 121;
pub const SQL_BEST_ROWID: i32 = 1;
pub const SQL_BIGINT: i32 = -5;
pub const SQL_BINARY: i32 = -2;
pub const SQL_BIND_BY_COLUMN: u32 = 0;
pub const SQL_BIND_TYPE: i32 = 5;
pub const SQL_BIND_TYPE_DEFAULT: u32 = 0;
pub const SQL_BIT: i32 = -7;
pub const SQL_BOOKMARK_PERSISTENCE: i32 = 82;
pub const SQL_BP_CLOSE: i32 = 1;
pub const SQL_BP_DELETE: i32 = 2;
pub const SQL_BP_DROP: i32 = 4;
pub const SQL_BP_OTHER_HSTMT: i32 = 32;
pub const SQL_BP_SCROLL: i32 = 64;
pub const SQL_BP_TRANSACTION: i32 = 8;
pub const SQL_BP_UPDATE: i32 = 16;
pub const SQL_BRC_EXPLICIT: i32 = 2;
pub const SQL_BRC_PROCEDURES: i32 = 1;
pub const SQL_BRC_ROLLED_UP: i32 = 4;
pub const SQL_BS_ROW_COUNT_EXPLICIT: i32 = 2;
pub const SQL_BS_ROW_COUNT_PROC: i32 = 8;
pub const SQL_BS_SELECT_EXPLICIT: i32 = 1;
pub const SQL_BS_SELECT_PROC: i32 = 4;
pub const SQL_CA1_ABSOLUTE: i32 = 2;
pub const SQL_CA1_BOOKMARK: i32 = 8;
pub const SQL_CA1_BULK_ADD: i32 = 65536;
pub const SQL_CA1_BULK_DELETE_BY_BOOKMARK: i32 = 262144;
pub const SQL_CA1_BULK_FETCH_BY_BOOKMARK: i32 = 524288;
pub const SQL_CA1_BULK_UPDATE_BY_BOOKMARK: i32 = 131072;
pub const SQL_CA1_LOCK_EXCLUSIVE: i32 = 128;
pub const SQL_CA1_LOCK_NO_CHANGE: i32 = 64;
pub const SQL_CA1_LOCK_UNLOCK: i32 = 256;
pub const SQL_CA1_NEXT: i32 = 1;
pub const SQL_CA1_POSITIONED_DELETE: i32 = 16384;
pub const SQL_CA1_POSITIONED_UPDATE: i32 = 8192;
pub const SQL_CA1_POS_DELETE: i32 = 2048;
pub const SQL_CA1_POS_POSITION: i32 = 512;
pub const SQL_CA1_POS_REFRESH: i32 = 4096;
pub const SQL_CA1_POS_UPDATE: i32 = 1024;
pub const SQL_CA1_RELATIVE: i32 = 4;
pub const SQL_CA1_SELECT_FOR_UPDATE: i32 = 32768;
pub const SQL_CA2_CRC_APPROXIMATE: i32 = 8192;
pub const SQL_CA2_CRC_EXACT: i32 = 4096;
pub const SQL_CA2_LOCK_CONCURRENCY: i32 = 2;
pub const SQL_CA2_MAX_ROWS_AFFECTS_ALL: i32 = 3968;
pub const SQL_CA2_MAX_ROWS_CATALOG: i32 = 2048;
pub const SQL_CA2_MAX_ROWS_DELETE: i32 = 512;
pub const SQL_CA2_MAX_ROWS_INSERT: i32 = 256;
pub const SQL_CA2_MAX_ROWS_SELECT: i32 = 128;
pub const SQL_CA2_MAX_ROWS_UPDATE: i32 = 1024;
pub const SQL_CA2_OPT_ROWVER_CONCURRENCY: i32 = 4;
pub const SQL_CA2_OPT_VALUES_CONCURRENCY: i32 = 8;
pub const SQL_CA2_READ_ONLY_CONCURRENCY: i32 = 1;
pub const SQL_CA2_SENSITIVITY_ADDITIONS: i32 = 16;
pub const SQL_CA2_SENSITIVITY_DELETIONS: i32 = 32;
pub const SQL_CA2_SENSITIVITY_UPDATES: i32 = 64;
pub const SQL_CA2_SIMULATE_NON_UNIQUE: i32 = 16384;
pub const SQL_CA2_SIMULATE_TRY_UNIQUE: i32 = 32768;
pub const SQL_CA2_SIMULATE_UNIQUE: i32 = 65536;
pub const SQL_CASCADE: i32 = 0;
pub const SQL_CATALOG_LOCATION: i32 = 114;
pub const SQL_CATALOG_NAME_SEPARATOR: i32 = 41;
pub const SQL_CATALOG_TERM: i32 = 42;
pub const SQL_CATALOG_USAGE: i32 = 92;
pub const SQL_CA_CONSTRAINT_DEFERRABLE: i32 = 64;
pub const SQL_CA_CONSTRAINT_INITIALLY_DEFERRED: i32 = 16;
pub const SQL_CA_CONSTRAINT_INITIALLY_IMMEDIATE: i32 = 32;
pub const SQL_CA_CONSTRAINT_NON_DEFERRABLE: i32 = 128;
pub const SQL_CA_CREATE_ASSERTION: i32 = 1;
pub const SQL_CB_NON_NULL: i32 = 1;
pub const SQL_CB_NULL: i32 = 0;
pub const SQL_CCOL_CREATE_COLLATION: i32 = 1;
pub const SQL_CCS_COLLATE_CLAUSE: i32 = 2;
pub const SQL_CCS_CREATE_CHARACTER_SET: i32 = 1;
pub const SQL_CCS_LIMITED_COLLATION: i32 = 4;
pub const SQL_CC_CLOSE: i32 = 1;
pub const SQL_CC_DELETE: i32 = 0;
pub const SQL_CC_PRESERVE: i32 = 2;
pub const SQL_CDO_COLLATION: i32 = 8;
pub const SQL_CDO_CONSTRAINT: i32 = 4;
pub const SQL_CDO_CONSTRAINT_DEFERRABLE: i32 = 128;
pub const SQL_CDO_CONSTRAINT_INITIALLY_DEFERRED: i32 = 32;
pub const SQL_CDO_CONSTRAINT_INITIALLY_IMMEDIATE: i32 = 64;
pub const SQL_CDO_CONSTRAINT_NAME_DEFINITION: i32 = 16;
pub const SQL_CDO_CONSTRAINT_NON_DEFERRABLE: i32 = 256;
pub const SQL_CDO_CREATE_DOMAIN: i32 = 1;
pub const SQL_CDO_DEFAULT: i32 = 2;
pub const SQL_CD_FALSE: i32 = 0;
pub const SQL_CD_TRUE: i32 = 1;
pub const SQL_CL_END: i32 = 2;
pub const SQL_CL_START: i32 = 1;
pub const SQL_CN_ANY: i32 = 2;
pub const SQL_CN_DIFFERENT: i32 = 1;
pub const SQL_CN_NONE: i32 = 0;
pub const SQL_CODE_DAY: i32 = 3;
pub const SQL_CODE_DAY_TO_HOUR: i32 = 8;
pub const SQL_CODE_DAY_TO_MINUTE: i32 = 9;
pub const SQL_CODE_DAY_TO_SECOND: i32 = 10;
pub const SQL_CODE_HOUR: i32 = 4;
pub const SQL_CODE_HOUR_TO_MINUTE: i32 = 11;
pub const SQL_CODE_HOUR_TO_SECOND: i32 = 12;
pub const SQL_CODE_MINUTE: i32 = 5;
pub const SQL_CODE_MINUTE_TO_SECOND: i32 = 13;
pub const SQL_CODE_MONTH: i32 = 2;
pub const SQL_CODE_SECOND: i32 = 6;
pub const SQL_CODE_YEAR: i32 = 1;
pub const SQL_CODE_YEAR_TO_MONTH: i32 = 7;
pub const SQL_COLATT_OPT_MAX: i32 = 18;
pub const SQL_COLATT_OPT_MIN: i32 = 0;
pub const SQL_COLUMN_ALIAS: i32 = 87;
pub const SQL_COLUMN_AUTO_INCREMENT: i32 = 11;
pub const SQL_COLUMN_CASE_SENSITIVE: i32 = 12;
pub const SQL_COLUMN_COUNT: i32 = 0;
pub const SQL_COLUMN_DISPLAY_SIZE: i32 = 6;
pub const SQL_COLUMN_IGNORE: i32 = -6;
pub const SQL_COLUMN_LABEL: i32 = 18;
pub const SQL_COLUMN_LENGTH: i32 = 3;
pub const SQL_COLUMN_MONEY: i32 = 9;
pub const SQL_COLUMN_NAME: i32 = 1;
pub const SQL_COLUMN_NULLABLE: i32 = 7;
pub const SQL_COLUMN_NUMBER_UNKNOWN: i32 = -2;
pub const SQL_COLUMN_OWNER_NAME: i32 = 16;
pub const SQL_COLUMN_PRECISION: i32 = 4;
pub const SQL_COLUMN_QUALIFIER_NAME: i32 = 17;
pub const SQL_COLUMN_SCALE: i32 = 5;
pub const SQL_COLUMN_SEARCHABLE: i32 = 13;
pub const SQL_COLUMN_TABLE_NAME: i32 = 15;
pub const SQL_COLUMN_TYPE: i32 = 2;
pub const SQL_COLUMN_TYPE_NAME: i32 = 14;
pub const SQL_COLUMN_UNSIGNED: i32 = 8;
pub const SQL_COLUMN_UPDATABLE: i32 = 10;
pub const SQL_COL_PRED_BASIC: i32 = 2;
pub const SQL_COL_PRED_CHAR: i32 = 1;
pub const SQL_CONCAT_NULL_BEHAVIOR: i32 = 22;
pub const SQL_CONCURRENCY: i32 = 7;
pub const SQL_CONCUR_DEFAULT: i32 = 1;
pub const SQL_CONCUR_LOCK: i32 = 2;
pub const SQL_CONCUR_READ_ONLY: i32 = 1;
pub const SQL_CONCUR_ROWVER: i32 = 3;
pub const SQL_CONCUR_TIMESTAMP: i32 = 3;
pub const SQL_CONCUR_VALUES: i32 = 4;
pub const SQL_CONVERT_BIGINT: i32 = 53;
pub const SQL_CONVERT_BINARY: i32 = 54;
pub const SQL_CONVERT_BIT: i32 = 55;
pub const SQL_CONVERT_CHAR: i32 = 56;
pub const SQL_CONVERT_DATE: i32 = 57;
pub const SQL_CONVERT_DECIMAL: i32 = 58;
pub const SQL_CONVERT_DOUBLE: i32 = 59;
pub const SQL_CONVERT_FLOAT: i32 = 60;
pub const SQL_CONVERT_FUNCTIONS: i32 = 48;
pub const SQL_CONVERT_GUID: i32 = 173;
pub const SQL_CONVERT_INTEGER: i32 = 61;
pub const SQL_CONVERT_INTERVAL_DAY_TIME: i32 = 123;
pub const SQL_CONVERT_INTERVAL_YEAR_MONTH: i32 = 124;
pub const SQL_CONVERT_LONGVARBINARY: i32 = 71;
pub const SQL_CONVERT_LONGVARCHAR: i32 = 62;
pub const SQL_CONVERT_NUMERIC: i32 = 63;
pub const SQL_CONVERT_REAL: i32 = 64;
pub const SQL_CONVERT_SMALLINT: i32 = 65;
pub const SQL_CONVERT_TIME: i32 = 66;
pub const SQL_CONVERT_TIMESTAMP: i32 = 67;
pub const SQL_CONVERT_TINYINT: i32 = 68;
pub const SQL_CONVERT_VARBINARY: i32 = 69;
pub const SQL_CONVERT_VARCHAR: i32 = 70;
pub const SQL_CONVERT_WCHAR: i32 = 122;
pub const SQL_CONVERT_WLONGVARCHAR: i32 = 125;
pub const SQL_CONVERT_WVARCHAR: i32 = 126;
pub const SQL_CORRELATION_NAME: i32 = 74;
pub const SQL_CP_DEFAULT: u32 = 0;
pub const SQL_CP_DRIVER_AWARE: u32 = 3;
pub const SQL_CP_MATCH_DEFAULT: u32 = 0;
pub const SQL_CP_OFF: u32 = 0;
pub const SQL_CP_ONE_PER_DRIVER: u32 = 1;
pub const SQL_CP_ONE_PER_HENV: u32 = 2;
pub const SQL_CP_RELAXED_MATCH: u32 = 1;
pub const SQL_CP_STRICT_MATCH: u32 = 0;
pub const SQL_CREATE_ASSERTION: i32 = 127;
pub const SQL_CREATE_CHARACTER_SET: i32 = 128;
pub const SQL_CREATE_COLLATION: i32 = 129;
pub const SQL_CREATE_DOMAIN: i32 = 130;
pub const SQL_CREATE_SCHEMA: i32 = 131;
pub const SQL_CREATE_TABLE: i32 = 132;
pub const SQL_CREATE_TRANSLATION: i32 = 133;
pub const SQL_CREATE_VIEW: i32 = 134;
pub const SQL_CR_CLOSE: i32 = 1;
pub const SQL_CR_DELETE: i32 = 0;
pub const SQL_CR_PRESERVE: i32 = 2;
pub const SQL_CS_AUTHORIZATION: i32 = 2;
pub const SQL_CS_CREATE_SCHEMA: i32 = 1;
pub const SQL_CS_DEFAULT_CHARACTER_SET: i32 = 4;
pub const SQL_CTR_CREATE_TRANSLATION: i32 = 1;
pub const SQL_CT_COLUMN_COLLATION: i32 = 2048;
pub const SQL_CT_COLUMN_CONSTRAINT: i32 = 512;
pub const SQL_CT_COLUMN_DEFAULT: i32 = 1024;
pub const SQL_CT_COMMIT_DELETE: i32 = 4;
pub const SQL_CT_COMMIT_PRESERVE: i32 = 2;
pub const SQL_CT_CONSTRAINT_DEFERRABLE: i32 = 128;
pub const SQL_CT_CONSTRAINT_INITIALLY_DEFERRED: i32 = 32;
pub const SQL_CT_CONSTRAINT_INITIALLY_IMMEDIATE: i32 = 64;
pub const SQL_CT_CONSTRAINT_NAME_DEFINITION: i32 = 8192;
pub const SQL_CT_CONSTRAINT_NON_DEFERRABLE: i32 = 256;
pub const SQL_CT_CREATE_TABLE: i32 = 1;
pub const SQL_CT_GLOBAL_TEMPORARY: i32 = 8;
pub const SQL_CT_LOCAL_TEMPORARY: i32 = 16;
pub const SQL_CT_TABLE_CONSTRAINT: i32 = 4096;
pub const SQL_CURRENT_QUALIFIER: i32 = 109;
pub const SQL_CURSOR_DYNAMIC: u32 = 2;
pub const SQL_CURSOR_FORWARD_ONLY: u32 = 0;
pub const SQL_CURSOR_KEYSET_DRIVEN: u32 = 1;
pub const SQL_CURSOR_ROLLBACK_BEHAVIOR: i32 = 24;
pub const SQL_CURSOR_STATIC: u32 = 3;
pub const SQL_CURSOR_TYPE: i32 = 6;
pub const SQL_CURSOR_TYPE_DEFAULT: u32 = 0;
pub const SQL_CUR_DEFAULT: u32 = 2;
pub const SQL_CUR_USE_DRIVER: u32 = 2;
pub const SQL_CUR_USE_IF_NEEDED: u32 = 0;
pub const SQL_CUR_USE_ODBC: u32 = 1;
pub const SQL_CU_DML_STATEMENTS: i32 = 1;
pub const SQL_CU_INDEX_DEFINITION: i32 = 8;
pub const SQL_CU_PRIVILEGE_DEFINITION: i32 = 16;
pub const SQL_CU_PROCEDURE_INVOCATION: i32 = 2;
pub const SQL_CU_TABLE_DEFINITION: i32 = 4;
pub const SQL_CVT_BIGINT: i32 = 16384;
pub const SQL_CVT_BINARY: i32 = 1024;
pub const SQL_CVT_BIT: i32 = 4096;
pub const SQL_CVT_CHAR: i32 = 1;
pub const SQL_CVT_DATE: i32 = 32768;
pub const SQL_CVT_DECIMAL: i32 = 4;
pub const SQL_CVT_DOUBLE: i32 = 128;
pub const SQL_CVT_FLOAT: i32 = 32;
pub const SQL_CVT_GUID: i32 = 16777216;
pub const SQL_CVT_INTEGER: i32 = 8;
pub const SQL_CVT_INTERVAL_DAY_TIME: i32 = 1048576;
pub const SQL_CVT_INTERVAL_YEAR_MONTH: i32 = 524288;
pub const SQL_CVT_LONGVARBINARY: i32 = 262144;
pub const SQL_CVT_LONGVARCHAR: i32 = 512;
pub const SQL_CVT_NUMERIC: i32 = 2;
pub const SQL_CVT_REAL: i32 = 64;
pub const SQL_CVT_SMALLINT: i32 = 16;
pub const SQL_CVT_TIME: i32 = 65536;
pub const SQL_CVT_TIMESTAMP: i32 = 131072;
pub const SQL_CVT_TINYINT: i32 = 8192;
pub const SQL_CVT_VARBINARY: i32 = 2048;
pub const SQL_CVT_VARCHAR: i32 = 256;
pub const SQL_CVT_WCHAR: i32 = 2097152;
pub const SQL_CVT_WLONGVARCHAR: i32 = 4194304;
pub const SQL_CVT_WVARCHAR: i32 = 8388608;
pub const SQL_CV_CASCADED: i32 = 4;
pub const SQL_CV_CHECK_OPTION: i32 = 2;
pub const SQL_CV_CREATE_VIEW: i32 = 1;
pub const SQL_CV_LOCAL: i32 = 8;
pub const SQL_C_BINARY: i32 = -2;
pub const SQL_C_BIT: i32 = -7;
#[cfg(target_arch = "x86")]
pub const SQL_C_BOOKMARK: i32 = -18;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const SQL_C_BOOKMARK: i32 = -27;
pub const SQL_C_CHAR: i32 = 1;
pub const SQL_C_DATE: i32 = 9;
pub const SQL_C_DEFAULT: i32 = 99;
pub const SQL_C_DOUBLE: i32 = 8;
pub const SQL_C_FLOAT: i32 = 7;
pub const SQL_C_GUID: i32 = -11;
pub const SQL_C_INTERVAL_DAY: i32 = 103;
pub const SQL_C_INTERVAL_DAY_TO_HOUR: i32 = 108;
pub const SQL_C_INTERVAL_DAY_TO_MINUTE: i32 = 109;
pub const SQL_C_INTERVAL_DAY_TO_SECOND: i32 = 110;
pub const SQL_C_INTERVAL_HOUR: i32 = 104;
pub const SQL_C_INTERVAL_HOUR_TO_MINUTE: i32 = 111;
pub const SQL_C_INTERVAL_HOUR_TO_SECOND: i32 = 112;
pub const SQL_C_INTERVAL_MINUTE: i32 = 105;
pub const SQL_C_INTERVAL_MINUTE_TO_SECOND: i32 = 113;
pub const SQL_C_INTERVAL_MONTH: i32 = 102;
pub const SQL_C_INTERVAL_SECOND: i32 = 106;
pub const SQL_C_INTERVAL_YEAR: i32 = 101;
pub const SQL_C_INTERVAL_YEAR_TO_MONTH: i32 = 107;
pub const SQL_C_LONG: i32 = 4;
pub const SQL_C_NUMERIC: i32 = 2;
pub const SQL_C_SBIGINT: i32 = -25;
pub const SQL_C_SHORT: i32 = 5;
pub const SQL_C_SLONG: i32 = -16;
pub const SQL_C_SSHORT: i32 = -15;
pub const SQL_C_STINYINT: i32 = -26;
pub const SQL_C_TIME: i32 = 10;
pub const SQL_C_TIMESTAMP: i32 = 11;
pub const SQL_C_TINYINT: i32 = -6;
pub const SQL_C_TYPE_DATE: i32 = 91;
pub const SQL_C_TYPE_TIME: i32 = 92;
pub const SQL_C_TYPE_TIMESTAMP: i32 = 93;
pub const SQL_C_UBIGINT: i32 = -27;
pub const SQL_C_ULONG: i32 = -18;
pub const SQL_C_USHORT: i32 = -17;
pub const SQL_C_UTINYINT: i32 = -28;
pub const SQL_C_VARBOOKMARK: i32 = -2;
pub const SQL_DATABASE_NAME: i32 = 16;
pub const SQL_DATE: i32 = 9;
pub const SQL_DATETIME_LITERALS: i32 = 119;
pub const SQL_DA_DROP_ASSERTION: i32 = 1;
pub const SQL_DB_DEFAULT: u32 = 0;
pub const SQL_DB_DISCONNECT: u32 = 1;
pub const SQL_DB_RETURN_TO_POOL: u32 = 0;
pub const SQL_DCS_DROP_CHARACTER_SET: i32 = 1;
pub const SQL_DC_DROP_COLLATION: i32 = 1;
pub const SQL_DDL_INDEX: i32 = 170;
pub const SQL_DD_CASCADE: i32 = 4;
pub const SQL_DD_DROP_DOMAIN: i32 = 1;
pub const SQL_DD_RESTRICT: i32 = 2;
pub const SQL_DEFAULT_PARAM: i32 = -5;
pub const SQL_DELETE: i32 = 3;
pub const SQL_DELETE_BY_BOOKMARK: i32 = 6;
pub const SQL_DESC_ARRAY_SIZE: i32 = 20;
pub const SQL_DESC_ARRAY_STATUS_PTR: i32 = 21;
pub const SQL_DESC_AUTO_UNIQUE_VALUE: i32 = 11;
pub const SQL_DESC_BASE_COLUMN_NAME: i32 = 22;
pub const SQL_DESC_BASE_TABLE_NAME: i32 = 23;
pub const SQL_DESC_BIND_OFFSET_PTR: i32 = 24;
pub const SQL_DESC_BIND_TYPE: i32 = 25;
pub const SQL_DESC_CASE_SENSITIVE: i32 = 12;
pub const SQL_DESC_CATALOG_NAME: i32 = 17;
pub const SQL_DESC_CONCISE_TYPE: i32 = 2;
pub const SQL_DESC_DATETIME_INTERVAL_PRECISION: i32 = 26;
pub const SQL_DESC_DISPLAY_SIZE: i32 = 6;
pub const SQL_DESC_FIXED_PREC_SCALE: i32 = 9;
pub const SQL_DESC_LABEL: i32 = 18;
pub const SQL_DESC_LITERAL_PREFIX: i32 = 27;
pub const SQL_DESC_LITERAL_SUFFIX: i32 = 28;
pub const SQL_DESC_LOCAL_TYPE_NAME: i32 = 29;
pub const SQL_DESC_MAXIMUM_SCALE: i32 = 30;
pub const SQL_DESC_MINIMUM_SCALE: i32 = 31;
pub const SQL_DESC_NUM_PREC_RADIX: i32 = 32;
pub const SQL_DESC_PARAMETER_TYPE: i32 = 33;
pub const SQL_DESC_ROWS_PROCESSED_PTR: i32 = 34;
pub const SQL_DESC_ROWVER: i32 = 35;
pub const SQL_DESC_SCHEMA_NAME: i32 = 16;
pub const SQL_DESC_SEARCHABLE: i32 = 13;
pub const SQL_DESC_TABLE_NAME: i32 = 15;
pub const SQL_DESC_TYPE_NAME: i32 = 14;
pub const SQL_DESC_UNSIGNED: i32 = 8;
pub const SQL_DESC_UPDATABLE: i32 = 10;
pub const SQL_DIAG_COLUMN_NUMBER: i32 = -1247;
pub const SQL_DIAG_CURSOR_ROW_COUNT: i32 = -1249;
pub const SQL_DIAG_ROW_NUMBER: i32 = -1248;
pub const SQL_DI_CREATE_INDEX: i32 = 1;
pub const SQL_DI_DROP_INDEX: i32 = 2;
pub const SQL_DL_SQL92_DATE: i32 = 1;
pub const SQL_DL_SQL92_INTERVAL_DAY: i32 = 32;
pub const SQL_DL_SQL92_INTERVAL_DAY_TO_HOUR: i32 = 1024;
pub const SQL_DL_SQL92_INTERVAL_DAY_TO_MINUTE: i32 = 2048;
pub const SQL_DL_SQL92_INTERVAL_DAY_TO_SECOND: i32 = 4096;
pub const SQL_DL_SQL92_INTERVAL_HOUR: i32 = 64;
pub const SQL_DL_SQL92_INTERVAL_HOUR_TO_MINUTE: i32 = 8192;
pub const SQL_DL_SQL92_INTERVAL_HOUR_TO_SECOND: i32 = 16384;
pub const SQL_DL_SQL92_INTERVAL_MINUTE: i32 = 128;
pub const SQL_DL_SQL92_INTERVAL_MINUTE_TO_SECOND: i32 = 32768;
pub const SQL_DL_SQL92_INTERVAL_MONTH: i32 = 16;
pub const SQL_DL_SQL92_INTERVAL_SECOND: i32 = 256;
pub const SQL_DL_SQL92_INTERVAL_YEAR: i32 = 8;
pub const SQL_DL_SQL92_INTERVAL_YEAR_TO_MONTH: i32 = 512;
pub const SQL_DL_SQL92_TIME: i32 = 2;
pub const SQL_DL_SQL92_TIMESTAMP: i32 = 4;
pub const SQL_DM_VER: i32 = 171;
pub const SQL_DRIVER_AWARE_POOLING_CAPABLE: i32 = 1;
pub const SQL_DRIVER_AWARE_POOLING_NOT_CAPABLE: i32 = 0;
pub const SQL_DRIVER_AWARE_POOLING_SUPPORTED: i32 = 10024;
pub const SQL_DRIVER_COMPLETE: i32 = 1;
pub const SQL_DRIVER_COMPLETE_REQUIRED: i32 = 3;
pub const SQL_DRIVER_CONN_ATTR_BASE: i32 = 16384;
pub const SQL_DRIVER_C_TYPE_BASE: i32 = 16384;
pub const SQL_DRIVER_DESC_FIELD_BASE: i32 = 16384;
pub const SQL_DRIVER_DIAG_FIELD_BASE: i32 = 16384;
pub const SQL_DRIVER_HDBC: i32 = 3;
pub const SQL_DRIVER_HDESC: i32 = 135;
pub const SQL_DRIVER_HENV: i32 = 4;
pub const SQL_DRIVER_HLIB: i32 = 76;
pub const SQL_DRIVER_HSTMT: i32 = 5;
pub const SQL_DRIVER_INFO_TYPE_BASE: i32 = 16384;
pub const SQL_DRIVER_NAME: i32 = 6;
pub const SQL_DRIVER_NOPROMPT: i32 = 0;
pub const SQL_DRIVER_ODBC_VER: i32 = 77;
pub const SQL_DRIVER_PROMPT: i32 = 2;
pub const SQL_DRIVER_SQL_TYPE_BASE: i32 = 16384;
pub const SQL_DRIVER_STMT_ATTR_BASE: i32 = 16384;
pub const SQL_DRIVER_VER: i32 = 7;
pub const SQL_DROP_ASSERTION: i32 = 136;
pub const SQL_DROP_CHARACTER_SET: i32 = 137;
pub const SQL_DROP_COLLATION: i32 = 138;
pub const SQL_DROP_DOMAIN: i32 = 139;
pub const SQL_DROP_SCHEMA: i32 = 140;
pub const SQL_DROP_TABLE: i32 = 141;
pub const SQL_DROP_TRANSLATION: i32 = 142;
pub const SQL_DROP_VIEW: i32 = 143;
pub const SQL_DS_CASCADE: i32 = 4;
pub const SQL_DS_DROP_SCHEMA: i32 = 1;
pub const SQL_DS_RESTRICT: i32 = 2;
pub const SQL_DTC_DONE: i32 = 0;
pub const SQL_DTC_ENLIST_EXPENSIVE: i32 = 1;
pub const SQL_DTC_TRANSITION_COST: i32 = 1750;
pub const SQL_DTC_UNENLIST_EXPENSIVE: i32 = 2;
pub const SQL_DTR_DROP_TRANSLATION: i32 = 1;
pub const SQL_DT_CASCADE: i32 = 4;
pub const SQL_DT_DROP_TABLE: i32 = 1;
pub const SQL_DT_RESTRICT: i32 = 2;
pub const SQL_DV_CASCADE: i32 = 4;
pub const SQL_DV_DROP_VIEW: i32 = 1;
pub const SQL_DV_RESTRICT: i32 = 2;
pub const SQL_DYNAMIC_CURSOR_ATTRIBUTES1: i32 = 144;
pub const SQL_DYNAMIC_CURSOR_ATTRIBUTES2: i32 = 145;
pub const SQL_ENSURE: i32 = 1;
pub const SQL_ENTIRE_ROWSET: i32 = 0;
pub const SQL_EXPRESSIONS_IN_ORDERBY: i32 = 27;
pub const SQL_FD_FETCH_BOOKMARK: i32 = 128;
pub const SQL_FD_FETCH_PREV: i32 = 8;
pub const SQL_FETCH_BOOKMARK: i32 = 8;
pub const SQL_FETCH_BY_BOOKMARK: i32 = 7;
pub const SQL_FETCH_FIRST_SYSTEM: i32 = 32;
pub const SQL_FETCH_FIRST_USER: i32 = 31;
pub const SQL_FETCH_PREV: i32 = 4;
pub const SQL_FILE_CATALOG: i32 = 2;
pub const SQL_FILE_NOT_SUPPORTED: i32 = 0;
pub const SQL_FILE_QUALIFIER: i32 = 2;
pub const SQL_FILE_TABLE: i32 = 1;
pub const SQL_FILE_USAGE: i32 = 84;
pub const SQL_FN_CVT_CAST: i32 = 2;
pub const SQL_FN_CVT_CONVERT: i32 = 1;
pub const SQL_FN_NUM_ABS: i32 = 1;
pub const SQL_FN_NUM_ACOS: i32 = 2;
pub const SQL_FN_NUM_ASIN: i32 = 4;
pub const SQL_FN_NUM_ATAN: i32 = 8;
pub const SQL_FN_NUM_ATAN2: i32 = 16;
pub const SQL_FN_NUM_CEILING: i32 = 32;
pub const SQL_FN_NUM_COS: i32 = 64;
pub const SQL_FN_NUM_COT: i32 = 128;
pub const SQL_FN_NUM_DEGREES: i32 = 262144;
pub const SQL_FN_NUM_EXP: i32 = 256;
pub const SQL_FN_NUM_FLOOR: i32 = 512;
pub const SQL_FN_NUM_LOG: i32 = 1024;
pub const SQL_FN_NUM_LOG10: i32 = 524288;
pub const SQL_FN_NUM_MOD: i32 = 2048;
pub const SQL_FN_NUM_PI: i32 = 65536;
pub const SQL_FN_NUM_POWER: i32 = 1048576;
pub const SQL_FN_NUM_RADIANS: i32 = 2097152;
pub const SQL_FN_NUM_RAND: i32 = 131072;
pub const SQL_FN_NUM_ROUND: i32 = 4194304;
pub const SQL_FN_NUM_SIGN: i32 = 4096;
pub const SQL_FN_NUM_SIN: i32 = 8192;
pub const SQL_FN_NUM_SQRT: i32 = 16384;
pub const SQL_FN_NUM_TAN: i32 = 32768;
pub const SQL_FN_NUM_TRUNCATE: i32 = 8388608;
pub const SQL_FN_STR_ASCII: i32 = 8192;
pub const SQL_FN_STR_BIT_LENGTH: i32 = 524288;
pub const SQL_FN_STR_CHAR: i32 = 16384;
pub const SQL_FN_STR_CHARACTER_LENGTH: i32 = 2097152;
pub const SQL_FN_STR_CHAR_LENGTH: i32 = 1048576;
pub const SQL_FN_STR_CONCAT: i32 = 1;
pub const SQL_FN_STR_DIFFERENCE: i32 = 32768;
pub const SQL_FN_STR_INSERT: i32 = 2;
pub const SQL_FN_STR_LCASE: i32 = 64;
pub const SQL_FN_STR_LEFT: i32 = 4;
pub const SQL_FN_STR_LENGTH: i32 = 16;
pub const SQL_FN_STR_LOCATE: i32 = 32;
pub const SQL_FN_STR_LOCATE_2: i32 = 65536;
pub const SQL_FN_STR_LTRIM: i32 = 8;
pub const SQL_FN_STR_OCTET_LENGTH: i32 = 4194304;
pub const SQL_FN_STR_POSITION: i32 = 8388608;
pub const SQL_FN_STR_REPEAT: i32 = 128;
pub const SQL_FN_STR_REPLACE: i32 = 256;
pub const SQL_FN_STR_RIGHT: i32 = 512;
pub const SQL_FN_STR_RTRIM: i32 = 1024;
pub const SQL_FN_STR_SOUNDEX: i32 = 131072;
pub const SQL_FN_STR_SPACE: i32 = 262144;
pub const SQL_FN_STR_SUBSTRING: i32 = 2048;
pub const SQL_FN_STR_UCASE: i32 = 4096;
pub const SQL_FN_SYS_DBNAME: i32 = 2;
pub const SQL_FN_SYS_IFNULL: i32 = 4;
pub const SQL_FN_SYS_USERNAME: i32 = 1;
pub const SQL_FN_TD_CURDATE: i32 = 2;
pub const SQL_FN_TD_CURRENT_DATE: i32 = 131072;
pub const SQL_FN_TD_CURRENT_TIME: i32 = 262144;
pub const SQL_FN_TD_CURRENT_TIMESTAMP: i32 = 524288;
pub const SQL_FN_TD_CURTIME: i32 = 512;
pub const SQL_FN_TD_DAYNAME: i32 = 32768;
pub const SQL_FN_TD_DAYOFMONTH: i32 = 4;
pub const SQL_FN_TD_DAYOFWEEK: i32 = 8;
pub const SQL_FN_TD_DAYOFYEAR: i32 = 16;
pub const SQL_FN_TD_EXTRACT: i32 = 1048576;
pub const SQL_FN_TD_HOUR: i32 = 1024;
pub const SQL_FN_TD_MINUTE: i32 = 2048;
pub const SQL_FN_TD_MONTH: i32 = 32;
pub const SQL_FN_TD_MONTHNAME: i32 = 65536;
pub const SQL_FN_TD_NOW: i32 = 1;
pub const SQL_FN_TD_QUARTER: i32 = 64;
pub const SQL_FN_TD_SECOND: i32 = 4096;
pub const SQL_FN_TD_TIMESTAMPADD: i32 = 8192;
pub const SQL_FN_TD_TIMESTAMPDIFF: i32 = 16384;
pub const SQL_FN_TD_WEEK: i32 = 128;
pub const SQL_FN_TD_YEAR: i32 = 256;
pub const SQL_FN_TSI_DAY: i32 = 16;
pub const SQL_FN_TSI_FRAC_SECOND: i32 = 1;
pub const SQL_FN_TSI_HOUR: i32 = 8;
pub const SQL_FN_TSI_MINUTE: i32 = 4;
pub const SQL_FN_TSI_MONTH: i32 = 64;
pub const SQL_FN_TSI_QUARTER: i32 = 128;
pub const SQL_FN_TSI_SECOND: i32 = 2;
pub const SQL_FN_TSI_WEEK: i32 = 32;
pub const SQL_FN_TSI_YEAR: i32 = 256;
pub const SQL_FORWARD_ONLY_CURSOR_ATTRIBUTES1: i32 = 146;
pub const SQL_FORWARD_ONLY_CURSOR_ATTRIBUTES2: i32 = 147;
pub const SQL_GB_COLLATE: i32 = 4;
pub const SQL_GB_GROUP_BY_CONTAINS_SELECT: i32 = 2;
pub const SQL_GB_GROUP_BY_EQUALS_SELECT: i32 = 1;
pub const SQL_GB_NOT_SUPPORTED: i32 = 0;
pub const SQL_GB_NO_RELATION: i32 = 3;
pub const SQL_GD_BLOCK: i32 = 4;
pub const SQL_GD_BOUND: i32 = 8;
pub const SQL_GD_OUTPUT_PARAMS: i32 = 16;
pub const SQL_GET_BOOKMARK: i32 = 13;
pub const SQL_GROUP_BY: i32 = 88;
pub const SQL_GUID: i32 = -11;
pub const SQL_HANDLE_SENV: i32 = 5;
pub const SQL_IGNORE: i32 = -6;
pub const SQL_IK_ALL: i32 = 3;
pub const SQL_IK_ASC: i32 = 1;
pub const SQL_IK_DESC: i32 = 2;
pub const SQL_IK_NONE: i32 = 0;
pub const SQL_INDEX_KEYWORDS: i32 = 148;
pub const SQL_INFO_FIRST: i32 = 0;
pub const SQL_INFO_SCHEMA_VIEWS: i32 = 149;
pub const SQL_INITIALLY_DEFERRED: i32 = 5;
pub const SQL_INITIALLY_IMMEDIATE: i32 = 6;
pub const SQL_INSERT_STATEMENT: i32 = 172;
pub const SQL_INTERVAL: i32 = 10;
pub const SQL_INTERVAL_DAY: i32 = 103;
pub const SQL_INTERVAL_DAY_TO_HOUR: i32 = 108;
pub const SQL_INTERVAL_DAY_TO_MINUTE: i32 = 109;
pub const SQL_INTERVAL_DAY_TO_SECOND: i32 = 110;
pub const SQL_INTERVAL_HOUR: i32 = 104;
pub const SQL_INTERVAL_HOUR_TO_MINUTE: i32 = 111;
pub const SQL_INTERVAL_HOUR_TO_SECOND: i32 = 112;
pub const SQL_INTERVAL_MINUTE: i32 = 105;
pub const SQL_INTERVAL_MINUTE_TO_SECOND: i32 = 113;
pub const SQL_INTERVAL_MONTH: i32 = 102;
pub const SQL_INTERVAL_SECOND: i32 = 106;
pub const SQL_INTERVAL_YEAR: i32 = 101;
pub const SQL_INTERVAL_YEAR_TO_MONTH: i32 = 107;
pub const SQL_ISV_ASSERTIONS: i32 = 1;
pub const SQL_ISV_CHARACTER_SETS: i32 = 2;
pub const SQL_ISV_CHECK_CONSTRAINTS: i32 = 4;
pub const SQL_ISV_COLLATIONS: i32 = 8;
pub const SQL_ISV_COLUMNS: i32 = 64;
pub const SQL_ISV_COLUMN_DOMAIN_USAGE: i32 = 16;
pub const SQL_ISV_COLUMN_PRIVILEGES: i32 = 32;
pub const SQL_ISV_CONSTRAINT_COLUMN_USAGE: i32 = 128;
pub const SQL_ISV_CONSTRAINT_TABLE_USAGE: i32 = 256;
pub const SQL_ISV_DOMAINS: i32 = 1024;
pub const SQL_ISV_DOMAIN_CONSTRAINTS: i32 = 512;
pub const SQL_ISV_KEY_COLUMN_USAGE: i32 = 2048;
pub const SQL_ISV_REFERENTIAL_CONSTRAINTS: i32 = 4096;
pub const SQL_ISV_SCHEMATA: i32 = 8192;
pub const SQL_ISV_SQL_LANGUAGES: i32 = 16384;
pub const SQL_ISV_TABLES: i32 = 131072;
pub const SQL_ISV_TABLE_CONSTRAINTS: i32 = 32768;
pub const SQL_ISV_TABLE_PRIVILEGES: i32 = 65536;
pub const SQL_ISV_TRANSLATIONS: i32 = 262144;
pub const SQL_ISV_USAGE_PRIVILEGES: i32 = 524288;
pub const SQL_ISV_VIEWS: i32 = 4194304;
pub const SQL_ISV_VIEW_COLUMN_USAGE: i32 = 1048576;
pub const SQL_ISV_VIEW_TABLE_USAGE: i32 = 2097152;
pub const SQL_IS_INSERT_LITERALS: i32 = 1;
pub const SQL_IS_INSERT_SEARCHED: i32 = 2;
pub const SQL_IS_INTEGER: i32 = -6;
pub const SQL_IS_POINTER: i32 = -4;
pub const SQL_IS_SELECT_INTO: i32 = 4;
pub const SQL_IS_SMALLINT: i32 = -8;
pub const SQL_IS_UINTEGER: i32 = -5;
pub const SQL_IS_USMALLINT: i32 = -7;
pub const SQL_KEYSET_CURSOR_ATTRIBUTES1: i32 = 150;
pub const SQL_KEYSET_CURSOR_ATTRIBUTES2: i32 = 151;
pub const SQL_KEYSET_SIZE: i32 = 8;
pub const SQL_KEYSET_SIZE_DEFAULT: u32 = 0;
pub const SQL_KEYWORDS: i32 = 89;
pub const SQL_LCK_EXCLUSIVE: i32 = 2;
pub const SQL_LCK_NO_CHANGE: i32 = 1;
pub const SQL_LCK_UNLOCK: i32 = 4;
pub const SQL_LEN_BINARY_ATTR_OFFSET: i32 = -100;
pub const SQL_LEN_DATA_AT_EXEC_OFFSET: i32 = -100;
pub const SQL_LIKE_ESCAPE_CLAUSE: i32 = 113;
pub const SQL_LIKE_ONLY: i32 = 1;
pub const SQL_LOCK_EXCLUSIVE: i32 = 1;
pub const SQL_LOCK_NO_CHANGE: i32 = 0;
pub const SQL_LOCK_TYPES: i32 = 78;
pub const SQL_LOCK_UNLOCK: i32 = 2;
pub const SQL_LOGIN_TIMEOUT: i32 = 103;
pub const SQL_LOGIN_TIMEOUT_DEFAULT: u32 = 15;
pub const SQL_LONGVARBINARY: i32 = -4;
pub const SQL_LONGVARCHAR: i32 = -1;
pub const SQL_MAX_ASYNC_CONCURRENT_STATEMENTS: i32 = 10022;
pub const SQL_MAX_BINARY_LITERAL_LEN: i32 = 112;
pub const SQL_MAX_CHAR_LITERAL_LEN: i32 = 108;
pub const SQL_MAX_DSN_LENGTH: i32 = 32;
pub const SQL_MAX_LENGTH: i32 = 3;
pub const SQL_MAX_LENGTH_DEFAULT: u32 = 0;
pub const SQL_MAX_OPTION_STRING_LENGTH: i32 = 256;
pub const SQL_MAX_OWNER_NAME_LEN: i32 = 32;
pub const SQL_MAX_PROCEDURE_NAME_LEN: i32 = 33;
pub const SQL_MAX_QUALIFIER_NAME_LEN: i32 = 34;
pub const SQL_MAX_ROWS: i32 = 1;
pub const SQL_MAX_ROWS_DEFAULT: u32 = 0;
pub const SQL_MAX_ROW_SIZE_INCLUDES_LONG: i32 = 103;
pub const SQL_MODE_DEFAULT: u32 = 0;
pub const SQL_MODE_READ_ONLY: u32 = 1;
pub const SQL_MODE_READ_WRITE: u32 = 0;
pub const SQL_MULTIPLE_ACTIVE_TXN: i32 = 37;
pub const SQL_MULT_RESULT_SETS: i32 = 36;
pub const SQL_NC_END: i32 = 4;
pub const SQL_NC_START: i32 = 2;
pub const SQL_NEED_LONG_DATA_LEN: i32 = 111;
pub const SQL_NNC_NON_NULL: i32 = 1;
pub const SQL_NNC_NULL: i32 = 0;
pub const SQL_NON_NULLABLE_COLUMNS: i32 = 75;
pub const SQL_NOSCAN: i32 = 2;
pub const SQL_NOSCAN_DEFAULT: u32 = 0;
pub const SQL_NOSCAN_OFF: u32 = 0;
pub const SQL_NOSCAN_ON: u32 = 1;
pub const SQL_NOT_DEFERRABLE: i32 = 7;
pub const SQL_NO_ACTION: i32 = 3;
pub const SQL_NO_COLUMN_NUMBER: i32 = -1;
pub const SQL_NO_DATA_FOUND: i32 = 100;
pub const SQL_NO_ROW_NUMBER: i32 = -1;
pub const SQL_NO_TOTAL: i32 = -4;
pub const SQL_NUMERIC_FUNCTIONS: i32 = 49;
pub const SQL_OAC_LEVEL1: i32 = 1;
pub const SQL_OAC_LEVEL2: i32 = 2;
pub const SQL_OAC_NONE: i32 = 0;
pub const SQL_ODBC_API_CONFORMANCE: i32 = 9;
pub const SQL_ODBC_CURSORS: i32 = 110;
pub const SQL_ODBC_INTERFACE_CONFORMANCE: i32 = 152;
pub const SQL_ODBC_SAG_CLI_CONFORMANCE: i32 = 12;
pub const SQL_ODBC_SQL_CONFORMANCE: i32 = 15;
pub const SQL_ODBC_SQL_OPT_IEF: i32 = 73;
pub const SQL_ODBC_VER: i32 = 10;
pub const SQL_OIC_CORE: u32 = 1;
pub const SQL_OIC_LEVEL1: u32 = 2;
pub const SQL_OIC_LEVEL2: u32 = 3;
pub const SQL_OPT_TRACE: i32 = 104;
pub const SQL_OPT_TRACEFILE: i32 = 105;
pub const SQL_OPT_TRACE_DEFAULT: u32 = 0;
pub const SQL_OPT_TRACE_FILE_DEFAULT: windows_core::PCSTR = windows_core::s!("\\SQL.LOG");
pub const SQL_OPT_TRACE_OFF: u32 = 0;
pub const SQL_OPT_TRACE_ON: u32 = 1;
pub const SQL_OSCC_COMPLIANT: i32 = 1;
pub const SQL_OSCC_NOT_COMPLIANT: i32 = 0;
pub const SQL_OSC_CORE: i32 = 1;
pub const SQL_OSC_EXTENDED: i32 = 2;
pub const SQL_OSC_MINIMUM: i32 = 0;
pub const SQL_OUTER_JOINS: i32 = 38;
pub const SQL_OU_DML_STATEMENTS: i32 = 1;
pub const SQL_OU_INDEX_DEFINITION: i32 = 8;
pub const SQL_OU_PRIVILEGE_DEFINITION: i32 = 16;
pub const SQL_OU_PROCEDURE_INVOCATION: i32 = 2;
pub const SQL_OU_TABLE_DEFINITION: i32 = 4;
pub const SQL_OV_ODBC2: u32 = 2;
pub const SQL_OV_ODBC3: u32 = 3;
pub const SQL_OV_ODBC3_80: u32 = 380;
pub const SQL_OWNER_TERM: i32 = 39;
pub const SQL_OWNER_USAGE: i32 = 91;
pub const SQL_PACKET_SIZE: i32 = 112;
pub const SQL_PARAM_ARRAY_ROW_COUNTS: i32 = 153;
pub const SQL_PARAM_ARRAY_SELECTS: i32 = 154;
pub const SQL_PARAM_BIND_BY_COLUMN: u32 = 0;
pub const SQL_PARAM_BIND_TYPE_DEFAULT: u32 = 0;
pub const SQL_PARAM_DIAG_UNAVAILABLE: i32 = 1;
pub const SQL_PARAM_ERROR: i32 = 5;
pub const SQL_PARAM_IGNORE: i32 = 1;
pub const SQL_PARAM_INPUT: i32 = 1;
pub const SQL_PARAM_INPUT_OUTPUT: i32 = 2;
pub const SQL_PARAM_INPUT_OUTPUT_STREAM: i32 = 8;
pub const SQL_PARAM_OUTPUT: i32 = 4;
pub const SQL_PARAM_OUTPUT_STREAM: i32 = 16;
pub const SQL_PARAM_PROCEED: i32 = 0;
pub const SQL_PARAM_SUCCESS: i32 = 0;
pub const SQL_PARAM_SUCCESS_WITH_INFO: i32 = 6;
pub const SQL_PARAM_TYPE_DEFAULT: i32 = 2;
pub const SQL_PARAM_TYPE_UNKNOWN: i32 = 0;
pub const SQL_PARAM_UNUSED: i32 = 7;
pub const SQL_PARC_BATCH: i32 = 1;
pub const SQL_PARC_NO_BATCH: i32 = 2;
pub const SQL_PAS_BATCH: i32 = 1;
pub const SQL_PAS_NO_BATCH: i32 = 2;
pub const SQL_PAS_NO_SELECT: i32 = 3;
pub const SQL_PC_NOT_PSEUDO: i32 = 1;
pub const SQL_POSITION: i32 = 0;
pub const SQL_POSITIONED_STATEMENTS: i32 = 80;
pub const SQL_POS_ADD: i32 = 16;
pub const SQL_POS_DELETE: i32 = 8;
pub const SQL_POS_OPERATIONS: i32 = 79;
pub const SQL_POS_POSITION: i32 = 1;
pub const SQL_POS_REFRESH: i32 = 2;
pub const SQL_POS_UPDATE: i32 = 4;
pub const SQL_PRED_SEARCHABLE: i32 = 3;
pub const SQL_PROCEDURES: i32 = 21;
pub const SQL_PROCEDURE_TERM: i32 = 40;
pub const SQL_PS_POSITIONED_DELETE: i32 = 1;
pub const SQL_PS_POSITIONED_UPDATE: i32 = 2;
pub const SQL_PS_SELECT_FOR_UPDATE: i32 = 4;
pub const SQL_PT_FUNCTION: i32 = 2;
pub const SQL_PT_PROCEDURE: i32 = 1;
pub const SQL_PT_UNKNOWN: i32 = 0;
pub const SQL_QL_END: i32 = 2;
pub const SQL_QL_START: i32 = 1;
pub const SQL_QUALIFIER_LOCATION: i32 = 114;
pub const SQL_QUALIFIER_NAME_SEPARATOR: i32 = 41;
pub const SQL_QUALIFIER_TERM: i32 = 42;
pub const SQL_QUALIFIER_USAGE: i32 = 92;
pub const SQL_QUERY_TIMEOUT: i32 = 0;
pub const SQL_QUERY_TIMEOUT_DEFAULT: u32 = 0;
pub const SQL_QUICK: i32 = 0;
pub const SQL_QUIET_MODE: i32 = 111;
pub const SQL_QUOTED_IDENTIFIER_CASE: i32 = 93;
pub const SQL_QU_DML_STATEMENTS: i32 = 1;
pub const SQL_QU_INDEX_DEFINITION: i32 = 8;
pub const SQL_QU_PRIVILEGE_DEFINITION: i32 = 16;
pub const SQL_QU_PROCEDURE_INVOCATION: i32 = 2;
pub const SQL_QU_TABLE_DEFINITION: i32 = 4;
pub const SQL_RD_DEFAULT: u32 = 1;
pub const SQL_RD_OFF: u32 = 0;
pub const SQL_RD_ON: u32 = 1;
pub const SQL_REFRESH: i32 = 1;
pub const SQL_RESET_CONNECTION_YES: u32 = 1;
pub const SQL_RESTRICT: i32 = 1;
pub const SQL_RESULT_COL: i32 = 3;
pub const SQL_RETRIEVE_DATA: i32 = 11;
pub const SQL_RETURN_VALUE: i32 = 5;
pub const SQL_ROWSET_SIZE: i32 = 9;
pub const SQL_ROWSET_SIZE_DEFAULT: u32 = 1;
pub const SQL_ROWVER: i32 = 2;
pub const SQL_ROW_ADDED: i32 = 4;
pub const SQL_ROW_DELETED: i32 = 1;
pub const SQL_ROW_ERROR: i32 = 5;
pub const SQL_ROW_IGNORE: i32 = 1;
pub const SQL_ROW_NOROW: i32 = 3;
pub const SQL_ROW_NUMBER: i32 = 14;
pub const SQL_ROW_NUMBER_UNKNOWN: i32 = -2;
pub const SQL_ROW_PROCEED: i32 = 0;
pub const SQL_ROW_SUCCESS: i32 = 0;
pub const SQL_ROW_SUCCESS_WITH_INFO: i32 = 6;
pub const SQL_ROW_UPDATED: i32 = 2;
pub const SQL_ROW_UPDATES: i32 = 11;
pub const SQL_SCCO_OPT_TIMESTAMP: i32 = 4;
pub const SQL_SCC_ISO92_CLI: i32 = 2;
pub const SQL_SCC_XOPEN_CLI_VERSION1: i32 = 1;
pub const SQL_SCHEMA_TERM: i32 = 39;
pub const SQL_SCHEMA_USAGE: i32 = 91;
pub const SQL_SCROLL_DYNAMIC: i32 = -2;
pub const SQL_SCROLL_FORWARD_ONLY: i32 = 0;
pub const SQL_SCROLL_KEYSET_DRIVEN: i32 = -1;
pub const SQL_SCROLL_OPTIONS: i32 = 44;
pub const SQL_SCROLL_STATIC: i32 = -3;
pub const SQL_SC_FIPS127_2_TRANSITIONAL: i32 = 2;
pub const SQL_SC_NON_UNIQUE: u32 = 0;
pub const SQL_SC_SQL92_ENTRY: i32 = 1;
pub const SQL_SC_SQL92_FULL: i32 = 8;
pub const SQL_SC_SQL92_INTERMEDIATE: i32 = 4;
pub const SQL_SC_TRY_UNIQUE: u32 = 1;
pub const SQL_SC_UNIQUE: u32 = 2;
pub const SQL_SDF_CURRENT_DATE: i32 = 1;
pub const SQL_SDF_CURRENT_TIME: i32 = 2;
pub const SQL_SDF_CURRENT_TIMESTAMP: i32 = 4;
pub const SQL_SEARCHABLE: i32 = 3;
pub const SQL_SETPARAM_VALUE_MAX: i32 = -1;
pub const SQL_SETPOS_MAX_LOCK_VALUE: i32 = 2;
pub const SQL_SETPOS_MAX_OPTION_VALUE: i32 = 4;
pub const SQL_SET_DEFAULT: i32 = 4;
pub const SQL_SET_NULL: i32 = 2;
pub const SQL_SFKD_CASCADE: i32 = 1;
pub const SQL_SFKD_NO_ACTION: i32 = 2;
pub const SQL_SFKD_SET_DEFAULT: i32 = 4;
pub const SQL_SFKD_SET_NULL: i32 = 8;
pub const SQL_SFKU_CASCADE: i32 = 1;
pub const SQL_SFKU_NO_ACTION: i32 = 2;
pub const SQL_SFKU_SET_DEFAULT: i32 = 4;
pub const SQL_SFKU_SET_NULL: i32 = 8;
pub const SQL_SG_DELETE_TABLE: i32 = 32;
pub const SQL_SG_INSERT_COLUMN: i32 = 128;
pub const SQL_SG_INSERT_TABLE: i32 = 64;
pub const SQL_SG_REFERENCES_COLUMN: i32 = 512;
pub const SQL_SG_REFERENCES_TABLE: i32 = 256;
pub const SQL_SG_SELECT_TABLE: i32 = 1024;
pub const SQL_SG_UPDATE_COLUMN: i32 = 4096;
pub const SQL_SG_UPDATE_TABLE: i32 = 2048;
pub const SQL_SG_USAGE_ON_CHARACTER_SET: i32 = 2;
pub const SQL_SG_USAGE_ON_COLLATION: i32 = 4;
pub const SQL_SG_USAGE_ON_DOMAIN: i32 = 1;
pub const SQL_SG_USAGE_ON_TRANSLATION: i32 = 8;
pub const SQL_SG_WITH_GRANT_OPTION: i32 = 16;
pub const SQL_SIGNED_OFFSET: i32 = -20;
pub const SQL_SIMULATE_CURSOR: i32 = 10;
pub const SQL_SNVF_BIT_LENGTH: i32 = 1;
pub const SQL_SNVF_CHARACTER_LENGTH: i32 = 4;
pub const SQL_SNVF_CHAR_LENGTH: i32 = 2;
pub const SQL_SNVF_EXTRACT: i32 = 8;
pub const SQL_SNVF_OCTET_LENGTH: i32 = 16;
pub const SQL_SNVF_POSITION: i32 = 32;
pub const SQL_SO_DYNAMIC: i32 = 4;
pub const SQL_SO_FORWARD_ONLY: i32 = 1;
pub const SQL_SO_KEYSET_DRIVEN: i32 = 2;
pub const SQL_SO_MIXED: i32 = 8;
pub const SQL_SO_STATIC: i32 = 16;
pub const SQL_SPEC_MAJOR: i32 = 3;
pub const SQL_SPEC_MINOR: i32 = 80;
pub const SQL_SPEC_STRING: windows_core::PCSTR = windows_core::s!("03.80");
pub const SQL_SP_BETWEEN: i32 = 2048;
pub const SQL_SP_COMPARISON: i32 = 4096;
pub const SQL_SP_EXISTS: i32 = 1;
pub const SQL_SP_IN: i32 = 1024;
pub const SQL_SP_ISNOTNULL: i32 = 2;
pub const SQL_SP_ISNULL: i32 = 4;
pub const SQL_SP_LIKE: i32 = 512;
pub const SQL_SP_MATCH_FULL: i32 = 8;
pub const SQL_SP_MATCH_PARTIAL: i32 = 16;
pub const SQL_SP_MATCH_UNIQUE_FULL: i32 = 32;
pub const SQL_SP_MATCH_UNIQUE_PARTIAL: i32 = 64;
pub const SQL_SP_OVERLAPS: i32 = 128;
pub const SQL_SP_QUANTIFIED_COMPARISON: i32 = 8192;
pub const SQL_SP_UNIQUE: i32 = 256;
pub const SQL_SQL92_DATETIME_FUNCTIONS: i32 = 155;
pub const SQL_SQL92_FOREIGN_KEY_DELETE_RULE: i32 = 156;
pub const SQL_SQL92_FOREIGN_KEY_UPDATE_RULE: i32 = 157;
pub const SQL_SQL92_GRANT: i32 = 158;
pub const SQL_SQL92_NUMERIC_VALUE_FUNCTIONS: i32 = 159;
pub const SQL_SQL92_PREDICATES: i32 = 160;
pub const SQL_SQL92_RELATIONAL_JOIN_OPERATORS: i32 = 161;
pub const SQL_SQL92_REVOKE: i32 = 162;
pub const SQL_SQL92_ROW_VALUE_CONSTRUCTOR: i32 = 163;
pub const SQL_SQL92_STRING_FUNCTIONS: i32 = 164;
pub const SQL_SQL92_VALUE_EXPRESSIONS: i32 = 165;
pub const SQL_SQLSTATE_SIZE: i32 = 5;
pub const SQL_SQL_CONFORMANCE: i32 = 118;
pub const SQL_SQ_COMPARISON: i32 = 1;
pub const SQL_SQ_CORRELATED_SUBQUERIES: i32 = 16;
pub const SQL_SQ_EXISTS: i32 = 2;
pub const SQL_SQ_IN: i32 = 4;
pub const SQL_SQ_QUANTIFIED: i32 = 8;
pub const SQL_SRJO_CORRESPONDING_CLAUSE: i32 = 1;
pub const SQL_SRJO_CROSS_JOIN: i32 = 2;
pub const SQL_SRJO_EXCEPT_JOIN: i32 = 4;
pub const SQL_SRJO_FULL_OUTER_JOIN: i32 = 8;
pub const SQL_SRJO_INNER_JOIN: i32 = 16;
pub const SQL_SRJO_INTERSECT_JOIN: i32 = 32;
pub const SQL_SRJO_LEFT_OUTER_JOIN: i32 = 64;
pub const SQL_SRJO_NATURAL_JOIN: i32 = 128;
pub const SQL_SRJO_RIGHT_OUTER_JOIN: i32 = 256;
pub const SQL_SRJO_UNION_JOIN: i32 = 512;
pub const SQL_SRVC_DEFAULT: i32 = 4;
pub const SQL_SRVC_NULL: i32 = 2;
pub const SQL_SRVC_ROW_SUBQUERY: i32 = 8;
pub const SQL_SRVC_VALUE_EXPRESSION: i32 = 1;
pub const SQL_SR_CASCADE: i32 = 32;
pub const SQL_SR_DELETE_TABLE: i32 = 128;
pub const SQL_SR_GRANT_OPTION_FOR: i32 = 16;
pub const SQL_SR_INSERT_COLUMN: i32 = 512;
pub const SQL_SR_INSERT_TABLE: i32 = 256;
pub const SQL_SR_REFERENCES_COLUMN: i32 = 2048;
pub const SQL_SR_REFERENCES_TABLE: i32 = 1024;
pub const SQL_SR_RESTRICT: i32 = 64;
pub const SQL_SR_SELECT_TABLE: i32 = 4096;
pub const SQL_SR_UPDATE_COLUMN: i32 = 16384;
pub const SQL_SR_UPDATE_TABLE: i32 = 8192;
pub const SQL_SR_USAGE_ON_CHARACTER_SET: i32 = 2;
pub const SQL_SR_USAGE_ON_COLLATION: i32 = 4;
pub const SQL_SR_USAGE_ON_DOMAIN: i32 = 1;
pub const SQL_SR_USAGE_ON_TRANSLATION: i32 = 8;
pub const SQL_SSF_CONVERT: i32 = 1;
pub const SQL_SSF_LOWER: i32 = 2;
pub const SQL_SSF_SUBSTRING: i32 = 8;
pub const SQL_SSF_TRANSLATE: i32 = 16;
pub const SQL_SSF_TRIM_BOTH: i32 = 32;
pub const SQL_SSF_TRIM_LEADING: i32 = 64;
pub const SQL_SSF_TRIM_TRAILING: i32 = 128;
pub const SQL_SSF_UPPER: i32 = 4;
pub const SQL_SS_ADDITIONS: i32 = 1;
pub const SQL_SS_DELETIONS: i32 = 2;
pub const SQL_SS_UPDATES: i32 = 4;
pub const SQL_STANDARD_CLI_CONFORMANCE: i32 = 166;
pub const SQL_STATIC_CURSOR_ATTRIBUTES1: i32 = 167;
pub const SQL_STATIC_CURSOR_ATTRIBUTES2: i32 = 168;
pub const SQL_STATIC_SENSITIVITY: i32 = 83;
pub const SQL_STRING_FUNCTIONS: i32 = 50;
pub const SQL_SUBQUERIES: i32 = 95;
pub const SQL_SU_DML_STATEMENTS: i32 = 1;
pub const SQL_SU_INDEX_DEFINITION: i32 = 8;
pub const SQL_SU_PRIVILEGE_DEFINITION: i32 = 16;
pub const SQL_SU_PROCEDURE_INVOCATION: i32 = 2;
pub const SQL_SU_TABLE_DEFINITION: i32 = 4;
pub const SQL_SVE_CASE: i32 = 1;
pub const SQL_SVE_CAST: i32 = 2;
pub const SQL_SVE_COALESCE: i32 = 4;
pub const SQL_SVE_NULLIF: i32 = 8;
pub const SQL_SYSTEM_FUNCTIONS: i32 = 51;
pub const SQL_TABLE_STAT: i32 = 0;
pub const SQL_TABLE_TERM: i32 = 45;
pub const SQL_TIME: i32 = 10;
pub const SQL_TIMEDATE_ADD_INTERVALS: i32 = 109;
pub const SQL_TIMEDATE_DIFF_INTERVALS: i32 = 110;
pub const SQL_TIMEDATE_FUNCTIONS: i32 = 52;
pub const SQL_TIMESTAMP: i32 = 11;
pub const SQL_TINYINT: i32 = -6;
pub const SQL_TRANSLATE_DLL: i32 = 106;
pub const SQL_TRANSLATE_OPTION: i32 = 107;
pub const SQL_TXN_ISOLATION: i32 = 108;
pub const SQL_TYPE_NULL: i32 = 0;
pub const SQL_UB_DEFAULT: u32 = 0;
pub const SQL_UB_FIXED: u32 = 1;
pub const SQL_UB_OFF: u32 = 0;
pub const SQL_UB_ON: u32 = 1;
pub const SQL_UB_VARIABLE: u32 = 2;
pub const SQL_UNICODE: i32 = -8;
pub const SQL_UNICODE_CHAR: i32 = -8;
pub const SQL_UNICODE_LONGVARCHAR: i32 = -10;
pub const SQL_UNICODE_VARCHAR: i32 = -9;
pub const SQL_UNION: i32 = 96;
pub const SQL_UNION_STATEMENT: i32 = 96;
pub const SQL_UNSEARCHABLE: i32 = 0;
pub const SQL_UNSIGNED_OFFSET: i32 = -22;
pub const SQL_UPDATE: i32 = 2;
pub const SQL_UPDATE_BY_BOOKMARK: i32 = 5;
pub const SQL_USE_BOOKMARKS: i32 = 12;
pub const SQL_US_UNION: i32 = 1;
pub const SQL_US_UNION_ALL: i32 = 2;
pub const SQL_U_UNION: i32 = 1;
pub const SQL_U_UNION_ALL: i32 = 2;
pub const SQL_VARBINARY: i32 = -3;
pub const TRACE_ON: i32 = 1;
pub const TRACE_VERSION: i32 = 1000;
pub const TRACE_VS_EVENT_ON: i32 = 2;
