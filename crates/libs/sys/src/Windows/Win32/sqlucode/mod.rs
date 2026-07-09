#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLBrowseConnectA(hdbc : super::sqltypes::SQLHDBC, szconnstrin : *const super::sqltypes::SQLCHAR, cbconnstrin : super::sqltypes::SQLSMALLINT, szconnstrout : *mut super::sqltypes::SQLCHAR, cbconnstroutmax : super::sqltypes::SQLSMALLINT, pcbconnstrout : *mut super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLBrowseConnectW(hdbc : super::sqltypes::SQLHDBC, szconnstrin : *const super::sqltypes::SQLWCHAR, cchconnstrin : super::sqltypes::SQLSMALLINT, szconnstrout : *mut super::sqltypes::SQLWCHAR, cchconnstroutmax : super::sqltypes::SQLSMALLINT, pcchconnstrout : *mut super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
#[cfg(target_arch = "x86")]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLColAttributeA(hstmt : super::sqltypes::SQLHSTMT, icol : super::sqltypes::SQLSMALLINT, ifield : super::sqltypes::SQLSMALLINT, pcharattr : super::sqltypes::SQLPOINTER, cbcharattrmax : super::sqltypes::SQLSMALLINT, pcbcharattr : *mut super::sqltypes::SQLSMALLINT, pnumattr : super::sqltypes::SQLPOINTER) -> super::sqltypes::SQLRETURN);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLColAttributeA(hstmt : super::sqltypes::SQLHSTMT, icol : super::sqltypes::SQLSMALLINT, ifield : super::sqltypes::SQLSMALLINT, pcharattr : super::sqltypes::SQLPOINTER, cbcharattrmax : super::sqltypes::SQLSMALLINT, pcbcharattr : *mut super::sqltypes::SQLSMALLINT, pnumattr : *mut super::sqltypes::SQLLEN) -> super::sqltypes::SQLRETURN);
#[cfg(target_arch = "x86")]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLColAttributeW(hstmt : super::sqltypes::SQLHSTMT, icol : super::sqltypes::SQLUSMALLINT, ifield : super::sqltypes::SQLUSMALLINT, pcharattr : super::sqltypes::SQLPOINTER, cbdescmax : super::sqltypes::SQLSMALLINT, pcbcharattr : *mut super::sqltypes::SQLSMALLINT, pnumattr : super::sqltypes::SQLPOINTER) -> super::sqltypes::SQLRETURN);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLColAttributeW(hstmt : super::sqltypes::SQLHSTMT, icol : super::sqltypes::SQLUSMALLINT, ifield : super::sqltypes::SQLUSMALLINT, pcharattr : super::sqltypes::SQLPOINTER, cbdescmax : super::sqltypes::SQLSMALLINT, pcbcharattr : *mut super::sqltypes::SQLSMALLINT, pnumattr : *mut super::sqltypes::SQLLEN) -> super::sqltypes::SQLRETURN);
#[cfg(target_arch = "x86")]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLColAttributesA(hstmt : super::sqltypes::SQLHSTMT, icol : super::sqltypes::SQLUSMALLINT, fdesctype : super::sqltypes::SQLUSMALLINT, rgbdesc : super::sqltypes::SQLPOINTER, cbdescmax : super::sqltypes::SQLSMALLINT, pcbdesc : *mut super::sqltypes::SQLSMALLINT, pfdesc : *mut super::sqltypes::SQLINTEGER) -> super::sqltypes::SQLRETURN);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLColAttributesA(hstmt : super::sqltypes::SQLHSTMT, icol : super::sqltypes::SQLUSMALLINT, fdesctype : super::sqltypes::SQLUSMALLINT, rgbdesc : super::sqltypes::SQLPOINTER, cbdescmax : super::sqltypes::SQLSMALLINT, pcbdesc : *mut super::sqltypes::SQLSMALLINT, pfdesc : *mut super::sqltypes::SQLLEN) -> super::sqltypes::SQLRETURN);
#[cfg(target_arch = "x86")]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLColAttributesW(hstmt : super::sqltypes::SQLHSTMT, icol : super::sqltypes::SQLUSMALLINT, fdesctype : super::sqltypes::SQLUSMALLINT, rgbdesc : super::sqltypes::SQLPOINTER, cbdescmax : super::sqltypes::SQLSMALLINT, pcbdesc : *mut super::sqltypes::SQLSMALLINT, pfdesc : *mut super::sqltypes::SQLINTEGER) -> super::sqltypes::SQLRETURN);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLColAttributesW(hstmt : super::sqltypes::SQLHSTMT, icol : super::sqltypes::SQLUSMALLINT, fdesctype : super::sqltypes::SQLUSMALLINT, rgbdesc : super::sqltypes::SQLPOINTER, cbdescmax : super::sqltypes::SQLSMALLINT, pcbdesc : *mut super::sqltypes::SQLSMALLINT, pfdesc : *mut super::sqltypes::SQLLEN) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLColumnPrivilegesA(hstmt : super::sqltypes::SQLHSTMT, szcatalogname : *const super::sqltypes::SQLCHAR, cbcatalogname : super::sqltypes::SQLSMALLINT, szschemaname : *const super::sqltypes::SQLCHAR, cbschemaname : super::sqltypes::SQLSMALLINT, sztablename : *const super::sqltypes::SQLCHAR, cbtablename : super::sqltypes::SQLSMALLINT, szcolumnname : *const super::sqltypes::SQLCHAR, cbcolumnname : super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLColumnPrivilegesW(hstmt : super::sqltypes::SQLHSTMT, szcatalogname : *const super::sqltypes::SQLWCHAR, cchcatalogname : super::sqltypes::SQLSMALLINT, szschemaname : *const super::sqltypes::SQLWCHAR, cchschemaname : super::sqltypes::SQLSMALLINT, sztablename : *const super::sqltypes::SQLWCHAR, cchtablename : super::sqltypes::SQLSMALLINT, szcolumnname : *const super::sqltypes::SQLWCHAR, cchcolumnname : super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLColumnsA(hstmt : super::sqltypes::SQLHSTMT, szcatalogname : *const super::sqltypes::SQLCHAR, cbcatalogname : super::sqltypes::SQLSMALLINT, szschemaname : *const super::sqltypes::SQLCHAR, cbschemaname : super::sqltypes::SQLSMALLINT, sztablename : *const super::sqltypes::SQLCHAR, cbtablename : super::sqltypes::SQLSMALLINT, szcolumnname : *const super::sqltypes::SQLCHAR, cbcolumnname : super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLColumnsW(hstmt : super::sqltypes::SQLHSTMT, szcatalogname : *const super::sqltypes::SQLWCHAR, cchcatalogname : super::sqltypes::SQLSMALLINT, szschemaname : *const super::sqltypes::SQLWCHAR, cchschemaname : super::sqltypes::SQLSMALLINT, sztablename : *const super::sqltypes::SQLWCHAR, cchtablename : super::sqltypes::SQLSMALLINT, szcolumnname : *const super::sqltypes::SQLWCHAR, cchcolumnname : super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLConnectA(hdbc : super::sqltypes::SQLHDBC, szdsn : *const super::sqltypes::SQLCHAR, cbdsn : super::sqltypes::SQLSMALLINT, szuid : *const super::sqltypes::SQLCHAR, cbuid : super::sqltypes::SQLSMALLINT, szauthstr : *const super::sqltypes::SQLCHAR, cbauthstr : super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLConnectW(hdbc : super::sqltypes::SQLHDBC, szdsn : *const super::sqltypes::SQLWCHAR, cchdsn : super::sqltypes::SQLSMALLINT, szuid : *const super::sqltypes::SQLWCHAR, cchuid : super::sqltypes::SQLSMALLINT, szauthstr : *const super::sqltypes::SQLWCHAR, cchauthstr : super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLDataSourcesA(henv : super::sqltypes::SQLHENV, fdirection : super::sqltypes::SQLUSMALLINT, szdsn : *mut super::sqltypes::SQLCHAR, cbdsnmax : super::sqltypes::SQLSMALLINT, pcbdsn : *mut super::sqltypes::SQLSMALLINT, szdescription : *mut super::sqltypes::SQLCHAR, cbdescriptionmax : super::sqltypes::SQLSMALLINT, pcbdescription : *mut super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLDataSourcesW(henv : super::sqltypes::SQLHENV, fdirection : super::sqltypes::SQLUSMALLINT, szdsn : *mut super::sqltypes::SQLWCHAR, cchdsnmax : super::sqltypes::SQLSMALLINT, pcchdsn : *mut super::sqltypes::SQLSMALLINT, wszdescription : *mut super::sqltypes::SQLWCHAR, cchdescriptionmax : super::sqltypes::SQLSMALLINT, pcchdescription : *mut super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
#[cfg(target_arch = "x86")]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLDescribeColA(hstmt : super::sqltypes::SQLHSTMT, icol : super::sqltypes::SQLUSMALLINT, szcolname : *mut super::sqltypes::SQLCHAR, cbcolnamemax : super::sqltypes::SQLSMALLINT, pcbcolname : *mut super::sqltypes::SQLSMALLINT, pfsqltype : *mut super::sqltypes::SQLSMALLINT, pcbcoldef : *mut super::sqltypes::SQLUINTEGER, pibscale : *mut super::sqltypes::SQLSMALLINT, pfnullable : *mut super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLDescribeColA(hstmt : super::sqltypes::SQLHSTMT, icol : super::sqltypes::SQLUSMALLINT, szcolname : *mut super::sqltypes::SQLCHAR, cbcolnamemax : super::sqltypes::SQLSMALLINT, pcbcolname : *mut super::sqltypes::SQLSMALLINT, pfsqltype : *mut super::sqltypes::SQLSMALLINT, pcbcoldef : *mut super::sqltypes::SQLULEN, pibscale : *mut super::sqltypes::SQLSMALLINT, pfnullable : *mut super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
#[cfg(target_arch = "x86")]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLDescribeColW(hstmt : super::sqltypes::SQLHSTMT, icol : super::sqltypes::SQLUSMALLINT, szcolname : *mut super::sqltypes::SQLWCHAR, cchcolnamemax : super::sqltypes::SQLSMALLINT, pcchcolname : *mut super::sqltypes::SQLSMALLINT, pfsqltype : *mut super::sqltypes::SQLSMALLINT, pcbcoldef : *mut super::sqltypes::SQLUINTEGER, pibscale : *mut super::sqltypes::SQLSMALLINT, pfnullable : *mut super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLDescribeColW(hstmt : super::sqltypes::SQLHSTMT, icol : super::sqltypes::SQLUSMALLINT, szcolname : *mut super::sqltypes::SQLWCHAR, cchcolnamemax : super::sqltypes::SQLSMALLINT, pcchcolname : *mut super::sqltypes::SQLSMALLINT, pfsqltype : *mut super::sqltypes::SQLSMALLINT, pcbcoldef : *mut super::sqltypes::SQLULEN, pibscale : *mut super::sqltypes::SQLSMALLINT, pfnullable : *mut super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
#[cfg(all(feature = "sqltypes", feature = "windef"))]
windows_link::link!("odbc32.dll" "system" fn SQLDriverConnectA(hdbc : super::sqltypes::SQLHDBC, hwnd : super::sqltypes::SQLHWND, szconnstrin : *const super::sqltypes::SQLCHAR, cbconnstrin : super::sqltypes::SQLSMALLINT, szconnstrout : *mut super::sqltypes::SQLCHAR, cbconnstroutmax : super::sqltypes::SQLSMALLINT, pcbconnstrout : *mut super::sqltypes::SQLSMALLINT, fdrivercompletion : super::sqltypes::SQLUSMALLINT) -> super::sqltypes::SQLRETURN);
#[cfg(all(feature = "sqltypes", feature = "windef"))]
windows_link::link!("odbc32.dll" "system" fn SQLDriverConnectW(hdbc : super::sqltypes::SQLHDBC, hwnd : super::sqltypes::SQLHWND, szconnstrin : *const super::sqltypes::SQLWCHAR, cchconnstrin : super::sqltypes::SQLSMALLINT, szconnstrout : *mut super::sqltypes::SQLWCHAR, cchconnstroutmax : super::sqltypes::SQLSMALLINT, pcchconnstrout : *mut super::sqltypes::SQLSMALLINT, fdrivercompletion : super::sqltypes::SQLUSMALLINT) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLDriversA(henv : super::sqltypes::SQLHENV, fdirection : super::sqltypes::SQLUSMALLINT, szdriverdesc : *mut super::sqltypes::SQLCHAR, cbdriverdescmax : super::sqltypes::SQLSMALLINT, pcbdriverdesc : *mut super::sqltypes::SQLSMALLINT, szdriverattributes : *mut super::sqltypes::SQLCHAR, cbdrvrattrmax : super::sqltypes::SQLSMALLINT, pcbdrvrattr : *mut super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLDriversW(henv : super::sqltypes::SQLHENV, fdirection : super::sqltypes::SQLUSMALLINT, szdriverdesc : *mut super::sqltypes::SQLWCHAR, cchdriverdescmax : super::sqltypes::SQLSMALLINT, pcchdriverdesc : *mut super::sqltypes::SQLSMALLINT, szdriverattributes : *mut super::sqltypes::SQLWCHAR, cchdrvrattrmax : super::sqltypes::SQLSMALLINT, pcchdrvrattr : *mut super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLErrorA(henv : super::sqltypes::SQLHENV, hdbc : super::sqltypes::SQLHDBC, hstmt : super::sqltypes::SQLHSTMT, szsqlstate : *mut super::sqltypes::SQLCHAR, pfnativeerror : *mut super::sqltypes::SQLINTEGER, szerrormsg : *mut super::sqltypes::SQLCHAR, cberrormsgmax : super::sqltypes::SQLSMALLINT, pcberrormsg : *mut super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLErrorW(henv : super::sqltypes::SQLHENV, hdbc : super::sqltypes::SQLHDBC, hstmt : super::sqltypes::SQLHSTMT, wszsqlstate : *mut super::sqltypes::SQLWCHAR, pfnativeerror : *mut super::sqltypes::SQLINTEGER, wszerrormsg : *mut super::sqltypes::SQLWCHAR, ccherrormsgmax : super::sqltypes::SQLSMALLINT, pccherrormsg : *mut super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLExecDirectA(hstmt : super::sqltypes::SQLHSTMT, szsqlstr : *const super::sqltypes::SQLCHAR, cbsqlstr : super::sqltypes::SQLINTEGER) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLExecDirectW(hstmt : super::sqltypes::SQLHSTMT, szsqlstr : *const super::sqltypes::SQLWCHAR, textlength : super::sqltypes::SQLINTEGER) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLForeignKeysA(hstmt : super::sqltypes::SQLHSTMT, szpkcatalogname : *const super::sqltypes::SQLCHAR, cbpkcatalogname : super::sqltypes::SQLSMALLINT, szpkschemaname : *const super::sqltypes::SQLCHAR, cbpkschemaname : super::sqltypes::SQLSMALLINT, szpktablename : *const super::sqltypes::SQLCHAR, cbpktablename : super::sqltypes::SQLSMALLINT, szfkcatalogname : *const super::sqltypes::SQLCHAR, cbfkcatalogname : super::sqltypes::SQLSMALLINT, szfkschemaname : *const super::sqltypes::SQLCHAR, cbfkschemaname : super::sqltypes::SQLSMALLINT, szfktablename : *const super::sqltypes::SQLCHAR, cbfktablename : super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLForeignKeysW(hstmt : super::sqltypes::SQLHSTMT, szpkcatalogname : *const super::sqltypes::SQLWCHAR, cchpkcatalogname : super::sqltypes::SQLSMALLINT, szpkschemaname : *const super::sqltypes::SQLWCHAR, cchpkschemaname : super::sqltypes::SQLSMALLINT, szpktablename : *const super::sqltypes::SQLWCHAR, cchpktablename : super::sqltypes::SQLSMALLINT, szfkcatalogname : *const super::sqltypes::SQLWCHAR, cchfkcatalogname : super::sqltypes::SQLSMALLINT, szfkschemaname : *const super::sqltypes::SQLWCHAR, cchfkschemaname : super::sqltypes::SQLSMALLINT, szfktablename : *const super::sqltypes::SQLWCHAR, cchfktablename : super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLGetConnectAttrA(hdbc : super::sqltypes::SQLHDBC, fattribute : super::sqltypes::SQLINTEGER, rgbvalue : super::sqltypes::SQLPOINTER, cbvaluemax : super::sqltypes::SQLINTEGER, pcbvalue : *mut super::sqltypes::SQLINTEGER) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLGetConnectAttrW(hdbc : super::sqltypes::SQLHDBC, fattribute : super::sqltypes::SQLINTEGER, rgbvalue : super::sqltypes::SQLPOINTER, cbvaluemax : super::sqltypes::SQLINTEGER, pcbvalue : *mut super::sqltypes::SQLINTEGER) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLGetConnectOptionA(hdbc : super::sqltypes::SQLHDBC, foption : super::sqltypes::SQLUSMALLINT, pvparam : super::sqltypes::SQLPOINTER) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLGetConnectOptionW(hdbc : super::sqltypes::SQLHDBC, foption : super::sqltypes::SQLUSMALLINT, pvparam : super::sqltypes::SQLPOINTER) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLGetCursorNameA(hstmt : super::sqltypes::SQLHSTMT, szcursor : *mut super::sqltypes::SQLCHAR, cbcursormax : super::sqltypes::SQLSMALLINT, pcbcursor : *mut super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLGetCursorNameW(hstmt : super::sqltypes::SQLHSTMT, szcursor : *mut super::sqltypes::SQLWCHAR, cchcursormax : super::sqltypes::SQLSMALLINT, pcchcursor : *mut super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLGetDescFieldA(hdesc : super::sqltypes::SQLHDESC, irecord : super::sqltypes::SQLSMALLINT, ifield : super::sqltypes::SQLSMALLINT, rgbvalue : super::sqltypes::SQLPOINTER, cbbufferlength : super::sqltypes::SQLINTEGER, stringlength : *mut super::sqltypes::SQLINTEGER) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLGetDescFieldW(hdesc : super::sqltypes::SQLHDESC, irecord : super::sqltypes::SQLSMALLINT, ifield : super::sqltypes::SQLSMALLINT, rgbvalue : super::sqltypes::SQLPOINTER, cbbufferlength : super::sqltypes::SQLINTEGER, stringlength : *mut super::sqltypes::SQLINTEGER) -> super::sqltypes::SQLRETURN);
#[cfg(target_arch = "x86")]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLGetDescRecA(hdesc : super::sqltypes::SQLHDESC, irecord : super::sqltypes::SQLSMALLINT, szname : *mut super::sqltypes::SQLCHAR, cbnamemax : super::sqltypes::SQLSMALLINT, pcbname : *mut super::sqltypes::SQLSMALLINT, pftype : *mut super::sqltypes::SQLSMALLINT, pfsubtype : *mut super::sqltypes::SQLSMALLINT, plength : *mut super::sqltypes::SQLINTEGER, pprecision : *mut super::sqltypes::SQLSMALLINT, pscale : *mut super::sqltypes::SQLSMALLINT, pnullable : *mut super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLGetDescRecA(hdesc : super::sqltypes::SQLHDESC, irecord : super::sqltypes::SQLSMALLINT, szname : *mut super::sqltypes::SQLCHAR, cbnamemax : super::sqltypes::SQLSMALLINT, pcbname : *mut super::sqltypes::SQLSMALLINT, pftype : *mut super::sqltypes::SQLSMALLINT, pfsubtype : *mut super::sqltypes::SQLSMALLINT, plength : *mut super::sqltypes::SQLLEN, pprecision : *mut super::sqltypes::SQLSMALLINT, pscale : *mut super::sqltypes::SQLSMALLINT, pnullable : *mut super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
#[cfg(target_arch = "x86")]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLGetDescRecW(hdesc : super::sqltypes::SQLHDESC, irecord : super::sqltypes::SQLSMALLINT, szname : *mut super::sqltypes::SQLWCHAR, cchnamemax : super::sqltypes::SQLSMALLINT, pcchname : *mut super::sqltypes::SQLSMALLINT, pftype : *mut super::sqltypes::SQLSMALLINT, pfsubtype : *mut super::sqltypes::SQLSMALLINT, plength : *mut super::sqltypes::SQLINTEGER, pprecision : *mut super::sqltypes::SQLSMALLINT, pscale : *mut super::sqltypes::SQLSMALLINT, pnullable : *mut super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLGetDescRecW(hdesc : super::sqltypes::SQLHDESC, irecord : super::sqltypes::SQLSMALLINT, szname : *mut super::sqltypes::SQLWCHAR, cchnamemax : super::sqltypes::SQLSMALLINT, pcchname : *mut super::sqltypes::SQLSMALLINT, pftype : *mut super::sqltypes::SQLSMALLINT, pfsubtype : *mut super::sqltypes::SQLSMALLINT, plength : *mut super::sqltypes::SQLLEN, pprecision : *mut super::sqltypes::SQLSMALLINT, pscale : *mut super::sqltypes::SQLSMALLINT, pnullable : *mut super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLGetDiagFieldA(fhandletype : super::sqltypes::SQLSMALLINT, handle : super::sqltypes::SQLHANDLE, irecord : super::sqltypes::SQLSMALLINT, fdiagfield : super::sqltypes::SQLSMALLINT, rgbdiaginfo : super::sqltypes::SQLPOINTER, cbdiaginfomax : super::sqltypes::SQLSMALLINT, pcbdiaginfo : *mut super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLGetDiagFieldW(fhandletype : super::sqltypes::SQLSMALLINT, handle : super::sqltypes::SQLHANDLE, irecord : super::sqltypes::SQLSMALLINT, fdiagfield : super::sqltypes::SQLSMALLINT, rgbdiaginfo : super::sqltypes::SQLPOINTER, cbbufferlength : super::sqltypes::SQLSMALLINT, pcbstringlength : *mut super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLGetDiagRecA(fhandletype : super::sqltypes::SQLSMALLINT, handle : super::sqltypes::SQLHANDLE, irecord : super::sqltypes::SQLSMALLINT, szsqlstate : *mut super::sqltypes::SQLCHAR, pfnativeerror : *mut super::sqltypes::SQLINTEGER, szerrormsg : *mut super::sqltypes::SQLCHAR, cberrormsgmax : super::sqltypes::SQLSMALLINT, pcberrormsg : *mut super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLGetDiagRecW(fhandletype : super::sqltypes::SQLSMALLINT, handle : super::sqltypes::SQLHANDLE, irecord : super::sqltypes::SQLSMALLINT, szsqlstate : *mut super::sqltypes::SQLWCHAR, pfnativeerror : *mut super::sqltypes::SQLINTEGER, szerrormsg : *mut super::sqltypes::SQLWCHAR, ccherrormsgmax : super::sqltypes::SQLSMALLINT, pccherrormsg : *mut super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLGetInfoA(hdbc : super::sqltypes::SQLHDBC, finfotype : super::sqltypes::SQLUSMALLINT, rgbinfovalue : super::sqltypes::SQLPOINTER, cbinfovaluemax : super::sqltypes::SQLSMALLINT, pcbinfovalue : *mut super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLGetInfoW(hdbc : super::sqltypes::SQLHDBC, finfotype : super::sqltypes::SQLUSMALLINT, rgbinfovalue : super::sqltypes::SQLPOINTER, cbinfovaluemax : super::sqltypes::SQLSMALLINT, pcbinfovalue : *mut super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLGetStmtAttrA(hstmt : super::sqltypes::SQLHSTMT, fattribute : super::sqltypes::SQLINTEGER, rgbvalue : super::sqltypes::SQLPOINTER, cbvaluemax : super::sqltypes::SQLINTEGER, pcbvalue : *mut super::sqltypes::SQLINTEGER) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLGetStmtAttrW(hstmt : super::sqltypes::SQLHSTMT, fattribute : super::sqltypes::SQLINTEGER, rgbvalue : super::sqltypes::SQLPOINTER, cbvaluemax : super::sqltypes::SQLINTEGER, pcbvalue : *mut super::sqltypes::SQLINTEGER) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLGetTypeInfoA(statementhandle : super::sqltypes::SQLHSTMT, datatype : super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLGetTypeInfoW(statementhandle : super::sqltypes::SQLHSTMT, datatype : super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLNativeSqlA(hdbc : super::sqltypes::SQLHDBC, szsqlstrin : *const super::sqltypes::SQLCHAR, cbsqlstrin : super::sqltypes::SQLINTEGER, szsqlstr : *mut super::sqltypes::SQLCHAR, cbsqlstrmax : super::sqltypes::SQLINTEGER, pcbsqlstr : *mut super::sqltypes::SQLINTEGER) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLNativeSqlW(hdbc : super::sqltypes::SQLHDBC, szsqlstrin : *const super::sqltypes::SQLWCHAR, cchsqlstrin : super::sqltypes::SQLINTEGER, szsqlstr : *mut super::sqltypes::SQLWCHAR, cchsqlstrmax : super::sqltypes::SQLINTEGER, pcchsqlstr : *mut super::sqltypes::SQLINTEGER) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLPrepareA(hstmt : super::sqltypes::SQLHSTMT, szsqlstr : *const super::sqltypes::SQLCHAR, cbsqlstr : super::sqltypes::SQLINTEGER) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLPrepareW(hstmt : super::sqltypes::SQLHSTMT, szsqlstr : *const super::sqltypes::SQLWCHAR, cchsqlstr : super::sqltypes::SQLINTEGER) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLPrimaryKeysA(hstmt : super::sqltypes::SQLHSTMT, szcatalogname : *const super::sqltypes::SQLCHAR, cbcatalogname : super::sqltypes::SQLSMALLINT, szschemaname : *const super::sqltypes::SQLCHAR, cbschemaname : super::sqltypes::SQLSMALLINT, sztablename : *const super::sqltypes::SQLCHAR, cbtablename : super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLPrimaryKeysW(hstmt : super::sqltypes::SQLHSTMT, szcatalogname : *const super::sqltypes::SQLWCHAR, cchcatalogname : super::sqltypes::SQLSMALLINT, szschemaname : *const super::sqltypes::SQLWCHAR, cchschemaname : super::sqltypes::SQLSMALLINT, sztablename : *const super::sqltypes::SQLWCHAR, cchtablename : super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLProcedureColumnsA(hstmt : super::sqltypes::SQLHSTMT, szcatalogname : *const super::sqltypes::SQLCHAR, cbcatalogname : super::sqltypes::SQLSMALLINT, szschemaname : *const super::sqltypes::SQLCHAR, cbschemaname : super::sqltypes::SQLSMALLINT, szprocname : *const super::sqltypes::SQLCHAR, cbprocname : super::sqltypes::SQLSMALLINT, szcolumnname : *const super::sqltypes::SQLCHAR, cbcolumnname : super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLProcedureColumnsW(hstmt : super::sqltypes::SQLHSTMT, szcatalogname : *const super::sqltypes::SQLWCHAR, cchcatalogname : super::sqltypes::SQLSMALLINT, szschemaname : *const super::sqltypes::SQLWCHAR, cchschemaname : super::sqltypes::SQLSMALLINT, szprocname : *const super::sqltypes::SQLWCHAR, cchprocname : super::sqltypes::SQLSMALLINT, szcolumnname : *const super::sqltypes::SQLWCHAR, cchcolumnname : super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLProceduresA(hstmt : super::sqltypes::SQLHSTMT, szcatalogname : *const super::sqltypes::SQLCHAR, cbcatalogname : super::sqltypes::SQLSMALLINT, szschemaname : *const super::sqltypes::SQLCHAR, cbschemaname : super::sqltypes::SQLSMALLINT, szprocname : *const super::sqltypes::SQLCHAR, cbprocname : super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLProceduresW(hstmt : super::sqltypes::SQLHSTMT, szcatalogname : *const super::sqltypes::SQLWCHAR, cchcatalogname : super::sqltypes::SQLSMALLINT, szschemaname : *const super::sqltypes::SQLWCHAR, cchschemaname : super::sqltypes::SQLSMALLINT, szprocname : *const super::sqltypes::SQLWCHAR, cchprocname : super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLSetConnectAttrA(hdbc : super::sqltypes::SQLHDBC, fattribute : super::sqltypes::SQLINTEGER, rgbvalue : super::sqltypes::SQLPOINTER, cbvalue : super::sqltypes::SQLINTEGER) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLSetConnectAttrW(hdbc : super::sqltypes::SQLHDBC, fattribute : super::sqltypes::SQLINTEGER, rgbvalue : super::sqltypes::SQLPOINTER, cbvalue : super::sqltypes::SQLINTEGER) -> super::sqltypes::SQLRETURN);
#[cfg(target_arch = "x86")]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLSetConnectOptionA(hdbc : super::sqltypes::SQLHDBC, foption : super::sqltypes::SQLUSMALLINT, vparam : super::sqltypes::SQLUINTEGER) -> super::sqltypes::SQLRETURN);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLSetConnectOptionA(hdbc : super::sqltypes::SQLHDBC, foption : super::sqltypes::SQLUSMALLINT, vparam : super::sqltypes::SQLULEN) -> super::sqltypes::SQLRETURN);
#[cfg(target_arch = "x86")]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLSetConnectOptionW(hdbc : super::sqltypes::SQLHDBC, foption : super::sqltypes::SQLUSMALLINT, vparam : super::sqltypes::SQLUINTEGER) -> super::sqltypes::SQLRETURN);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLSetConnectOptionW(hdbc : super::sqltypes::SQLHDBC, foption : super::sqltypes::SQLUSMALLINT, vparam : super::sqltypes::SQLULEN) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLSetCursorNameA(hstmt : super::sqltypes::SQLHSTMT, szcursor : *const super::sqltypes::SQLCHAR, cbcursor : super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLSetCursorNameW(hstmt : super::sqltypes::SQLHSTMT, szcursor : *const super::sqltypes::SQLWCHAR, cchcursor : super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLSetDescFieldW(descriptorhandle : super::sqltypes::SQLHDESC, recnumber : super::sqltypes::SQLSMALLINT, fieldidentifier : super::sqltypes::SQLSMALLINT, value : super::sqltypes::SQLPOINTER, bufferlength : super::sqltypes::SQLINTEGER) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLSetStmtAttrW(hstmt : super::sqltypes::SQLHSTMT, fattribute : super::sqltypes::SQLINTEGER, rgbvalue : super::sqltypes::SQLPOINTER, cbvaluemax : super::sqltypes::SQLINTEGER) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLSpecialColumnsA(hstmt : super::sqltypes::SQLHSTMT, fcoltype : super::sqltypes::SQLUSMALLINT, szcatalogname : *const super::sqltypes::SQLCHAR, cbcatalogname : super::sqltypes::SQLSMALLINT, szschemaname : *const super::sqltypes::SQLCHAR, cbschemaname : super::sqltypes::SQLSMALLINT, sztablename : *const super::sqltypes::SQLCHAR, cbtablename : super::sqltypes::SQLSMALLINT, fscope : super::sqltypes::SQLUSMALLINT, fnullable : super::sqltypes::SQLUSMALLINT) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLSpecialColumnsW(hstmt : super::sqltypes::SQLHSTMT, fcoltype : super::sqltypes::SQLUSMALLINT, szcatalogname : *const super::sqltypes::SQLWCHAR, cchcatalogname : super::sqltypes::SQLSMALLINT, szschemaname : *const super::sqltypes::SQLWCHAR, cchschemaname : super::sqltypes::SQLSMALLINT, sztablename : *const super::sqltypes::SQLWCHAR, cchtablename : super::sqltypes::SQLSMALLINT, fscope : super::sqltypes::SQLUSMALLINT, fnullable : super::sqltypes::SQLUSMALLINT) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLStatisticsA(hstmt : super::sqltypes::SQLHSTMT, szcatalogname : *const super::sqltypes::SQLCHAR, cbcatalogname : super::sqltypes::SQLSMALLINT, szschemaname : *const super::sqltypes::SQLCHAR, cbschemaname : super::sqltypes::SQLSMALLINT, sztablename : *const super::sqltypes::SQLCHAR, cbtablename : super::sqltypes::SQLSMALLINT, funique : super::sqltypes::SQLUSMALLINT, faccuracy : super::sqltypes::SQLUSMALLINT) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLStatisticsW(hstmt : super::sqltypes::SQLHSTMT, szcatalogname : *const super::sqltypes::SQLWCHAR, cchcatalogname : super::sqltypes::SQLSMALLINT, szschemaname : *const super::sqltypes::SQLWCHAR, cchschemaname : super::sqltypes::SQLSMALLINT, sztablename : *const super::sqltypes::SQLWCHAR, cchtablename : super::sqltypes::SQLSMALLINT, funique : super::sqltypes::SQLUSMALLINT, faccuracy : super::sqltypes::SQLUSMALLINT) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLTablePrivilegesA(hstmt : super::sqltypes::SQLHSTMT, szcatalogname : *const super::sqltypes::SQLCHAR, cbcatalogname : super::sqltypes::SQLSMALLINT, szschemaname : *const super::sqltypes::SQLCHAR, cbschemaname : super::sqltypes::SQLSMALLINT, sztablename : *const super::sqltypes::SQLCHAR, cbtablename : super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLTablePrivilegesW(hstmt : super::sqltypes::SQLHSTMT, szcatalogname : *const super::sqltypes::SQLWCHAR, cchcatalogname : super::sqltypes::SQLSMALLINT, szschemaname : *const super::sqltypes::SQLWCHAR, cchschemaname : super::sqltypes::SQLSMALLINT, sztablename : *const super::sqltypes::SQLWCHAR, cchtablename : super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLTablesA(hstmt : super::sqltypes::SQLHSTMT, szcatalogname : *const super::sqltypes::SQLCHAR, cbcatalogname : super::sqltypes::SQLSMALLINT, szschemaname : *const super::sqltypes::SQLCHAR, cbschemaname : super::sqltypes::SQLSMALLINT, sztablename : *const super::sqltypes::SQLCHAR, cbtablename : super::sqltypes::SQLSMALLINT, sztabletype : *const super::sqltypes::SQLCHAR, cbtabletype : super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
#[cfg(feature = "sqltypes")]
windows_link::link!("odbc32.dll" "system" fn SQLTablesW(hstmt : super::sqltypes::SQLHSTMT, szcatalogname : *const super::sqltypes::SQLWCHAR, cchcatalogname : super::sqltypes::SQLSMALLINT, szschemaname : *const super::sqltypes::SQLWCHAR, cchschemaname : super::sqltypes::SQLSMALLINT, sztablename : *const super::sqltypes::SQLWCHAR, cchtablename : super::sqltypes::SQLSMALLINT, sztabletype : *const super::sqltypes::SQLWCHAR, cchtabletype : super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
pub const SQL_C_TCHAR: u32 = 1;
pub const SQL_C_WCHAR: i32 = -8;
pub const SQL_SQLSTATE_SIZEW: u32 = 10;
pub const SQL_WCHAR: i32 = -8;
pub const SQL_WLONGVARCHAR: i32 = -10;
pub const SQL_WVARCHAR: i32 = -9;
