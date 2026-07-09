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
#[cfg(feature = "Win32_sqltypes")]
#[inline]
pub unsafe fn SQLAllocHandleStd(fhandletype: super::sqltypes::SQLSMALLINT, hinput: super::sqltypes::SQLHANDLE, phoutput: *mut super::sqltypes::SQLHANDLE) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLAllocHandleStd(fhandletype : super::sqltypes::SQLSMALLINT, hinput : super::sqltypes::SQLHANDLE, phoutput : *mut super::sqltypes::SQLHANDLE) -> super::sqltypes::SQLRETURN);
    unsafe { SQLAllocHandleStd(fhandletype, hinput, phoutput as _) }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_sqltypes")]
#[inline]
pub unsafe fn SQLBindParameter(hstmt: super::sqltypes::SQLHSTMT, ipar: super::sqltypes::SQLUSMALLINT, fparamtype: super::sqltypes::SQLSMALLINT, fctype: super::sqltypes::SQLSMALLINT, fsqltype: super::sqltypes::SQLSMALLINT, cbcoldef: super::sqltypes::SQLUINTEGER, ibscale: super::sqltypes::SQLSMALLINT, rgbvalue: super::sqltypes::SQLPOINTER, cbvaluemax: super::sqltypes::SQLINTEGER, pcbvalue: *mut super::sqltypes::SQLINTEGER) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLBindParameter(hstmt : super::sqltypes::SQLHSTMT, ipar : super::sqltypes::SQLUSMALLINT, fparamtype : super::sqltypes::SQLSMALLINT, fctype : super::sqltypes::SQLSMALLINT, fsqltype : super::sqltypes::SQLSMALLINT, cbcoldef : super::sqltypes::SQLUINTEGER, ibscale : super::sqltypes::SQLSMALLINT, rgbvalue : super::sqltypes::SQLPOINTER, cbvaluemax : super::sqltypes::SQLINTEGER, pcbvalue : *mut super::sqltypes::SQLINTEGER) -> super::sqltypes::SQLRETURN);
    unsafe { SQLBindParameter(hstmt, ipar, fparamtype, fctype, fsqltype, cbcoldef, ibscale, rgbvalue, cbvaluemax, pcbvalue as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_sqltypes")]
#[inline]
pub unsafe fn SQLBindParameter(hstmt: super::sqltypes::SQLHSTMT, ipar: super::sqltypes::SQLUSMALLINT, fparamtype: super::sqltypes::SQLSMALLINT, fctype: super::sqltypes::SQLSMALLINT, fsqltype: super::sqltypes::SQLSMALLINT, cbcoldef: super::sqltypes::SQLULEN, ibscale: super::sqltypes::SQLSMALLINT, rgbvalue: super::sqltypes::SQLPOINTER, cbvaluemax: super::sqltypes::SQLLEN, pcbvalue: *mut super::sqltypes::SQLLEN) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLBindParameter(hstmt : super::sqltypes::SQLHSTMT, ipar : super::sqltypes::SQLUSMALLINT, fparamtype : super::sqltypes::SQLSMALLINT, fctype : super::sqltypes::SQLSMALLINT, fsqltype : super::sqltypes::SQLSMALLINT, cbcoldef : super::sqltypes::SQLULEN, ibscale : super::sqltypes::SQLSMALLINT, rgbvalue : super::sqltypes::SQLPOINTER, cbvaluemax : super::sqltypes::SQLLEN, pcbvalue : *mut super::sqltypes::SQLLEN) -> super::sqltypes::SQLRETURN);
    unsafe { SQLBindParameter(hstmt, ipar, fparamtype, fctype, fsqltype, cbcoldef, ibscale, rgbvalue, cbvaluemax, pcbvalue as _) }
}
#[cfg(feature = "Win32_sqltypes")]
#[inline]
pub unsafe fn SQLBrowseConnect(hdbc: super::sqltypes::SQLHDBC, szconnstrin: &[super::sqltypes::SQLCHAR], szconnstrout: Option<&mut [super::sqltypes::SQLCHAR]>, pcchconnstrout: Option<*mut super::sqltypes::SQLSMALLINT>) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLBrowseConnect(hdbc : super::sqltypes::SQLHDBC, szconnstrin : *const super::sqltypes::SQLCHAR, cchconnstrin : super::sqltypes::SQLSMALLINT, szconnstrout : *mut super::sqltypes::SQLCHAR, cchconnstroutmax : super::sqltypes::SQLSMALLINT, pcchconnstrout : *mut super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
    unsafe { SQLBrowseConnect(hdbc, core::mem::transmute(szconnstrin.as_ptr()), super::sqltypes::SQLSMALLINT(szconnstrin.len().try_into().unwrap()), core::mem::transmute(szconnstrout.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), szconnstrout.as_deref().map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())), pcchconnstrout.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_sqltypes")]
#[inline]
pub unsafe fn SQLBulkOperations(statementhandle: super::sqltypes::SQLHSTMT, operation: super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLBulkOperations(statementhandle : super::sqltypes::SQLHSTMT, operation : super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
    unsafe { SQLBulkOperations(statementhandle, operation) }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_sqltypes")]
#[inline]
pub unsafe fn SQLColAttributes(hstmt: super::sqltypes::SQLHSTMT, icol: super::sqltypes::SQLUSMALLINT, fdesctype: super::sqltypes::SQLUSMALLINT, rgbdesc: super::sqltypes::SQLPOINTER, cbdescmax: super::sqltypes::SQLSMALLINT, pcbdesc: *mut super::sqltypes::SQLSMALLINT, pfdesc: *mut super::sqltypes::SQLINTEGER) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLColAttributes(hstmt : super::sqltypes::SQLHSTMT, icol : super::sqltypes::SQLUSMALLINT, fdesctype : super::sqltypes::SQLUSMALLINT, rgbdesc : super::sqltypes::SQLPOINTER, cbdescmax : super::sqltypes::SQLSMALLINT, pcbdesc : *mut super::sqltypes::SQLSMALLINT, pfdesc : *mut super::sqltypes::SQLINTEGER) -> super::sqltypes::SQLRETURN);
    unsafe { SQLColAttributes(hstmt, icol, fdesctype, rgbdesc, cbdescmax, pcbdesc as _, pfdesc as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_sqltypes")]
#[inline]
pub unsafe fn SQLColAttributes(hstmt: super::sqltypes::SQLHSTMT, icol: super::sqltypes::SQLUSMALLINT, fdesctype: super::sqltypes::SQLUSMALLINT, rgbdesc: super::sqltypes::SQLPOINTER, cbdescmax: super::sqltypes::SQLSMALLINT, pcbdesc: *mut super::sqltypes::SQLSMALLINT, pfdesc: *mut super::sqltypes::SQLLEN) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLColAttributes(hstmt : super::sqltypes::SQLHSTMT, icol : super::sqltypes::SQLUSMALLINT, fdesctype : super::sqltypes::SQLUSMALLINT, rgbdesc : super::sqltypes::SQLPOINTER, cbdescmax : super::sqltypes::SQLSMALLINT, pcbdesc : *mut super::sqltypes::SQLSMALLINT, pfdesc : *mut super::sqltypes::SQLLEN) -> super::sqltypes::SQLRETURN);
    unsafe { SQLColAttributes(hstmt, icol, fdesctype, rgbdesc, cbdescmax, pcbdesc as _, pfdesc as _) }
}
#[cfg(feature = "Win32_sqltypes")]
#[inline]
pub unsafe fn SQLColumnPrivileges(hstmt: super::sqltypes::SQLHSTMT, szcatalogname: Option<&[super::sqltypes::SQLCHAR]>, szschemaname: Option<&[super::sqltypes::SQLCHAR]>, sztablename: Option<&[super::sqltypes::SQLCHAR]>, szcolumnname: Option<&[super::sqltypes::SQLCHAR]>) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLColumnPrivileges(hstmt : super::sqltypes::SQLHSTMT, szcatalogname : *const super::sqltypes::SQLCHAR, cchcatalogname : super::sqltypes::SQLSMALLINT, szschemaname : *const super::sqltypes::SQLCHAR, cchschemaname : super::sqltypes::SQLSMALLINT, sztablename : *const super::sqltypes::SQLCHAR, cchtablename : super::sqltypes::SQLSMALLINT, szcolumnname : *const super::sqltypes::SQLCHAR, cchcolumnname : super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
    unsafe {
        SQLColumnPrivileges(
            hstmt,
            core::mem::transmute(szcatalogname.map_or(core::ptr::null(), |slice| slice.as_ptr())),
            szcatalogname.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
            core::mem::transmute(szschemaname.map_or(core::ptr::null(), |slice| slice.as_ptr())),
            szschemaname.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
            core::mem::transmute(sztablename.map_or(core::ptr::null(), |slice| slice.as_ptr())),
            sztablename.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
            core::mem::transmute(szcolumnname.map_or(core::ptr::null(), |slice| slice.as_ptr())),
            szcolumnname.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
        )
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_sqltypes")]
#[inline]
pub unsafe fn SQLDescribeParam(hstmt: super::sqltypes::SQLHSTMT, ipar: super::sqltypes::SQLUSMALLINT, pfsqltype: Option<*mut super::sqltypes::SQLSMALLINT>, pcbparamdef: Option<*mut super::sqltypes::SQLUINTEGER>, pibscale: Option<*mut super::sqltypes::SQLSMALLINT>, pfnullable: Option<*mut super::sqltypes::SQLSMALLINT>) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLDescribeParam(hstmt : super::sqltypes::SQLHSTMT, ipar : super::sqltypes::SQLUSMALLINT, pfsqltype : *mut super::sqltypes::SQLSMALLINT, pcbparamdef : *mut super::sqltypes::SQLUINTEGER, pibscale : *mut super::sqltypes::SQLSMALLINT, pfnullable : *mut super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
    unsafe { SQLDescribeParam(hstmt, ipar, pfsqltype.unwrap_or(core::mem::zeroed()) as _, pcbparamdef.unwrap_or(core::mem::zeroed()) as _, pibscale.unwrap_or(core::mem::zeroed()) as _, pfnullable.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_sqltypes")]
#[inline]
pub unsafe fn SQLDescribeParam(hstmt: super::sqltypes::SQLHSTMT, ipar: super::sqltypes::SQLUSMALLINT, pfsqltype: Option<*mut super::sqltypes::SQLSMALLINT>, pcbparamdef: Option<*mut super::sqltypes::SQLULEN>, pibscale: Option<*mut super::sqltypes::SQLSMALLINT>, pfnullable: Option<*mut super::sqltypes::SQLSMALLINT>) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLDescribeParam(hstmt : super::sqltypes::SQLHSTMT, ipar : super::sqltypes::SQLUSMALLINT, pfsqltype : *mut super::sqltypes::SQLSMALLINT, pcbparamdef : *mut super::sqltypes::SQLULEN, pibscale : *mut super::sqltypes::SQLSMALLINT, pfnullable : *mut super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
    unsafe { SQLDescribeParam(hstmt, ipar, pfsqltype.unwrap_or(core::mem::zeroed()) as _, pcbparamdef.unwrap_or(core::mem::zeroed()) as _, pibscale.unwrap_or(core::mem::zeroed()) as _, pfnullable.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Win32_sqltypes", feature = "Win32_windef"))]
#[inline]
pub unsafe fn SQLDriverConnect(hdbc: super::sqltypes::SQLHDBC, hwnd: super::sqltypes::SQLHWND, szconnstrin: &[super::sqltypes::SQLCHAR], szconnstrout: Option<&mut [super::sqltypes::SQLCHAR]>, pcchconnstrout: Option<*mut super::sqltypes::SQLSMALLINT>, fdrivercompletion: super::sqltypes::SQLUSMALLINT) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLDriverConnect(hdbc : super::sqltypes::SQLHDBC, hwnd : super::sqltypes::SQLHWND, szconnstrin : *const super::sqltypes::SQLCHAR, cchconnstrin : super::sqltypes::SQLSMALLINT, szconnstrout : *mut super::sqltypes::SQLCHAR, cchconnstroutmax : super::sqltypes::SQLSMALLINT, pcchconnstrout : *mut super::sqltypes::SQLSMALLINT, fdrivercompletion : super::sqltypes::SQLUSMALLINT) -> super::sqltypes::SQLRETURN);
    unsafe { SQLDriverConnect(hdbc, hwnd, core::mem::transmute(szconnstrin.as_ptr()), super::sqltypes::SQLSMALLINT(szconnstrin.len().try_into().unwrap()), core::mem::transmute(szconnstrout.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), szconnstrout.as_deref().map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())), pcchconnstrout.unwrap_or(core::mem::zeroed()) as _, fdrivercompletion) }
}
#[cfg(feature = "Win32_sqltypes")]
#[inline]
pub unsafe fn SQLDrivers(henv: super::sqltypes::SQLHENV, fdirection: super::sqltypes::SQLUSMALLINT, szdriverdesc: Option<&mut [super::sqltypes::SQLCHAR]>, pcchdriverdesc: Option<*mut super::sqltypes::SQLSMALLINT>, szdriverattributes: Option<&mut [super::sqltypes::SQLCHAR]>, pcchdrvrattr: Option<*mut super::sqltypes::SQLSMALLINT>) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLDrivers(henv : super::sqltypes::SQLHENV, fdirection : super::sqltypes::SQLUSMALLINT, szdriverdesc : *mut super::sqltypes::SQLCHAR, cchdriverdescmax : super::sqltypes::SQLSMALLINT, pcchdriverdesc : *mut super::sqltypes::SQLSMALLINT, szdriverattributes : *mut super::sqltypes::SQLCHAR, cchdrvrattrmax : super::sqltypes::SQLSMALLINT, pcchdrvrattr : *mut super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
    unsafe {
        SQLDrivers(
            henv,
            fdirection,
            core::mem::transmute(szdriverdesc.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
            szdriverdesc.as_deref().map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
            pcchdriverdesc.unwrap_or(core::mem::zeroed()) as _,
            core::mem::transmute(szdriverattributes.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
            szdriverattributes.as_deref().map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
            pcchdrvrattr.unwrap_or(core::mem::zeroed()) as _,
        )
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_sqltypes")]
#[inline]
pub unsafe fn SQLExtendedFetch(hstmt: super::sqltypes::SQLHSTMT, ffetchtype: super::sqltypes::SQLUSMALLINT, irow: super::sqltypes::SQLINTEGER, pcrow: Option<*mut super::sqltypes::SQLUINTEGER>, rgfrowstatus: Option<*mut super::sqltypes::SQLUSMALLINT>) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLExtendedFetch(hstmt : super::sqltypes::SQLHSTMT, ffetchtype : super::sqltypes::SQLUSMALLINT, irow : super::sqltypes::SQLINTEGER, pcrow : *mut super::sqltypes::SQLUINTEGER, rgfrowstatus : *mut super::sqltypes::SQLUSMALLINT) -> super::sqltypes::SQLRETURN);
    unsafe { SQLExtendedFetch(hstmt, ffetchtype, irow, pcrow.unwrap_or(core::mem::zeroed()) as _, rgfrowstatus.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_sqltypes")]
#[inline]
pub unsafe fn SQLExtendedFetch(hstmt: super::sqltypes::SQLHSTMT, ffetchtype: super::sqltypes::SQLUSMALLINT, irow: super::sqltypes::SQLLEN, pcrow: Option<*mut super::sqltypes::SQLULEN>, rgfrowstatus: Option<*mut super::sqltypes::SQLUSMALLINT>) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLExtendedFetch(hstmt : super::sqltypes::SQLHSTMT, ffetchtype : super::sqltypes::SQLUSMALLINT, irow : super::sqltypes::SQLLEN, pcrow : *mut super::sqltypes::SQLULEN, rgfrowstatus : *mut super::sqltypes::SQLUSMALLINT) -> super::sqltypes::SQLRETURN);
    unsafe { SQLExtendedFetch(hstmt, ffetchtype, irow, pcrow.unwrap_or(core::mem::zeroed()) as _, rgfrowstatus.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_sqltypes")]
#[inline]
pub unsafe fn SQLForeignKeys(hstmt: super::sqltypes::SQLHSTMT, szpkcatalogname: Option<&[super::sqltypes::SQLCHAR]>, szpkschemaname: Option<&[super::sqltypes::SQLCHAR]>, szpktablename: Option<&[super::sqltypes::SQLCHAR]>, szfkcatalogname: Option<&[super::sqltypes::SQLCHAR]>, szfkschemaname: Option<&[super::sqltypes::SQLCHAR]>, szfktablename: Option<&[super::sqltypes::SQLCHAR]>) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLForeignKeys(hstmt : super::sqltypes::SQLHSTMT, szpkcatalogname : *const super::sqltypes::SQLCHAR, cchpkcatalogname : super::sqltypes::SQLSMALLINT, szpkschemaname : *const super::sqltypes::SQLCHAR, cchpkschemaname : super::sqltypes::SQLSMALLINT, szpktablename : *const super::sqltypes::SQLCHAR, cchpktablename : super::sqltypes::SQLSMALLINT, szfkcatalogname : *const super::sqltypes::SQLCHAR, cchfkcatalogname : super::sqltypes::SQLSMALLINT, szfkschemaname : *const super::sqltypes::SQLCHAR, cchfkschemaname : super::sqltypes::SQLSMALLINT, szfktablename : *const super::sqltypes::SQLCHAR, cchfktablename : super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
    unsafe {
        SQLForeignKeys(
            hstmt,
            core::mem::transmute(szpkcatalogname.map_or(core::ptr::null(), |slice| slice.as_ptr())),
            szpkcatalogname.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
            core::mem::transmute(szpkschemaname.map_or(core::ptr::null(), |slice| slice.as_ptr())),
            szpkschemaname.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
            core::mem::transmute(szpktablename.map_or(core::ptr::null(), |slice| slice.as_ptr())),
            szpktablename.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
            core::mem::transmute(szfkcatalogname.map_or(core::ptr::null(), |slice| slice.as_ptr())),
            szfkcatalogname.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
            core::mem::transmute(szfkschemaname.map_or(core::ptr::null(), |slice| slice.as_ptr())),
            szfkschemaname.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
            core::mem::transmute(szfktablename.map_or(core::ptr::null(), |slice| slice.as_ptr())),
            szfktablename.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
        )
    }
}
#[cfg(feature = "Win32_sqltypes")]
#[inline]
pub unsafe fn SQLMoreResults(hstmt: super::sqltypes::SQLHSTMT) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLMoreResults(hstmt : super::sqltypes::SQLHSTMT) -> super::sqltypes::SQLRETURN);
    unsafe { SQLMoreResults(hstmt) }
}
#[cfg(feature = "Win32_sqltypes")]
#[inline]
pub unsafe fn SQLNativeSql(hdbc: super::sqltypes::SQLHDBC, szsqlstrin: &[super::sqltypes::SQLCHAR], szsqlstr: Option<&mut [super::sqltypes::SQLCHAR]>, pcbsqlstr: *mut super::sqltypes::SQLINTEGER) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLNativeSql(hdbc : super::sqltypes::SQLHDBC, szsqlstrin : *const super::sqltypes::SQLCHAR, cchsqlstrin : super::sqltypes::SQLINTEGER, szsqlstr : *mut super::sqltypes::SQLCHAR, cchsqlstrmax : super::sqltypes::SQLINTEGER, pcbsqlstr : *mut super::sqltypes::SQLINTEGER) -> super::sqltypes::SQLRETURN);
    unsafe { SQLNativeSql(hdbc, core::mem::transmute(szsqlstrin.as_ptr()), super::sqltypes::SQLINTEGER(szsqlstrin.len().try_into().unwrap()), core::mem::transmute(szsqlstr.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), szsqlstr.as_deref().map_or(super::sqltypes::SQLINTEGER(0), |slice| super::sqltypes::SQLINTEGER(slice.len().try_into().unwrap())), pcbsqlstr as _) }
}
#[cfg(feature = "Win32_sqltypes")]
#[inline]
pub unsafe fn SQLNumParams(hstmt: super::sqltypes::SQLHSTMT, pcpar: Option<*mut super::sqltypes::SQLSMALLINT>) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLNumParams(hstmt : super::sqltypes::SQLHSTMT, pcpar : *mut super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
    unsafe { SQLNumParams(hstmt, pcpar.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_sqltypes")]
#[inline]
pub unsafe fn SQLParamOptions(hstmt: super::sqltypes::SQLHSTMT, crow: super::sqltypes::SQLUINTEGER, pirow: *mut super::sqltypes::SQLUINTEGER) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLParamOptions(hstmt : super::sqltypes::SQLHSTMT, crow : super::sqltypes::SQLUINTEGER, pirow : *mut super::sqltypes::SQLUINTEGER) -> super::sqltypes::SQLRETURN);
    unsafe { SQLParamOptions(hstmt, crow, pirow as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_sqltypes")]
#[inline]
pub unsafe fn SQLParamOptions(hstmt: super::sqltypes::SQLHSTMT, crow: super::sqltypes::SQLULEN, pirow: *mut super::sqltypes::SQLULEN) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLParamOptions(hstmt : super::sqltypes::SQLHSTMT, crow : super::sqltypes::SQLULEN, pirow : *mut super::sqltypes::SQLULEN) -> super::sqltypes::SQLRETURN);
    unsafe { SQLParamOptions(hstmt, crow, pirow as _) }
}
#[cfg(feature = "Win32_sqltypes")]
#[inline]
pub unsafe fn SQLPrimaryKeys(hstmt: super::sqltypes::SQLHSTMT, szcatalogname: Option<&[super::sqltypes::SQLCHAR]>, szschemaname: Option<&[super::sqltypes::SQLCHAR]>, sztablename: Option<&[super::sqltypes::SQLCHAR]>) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLPrimaryKeys(hstmt : super::sqltypes::SQLHSTMT, szcatalogname : *const super::sqltypes::SQLCHAR, cchcatalogname : super::sqltypes::SQLSMALLINT, szschemaname : *const super::sqltypes::SQLCHAR, cchschemaname : super::sqltypes::SQLSMALLINT, sztablename : *const super::sqltypes::SQLCHAR, cchtablename : super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
    unsafe {
        SQLPrimaryKeys(
            hstmt,
            core::mem::transmute(szcatalogname.map_or(core::ptr::null(), |slice| slice.as_ptr())),
            szcatalogname.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
            core::mem::transmute(szschemaname.map_or(core::ptr::null(), |slice| slice.as_ptr())),
            szschemaname.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
            core::mem::transmute(sztablename.map_or(core::ptr::null(), |slice| slice.as_ptr())),
            sztablename.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
        )
    }
}
#[cfg(feature = "Win32_sqltypes")]
#[inline]
pub unsafe fn SQLProcedureColumns(hstmt: super::sqltypes::SQLHSTMT, szcatalogname: Option<&[super::sqltypes::SQLCHAR]>, szschemaname: Option<&[super::sqltypes::SQLCHAR]>, szprocname: Option<&[super::sqltypes::SQLCHAR]>, szcolumnname: Option<&[super::sqltypes::SQLCHAR]>) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLProcedureColumns(hstmt : super::sqltypes::SQLHSTMT, szcatalogname : *const super::sqltypes::SQLCHAR, cchcatalogname : super::sqltypes::SQLSMALLINT, szschemaname : *const super::sqltypes::SQLCHAR, cchschemaname : super::sqltypes::SQLSMALLINT, szprocname : *const super::sqltypes::SQLCHAR, cchprocname : super::sqltypes::SQLSMALLINT, szcolumnname : *const super::sqltypes::SQLCHAR, cchcolumnname : super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
    unsafe {
        SQLProcedureColumns(
            hstmt,
            core::mem::transmute(szcatalogname.map_or(core::ptr::null(), |slice| slice.as_ptr())),
            szcatalogname.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
            core::mem::transmute(szschemaname.map_or(core::ptr::null(), |slice| slice.as_ptr())),
            szschemaname.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
            core::mem::transmute(szprocname.map_or(core::ptr::null(), |slice| slice.as_ptr())),
            szprocname.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
            core::mem::transmute(szcolumnname.map_or(core::ptr::null(), |slice| slice.as_ptr())),
            szcolumnname.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
        )
    }
}
#[cfg(feature = "Win32_sqltypes")]
#[inline]
pub unsafe fn SQLProcedures(hstmt: super::sqltypes::SQLHSTMT, szcatalogname: Option<&[super::sqltypes::SQLCHAR]>, szschemaname: Option<&[super::sqltypes::SQLCHAR]>, szprocname: Option<&[super::sqltypes::SQLCHAR]>) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLProcedures(hstmt : super::sqltypes::SQLHSTMT, szcatalogname : *const super::sqltypes::SQLCHAR, cchcatalogname : super::sqltypes::SQLSMALLINT, szschemaname : *const super::sqltypes::SQLCHAR, cchschemaname : super::sqltypes::SQLSMALLINT, szprocname : *const super::sqltypes::SQLCHAR, cchprocname : super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
    unsafe {
        SQLProcedures(
            hstmt,
            core::mem::transmute(szcatalogname.map_or(core::ptr::null(), |slice| slice.as_ptr())),
            szcatalogname.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
            core::mem::transmute(szschemaname.map_or(core::ptr::null(), |slice| slice.as_ptr())),
            szschemaname.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
            core::mem::transmute(szprocname.map_or(core::ptr::null(), |slice| slice.as_ptr())),
            szprocname.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
        )
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_sqltypes")]
#[inline]
pub unsafe fn SQLSetPos(hstmt: super::sqltypes::SQLHSTMT, irow: super::sqltypes::SQLUSMALLINT, foption: super::sqltypes::SQLUSMALLINT, flock: super::sqltypes::SQLUSMALLINT) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLSetPos(hstmt : super::sqltypes::SQLHSTMT, irow : super::sqltypes::SQLUSMALLINT, foption : super::sqltypes::SQLUSMALLINT, flock : super::sqltypes::SQLUSMALLINT) -> super::sqltypes::SQLRETURN);
    unsafe { SQLSetPos(hstmt, irow, foption, flock) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_sqltypes")]
#[inline]
pub unsafe fn SQLSetPos(hstmt: super::sqltypes::SQLHSTMT, irow: super::sqltypes::SQLSETPOSIROW, foption: super::sqltypes::SQLUSMALLINT, flock: super::sqltypes::SQLUSMALLINT) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLSetPos(hstmt : super::sqltypes::SQLHSTMT, irow : super::sqltypes::SQLSETPOSIROW, foption : super::sqltypes::SQLUSMALLINT, flock : super::sqltypes::SQLUSMALLINT) -> super::sqltypes::SQLRETURN);
    unsafe { SQLSetPos(hstmt, irow, foption, flock) }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_sqltypes")]
#[inline]
pub unsafe fn SQLSetScrollOptions(hstmt: super::sqltypes::SQLHSTMT, fconcurrency: super::sqltypes::SQLUSMALLINT, crowkeyset: super::sqltypes::SQLINTEGER, crowrowset: super::sqltypes::SQLUSMALLINT) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLSetScrollOptions(hstmt : super::sqltypes::SQLHSTMT, fconcurrency : super::sqltypes::SQLUSMALLINT, crowkeyset : super::sqltypes::SQLINTEGER, crowrowset : super::sqltypes::SQLUSMALLINT) -> super::sqltypes::SQLRETURN);
    unsafe { SQLSetScrollOptions(hstmt, fconcurrency, crowkeyset, crowrowset) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_sqltypes")]
#[inline]
pub unsafe fn SQLSetScrollOptions(hstmt: super::sqltypes::SQLHSTMT, fconcurrency: super::sqltypes::SQLUSMALLINT, crowkeyset: super::sqltypes::SQLLEN, crowrowset: super::sqltypes::SQLUSMALLINT) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLSetScrollOptions(hstmt : super::sqltypes::SQLHSTMT, fconcurrency : super::sqltypes::SQLUSMALLINT, crowkeyset : super::sqltypes::SQLLEN, crowrowset : super::sqltypes::SQLUSMALLINT) -> super::sqltypes::SQLRETURN);
    unsafe { SQLSetScrollOptions(hstmt, fconcurrency, crowkeyset, crowrowset) }
}
#[cfg(feature = "Win32_sqltypes")]
#[inline]
pub unsafe fn SQLTablePrivileges(hstmt: super::sqltypes::SQLHSTMT, szcatalogname: Option<&[super::sqltypes::SQLCHAR]>, szschemaname: Option<&[super::sqltypes::SQLCHAR]>, sztablename: Option<&[super::sqltypes::SQLCHAR]>) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLTablePrivileges(hstmt : super::sqltypes::SQLHSTMT, szcatalogname : *const super::sqltypes::SQLCHAR, cchcatalogname : super::sqltypes::SQLSMALLINT, szschemaname : *const super::sqltypes::SQLCHAR, cchschemaname : super::sqltypes::SQLSMALLINT, sztablename : *const super::sqltypes::SQLCHAR, cchtablename : super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
    unsafe {
        SQLTablePrivileges(
            hstmt,
            core::mem::transmute(szcatalogname.map_or(core::ptr::null(), |slice| slice.as_ptr())),
            szcatalogname.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
            core::mem::transmute(szschemaname.map_or(core::ptr::null(), |slice| slice.as_ptr())),
            szschemaname.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
            core::mem::transmute(sztablename.map_or(core::ptr::null(), |slice| slice.as_ptr())),
            sztablename.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
        )
    }
}
#[repr(C)]
#[cfg(feature = "Win32_sqltypes")]
#[derive(Clone, Copy)]
pub struct ODBC_VS_ARGS {
    pub pguidEvent: *const windows_core::GUID,
    pub dwFlags: u32,
    pub Anonymous: ODBC_VS_ARGS_0,
    pub Anonymous2: ODBC_VS_ARGS_1,
    pub RetCode: super::sqltypes::RETCODE,
}
#[cfg(feature = "Win32_sqltypes")]
impl Default for ODBC_VS_ARGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_sqltypes")]
#[derive(Clone, Copy)]
pub union ODBC_VS_ARGS_0 {
    pub wszArg: *mut u16,
    pub szArg: *mut i8,
}
#[cfg(feature = "Win32_sqltypes")]
impl Default for ODBC_VS_ARGS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_sqltypes")]
#[derive(Clone, Copy)]
pub union ODBC_VS_ARGS_1 {
    pub wszCorrelation: *mut u16,
    pub szCorrelation: *mut i8,
}
#[cfg(feature = "Win32_sqltypes")]
impl Default for ODBC_VS_ARGS_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ODBC_VS_FLAG_RETCODE: u32 = 4;
pub const ODBC_VS_FLAG_STOP: u32 = 8;
pub const ODBC_VS_FLAG_UNICODE_ARG: u32 = 1;
pub const ODBC_VS_FLAG_UNICODE_COR: u32 = 2;
#[cfg(feature = "Win32_sqltypes")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PODBC_VS_ARGS(pub *mut ODBC_VS_ARGS);
#[cfg(feature = "Win32_sqltypes")]
impl PODBC_VS_ARGS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_sqltypes")]
impl Default for PODBC_VS_ARGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_sqltypes")]
pub type SQLSTATE = [super::sqltypes::SQLTCHAR; 6];
pub const SQL_AA_FALSE: u32 = 0;
pub const SQL_AA_TRUE: u32 = 1;
pub const SQL_ACCESS_MODE: u32 = 101;
pub const SQL_ACTIVE_CONNECTIONS: u32 = 0;
pub const SQL_ACTIVE_ENVIRONMENTS: u32 = 116;
pub const SQL_ACTIVE_STATEMENTS: u32 = 1;
pub const SQL_ADD: u32 = 4;
pub const SQL_AD_ADD_CONSTRAINT_DEFERRABLE: u32 = 128;
pub const SQL_AD_ADD_CONSTRAINT_INITIALLY_DEFERRED: u32 = 32;
pub const SQL_AD_ADD_CONSTRAINT_INITIALLY_IMMEDIATE: u32 = 64;
pub const SQL_AD_ADD_CONSTRAINT_NON_DEFERRABLE: u32 = 256;
pub const SQL_AD_ADD_DOMAIN_CONSTRAINT: u32 = 2;
pub const SQL_AD_ADD_DOMAIN_DEFAULT: u32 = 8;
pub const SQL_AD_CONSTRAINT_NAME_DEFINITION: u32 = 1;
pub const SQL_AD_DROP_DOMAIN_CONSTRAINT: u32 = 4;
pub const SQL_AD_DROP_DOMAIN_DEFAULT: u32 = 16;
pub const SQL_AF_ALL: u32 = 64;
pub const SQL_AF_AVG: u32 = 1;
pub const SQL_AF_COUNT: u32 = 2;
pub const SQL_AF_DISTINCT: u32 = 32;
pub const SQL_AF_MAX: u32 = 4;
pub const SQL_AF_MIN: u32 = 8;
pub const SQL_AF_SUM: u32 = 16;
pub const SQL_AGGREGATE_FUNCTIONS: u32 = 169;
pub const SQL_ALL_CATALOGS: windows_core::PCSTR = windows_core::s!("%");
pub const SQL_ALL_EXCEPT_LIKE: u32 = 2;
pub const SQL_ALL_SCHEMAS: windows_core::PCSTR = windows_core::s!("%");
pub const SQL_ALL_TABLE_TYPES: windows_core::PCSTR = windows_core::s!("%");
pub const SQL_ALTER_DOMAIN: u32 = 117;
pub const SQL_API_ALL_FUNCTIONS: u32 = 0;
pub const SQL_API_LOADBYORDINAL: u32 = 199;
pub const SQL_API_ODBC3_ALL_FUNCTIONS: u32 = 999;
pub const SQL_API_ODBC3_ALL_FUNCTIONS_SIZE: u32 = 250;
pub const SQL_API_SQLALLOCHANDLESTD: u32 = 73;
pub const SQL_API_SQLBINDPARAMETER: u32 = 72;
pub const SQL_API_SQLBROWSECONNECT: u32 = 55;
pub const SQL_API_SQLBULKOPERATIONS: u32 = 24;
pub const SQL_API_SQLCOLATTRIBUTES: u32 = 6;
pub const SQL_API_SQLCOLUMNPRIVILEGES: u32 = 56;
pub const SQL_API_SQLDESCRIBEPARAM: u32 = 58;
pub const SQL_API_SQLDRIVERCONNECT: u32 = 41;
pub const SQL_API_SQLDRIVERS: u32 = 71;
pub const SQL_API_SQLEXTENDEDFETCH: u32 = 59;
pub const SQL_API_SQLFOREIGNKEYS: u32 = 60;
pub const SQL_API_SQLMORERESULTS: u32 = 61;
pub const SQL_API_SQLNATIVESQL: u32 = 62;
pub const SQL_API_SQLNUMPARAMS: u32 = 63;
pub const SQL_API_SQLPARAMOPTIONS: u32 = 64;
pub const SQL_API_SQLPRIMARYKEYS: u32 = 65;
pub const SQL_API_SQLPRIVATEDRIVERS: u32 = 79;
pub const SQL_API_SQLPROCEDURECOLUMNS: u32 = 66;
pub const SQL_API_SQLPROCEDURES: u32 = 67;
pub const SQL_API_SQLSETPOS: u32 = 68;
pub const SQL_API_SQLSETSCROLLOPTIONS: u32 = 69;
pub const SQL_API_SQLTABLEPRIVILEGES: u32 = 70;
pub const SQL_ASYNC_DBC_CAPABLE: u32 = 1;
pub const SQL_ASYNC_DBC_ENABLE_DEFAULT: u32 = 0;
pub const SQL_ASYNC_DBC_ENABLE_OFF: u32 = 0;
pub const SQL_ASYNC_DBC_ENABLE_ON: u32 = 1;
pub const SQL_ASYNC_DBC_FUNCTIONS: u32 = 10023;
pub const SQL_ASYNC_DBC_NOT_CAPABLE: u32 = 0;
pub const SQL_ASYNC_ENABLE: u32 = 4;
pub const SQL_ASYNC_ENABLE_DEFAULT: u32 = 0;
pub const SQL_ASYNC_ENABLE_OFF: u32 = 0;
pub const SQL_ASYNC_ENABLE_ON: u32 = 1;
pub const SQL_ASYNC_MODE: u32 = 10021;
pub const SQL_ASYNC_NOTIFICATION: u32 = 10025;
pub const SQL_ASYNC_NOTIFICATION_CAPABLE: u32 = 1;
pub const SQL_ASYNC_NOTIFICATION_NOT_CAPABLE: u32 = 0;
pub const SQL_ATTR_ACCESS_MODE: u32 = 101;
pub const SQL_ATTR_ANSI_APP: u32 = 115;
pub const SQL_ATTR_APPLICATION_KEY: u32 = 203;
pub const SQL_ATTR_ASYNC_DBC_EVENT: u32 = 119;
pub const SQL_ATTR_ASYNC_DBC_FUNCTIONS_ENABLE: u32 = 117;
pub const SQL_ATTR_ASYNC_ENABLE: u32 = 4;
pub const SQL_ATTR_ASYNC_STMT_EVENT: u32 = 29;
pub const SQL_ATTR_AUTOCOMMIT: u32 = 102;
pub const SQL_ATTR_CONCURRENCY: u32 = 7;
pub const SQL_ATTR_CONNECTION_DEAD: u32 = 1209;
pub const SQL_ATTR_CONNECTION_POOLING: u32 = 201;
pub const SQL_ATTR_CONNECTION_TIMEOUT: u32 = 113;
pub const SQL_ATTR_CP_MATCH: u32 = 202;
pub const SQL_ATTR_CURRENT_CATALOG: u32 = 109;
pub const SQL_ATTR_CURSOR_TYPE: u32 = 6;
pub const SQL_ATTR_DISCONNECT_BEHAVIOR: u32 = 114;
pub const SQL_ATTR_ENABLE_AUTO_IPD: u32 = 15;
pub const SQL_ATTR_ENLIST_IN_DTC: u32 = 1207;
pub const SQL_ATTR_ENLIST_IN_XA: u32 = 1208;
pub const SQL_ATTR_FETCH_BOOKMARK_PTR: u32 = 16;
pub const SQL_ATTR_KEYSET_SIZE: u32 = 8;
pub const SQL_ATTR_LOGIN_TIMEOUT: u32 = 103;
pub const SQL_ATTR_MAX_LENGTH: u32 = 3;
pub const SQL_ATTR_MAX_ROWS: u32 = 1;
pub const SQL_ATTR_NOSCAN: u32 = 2;
pub const SQL_ATTR_ODBC_CURSORS: u32 = 110;
pub const SQL_ATTR_ODBC_VERSION: u32 = 200;
pub const SQL_ATTR_PACKET_SIZE: u32 = 112;
pub const SQL_ATTR_PARAMSET_SIZE: u32 = 22;
pub const SQL_ATTR_PARAMS_PROCESSED_PTR: u32 = 21;
pub const SQL_ATTR_PARAM_BIND_OFFSET_PTR: u32 = 17;
pub const SQL_ATTR_PARAM_BIND_TYPE: u32 = 18;
pub const SQL_ATTR_PARAM_OPERATION_PTR: u32 = 19;
pub const SQL_ATTR_PARAM_STATUS_PTR: u32 = 20;
pub const SQL_ATTR_QUERY_TIMEOUT: u32 = 0;
pub const SQL_ATTR_QUIET_MODE: u32 = 111;
pub const SQL_ATTR_READONLY: u32 = 0;
pub const SQL_ATTR_READWRITE_UNKNOWN: u32 = 2;
pub const SQL_ATTR_RESET_CONNECTION: u32 = 116;
pub const SQL_ATTR_RETRIEVE_DATA: u32 = 11;
pub const SQL_ATTR_ROWS_FETCHED_PTR: u32 = 26;
pub const SQL_ATTR_ROW_ARRAY_SIZE: u32 = 27;
pub const SQL_ATTR_ROW_BIND_OFFSET_PTR: u32 = 23;
pub const SQL_ATTR_ROW_BIND_TYPE: u32 = 5;
pub const SQL_ATTR_ROW_NUMBER: u32 = 14;
pub const SQL_ATTR_ROW_OPERATION_PTR: u32 = 24;
pub const SQL_ATTR_ROW_STATUS_PTR: u32 = 25;
pub const SQL_ATTR_SIMULATE_CURSOR: u32 = 10;
pub const SQL_ATTR_TRACE: u32 = 104;
pub const SQL_ATTR_TRACEFILE: u32 = 105;
pub const SQL_ATTR_TRANSLATE_LIB: u32 = 106;
pub const SQL_ATTR_TRANSLATE_OPTION: u32 = 107;
pub const SQL_ATTR_TXN_ISOLATION: u32 = 108;
pub const SQL_ATTR_USE_BOOKMARKS: u32 = 12;
pub const SQL_ATTR_WRITE: u32 = 1;
pub const SQL_AT_ADD_COLUMN_COLLATION: u32 = 128;
pub const SQL_AT_ADD_COLUMN_DEFAULT: u32 = 64;
pub const SQL_AT_ADD_COLUMN_SINGLE: u32 = 32;
pub const SQL_AT_ADD_TABLE_CONSTRAINT: u32 = 4096;
pub const SQL_AT_CONSTRAINT_DEFERRABLE: u32 = 262144;
pub const SQL_AT_CONSTRAINT_INITIALLY_DEFERRED: u32 = 65536;
pub const SQL_AT_CONSTRAINT_INITIALLY_IMMEDIATE: u32 = 131072;
pub const SQL_AT_CONSTRAINT_NAME_DEFINITION: u32 = 32768;
pub const SQL_AT_CONSTRAINT_NON_DEFERRABLE: u32 = 524288;
pub const SQL_AT_DROP_COLUMN_CASCADE: u32 = 1024;
pub const SQL_AT_DROP_COLUMN_DEFAULT: u32 = 512;
pub const SQL_AT_DROP_COLUMN_RESTRICT: u32 = 2048;
pub const SQL_AT_DROP_TABLE_CONSTRAINT_CASCADE: u32 = 8192;
pub const SQL_AT_DROP_TABLE_CONSTRAINT_RESTRICT: u32 = 16384;
pub const SQL_AT_SET_COLUMN_DEFAULT: u32 = 256;
pub const SQL_AUTOCOMMIT: u32 = 102;
pub const SQL_AUTOCOMMIT_DEFAULT: u32 = 1;
pub const SQL_AUTOCOMMIT_OFF: u32 = 0;
pub const SQL_AUTOCOMMIT_ON: u32 = 1;
pub const SQL_BATCH_ROW_COUNT: u32 = 120;
pub const SQL_BATCH_SUPPORT: u32 = 121;
pub const SQL_BEST_ROWID: u32 = 1;
pub const SQL_BIGINT: i32 = -5;
pub const SQL_BINARY: i32 = -2;
pub const SQL_BIND_BY_COLUMN: u32 = 0;
pub const SQL_BIND_TYPE: u32 = 5;
pub const SQL_BIND_TYPE_DEFAULT: u32 = 0;
pub const SQL_BIT: i32 = -7;
pub const SQL_BOOKMARK_PERSISTENCE: u32 = 82;
pub const SQL_BP_CLOSE: u32 = 1;
pub const SQL_BP_DELETE: u32 = 2;
pub const SQL_BP_DROP: u32 = 4;
pub const SQL_BP_OTHER_HSTMT: u32 = 32;
pub const SQL_BP_SCROLL: u32 = 64;
pub const SQL_BP_TRANSACTION: u32 = 8;
pub const SQL_BP_UPDATE: u32 = 16;
pub const SQL_BRC_EXPLICIT: u32 = 2;
pub const SQL_BRC_PROCEDURES: u32 = 1;
pub const SQL_BRC_ROLLED_UP: u32 = 4;
pub const SQL_BS_ROW_COUNT_EXPLICIT: u32 = 2;
pub const SQL_BS_ROW_COUNT_PROC: u32 = 8;
pub const SQL_BS_SELECT_EXPLICIT: u32 = 1;
pub const SQL_BS_SELECT_PROC: u32 = 4;
pub const SQL_CA1_ABSOLUTE: u32 = 2;
pub const SQL_CA1_BOOKMARK: u32 = 8;
pub const SQL_CA1_BULK_ADD: u32 = 65536;
pub const SQL_CA1_BULK_DELETE_BY_BOOKMARK: u32 = 262144;
pub const SQL_CA1_BULK_FETCH_BY_BOOKMARK: u32 = 524288;
pub const SQL_CA1_BULK_UPDATE_BY_BOOKMARK: u32 = 131072;
pub const SQL_CA1_LOCK_EXCLUSIVE: u32 = 128;
pub const SQL_CA1_LOCK_NO_CHANGE: u32 = 64;
pub const SQL_CA1_LOCK_UNLOCK: u32 = 256;
pub const SQL_CA1_NEXT: u32 = 1;
pub const SQL_CA1_POSITIONED_DELETE: u32 = 16384;
pub const SQL_CA1_POSITIONED_UPDATE: u32 = 8192;
pub const SQL_CA1_POS_DELETE: u32 = 2048;
pub const SQL_CA1_POS_POSITION: u32 = 512;
pub const SQL_CA1_POS_REFRESH: u32 = 4096;
pub const SQL_CA1_POS_UPDATE: u32 = 1024;
pub const SQL_CA1_RELATIVE: u32 = 4;
pub const SQL_CA1_SELECT_FOR_UPDATE: u32 = 32768;
pub const SQL_CA2_CRC_APPROXIMATE: u32 = 8192;
pub const SQL_CA2_CRC_EXACT: u32 = 4096;
pub const SQL_CA2_LOCK_CONCURRENCY: u32 = 2;
pub const SQL_CA2_MAX_ROWS_AFFECTS_ALL: u32 = 3968;
pub const SQL_CA2_MAX_ROWS_CATALOG: u32 = 2048;
pub const SQL_CA2_MAX_ROWS_DELETE: u32 = 512;
pub const SQL_CA2_MAX_ROWS_INSERT: u32 = 256;
pub const SQL_CA2_MAX_ROWS_SELECT: u32 = 128;
pub const SQL_CA2_MAX_ROWS_UPDATE: u32 = 1024;
pub const SQL_CA2_OPT_ROWVER_CONCURRENCY: u32 = 4;
pub const SQL_CA2_OPT_VALUES_CONCURRENCY: u32 = 8;
pub const SQL_CA2_READ_ONLY_CONCURRENCY: u32 = 1;
pub const SQL_CA2_SENSITIVITY_ADDITIONS: u32 = 16;
pub const SQL_CA2_SENSITIVITY_DELETIONS: u32 = 32;
pub const SQL_CA2_SENSITIVITY_UPDATES: u32 = 64;
pub const SQL_CA2_SIMULATE_NON_UNIQUE: u32 = 16384;
pub const SQL_CA2_SIMULATE_TRY_UNIQUE: u32 = 32768;
pub const SQL_CA2_SIMULATE_UNIQUE: u32 = 65536;
pub const SQL_CASCADE: u32 = 0;
pub const SQL_CATALOG_LOCATION: u32 = 114;
pub const SQL_CATALOG_NAME_SEPARATOR: u32 = 41;
pub const SQL_CATALOG_TERM: u32 = 42;
pub const SQL_CATALOG_USAGE: u32 = 92;
pub const SQL_CA_CONSTRAINT_DEFERRABLE: u32 = 64;
pub const SQL_CA_CONSTRAINT_INITIALLY_DEFERRED: u32 = 16;
pub const SQL_CA_CONSTRAINT_INITIALLY_IMMEDIATE: u32 = 32;
pub const SQL_CA_CONSTRAINT_NON_DEFERRABLE: u32 = 128;
pub const SQL_CA_CREATE_ASSERTION: u32 = 1;
pub const SQL_CB_NON_NULL: u32 = 1;
pub const SQL_CB_NULL: u32 = 0;
pub const SQL_CCOL_CREATE_COLLATION: u32 = 1;
pub const SQL_CCS_COLLATE_CLAUSE: u32 = 2;
pub const SQL_CCS_CREATE_CHARACTER_SET: u32 = 1;
pub const SQL_CCS_LIMITED_COLLATION: u32 = 4;
pub const SQL_CC_CLOSE: u32 = 1;
pub const SQL_CC_DELETE: u32 = 0;
pub const SQL_CC_PRESERVE: u32 = 2;
pub const SQL_CDO_COLLATION: u32 = 8;
pub const SQL_CDO_CONSTRAINT: u32 = 4;
pub const SQL_CDO_CONSTRAINT_DEFERRABLE: u32 = 128;
pub const SQL_CDO_CONSTRAINT_INITIALLY_DEFERRED: u32 = 32;
pub const SQL_CDO_CONSTRAINT_INITIALLY_IMMEDIATE: u32 = 64;
pub const SQL_CDO_CONSTRAINT_NAME_DEFINITION: u32 = 16;
pub const SQL_CDO_CONSTRAINT_NON_DEFERRABLE: u32 = 256;
pub const SQL_CDO_CREATE_DOMAIN: u32 = 1;
pub const SQL_CDO_DEFAULT: u32 = 2;
pub const SQL_CD_FALSE: u32 = 0;
pub const SQL_CD_TRUE: u32 = 1;
pub const SQL_CL_END: u32 = 2;
pub const SQL_CL_START: u32 = 1;
pub const SQL_CN_ANY: u32 = 2;
pub const SQL_CN_DIFFERENT: u32 = 1;
pub const SQL_CN_NONE: u32 = 0;
pub const SQL_CODE_DAY: u32 = 3;
pub const SQL_CODE_DAY_TO_HOUR: u32 = 8;
pub const SQL_CODE_DAY_TO_MINUTE: u32 = 9;
pub const SQL_CODE_DAY_TO_SECOND: u32 = 10;
pub const SQL_CODE_HOUR: u32 = 4;
pub const SQL_CODE_HOUR_TO_MINUTE: u32 = 11;
pub const SQL_CODE_HOUR_TO_SECOND: u32 = 12;
pub const SQL_CODE_MINUTE: u32 = 5;
pub const SQL_CODE_MINUTE_TO_SECOND: u32 = 13;
pub const SQL_CODE_MONTH: u32 = 2;
pub const SQL_CODE_SECOND: u32 = 6;
pub const SQL_CODE_YEAR: u32 = 1;
pub const SQL_CODE_YEAR_TO_MONTH: u32 = 7;
pub const SQL_COLATT_OPT_MAX: u32 = 18;
pub const SQL_COLATT_OPT_MIN: u32 = 0;
pub const SQL_COLUMN_ALIAS: u32 = 87;
pub const SQL_COLUMN_AUTO_INCREMENT: u32 = 11;
pub const SQL_COLUMN_CASE_SENSITIVE: u32 = 12;
pub const SQL_COLUMN_COUNT: u32 = 0;
pub const SQL_COLUMN_DISPLAY_SIZE: u32 = 6;
pub const SQL_COLUMN_IGNORE: i32 = -6;
pub const SQL_COLUMN_LABEL: u32 = 18;
pub const SQL_COLUMN_LENGTH: u32 = 3;
pub const SQL_COLUMN_MONEY: u32 = 9;
pub const SQL_COLUMN_NAME: u32 = 1;
pub const SQL_COLUMN_NULLABLE: u32 = 7;
pub const SQL_COLUMN_NUMBER_UNKNOWN: i32 = -2;
pub const SQL_COLUMN_OWNER_NAME: u32 = 16;
pub const SQL_COLUMN_PRECISION: u32 = 4;
pub const SQL_COLUMN_QUALIFIER_NAME: u32 = 17;
pub const SQL_COLUMN_SCALE: u32 = 5;
pub const SQL_COLUMN_SEARCHABLE: u32 = 13;
pub const SQL_COLUMN_TABLE_NAME: u32 = 15;
pub const SQL_COLUMN_TYPE: u32 = 2;
pub const SQL_COLUMN_TYPE_NAME: u32 = 14;
pub const SQL_COLUMN_UNSIGNED: u32 = 8;
pub const SQL_COLUMN_UPDATABLE: u32 = 10;
pub const SQL_COL_PRED_BASIC: u32 = 2;
pub const SQL_COL_PRED_CHAR: u32 = 1;
pub const SQL_CONCAT_NULL_BEHAVIOR: u32 = 22;
pub const SQL_CONCURRENCY: u32 = 7;
pub const SQL_CONCUR_DEFAULT: u32 = 1;
pub const SQL_CONCUR_LOCK: u32 = 2;
pub const SQL_CONCUR_READ_ONLY: u32 = 1;
pub const SQL_CONCUR_ROWVER: u32 = 3;
pub const SQL_CONCUR_TIMESTAMP: u32 = 3;
pub const SQL_CONCUR_VALUES: u32 = 4;
pub const SQL_CONVERT_BIGINT: u32 = 53;
pub const SQL_CONVERT_BINARY: u32 = 54;
pub const SQL_CONVERT_BIT: u32 = 55;
pub const SQL_CONVERT_CHAR: u32 = 56;
pub const SQL_CONVERT_DATE: u32 = 57;
pub const SQL_CONVERT_DECIMAL: u32 = 58;
pub const SQL_CONVERT_DOUBLE: u32 = 59;
pub const SQL_CONVERT_FLOAT: u32 = 60;
pub const SQL_CONVERT_FUNCTIONS: u32 = 48;
pub const SQL_CONVERT_GUID: u32 = 173;
pub const SQL_CONVERT_INTEGER: u32 = 61;
pub const SQL_CONVERT_INTERVAL_DAY_TIME: u32 = 123;
pub const SQL_CONVERT_INTERVAL_YEAR_MONTH: u32 = 124;
pub const SQL_CONVERT_LONGVARBINARY: u32 = 71;
pub const SQL_CONVERT_LONGVARCHAR: u32 = 62;
pub const SQL_CONVERT_NUMERIC: u32 = 63;
pub const SQL_CONVERT_REAL: u32 = 64;
pub const SQL_CONVERT_SMALLINT: u32 = 65;
pub const SQL_CONVERT_TIME: u32 = 66;
pub const SQL_CONVERT_TIMESTAMP: u32 = 67;
pub const SQL_CONVERT_TINYINT: u32 = 68;
pub const SQL_CONVERT_VARBINARY: u32 = 69;
pub const SQL_CONVERT_VARCHAR: u32 = 70;
pub const SQL_CONVERT_WCHAR: u32 = 122;
pub const SQL_CONVERT_WLONGVARCHAR: u32 = 125;
pub const SQL_CONVERT_WVARCHAR: u32 = 126;
pub const SQL_CORRELATION_NAME: u32 = 74;
pub const SQL_CP_DEFAULT: u32 = 0;
pub const SQL_CP_DRIVER_AWARE: u32 = 3;
pub const SQL_CP_MATCH_DEFAULT: u32 = 0;
pub const SQL_CP_OFF: u32 = 0;
pub const SQL_CP_ONE_PER_DRIVER: u32 = 1;
pub const SQL_CP_ONE_PER_HENV: u32 = 2;
pub const SQL_CP_RELAXED_MATCH: u32 = 1;
pub const SQL_CP_STRICT_MATCH: u32 = 0;
pub const SQL_CREATE_ASSERTION: u32 = 127;
pub const SQL_CREATE_CHARACTER_SET: u32 = 128;
pub const SQL_CREATE_COLLATION: u32 = 129;
pub const SQL_CREATE_DOMAIN: u32 = 130;
pub const SQL_CREATE_SCHEMA: u32 = 131;
pub const SQL_CREATE_TABLE: u32 = 132;
pub const SQL_CREATE_TRANSLATION: u32 = 133;
pub const SQL_CREATE_VIEW: u32 = 134;
pub const SQL_CR_CLOSE: u32 = 1;
pub const SQL_CR_DELETE: u32 = 0;
pub const SQL_CR_PRESERVE: u32 = 2;
pub const SQL_CS_AUTHORIZATION: u32 = 2;
pub const SQL_CS_CREATE_SCHEMA: u32 = 1;
pub const SQL_CS_DEFAULT_CHARACTER_SET: u32 = 4;
pub const SQL_CTR_CREATE_TRANSLATION: u32 = 1;
pub const SQL_CT_COLUMN_COLLATION: u32 = 2048;
pub const SQL_CT_COLUMN_CONSTRAINT: u32 = 512;
pub const SQL_CT_COLUMN_DEFAULT: u32 = 1024;
pub const SQL_CT_COMMIT_DELETE: u32 = 4;
pub const SQL_CT_COMMIT_PRESERVE: u32 = 2;
pub const SQL_CT_CONSTRAINT_DEFERRABLE: u32 = 128;
pub const SQL_CT_CONSTRAINT_INITIALLY_DEFERRED: u32 = 32;
pub const SQL_CT_CONSTRAINT_INITIALLY_IMMEDIATE: u32 = 64;
pub const SQL_CT_CONSTRAINT_NAME_DEFINITION: u32 = 8192;
pub const SQL_CT_CONSTRAINT_NON_DEFERRABLE: u32 = 256;
pub const SQL_CT_CREATE_TABLE: u32 = 1;
pub const SQL_CT_GLOBAL_TEMPORARY: u32 = 8;
pub const SQL_CT_LOCAL_TEMPORARY: u32 = 16;
pub const SQL_CT_TABLE_CONSTRAINT: u32 = 4096;
pub const SQL_CURRENT_QUALIFIER: u32 = 109;
pub const SQL_CURSOR_DYNAMIC: u32 = 2;
pub const SQL_CURSOR_FORWARD_ONLY: u32 = 0;
pub const SQL_CURSOR_KEYSET_DRIVEN: u32 = 1;
pub const SQL_CURSOR_ROLLBACK_BEHAVIOR: u32 = 24;
pub const SQL_CURSOR_STATIC: u32 = 3;
pub const SQL_CURSOR_TYPE: u32 = 6;
pub const SQL_CURSOR_TYPE_DEFAULT: u32 = 0;
pub const SQL_CUR_DEFAULT: u32 = 2;
pub const SQL_CUR_USE_DRIVER: u32 = 2;
pub const SQL_CUR_USE_IF_NEEDED: u32 = 0;
pub const SQL_CUR_USE_ODBC: u32 = 1;
pub const SQL_CU_DML_STATEMENTS: u32 = 1;
pub const SQL_CU_INDEX_DEFINITION: u32 = 8;
pub const SQL_CU_PRIVILEGE_DEFINITION: u32 = 16;
pub const SQL_CU_PROCEDURE_INVOCATION: u32 = 2;
pub const SQL_CU_TABLE_DEFINITION: u32 = 4;
pub const SQL_CVT_BIGINT: u32 = 16384;
pub const SQL_CVT_BINARY: u32 = 1024;
pub const SQL_CVT_BIT: u32 = 4096;
pub const SQL_CVT_CHAR: u32 = 1;
pub const SQL_CVT_DATE: u32 = 32768;
pub const SQL_CVT_DECIMAL: u32 = 4;
pub const SQL_CVT_DOUBLE: u32 = 128;
pub const SQL_CVT_FLOAT: u32 = 32;
pub const SQL_CVT_GUID: u32 = 16777216;
pub const SQL_CVT_INTEGER: u32 = 8;
pub const SQL_CVT_INTERVAL_DAY_TIME: u32 = 1048576;
pub const SQL_CVT_INTERVAL_YEAR_MONTH: u32 = 524288;
pub const SQL_CVT_LONGVARBINARY: u32 = 262144;
pub const SQL_CVT_LONGVARCHAR: u32 = 512;
pub const SQL_CVT_NUMERIC: u32 = 2;
pub const SQL_CVT_REAL: u32 = 64;
pub const SQL_CVT_SMALLINT: u32 = 16;
pub const SQL_CVT_TIME: u32 = 65536;
pub const SQL_CVT_TIMESTAMP: u32 = 131072;
pub const SQL_CVT_TINYINT: u32 = 8192;
pub const SQL_CVT_VARBINARY: u32 = 2048;
pub const SQL_CVT_VARCHAR: u32 = 256;
pub const SQL_CVT_WCHAR: u32 = 2097152;
pub const SQL_CVT_WLONGVARCHAR: u32 = 4194304;
pub const SQL_CVT_WVARCHAR: u32 = 8388608;
pub const SQL_CV_CASCADED: u32 = 4;
pub const SQL_CV_CHECK_OPTION: u32 = 2;
pub const SQL_CV_CREATE_VIEW: u32 = 1;
pub const SQL_CV_LOCAL: u32 = 8;
pub const SQL_C_BINARY: i32 = -2;
pub const SQL_C_BIT: i32 = -7;
#[cfg(target_arch = "x86")]
pub const SQL_C_BOOKMARK: i32 = -18;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const SQL_C_BOOKMARK: i32 = -27;
pub const SQL_C_CHAR: u32 = 1;
pub const SQL_C_DATE: u32 = 9;
pub const SQL_C_DEFAULT: u32 = 99;
pub const SQL_C_DOUBLE: u32 = 8;
pub const SQL_C_FLOAT: u32 = 7;
pub const SQL_C_GUID: i32 = -11;
pub const SQL_C_INTERVAL_DAY: u32 = 103;
pub const SQL_C_INTERVAL_DAY_TO_HOUR: u32 = 108;
pub const SQL_C_INTERVAL_DAY_TO_MINUTE: u32 = 109;
pub const SQL_C_INTERVAL_DAY_TO_SECOND: u32 = 110;
pub const SQL_C_INTERVAL_HOUR: u32 = 104;
pub const SQL_C_INTERVAL_HOUR_TO_MINUTE: u32 = 111;
pub const SQL_C_INTERVAL_HOUR_TO_SECOND: u32 = 112;
pub const SQL_C_INTERVAL_MINUTE: u32 = 105;
pub const SQL_C_INTERVAL_MINUTE_TO_SECOND: u32 = 113;
pub const SQL_C_INTERVAL_MONTH: u32 = 102;
pub const SQL_C_INTERVAL_SECOND: u32 = 106;
pub const SQL_C_INTERVAL_YEAR: u32 = 101;
pub const SQL_C_INTERVAL_YEAR_TO_MONTH: u32 = 107;
pub const SQL_C_LONG: u32 = 4;
pub const SQL_C_NUMERIC: u32 = 2;
pub const SQL_C_SBIGINT: i32 = -25;
pub const SQL_C_SHORT: u32 = 5;
pub const SQL_C_SLONG: i32 = -16;
pub const SQL_C_SSHORT: i32 = -15;
pub const SQL_C_STINYINT: i32 = -26;
pub const SQL_C_TIME: u32 = 10;
pub const SQL_C_TIMESTAMP: u32 = 11;
pub const SQL_C_TINYINT: i32 = -6;
pub const SQL_C_TYPE_DATE: u32 = 91;
pub const SQL_C_TYPE_TIME: u32 = 92;
pub const SQL_C_TYPE_TIMESTAMP: u32 = 93;
pub const SQL_C_UBIGINT: i32 = -27;
pub const SQL_C_ULONG: i32 = -18;
pub const SQL_C_USHORT: i32 = -17;
pub const SQL_C_UTINYINT: i32 = -28;
pub const SQL_C_VARBOOKMARK: i32 = -2;
pub const SQL_DATABASE_NAME: u32 = 16;
pub const SQL_DATE: u32 = 9;
pub const SQL_DATETIME_LITERALS: u32 = 119;
pub const SQL_DA_DROP_ASSERTION: u32 = 1;
pub const SQL_DB_DEFAULT: u32 = 0;
pub const SQL_DB_DISCONNECT: u32 = 1;
pub const SQL_DB_RETURN_TO_POOL: u32 = 0;
pub const SQL_DCS_DROP_CHARACTER_SET: u32 = 1;
pub const SQL_DC_DROP_COLLATION: u32 = 1;
pub const SQL_DDL_INDEX: u32 = 170;
pub const SQL_DD_CASCADE: u32 = 4;
pub const SQL_DD_DROP_DOMAIN: u32 = 1;
pub const SQL_DD_RESTRICT: u32 = 2;
pub const SQL_DEFAULT_PARAM: i32 = -5;
pub const SQL_DELETE: u32 = 3;
pub const SQL_DELETE_BY_BOOKMARK: u32 = 6;
pub const SQL_DESC_ARRAY_SIZE: u32 = 20;
pub const SQL_DESC_ARRAY_STATUS_PTR: u32 = 21;
pub const SQL_DESC_AUTO_UNIQUE_VALUE: u32 = 11;
pub const SQL_DESC_BASE_COLUMN_NAME: u32 = 22;
pub const SQL_DESC_BASE_TABLE_NAME: u32 = 23;
pub const SQL_DESC_BIND_OFFSET_PTR: u32 = 24;
pub const SQL_DESC_BIND_TYPE: u32 = 25;
pub const SQL_DESC_CASE_SENSITIVE: u32 = 12;
pub const SQL_DESC_CATALOG_NAME: u32 = 17;
pub const SQL_DESC_CONCISE_TYPE: u32 = 2;
pub const SQL_DESC_DATETIME_INTERVAL_PRECISION: u32 = 26;
pub const SQL_DESC_DISPLAY_SIZE: u32 = 6;
pub const SQL_DESC_FIXED_PREC_SCALE: u32 = 9;
pub const SQL_DESC_LABEL: u32 = 18;
pub const SQL_DESC_LITERAL_PREFIX: u32 = 27;
pub const SQL_DESC_LITERAL_SUFFIX: u32 = 28;
pub const SQL_DESC_LOCAL_TYPE_NAME: u32 = 29;
pub const SQL_DESC_MAXIMUM_SCALE: u32 = 30;
pub const SQL_DESC_MINIMUM_SCALE: u32 = 31;
pub const SQL_DESC_NUM_PREC_RADIX: u32 = 32;
pub const SQL_DESC_PARAMETER_TYPE: u32 = 33;
pub const SQL_DESC_ROWS_PROCESSED_PTR: u32 = 34;
pub const SQL_DESC_ROWVER: u32 = 35;
pub const SQL_DESC_SCHEMA_NAME: u32 = 16;
pub const SQL_DESC_SEARCHABLE: u32 = 13;
pub const SQL_DESC_TABLE_NAME: u32 = 15;
pub const SQL_DESC_TYPE_NAME: u32 = 14;
pub const SQL_DESC_UNSIGNED: u32 = 8;
pub const SQL_DESC_UPDATABLE: u32 = 10;
pub const SQL_DIAG_COLUMN_NUMBER: i32 = -1247;
pub const SQL_DIAG_CURSOR_ROW_COUNT: i32 = -1249;
pub const SQL_DIAG_ROW_NUMBER: i32 = -1248;
pub const SQL_DI_CREATE_INDEX: u32 = 1;
pub const SQL_DI_DROP_INDEX: u32 = 2;
pub const SQL_DL_SQL92_DATE: u32 = 1;
pub const SQL_DL_SQL92_INTERVAL_DAY: u32 = 32;
pub const SQL_DL_SQL92_INTERVAL_DAY_TO_HOUR: u32 = 1024;
pub const SQL_DL_SQL92_INTERVAL_DAY_TO_MINUTE: u32 = 2048;
pub const SQL_DL_SQL92_INTERVAL_DAY_TO_SECOND: u32 = 4096;
pub const SQL_DL_SQL92_INTERVAL_HOUR: u32 = 64;
pub const SQL_DL_SQL92_INTERVAL_HOUR_TO_MINUTE: u32 = 8192;
pub const SQL_DL_SQL92_INTERVAL_HOUR_TO_SECOND: u32 = 16384;
pub const SQL_DL_SQL92_INTERVAL_MINUTE: u32 = 128;
pub const SQL_DL_SQL92_INTERVAL_MINUTE_TO_SECOND: u32 = 32768;
pub const SQL_DL_SQL92_INTERVAL_MONTH: u32 = 16;
pub const SQL_DL_SQL92_INTERVAL_SECOND: u32 = 256;
pub const SQL_DL_SQL92_INTERVAL_YEAR: u32 = 8;
pub const SQL_DL_SQL92_INTERVAL_YEAR_TO_MONTH: u32 = 512;
pub const SQL_DL_SQL92_TIME: u32 = 2;
pub const SQL_DL_SQL92_TIMESTAMP: u32 = 4;
pub const SQL_DM_VER: u32 = 171;
pub const SQL_DRIVER_AWARE_POOLING_CAPABLE: u32 = 1;
pub const SQL_DRIVER_AWARE_POOLING_NOT_CAPABLE: u32 = 0;
pub const SQL_DRIVER_AWARE_POOLING_SUPPORTED: u32 = 10024;
pub const SQL_DRIVER_COMPLETE: u32 = 1;
pub const SQL_DRIVER_COMPLETE_REQUIRED: u32 = 3;
pub const SQL_DRIVER_CONN_ATTR_BASE: u32 = 16384;
pub const SQL_DRIVER_C_TYPE_BASE: u32 = 16384;
pub const SQL_DRIVER_DESC_FIELD_BASE: u32 = 16384;
pub const SQL_DRIVER_DIAG_FIELD_BASE: u32 = 16384;
pub const SQL_DRIVER_HDBC: u32 = 3;
pub const SQL_DRIVER_HDESC: u32 = 135;
pub const SQL_DRIVER_HENV: u32 = 4;
pub const SQL_DRIVER_HLIB: u32 = 76;
pub const SQL_DRIVER_HSTMT: u32 = 5;
pub const SQL_DRIVER_INFO_TYPE_BASE: u32 = 16384;
pub const SQL_DRIVER_NAME: u32 = 6;
pub const SQL_DRIVER_NOPROMPT: u32 = 0;
pub const SQL_DRIVER_ODBC_VER: u32 = 77;
pub const SQL_DRIVER_PROMPT: u32 = 2;
pub const SQL_DRIVER_SQL_TYPE_BASE: u32 = 16384;
pub const SQL_DRIVER_STMT_ATTR_BASE: u32 = 16384;
pub const SQL_DRIVER_VER: u32 = 7;
pub const SQL_DROP_ASSERTION: u32 = 136;
pub const SQL_DROP_CHARACTER_SET: u32 = 137;
pub const SQL_DROP_COLLATION: u32 = 138;
pub const SQL_DROP_DOMAIN: u32 = 139;
pub const SQL_DROP_SCHEMA: u32 = 140;
pub const SQL_DROP_TABLE: u32 = 141;
pub const SQL_DROP_TRANSLATION: u32 = 142;
pub const SQL_DROP_VIEW: u32 = 143;
pub const SQL_DS_CASCADE: u32 = 4;
pub const SQL_DS_DROP_SCHEMA: u32 = 1;
pub const SQL_DS_RESTRICT: u32 = 2;
pub const SQL_DTC_DONE: u32 = 0;
pub const SQL_DTC_ENLIST_EXPENSIVE: u32 = 1;
pub const SQL_DTC_TRANSITION_COST: u32 = 1750;
pub const SQL_DTC_UNENLIST_EXPENSIVE: u32 = 2;
pub const SQL_DTR_DROP_TRANSLATION: u32 = 1;
pub const SQL_DT_CASCADE: u32 = 4;
pub const SQL_DT_DROP_TABLE: u32 = 1;
pub const SQL_DT_RESTRICT: u32 = 2;
pub const SQL_DV_CASCADE: u32 = 4;
pub const SQL_DV_DROP_VIEW: u32 = 1;
pub const SQL_DV_RESTRICT: u32 = 2;
pub const SQL_DYNAMIC_CURSOR_ATTRIBUTES1: u32 = 144;
pub const SQL_DYNAMIC_CURSOR_ATTRIBUTES2: u32 = 145;
pub const SQL_ENSURE: u32 = 1;
pub const SQL_ENTIRE_ROWSET: u32 = 0;
pub const SQL_EXPRESSIONS_IN_ORDERBY: u32 = 27;
pub const SQL_FD_FETCH_BOOKMARK: u32 = 128;
pub const SQL_FD_FETCH_PREV: u32 = 8;
pub const SQL_FETCH_BOOKMARK: u32 = 8;
pub const SQL_FETCH_BY_BOOKMARK: u32 = 7;
pub const SQL_FETCH_FIRST_SYSTEM: u32 = 32;
pub const SQL_FETCH_FIRST_USER: u32 = 31;
pub const SQL_FETCH_PREV: u32 = 4;
pub const SQL_FILE_CATALOG: u32 = 2;
pub const SQL_FILE_NOT_SUPPORTED: u32 = 0;
pub const SQL_FILE_QUALIFIER: u32 = 2;
pub const SQL_FILE_TABLE: u32 = 1;
pub const SQL_FILE_USAGE: u32 = 84;
pub const SQL_FN_CVT_CAST: u32 = 2;
pub const SQL_FN_CVT_CONVERT: u32 = 1;
pub const SQL_FN_NUM_ABS: u32 = 1;
pub const SQL_FN_NUM_ACOS: u32 = 2;
pub const SQL_FN_NUM_ASIN: u32 = 4;
pub const SQL_FN_NUM_ATAN: u32 = 8;
pub const SQL_FN_NUM_ATAN2: u32 = 16;
pub const SQL_FN_NUM_CEILING: u32 = 32;
pub const SQL_FN_NUM_COS: u32 = 64;
pub const SQL_FN_NUM_COT: u32 = 128;
pub const SQL_FN_NUM_DEGREES: u32 = 262144;
pub const SQL_FN_NUM_EXP: u32 = 256;
pub const SQL_FN_NUM_FLOOR: u32 = 512;
pub const SQL_FN_NUM_LOG: u32 = 1024;
pub const SQL_FN_NUM_LOG10: u32 = 524288;
pub const SQL_FN_NUM_MOD: u32 = 2048;
pub const SQL_FN_NUM_PI: u32 = 65536;
pub const SQL_FN_NUM_POWER: u32 = 1048576;
pub const SQL_FN_NUM_RADIANS: u32 = 2097152;
pub const SQL_FN_NUM_RAND: u32 = 131072;
pub const SQL_FN_NUM_ROUND: u32 = 4194304;
pub const SQL_FN_NUM_SIGN: u32 = 4096;
pub const SQL_FN_NUM_SIN: u32 = 8192;
pub const SQL_FN_NUM_SQRT: u32 = 16384;
pub const SQL_FN_NUM_TAN: u32 = 32768;
pub const SQL_FN_NUM_TRUNCATE: u32 = 8388608;
pub const SQL_FN_STR_ASCII: u32 = 8192;
pub const SQL_FN_STR_BIT_LENGTH: u32 = 524288;
pub const SQL_FN_STR_CHAR: u32 = 16384;
pub const SQL_FN_STR_CHARACTER_LENGTH: u32 = 2097152;
pub const SQL_FN_STR_CHAR_LENGTH: u32 = 1048576;
pub const SQL_FN_STR_CONCAT: u32 = 1;
pub const SQL_FN_STR_DIFFERENCE: u32 = 32768;
pub const SQL_FN_STR_INSERT: u32 = 2;
pub const SQL_FN_STR_LCASE: u32 = 64;
pub const SQL_FN_STR_LEFT: u32 = 4;
pub const SQL_FN_STR_LENGTH: u32 = 16;
pub const SQL_FN_STR_LOCATE: u32 = 32;
pub const SQL_FN_STR_LOCATE_2: u32 = 65536;
pub const SQL_FN_STR_LTRIM: u32 = 8;
pub const SQL_FN_STR_OCTET_LENGTH: u32 = 4194304;
pub const SQL_FN_STR_POSITION: u32 = 8388608;
pub const SQL_FN_STR_REPEAT: u32 = 128;
pub const SQL_FN_STR_REPLACE: u32 = 256;
pub const SQL_FN_STR_RIGHT: u32 = 512;
pub const SQL_FN_STR_RTRIM: u32 = 1024;
pub const SQL_FN_STR_SOUNDEX: u32 = 131072;
pub const SQL_FN_STR_SPACE: u32 = 262144;
pub const SQL_FN_STR_SUBSTRING: u32 = 2048;
pub const SQL_FN_STR_UCASE: u32 = 4096;
pub const SQL_FN_SYS_DBNAME: u32 = 2;
pub const SQL_FN_SYS_IFNULL: u32 = 4;
pub const SQL_FN_SYS_USERNAME: u32 = 1;
pub const SQL_FN_TD_CURDATE: u32 = 2;
pub const SQL_FN_TD_CURRENT_DATE: u32 = 131072;
pub const SQL_FN_TD_CURRENT_TIME: u32 = 262144;
pub const SQL_FN_TD_CURRENT_TIMESTAMP: u32 = 524288;
pub const SQL_FN_TD_CURTIME: u32 = 512;
pub const SQL_FN_TD_DAYNAME: u32 = 32768;
pub const SQL_FN_TD_DAYOFMONTH: u32 = 4;
pub const SQL_FN_TD_DAYOFWEEK: u32 = 8;
pub const SQL_FN_TD_DAYOFYEAR: u32 = 16;
pub const SQL_FN_TD_EXTRACT: u32 = 1048576;
pub const SQL_FN_TD_HOUR: u32 = 1024;
pub const SQL_FN_TD_MINUTE: u32 = 2048;
pub const SQL_FN_TD_MONTH: u32 = 32;
pub const SQL_FN_TD_MONTHNAME: u32 = 65536;
pub const SQL_FN_TD_NOW: u32 = 1;
pub const SQL_FN_TD_QUARTER: u32 = 64;
pub const SQL_FN_TD_SECOND: u32 = 4096;
pub const SQL_FN_TD_TIMESTAMPADD: u32 = 8192;
pub const SQL_FN_TD_TIMESTAMPDIFF: u32 = 16384;
pub const SQL_FN_TD_WEEK: u32 = 128;
pub const SQL_FN_TD_YEAR: u32 = 256;
pub const SQL_FN_TSI_DAY: u32 = 16;
pub const SQL_FN_TSI_FRAC_SECOND: u32 = 1;
pub const SQL_FN_TSI_HOUR: u32 = 8;
pub const SQL_FN_TSI_MINUTE: u32 = 4;
pub const SQL_FN_TSI_MONTH: u32 = 64;
pub const SQL_FN_TSI_QUARTER: u32 = 128;
pub const SQL_FN_TSI_SECOND: u32 = 2;
pub const SQL_FN_TSI_WEEK: u32 = 32;
pub const SQL_FN_TSI_YEAR: u32 = 256;
pub const SQL_FORWARD_ONLY_CURSOR_ATTRIBUTES1: u32 = 146;
pub const SQL_FORWARD_ONLY_CURSOR_ATTRIBUTES2: u32 = 147;
pub const SQL_GB_COLLATE: u32 = 4;
pub const SQL_GB_GROUP_BY_CONTAINS_SELECT: u32 = 2;
pub const SQL_GB_GROUP_BY_EQUALS_SELECT: u32 = 1;
pub const SQL_GB_NOT_SUPPORTED: u32 = 0;
pub const SQL_GB_NO_RELATION: u32 = 3;
pub const SQL_GD_BLOCK: u32 = 4;
pub const SQL_GD_BOUND: u32 = 8;
pub const SQL_GD_OUTPUT_PARAMS: u32 = 16;
pub const SQL_GET_BOOKMARK: u32 = 13;
pub const SQL_GROUP_BY: u32 = 88;
pub const SQL_GUID: i32 = -11;
pub const SQL_HANDLE_SENV: u32 = 5;
pub const SQL_IGNORE: i32 = -6;
pub const SQL_IK_ALL: u32 = 3;
pub const SQL_IK_ASC: u32 = 1;
pub const SQL_IK_DESC: u32 = 2;
pub const SQL_IK_NONE: u32 = 0;
pub const SQL_INDEX_KEYWORDS: u32 = 148;
pub const SQL_INFO_FIRST: u32 = 0;
pub const SQL_INFO_SCHEMA_VIEWS: u32 = 149;
pub const SQL_INITIALLY_DEFERRED: u32 = 5;
pub const SQL_INITIALLY_IMMEDIATE: u32 = 6;
pub const SQL_INSERT_STATEMENT: u32 = 172;
pub const SQL_INTERVAL: u32 = 10;
pub const SQL_INTERVAL_DAY: u32 = 103;
pub const SQL_INTERVAL_DAY_TO_HOUR: u32 = 108;
pub const SQL_INTERVAL_DAY_TO_MINUTE: u32 = 109;
pub const SQL_INTERVAL_DAY_TO_SECOND: u32 = 110;
pub const SQL_INTERVAL_HOUR: u32 = 104;
pub const SQL_INTERVAL_HOUR_TO_MINUTE: u32 = 111;
pub const SQL_INTERVAL_HOUR_TO_SECOND: u32 = 112;
pub const SQL_INTERVAL_MINUTE: u32 = 105;
pub const SQL_INTERVAL_MINUTE_TO_SECOND: u32 = 113;
pub const SQL_INTERVAL_MONTH: u32 = 102;
pub const SQL_INTERVAL_SECOND: u32 = 106;
pub const SQL_INTERVAL_YEAR: u32 = 101;
pub const SQL_INTERVAL_YEAR_TO_MONTH: u32 = 107;
pub const SQL_ISV_ASSERTIONS: u32 = 1;
pub const SQL_ISV_CHARACTER_SETS: u32 = 2;
pub const SQL_ISV_CHECK_CONSTRAINTS: u32 = 4;
pub const SQL_ISV_COLLATIONS: u32 = 8;
pub const SQL_ISV_COLUMNS: u32 = 64;
pub const SQL_ISV_COLUMN_DOMAIN_USAGE: u32 = 16;
pub const SQL_ISV_COLUMN_PRIVILEGES: u32 = 32;
pub const SQL_ISV_CONSTRAINT_COLUMN_USAGE: u32 = 128;
pub const SQL_ISV_CONSTRAINT_TABLE_USAGE: u32 = 256;
pub const SQL_ISV_DOMAINS: u32 = 1024;
pub const SQL_ISV_DOMAIN_CONSTRAINTS: u32 = 512;
pub const SQL_ISV_KEY_COLUMN_USAGE: u32 = 2048;
pub const SQL_ISV_REFERENTIAL_CONSTRAINTS: u32 = 4096;
pub const SQL_ISV_SCHEMATA: u32 = 8192;
pub const SQL_ISV_SQL_LANGUAGES: u32 = 16384;
pub const SQL_ISV_TABLES: u32 = 131072;
pub const SQL_ISV_TABLE_CONSTRAINTS: u32 = 32768;
pub const SQL_ISV_TABLE_PRIVILEGES: u32 = 65536;
pub const SQL_ISV_TRANSLATIONS: u32 = 262144;
pub const SQL_ISV_USAGE_PRIVILEGES: u32 = 524288;
pub const SQL_ISV_VIEWS: u32 = 4194304;
pub const SQL_ISV_VIEW_COLUMN_USAGE: u32 = 1048576;
pub const SQL_ISV_VIEW_TABLE_USAGE: u32 = 2097152;
pub const SQL_IS_INSERT_LITERALS: u32 = 1;
pub const SQL_IS_INSERT_SEARCHED: u32 = 2;
pub const SQL_IS_INTEGER: i32 = -6;
pub const SQL_IS_POINTER: i32 = -4;
pub const SQL_IS_SELECT_INTO: u32 = 4;
pub const SQL_IS_SMALLINT: i32 = -8;
pub const SQL_IS_UINTEGER: i32 = -5;
pub const SQL_IS_USMALLINT: i32 = -7;
pub const SQL_KEYSET_CURSOR_ATTRIBUTES1: u32 = 150;
pub const SQL_KEYSET_CURSOR_ATTRIBUTES2: u32 = 151;
pub const SQL_KEYSET_SIZE: u32 = 8;
pub const SQL_KEYSET_SIZE_DEFAULT: u32 = 0;
pub const SQL_KEYWORDS: u32 = 89;
pub const SQL_LCK_EXCLUSIVE: u32 = 2;
pub const SQL_LCK_NO_CHANGE: u32 = 1;
pub const SQL_LCK_UNLOCK: u32 = 4;
pub const SQL_LEN_BINARY_ATTR_OFFSET: i32 = -100;
pub const SQL_LEN_DATA_AT_EXEC_OFFSET: i32 = -100;
pub const SQL_LIKE_ESCAPE_CLAUSE: u32 = 113;
pub const SQL_LIKE_ONLY: u32 = 1;
pub const SQL_LOCK_EXCLUSIVE: u32 = 1;
pub const SQL_LOCK_NO_CHANGE: u32 = 0;
pub const SQL_LOCK_TYPES: u32 = 78;
pub const SQL_LOCK_UNLOCK: u32 = 2;
pub const SQL_LOGIN_TIMEOUT: u32 = 103;
pub const SQL_LOGIN_TIMEOUT_DEFAULT: u32 = 15;
pub const SQL_LONGVARBINARY: i32 = -4;
pub const SQL_LONGVARCHAR: i32 = -1;
pub const SQL_MAX_ASYNC_CONCURRENT_STATEMENTS: u32 = 10022;
pub const SQL_MAX_BINARY_LITERAL_LEN: u32 = 112;
pub const SQL_MAX_CHAR_LITERAL_LEN: u32 = 108;
pub const SQL_MAX_DSN_LENGTH: u32 = 32;
pub const SQL_MAX_LENGTH: u32 = 3;
pub const SQL_MAX_LENGTH_DEFAULT: u32 = 0;
pub const SQL_MAX_OPTION_STRING_LENGTH: u32 = 256;
pub const SQL_MAX_OWNER_NAME_LEN: u32 = 32;
pub const SQL_MAX_PROCEDURE_NAME_LEN: u32 = 33;
pub const SQL_MAX_QUALIFIER_NAME_LEN: u32 = 34;
pub const SQL_MAX_ROWS: u32 = 1;
pub const SQL_MAX_ROWS_DEFAULT: u32 = 0;
pub const SQL_MAX_ROW_SIZE_INCLUDES_LONG: u32 = 103;
pub const SQL_MODE_DEFAULT: u32 = 0;
pub const SQL_MODE_READ_ONLY: u32 = 1;
pub const SQL_MODE_READ_WRITE: u32 = 0;
pub const SQL_MULTIPLE_ACTIVE_TXN: u32 = 37;
pub const SQL_MULT_RESULT_SETS: u32 = 36;
pub const SQL_NC_END: u32 = 4;
pub const SQL_NC_START: u32 = 2;
pub const SQL_NEED_LONG_DATA_LEN: u32 = 111;
pub const SQL_NNC_NON_NULL: u32 = 1;
pub const SQL_NNC_NULL: u32 = 0;
pub const SQL_NON_NULLABLE_COLUMNS: u32 = 75;
pub const SQL_NOSCAN: u32 = 2;
pub const SQL_NOSCAN_DEFAULT: u32 = 0;
pub const SQL_NOSCAN_OFF: u32 = 0;
pub const SQL_NOSCAN_ON: u32 = 1;
pub const SQL_NOT_DEFERRABLE: u32 = 7;
pub const SQL_NO_ACTION: u32 = 3;
pub const SQL_NO_COLUMN_NUMBER: i32 = -1;
pub const SQL_NO_DATA_FOUND: u32 = 100;
pub const SQL_NO_ROW_NUMBER: i32 = -1;
pub const SQL_NO_TOTAL: i32 = -4;
pub const SQL_NUMERIC_FUNCTIONS: u32 = 49;
pub const SQL_OAC_LEVEL1: u32 = 1;
pub const SQL_OAC_LEVEL2: u32 = 2;
pub const SQL_OAC_NONE: u32 = 0;
pub const SQL_ODBC_API_CONFORMANCE: u32 = 9;
pub const SQL_ODBC_CURSORS: u32 = 110;
pub const SQL_ODBC_INTERFACE_CONFORMANCE: u32 = 152;
pub const SQL_ODBC_SAG_CLI_CONFORMANCE: u32 = 12;
pub const SQL_ODBC_SQL_CONFORMANCE: u32 = 15;
pub const SQL_ODBC_SQL_OPT_IEF: u32 = 73;
pub const SQL_ODBC_VER: u32 = 10;
pub const SQL_OIC_CORE: u32 = 1;
pub const SQL_OIC_LEVEL1: u32 = 2;
pub const SQL_OIC_LEVEL2: u32 = 3;
pub const SQL_OPT_TRACE: u32 = 104;
pub const SQL_OPT_TRACEFILE: u32 = 105;
pub const SQL_OPT_TRACE_DEFAULT: u32 = 0;
pub const SQL_OPT_TRACE_FILE_DEFAULT: windows_core::PCSTR = windows_core::s!("\\SQL.LOG");
pub const SQL_OPT_TRACE_OFF: u32 = 0;
pub const SQL_OPT_TRACE_ON: u32 = 1;
pub const SQL_OSCC_COMPLIANT: u32 = 1;
pub const SQL_OSCC_NOT_COMPLIANT: u32 = 0;
pub const SQL_OSC_CORE: u32 = 1;
pub const SQL_OSC_EXTENDED: u32 = 2;
pub const SQL_OSC_MINIMUM: u32 = 0;
pub const SQL_OUTER_JOINS: u32 = 38;
pub const SQL_OU_DML_STATEMENTS: u32 = 1;
pub const SQL_OU_INDEX_DEFINITION: u32 = 8;
pub const SQL_OU_PRIVILEGE_DEFINITION: u32 = 16;
pub const SQL_OU_PROCEDURE_INVOCATION: u32 = 2;
pub const SQL_OU_TABLE_DEFINITION: u32 = 4;
pub const SQL_OV_ODBC2: u32 = 2;
pub const SQL_OV_ODBC3: u32 = 3;
pub const SQL_OV_ODBC3_80: u32 = 380;
pub const SQL_OWNER_TERM: u32 = 39;
pub const SQL_OWNER_USAGE: u32 = 91;
pub const SQL_PACKET_SIZE: u32 = 112;
pub const SQL_PARAM_ARRAY_ROW_COUNTS: u32 = 153;
pub const SQL_PARAM_ARRAY_SELECTS: u32 = 154;
pub const SQL_PARAM_BIND_BY_COLUMN: u32 = 0;
pub const SQL_PARAM_BIND_TYPE_DEFAULT: u32 = 0;
pub const SQL_PARAM_DIAG_UNAVAILABLE: u32 = 1;
pub const SQL_PARAM_ERROR: u32 = 5;
pub const SQL_PARAM_IGNORE: u32 = 1;
pub const SQL_PARAM_INPUT: u32 = 1;
pub const SQL_PARAM_INPUT_OUTPUT: u32 = 2;
pub const SQL_PARAM_INPUT_OUTPUT_STREAM: u32 = 8;
pub const SQL_PARAM_OUTPUT: u32 = 4;
pub const SQL_PARAM_OUTPUT_STREAM: u32 = 16;
pub const SQL_PARAM_PROCEED: u32 = 0;
pub const SQL_PARAM_SUCCESS: u32 = 0;
pub const SQL_PARAM_SUCCESS_WITH_INFO: u32 = 6;
pub const SQL_PARAM_TYPE_DEFAULT: u32 = 2;
pub const SQL_PARAM_TYPE_UNKNOWN: u32 = 0;
pub const SQL_PARAM_UNUSED: u32 = 7;
pub const SQL_PARC_BATCH: u32 = 1;
pub const SQL_PARC_NO_BATCH: u32 = 2;
pub const SQL_PAS_BATCH: u32 = 1;
pub const SQL_PAS_NO_BATCH: u32 = 2;
pub const SQL_PAS_NO_SELECT: u32 = 3;
pub const SQL_PC_NOT_PSEUDO: u32 = 1;
pub const SQL_POSITION: u32 = 0;
pub const SQL_POSITIONED_STATEMENTS: u32 = 80;
pub const SQL_POS_ADD: u32 = 16;
pub const SQL_POS_DELETE: u32 = 8;
pub const SQL_POS_OPERATIONS: u32 = 79;
pub const SQL_POS_POSITION: u32 = 1;
pub const SQL_POS_REFRESH: u32 = 2;
pub const SQL_POS_UPDATE: u32 = 4;
pub const SQL_PRED_SEARCHABLE: u32 = 3;
pub const SQL_PROCEDURES: u32 = 21;
pub const SQL_PROCEDURE_TERM: u32 = 40;
pub const SQL_PS_POSITIONED_DELETE: u32 = 1;
pub const SQL_PS_POSITIONED_UPDATE: u32 = 2;
pub const SQL_PS_SELECT_FOR_UPDATE: u32 = 4;
pub const SQL_PT_FUNCTION: u32 = 2;
pub const SQL_PT_PROCEDURE: u32 = 1;
pub const SQL_PT_UNKNOWN: u32 = 0;
pub const SQL_QL_END: u32 = 2;
pub const SQL_QL_START: u32 = 1;
pub const SQL_QUALIFIER_LOCATION: u32 = 114;
pub const SQL_QUALIFIER_NAME_SEPARATOR: u32 = 41;
pub const SQL_QUALIFIER_TERM: u32 = 42;
pub const SQL_QUALIFIER_USAGE: u32 = 92;
pub const SQL_QUERY_TIMEOUT: u32 = 0;
pub const SQL_QUERY_TIMEOUT_DEFAULT: u32 = 0;
pub const SQL_QUICK: u32 = 0;
pub const SQL_QUIET_MODE: u32 = 111;
pub const SQL_QUOTED_IDENTIFIER_CASE: u32 = 93;
pub const SQL_QU_DML_STATEMENTS: u32 = 1;
pub const SQL_QU_INDEX_DEFINITION: u32 = 8;
pub const SQL_QU_PRIVILEGE_DEFINITION: u32 = 16;
pub const SQL_QU_PROCEDURE_INVOCATION: u32 = 2;
pub const SQL_QU_TABLE_DEFINITION: u32 = 4;
pub const SQL_RD_DEFAULT: u32 = 1;
pub const SQL_RD_OFF: u32 = 0;
pub const SQL_RD_ON: u32 = 1;
pub const SQL_REFRESH: u32 = 1;
pub const SQL_RESET_CONNECTION_YES: u32 = 1;
pub const SQL_RESTRICT: u32 = 1;
pub const SQL_RESULT_COL: u32 = 3;
pub const SQL_RETRIEVE_DATA: u32 = 11;
pub const SQL_RETURN_VALUE: u32 = 5;
pub const SQL_ROWSET_SIZE: u32 = 9;
pub const SQL_ROWSET_SIZE_DEFAULT: u32 = 1;
pub const SQL_ROWVER: u32 = 2;
pub const SQL_ROW_ADDED: u32 = 4;
pub const SQL_ROW_DELETED: u32 = 1;
pub const SQL_ROW_ERROR: u32 = 5;
pub const SQL_ROW_IGNORE: u32 = 1;
pub const SQL_ROW_NOROW: u32 = 3;
pub const SQL_ROW_NUMBER: u32 = 14;
pub const SQL_ROW_NUMBER_UNKNOWN: i32 = -2;
pub const SQL_ROW_PROCEED: u32 = 0;
pub const SQL_ROW_SUCCESS: u32 = 0;
pub const SQL_ROW_SUCCESS_WITH_INFO: u32 = 6;
pub const SQL_ROW_UPDATED: u32 = 2;
pub const SQL_ROW_UPDATES: u32 = 11;
pub const SQL_SCCO_OPT_TIMESTAMP: u32 = 4;
pub const SQL_SCC_ISO92_CLI: u32 = 2;
pub const SQL_SCC_XOPEN_CLI_VERSION1: u32 = 1;
pub const SQL_SCHEMA_TERM: u32 = 39;
pub const SQL_SCHEMA_USAGE: u32 = 91;
pub const SQL_SCROLL_DYNAMIC: i32 = -2;
pub const SQL_SCROLL_FORWARD_ONLY: u32 = 0;
pub const SQL_SCROLL_KEYSET_DRIVEN: i32 = -1;
pub const SQL_SCROLL_OPTIONS: u32 = 44;
pub const SQL_SCROLL_STATIC: i32 = -3;
pub const SQL_SC_FIPS127_2_TRANSITIONAL: u32 = 2;
pub const SQL_SC_NON_UNIQUE: u32 = 0;
pub const SQL_SC_SQL92_ENTRY: u32 = 1;
pub const SQL_SC_SQL92_FULL: u32 = 8;
pub const SQL_SC_SQL92_INTERMEDIATE: u32 = 4;
pub const SQL_SC_TRY_UNIQUE: u32 = 1;
pub const SQL_SC_UNIQUE: u32 = 2;
pub const SQL_SDF_CURRENT_DATE: u32 = 1;
pub const SQL_SDF_CURRENT_TIME: u32 = 2;
pub const SQL_SDF_CURRENT_TIMESTAMP: u32 = 4;
pub const SQL_SEARCHABLE: u32 = 3;
pub const SQL_SETPARAM_VALUE_MAX: i32 = -1;
pub const SQL_SETPOS_MAX_LOCK_VALUE: u32 = 2;
pub const SQL_SETPOS_MAX_OPTION_VALUE: u32 = 4;
pub const SQL_SET_DEFAULT: u32 = 4;
pub const SQL_SET_NULL: u32 = 2;
pub const SQL_SFKD_CASCADE: u32 = 1;
pub const SQL_SFKD_NO_ACTION: u32 = 2;
pub const SQL_SFKD_SET_DEFAULT: u32 = 4;
pub const SQL_SFKD_SET_NULL: u32 = 8;
pub const SQL_SFKU_CASCADE: u32 = 1;
pub const SQL_SFKU_NO_ACTION: u32 = 2;
pub const SQL_SFKU_SET_DEFAULT: u32 = 4;
pub const SQL_SFKU_SET_NULL: u32 = 8;
pub const SQL_SG_DELETE_TABLE: u32 = 32;
pub const SQL_SG_INSERT_COLUMN: u32 = 128;
pub const SQL_SG_INSERT_TABLE: u32 = 64;
pub const SQL_SG_REFERENCES_COLUMN: u32 = 512;
pub const SQL_SG_REFERENCES_TABLE: u32 = 256;
pub const SQL_SG_SELECT_TABLE: u32 = 1024;
pub const SQL_SG_UPDATE_COLUMN: u32 = 4096;
pub const SQL_SG_UPDATE_TABLE: u32 = 2048;
pub const SQL_SG_USAGE_ON_CHARACTER_SET: u32 = 2;
pub const SQL_SG_USAGE_ON_COLLATION: u32 = 4;
pub const SQL_SG_USAGE_ON_DOMAIN: u32 = 1;
pub const SQL_SG_USAGE_ON_TRANSLATION: u32 = 8;
pub const SQL_SG_WITH_GRANT_OPTION: u32 = 16;
pub const SQL_SIGNED_OFFSET: i32 = -20;
pub const SQL_SIMULATE_CURSOR: u32 = 10;
pub const SQL_SNVF_BIT_LENGTH: u32 = 1;
pub const SQL_SNVF_CHARACTER_LENGTH: u32 = 4;
pub const SQL_SNVF_CHAR_LENGTH: u32 = 2;
pub const SQL_SNVF_EXTRACT: u32 = 8;
pub const SQL_SNVF_OCTET_LENGTH: u32 = 16;
pub const SQL_SNVF_POSITION: u32 = 32;
pub const SQL_SO_DYNAMIC: u32 = 4;
pub const SQL_SO_FORWARD_ONLY: u32 = 1;
pub const SQL_SO_KEYSET_DRIVEN: u32 = 2;
pub const SQL_SO_MIXED: u32 = 8;
pub const SQL_SO_STATIC: u32 = 16;
pub const SQL_SPEC_MAJOR: u32 = 3;
pub const SQL_SPEC_MINOR: u32 = 80;
pub const SQL_SPEC_STRING: windows_core::PCSTR = windows_core::s!("03.80");
pub const SQL_SP_BETWEEN: u32 = 2048;
pub const SQL_SP_COMPARISON: u32 = 4096;
pub const SQL_SP_EXISTS: u32 = 1;
pub const SQL_SP_IN: u32 = 1024;
pub const SQL_SP_ISNOTNULL: u32 = 2;
pub const SQL_SP_ISNULL: u32 = 4;
pub const SQL_SP_LIKE: u32 = 512;
pub const SQL_SP_MATCH_FULL: u32 = 8;
pub const SQL_SP_MATCH_PARTIAL: u32 = 16;
pub const SQL_SP_MATCH_UNIQUE_FULL: u32 = 32;
pub const SQL_SP_MATCH_UNIQUE_PARTIAL: u32 = 64;
pub const SQL_SP_OVERLAPS: u32 = 128;
pub const SQL_SP_QUANTIFIED_COMPARISON: u32 = 8192;
pub const SQL_SP_UNIQUE: u32 = 256;
pub const SQL_SQL92_DATETIME_FUNCTIONS: u32 = 155;
pub const SQL_SQL92_FOREIGN_KEY_DELETE_RULE: u32 = 156;
pub const SQL_SQL92_FOREIGN_KEY_UPDATE_RULE: u32 = 157;
pub const SQL_SQL92_GRANT: u32 = 158;
pub const SQL_SQL92_NUMERIC_VALUE_FUNCTIONS: u32 = 159;
pub const SQL_SQL92_PREDICATES: u32 = 160;
pub const SQL_SQL92_RELATIONAL_JOIN_OPERATORS: u32 = 161;
pub const SQL_SQL92_REVOKE: u32 = 162;
pub const SQL_SQL92_ROW_VALUE_CONSTRUCTOR: u32 = 163;
pub const SQL_SQL92_STRING_FUNCTIONS: u32 = 164;
pub const SQL_SQL92_VALUE_EXPRESSIONS: u32 = 165;
pub const SQL_SQLSTATE_SIZE: u32 = 5;
pub const SQL_SQL_CONFORMANCE: u32 = 118;
pub const SQL_SQ_COMPARISON: u32 = 1;
pub const SQL_SQ_CORRELATED_SUBQUERIES: u32 = 16;
pub const SQL_SQ_EXISTS: u32 = 2;
pub const SQL_SQ_IN: u32 = 4;
pub const SQL_SQ_QUANTIFIED: u32 = 8;
pub const SQL_SRJO_CORRESPONDING_CLAUSE: u32 = 1;
pub const SQL_SRJO_CROSS_JOIN: u32 = 2;
pub const SQL_SRJO_EXCEPT_JOIN: u32 = 4;
pub const SQL_SRJO_FULL_OUTER_JOIN: u32 = 8;
pub const SQL_SRJO_INNER_JOIN: u32 = 16;
pub const SQL_SRJO_INTERSECT_JOIN: u32 = 32;
pub const SQL_SRJO_LEFT_OUTER_JOIN: u32 = 64;
pub const SQL_SRJO_NATURAL_JOIN: u32 = 128;
pub const SQL_SRJO_RIGHT_OUTER_JOIN: u32 = 256;
pub const SQL_SRJO_UNION_JOIN: u32 = 512;
pub const SQL_SRVC_DEFAULT: u32 = 4;
pub const SQL_SRVC_NULL: u32 = 2;
pub const SQL_SRVC_ROW_SUBQUERY: u32 = 8;
pub const SQL_SRVC_VALUE_EXPRESSION: u32 = 1;
pub const SQL_SR_CASCADE: u32 = 32;
pub const SQL_SR_DELETE_TABLE: u32 = 128;
pub const SQL_SR_GRANT_OPTION_FOR: u32 = 16;
pub const SQL_SR_INSERT_COLUMN: u32 = 512;
pub const SQL_SR_INSERT_TABLE: u32 = 256;
pub const SQL_SR_REFERENCES_COLUMN: u32 = 2048;
pub const SQL_SR_REFERENCES_TABLE: u32 = 1024;
pub const SQL_SR_RESTRICT: u32 = 64;
pub const SQL_SR_SELECT_TABLE: u32 = 4096;
pub const SQL_SR_UPDATE_COLUMN: u32 = 16384;
pub const SQL_SR_UPDATE_TABLE: u32 = 8192;
pub const SQL_SR_USAGE_ON_CHARACTER_SET: u32 = 2;
pub const SQL_SR_USAGE_ON_COLLATION: u32 = 4;
pub const SQL_SR_USAGE_ON_DOMAIN: u32 = 1;
pub const SQL_SR_USAGE_ON_TRANSLATION: u32 = 8;
pub const SQL_SSF_CONVERT: u32 = 1;
pub const SQL_SSF_LOWER: u32 = 2;
pub const SQL_SSF_SUBSTRING: u32 = 8;
pub const SQL_SSF_TRANSLATE: u32 = 16;
pub const SQL_SSF_TRIM_BOTH: u32 = 32;
pub const SQL_SSF_TRIM_LEADING: u32 = 64;
pub const SQL_SSF_TRIM_TRAILING: u32 = 128;
pub const SQL_SSF_UPPER: u32 = 4;
pub const SQL_SS_ADDITIONS: u32 = 1;
pub const SQL_SS_DELETIONS: u32 = 2;
pub const SQL_SS_UPDATES: u32 = 4;
pub const SQL_STANDARD_CLI_CONFORMANCE: u32 = 166;
pub const SQL_STATIC_CURSOR_ATTRIBUTES1: u32 = 167;
pub const SQL_STATIC_CURSOR_ATTRIBUTES2: u32 = 168;
pub const SQL_STATIC_SENSITIVITY: u32 = 83;
pub const SQL_STRING_FUNCTIONS: u32 = 50;
pub const SQL_SUBQUERIES: u32 = 95;
pub const SQL_SU_DML_STATEMENTS: u32 = 1;
pub const SQL_SU_INDEX_DEFINITION: u32 = 8;
pub const SQL_SU_PRIVILEGE_DEFINITION: u32 = 16;
pub const SQL_SU_PROCEDURE_INVOCATION: u32 = 2;
pub const SQL_SU_TABLE_DEFINITION: u32 = 4;
pub const SQL_SVE_CASE: u32 = 1;
pub const SQL_SVE_CAST: u32 = 2;
pub const SQL_SVE_COALESCE: u32 = 4;
pub const SQL_SVE_NULLIF: u32 = 8;
pub const SQL_SYSTEM_FUNCTIONS: u32 = 51;
pub const SQL_TABLE_STAT: u32 = 0;
pub const SQL_TABLE_TERM: u32 = 45;
pub const SQL_TIME: u32 = 10;
pub const SQL_TIMEDATE_ADD_INTERVALS: u32 = 109;
pub const SQL_TIMEDATE_DIFF_INTERVALS: u32 = 110;
pub const SQL_TIMEDATE_FUNCTIONS: u32 = 52;
pub const SQL_TIMESTAMP: u32 = 11;
pub const SQL_TINYINT: i32 = -6;
pub const SQL_TRANSLATE_DLL: u32 = 106;
pub const SQL_TRANSLATE_OPTION: u32 = 107;
pub const SQL_TXN_ISOLATION: u32 = 108;
pub const SQL_TYPE_NULL: u32 = 0;
pub const SQL_UB_DEFAULT: u32 = 0;
pub const SQL_UB_FIXED: u32 = 1;
pub const SQL_UB_OFF: u32 = 0;
pub const SQL_UB_ON: u32 = 1;
pub const SQL_UB_VARIABLE: u32 = 2;
pub const SQL_UNICODE: i32 = -8;
pub const SQL_UNICODE_CHAR: i32 = -8;
pub const SQL_UNICODE_LONGVARCHAR: i32 = -10;
pub const SQL_UNICODE_VARCHAR: i32 = -9;
pub const SQL_UNION: u32 = 96;
pub const SQL_UNION_STATEMENT: u32 = 96;
pub const SQL_UNSEARCHABLE: u32 = 0;
pub const SQL_UNSIGNED_OFFSET: i32 = -22;
pub const SQL_UPDATE: u32 = 2;
pub const SQL_UPDATE_BY_BOOKMARK: u32 = 5;
pub const SQL_USE_BOOKMARKS: u32 = 12;
pub const SQL_US_UNION: u32 = 1;
pub const SQL_US_UNION_ALL: u32 = 2;
pub const SQL_U_UNION: u32 = 1;
pub const SQL_U_UNION_ALL: u32 = 2;
pub const SQL_VARBINARY: i32 = -3;
pub const TRACE_ON: u32 = 1;
pub const TRACE_VERSION: u32 = 1000;
pub const TRACE_VS_EVENT_ON: u32 = 2;
