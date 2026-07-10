#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLAllocConnect(environmenthandle : super::sqltypes::SQLHENV, connectionhandle : *mut super::sqltypes::SQLHDBC) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLAllocEnv(environmenthandle : *mut super::sqltypes::SQLHENV) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLAllocHandle(handletype : super::sqltypes::SQLSMALLINT, inputhandle : super::sqltypes::SQLHANDLE, outputhandle : *mut super::sqltypes::SQLHANDLE) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLAllocStmt(connectionhandle : super::sqltypes::SQLHDBC, statementhandle : *mut super::sqltypes::SQLHSTMT) -> super::sqltypes::SQLRETURN);
#[cfg(target_arch = "x86")]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLBindCol(statementhandle : super::sqltypes::SQLHSTMT, columnnumber : super::sqltypes::SQLUSMALLINT, targettype : super::sqltypes::SQLSMALLINT, targetvalue : super::sqltypes::SQLPOINTER, bufferlength : super::sqltypes::SQLINTEGER, strlen_or_ind : *mut super::sqltypes::SQLINTEGER) -> super::sqltypes::SQLRETURN);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLBindCol(statementhandle : super::sqltypes::SQLHSTMT, columnnumber : super::sqltypes::SQLUSMALLINT, targettype : super::sqltypes::SQLSMALLINT, targetvalue : super::sqltypes::SQLPOINTER, bufferlength : super::sqltypes::SQLLEN, strlen_or_ind : *mut super::sqltypes::SQLLEN) -> super::sqltypes::SQLRETURN);
#[cfg(target_arch = "x86")]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLBindParam(statementhandle : super::sqltypes::SQLHSTMT, parameternumber : super::sqltypes::SQLUSMALLINT, valuetype : super::sqltypes::SQLSMALLINT, parametertype : super::sqltypes::SQLSMALLINT, lengthprecision : super::sqltypes::SQLUINTEGER, parameterscale : super::sqltypes::SQLSMALLINT, parametervalue : super::sqltypes::SQLPOINTER, strlen_or_ind : *mut super::sqltypes::SQLINTEGER) -> super::sqltypes::SQLRETURN);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLBindParam(statementhandle : super::sqltypes::SQLHSTMT, parameternumber : super::sqltypes::SQLUSMALLINT, valuetype : super::sqltypes::SQLSMALLINT, parametertype : super::sqltypes::SQLSMALLINT, lengthprecision : super::sqltypes::SQLULEN, parameterscale : super::sqltypes::SQLSMALLINT, parametervalue : super::sqltypes::SQLPOINTER, strlen_or_ind : *mut super::sqltypes::SQLLEN) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLCancel(statementhandle : super::sqltypes::SQLHSTMT) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLCancelHandle(handletype : super::sqltypes::SQLSMALLINT, inputhandle : super::sqltypes::SQLHANDLE) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLCloseCursor(statementhandle : super::sqltypes::SQLHSTMT) -> super::sqltypes::SQLRETURN);
#[cfg(target_arch = "x86")]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLColAttribute(statementhandle : super::sqltypes::SQLHSTMT, columnnumber : super::sqltypes::SQLUSMALLINT, fieldidentifier : super::sqltypes::SQLUSMALLINT, characterattribute : super::sqltypes::SQLPOINTER, bufferlength : super::sqltypes::SQLSMALLINT, stringlength : *mut super::sqltypes::SQLSMALLINT, numericattribute : super::sqltypes::SQLPOINTER) -> super::sqltypes::SQLRETURN);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLColAttribute(statementhandle : super::sqltypes::SQLHSTMT, columnnumber : super::sqltypes::SQLUSMALLINT, fieldidentifier : super::sqltypes::SQLUSMALLINT, characterattribute : super::sqltypes::SQLPOINTER, bufferlength : super::sqltypes::SQLSMALLINT, stringlength : *mut super::sqltypes::SQLSMALLINT, numericattribute : *mut super::sqltypes::SQLLEN) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLColumns(statementhandle : super::sqltypes::SQLHSTMT, catalogname : *const super::sqltypes::SQLCHAR, namelength1 : super::sqltypes::SQLSMALLINT, schemaname : *const super::sqltypes::SQLCHAR, namelength2 : super::sqltypes::SQLSMALLINT, tablename : *const super::sqltypes::SQLCHAR, namelength3 : super::sqltypes::SQLSMALLINT, columnname : *const super::sqltypes::SQLCHAR, namelength4 : super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLCompleteAsync(handletype : super::sqltypes::SQLSMALLINT, handle : super::sqltypes::SQLHANDLE, asyncretcodeptr : *mut super::sqltypes::RETCODE) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLConnect(connectionhandle : super::sqltypes::SQLHDBC, servername : *const super::sqltypes::SQLCHAR, namelength1 : super::sqltypes::SQLSMALLINT, username : *const super::sqltypes::SQLCHAR, namelength2 : super::sqltypes::SQLSMALLINT, authentication : *const super::sqltypes::SQLCHAR, namelength3 : super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLCopyDesc(sourcedeschandle : super::sqltypes::SQLHDESC, targetdeschandle : super::sqltypes::SQLHDESC) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLDataSources(environmenthandle : super::sqltypes::SQLHENV, direction : super::sqltypes::SQLUSMALLINT, servername : *mut super::sqltypes::SQLCHAR, bufferlength1 : super::sqltypes::SQLSMALLINT, namelength1ptr : *mut super::sqltypes::SQLSMALLINT, description : *mut super::sqltypes::SQLCHAR, bufferlength2 : super::sqltypes::SQLSMALLINT, namelength2ptr : *mut super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
#[cfg(target_arch = "x86")]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLDescribeCol(statementhandle : super::sqltypes::SQLHSTMT, columnnumber : super::sqltypes::SQLUSMALLINT, columnname : *mut super::sqltypes::SQLCHAR, bufferlength : super::sqltypes::SQLSMALLINT, namelength : *mut super::sqltypes::SQLSMALLINT, datatype : *mut super::sqltypes::SQLSMALLINT, columnsize : *mut super::sqltypes::SQLUINTEGER, decimaldigits : *mut super::sqltypes::SQLSMALLINT, nullable : *mut super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLDescribeCol(statementhandle : super::sqltypes::SQLHSTMT, columnnumber : super::sqltypes::SQLUSMALLINT, columnname : *mut super::sqltypes::SQLCHAR, bufferlength : super::sqltypes::SQLSMALLINT, namelength : *mut super::sqltypes::SQLSMALLINT, datatype : *mut super::sqltypes::SQLSMALLINT, columnsize : *mut super::sqltypes::SQLULEN, decimaldigits : *mut super::sqltypes::SQLSMALLINT, nullable : *mut super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLDisconnect(connectionhandle : super::sqltypes::SQLHDBC) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLEndTran(handletype : super::sqltypes::SQLSMALLINT, handle : super::sqltypes::SQLHANDLE, completiontype : super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLError(environmenthandle : super::sqltypes::SQLHENV, connectionhandle : super::sqltypes::SQLHDBC, statementhandle : super::sqltypes::SQLHSTMT, sqlstate : *mut super::sqltypes::SQLCHAR, nativeerror : *mut super::sqltypes::SQLINTEGER, messagetext : *mut super::sqltypes::SQLCHAR, bufferlength : super::sqltypes::SQLSMALLINT, textlength : *mut super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLExecDirect(statementhandle : super::sqltypes::SQLHSTMT, statementtext : *const super::sqltypes::SQLCHAR, textlength : super::sqltypes::SQLINTEGER) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLExecute(statementhandle : super::sqltypes::SQLHSTMT) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLFetch(statementhandle : super::sqltypes::SQLHSTMT) -> super::sqltypes::SQLRETURN);
#[cfg(target_arch = "x86")]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLFetchScroll(statementhandle : super::sqltypes::SQLHSTMT, fetchorientation : super::sqltypes::SQLSMALLINT, fetchoffset : super::sqltypes::SQLINTEGER) -> super::sqltypes::SQLRETURN);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLFetchScroll(statementhandle : super::sqltypes::SQLHSTMT, fetchorientation : super::sqltypes::SQLSMALLINT, fetchoffset : super::sqltypes::SQLLEN) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLFreeConnect(connectionhandle : super::sqltypes::SQLHDBC) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLFreeEnv(environmenthandle : super::sqltypes::SQLHENV) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLFreeHandle(handletype : super::sqltypes::SQLSMALLINT, handle : super::sqltypes::SQLHANDLE) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLFreeStmt(statementhandle : super::sqltypes::SQLHSTMT, option : super::sqltypes::SQLUSMALLINT) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLGetConnectAttr(connectionhandle : super::sqltypes::SQLHDBC, attribute : super::sqltypes::SQLINTEGER, value : super::sqltypes::SQLPOINTER, bufferlength : super::sqltypes::SQLINTEGER, stringlengthptr : *mut super::sqltypes::SQLINTEGER) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLGetConnectOption(connectionhandle : super::sqltypes::SQLHDBC, option : super::sqltypes::SQLUSMALLINT, value : super::sqltypes::SQLPOINTER) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLGetCursorName(statementhandle : super::sqltypes::SQLHSTMT, cursorname : *mut super::sqltypes::SQLCHAR, bufferlength : super::sqltypes::SQLSMALLINT, namelengthptr : *mut super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
#[cfg(target_arch = "x86")]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLGetData(statementhandle : super::sqltypes::SQLHSTMT, columnnumber : super::sqltypes::SQLUSMALLINT, targettype : super::sqltypes::SQLSMALLINT, targetvalue : super::sqltypes::SQLPOINTER, bufferlength : super::sqltypes::SQLINTEGER, strlen_or_indptr : *mut super::sqltypes::SQLINTEGER) -> super::sqltypes::SQLRETURN);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLGetData(statementhandle : super::sqltypes::SQLHSTMT, columnnumber : super::sqltypes::SQLUSMALLINT, targettype : super::sqltypes::SQLSMALLINT, targetvalue : super::sqltypes::SQLPOINTER, bufferlength : super::sqltypes::SQLLEN, strlen_or_indptr : *mut super::sqltypes::SQLLEN) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLGetDescField(descriptorhandle : super::sqltypes::SQLHDESC, recnumber : super::sqltypes::SQLSMALLINT, fieldidentifier : super::sqltypes::SQLSMALLINT, value : super::sqltypes::SQLPOINTER, bufferlength : super::sqltypes::SQLINTEGER, stringlength : *mut super::sqltypes::SQLINTEGER) -> super::sqltypes::SQLRETURN);
#[cfg(target_arch = "x86")]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLGetDescRec(descriptorhandle : super::sqltypes::SQLHDESC, recnumber : super::sqltypes::SQLSMALLINT, name : *mut super::sqltypes::SQLCHAR, bufferlength : super::sqltypes::SQLSMALLINT, stringlengthptr : *mut super::sqltypes::SQLSMALLINT, typeptr : *mut super::sqltypes::SQLSMALLINT, subtypeptr : *mut super::sqltypes::SQLSMALLINT, lengthptr : *mut super::sqltypes::SQLINTEGER, precisionptr : *mut super::sqltypes::SQLSMALLINT, scaleptr : *mut super::sqltypes::SQLSMALLINT, nullableptr : *mut super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLGetDescRec(descriptorhandle : super::sqltypes::SQLHDESC, recnumber : super::sqltypes::SQLSMALLINT, name : *mut super::sqltypes::SQLCHAR, bufferlength : super::sqltypes::SQLSMALLINT, stringlengthptr : *mut super::sqltypes::SQLSMALLINT, typeptr : *mut super::sqltypes::SQLSMALLINT, subtypeptr : *mut super::sqltypes::SQLSMALLINT, lengthptr : *mut super::sqltypes::SQLLEN, precisionptr : *mut super::sqltypes::SQLSMALLINT, scaleptr : *mut super::sqltypes::SQLSMALLINT, nullableptr : *mut super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLGetDiagField(handletype : super::sqltypes::SQLSMALLINT, handle : super::sqltypes::SQLHANDLE, recnumber : super::sqltypes::SQLSMALLINT, diagidentifier : super::sqltypes::SQLSMALLINT, diaginfo : super::sqltypes::SQLPOINTER, bufferlength : super::sqltypes::SQLSMALLINT, stringlength : *mut super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLGetDiagRec(handletype : super::sqltypes::SQLSMALLINT, handle : super::sqltypes::SQLHANDLE, recnumber : super::sqltypes::SQLSMALLINT, sqlstate : *mut super::sqltypes::SQLCHAR, nativeerror : *mut super::sqltypes::SQLINTEGER, messagetext : *mut super::sqltypes::SQLCHAR, bufferlength : super::sqltypes::SQLSMALLINT, textlength : *mut super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLGetEnvAttr(environmenthandle : super::sqltypes::SQLHENV, attribute : super::sqltypes::SQLINTEGER, value : super::sqltypes::SQLPOINTER, bufferlength : super::sqltypes::SQLINTEGER, stringlength : *mut super::sqltypes::SQLINTEGER) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLGetFunctions(connectionhandle : super::sqltypes::SQLHDBC, functionid : super::sqltypes::SQLUSMALLINT, supported : *mut super::sqltypes::SQLUSMALLINT) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLGetInfo(connectionhandle : super::sqltypes::SQLHDBC, infotype : super::sqltypes::SQLUSMALLINT, infovalue : super::sqltypes::SQLPOINTER, bufferlength : super::sqltypes::SQLSMALLINT, stringlengthptr : *mut super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLGetStmtAttr(statementhandle : super::sqltypes::SQLHSTMT, attribute : super::sqltypes::SQLINTEGER, value : super::sqltypes::SQLPOINTER, bufferlength : super::sqltypes::SQLINTEGER, stringlength : *mut super::sqltypes::SQLINTEGER) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLGetStmtOption(statementhandle : super::sqltypes::SQLHSTMT, option : super::sqltypes::SQLUSMALLINT, value : super::sqltypes::SQLPOINTER) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLGetTypeInfo(statementhandle : super::sqltypes::SQLHSTMT, datatype : super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLNumResultCols(statementhandle : super::sqltypes::SQLHSTMT, columncount : *mut super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLParamData(statementhandle : super::sqltypes::SQLHSTMT, value : *mut super::sqltypes::SQLPOINTER) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLPrepare(statementhandle : super::sqltypes::SQLHSTMT, statementtext : *const super::sqltypes::SQLCHAR, textlength : super::sqltypes::SQLINTEGER) -> super::sqltypes::SQLRETURN);
#[cfg(target_arch = "x86")]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLPutData(statementhandle : super::sqltypes::SQLHSTMT, data : super::sqltypes::SQLPOINTER, strlen_or_ind : super::sqltypes::SQLINTEGER) -> super::sqltypes::SQLRETURN);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLPutData(statementhandle : super::sqltypes::SQLHSTMT, data : super::sqltypes::SQLPOINTER, strlen_or_ind : super::sqltypes::SQLLEN) -> super::sqltypes::SQLRETURN);
#[cfg(target_arch = "x86")]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLRowCount(statementhandle : super::sqltypes::SQLHSTMT, rowcount : *mut super::sqltypes::SQLINTEGER) -> super::sqltypes::SQLRETURN);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLRowCount(statementhandle : super::sqltypes::SQLHSTMT, rowcount : *mut super::sqltypes::SQLLEN) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLSetConnectAttr(connectionhandle : super::sqltypes::SQLHDBC, attribute : super::sqltypes::SQLINTEGER, value : super::sqltypes::SQLPOINTER, stringlength : super::sqltypes::SQLINTEGER) -> super::sqltypes::SQLRETURN);
#[cfg(target_arch = "x86")]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLSetConnectOption(connectionhandle : super::sqltypes::SQLHDBC, option : super::sqltypes::SQLUSMALLINT, value : super::sqltypes::SQLUINTEGER) -> super::sqltypes::SQLRETURN);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLSetConnectOption(connectionhandle : super::sqltypes::SQLHDBC, option : super::sqltypes::SQLUSMALLINT, value : super::sqltypes::SQLULEN) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLSetCursorName(statementhandle : super::sqltypes::SQLHSTMT, cursorname : *const super::sqltypes::SQLCHAR, namelength : super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLSetDescField(descriptorhandle : super::sqltypes::SQLHDESC, recnumber : super::sqltypes::SQLSMALLINT, fieldidentifier : super::sqltypes::SQLSMALLINT, value : super::sqltypes::SQLPOINTER, bufferlength : super::sqltypes::SQLINTEGER) -> super::sqltypes::SQLRETURN);
#[cfg(target_arch = "x86")]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLSetDescRec(descriptorhandle : super::sqltypes::SQLHDESC, recnumber : super::sqltypes::SQLSMALLINT, r#type : super::sqltypes::SQLSMALLINT, subtype : super::sqltypes::SQLSMALLINT, length : super::sqltypes::SQLINTEGER, precision : super::sqltypes::SQLSMALLINT, scale : super::sqltypes::SQLSMALLINT, data : super::sqltypes::SQLPOINTER, stringlength : *mut super::sqltypes::SQLINTEGER, indicator : *mut super::sqltypes::SQLINTEGER) -> super::sqltypes::SQLRETURN);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLSetDescRec(descriptorhandle : super::sqltypes::SQLHDESC, recnumber : super::sqltypes::SQLSMALLINT, r#type : super::sqltypes::SQLSMALLINT, subtype : super::sqltypes::SQLSMALLINT, length : super::sqltypes::SQLLEN, precision : super::sqltypes::SQLSMALLINT, scale : super::sqltypes::SQLSMALLINT, data : super::sqltypes::SQLPOINTER, stringlength : *mut super::sqltypes::SQLLEN, indicator : *mut super::sqltypes::SQLLEN) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLSetEnvAttr(environmenthandle : super::sqltypes::SQLHENV, attribute : super::sqltypes::SQLINTEGER, value : super::sqltypes::SQLPOINTER, stringlength : super::sqltypes::SQLINTEGER) -> super::sqltypes::SQLRETURN);
#[cfg(target_arch = "x86")]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLSetParam(statementhandle : super::sqltypes::SQLHSTMT, parameternumber : super::sqltypes::SQLUSMALLINT, valuetype : super::sqltypes::SQLSMALLINT, parametertype : super::sqltypes::SQLSMALLINT, lengthprecision : super::sqltypes::SQLUINTEGER, parameterscale : super::sqltypes::SQLSMALLINT, parametervalue : super::sqltypes::SQLPOINTER, strlen_or_ind : *mut super::sqltypes::SQLINTEGER) -> super::sqltypes::SQLRETURN);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLSetParam(statementhandle : super::sqltypes::SQLHSTMT, parameternumber : super::sqltypes::SQLUSMALLINT, valuetype : super::sqltypes::SQLSMALLINT, parametertype : super::sqltypes::SQLSMALLINT, lengthprecision : super::sqltypes::SQLULEN, parameterscale : super::sqltypes::SQLSMALLINT, parametervalue : super::sqltypes::SQLPOINTER, strlen_or_ind : *mut super::sqltypes::SQLLEN) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLSetStmtAttr(statementhandle : super::sqltypes::SQLHSTMT, attribute : super::sqltypes::SQLINTEGER, value : super::sqltypes::SQLPOINTER, stringlength : super::sqltypes::SQLINTEGER) -> super::sqltypes::SQLRETURN);
#[cfg(target_arch = "x86")]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLSetStmtOption(statementhandle : super::sqltypes::SQLHSTMT, option : super::sqltypes::SQLUSMALLINT, value : super::sqltypes::SQLUINTEGER) -> super::sqltypes::SQLRETURN);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLSetStmtOption(statementhandle : super::sqltypes::SQLHSTMT, option : super::sqltypes::SQLUSMALLINT, value : super::sqltypes::SQLULEN) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLSpecialColumns(statementhandle : super::sqltypes::SQLHSTMT, identifiertype : super::sqltypes::SQLUSMALLINT, catalogname : *const super::sqltypes::SQLCHAR, namelength1 : super::sqltypes::SQLSMALLINT, schemaname : *const super::sqltypes::SQLCHAR, namelength2 : super::sqltypes::SQLSMALLINT, tablename : *const super::sqltypes::SQLCHAR, namelength3 : super::sqltypes::SQLSMALLINT, scope : super::sqltypes::SQLUSMALLINT, nullable : super::sqltypes::SQLUSMALLINT) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLStatistics(statementhandle : super::sqltypes::SQLHSTMT, catalogname : *const super::sqltypes::SQLCHAR, namelength1 : super::sqltypes::SQLSMALLINT, schemaname : *const super::sqltypes::SQLCHAR, namelength2 : super::sqltypes::SQLSMALLINT, tablename : *const super::sqltypes::SQLCHAR, namelength3 : super::sqltypes::SQLSMALLINT, unique : super::sqltypes::SQLUSMALLINT, reserved : super::sqltypes::SQLUSMALLINT) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLTables(statementhandle : super::sqltypes::SQLHSTMT, catalogname : *const super::sqltypes::SQLCHAR, namelength1 : super::sqltypes::SQLSMALLINT, schemaname : *const super::sqltypes::SQLCHAR, namelength2 : super::sqltypes::SQLSMALLINT, tablename : *const super::sqltypes::SQLCHAR, namelength3 : super::sqltypes::SQLSMALLINT, tabletype : *const super::sqltypes::SQLCHAR, namelength4 : super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLTransact(environmenthandle : super::sqltypes::SQLHENV, connectionhandle : super::sqltypes::SQLHDBC, completiontype : super::sqltypes::SQLUSMALLINT) -> super::sqltypes::SQLRETURN);
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
