#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLAllocConnect(environmenthandle : super::SQLHENV, connectionhandle : *mut super::SQLHDBC) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLAllocEnv(environmenthandle : *mut super::SQLHENV) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLAllocHandle(handletype : super::SQLSMALLINT, inputhandle : super::SQLHANDLE, outputhandle : *mut super::SQLHANDLE) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLAllocStmt(connectionhandle : super::SQLHDBC, statementhandle : *mut super::SQLHSTMT) -> super::SQLRETURN);
#[cfg(target_arch = "x86")]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLBindCol(statementhandle : super::SQLHSTMT, columnnumber : super::SQLUSMALLINT, targettype : super::SQLSMALLINT, targetvalue : super::SQLPOINTER, bufferlength : super::SQLINTEGER, strlen_or_ind : *mut super::SQLINTEGER) -> super::SQLRETURN);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLBindCol(statementhandle : super::SQLHSTMT, columnnumber : super::SQLUSMALLINT, targettype : super::SQLSMALLINT, targetvalue : super::SQLPOINTER, bufferlength : super::SQLLEN, strlen_or_ind : *mut super::SQLLEN) -> super::SQLRETURN);
#[cfg(target_arch = "x86")]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLBindParam(statementhandle : super::SQLHSTMT, parameternumber : super::SQLUSMALLINT, valuetype : super::SQLSMALLINT, parametertype : super::SQLSMALLINT, lengthprecision : super::SQLUINTEGER, parameterscale : super::SQLSMALLINT, parametervalue : super::SQLPOINTER, strlen_or_ind : *mut super::SQLINTEGER) -> super::SQLRETURN);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLBindParam(statementhandle : super::SQLHSTMT, parameternumber : super::SQLUSMALLINT, valuetype : super::SQLSMALLINT, parametertype : super::SQLSMALLINT, lengthprecision : super::SQLULEN, parameterscale : super::SQLSMALLINT, parametervalue : super::SQLPOINTER, strlen_or_ind : *mut super::SQLLEN) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLCancel(statementhandle : super::SQLHSTMT) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLCancelHandle(handletype : super::SQLSMALLINT, inputhandle : super::SQLHANDLE) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLCloseCursor(statementhandle : super::SQLHSTMT) -> super::SQLRETURN);
#[cfg(target_arch = "x86")]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLColAttribute(statementhandle : super::SQLHSTMT, columnnumber : super::SQLUSMALLINT, fieldidentifier : super::SQLUSMALLINT, characterattribute : super::SQLPOINTER, bufferlength : super::SQLSMALLINT, stringlength : *mut super::SQLSMALLINT, numericattribute : super::SQLPOINTER) -> super::SQLRETURN);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLColAttribute(statementhandle : super::SQLHSTMT, columnnumber : super::SQLUSMALLINT, fieldidentifier : super::SQLUSMALLINT, characterattribute : super::SQLPOINTER, bufferlength : super::SQLSMALLINT, stringlength : *mut super::SQLSMALLINT, numericattribute : *mut super::SQLLEN) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLColumns(statementhandle : super::SQLHSTMT, catalogname : *const super::SQLCHAR, namelength1 : super::SQLSMALLINT, schemaname : *const super::SQLCHAR, namelength2 : super::SQLSMALLINT, tablename : *const super::SQLCHAR, namelength3 : super::SQLSMALLINT, columnname : *const super::SQLCHAR, namelength4 : super::SQLSMALLINT) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLCompleteAsync(handletype : super::SQLSMALLINT, handle : super::SQLHANDLE, asyncretcodeptr : *mut super::RETCODE) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLConnect(connectionhandle : super::SQLHDBC, servername : *const super::SQLCHAR, namelength1 : super::SQLSMALLINT, username : *const super::SQLCHAR, namelength2 : super::SQLSMALLINT, authentication : *const super::SQLCHAR, namelength3 : super::SQLSMALLINT) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLCopyDesc(sourcedeschandle : super::SQLHDESC, targetdeschandle : super::SQLHDESC) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLDataSources(environmenthandle : super::SQLHENV, direction : super::SQLUSMALLINT, servername : *mut super::SQLCHAR, bufferlength1 : super::SQLSMALLINT, namelength1ptr : *mut super::SQLSMALLINT, description : *mut super::SQLCHAR, bufferlength2 : super::SQLSMALLINT, namelength2ptr : *mut super::SQLSMALLINT) -> super::SQLRETURN);
#[cfg(target_arch = "x86")]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLDescribeCol(statementhandle : super::SQLHSTMT, columnnumber : super::SQLUSMALLINT, columnname : *mut super::SQLCHAR, bufferlength : super::SQLSMALLINT, namelength : *mut super::SQLSMALLINT, datatype : *mut super::SQLSMALLINT, columnsize : *mut super::SQLUINTEGER, decimaldigits : *mut super::SQLSMALLINT, nullable : *mut super::SQLSMALLINT) -> super::SQLRETURN);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLDescribeCol(statementhandle : super::SQLHSTMT, columnnumber : super::SQLUSMALLINT, columnname : *mut super::SQLCHAR, bufferlength : super::SQLSMALLINT, namelength : *mut super::SQLSMALLINT, datatype : *mut super::SQLSMALLINT, columnsize : *mut super::SQLULEN, decimaldigits : *mut super::SQLSMALLINT, nullable : *mut super::SQLSMALLINT) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLDisconnect(connectionhandle : super::SQLHDBC) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLEndTran(handletype : super::SQLSMALLINT, handle : super::SQLHANDLE, completiontype : super::SQLSMALLINT) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLError(environmenthandle : super::SQLHENV, connectionhandle : super::SQLHDBC, statementhandle : super::SQLHSTMT, sqlstate : *mut super::SQLCHAR, nativeerror : *mut super::SQLINTEGER, messagetext : *mut super::SQLCHAR, bufferlength : super::SQLSMALLINT, textlength : *mut super::SQLSMALLINT) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLExecDirect(statementhandle : super::SQLHSTMT, statementtext : *const super::SQLCHAR, textlength : super::SQLINTEGER) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLExecute(statementhandle : super::SQLHSTMT) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLFetch(statementhandle : super::SQLHSTMT) -> super::SQLRETURN);
#[cfg(target_arch = "x86")]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLFetchScroll(statementhandle : super::SQLHSTMT, fetchorientation : super::SQLSMALLINT, fetchoffset : super::SQLINTEGER) -> super::SQLRETURN);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLFetchScroll(statementhandle : super::SQLHSTMT, fetchorientation : super::SQLSMALLINT, fetchoffset : super::SQLLEN) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLFreeConnect(connectionhandle : super::SQLHDBC) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLFreeEnv(environmenthandle : super::SQLHENV) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLFreeHandle(handletype : super::SQLSMALLINT, handle : super::SQLHANDLE) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLFreeStmt(statementhandle : super::SQLHSTMT, option : super::SQLUSMALLINT) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLGetConnectAttr(connectionhandle : super::SQLHDBC, attribute : super::SQLINTEGER, value : super::SQLPOINTER, bufferlength : super::SQLINTEGER, stringlengthptr : *mut super::SQLINTEGER) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLGetConnectOption(connectionhandle : super::SQLHDBC, option : super::SQLUSMALLINT, value : super::SQLPOINTER) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLGetCursorName(statementhandle : super::SQLHSTMT, cursorname : *mut super::SQLCHAR, bufferlength : super::SQLSMALLINT, namelengthptr : *mut super::SQLSMALLINT) -> super::SQLRETURN);
#[cfg(target_arch = "x86")]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLGetData(statementhandle : super::SQLHSTMT, columnnumber : super::SQLUSMALLINT, targettype : super::SQLSMALLINT, targetvalue : super::SQLPOINTER, bufferlength : super::SQLINTEGER, strlen_or_indptr : *mut super::SQLINTEGER) -> super::SQLRETURN);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLGetData(statementhandle : super::SQLHSTMT, columnnumber : super::SQLUSMALLINT, targettype : super::SQLSMALLINT, targetvalue : super::SQLPOINTER, bufferlength : super::SQLLEN, strlen_or_indptr : *mut super::SQLLEN) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLGetDescField(descriptorhandle : super::SQLHDESC, recnumber : super::SQLSMALLINT, fieldidentifier : super::SQLSMALLINT, value : super::SQLPOINTER, bufferlength : super::SQLINTEGER, stringlength : *mut super::SQLINTEGER) -> super::SQLRETURN);
#[cfg(target_arch = "x86")]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLGetDescRec(descriptorhandle : super::SQLHDESC, recnumber : super::SQLSMALLINT, name : *mut super::SQLCHAR, bufferlength : super::SQLSMALLINT, stringlengthptr : *mut super::SQLSMALLINT, typeptr : *mut super::SQLSMALLINT, subtypeptr : *mut super::SQLSMALLINT, lengthptr : *mut super::SQLINTEGER, precisionptr : *mut super::SQLSMALLINT, scaleptr : *mut super::SQLSMALLINT, nullableptr : *mut super::SQLSMALLINT) -> super::SQLRETURN);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLGetDescRec(descriptorhandle : super::SQLHDESC, recnumber : super::SQLSMALLINT, name : *mut super::SQLCHAR, bufferlength : super::SQLSMALLINT, stringlengthptr : *mut super::SQLSMALLINT, typeptr : *mut super::SQLSMALLINT, subtypeptr : *mut super::SQLSMALLINT, lengthptr : *mut super::SQLLEN, precisionptr : *mut super::SQLSMALLINT, scaleptr : *mut super::SQLSMALLINT, nullableptr : *mut super::SQLSMALLINT) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLGetDiagField(handletype : super::SQLSMALLINT, handle : super::SQLHANDLE, recnumber : super::SQLSMALLINT, diagidentifier : super::SQLSMALLINT, diaginfo : super::SQLPOINTER, bufferlength : super::SQLSMALLINT, stringlength : *mut super::SQLSMALLINT) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLGetDiagRec(handletype : super::SQLSMALLINT, handle : super::SQLHANDLE, recnumber : super::SQLSMALLINT, sqlstate : *mut super::SQLCHAR, nativeerror : *mut super::SQLINTEGER, messagetext : *mut super::SQLCHAR, bufferlength : super::SQLSMALLINT, textlength : *mut super::SQLSMALLINT) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLGetEnvAttr(environmenthandle : super::SQLHENV, attribute : super::SQLINTEGER, value : super::SQLPOINTER, bufferlength : super::SQLINTEGER, stringlength : *mut super::SQLINTEGER) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLGetFunctions(connectionhandle : super::SQLHDBC, functionid : super::SQLUSMALLINT, supported : *mut super::SQLUSMALLINT) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLGetInfo(connectionhandle : super::SQLHDBC, infotype : super::SQLUSMALLINT, infovalue : super::SQLPOINTER, bufferlength : super::SQLSMALLINT, stringlengthptr : *mut super::SQLSMALLINT) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLGetStmtAttr(statementhandle : super::SQLHSTMT, attribute : super::SQLINTEGER, value : super::SQLPOINTER, bufferlength : super::SQLINTEGER, stringlength : *mut super::SQLINTEGER) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLGetStmtOption(statementhandle : super::SQLHSTMT, option : super::SQLUSMALLINT, value : super::SQLPOINTER) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLGetTypeInfo(statementhandle : super::SQLHSTMT, datatype : super::SQLSMALLINT) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLNumResultCols(statementhandle : super::SQLHSTMT, columncount : *mut super::SQLSMALLINT) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLParamData(statementhandle : super::SQLHSTMT, value : *mut super::SQLPOINTER) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLPrepare(statementhandle : super::SQLHSTMT, statementtext : *const super::SQLCHAR, textlength : super::SQLINTEGER) -> super::SQLRETURN);
#[cfg(target_arch = "x86")]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLPutData(statementhandle : super::SQLHSTMT, data : super::SQLPOINTER, strlen_or_ind : super::SQLINTEGER) -> super::SQLRETURN);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLPutData(statementhandle : super::SQLHSTMT, data : super::SQLPOINTER, strlen_or_ind : super::SQLLEN) -> super::SQLRETURN);
#[cfg(target_arch = "x86")]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLRowCount(statementhandle : super::SQLHSTMT, rowcount : *mut super::SQLINTEGER) -> super::SQLRETURN);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLRowCount(statementhandle : super::SQLHSTMT, rowcount : *mut super::SQLLEN) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLSetConnectAttr(connectionhandle : super::SQLHDBC, attribute : super::SQLINTEGER, value : super::SQLPOINTER, stringlength : super::SQLINTEGER) -> super::SQLRETURN);
#[cfg(target_arch = "x86")]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLSetConnectOption(connectionhandle : super::SQLHDBC, option : super::SQLUSMALLINT, value : super::SQLUINTEGER) -> super::SQLRETURN);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLSetConnectOption(connectionhandle : super::SQLHDBC, option : super::SQLUSMALLINT, value : super::SQLULEN) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLSetCursorName(statementhandle : super::SQLHSTMT, cursorname : *const super::SQLCHAR, namelength : super::SQLSMALLINT) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLSetDescField(descriptorhandle : super::SQLHDESC, recnumber : super::SQLSMALLINT, fieldidentifier : super::SQLSMALLINT, value : super::SQLPOINTER, bufferlength : super::SQLINTEGER) -> super::SQLRETURN);
#[cfg(target_arch = "x86")]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLSetDescRec(descriptorhandle : super::SQLHDESC, recnumber : super::SQLSMALLINT, r#type : super::SQLSMALLINT, subtype : super::SQLSMALLINT, length : super::SQLINTEGER, precision : super::SQLSMALLINT, scale : super::SQLSMALLINT, data : super::SQLPOINTER, stringlength : *mut super::SQLINTEGER, indicator : *mut super::SQLINTEGER) -> super::SQLRETURN);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLSetDescRec(descriptorhandle : super::SQLHDESC, recnumber : super::SQLSMALLINT, r#type : super::SQLSMALLINT, subtype : super::SQLSMALLINT, length : super::SQLLEN, precision : super::SQLSMALLINT, scale : super::SQLSMALLINT, data : super::SQLPOINTER, stringlength : *mut super::SQLLEN, indicator : *mut super::SQLLEN) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLSetEnvAttr(environmenthandle : super::SQLHENV, attribute : super::SQLINTEGER, value : super::SQLPOINTER, stringlength : super::SQLINTEGER) -> super::SQLRETURN);
#[cfg(target_arch = "x86")]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLSetParam(statementhandle : super::SQLHSTMT, parameternumber : super::SQLUSMALLINT, valuetype : super::SQLSMALLINT, parametertype : super::SQLSMALLINT, lengthprecision : super::SQLUINTEGER, parameterscale : super::SQLSMALLINT, parametervalue : super::SQLPOINTER, strlen_or_ind : *mut super::SQLINTEGER) -> super::SQLRETURN);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLSetParam(statementhandle : super::SQLHSTMT, parameternumber : super::SQLUSMALLINT, valuetype : super::SQLSMALLINT, parametertype : super::SQLSMALLINT, lengthprecision : super::SQLULEN, parameterscale : super::SQLSMALLINT, parametervalue : super::SQLPOINTER, strlen_or_ind : *mut super::SQLLEN) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLSetStmtAttr(statementhandle : super::SQLHSTMT, attribute : super::SQLINTEGER, value : super::SQLPOINTER, stringlength : super::SQLINTEGER) -> super::SQLRETURN);
#[cfg(target_arch = "x86")]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLSetStmtOption(statementhandle : super::SQLHSTMT, option : super::SQLUSMALLINT, value : super::SQLUINTEGER) -> super::SQLRETURN);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLSetStmtOption(statementhandle : super::SQLHSTMT, option : super::SQLUSMALLINT, value : super::SQLULEN) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLSpecialColumns(statementhandle : super::SQLHSTMT, identifiertype : super::SQLUSMALLINT, catalogname : *const super::SQLCHAR, namelength1 : super::SQLSMALLINT, schemaname : *const super::SQLCHAR, namelength2 : super::SQLSMALLINT, tablename : *const super::SQLCHAR, namelength3 : super::SQLSMALLINT, scope : super::SQLUSMALLINT, nullable : super::SQLUSMALLINT) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLStatistics(statementhandle : super::SQLHSTMT, catalogname : *const super::SQLCHAR, namelength1 : super::SQLSMALLINT, schemaname : *const super::SQLCHAR, namelength2 : super::SQLSMALLINT, tablename : *const super::SQLCHAR, namelength3 : super::SQLSMALLINT, unique : super::SQLUSMALLINT, reserved : super::SQLUSMALLINT) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLTables(statementhandle : super::SQLHSTMT, catalogname : *const super::SQLCHAR, namelength1 : super::SQLSMALLINT, schemaname : *const super::SQLCHAR, namelength2 : super::SQLSMALLINT, tablename : *const super::SQLCHAR, namelength3 : super::SQLSMALLINT, tabletype : *const super::SQLCHAR, namelength4 : super::SQLSMALLINT) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLTransact(environmenthandle : super::SQLHENV, connectionhandle : super::SQLHDBC, completiontype : super::SQLUSMALLINT) -> super::SQLRETURN);
pub const ODBCVER: i32 = 896;
pub const SQL_ACCESSIBLE_PROCEDURES: i32 = 20;
pub const SQL_ACCESSIBLE_TABLES: i32 = 19;
pub const SQL_ALL_TYPES: i32 = 0;
pub const SQL_ALTER_TABLE: i32 = 86;
pub const SQL_AM_CONNECTION: i32 = 1;
pub const SQL_AM_NONE: i32 = 0;
pub const SQL_AM_STATEMENT: i32 = 2;
pub const SQL_APD_TYPE: i32 = -100;
pub const SQL_API_SQLALLOCCONNECT: i32 = 1;
pub const SQL_API_SQLALLOCENV: i32 = 2;
pub const SQL_API_SQLALLOCHANDLE: i32 = 1001;
pub const SQL_API_SQLALLOCSTMT: i32 = 3;
pub const SQL_API_SQLBINDCOL: i32 = 4;
pub const SQL_API_SQLBINDPARAM: i32 = 1002;
pub const SQL_API_SQLCANCEL: i32 = 5;
pub const SQL_API_SQLCANCELHANDLE: i32 = 1550;
pub const SQL_API_SQLCLOSECURSOR: i32 = 1003;
pub const SQL_API_SQLCOLATTRIBUTE: i32 = 6;
pub const SQL_API_SQLCOLUMNS: i32 = 40;
pub const SQL_API_SQLCOMPLETEASYNC: i32 = 1551;
pub const SQL_API_SQLCONNECT: i32 = 7;
pub const SQL_API_SQLCOPYDESC: i32 = 1004;
pub const SQL_API_SQLDATASOURCES: i32 = 57;
pub const SQL_API_SQLDESCRIBECOL: i32 = 8;
pub const SQL_API_SQLDISCONNECT: i32 = 9;
pub const SQL_API_SQLENDTRAN: i32 = 1005;
pub const SQL_API_SQLERROR: i32 = 10;
pub const SQL_API_SQLEXECDIRECT: i32 = 11;
pub const SQL_API_SQLEXECUTE: i32 = 12;
pub const SQL_API_SQLFETCH: i32 = 13;
pub const SQL_API_SQLFETCHSCROLL: i32 = 1021;
pub const SQL_API_SQLFREECONNECT: i32 = 14;
pub const SQL_API_SQLFREEENV: i32 = 15;
pub const SQL_API_SQLFREEHANDLE: i32 = 1006;
pub const SQL_API_SQLFREESTMT: i32 = 16;
pub const SQL_API_SQLGETCONNECTATTR: i32 = 1007;
pub const SQL_API_SQLGETCONNECTOPTION: i32 = 42;
pub const SQL_API_SQLGETCURSORNAME: i32 = 17;
pub const SQL_API_SQLGETDATA: i32 = 43;
pub const SQL_API_SQLGETDESCFIELD: i32 = 1008;
pub const SQL_API_SQLGETDESCREC: i32 = 1009;
pub const SQL_API_SQLGETDIAGFIELD: i32 = 1010;
pub const SQL_API_SQLGETDIAGREC: i32 = 1011;
pub const SQL_API_SQLGETENVATTR: i32 = 1012;
pub const SQL_API_SQLGETFUNCTIONS: i32 = 44;
pub const SQL_API_SQLGETINFO: i32 = 45;
pub const SQL_API_SQLGETSTMTATTR: i32 = 1014;
pub const SQL_API_SQLGETSTMTOPTION: i32 = 46;
pub const SQL_API_SQLGETTYPEINFO: i32 = 47;
pub const SQL_API_SQLNUMRESULTCOLS: i32 = 18;
pub const SQL_API_SQLPARAMDATA: i32 = 48;
pub const SQL_API_SQLPREPARE: i32 = 19;
pub const SQL_API_SQLPUTDATA: i32 = 49;
pub const SQL_API_SQLROWCOUNT: i32 = 20;
pub const SQL_API_SQLSETCONNECTATTR: i32 = 1016;
pub const SQL_API_SQLSETCONNECTOPTION: i32 = 50;
pub const SQL_API_SQLSETCURSORNAME: i32 = 21;
pub const SQL_API_SQLSETDESCFIELD: i32 = 1017;
pub const SQL_API_SQLSETDESCREC: i32 = 1018;
pub const SQL_API_SQLSETENVATTR: i32 = 1019;
pub const SQL_API_SQLSETPARAM: i32 = 22;
pub const SQL_API_SQLSETSTMTATTR: i32 = 1020;
pub const SQL_API_SQLSETSTMTOPTION: i32 = 51;
pub const SQL_API_SQLSPECIALCOLUMNS: i32 = 52;
pub const SQL_API_SQLSTATISTICS: i32 = 53;
pub const SQL_API_SQLTABLES: i32 = 54;
pub const SQL_API_SQLTRANSACT: i32 = 23;
pub const SQL_ARD_TYPE: i32 = -99;
pub const SQL_ATTR_APP_PARAM_DESC: i32 = 10011;
pub const SQL_ATTR_APP_ROW_DESC: i32 = 10010;
pub const SQL_ATTR_AUTO_IPD: i32 = 10001;
pub const SQL_ATTR_CURSOR_SCROLLABLE: i32 = -1;
pub const SQL_ATTR_CURSOR_SENSITIVITY: i32 = -2;
pub const SQL_ATTR_IMP_PARAM_DESC: i32 = 10013;
pub const SQL_ATTR_IMP_ROW_DESC: i32 = 10012;
pub const SQL_ATTR_METADATA_ID: i32 = 10014;
pub const SQL_ATTR_OUTPUT_NTS: i32 = 10001;
pub const SQL_AT_ADD_COLUMN: i32 = 1;
pub const SQL_AT_ADD_CONSTRAINT: i32 = 8;
pub const SQL_AT_DROP_COLUMN: i32 = 2;
pub const SQL_CATALOG_NAME: i32 = 10003;
pub const SQL_CB_CLOSE: i32 = 1;
pub const SQL_CB_DELETE: i32 = 0;
pub const SQL_CB_PRESERVE: i32 = 2;
pub const SQL_CHAR: i32 = 1;
pub const SQL_CLOSE: i32 = 0;
pub const SQL_CODE_DATE: i32 = 1;
pub const SQL_CODE_TIME: i32 = 2;
pub const SQL_CODE_TIMESTAMP: i32 = 3;
pub const SQL_COLLATION_SEQ: i32 = 10004;
pub const SQL_COMMIT: i32 = 0;
pub const SQL_CURSOR_COMMIT_BEHAVIOR: i32 = 23;
pub const SQL_CURSOR_SENSITIVITY: i32 = 10001;
pub const SQL_DATA_AT_EXEC: i32 = -2;
pub const SQL_DATA_SOURCE_NAME: i32 = 2;
pub const SQL_DATA_SOURCE_READ_ONLY: i32 = 25;
pub const SQL_DATETIME: i32 = 9;
pub const SQL_DATE_LEN: i32 = 10;
pub const SQL_DBMS_NAME: i32 = 17;
pub const SQL_DBMS_VER: i32 = 18;
pub const SQL_DECIMAL: i32 = 3;
pub const SQL_DEFAULT: i32 = 99;
pub const SQL_DEFAULT_TXN_ISOLATION: i32 = 26;
pub const SQL_DESCRIBE_PARAMETER: i32 = 10002;
pub const SQL_DESC_ALLOC_AUTO: i32 = 1;
pub const SQL_DESC_ALLOC_TYPE: i32 = 1099;
pub const SQL_DESC_ALLOC_USER: i32 = 2;
pub const SQL_DESC_COUNT: i32 = 1001;
pub const SQL_DESC_DATA_PTR: i32 = 1010;
pub const SQL_DESC_DATETIME_INTERVAL_CODE: i32 = 1007;
pub const SQL_DESC_INDICATOR_PTR: i32 = 1009;
pub const SQL_DESC_LENGTH: i32 = 1003;
pub const SQL_DESC_NAME: i32 = 1011;
pub const SQL_DESC_NULLABLE: i32 = 1008;
pub const SQL_DESC_OCTET_LENGTH: i32 = 1013;
pub const SQL_DESC_OCTET_LENGTH_PTR: i32 = 1004;
pub const SQL_DESC_PRECISION: i32 = 1005;
pub const SQL_DESC_SCALE: i32 = 1006;
pub const SQL_DESC_TYPE: i32 = 1002;
pub const SQL_DESC_UNNAMED: i32 = 1012;
pub const SQL_DIAG_ALTER_DOMAIN: i32 = 3;
pub const SQL_DIAG_ALTER_TABLE: i32 = 4;
pub const SQL_DIAG_CALL: i32 = 7;
pub const SQL_DIAG_CLASS_ORIGIN: i32 = 8;
pub const SQL_DIAG_CONNECTION_NAME: i32 = 10;
pub const SQL_DIAG_CREATE_ASSERTION: i32 = 6;
pub const SQL_DIAG_CREATE_CHARACTER_SET: i32 = 8;
pub const SQL_DIAG_CREATE_COLLATION: i32 = 10;
pub const SQL_DIAG_CREATE_DOMAIN: i32 = 23;
pub const SQL_DIAG_CREATE_INDEX: i32 = -1;
pub const SQL_DIAG_CREATE_SCHEMA: i32 = 64;
pub const SQL_DIAG_CREATE_TABLE: i32 = 77;
pub const SQL_DIAG_CREATE_TRANSLATION: i32 = 79;
pub const SQL_DIAG_CREATE_VIEW: i32 = 84;
pub const SQL_DIAG_DELETE_WHERE: i32 = 19;
pub const SQL_DIAG_DROP_ASSERTION: i32 = 24;
pub const SQL_DIAG_DROP_CHARACTER_SET: i32 = 25;
pub const SQL_DIAG_DROP_COLLATION: i32 = 26;
pub const SQL_DIAG_DROP_DOMAIN: i32 = 27;
pub const SQL_DIAG_DROP_INDEX: i32 = -2;
pub const SQL_DIAG_DROP_SCHEMA: i32 = 31;
pub const SQL_DIAG_DROP_TABLE: i32 = 32;
pub const SQL_DIAG_DROP_TRANSLATION: i32 = 33;
pub const SQL_DIAG_DROP_VIEW: i32 = 36;
pub const SQL_DIAG_DYNAMIC_DELETE_CURSOR: i32 = 38;
pub const SQL_DIAG_DYNAMIC_FUNCTION: i32 = 7;
pub const SQL_DIAG_DYNAMIC_FUNCTION_CODE: i32 = 12;
pub const SQL_DIAG_DYNAMIC_UPDATE_CURSOR: i32 = 81;
pub const SQL_DIAG_GRANT: i32 = 48;
pub const SQL_DIAG_INSERT: i32 = 50;
pub const SQL_DIAG_MESSAGE_TEXT: i32 = 6;
pub const SQL_DIAG_NATIVE: i32 = 5;
pub const SQL_DIAG_NUMBER: i32 = 2;
pub const SQL_DIAG_RETURNCODE: i32 = 1;
pub const SQL_DIAG_REVOKE: i32 = 59;
pub const SQL_DIAG_ROW_COUNT: i32 = 3;
pub const SQL_DIAG_SELECT_CURSOR: i32 = 85;
pub const SQL_DIAG_SERVER_NAME: i32 = 11;
pub const SQL_DIAG_SQLSTATE: i32 = 4;
pub const SQL_DIAG_SUBCLASS_ORIGIN: i32 = 9;
pub const SQL_DIAG_UNKNOWN_STATEMENT: i32 = 0;
pub const SQL_DIAG_UPDATE_WHERE: i32 = 82;
pub const SQL_DOUBLE: i32 = 8;
pub const SQL_DROP: i32 = 1;
pub const SQL_ERROR: i32 = -1;
pub const SQL_FALSE: i32 = 0;
pub const SQL_FD_FETCH_ABSOLUTE: i32 = 16;
pub const SQL_FD_FETCH_FIRST: i32 = 2;
pub const SQL_FD_FETCH_LAST: i32 = 4;
pub const SQL_FD_FETCH_NEXT: i32 = 1;
pub const SQL_FD_FETCH_PRIOR: i32 = 8;
pub const SQL_FD_FETCH_RELATIVE: i32 = 32;
pub const SQL_FETCH_ABSOLUTE: i32 = 5;
pub const SQL_FETCH_DIRECTION: i32 = 8;
pub const SQL_FETCH_FIRST: i32 = 2;
pub const SQL_FETCH_LAST: i32 = 3;
pub const SQL_FETCH_NEXT: i32 = 1;
pub const SQL_FETCH_PRIOR: i32 = 4;
pub const SQL_FETCH_RELATIVE: i32 = 6;
pub const SQL_FLOAT: i32 = 6;
pub const SQL_GD_ANY_COLUMN: i32 = 1;
pub const SQL_GD_ANY_ORDER: i32 = 2;
pub const SQL_GETDATA_EXTENSIONS: i32 = 81;
pub const SQL_HANDLE_DBC: i32 = 2;
pub const SQL_HANDLE_DESC: i32 = 4;
pub const SQL_HANDLE_ENV: i32 = 1;
pub const SQL_HANDLE_STMT: i32 = 3;
pub const SQL_IC_LOWER: i32 = 2;
pub const SQL_IC_MIXED: i32 = 4;
pub const SQL_IC_SENSITIVE: i32 = 3;
pub const SQL_IC_UPPER: i32 = 1;
pub const SQL_IDENTIFIER_CASE: i32 = 28;
pub const SQL_IDENTIFIER_QUOTE_CHAR: i32 = 29;
pub const SQL_INDEX_ALL: i32 = 1;
pub const SQL_INDEX_CLUSTERED: i32 = 1;
pub const SQL_INDEX_HASHED: i32 = 2;
pub const SQL_INDEX_OTHER: i32 = 3;
pub const SQL_INDEX_UNIQUE: i32 = 0;
pub const SQL_INSENSITIVE: i32 = 1;
pub const SQL_INTEGER: i32 = 4;
pub const SQL_INTEGRITY: i32 = 73;
pub const SQL_INVALID_HANDLE: i32 = -2;
pub const SQL_MAXIMUM_CATALOG_NAME_LENGTH: i32 = 34;
pub const SQL_MAXIMUM_COLUMNS_IN_GROUP_BY: i32 = 97;
pub const SQL_MAXIMUM_COLUMNS_IN_INDEX: i32 = 98;
pub const SQL_MAXIMUM_COLUMNS_IN_ORDER_BY: i32 = 99;
pub const SQL_MAXIMUM_COLUMNS_IN_SELECT: i32 = 100;
pub const SQL_MAXIMUM_COLUMN_NAME_LENGTH: i32 = 30;
pub const SQL_MAXIMUM_CONCURRENT_ACTIVITIES: i32 = 1;
pub const SQL_MAXIMUM_CURSOR_NAME_LENGTH: i32 = 31;
pub const SQL_MAXIMUM_DRIVER_CONNECTIONS: i32 = 0;
pub const SQL_MAXIMUM_IDENTIFIER_LENGTH: i32 = 10005;
pub const SQL_MAXIMUM_INDEX_SIZE: i32 = 102;
pub const SQL_MAXIMUM_ROW_SIZE: i32 = 104;
pub const SQL_MAXIMUM_SCHEMA_NAME_LENGTH: i32 = 32;
pub const SQL_MAXIMUM_STATEMENT_LENGTH: i32 = 105;
pub const SQL_MAXIMUM_TABLES_IN_SELECT: i32 = 106;
pub const SQL_MAXIMUM_USER_NAME_LENGTH: i32 = 107;
pub const SQL_MAX_CATALOG_NAME_LEN: i32 = 34;
pub const SQL_MAX_COLUMNS_IN_GROUP_BY: i32 = 97;
pub const SQL_MAX_COLUMNS_IN_INDEX: i32 = 98;
pub const SQL_MAX_COLUMNS_IN_ORDER_BY: i32 = 99;
pub const SQL_MAX_COLUMNS_IN_SELECT: i32 = 100;
pub const SQL_MAX_COLUMNS_IN_TABLE: i32 = 101;
pub const SQL_MAX_COLUMN_NAME_LEN: i32 = 30;
pub const SQL_MAX_CONCURRENT_ACTIVITIES: i32 = 1;
pub const SQL_MAX_CURSOR_NAME_LEN: i32 = 31;
pub const SQL_MAX_DRIVER_CONNECTIONS: i32 = 0;
pub const SQL_MAX_IDENTIFIER_LEN: i32 = 10005;
pub const SQL_MAX_INDEX_SIZE: i32 = 102;
pub const SQL_MAX_MESSAGE_LENGTH: i32 = 512;
pub const SQL_MAX_ROW_SIZE: i32 = 104;
pub const SQL_MAX_SCHEMA_NAME_LEN: i32 = 32;
pub const SQL_MAX_STATEMENT_LEN: i32 = 105;
pub const SQL_MAX_TABLES_IN_SELECT: i32 = 106;
pub const SQL_MAX_TABLE_NAME_LEN: i32 = 35;
pub const SQL_MAX_USER_NAME_LEN: i32 = 107;
pub const SQL_NAMED: i32 = 0;
pub const SQL_NC_HIGH: i32 = 0;
pub const SQL_NC_LOW: i32 = 1;
pub const SQL_NEED_DATA: i32 = 99;
pub const SQL_NONSCROLLABLE: i32 = 0;
pub const SQL_NO_DATA: i32 = 100;
pub const SQL_NO_NULLS: i32 = 0;
pub const SQL_NTS: i32 = -3;
pub const SQL_NTSL: i32 = -3;
pub const SQL_NULLABLE: i32 = 1;
pub const SQL_NULLABLE_UNKNOWN: i32 = 2;
pub const SQL_NULL_COLLATION: i32 = 85;
pub const SQL_NULL_DATA: i32 = -1;
pub const SQL_NULL_HANDLE: i32 = 0;
pub const SQL_NULL_HDBC: i32 = 0;
pub const SQL_NULL_HDESC: i32 = 0;
pub const SQL_NULL_HENV: i32 = 0;
pub const SQL_NULL_HSTMT: i32 = 0;
pub const SQL_NUMERIC: i32 = 2;
pub const SQL_OJ_ALL_COMPARISON_OPS: i32 = 64;
pub const SQL_OJ_CAPABILITIES: i32 = 115;
pub const SQL_OJ_FULL: i32 = 4;
pub const SQL_OJ_INNER: i32 = 32;
pub const SQL_OJ_LEFT: i32 = 1;
pub const SQL_OJ_NESTED: i32 = 8;
pub const SQL_OJ_NOT_ORDERED: i32 = 16;
pub const SQL_OJ_RIGHT: i32 = 2;
pub const SQL_ORDER_BY_COLUMNS_IN_SELECT: i32 = 90;
pub const SQL_OUTER_JOIN_CAPABILITIES: i32 = 115;
pub const SQL_PARAM_DATA_AVAILABLE: i32 = 101;
pub const SQL_PC_NON_PSEUDO: i32 = 1;
pub const SQL_PC_PSEUDO: i32 = 2;
pub const SQL_PC_UNKNOWN: i32 = 0;
pub const SQL_PRED_BASIC: i32 = 2;
pub const SQL_PRED_CHAR: i32 = 1;
pub const SQL_PRED_NONE: i32 = 0;
pub const SQL_REAL: i32 = 7;
pub const SQL_RESET_PARAMS: i32 = 3;
pub const SQL_ROLLBACK: i32 = 1;
pub const SQL_ROW_IDENTIFIER: i32 = 1;
pub const SQL_SCCO_LOCK: i32 = 2;
pub const SQL_SCCO_OPT_ROWVER: i32 = 4;
pub const SQL_SCCO_OPT_VALUES: i32 = 8;
pub const SQL_SCCO_READ_ONLY: i32 = 1;
pub const SQL_SCOPE_CURROW: i32 = 0;
pub const SQL_SCOPE_SESSION: i32 = 2;
pub const SQL_SCOPE_TRANSACTION: i32 = 1;
pub const SQL_SCROLLABLE: i32 = 1;
pub const SQL_SCROLL_CONCURRENCY: i32 = 43;
pub const SQL_SEARCH_PATTERN_ESCAPE: i32 = 14;
pub const SQL_SENSITIVE: i32 = 2;
pub const SQL_SERVER_NAME: i32 = 13;
pub const SQL_SMALLINT: i32 = 5;
pub const SQL_SPECIAL_CHARACTERS: i32 = 94;
pub const SQL_STILL_EXECUTING: i32 = 2;
pub const SQL_SUCCESS: i32 = 0;
pub const SQL_SUCCESS_WITH_INFO: i32 = 1;
pub const SQL_TC_ALL: i32 = 2;
pub const SQL_TC_DDL_COMMIT: i32 = 3;
pub const SQL_TC_DDL_IGNORE: i32 = 4;
pub const SQL_TC_DML: i32 = 1;
pub const SQL_TC_NONE: i32 = 0;
pub const SQL_TIMESTAMP_LEN: i32 = 19;
pub const SQL_TIME_LEN: i32 = 8;
pub const SQL_TRANSACTION_CAPABLE: i32 = 46;
pub const SQL_TRANSACTION_ISOLATION_OPTION: i32 = 72;
pub const SQL_TRANSACTION_READ_COMMITTED: i32 = 2;
pub const SQL_TRANSACTION_READ_UNCOMMITTED: i32 = 1;
pub const SQL_TRANSACTION_REPEATABLE_READ: i32 = 4;
pub const SQL_TRANSACTION_SERIALIZABLE: i32 = 8;
pub const SQL_TRUE: i32 = 1;
pub const SQL_TXN_CAPABLE: i32 = 46;
pub const SQL_TXN_ISOLATION_OPTION: i32 = 72;
pub const SQL_TXN_READ_COMMITTED: i32 = 2;
pub const SQL_TXN_READ_UNCOMMITTED: i32 = 1;
pub const SQL_TXN_REPEATABLE_READ: i32 = 4;
pub const SQL_TXN_SERIALIZABLE: i32 = 8;
pub const SQL_TYPE_DATE: i32 = 91;
pub const SQL_TYPE_TIME: i32 = 92;
pub const SQL_TYPE_TIMESTAMP: i32 = 93;
pub const SQL_UNBIND: i32 = 2;
pub const SQL_UNKNOWN_TYPE: i32 = 0;
pub const SQL_UNNAMED: i32 = 1;
pub const SQL_UNSPECIFIED: i32 = 0;
pub const SQL_USER_NAME: i32 = 47;
pub const SQL_VARCHAR: i32 = 12;
pub const SQL_XOPEN_CLI_YEAR: i32 = 10000;
