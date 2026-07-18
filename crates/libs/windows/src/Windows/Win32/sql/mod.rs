#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLAllocConnect(environmenthandle: super::SQLHENV, connectionhandle: *mut super::SQLHDBC) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLAllocConnect(environmenthandle : super::SQLHENV, connectionhandle : *mut super::SQLHDBC) -> super::SQLRETURN);
    unsafe { SQLAllocConnect(environmenthandle, connectionhandle as _) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLAllocEnv(environmenthandle: *mut super::SQLHENV) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLAllocEnv(environmenthandle : *mut super::SQLHENV) -> super::SQLRETURN);
    unsafe { SQLAllocEnv(environmenthandle as _) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLAllocHandle(handletype: super::SQLSMALLINT, inputhandle: super::SQLHANDLE, outputhandle: *mut super::SQLHANDLE) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLAllocHandle(handletype : super::SQLSMALLINT, inputhandle : super::SQLHANDLE, outputhandle : *mut super::SQLHANDLE) -> super::SQLRETURN);
    unsafe { SQLAllocHandle(handletype, inputhandle, outputhandle as _) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLAllocStmt(connectionhandle: super::SQLHDBC, statementhandle: *mut super::SQLHSTMT) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLAllocStmt(connectionhandle : super::SQLHDBC, statementhandle : *mut super::SQLHSTMT) -> super::SQLRETURN);
    unsafe { SQLAllocStmt(connectionhandle, statementhandle as _) }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLBindCol(statementhandle: super::SQLHSTMT, columnnumber: super::SQLUSMALLINT, targettype: super::SQLSMALLINT, targetvalue: Option<super::SQLPOINTER>, bufferlength: super::SQLINTEGER, strlen_or_ind: Option<*mut super::SQLINTEGER>) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLBindCol(statementhandle : super::SQLHSTMT, columnnumber : super::SQLUSMALLINT, targettype : super::SQLSMALLINT, targetvalue : super::SQLPOINTER, bufferlength : super::SQLINTEGER, strlen_or_ind : *mut super::SQLINTEGER) -> super::SQLRETURN);
    unsafe { SQLBindCol(statementhandle, columnnumber, targettype, targetvalue.unwrap_or(core::mem::zeroed()) as _, bufferlength, strlen_or_ind.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLBindCol(statementhandle: super::SQLHSTMT, columnnumber: super::SQLUSMALLINT, targettype: super::SQLSMALLINT, targetvalue: Option<super::SQLPOINTER>, bufferlength: super::SQLLEN, strlen_or_ind: Option<*mut super::SQLLEN>) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLBindCol(statementhandle : super::SQLHSTMT, columnnumber : super::SQLUSMALLINT, targettype : super::SQLSMALLINT, targetvalue : super::SQLPOINTER, bufferlength : super::SQLLEN, strlen_or_ind : *mut super::SQLLEN) -> super::SQLRETURN);
    unsafe { SQLBindCol(statementhandle, columnnumber, targettype, targetvalue.unwrap_or(core::mem::zeroed()) as _, bufferlength, strlen_or_ind.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLBindParam(statementhandle: super::SQLHSTMT, parameternumber: super::SQLUSMALLINT, valuetype: super::SQLSMALLINT, parametertype: super::SQLSMALLINT, lengthprecision: super::SQLUINTEGER, parameterscale: super::SQLSMALLINT, parametervalue: super::SQLPOINTER, strlen_or_ind: *mut super::SQLINTEGER) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLBindParam(statementhandle : super::SQLHSTMT, parameternumber : super::SQLUSMALLINT, valuetype : super::SQLSMALLINT, parametertype : super::SQLSMALLINT, lengthprecision : super::SQLUINTEGER, parameterscale : super::SQLSMALLINT, parametervalue : super::SQLPOINTER, strlen_or_ind : *mut super::SQLINTEGER) -> super::SQLRETURN);
    unsafe { SQLBindParam(statementhandle, parameternumber, valuetype, parametertype, lengthprecision, parameterscale, parametervalue, strlen_or_ind as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLBindParam(statementhandle: super::SQLHSTMT, parameternumber: super::SQLUSMALLINT, valuetype: super::SQLSMALLINT, parametertype: super::SQLSMALLINT, lengthprecision: super::SQLULEN, parameterscale: super::SQLSMALLINT, parametervalue: super::SQLPOINTER, strlen_or_ind: *mut super::SQLLEN) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLBindParam(statementhandle : super::SQLHSTMT, parameternumber : super::SQLUSMALLINT, valuetype : super::SQLSMALLINT, parametertype : super::SQLSMALLINT, lengthprecision : super::SQLULEN, parameterscale : super::SQLSMALLINT, parametervalue : super::SQLPOINTER, strlen_or_ind : *mut super::SQLLEN) -> super::SQLRETURN);
    unsafe { SQLBindParam(statementhandle, parameternumber, valuetype, parametertype, lengthprecision, parameterscale, parametervalue, strlen_or_ind as _) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLCancel(statementhandle: super::SQLHSTMT) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLCancel(statementhandle : super::SQLHSTMT) -> super::SQLRETURN);
    unsafe { SQLCancel(statementhandle) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLCancelHandle(handletype: super::SQLSMALLINT, inputhandle: super::SQLHANDLE) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLCancelHandle(handletype : super::SQLSMALLINT, inputhandle : super::SQLHANDLE) -> super::SQLRETURN);
    unsafe { SQLCancelHandle(handletype, inputhandle) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLCloseCursor(statementhandle: super::SQLHSTMT) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLCloseCursor(statementhandle : super::SQLHSTMT) -> super::SQLRETURN);
    unsafe { SQLCloseCursor(statementhandle) }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLColAttribute(statementhandle: super::SQLHSTMT, columnnumber: super::SQLUSMALLINT, fieldidentifier: super::SQLUSMALLINT, characterattribute: Option<super::SQLPOINTER>, bufferlength: super::SQLSMALLINT, stringlength: Option<*mut super::SQLSMALLINT>, numericattribute: Option<super::SQLPOINTER>) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLColAttribute(statementhandle : super::SQLHSTMT, columnnumber : super::SQLUSMALLINT, fieldidentifier : super::SQLUSMALLINT, characterattribute : super::SQLPOINTER, bufferlength : super::SQLSMALLINT, stringlength : *mut super::SQLSMALLINT, numericattribute : super::SQLPOINTER) -> super::SQLRETURN);
    unsafe { SQLColAttribute(statementhandle, columnnumber, fieldidentifier, characterattribute.unwrap_or(core::mem::zeroed()) as _, bufferlength, stringlength.unwrap_or(core::mem::zeroed()) as _, numericattribute.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLColAttribute(statementhandle: super::SQLHSTMT, columnnumber: super::SQLUSMALLINT, fieldidentifier: super::SQLUSMALLINT, characterattribute: Option<super::SQLPOINTER>, bufferlength: super::SQLSMALLINT, stringlength: Option<*mut super::SQLSMALLINT>, numericattribute: Option<*mut super::SQLLEN>) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLColAttribute(statementhandle : super::SQLHSTMT, columnnumber : super::SQLUSMALLINT, fieldidentifier : super::SQLUSMALLINT, characterattribute : super::SQLPOINTER, bufferlength : super::SQLSMALLINT, stringlength : *mut super::SQLSMALLINT, numericattribute : *mut super::SQLLEN) -> super::SQLRETURN);
    unsafe { SQLColAttribute(statementhandle, columnnumber, fieldidentifier, characterattribute.unwrap_or(core::mem::zeroed()) as _, bufferlength, stringlength.unwrap_or(core::mem::zeroed()) as _, numericattribute.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLColumns(statementhandle: super::SQLHSTMT, catalogname: Option<&[super::SQLCHAR]>, schemaname: Option<&[super::SQLCHAR]>, tablename: Option<&[super::SQLCHAR]>, columnname: Option<&[super::SQLCHAR]>) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLColumns(statementhandle : super::SQLHSTMT, catalogname : *const super::SQLCHAR, namelength1 : super::SQLSMALLINT, schemaname : *const super::SQLCHAR, namelength2 : super::SQLSMALLINT, tablename : *const super::SQLCHAR, namelength3 : super::SQLSMALLINT, columnname : *const super::SQLCHAR, namelength4 : super::SQLSMALLINT) -> super::SQLRETURN);
    unsafe {
        SQLColumns(
            statementhandle,
            catalogname.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            catalogname.map_or(super::SQLSMALLINT(0), |slice| super::SQLSMALLINT(slice.len().try_into().unwrap())),
            schemaname.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            schemaname.map_or(super::SQLSMALLINT(0), |slice| super::SQLSMALLINT(slice.len().try_into().unwrap())),
            tablename.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            tablename.map_or(super::SQLSMALLINT(0), |slice| super::SQLSMALLINT(slice.len().try_into().unwrap())),
            columnname.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            columnname.map_or(super::SQLSMALLINT(0), |slice| super::SQLSMALLINT(slice.len().try_into().unwrap())),
        )
    }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLCompleteAsync(handletype: super::SQLSMALLINT, handle: super::SQLHANDLE, asyncretcodeptr: *mut super::RETCODE) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLCompleteAsync(handletype : super::SQLSMALLINT, handle : super::SQLHANDLE, asyncretcodeptr : *mut super::RETCODE) -> super::SQLRETURN);
    unsafe { SQLCompleteAsync(handletype, handle, asyncretcodeptr as _) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLConnect(connectionhandle: super::SQLHDBC, servername: &[super::SQLCHAR], username: &[super::SQLCHAR], authentication: &[super::SQLCHAR]) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLConnect(connectionhandle : super::SQLHDBC, servername : *const super::SQLCHAR, namelength1 : super::SQLSMALLINT, username : *const super::SQLCHAR, namelength2 : super::SQLSMALLINT, authentication : *const super::SQLCHAR, namelength3 : super::SQLSMALLINT) -> super::SQLRETURN);
    unsafe { SQLConnect(connectionhandle, servername.as_ptr(), super::SQLSMALLINT(servername.len().try_into().unwrap()), username.as_ptr(), super::SQLSMALLINT(username.len().try_into().unwrap()), authentication.as_ptr(), super::SQLSMALLINT(authentication.len().try_into().unwrap())) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLCopyDesc(sourcedeschandle: super::SQLHDESC, targetdeschandle: super::SQLHDESC) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLCopyDesc(sourcedeschandle : super::SQLHDESC, targetdeschandle : super::SQLHDESC) -> super::SQLRETURN);
    unsafe { SQLCopyDesc(sourcedeschandle, targetdeschandle) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLDataSources(environmenthandle: super::SQLHENV, direction: super::SQLUSMALLINT, servername: Option<&mut [super::SQLCHAR]>, namelength1ptr: Option<*mut super::SQLSMALLINT>, description: Option<&mut [super::SQLCHAR]>, namelength2ptr: Option<*mut super::SQLSMALLINT>) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLDataSources(environmenthandle : super::SQLHENV, direction : super::SQLUSMALLINT, servername : *mut super::SQLCHAR, bufferlength1 : super::SQLSMALLINT, namelength1ptr : *mut super::SQLSMALLINT, description : *mut super::SQLCHAR, bufferlength2 : super::SQLSMALLINT, namelength2ptr : *mut super::SQLSMALLINT) -> super::SQLRETURN);
    unsafe {
        SQLDataSources(
            environmenthandle,
            direction,
            servername.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut()),
            servername.as_deref().map_or(super::SQLSMALLINT(0), |slice| super::SQLSMALLINT(slice.len().try_into().unwrap())),
            namelength1ptr.unwrap_or(core::mem::zeroed()) as _,
            description.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut()),
            description.as_deref().map_or(super::SQLSMALLINT(0), |slice| super::SQLSMALLINT(slice.len().try_into().unwrap())),
            namelength2ptr.unwrap_or(core::mem::zeroed()) as _,
        )
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLDescribeCol(statementhandle: super::SQLHSTMT, columnnumber: super::SQLUSMALLINT, columnname: Option<&mut [super::SQLCHAR]>, namelength: Option<*mut super::SQLSMALLINT>, datatype: Option<*mut super::SQLSMALLINT>, columnsize: Option<*mut super::SQLUINTEGER>, decimaldigits: Option<*mut super::SQLSMALLINT>, nullable: Option<*mut super::SQLSMALLINT>) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLDescribeCol(statementhandle : super::SQLHSTMT, columnnumber : super::SQLUSMALLINT, columnname : *mut super::SQLCHAR, bufferlength : super::SQLSMALLINT, namelength : *mut super::SQLSMALLINT, datatype : *mut super::SQLSMALLINT, columnsize : *mut super::SQLUINTEGER, decimaldigits : *mut super::SQLSMALLINT, nullable : *mut super::SQLSMALLINT) -> super::SQLRETURN);
    unsafe { SQLDescribeCol(statementhandle, columnnumber, columnname.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut()), columnname.as_deref().map_or(super::SQLSMALLINT(0), |slice| super::SQLSMALLINT(slice.len().try_into().unwrap())), namelength.unwrap_or(core::mem::zeroed()) as _, datatype.unwrap_or(core::mem::zeroed()) as _, columnsize.unwrap_or(core::mem::zeroed()) as _, decimaldigits.unwrap_or(core::mem::zeroed()) as _, nullable.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLDescribeCol(statementhandle: super::SQLHSTMT, columnnumber: super::SQLUSMALLINT, columnname: Option<&mut [super::SQLCHAR]>, namelength: Option<*mut super::SQLSMALLINT>, datatype: Option<*mut super::SQLSMALLINT>, columnsize: Option<*mut super::SQLULEN>, decimaldigits: Option<*mut super::SQLSMALLINT>, nullable: Option<*mut super::SQLSMALLINT>) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLDescribeCol(statementhandle : super::SQLHSTMT, columnnumber : super::SQLUSMALLINT, columnname : *mut super::SQLCHAR, bufferlength : super::SQLSMALLINT, namelength : *mut super::SQLSMALLINT, datatype : *mut super::SQLSMALLINT, columnsize : *mut super::SQLULEN, decimaldigits : *mut super::SQLSMALLINT, nullable : *mut super::SQLSMALLINT) -> super::SQLRETURN);
    unsafe { SQLDescribeCol(statementhandle, columnnumber, columnname.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut()), columnname.as_deref().map_or(super::SQLSMALLINT(0), |slice| super::SQLSMALLINT(slice.len().try_into().unwrap())), namelength.unwrap_or(core::mem::zeroed()) as _, datatype.unwrap_or(core::mem::zeroed()) as _, columnsize.unwrap_or(core::mem::zeroed()) as _, decimaldigits.unwrap_or(core::mem::zeroed()) as _, nullable.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLDisconnect(connectionhandle: super::SQLHDBC) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLDisconnect(connectionhandle : super::SQLHDBC) -> super::SQLRETURN);
    unsafe { SQLDisconnect(connectionhandle) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLEndTran(handletype: super::SQLSMALLINT, handle: super::SQLHANDLE, completiontype: super::SQLSMALLINT) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLEndTran(handletype : super::SQLSMALLINT, handle : super::SQLHANDLE, completiontype : super::SQLSMALLINT) -> super::SQLRETURN);
    unsafe { SQLEndTran(handletype, handle, completiontype) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLError(environmenthandle: super::SQLHENV, connectionhandle: super::SQLHDBC, statementhandle: super::SQLHSTMT, sqlstate: &mut [super::SQLCHAR; 6], nativeerror: Option<*mut super::SQLINTEGER>, messagetext: Option<&mut [super::SQLCHAR]>, textlength: Option<*mut super::SQLSMALLINT>) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLError(environmenthandle : super::SQLHENV, connectionhandle : super::SQLHDBC, statementhandle : super::SQLHSTMT, sqlstate : *mut super::SQLCHAR, nativeerror : *mut super::SQLINTEGER, messagetext : *mut super::SQLCHAR, bufferlength : super::SQLSMALLINT, textlength : *mut super::SQLSMALLINT) -> super::SQLRETURN);
    unsafe { SQLError(environmenthandle, connectionhandle, statementhandle, sqlstate.as_mut_ptr(), nativeerror.unwrap_or(core::mem::zeroed()) as _, messagetext.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut()), messagetext.as_deref().map_or(super::SQLSMALLINT(0), |slice| super::SQLSMALLINT(slice.len().try_into().unwrap())), textlength.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLExecDirect(statementhandle: super::SQLHSTMT, statementtext: Option<&[super::SQLCHAR]>) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLExecDirect(statementhandle : super::SQLHSTMT, statementtext : *const super::SQLCHAR, textlength : super::SQLINTEGER) -> super::SQLRETURN);
    unsafe { SQLExecDirect(statementhandle, statementtext.map_or(core::ptr::null(), |slice| slice.as_ptr()), statementtext.map_or(super::SQLINTEGER(0), |slice| super::SQLINTEGER(slice.len().try_into().unwrap()))) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLExecute(statementhandle: super::SQLHSTMT) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLExecute(statementhandle : super::SQLHSTMT) -> super::SQLRETURN);
    unsafe { SQLExecute(statementhandle) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLFetch(statementhandle: super::SQLHSTMT) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLFetch(statementhandle : super::SQLHSTMT) -> super::SQLRETURN);
    unsafe { SQLFetch(statementhandle) }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLFetchScroll(statementhandle: super::SQLHSTMT, fetchorientation: super::SQLSMALLINT, fetchoffset: super::SQLINTEGER) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLFetchScroll(statementhandle : super::SQLHSTMT, fetchorientation : super::SQLSMALLINT, fetchoffset : super::SQLINTEGER) -> super::SQLRETURN);
    unsafe { SQLFetchScroll(statementhandle, fetchorientation, fetchoffset) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLFetchScroll(statementhandle: super::SQLHSTMT, fetchorientation: super::SQLSMALLINT, fetchoffset: super::SQLLEN) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLFetchScroll(statementhandle : super::SQLHSTMT, fetchorientation : super::SQLSMALLINT, fetchoffset : super::SQLLEN) -> super::SQLRETURN);
    unsafe { SQLFetchScroll(statementhandle, fetchorientation, fetchoffset) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLFreeConnect(connectionhandle: super::SQLHDBC) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLFreeConnect(connectionhandle : super::SQLHDBC) -> super::SQLRETURN);
    unsafe { SQLFreeConnect(connectionhandle) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLFreeEnv(environmenthandle: super::SQLHENV) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLFreeEnv(environmenthandle : super::SQLHENV) -> super::SQLRETURN);
    unsafe { SQLFreeEnv(environmenthandle) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLFreeHandle(handletype: super::SQLSMALLINT, handle: super::SQLHANDLE) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLFreeHandle(handletype : super::SQLSMALLINT, handle : super::SQLHANDLE) -> super::SQLRETURN);
    unsafe { SQLFreeHandle(handletype, handle) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLFreeStmt(statementhandle: super::SQLHSTMT, option: super::SQLUSMALLINT) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLFreeStmt(statementhandle : super::SQLHSTMT, option : super::SQLUSMALLINT) -> super::SQLRETURN);
    unsafe { SQLFreeStmt(statementhandle, option) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLGetConnectAttr(connectionhandle: super::SQLHDBC, attribute: super::SQLINTEGER, value: Option<super::SQLPOINTER>, bufferlength: super::SQLINTEGER, stringlengthptr: Option<*mut super::SQLINTEGER>) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLGetConnectAttr(connectionhandle : super::SQLHDBC, attribute : super::SQLINTEGER, value : super::SQLPOINTER, bufferlength : super::SQLINTEGER, stringlengthptr : *mut super::SQLINTEGER) -> super::SQLRETURN);
    unsafe { SQLGetConnectAttr(connectionhandle, attribute, value.unwrap_or(core::mem::zeroed()) as _, bufferlength, stringlengthptr.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLGetConnectOption(connectionhandle: super::SQLHDBC, option: super::SQLUSMALLINT, value: super::SQLPOINTER) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLGetConnectOption(connectionhandle : super::SQLHDBC, option : super::SQLUSMALLINT, value : super::SQLPOINTER) -> super::SQLRETURN);
    unsafe { SQLGetConnectOption(connectionhandle, option, value) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLGetCursorName(statementhandle: super::SQLHSTMT, cursorname: Option<&mut [super::SQLCHAR]>, namelengthptr: Option<*mut super::SQLSMALLINT>) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLGetCursorName(statementhandle : super::SQLHSTMT, cursorname : *mut super::SQLCHAR, bufferlength : super::SQLSMALLINT, namelengthptr : *mut super::SQLSMALLINT) -> super::SQLRETURN);
    unsafe { SQLGetCursorName(statementhandle, cursorname.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut()), cursorname.as_deref().map_or(super::SQLSMALLINT(0), |slice| super::SQLSMALLINT(slice.len().try_into().unwrap())), namelengthptr.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLGetData(statementhandle: super::SQLHSTMT, columnnumber: super::SQLUSMALLINT, targettype: super::SQLSMALLINT, targetvalue: Option<super::SQLPOINTER>, bufferlength: super::SQLINTEGER, strlen_or_indptr: Option<*mut super::SQLINTEGER>) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLGetData(statementhandle : super::SQLHSTMT, columnnumber : super::SQLUSMALLINT, targettype : super::SQLSMALLINT, targetvalue : super::SQLPOINTER, bufferlength : super::SQLINTEGER, strlen_or_indptr : *mut super::SQLINTEGER) -> super::SQLRETURN);
    unsafe { SQLGetData(statementhandle, columnnumber, targettype, targetvalue.unwrap_or(core::mem::zeroed()) as _, bufferlength, strlen_or_indptr.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLGetData(statementhandle: super::SQLHSTMT, columnnumber: super::SQLUSMALLINT, targettype: super::SQLSMALLINT, targetvalue: Option<super::SQLPOINTER>, bufferlength: super::SQLLEN, strlen_or_indptr: Option<*mut super::SQLLEN>) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLGetData(statementhandle : super::SQLHSTMT, columnnumber : super::SQLUSMALLINT, targettype : super::SQLSMALLINT, targetvalue : super::SQLPOINTER, bufferlength : super::SQLLEN, strlen_or_indptr : *mut super::SQLLEN) -> super::SQLRETURN);
    unsafe { SQLGetData(statementhandle, columnnumber, targettype, targetvalue.unwrap_or(core::mem::zeroed()) as _, bufferlength, strlen_or_indptr.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLGetDescField(descriptorhandle: super::SQLHDESC, recnumber: super::SQLSMALLINT, fieldidentifier: super::SQLSMALLINT, value: Option<super::SQLPOINTER>, bufferlength: super::SQLINTEGER, stringlength: Option<*mut super::SQLINTEGER>) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLGetDescField(descriptorhandle : super::SQLHDESC, recnumber : super::SQLSMALLINT, fieldidentifier : super::SQLSMALLINT, value : super::SQLPOINTER, bufferlength : super::SQLINTEGER, stringlength : *mut super::SQLINTEGER) -> super::SQLRETURN);
    unsafe { SQLGetDescField(descriptorhandle, recnumber, fieldidentifier, value.unwrap_or(core::mem::zeroed()) as _, bufferlength, stringlength.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLGetDescRec(descriptorhandle: super::SQLHDESC, recnumber: super::SQLSMALLINT, name: Option<&mut [super::SQLCHAR]>, stringlengthptr: Option<*mut super::SQLSMALLINT>, typeptr: Option<*mut super::SQLSMALLINT>, subtypeptr: Option<*mut super::SQLSMALLINT>, lengthptr: Option<*mut super::SQLINTEGER>, precisionptr: Option<*mut super::SQLSMALLINT>, scaleptr: Option<*mut super::SQLSMALLINT>, nullableptr: Option<*mut super::SQLSMALLINT>) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLGetDescRec(descriptorhandle : super::SQLHDESC, recnumber : super::SQLSMALLINT, name : *mut super::SQLCHAR, bufferlength : super::SQLSMALLINT, stringlengthptr : *mut super::SQLSMALLINT, typeptr : *mut super::SQLSMALLINT, subtypeptr : *mut super::SQLSMALLINT, lengthptr : *mut super::SQLINTEGER, precisionptr : *mut super::SQLSMALLINT, scaleptr : *mut super::SQLSMALLINT, nullableptr : *mut super::SQLSMALLINT) -> super::SQLRETURN);
    unsafe {
        SQLGetDescRec(
            descriptorhandle,
            recnumber,
            name.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut()),
            name.as_deref().map_or(super::SQLSMALLINT(0), |slice| super::SQLSMALLINT(slice.len().try_into().unwrap())),
            stringlengthptr.unwrap_or(core::mem::zeroed()) as _,
            typeptr.unwrap_or(core::mem::zeroed()) as _,
            subtypeptr.unwrap_or(core::mem::zeroed()) as _,
            lengthptr.unwrap_or(core::mem::zeroed()) as _,
            precisionptr.unwrap_or(core::mem::zeroed()) as _,
            scaleptr.unwrap_or(core::mem::zeroed()) as _,
            nullableptr.unwrap_or(core::mem::zeroed()) as _,
        )
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLGetDescRec(descriptorhandle: super::SQLHDESC, recnumber: super::SQLSMALLINT, name: Option<&mut [super::SQLCHAR]>, stringlengthptr: Option<*mut super::SQLSMALLINT>, typeptr: Option<*mut super::SQLSMALLINT>, subtypeptr: Option<*mut super::SQLSMALLINT>, lengthptr: Option<*mut super::SQLLEN>, precisionptr: Option<*mut super::SQLSMALLINT>, scaleptr: Option<*mut super::SQLSMALLINT>, nullableptr: Option<*mut super::SQLSMALLINT>) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLGetDescRec(descriptorhandle : super::SQLHDESC, recnumber : super::SQLSMALLINT, name : *mut super::SQLCHAR, bufferlength : super::SQLSMALLINT, stringlengthptr : *mut super::SQLSMALLINT, typeptr : *mut super::SQLSMALLINT, subtypeptr : *mut super::SQLSMALLINT, lengthptr : *mut super::SQLLEN, precisionptr : *mut super::SQLSMALLINT, scaleptr : *mut super::SQLSMALLINT, nullableptr : *mut super::SQLSMALLINT) -> super::SQLRETURN);
    unsafe {
        SQLGetDescRec(
            descriptorhandle,
            recnumber,
            name.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut()),
            name.as_deref().map_or(super::SQLSMALLINT(0), |slice| super::SQLSMALLINT(slice.len().try_into().unwrap())),
            stringlengthptr.unwrap_or(core::mem::zeroed()) as _,
            typeptr.unwrap_or(core::mem::zeroed()) as _,
            subtypeptr.unwrap_or(core::mem::zeroed()) as _,
            lengthptr.unwrap_or(core::mem::zeroed()) as _,
            precisionptr.unwrap_or(core::mem::zeroed()) as _,
            scaleptr.unwrap_or(core::mem::zeroed()) as _,
            nullableptr.unwrap_or(core::mem::zeroed()) as _,
        )
    }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLGetDiagField(handletype: super::SQLSMALLINT, handle: super::SQLHANDLE, recnumber: super::SQLSMALLINT, diagidentifier: super::SQLSMALLINT, diaginfo: Option<super::SQLPOINTER>, bufferlength: super::SQLSMALLINT, stringlength: Option<*mut super::SQLSMALLINT>) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLGetDiagField(handletype : super::SQLSMALLINT, handle : super::SQLHANDLE, recnumber : super::SQLSMALLINT, diagidentifier : super::SQLSMALLINT, diaginfo : super::SQLPOINTER, bufferlength : super::SQLSMALLINT, stringlength : *mut super::SQLSMALLINT) -> super::SQLRETURN);
    unsafe { SQLGetDiagField(handletype, handle, recnumber, diagidentifier, diaginfo.unwrap_or(core::mem::zeroed()) as _, bufferlength, stringlength.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLGetDiagRec(handletype: super::SQLSMALLINT, handle: super::SQLHANDLE, recnumber: super::SQLSMALLINT, sqlstate: Option<&mut [super::SQLCHAR; 6]>, nativeerror: *mut super::SQLINTEGER, messagetext: Option<&mut [super::SQLCHAR]>, textlength: Option<*mut super::SQLSMALLINT>) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLGetDiagRec(handletype : super::SQLSMALLINT, handle : super::SQLHANDLE, recnumber : super::SQLSMALLINT, sqlstate : *mut super::SQLCHAR, nativeerror : *mut super::SQLINTEGER, messagetext : *mut super::SQLCHAR, bufferlength : super::SQLSMALLINT, textlength : *mut super::SQLSMALLINT) -> super::SQLRETURN);
    unsafe { SQLGetDiagRec(handletype, handle, recnumber, sqlstate.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut()), nativeerror as _, messagetext.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut()), messagetext.as_deref().map_or(super::SQLSMALLINT(0), |slice| super::SQLSMALLINT(slice.len().try_into().unwrap())), textlength.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLGetEnvAttr(environmenthandle: super::SQLHENV, attribute: super::SQLINTEGER, value: super::SQLPOINTER, bufferlength: super::SQLINTEGER, stringlength: Option<*mut super::SQLINTEGER>) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLGetEnvAttr(environmenthandle : super::SQLHENV, attribute : super::SQLINTEGER, value : super::SQLPOINTER, bufferlength : super::SQLINTEGER, stringlength : *mut super::SQLINTEGER) -> super::SQLRETURN);
    unsafe { SQLGetEnvAttr(environmenthandle, attribute, value as _, bufferlength, stringlength.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLGetFunctions(connectionhandle: super::SQLHDBC, functionid: super::SQLUSMALLINT, supported: Option<*mut super::SQLUSMALLINT>) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLGetFunctions(connectionhandle : super::SQLHDBC, functionid : super::SQLUSMALLINT, supported : *mut super::SQLUSMALLINT) -> super::SQLRETURN);
    unsafe { SQLGetFunctions(connectionhandle, functionid, supported.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLGetInfo(connectionhandle: super::SQLHDBC, infotype: super::SQLUSMALLINT, infovalue: Option<super::SQLPOINTER>, bufferlength: super::SQLSMALLINT, stringlengthptr: Option<*mut super::SQLSMALLINT>) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLGetInfo(connectionhandle : super::SQLHDBC, infotype : super::SQLUSMALLINT, infovalue : super::SQLPOINTER, bufferlength : super::SQLSMALLINT, stringlengthptr : *mut super::SQLSMALLINT) -> super::SQLRETURN);
    unsafe { SQLGetInfo(connectionhandle, infotype, infovalue.unwrap_or(core::mem::zeroed()) as _, bufferlength, stringlengthptr.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLGetStmtAttr(statementhandle: super::SQLHSTMT, attribute: super::SQLINTEGER, value: Option<super::SQLPOINTER>, bufferlength: super::SQLINTEGER, stringlength: Option<*mut super::SQLINTEGER>) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLGetStmtAttr(statementhandle : super::SQLHSTMT, attribute : super::SQLINTEGER, value : super::SQLPOINTER, bufferlength : super::SQLINTEGER, stringlength : *mut super::SQLINTEGER) -> super::SQLRETURN);
    unsafe { SQLGetStmtAttr(statementhandle, attribute, value.unwrap_or(core::mem::zeroed()) as _, bufferlength, stringlength.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLGetStmtOption(statementhandle: super::SQLHSTMT, option: super::SQLUSMALLINT, value: super::SQLPOINTER) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLGetStmtOption(statementhandle : super::SQLHSTMT, option : super::SQLUSMALLINT, value : super::SQLPOINTER) -> super::SQLRETURN);
    unsafe { SQLGetStmtOption(statementhandle, option, value) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLGetTypeInfo(statementhandle: super::SQLHSTMT, datatype: super::SQLSMALLINT) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLGetTypeInfo(statementhandle : super::SQLHSTMT, datatype : super::SQLSMALLINT) -> super::SQLRETURN);
    unsafe { SQLGetTypeInfo(statementhandle, datatype) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLNumResultCols(statementhandle: super::SQLHSTMT, columncount: *mut super::SQLSMALLINT) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLNumResultCols(statementhandle : super::SQLHSTMT, columncount : *mut super::SQLSMALLINT) -> super::SQLRETURN);
    unsafe { SQLNumResultCols(statementhandle, columncount as _) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLParamData(statementhandle: super::SQLHSTMT, value: Option<*mut super::SQLPOINTER>) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLParamData(statementhandle : super::SQLHSTMT, value : *mut super::SQLPOINTER) -> super::SQLRETURN);
    unsafe { SQLParamData(statementhandle, value.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLPrepare(statementhandle: super::SQLHSTMT, statementtext: &[super::SQLCHAR]) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLPrepare(statementhandle : super::SQLHSTMT, statementtext : *const super::SQLCHAR, textlength : super::SQLINTEGER) -> super::SQLRETURN);
    unsafe { SQLPrepare(statementhandle, statementtext.as_ptr(), super::SQLINTEGER(statementtext.len().try_into().unwrap())) }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLPutData(statementhandle: super::SQLHSTMT, data: super::SQLPOINTER, strlen_or_ind: super::SQLINTEGER) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLPutData(statementhandle : super::SQLHSTMT, data : super::SQLPOINTER, strlen_or_ind : super::SQLINTEGER) -> super::SQLRETURN);
    unsafe { SQLPutData(statementhandle, data, strlen_or_ind) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLPutData(statementhandle: super::SQLHSTMT, data: super::SQLPOINTER, strlen_or_ind: super::SQLLEN) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLPutData(statementhandle : super::SQLHSTMT, data : super::SQLPOINTER, strlen_or_ind : super::SQLLEN) -> super::SQLRETURN);
    unsafe { SQLPutData(statementhandle, data, strlen_or_ind) }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLRowCount(statementhandle: super::SQLHSTMT, rowcount: *mut super::SQLINTEGER) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLRowCount(statementhandle : super::SQLHSTMT, rowcount : *mut super::SQLINTEGER) -> super::SQLRETURN);
    unsafe { SQLRowCount(statementhandle, rowcount as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLRowCount(statementhandle: super::SQLHSTMT, rowcount: *mut super::SQLLEN) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLRowCount(statementhandle : super::SQLHSTMT, rowcount : *mut super::SQLLEN) -> super::SQLRETURN);
    unsafe { SQLRowCount(statementhandle, rowcount as _) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLSetConnectAttr(connectionhandle: super::SQLHDBC, attribute: super::SQLINTEGER, value: Option<super::SQLPOINTER>, stringlength: super::SQLINTEGER) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLSetConnectAttr(connectionhandle : super::SQLHDBC, attribute : super::SQLINTEGER, value : super::SQLPOINTER, stringlength : super::SQLINTEGER) -> super::SQLRETURN);
    unsafe { SQLSetConnectAttr(connectionhandle, attribute, value.unwrap_or(core::mem::zeroed()) as _, stringlength) }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLSetConnectOption(connectionhandle: super::SQLHDBC, option: super::SQLUSMALLINT, value: super::SQLUINTEGER) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLSetConnectOption(connectionhandle : super::SQLHDBC, option : super::SQLUSMALLINT, value : super::SQLUINTEGER) -> super::SQLRETURN);
    unsafe { SQLSetConnectOption(connectionhandle, option, value) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLSetConnectOption(connectionhandle: super::SQLHDBC, option: super::SQLUSMALLINT, value: super::SQLULEN) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLSetConnectOption(connectionhandle : super::SQLHDBC, option : super::SQLUSMALLINT, value : super::SQLULEN) -> super::SQLRETURN);
    unsafe { SQLSetConnectOption(connectionhandle, option, value) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLSetCursorName(statementhandle: super::SQLHSTMT, cursorname: &[super::SQLCHAR]) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLSetCursorName(statementhandle : super::SQLHSTMT, cursorname : *const super::SQLCHAR, namelength : super::SQLSMALLINT) -> super::SQLRETURN);
    unsafe { SQLSetCursorName(statementhandle, cursorname.as_ptr(), super::SQLSMALLINT(cursorname.len().try_into().unwrap())) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLSetDescField(descriptorhandle: super::SQLHDESC, recnumber: super::SQLSMALLINT, fieldidentifier: super::SQLSMALLINT, value: super::SQLPOINTER, bufferlength: super::SQLINTEGER) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLSetDescField(descriptorhandle : super::SQLHDESC, recnumber : super::SQLSMALLINT, fieldidentifier : super::SQLSMALLINT, value : super::SQLPOINTER, bufferlength : super::SQLINTEGER) -> super::SQLRETURN);
    unsafe { SQLSetDescField(descriptorhandle, recnumber, fieldidentifier, value, bufferlength) }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLSetDescRec(descriptorhandle: super::SQLHDESC, recnumber: super::SQLSMALLINT, r#type: super::SQLSMALLINT, subtype: super::SQLSMALLINT, length: super::SQLINTEGER, precision: super::SQLSMALLINT, scale: super::SQLSMALLINT, data: Option<super::SQLPOINTER>, stringlength: Option<*mut super::SQLINTEGER>, indicator: Option<*mut super::SQLINTEGER>) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLSetDescRec(descriptorhandle : super::SQLHDESC, recnumber : super::SQLSMALLINT, r#type : super::SQLSMALLINT, subtype : super::SQLSMALLINT, length : super::SQLINTEGER, precision : super::SQLSMALLINT, scale : super::SQLSMALLINT, data : super::SQLPOINTER, stringlength : *mut super::SQLINTEGER, indicator : *mut super::SQLINTEGER) -> super::SQLRETURN);
    unsafe { SQLSetDescRec(descriptorhandle, recnumber, r#type, subtype, length, precision, scale, data.unwrap_or(core::mem::zeroed()) as _, stringlength.unwrap_or(core::mem::zeroed()) as _, indicator.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLSetDescRec(descriptorhandle: super::SQLHDESC, recnumber: super::SQLSMALLINT, r#type: super::SQLSMALLINT, subtype: super::SQLSMALLINT, length: super::SQLLEN, precision: super::SQLSMALLINT, scale: super::SQLSMALLINT, data: Option<super::SQLPOINTER>, stringlength: Option<*mut super::SQLLEN>, indicator: Option<*mut super::SQLLEN>) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLSetDescRec(descriptorhandle : super::SQLHDESC, recnumber : super::SQLSMALLINT, r#type : super::SQLSMALLINT, subtype : super::SQLSMALLINT, length : super::SQLLEN, precision : super::SQLSMALLINT, scale : super::SQLSMALLINT, data : super::SQLPOINTER, stringlength : *mut super::SQLLEN, indicator : *mut super::SQLLEN) -> super::SQLRETURN);
    unsafe { SQLSetDescRec(descriptorhandle, recnumber, r#type, subtype, length, precision, scale, data.unwrap_or(core::mem::zeroed()) as _, stringlength.unwrap_or(core::mem::zeroed()) as _, indicator.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLSetEnvAttr(environmenthandle: super::SQLHENV, attribute: super::SQLINTEGER, value: Option<super::SQLPOINTER>, stringlength: super::SQLINTEGER) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLSetEnvAttr(environmenthandle : super::SQLHENV, attribute : super::SQLINTEGER, value : super::SQLPOINTER, stringlength : super::SQLINTEGER) -> super::SQLRETURN);
    unsafe { SQLSetEnvAttr(environmenthandle, attribute, value.unwrap_or(core::mem::zeroed()) as _, stringlength) }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLSetParam(statementhandle: super::SQLHSTMT, parameternumber: super::SQLUSMALLINT, valuetype: super::SQLSMALLINT, parametertype: super::SQLSMALLINT, lengthprecision: super::SQLUINTEGER, parameterscale: super::SQLSMALLINT, parametervalue: super::SQLPOINTER, strlen_or_ind: *mut super::SQLINTEGER) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLSetParam(statementhandle : super::SQLHSTMT, parameternumber : super::SQLUSMALLINT, valuetype : super::SQLSMALLINT, parametertype : super::SQLSMALLINT, lengthprecision : super::SQLUINTEGER, parameterscale : super::SQLSMALLINT, parametervalue : super::SQLPOINTER, strlen_or_ind : *mut super::SQLINTEGER) -> super::SQLRETURN);
    unsafe { SQLSetParam(statementhandle, parameternumber, valuetype, parametertype, lengthprecision, parameterscale, parametervalue, strlen_or_ind as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLSetParam(statementhandle: super::SQLHSTMT, parameternumber: super::SQLUSMALLINT, valuetype: super::SQLSMALLINT, parametertype: super::SQLSMALLINT, lengthprecision: super::SQLULEN, parameterscale: super::SQLSMALLINT, parametervalue: super::SQLPOINTER, strlen_or_ind: *mut super::SQLLEN) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLSetParam(statementhandle : super::SQLHSTMT, parameternumber : super::SQLUSMALLINT, valuetype : super::SQLSMALLINT, parametertype : super::SQLSMALLINT, lengthprecision : super::SQLULEN, parameterscale : super::SQLSMALLINT, parametervalue : super::SQLPOINTER, strlen_or_ind : *mut super::SQLLEN) -> super::SQLRETURN);
    unsafe { SQLSetParam(statementhandle, parameternumber, valuetype, parametertype, lengthprecision, parameterscale, parametervalue, strlen_or_ind as _) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLSetStmtAttr(statementhandle: super::SQLHSTMT, attribute: super::SQLINTEGER, value: super::SQLPOINTER, stringlength: super::SQLINTEGER) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLSetStmtAttr(statementhandle : super::SQLHSTMT, attribute : super::SQLINTEGER, value : super::SQLPOINTER, stringlength : super::SQLINTEGER) -> super::SQLRETURN);
    unsafe { SQLSetStmtAttr(statementhandle, attribute, value, stringlength) }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLSetStmtOption(statementhandle: super::SQLHSTMT, option: super::SQLUSMALLINT, value: super::SQLUINTEGER) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLSetStmtOption(statementhandle : super::SQLHSTMT, option : super::SQLUSMALLINT, value : super::SQLUINTEGER) -> super::SQLRETURN);
    unsafe { SQLSetStmtOption(statementhandle, option, value) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLSetStmtOption(statementhandle: super::SQLHSTMT, option: super::SQLUSMALLINT, value: super::SQLULEN) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLSetStmtOption(statementhandle : super::SQLHSTMT, option : super::SQLUSMALLINT, value : super::SQLULEN) -> super::SQLRETURN);
    unsafe { SQLSetStmtOption(statementhandle, option, value) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLSpecialColumns(statementhandle: super::SQLHSTMT, identifiertype: super::SQLUSMALLINT, catalogname: Option<&[super::SQLCHAR]>, schemaname: Option<&[super::SQLCHAR]>, tablename: Option<&[super::SQLCHAR]>, scope: super::SQLUSMALLINT, nullable: super::SQLUSMALLINT) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLSpecialColumns(statementhandle : super::SQLHSTMT, identifiertype : super::SQLUSMALLINT, catalogname : *const super::SQLCHAR, namelength1 : super::SQLSMALLINT, schemaname : *const super::SQLCHAR, namelength2 : super::SQLSMALLINT, tablename : *const super::SQLCHAR, namelength3 : super::SQLSMALLINT, scope : super::SQLUSMALLINT, nullable : super::SQLUSMALLINT) -> super::SQLRETURN);
    unsafe {
        SQLSpecialColumns(
            statementhandle,
            identifiertype,
            catalogname.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            catalogname.map_or(super::SQLSMALLINT(0), |slice| super::SQLSMALLINT(slice.len().try_into().unwrap())),
            schemaname.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            schemaname.map_or(super::SQLSMALLINT(0), |slice| super::SQLSMALLINT(slice.len().try_into().unwrap())),
            tablename.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            tablename.map_or(super::SQLSMALLINT(0), |slice| super::SQLSMALLINT(slice.len().try_into().unwrap())),
            scope,
            nullable,
        )
    }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLStatistics(statementhandle: super::SQLHSTMT, catalogname: Option<&[super::SQLCHAR]>, schemaname: Option<&[super::SQLCHAR]>, tablename: Option<&[super::SQLCHAR]>, unique: super::SQLUSMALLINT, reserved: super::SQLUSMALLINT) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLStatistics(statementhandle : super::SQLHSTMT, catalogname : *const super::SQLCHAR, namelength1 : super::SQLSMALLINT, schemaname : *const super::SQLCHAR, namelength2 : super::SQLSMALLINT, tablename : *const super::SQLCHAR, namelength3 : super::SQLSMALLINT, unique : super::SQLUSMALLINT, reserved : super::SQLUSMALLINT) -> super::SQLRETURN);
    unsafe {
        SQLStatistics(
            statementhandle,
            catalogname.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            catalogname.map_or(super::SQLSMALLINT(0), |slice| super::SQLSMALLINT(slice.len().try_into().unwrap())),
            schemaname.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            schemaname.map_or(super::SQLSMALLINT(0), |slice| super::SQLSMALLINT(slice.len().try_into().unwrap())),
            tablename.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            tablename.map_or(super::SQLSMALLINT(0), |slice| super::SQLSMALLINT(slice.len().try_into().unwrap())),
            unique,
            reserved,
        )
    }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLTables(statementhandle: super::SQLHSTMT, catalogname: Option<&[super::SQLCHAR]>, schemaname: Option<&[super::SQLCHAR]>, tablename: Option<&[super::SQLCHAR]>, tabletype: Option<&[super::SQLCHAR]>) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLTables(statementhandle : super::SQLHSTMT, catalogname : *const super::SQLCHAR, namelength1 : super::SQLSMALLINT, schemaname : *const super::SQLCHAR, namelength2 : super::SQLSMALLINT, tablename : *const super::SQLCHAR, namelength3 : super::SQLSMALLINT, tabletype : *const super::SQLCHAR, namelength4 : super::SQLSMALLINT) -> super::SQLRETURN);
    unsafe {
        SQLTables(
            statementhandle,
            catalogname.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            catalogname.map_or(super::SQLSMALLINT(0), |slice| super::SQLSMALLINT(slice.len().try_into().unwrap())),
            schemaname.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            schemaname.map_or(super::SQLSMALLINT(0), |slice| super::SQLSMALLINT(slice.len().try_into().unwrap())),
            tablename.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            tablename.map_or(super::SQLSMALLINT(0), |slice| super::SQLSMALLINT(slice.len().try_into().unwrap())),
            tabletype.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            tabletype.map_or(super::SQLSMALLINT(0), |slice| super::SQLSMALLINT(slice.len().try_into().unwrap())),
        )
    }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLTransact(environmenthandle: super::SQLHENV, connectionhandle: super::SQLHDBC, completiontype: super::SQLUSMALLINT) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLTransact(environmenthandle : super::SQLHENV, connectionhandle : super::SQLHDBC, completiontype : super::SQLUSMALLINT) -> super::SQLRETURN);
    unsafe { SQLTransact(environmenthandle, connectionhandle, completiontype) }
}
pub const ODBCVER: u32 = 896;
pub const SQL_ACCESSIBLE_PROCEDURES: u32 = 20;
pub const SQL_ACCESSIBLE_TABLES: u32 = 19;
pub const SQL_ALL_TYPES: u32 = 0;
pub const SQL_ALTER_TABLE: u32 = 86;
pub const SQL_AM_CONNECTION: u32 = 1;
pub const SQL_AM_NONE: u32 = 0;
pub const SQL_AM_STATEMENT: u32 = 2;
pub const SQL_APD_TYPE: i32 = -100;
pub const SQL_API_SQLALLOCCONNECT: u32 = 1;
pub const SQL_API_SQLALLOCENV: u32 = 2;
pub const SQL_API_SQLALLOCHANDLE: u32 = 1001;
pub const SQL_API_SQLALLOCSTMT: u32 = 3;
pub const SQL_API_SQLBINDCOL: u32 = 4;
pub const SQL_API_SQLBINDPARAM: u32 = 1002;
pub const SQL_API_SQLCANCEL: u32 = 5;
pub const SQL_API_SQLCANCELHANDLE: u32 = 1550;
pub const SQL_API_SQLCLOSECURSOR: u32 = 1003;
pub const SQL_API_SQLCOLATTRIBUTE: u32 = 6;
pub const SQL_API_SQLCOLUMNS: u32 = 40;
pub const SQL_API_SQLCOMPLETEASYNC: u32 = 1551;
pub const SQL_API_SQLCONNECT: u32 = 7;
pub const SQL_API_SQLCOPYDESC: u32 = 1004;
pub const SQL_API_SQLDATASOURCES: u32 = 57;
pub const SQL_API_SQLDESCRIBECOL: u32 = 8;
pub const SQL_API_SQLDISCONNECT: u32 = 9;
pub const SQL_API_SQLENDTRAN: u32 = 1005;
pub const SQL_API_SQLERROR: u32 = 10;
pub const SQL_API_SQLEXECDIRECT: u32 = 11;
pub const SQL_API_SQLEXECUTE: u32 = 12;
pub const SQL_API_SQLFETCH: u32 = 13;
pub const SQL_API_SQLFETCHSCROLL: u32 = 1021;
pub const SQL_API_SQLFREECONNECT: u32 = 14;
pub const SQL_API_SQLFREEENV: u32 = 15;
pub const SQL_API_SQLFREEHANDLE: u32 = 1006;
pub const SQL_API_SQLFREESTMT: u32 = 16;
pub const SQL_API_SQLGETCONNECTATTR: u32 = 1007;
pub const SQL_API_SQLGETCONNECTOPTION: u32 = 42;
pub const SQL_API_SQLGETCURSORNAME: u32 = 17;
pub const SQL_API_SQLGETDATA: u32 = 43;
pub const SQL_API_SQLGETDESCFIELD: u32 = 1008;
pub const SQL_API_SQLGETDESCREC: u32 = 1009;
pub const SQL_API_SQLGETDIAGFIELD: u32 = 1010;
pub const SQL_API_SQLGETDIAGREC: u32 = 1011;
pub const SQL_API_SQLGETENVATTR: u32 = 1012;
pub const SQL_API_SQLGETFUNCTIONS: u32 = 44;
pub const SQL_API_SQLGETINFO: u32 = 45;
pub const SQL_API_SQLGETSTMTATTR: u32 = 1014;
pub const SQL_API_SQLGETSTMTOPTION: u32 = 46;
pub const SQL_API_SQLGETTYPEINFO: u32 = 47;
pub const SQL_API_SQLNUMRESULTCOLS: u32 = 18;
pub const SQL_API_SQLPARAMDATA: u32 = 48;
pub const SQL_API_SQLPREPARE: u32 = 19;
pub const SQL_API_SQLPUTDATA: u32 = 49;
pub const SQL_API_SQLROWCOUNT: u32 = 20;
pub const SQL_API_SQLSETCONNECTATTR: u32 = 1016;
pub const SQL_API_SQLSETCONNECTOPTION: u32 = 50;
pub const SQL_API_SQLSETCURSORNAME: u32 = 21;
pub const SQL_API_SQLSETDESCFIELD: u32 = 1017;
pub const SQL_API_SQLSETDESCREC: u32 = 1018;
pub const SQL_API_SQLSETENVATTR: u32 = 1019;
pub const SQL_API_SQLSETPARAM: u32 = 22;
pub const SQL_API_SQLSETSTMTATTR: u32 = 1020;
pub const SQL_API_SQLSETSTMTOPTION: u32 = 51;
pub const SQL_API_SQLSPECIALCOLUMNS: u32 = 52;
pub const SQL_API_SQLSTATISTICS: u32 = 53;
pub const SQL_API_SQLTABLES: u32 = 54;
pub const SQL_API_SQLTRANSACT: u32 = 23;
pub const SQL_ARD_TYPE: i32 = -99;
pub const SQL_ATTR_APP_PARAM_DESC: u32 = 10011;
pub const SQL_ATTR_APP_ROW_DESC: u32 = 10010;
pub const SQL_ATTR_AUTO_IPD: u32 = 10001;
pub const SQL_ATTR_CURSOR_SCROLLABLE: i32 = -1;
pub const SQL_ATTR_CURSOR_SENSITIVITY: i32 = -2;
pub const SQL_ATTR_IMP_PARAM_DESC: u32 = 10013;
pub const SQL_ATTR_IMP_ROW_DESC: u32 = 10012;
pub const SQL_ATTR_METADATA_ID: u32 = 10014;
pub const SQL_ATTR_OUTPUT_NTS: u32 = 10001;
pub const SQL_AT_ADD_COLUMN: u32 = 1;
pub const SQL_AT_ADD_CONSTRAINT: u32 = 8;
pub const SQL_AT_DROP_COLUMN: u32 = 2;
pub const SQL_CATALOG_NAME: u32 = 10003;
pub const SQL_CB_CLOSE: u32 = 1;
pub const SQL_CB_DELETE: u32 = 0;
pub const SQL_CB_PRESERVE: u32 = 2;
pub const SQL_CHAR: u32 = 1;
pub const SQL_CLOSE: u32 = 0;
pub const SQL_CODE_DATE: u32 = 1;
pub const SQL_CODE_TIME: u32 = 2;
pub const SQL_CODE_TIMESTAMP: u32 = 3;
pub const SQL_COLLATION_SEQ: u32 = 10004;
pub const SQL_COMMIT: u32 = 0;
pub const SQL_CURSOR_COMMIT_BEHAVIOR: u32 = 23;
pub const SQL_CURSOR_SENSITIVITY: u32 = 10001;
pub const SQL_DATA_AT_EXEC: i32 = -2;
pub const SQL_DATA_SOURCE_NAME: u32 = 2;
pub const SQL_DATA_SOURCE_READ_ONLY: u32 = 25;
pub const SQL_DATETIME: u32 = 9;
pub const SQL_DATE_LEN: u32 = 10;
pub const SQL_DBMS_NAME: u32 = 17;
pub const SQL_DBMS_VER: u32 = 18;
pub const SQL_DECIMAL: u32 = 3;
pub const SQL_DEFAULT: u32 = 99;
pub const SQL_DEFAULT_TXN_ISOLATION: u32 = 26;
pub const SQL_DESCRIBE_PARAMETER: u32 = 10002;
pub const SQL_DESC_ALLOC_AUTO: u32 = 1;
pub const SQL_DESC_ALLOC_TYPE: u32 = 1099;
pub const SQL_DESC_ALLOC_USER: u32 = 2;
pub const SQL_DESC_COUNT: u32 = 1001;
pub const SQL_DESC_DATA_PTR: u32 = 1010;
pub const SQL_DESC_DATETIME_INTERVAL_CODE: u32 = 1007;
pub const SQL_DESC_INDICATOR_PTR: u32 = 1009;
pub const SQL_DESC_LENGTH: u32 = 1003;
pub const SQL_DESC_NAME: u32 = 1011;
pub const SQL_DESC_NULLABLE: u32 = 1008;
pub const SQL_DESC_OCTET_LENGTH: u32 = 1013;
pub const SQL_DESC_OCTET_LENGTH_PTR: u32 = 1004;
pub const SQL_DESC_PRECISION: u32 = 1005;
pub const SQL_DESC_SCALE: u32 = 1006;
pub const SQL_DESC_TYPE: u32 = 1002;
pub const SQL_DESC_UNNAMED: u32 = 1012;
pub const SQL_DIAG_ALTER_DOMAIN: u32 = 3;
pub const SQL_DIAG_ALTER_TABLE: u32 = 4;
pub const SQL_DIAG_CALL: u32 = 7;
pub const SQL_DIAG_CLASS_ORIGIN: u32 = 8;
pub const SQL_DIAG_CONNECTION_NAME: u32 = 10;
pub const SQL_DIAG_CREATE_ASSERTION: u32 = 6;
pub const SQL_DIAG_CREATE_CHARACTER_SET: u32 = 8;
pub const SQL_DIAG_CREATE_COLLATION: u32 = 10;
pub const SQL_DIAG_CREATE_DOMAIN: u32 = 23;
pub const SQL_DIAG_CREATE_INDEX: i32 = -1;
pub const SQL_DIAG_CREATE_SCHEMA: u32 = 64;
pub const SQL_DIAG_CREATE_TABLE: u32 = 77;
pub const SQL_DIAG_CREATE_TRANSLATION: u32 = 79;
pub const SQL_DIAG_CREATE_VIEW: u32 = 84;
pub const SQL_DIAG_DELETE_WHERE: u32 = 19;
pub const SQL_DIAG_DROP_ASSERTION: u32 = 24;
pub const SQL_DIAG_DROP_CHARACTER_SET: u32 = 25;
pub const SQL_DIAG_DROP_COLLATION: u32 = 26;
pub const SQL_DIAG_DROP_DOMAIN: u32 = 27;
pub const SQL_DIAG_DROP_INDEX: i32 = -2;
pub const SQL_DIAG_DROP_SCHEMA: u32 = 31;
pub const SQL_DIAG_DROP_TABLE: u32 = 32;
pub const SQL_DIAG_DROP_TRANSLATION: u32 = 33;
pub const SQL_DIAG_DROP_VIEW: u32 = 36;
pub const SQL_DIAG_DYNAMIC_DELETE_CURSOR: u32 = 38;
pub const SQL_DIAG_DYNAMIC_FUNCTION: u32 = 7;
pub const SQL_DIAG_DYNAMIC_FUNCTION_CODE: u32 = 12;
pub const SQL_DIAG_DYNAMIC_UPDATE_CURSOR: u32 = 81;
pub const SQL_DIAG_GRANT: u32 = 48;
pub const SQL_DIAG_INSERT: u32 = 50;
pub const SQL_DIAG_MESSAGE_TEXT: u32 = 6;
pub const SQL_DIAG_NATIVE: u32 = 5;
pub const SQL_DIAG_NUMBER: u32 = 2;
pub const SQL_DIAG_RETURNCODE: u32 = 1;
pub const SQL_DIAG_REVOKE: u32 = 59;
pub const SQL_DIAG_ROW_COUNT: u32 = 3;
pub const SQL_DIAG_SELECT_CURSOR: u32 = 85;
pub const SQL_DIAG_SERVER_NAME: u32 = 11;
pub const SQL_DIAG_SQLSTATE: u32 = 4;
pub const SQL_DIAG_SUBCLASS_ORIGIN: u32 = 9;
pub const SQL_DIAG_UNKNOWN_STATEMENT: u32 = 0;
pub const SQL_DIAG_UPDATE_WHERE: u32 = 82;
pub const SQL_DOUBLE: u32 = 8;
pub const SQL_DROP: u32 = 1;
pub const SQL_ERROR: i32 = -1;
pub const SQL_FALSE: u32 = 0;
pub const SQL_FD_FETCH_ABSOLUTE: u32 = 16;
pub const SQL_FD_FETCH_FIRST: u32 = 2;
pub const SQL_FD_FETCH_LAST: u32 = 4;
pub const SQL_FD_FETCH_NEXT: u32 = 1;
pub const SQL_FD_FETCH_PRIOR: u32 = 8;
pub const SQL_FD_FETCH_RELATIVE: u32 = 32;
pub const SQL_FETCH_ABSOLUTE: u32 = 5;
pub const SQL_FETCH_DIRECTION: u32 = 8;
pub const SQL_FETCH_FIRST: u32 = 2;
pub const SQL_FETCH_LAST: u32 = 3;
pub const SQL_FETCH_NEXT: u32 = 1;
pub const SQL_FETCH_PRIOR: u32 = 4;
pub const SQL_FETCH_RELATIVE: u32 = 6;
pub const SQL_FLOAT: u32 = 6;
pub const SQL_GD_ANY_COLUMN: u32 = 1;
pub const SQL_GD_ANY_ORDER: u32 = 2;
pub const SQL_GETDATA_EXTENSIONS: u32 = 81;
pub const SQL_HANDLE_DBC: u32 = 2;
pub const SQL_HANDLE_DESC: u32 = 4;
pub const SQL_HANDLE_ENV: u32 = 1;
pub const SQL_HANDLE_STMT: u32 = 3;
pub const SQL_IC_LOWER: u32 = 2;
pub const SQL_IC_MIXED: u32 = 4;
pub const SQL_IC_SENSITIVE: u32 = 3;
pub const SQL_IC_UPPER: u32 = 1;
pub const SQL_IDENTIFIER_CASE: u32 = 28;
pub const SQL_IDENTIFIER_QUOTE_CHAR: u32 = 29;
pub const SQL_INDEX_ALL: u32 = 1;
pub const SQL_INDEX_CLUSTERED: u32 = 1;
pub const SQL_INDEX_HASHED: u32 = 2;
pub const SQL_INDEX_OTHER: u32 = 3;
pub const SQL_INDEX_UNIQUE: u32 = 0;
pub const SQL_INSENSITIVE: u32 = 1;
pub const SQL_INTEGER: u32 = 4;
pub const SQL_INTEGRITY: u32 = 73;
pub const SQL_INVALID_HANDLE: i32 = -2;
pub const SQL_MAXIMUM_CATALOG_NAME_LENGTH: u32 = 34;
pub const SQL_MAXIMUM_COLUMNS_IN_GROUP_BY: u32 = 97;
pub const SQL_MAXIMUM_COLUMNS_IN_INDEX: u32 = 98;
pub const SQL_MAXIMUM_COLUMNS_IN_ORDER_BY: u32 = 99;
pub const SQL_MAXIMUM_COLUMNS_IN_SELECT: u32 = 100;
pub const SQL_MAXIMUM_COLUMN_NAME_LENGTH: u32 = 30;
pub const SQL_MAXIMUM_CONCURRENT_ACTIVITIES: u32 = 1;
pub const SQL_MAXIMUM_CURSOR_NAME_LENGTH: u32 = 31;
pub const SQL_MAXIMUM_DRIVER_CONNECTIONS: u32 = 0;
pub const SQL_MAXIMUM_IDENTIFIER_LENGTH: u32 = 10005;
pub const SQL_MAXIMUM_INDEX_SIZE: u32 = 102;
pub const SQL_MAXIMUM_ROW_SIZE: u32 = 104;
pub const SQL_MAXIMUM_SCHEMA_NAME_LENGTH: u32 = 32;
pub const SQL_MAXIMUM_STATEMENT_LENGTH: u32 = 105;
pub const SQL_MAXIMUM_TABLES_IN_SELECT: u32 = 106;
pub const SQL_MAXIMUM_USER_NAME_LENGTH: u32 = 107;
pub const SQL_MAX_CATALOG_NAME_LEN: u32 = 34;
pub const SQL_MAX_COLUMNS_IN_GROUP_BY: u32 = 97;
pub const SQL_MAX_COLUMNS_IN_INDEX: u32 = 98;
pub const SQL_MAX_COLUMNS_IN_ORDER_BY: u32 = 99;
pub const SQL_MAX_COLUMNS_IN_SELECT: u32 = 100;
pub const SQL_MAX_COLUMNS_IN_TABLE: u32 = 101;
pub const SQL_MAX_COLUMN_NAME_LEN: u32 = 30;
pub const SQL_MAX_CONCURRENT_ACTIVITIES: u32 = 1;
pub const SQL_MAX_CURSOR_NAME_LEN: u32 = 31;
pub const SQL_MAX_DRIVER_CONNECTIONS: u32 = 0;
pub const SQL_MAX_IDENTIFIER_LEN: u32 = 10005;
pub const SQL_MAX_INDEX_SIZE: u32 = 102;
pub const SQL_MAX_MESSAGE_LENGTH: u32 = 512;
pub const SQL_MAX_ROW_SIZE: u32 = 104;
pub const SQL_MAX_SCHEMA_NAME_LEN: u32 = 32;
pub const SQL_MAX_STATEMENT_LEN: u32 = 105;
pub const SQL_MAX_TABLES_IN_SELECT: u32 = 106;
pub const SQL_MAX_TABLE_NAME_LEN: u32 = 35;
pub const SQL_MAX_USER_NAME_LEN: u32 = 107;
pub const SQL_NAMED: u32 = 0;
pub const SQL_NC_HIGH: u32 = 0;
pub const SQL_NC_LOW: u32 = 1;
pub const SQL_NEED_DATA: u32 = 99;
pub const SQL_NONSCROLLABLE: u32 = 0;
pub const SQL_NO_DATA: u32 = 100;
pub const SQL_NO_NULLS: u32 = 0;
pub const SQL_NTS: i32 = -3;
pub const SQL_NTSL: i32 = -3;
pub const SQL_NULLABLE: u32 = 1;
pub const SQL_NULLABLE_UNKNOWN: u32 = 2;
pub const SQL_NULL_COLLATION: u32 = 85;
pub const SQL_NULL_DATA: i32 = -1;
pub const SQL_NULL_HANDLE: u32 = 0;
pub const SQL_NULL_HDBC: u32 = 0;
pub const SQL_NULL_HDESC: u32 = 0;
pub const SQL_NULL_HENV: u32 = 0;
pub const SQL_NULL_HSTMT: u32 = 0;
pub const SQL_NUMERIC: u32 = 2;
pub const SQL_OJ_ALL_COMPARISON_OPS: u32 = 64;
pub const SQL_OJ_CAPABILITIES: u32 = 115;
pub const SQL_OJ_FULL: u32 = 4;
pub const SQL_OJ_INNER: u32 = 32;
pub const SQL_OJ_LEFT: u32 = 1;
pub const SQL_OJ_NESTED: u32 = 8;
pub const SQL_OJ_NOT_ORDERED: u32 = 16;
pub const SQL_OJ_RIGHT: u32 = 2;
pub const SQL_ORDER_BY_COLUMNS_IN_SELECT: u32 = 90;
pub const SQL_OUTER_JOIN_CAPABILITIES: u32 = 115;
pub const SQL_PARAM_DATA_AVAILABLE: u32 = 101;
pub const SQL_PC_NON_PSEUDO: u32 = 1;
pub const SQL_PC_PSEUDO: u32 = 2;
pub const SQL_PC_UNKNOWN: u32 = 0;
pub const SQL_PRED_BASIC: u32 = 2;
pub const SQL_PRED_CHAR: u32 = 1;
pub const SQL_PRED_NONE: u32 = 0;
pub const SQL_REAL: u32 = 7;
pub const SQL_RESET_PARAMS: u32 = 3;
pub const SQL_ROLLBACK: u32 = 1;
pub const SQL_ROW_IDENTIFIER: u32 = 1;
pub const SQL_SCCO_LOCK: u32 = 2;
pub const SQL_SCCO_OPT_ROWVER: u32 = 4;
pub const SQL_SCCO_OPT_VALUES: u32 = 8;
pub const SQL_SCCO_READ_ONLY: u32 = 1;
pub const SQL_SCOPE_CURROW: u32 = 0;
pub const SQL_SCOPE_SESSION: u32 = 2;
pub const SQL_SCOPE_TRANSACTION: u32 = 1;
pub const SQL_SCROLLABLE: u32 = 1;
pub const SQL_SCROLL_CONCURRENCY: u32 = 43;
pub const SQL_SEARCH_PATTERN_ESCAPE: u32 = 14;
pub const SQL_SENSITIVE: u32 = 2;
pub const SQL_SERVER_NAME: u32 = 13;
pub const SQL_SMALLINT: u32 = 5;
pub const SQL_SPECIAL_CHARACTERS: u32 = 94;
pub const SQL_STILL_EXECUTING: u32 = 2;
pub const SQL_SUCCESS: u32 = 0;
pub const SQL_SUCCESS_WITH_INFO: u32 = 1;
pub const SQL_TC_ALL: u32 = 2;
pub const SQL_TC_DDL_COMMIT: u32 = 3;
pub const SQL_TC_DDL_IGNORE: u32 = 4;
pub const SQL_TC_DML: u32 = 1;
pub const SQL_TC_NONE: u32 = 0;
pub const SQL_TIMESTAMP_LEN: u32 = 19;
pub const SQL_TIME_LEN: u32 = 8;
pub const SQL_TRANSACTION_CAPABLE: u32 = 46;
pub const SQL_TRANSACTION_ISOLATION_OPTION: u32 = 72;
pub const SQL_TRANSACTION_READ_COMMITTED: u32 = 2;
pub const SQL_TRANSACTION_READ_UNCOMMITTED: u32 = 1;
pub const SQL_TRANSACTION_REPEATABLE_READ: u32 = 4;
pub const SQL_TRANSACTION_SERIALIZABLE: u32 = 8;
pub const SQL_TRUE: u32 = 1;
pub const SQL_TXN_CAPABLE: u32 = 46;
pub const SQL_TXN_ISOLATION_OPTION: u32 = 72;
pub const SQL_TXN_READ_COMMITTED: u32 = 2;
pub const SQL_TXN_READ_UNCOMMITTED: u32 = 1;
pub const SQL_TXN_REPEATABLE_READ: u32 = 4;
pub const SQL_TXN_SERIALIZABLE: u32 = 8;
pub const SQL_TYPE_DATE: u32 = 91;
pub const SQL_TYPE_TIME: u32 = 92;
pub const SQL_TYPE_TIMESTAMP: u32 = 93;
pub const SQL_UNBIND: u32 = 2;
pub const SQL_UNKNOWN_TYPE: u32 = 0;
pub const SQL_UNNAMED: u32 = 1;
pub const SQL_UNSPECIFIED: u32 = 0;
pub const SQL_USER_NAME: u32 = 47;
pub const SQL_VARCHAR: u32 = 12;
pub const SQL_XOPEN_CLI_YEAR: u32 = 10000;
