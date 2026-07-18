#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLBrowseConnectA(hdbc : super::SQLHDBC, szconnstrin : *const super::SQLCHAR, cbconnstrin : super::SQLSMALLINT, szconnstrout : *mut super::SQLCHAR, cbconnstroutmax : super::SQLSMALLINT, pcbconnstrout : *mut super::SQLSMALLINT) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLBrowseConnectW(hdbc : super::SQLHDBC, szconnstrin : *const super::SQLWCHAR, cchconnstrin : super::SQLSMALLINT, szconnstrout : *mut super::SQLWCHAR, cchconnstroutmax : super::SQLSMALLINT, pcchconnstrout : *mut super::SQLSMALLINT) -> super::SQLRETURN);
#[cfg(target_arch = "x86")]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLColAttributeA(hstmt : super::SQLHSTMT, icol : super::SQLSMALLINT, ifield : super::SQLSMALLINT, pcharattr : super::SQLPOINTER, cbcharattrmax : super::SQLSMALLINT, pcbcharattr : *mut super::SQLSMALLINT, pnumattr : super::SQLPOINTER) -> super::SQLRETURN);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLColAttributeA(hstmt : super::SQLHSTMT, icol : super::SQLSMALLINT, ifield : super::SQLSMALLINT, pcharattr : super::SQLPOINTER, cbcharattrmax : super::SQLSMALLINT, pcbcharattr : *mut super::SQLSMALLINT, pnumattr : *mut super::SQLLEN) -> super::SQLRETURN);
#[cfg(target_arch = "x86")]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLColAttributeW(hstmt : super::SQLHSTMT, icol : super::SQLUSMALLINT, ifield : super::SQLUSMALLINT, pcharattr : super::SQLPOINTER, cbdescmax : super::SQLSMALLINT, pcbcharattr : *mut super::SQLSMALLINT, pnumattr : super::SQLPOINTER) -> super::SQLRETURN);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLColAttributeW(hstmt : super::SQLHSTMT, icol : super::SQLUSMALLINT, ifield : super::SQLUSMALLINT, pcharattr : super::SQLPOINTER, cbdescmax : super::SQLSMALLINT, pcbcharattr : *mut super::SQLSMALLINT, pnumattr : *mut super::SQLLEN) -> super::SQLRETURN);
#[cfg(target_arch = "x86")]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLColAttributesA(hstmt : super::SQLHSTMT, icol : super::SQLUSMALLINT, fdesctype : super::SQLUSMALLINT, rgbdesc : super::SQLPOINTER, cbdescmax : super::SQLSMALLINT, pcbdesc : *mut super::SQLSMALLINT, pfdesc : *mut super::SQLINTEGER) -> super::SQLRETURN);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLColAttributesA(hstmt : super::SQLHSTMT, icol : super::SQLUSMALLINT, fdesctype : super::SQLUSMALLINT, rgbdesc : super::SQLPOINTER, cbdescmax : super::SQLSMALLINT, pcbdesc : *mut super::SQLSMALLINT, pfdesc : *mut super::SQLLEN) -> super::SQLRETURN);
#[cfg(target_arch = "x86")]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLColAttributesW(hstmt : super::SQLHSTMT, icol : super::SQLUSMALLINT, fdesctype : super::SQLUSMALLINT, rgbdesc : super::SQLPOINTER, cbdescmax : super::SQLSMALLINT, pcbdesc : *mut super::SQLSMALLINT, pfdesc : *mut super::SQLINTEGER) -> super::SQLRETURN);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLColAttributesW(hstmt : super::SQLHSTMT, icol : super::SQLUSMALLINT, fdesctype : super::SQLUSMALLINT, rgbdesc : super::SQLPOINTER, cbdescmax : super::SQLSMALLINT, pcbdesc : *mut super::SQLSMALLINT, pfdesc : *mut super::SQLLEN) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLColumnPrivilegesA(hstmt : super::SQLHSTMT, szcatalogname : *const super::SQLCHAR, cbcatalogname : super::SQLSMALLINT, szschemaname : *const super::SQLCHAR, cbschemaname : super::SQLSMALLINT, sztablename : *const super::SQLCHAR, cbtablename : super::SQLSMALLINT, szcolumnname : *const super::SQLCHAR, cbcolumnname : super::SQLSMALLINT) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLColumnPrivilegesW(hstmt : super::SQLHSTMT, szcatalogname : *const super::SQLWCHAR, cchcatalogname : super::SQLSMALLINT, szschemaname : *const super::SQLWCHAR, cchschemaname : super::SQLSMALLINT, sztablename : *const super::SQLWCHAR, cchtablename : super::SQLSMALLINT, szcolumnname : *const super::SQLWCHAR, cchcolumnname : super::SQLSMALLINT) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLColumnsA(hstmt : super::SQLHSTMT, szcatalogname : *const super::SQLCHAR, cbcatalogname : super::SQLSMALLINT, szschemaname : *const super::SQLCHAR, cbschemaname : super::SQLSMALLINT, sztablename : *const super::SQLCHAR, cbtablename : super::SQLSMALLINT, szcolumnname : *const super::SQLCHAR, cbcolumnname : super::SQLSMALLINT) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLColumnsW(hstmt : super::SQLHSTMT, szcatalogname : *const super::SQLWCHAR, cchcatalogname : super::SQLSMALLINT, szschemaname : *const super::SQLWCHAR, cchschemaname : super::SQLSMALLINT, sztablename : *const super::SQLWCHAR, cchtablename : super::SQLSMALLINT, szcolumnname : *const super::SQLWCHAR, cchcolumnname : super::SQLSMALLINT) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLConnectA(hdbc : super::SQLHDBC, szdsn : *const super::SQLCHAR, cbdsn : super::SQLSMALLINT, szuid : *const super::SQLCHAR, cbuid : super::SQLSMALLINT, szauthstr : *const super::SQLCHAR, cbauthstr : super::SQLSMALLINT) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLConnectW(hdbc : super::SQLHDBC, szdsn : *const super::SQLWCHAR, cchdsn : super::SQLSMALLINT, szuid : *const super::SQLWCHAR, cchuid : super::SQLSMALLINT, szauthstr : *const super::SQLWCHAR, cchauthstr : super::SQLSMALLINT) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLDataSourcesA(henv : super::SQLHENV, fdirection : super::SQLUSMALLINT, szdsn : *mut super::SQLCHAR, cbdsnmax : super::SQLSMALLINT, pcbdsn : *mut super::SQLSMALLINT, szdescription : *mut super::SQLCHAR, cbdescriptionmax : super::SQLSMALLINT, pcbdescription : *mut super::SQLSMALLINT) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLDataSourcesW(henv : super::SQLHENV, fdirection : super::SQLUSMALLINT, szdsn : *mut super::SQLWCHAR, cchdsnmax : super::SQLSMALLINT, pcchdsn : *mut super::SQLSMALLINT, wszdescription : *mut super::SQLWCHAR, cchdescriptionmax : super::SQLSMALLINT, pcchdescription : *mut super::SQLSMALLINT) -> super::SQLRETURN);
#[cfg(target_arch = "x86")]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLDescribeColA(hstmt : super::SQLHSTMT, icol : super::SQLUSMALLINT, szcolname : *mut super::SQLCHAR, cbcolnamemax : super::SQLSMALLINT, pcbcolname : *mut super::SQLSMALLINT, pfsqltype : *mut super::SQLSMALLINT, pcbcoldef : *mut super::SQLUINTEGER, pibscale : *mut super::SQLSMALLINT, pfnullable : *mut super::SQLSMALLINT) -> super::SQLRETURN);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLDescribeColA(hstmt : super::SQLHSTMT, icol : super::SQLUSMALLINT, szcolname : *mut super::SQLCHAR, cbcolnamemax : super::SQLSMALLINT, pcbcolname : *mut super::SQLSMALLINT, pfsqltype : *mut super::SQLSMALLINT, pcbcoldef : *mut super::SQLULEN, pibscale : *mut super::SQLSMALLINT, pfnullable : *mut super::SQLSMALLINT) -> super::SQLRETURN);
#[cfg(target_arch = "x86")]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLDescribeColW(hstmt : super::SQLHSTMT, icol : super::SQLUSMALLINT, szcolname : *mut super::SQLWCHAR, cchcolnamemax : super::SQLSMALLINT, pcchcolname : *mut super::SQLSMALLINT, pfsqltype : *mut super::SQLSMALLINT, pcbcoldef : *mut super::SQLUINTEGER, pibscale : *mut super::SQLSMALLINT, pfnullable : *mut super::SQLSMALLINT) -> super::SQLRETURN);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLDescribeColW(hstmt : super::SQLHSTMT, icol : super::SQLUSMALLINT, szcolname : *mut super::SQLWCHAR, cchcolnamemax : super::SQLSMALLINT, pcchcolname : *mut super::SQLSMALLINT, pfsqltype : *mut super::SQLSMALLINT, pcbcoldef : *mut super::SQLULEN, pibscale : *mut super::SQLSMALLINT, pfnullable : *mut super::SQLSMALLINT) -> super::SQLRETURN);
#[cfg(all(feature = "sqltypes", feature = "windef"))]
windows_link::link!("odbc32.dll" "system" fn SQLDriverConnectA(hdbc : super::SQLHDBC, hwnd : super::SQLHWND, szconnstrin : *const super::SQLCHAR, cbconnstrin : super::SQLSMALLINT, szconnstrout : *mut super::SQLCHAR, cbconnstroutmax : super::SQLSMALLINT, pcbconnstrout : *mut super::SQLSMALLINT, fdrivercompletion : super::SQLUSMALLINT) -> super::SQLRETURN);
#[cfg(all(feature = "sqltypes", feature = "windef"))]
windows_link::link!("odbc32.dll" "system" fn SQLDriverConnectW(hdbc : super::SQLHDBC, hwnd : super::SQLHWND, szconnstrin : *const super::SQLWCHAR, cchconnstrin : super::SQLSMALLINT, szconnstrout : *mut super::SQLWCHAR, cchconnstroutmax : super::SQLSMALLINT, pcchconnstrout : *mut super::SQLSMALLINT, fdrivercompletion : super::SQLUSMALLINT) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLDriversA(henv : super::SQLHENV, fdirection : super::SQLUSMALLINT, szdriverdesc : *mut super::SQLCHAR, cbdriverdescmax : super::SQLSMALLINT, pcbdriverdesc : *mut super::SQLSMALLINT, szdriverattributes : *mut super::SQLCHAR, cbdrvrattrmax : super::SQLSMALLINT, pcbdrvrattr : *mut super::SQLSMALLINT) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLDriversW(henv : super::SQLHENV, fdirection : super::SQLUSMALLINT, szdriverdesc : *mut super::SQLWCHAR, cchdriverdescmax : super::SQLSMALLINT, pcchdriverdesc : *mut super::SQLSMALLINT, szdriverattributes : *mut super::SQLWCHAR, cchdrvrattrmax : super::SQLSMALLINT, pcchdrvrattr : *mut super::SQLSMALLINT) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLErrorA(henv : super::SQLHENV, hdbc : super::SQLHDBC, hstmt : super::SQLHSTMT, szsqlstate : *mut super::SQLCHAR, pfnativeerror : *mut super::SQLINTEGER, szerrormsg : *mut super::SQLCHAR, cberrormsgmax : super::SQLSMALLINT, pcberrormsg : *mut super::SQLSMALLINT) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLErrorW(henv : super::SQLHENV, hdbc : super::SQLHDBC, hstmt : super::SQLHSTMT, wszsqlstate : *mut super::SQLWCHAR, pfnativeerror : *mut super::SQLINTEGER, wszerrormsg : *mut super::SQLWCHAR, ccherrormsgmax : super::SQLSMALLINT, pccherrormsg : *mut super::SQLSMALLINT) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLExecDirectA(hstmt : super::SQLHSTMT, szsqlstr : *const super::SQLCHAR, cbsqlstr : super::SQLINTEGER) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLExecDirectW(hstmt : super::SQLHSTMT, szsqlstr : *const super::SQLWCHAR, textlength : super::SQLINTEGER) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLForeignKeysA(hstmt : super::SQLHSTMT, szpkcatalogname : *const super::SQLCHAR, cbpkcatalogname : super::SQLSMALLINT, szpkschemaname : *const super::SQLCHAR, cbpkschemaname : super::SQLSMALLINT, szpktablename : *const super::SQLCHAR, cbpktablename : super::SQLSMALLINT, szfkcatalogname : *const super::SQLCHAR, cbfkcatalogname : super::SQLSMALLINT, szfkschemaname : *const super::SQLCHAR, cbfkschemaname : super::SQLSMALLINT, szfktablename : *const super::SQLCHAR, cbfktablename : super::SQLSMALLINT) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLForeignKeysW(hstmt : super::SQLHSTMT, szpkcatalogname : *const super::SQLWCHAR, cchpkcatalogname : super::SQLSMALLINT, szpkschemaname : *const super::SQLWCHAR, cchpkschemaname : super::SQLSMALLINT, szpktablename : *const super::SQLWCHAR, cchpktablename : super::SQLSMALLINT, szfkcatalogname : *const super::SQLWCHAR, cchfkcatalogname : super::SQLSMALLINT, szfkschemaname : *const super::SQLWCHAR, cchfkschemaname : super::SQLSMALLINT, szfktablename : *const super::SQLWCHAR, cchfktablename : super::SQLSMALLINT) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLGetConnectAttrA(hdbc : super::SQLHDBC, fattribute : super::SQLINTEGER, rgbvalue : super::SQLPOINTER, cbvaluemax : super::SQLINTEGER, pcbvalue : *mut super::SQLINTEGER) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLGetConnectAttrW(hdbc : super::SQLHDBC, fattribute : super::SQLINTEGER, rgbvalue : super::SQLPOINTER, cbvaluemax : super::SQLINTEGER, pcbvalue : *mut super::SQLINTEGER) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLGetConnectOptionA(hdbc : super::SQLHDBC, foption : super::SQLUSMALLINT, pvparam : super::SQLPOINTER) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLGetConnectOptionW(hdbc : super::SQLHDBC, foption : super::SQLUSMALLINT, pvparam : super::SQLPOINTER) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLGetCursorNameA(hstmt : super::SQLHSTMT, szcursor : *mut super::SQLCHAR, cbcursormax : super::SQLSMALLINT, pcbcursor : *mut super::SQLSMALLINT) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLGetCursorNameW(hstmt : super::SQLHSTMT, szcursor : *mut super::SQLWCHAR, cchcursormax : super::SQLSMALLINT, pcchcursor : *mut super::SQLSMALLINT) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLGetDescFieldA(hdesc : super::SQLHDESC, irecord : super::SQLSMALLINT, ifield : super::SQLSMALLINT, rgbvalue : super::SQLPOINTER, cbbufferlength : super::SQLINTEGER, stringlength : *mut super::SQLINTEGER) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLGetDescFieldW(hdesc : super::SQLHDESC, irecord : super::SQLSMALLINT, ifield : super::SQLSMALLINT, rgbvalue : super::SQLPOINTER, cbbufferlength : super::SQLINTEGER, stringlength : *mut super::SQLINTEGER) -> super::SQLRETURN);
#[cfg(target_arch = "x86")]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLGetDescRecA(hdesc : super::SQLHDESC, irecord : super::SQLSMALLINT, szname : *mut super::SQLCHAR, cbnamemax : super::SQLSMALLINT, pcbname : *mut super::SQLSMALLINT, pftype : *mut super::SQLSMALLINT, pfsubtype : *mut super::SQLSMALLINT, plength : *mut super::SQLINTEGER, pprecision : *mut super::SQLSMALLINT, pscale : *mut super::SQLSMALLINT, pnullable : *mut super::SQLSMALLINT) -> super::SQLRETURN);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLGetDescRecA(hdesc : super::SQLHDESC, irecord : super::SQLSMALLINT, szname : *mut super::SQLCHAR, cbnamemax : super::SQLSMALLINT, pcbname : *mut super::SQLSMALLINT, pftype : *mut super::SQLSMALLINT, pfsubtype : *mut super::SQLSMALLINT, plength : *mut super::SQLLEN, pprecision : *mut super::SQLSMALLINT, pscale : *mut super::SQLSMALLINT, pnullable : *mut super::SQLSMALLINT) -> super::SQLRETURN);
#[cfg(target_arch = "x86")]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLGetDescRecW(hdesc : super::SQLHDESC, irecord : super::SQLSMALLINT, szname : *mut super::SQLWCHAR, cchnamemax : super::SQLSMALLINT, pcchname : *mut super::SQLSMALLINT, pftype : *mut super::SQLSMALLINT, pfsubtype : *mut super::SQLSMALLINT, plength : *mut super::SQLINTEGER, pprecision : *mut super::SQLSMALLINT, pscale : *mut super::SQLSMALLINT, pnullable : *mut super::SQLSMALLINT) -> super::SQLRETURN);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLGetDescRecW(hdesc : super::SQLHDESC, irecord : super::SQLSMALLINT, szname : *mut super::SQLWCHAR, cchnamemax : super::SQLSMALLINT, pcchname : *mut super::SQLSMALLINT, pftype : *mut super::SQLSMALLINT, pfsubtype : *mut super::SQLSMALLINT, plength : *mut super::SQLLEN, pprecision : *mut super::SQLSMALLINT, pscale : *mut super::SQLSMALLINT, pnullable : *mut super::SQLSMALLINT) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLGetDiagFieldA(fhandletype : super::SQLSMALLINT, handle : super::SQLHANDLE, irecord : super::SQLSMALLINT, fdiagfield : super::SQLSMALLINT, rgbdiaginfo : super::SQLPOINTER, cbdiaginfomax : super::SQLSMALLINT, pcbdiaginfo : *mut super::SQLSMALLINT) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLGetDiagFieldW(fhandletype : super::SQLSMALLINT, handle : super::SQLHANDLE, irecord : super::SQLSMALLINT, fdiagfield : super::SQLSMALLINT, rgbdiaginfo : super::SQLPOINTER, cbbufferlength : super::SQLSMALLINT, pcbstringlength : *mut super::SQLSMALLINT) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLGetDiagRecA(fhandletype : super::SQLSMALLINT, handle : super::SQLHANDLE, irecord : super::SQLSMALLINT, szsqlstate : *mut super::SQLCHAR, pfnativeerror : *mut super::SQLINTEGER, szerrormsg : *mut super::SQLCHAR, cberrormsgmax : super::SQLSMALLINT, pcberrormsg : *mut super::SQLSMALLINT) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLGetDiagRecW(fhandletype : super::SQLSMALLINT, handle : super::SQLHANDLE, irecord : super::SQLSMALLINT, szsqlstate : *mut super::SQLWCHAR, pfnativeerror : *mut super::SQLINTEGER, szerrormsg : *mut super::SQLWCHAR, ccherrormsgmax : super::SQLSMALLINT, pccherrormsg : *mut super::SQLSMALLINT) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLGetInfoA(hdbc : super::SQLHDBC, finfotype : super::SQLUSMALLINT, rgbinfovalue : super::SQLPOINTER, cbinfovaluemax : super::SQLSMALLINT, pcbinfovalue : *mut super::SQLSMALLINT) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLGetInfoW(hdbc : super::SQLHDBC, finfotype : super::SQLUSMALLINT, rgbinfovalue : super::SQLPOINTER, cbinfovaluemax : super::SQLSMALLINT, pcbinfovalue : *mut super::SQLSMALLINT) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLGetStmtAttrA(hstmt : super::SQLHSTMT, fattribute : super::SQLINTEGER, rgbvalue : super::SQLPOINTER, cbvaluemax : super::SQLINTEGER, pcbvalue : *mut super::SQLINTEGER) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLGetStmtAttrW(hstmt : super::SQLHSTMT, fattribute : super::SQLINTEGER, rgbvalue : super::SQLPOINTER, cbvaluemax : super::SQLINTEGER, pcbvalue : *mut super::SQLINTEGER) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLGetTypeInfoA(statementhandle : super::SQLHSTMT, datatype : super::SQLSMALLINT) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLGetTypeInfoW(statementhandle : super::SQLHSTMT, datatype : super::SQLSMALLINT) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLNativeSqlA(hdbc : super::SQLHDBC, szsqlstrin : *const super::SQLCHAR, cbsqlstrin : super::SQLINTEGER, szsqlstr : *mut super::SQLCHAR, cbsqlstrmax : super::SQLINTEGER, pcbsqlstr : *mut super::SQLINTEGER) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLNativeSqlW(hdbc : super::SQLHDBC, szsqlstrin : *const super::SQLWCHAR, cchsqlstrin : super::SQLINTEGER, szsqlstr : *mut super::SQLWCHAR, cchsqlstrmax : super::SQLINTEGER, pcchsqlstr : *mut super::SQLINTEGER) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLPrepareA(hstmt : super::SQLHSTMT, szsqlstr : *const super::SQLCHAR, cbsqlstr : super::SQLINTEGER) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLPrepareW(hstmt : super::SQLHSTMT, szsqlstr : *const super::SQLWCHAR, cchsqlstr : super::SQLINTEGER) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLPrimaryKeysA(hstmt : super::SQLHSTMT, szcatalogname : *const super::SQLCHAR, cbcatalogname : super::SQLSMALLINT, szschemaname : *const super::SQLCHAR, cbschemaname : super::SQLSMALLINT, sztablename : *const super::SQLCHAR, cbtablename : super::SQLSMALLINT) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLPrimaryKeysW(hstmt : super::SQLHSTMT, szcatalogname : *const super::SQLWCHAR, cchcatalogname : super::SQLSMALLINT, szschemaname : *const super::SQLWCHAR, cchschemaname : super::SQLSMALLINT, sztablename : *const super::SQLWCHAR, cchtablename : super::SQLSMALLINT) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLProcedureColumnsA(hstmt : super::SQLHSTMT, szcatalogname : *const super::SQLCHAR, cbcatalogname : super::SQLSMALLINT, szschemaname : *const super::SQLCHAR, cbschemaname : super::SQLSMALLINT, szprocname : *const super::SQLCHAR, cbprocname : super::SQLSMALLINT, szcolumnname : *const super::SQLCHAR, cbcolumnname : super::SQLSMALLINT) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLProcedureColumnsW(hstmt : super::SQLHSTMT, szcatalogname : *const super::SQLWCHAR, cchcatalogname : super::SQLSMALLINT, szschemaname : *const super::SQLWCHAR, cchschemaname : super::SQLSMALLINT, szprocname : *const super::SQLWCHAR, cchprocname : super::SQLSMALLINT, szcolumnname : *const super::SQLWCHAR, cchcolumnname : super::SQLSMALLINT) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLProceduresA(hstmt : super::SQLHSTMT, szcatalogname : *const super::SQLCHAR, cbcatalogname : super::SQLSMALLINT, szschemaname : *const super::SQLCHAR, cbschemaname : super::SQLSMALLINT, szprocname : *const super::SQLCHAR, cbprocname : super::SQLSMALLINT) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLProceduresW(hstmt : super::SQLHSTMT, szcatalogname : *const super::SQLWCHAR, cchcatalogname : super::SQLSMALLINT, szschemaname : *const super::SQLWCHAR, cchschemaname : super::SQLSMALLINT, szprocname : *const super::SQLWCHAR, cchprocname : super::SQLSMALLINT) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLSetConnectAttrA(hdbc : super::SQLHDBC, fattribute : super::SQLINTEGER, rgbvalue : super::SQLPOINTER, cbvalue : super::SQLINTEGER) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLSetConnectAttrW(hdbc : super::SQLHDBC, fattribute : super::SQLINTEGER, rgbvalue : super::SQLPOINTER, cbvalue : super::SQLINTEGER) -> super::SQLRETURN);
#[cfg(target_arch = "x86")]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLSetConnectOptionA(hdbc : super::SQLHDBC, foption : super::SQLUSMALLINT, vparam : super::SQLUINTEGER) -> super::SQLRETURN);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLSetConnectOptionA(hdbc : super::SQLHDBC, foption : super::SQLUSMALLINT, vparam : super::SQLULEN) -> super::SQLRETURN);
#[cfg(target_arch = "x86")]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLSetConnectOptionW(hdbc : super::SQLHDBC, foption : super::SQLUSMALLINT, vparam : super::SQLUINTEGER) -> super::SQLRETURN);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLSetConnectOptionW(hdbc : super::SQLHDBC, foption : super::SQLUSMALLINT, vparam : super::SQLULEN) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLSetCursorNameA(hstmt : super::SQLHSTMT, szcursor : *const super::SQLCHAR, cbcursor : super::SQLSMALLINT) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLSetCursorNameW(hstmt : super::SQLHSTMT, szcursor : *const super::SQLWCHAR, cchcursor : super::SQLSMALLINT) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLSetDescFieldW(descriptorhandle : super::SQLHDESC, recnumber : super::SQLSMALLINT, fieldidentifier : super::SQLSMALLINT, value : super::SQLPOINTER, bufferlength : super::SQLINTEGER) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLSetStmtAttrW(hstmt : super::SQLHSTMT, fattribute : super::SQLINTEGER, rgbvalue : super::SQLPOINTER, cbvaluemax : super::SQLINTEGER) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLSpecialColumnsA(hstmt : super::SQLHSTMT, fcoltype : super::SQLUSMALLINT, szcatalogname : *const super::SQLCHAR, cbcatalogname : super::SQLSMALLINT, szschemaname : *const super::SQLCHAR, cbschemaname : super::SQLSMALLINT, sztablename : *const super::SQLCHAR, cbtablename : super::SQLSMALLINT, fscope : super::SQLUSMALLINT, fnullable : super::SQLUSMALLINT) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLSpecialColumnsW(hstmt : super::SQLHSTMT, fcoltype : super::SQLUSMALLINT, szcatalogname : *const super::SQLWCHAR, cchcatalogname : super::SQLSMALLINT, szschemaname : *const super::SQLWCHAR, cchschemaname : super::SQLSMALLINT, sztablename : *const super::SQLWCHAR, cchtablename : super::SQLSMALLINT, fscope : super::SQLUSMALLINT, fnullable : super::SQLUSMALLINT) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLStatisticsA(hstmt : super::SQLHSTMT, szcatalogname : *const super::SQLCHAR, cbcatalogname : super::SQLSMALLINT, szschemaname : *const super::SQLCHAR, cbschemaname : super::SQLSMALLINT, sztablename : *const super::SQLCHAR, cbtablename : super::SQLSMALLINT, funique : super::SQLUSMALLINT, faccuracy : super::SQLUSMALLINT) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLStatisticsW(hstmt : super::SQLHSTMT, szcatalogname : *const super::SQLWCHAR, cchcatalogname : super::SQLSMALLINT, szschemaname : *const super::SQLWCHAR, cchschemaname : super::SQLSMALLINT, sztablename : *const super::SQLWCHAR, cchtablename : super::SQLSMALLINT, funique : super::SQLUSMALLINT, faccuracy : super::SQLUSMALLINT) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLTablePrivilegesA(hstmt : super::SQLHSTMT, szcatalogname : *const super::SQLCHAR, cbcatalogname : super::SQLSMALLINT, szschemaname : *const super::SQLCHAR, cbschemaname : super::SQLSMALLINT, sztablename : *const super::SQLCHAR, cbtablename : super::SQLSMALLINT) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLTablePrivilegesW(hstmt : super::SQLHSTMT, szcatalogname : *const super::SQLWCHAR, cchcatalogname : super::SQLSMALLINT, szschemaname : *const super::SQLWCHAR, cchschemaname : super::SQLSMALLINT, sztablename : *const super::SQLWCHAR, cchtablename : super::SQLSMALLINT) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLTablesA(hstmt : super::SQLHSTMT, szcatalogname : *const super::SQLCHAR, cbcatalogname : super::SQLSMALLINT, szschemaname : *const super::SQLCHAR, cbschemaname : super::SQLSMALLINT, sztablename : *const super::SQLCHAR, cbtablename : super::SQLSMALLINT, sztabletype : *const super::SQLCHAR, cbtabletype : super::SQLSMALLINT) -> super::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLTablesW(hstmt : super::SQLHSTMT, szcatalogname : *const super::SQLWCHAR, cchcatalogname : super::SQLSMALLINT, szschemaname : *const super::SQLWCHAR, cchschemaname : super::SQLSMALLINT, sztablename : *const super::SQLWCHAR, cchtablename : super::SQLSMALLINT, sztabletype : *const super::SQLWCHAR, cchtabletype : super::SQLSMALLINT) -> super::SQLRETURN);
pub const SQL_C_TCHAR: u32 = 1;
pub const SQL_C_WCHAR: i32 = -8;
pub const SQL_SQLSTATE_SIZEW: u32 = 10;
pub const SQL_WCHAR: i32 = -8;
pub const SQL_WLONGVARCHAR: i32 = -10;
pub const SQL_WVARCHAR: i32 = -9;
