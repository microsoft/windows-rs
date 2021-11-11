#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Win32_System_Search_Common")]
pub mod Common;
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn ODBCGetTryWaitValue() -> u32;
    #[doc = "*Required features: `Win32_System_Search`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ODBCSetTryWaitValue(dwvalue: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLAllocConnect(environmenthandle: *mut ::core::ffi::c_void, connectionhandle: *mut *mut ::core::ffi::c_void) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLAllocEnv(environmenthandle: *mut *mut ::core::ffi::c_void) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLAllocHandle(handletype: i16, inputhandle: *mut ::core::ffi::c_void, outputhandle: *mut *mut ::core::ffi::c_void) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLAllocHandleStd(fhandletype: i16, hinput: *mut ::core::ffi::c_void, phoutput: *mut *mut ::core::ffi::c_void) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLAllocStmt(connectionhandle: *mut ::core::ffi::c_void, statementhandle: *mut *mut ::core::ffi::c_void) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn SQLBindCol(statementhandle: *mut ::core::ffi::c_void, columnnumber: u16, targettype: i16, targetvalue: *mut ::core::ffi::c_void, bufferlength: i64, strlen_or_ind: *mut i64) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86",))]
    pub fn SQLBindCol(statementhandle: *mut ::core::ffi::c_void, columnnumber: u16, targettype: i16, targetvalue: *mut ::core::ffi::c_void, bufferlength: i32, strlen_or_ind: *mut i32) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn SQLBindParam(statementhandle: *mut ::core::ffi::c_void, parameternumber: u16, valuetype: i16, parametertype: i16, lengthprecision: u64, parameterscale: i16, parametervalue: *mut ::core::ffi::c_void, strlen_or_ind: *mut i64) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86",))]
    pub fn SQLBindParam(statementhandle: *mut ::core::ffi::c_void, parameternumber: u16, valuetype: i16, parametertype: i16, lengthprecision: u32, parameterscale: i16, parametervalue: *mut ::core::ffi::c_void, strlen_or_ind: *mut i32) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn SQLBindParameter(hstmt: *mut ::core::ffi::c_void, ipar: u16, fparamtype: i16, fctype: i16, fsqltype: i16, cbcoldef: u64, ibscale: i16, rgbvalue: *mut ::core::ffi::c_void, cbvaluemax: i64, pcbvalue: *mut i64) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86",))]
    pub fn SQLBindParameter(hstmt: *mut ::core::ffi::c_void, ipar: u16, fparamtype: i16, fctype: i16, fsqltype: i16, cbcoldef: u32, ibscale: i16, rgbvalue: *mut ::core::ffi::c_void, cbvaluemax: i32, pcbvalue: *mut i32) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLBrowseConnect(hdbc: *mut ::core::ffi::c_void, szconnstrin: *const u8, cchconnstrin: i16, szconnstrout: *mut u8, cchconnstroutmax: i16, pcchconnstrout: *mut i16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLBrowseConnectA(hdbc: *mut ::core::ffi::c_void, szconnstrin: *const u8, cbconnstrin: i16, szconnstrout: *mut u8, cbconnstroutmax: i16, pcbconnstrout: *mut i16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLBrowseConnectW(hdbc: *mut ::core::ffi::c_void, szconnstrin: *const u16, cchconnstrin: i16, szconnstrout: *mut u16, cchconnstroutmax: i16, pcchconnstrout: *mut i16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLBulkOperations(statementhandle: *mut ::core::ffi::c_void, operation: i16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLCancel(statementhandle: *mut ::core::ffi::c_void) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLCancelHandle(handletype: i16, inputhandle: *mut ::core::ffi::c_void) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLCloseCursor(statementhandle: *mut ::core::ffi::c_void) -> i16;
    #[doc = "*Required features: `Win32_System_Search`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SQLCloseEnumServers(henumhandle: super::super::Foundation::HANDLE) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn SQLColAttribute(statementhandle: *mut ::core::ffi::c_void, columnnumber: u16, fieldidentifier: u16, characterattribute: *mut ::core::ffi::c_void, bufferlength: i16, stringlength: *mut i16, numericattribute: *mut i64) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86",))]
    pub fn SQLColAttribute(statementhandle: *mut ::core::ffi::c_void, columnnumber: u16, fieldidentifier: u16, characterattribute: *mut ::core::ffi::c_void, bufferlength: i16, stringlength: *mut i16, numericattribute: *mut ::core::ffi::c_void) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn SQLColAttributeA(hstmt: *mut ::core::ffi::c_void, icol: i16, ifield: i16, pcharattr: *mut ::core::ffi::c_void, cbcharattrmax: i16, pcbcharattr: *mut i16, pnumattr: *mut i64) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86",))]
    pub fn SQLColAttributeA(hstmt: *mut ::core::ffi::c_void, icol: i16, ifield: i16, pcharattr: *mut ::core::ffi::c_void, cbcharattrmax: i16, pcbcharattr: *mut i16, pnumattr: *mut ::core::ffi::c_void) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn SQLColAttributeW(hstmt: *mut ::core::ffi::c_void, icol: u16, ifield: u16, pcharattr: *mut ::core::ffi::c_void, cbdescmax: i16, pcbcharattr: *mut i16, pnumattr: *mut i64) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86",))]
    pub fn SQLColAttributeW(hstmt: *mut ::core::ffi::c_void, icol: u16, ifield: u16, pcharattr: *mut ::core::ffi::c_void, cbdescmax: i16, pcbcharattr: *mut i16, pnumattr: *mut ::core::ffi::c_void) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn SQLColAttributes(hstmt: *mut ::core::ffi::c_void, icol: u16, fdesctype: u16, rgbdesc: *mut ::core::ffi::c_void, cbdescmax: i16, pcbdesc: *mut i16, pfdesc: *mut i64) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86",))]
    pub fn SQLColAttributes(hstmt: *mut ::core::ffi::c_void, icol: u16, fdesctype: u16, rgbdesc: *mut ::core::ffi::c_void, cbdescmax: i16, pcbdesc: *mut i16, pfdesc: *mut i32) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn SQLColAttributesA(hstmt: *mut ::core::ffi::c_void, icol: u16, fdesctype: u16, rgbdesc: *mut ::core::ffi::c_void, cbdescmax: i16, pcbdesc: *mut i16, pfdesc: *mut i64) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86",))]
    pub fn SQLColAttributesA(hstmt: *mut ::core::ffi::c_void, icol: u16, fdesctype: u16, rgbdesc: *mut ::core::ffi::c_void, cbdescmax: i16, pcbdesc: *mut i16, pfdesc: *mut i32) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn SQLColAttributesW(hstmt: *mut ::core::ffi::c_void, icol: u16, fdesctype: u16, rgbdesc: *mut ::core::ffi::c_void, cbdescmax: i16, pcbdesc: *mut i16, pfdesc: *mut i64) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86",))]
    pub fn SQLColAttributesW(hstmt: *mut ::core::ffi::c_void, icol: u16, fdesctype: u16, rgbdesc: *mut ::core::ffi::c_void, cbdescmax: i16, pcbdesc: *mut i16, pfdesc: *mut i32) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLColumnPrivileges(hstmt: *mut ::core::ffi::c_void, szcatalogname: *const u8, cchcatalogname: i16, szschemaname: *const u8, cchschemaname: i16, sztablename: *const u8, cchtablename: i16, szcolumnname: *const u8, cchcolumnname: i16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLColumnPrivilegesA(hstmt: *mut ::core::ffi::c_void, szcatalogname: *const u8, cbcatalogname: i16, szschemaname: *const u8, cbschemaname: i16, sztablename: *const u8, cbtablename: i16, szcolumnname: *const u8, cbcolumnname: i16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLColumnPrivilegesW(hstmt: *mut ::core::ffi::c_void, szcatalogname: *const u16, cchcatalogname: i16, szschemaname: *const u16, cchschemaname: i16, sztablename: *const u16, cchtablename: i16, szcolumnname: *const u16, cchcolumnname: i16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLColumns(statementhandle: *mut ::core::ffi::c_void, catalogname: *const u8, namelength1: i16, schemaname: *const u8, namelength2: i16, tablename: *const u8, namelength3: i16, columnname: *const u8, namelength4: i16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLColumnsA(hstmt: *mut ::core::ffi::c_void, szcatalogname: *const u8, cbcatalogname: i16, szschemaname: *const u8, cbschemaname: i16, sztablename: *const u8, cbtablename: i16, szcolumnname: *const u8, cbcolumnname: i16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLColumnsW(hstmt: *mut ::core::ffi::c_void, szcatalogname: *const u16, cchcatalogname: i16, szschemaname: *const u16, cchschemaname: i16, sztablename: *const u16, cchtablename: i16, szcolumnname: *const u16, cchcolumnname: i16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLCompleteAsync(handletype: i16, handle: *mut ::core::ffi::c_void, asyncretcodeptr: *mut i16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLConnect(connectionhandle: *mut ::core::ffi::c_void, servername: *const u8, namelength1: i16, username: *const u8, namelength2: i16, authentication: *const u8, namelength3: i16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLConnectA(hdbc: *mut ::core::ffi::c_void, szdsn: *const u8, cbdsn: i16, szuid: *const u8, cbuid: i16, szauthstr: *const u8, cbauthstr: i16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLConnectW(hdbc: *mut ::core::ffi::c_void, szdsn: *const u16, cchdsn: i16, szuid: *const u16, cchuid: i16, szauthstr: *const u16, cchauthstr: i16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLCopyDesc(sourcedeschandle: *mut ::core::ffi::c_void, targetdeschandle: *mut ::core::ffi::c_void) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLDataSources(environmenthandle: *mut ::core::ffi::c_void, direction: u16, servername: *mut u8, bufferlength1: i16, namelength1ptr: *mut i16, description: *mut u8, bufferlength2: i16, namelength2ptr: *mut i16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLDataSourcesA(henv: *mut ::core::ffi::c_void, fdirection: u16, szdsn: *mut u8, cbdsnmax: i16, pcbdsn: *mut i16, szdescription: *mut u8, cbdescriptionmax: i16, pcbdescription: *mut i16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLDataSourcesW(henv: *mut ::core::ffi::c_void, fdirection: u16, szdsn: *mut u16, cchdsnmax: i16, pcchdsn: *mut i16, wszdescription: *mut u16, cchdescriptionmax: i16, pcchdescription: *mut i16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn SQLDescribeCol(statementhandle: *mut ::core::ffi::c_void, columnnumber: u16, columnname: *mut u8, bufferlength: i16, namelength: *mut i16, datatype: *mut i16, columnsize: *mut u64, decimaldigits: *mut i16, nullable: *mut i16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86",))]
    pub fn SQLDescribeCol(statementhandle: *mut ::core::ffi::c_void, columnnumber: u16, columnname: *mut u8, bufferlength: i16, namelength: *mut i16, datatype: *mut i16, columnsize: *mut u32, decimaldigits: *mut i16, nullable: *mut i16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn SQLDescribeColA(hstmt: *mut ::core::ffi::c_void, icol: u16, szcolname: *mut u8, cbcolnamemax: i16, pcbcolname: *mut i16, pfsqltype: *mut i16, pcbcoldef: *mut u64, pibscale: *mut i16, pfnullable: *mut i16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86",))]
    pub fn SQLDescribeColA(hstmt: *mut ::core::ffi::c_void, icol: u16, szcolname: *mut u8, cbcolnamemax: i16, pcbcolname: *mut i16, pfsqltype: *mut i16, pcbcoldef: *mut u32, pibscale: *mut i16, pfnullable: *mut i16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn SQLDescribeColW(hstmt: *mut ::core::ffi::c_void, icol: u16, szcolname: *mut u16, cchcolnamemax: i16, pcchcolname: *mut i16, pfsqltype: *mut i16, pcbcoldef: *mut u64, pibscale: *mut i16, pfnullable: *mut i16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86",))]
    pub fn SQLDescribeColW(hstmt: *mut ::core::ffi::c_void, icol: u16, szcolname: *mut u16, cchcolnamemax: i16, pcchcolname: *mut i16, pfsqltype: *mut i16, pcbcoldef: *mut u32, pibscale: *mut i16, pfnullable: *mut i16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn SQLDescribeParam(hstmt: *mut ::core::ffi::c_void, ipar: u16, pfsqltype: *mut i16, pcbparamdef: *mut u64, pibscale: *mut i16, pfnullable: *mut i16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86",))]
    pub fn SQLDescribeParam(hstmt: *mut ::core::ffi::c_void, ipar: u16, pfsqltype: *mut i16, pcbparamdef: *mut u32, pibscale: *mut i16, pfnullable: *mut i16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLDisconnect(connectionhandle: *mut ::core::ffi::c_void) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLDriverConnect(hdbc: *mut ::core::ffi::c_void, hwnd: isize, szconnstrin: *const u8, cchconnstrin: i16, szconnstrout: *mut u8, cchconnstroutmax: i16, pcchconnstrout: *mut i16, fdrivercompletion: u16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLDriverConnectA(hdbc: *mut ::core::ffi::c_void, hwnd: isize, szconnstrin: *const u8, cbconnstrin: i16, szconnstrout: *mut u8, cbconnstroutmax: i16, pcbconnstrout: *mut i16, fdrivercompletion: u16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLDriverConnectW(hdbc: *mut ::core::ffi::c_void, hwnd: isize, szconnstrin: *const u16, cchconnstrin: i16, szconnstrout: *mut u16, cchconnstroutmax: i16, pcchconnstrout: *mut i16, fdrivercompletion: u16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLDrivers(henv: *mut ::core::ffi::c_void, fdirection: u16, szdriverdesc: *mut u8, cchdriverdescmax: i16, pcchdriverdesc: *mut i16, szdriverattributes: *mut u8, cchdrvrattrmax: i16, pcchdrvrattr: *mut i16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLDriversA(henv: *mut ::core::ffi::c_void, fdirection: u16, szdriverdesc: *mut u8, cbdriverdescmax: i16, pcbdriverdesc: *mut i16, szdriverattributes: *mut u8, cbdrvrattrmax: i16, pcbdrvrattr: *mut i16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLDriversW(henv: *mut ::core::ffi::c_void, fdirection: u16, szdriverdesc: *mut u16, cchdriverdescmax: i16, pcchdriverdesc: *mut i16, szdriverattributes: *mut u16, cchdrvrattrmax: i16, pcchdrvrattr: *mut i16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLEndTran(handletype: i16, handle: *mut ::core::ffi::c_void, completiontype: i16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLError(environmenthandle: *mut ::core::ffi::c_void, connectionhandle: *mut ::core::ffi::c_void, statementhandle: *mut ::core::ffi::c_void, sqlstate: *mut u8, nativeerror: *mut i32, messagetext: *mut u8, bufferlength: i16, textlength: *mut i16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLErrorA(henv: *mut ::core::ffi::c_void, hdbc: *mut ::core::ffi::c_void, hstmt: *mut ::core::ffi::c_void, szsqlstate: *mut u8, pfnativeerror: *mut i32, szerrormsg: *mut u8, cberrormsgmax: i16, pcberrormsg: *mut i16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLErrorW(henv: *mut ::core::ffi::c_void, hdbc: *mut ::core::ffi::c_void, hstmt: *mut ::core::ffi::c_void, wszsqlstate: *mut u16, pfnativeerror: *mut i32, wszerrormsg: *mut u16, ccherrormsgmax: i16, pccherrormsg: *mut i16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLExecDirect(statementhandle: *mut ::core::ffi::c_void, statementtext: *const u8, textlength: i32) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLExecDirectA(hstmt: *mut ::core::ffi::c_void, szsqlstr: *const u8, cbsqlstr: i32) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLExecDirectW(hstmt: *mut ::core::ffi::c_void, szsqlstr: *const u16, textlength: i32) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLExecute(statementhandle: *mut ::core::ffi::c_void) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn SQLExtendedFetch(hstmt: *mut ::core::ffi::c_void, ffetchtype: u16, irow: i64, pcrow: *mut u64, rgfrowstatus: *mut u16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86",))]
    pub fn SQLExtendedFetch(hstmt: *mut ::core::ffi::c_void, ffetchtype: u16, irow: i32, pcrow: *mut u32, rgfrowstatus: *mut u16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLFetch(statementhandle: *mut ::core::ffi::c_void) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn SQLFetchScroll(statementhandle: *mut ::core::ffi::c_void, fetchorientation: i16, fetchoffset: i64) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86",))]
    pub fn SQLFetchScroll(statementhandle: *mut ::core::ffi::c_void, fetchorientation: i16, fetchoffset: i32) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLForeignKeys(hstmt: *mut ::core::ffi::c_void, szpkcatalogname: *const u8, cchpkcatalogname: i16, szpkschemaname: *const u8, cchpkschemaname: i16, szpktablename: *const u8, cchpktablename: i16, szfkcatalogname: *const u8, cchfkcatalogname: i16, szfkschemaname: *const u8, cchfkschemaname: i16, szfktablename: *const u8, cchfktablename: i16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLForeignKeysA(hstmt: *mut ::core::ffi::c_void, szpkcatalogname: *const u8, cbpkcatalogname: i16, szpkschemaname: *const u8, cbpkschemaname: i16, szpktablename: *const u8, cbpktablename: i16, szfkcatalogname: *const u8, cbfkcatalogname: i16, szfkschemaname: *const u8, cbfkschemaname: i16, szfktablename: *const u8, cbfktablename: i16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLForeignKeysW(hstmt: *mut ::core::ffi::c_void, szpkcatalogname: *const u16, cchpkcatalogname: i16, szpkschemaname: *const u16, cchpkschemaname: i16, szpktablename: *const u16, cchpktablename: i16, szfkcatalogname: *const u16, cchfkcatalogname: i16, szfkschemaname: *const u16, cchfkschemaname: i16, szfktablename: *const u16, cchfktablename: i16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLFreeConnect(connectionhandle: *mut ::core::ffi::c_void) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLFreeEnv(environmenthandle: *mut ::core::ffi::c_void) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLFreeHandle(handletype: i16, handle: *mut ::core::ffi::c_void) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLFreeStmt(statementhandle: *mut ::core::ffi::c_void, option: u16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLGetConnectAttr(connectionhandle: *mut ::core::ffi::c_void, attribute: i32, value: *mut ::core::ffi::c_void, bufferlength: i32, stringlengthptr: *mut i32) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLGetConnectAttrA(hdbc: *mut ::core::ffi::c_void, fattribute: i32, rgbvalue: *mut ::core::ffi::c_void, cbvaluemax: i32, pcbvalue: *mut i32) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLGetConnectAttrW(hdbc: *mut ::core::ffi::c_void, fattribute: i32, rgbvalue: *mut ::core::ffi::c_void, cbvaluemax: i32, pcbvalue: *mut i32) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLGetConnectOption(connectionhandle: *mut ::core::ffi::c_void, option: u16, value: *mut ::core::ffi::c_void) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLGetConnectOptionA(hdbc: *mut ::core::ffi::c_void, foption: u16, pvparam: *mut ::core::ffi::c_void) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLGetConnectOptionW(hdbc: *mut ::core::ffi::c_void, foption: u16, pvparam: *mut ::core::ffi::c_void) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLGetCursorName(statementhandle: *mut ::core::ffi::c_void, cursorname: *mut u8, bufferlength: i16, namelengthptr: *mut i16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLGetCursorNameA(hstmt: *mut ::core::ffi::c_void, szcursor: *mut u8, cbcursormax: i16, pcbcursor: *mut i16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLGetCursorNameW(hstmt: *mut ::core::ffi::c_void, szcursor: *mut u16, cchcursormax: i16, pcchcursor: *mut i16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn SQLGetData(statementhandle: *mut ::core::ffi::c_void, columnnumber: u16, targettype: i16, targetvalue: *mut ::core::ffi::c_void, bufferlength: i64, strlen_or_indptr: *mut i64) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86",))]
    pub fn SQLGetData(statementhandle: *mut ::core::ffi::c_void, columnnumber: u16, targettype: i16, targetvalue: *mut ::core::ffi::c_void, bufferlength: i32, strlen_or_indptr: *mut i32) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLGetDescField(descriptorhandle: *mut ::core::ffi::c_void, recnumber: i16, fieldidentifier: i16, value: *mut ::core::ffi::c_void, bufferlength: i32, stringlength: *mut i32) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLGetDescFieldA(hdesc: *mut ::core::ffi::c_void, irecord: i16, ifield: i16, rgbvalue: *mut ::core::ffi::c_void, cbbufferlength: i32, stringlength: *mut i32) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLGetDescFieldW(hdesc: *mut ::core::ffi::c_void, irecord: i16, ifield: i16, rgbvalue: *mut ::core::ffi::c_void, cbbufferlength: i32, stringlength: *mut i32) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn SQLGetDescRec(descriptorhandle: *mut ::core::ffi::c_void, recnumber: i16, name: *mut u8, bufferlength: i16, stringlengthptr: *mut i16, typeptr: *mut i16, subtypeptr: *mut i16, lengthptr: *mut i64, precisionptr: *mut i16, scaleptr: *mut i16, nullableptr: *mut i16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86",))]
    pub fn SQLGetDescRec(descriptorhandle: *mut ::core::ffi::c_void, recnumber: i16, name: *mut u8, bufferlength: i16, stringlengthptr: *mut i16, typeptr: *mut i16, subtypeptr: *mut i16, lengthptr: *mut i32, precisionptr: *mut i16, scaleptr: *mut i16, nullableptr: *mut i16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn SQLGetDescRecA(hdesc: *mut ::core::ffi::c_void, irecord: i16, szname: *mut u8, cbnamemax: i16, pcbname: *mut i16, pftype: *mut i16, pfsubtype: *mut i16, plength: *mut i64, pprecision: *mut i16, pscale: *mut i16, pnullable: *mut i16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86",))]
    pub fn SQLGetDescRecA(hdesc: *mut ::core::ffi::c_void, irecord: i16, szname: *mut u8, cbnamemax: i16, pcbname: *mut i16, pftype: *mut i16, pfsubtype: *mut i16, plength: *mut i32, pprecision: *mut i16, pscale: *mut i16, pnullable: *mut i16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn SQLGetDescRecW(hdesc: *mut ::core::ffi::c_void, irecord: i16, szname: *mut u16, cchnamemax: i16, pcchname: *mut i16, pftype: *mut i16, pfsubtype: *mut i16, plength: *mut i64, pprecision: *mut i16, pscale: *mut i16, pnullable: *mut i16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86",))]
    pub fn SQLGetDescRecW(hdesc: *mut ::core::ffi::c_void, irecord: i16, szname: *mut u16, cchnamemax: i16, pcchname: *mut i16, pftype: *mut i16, pfsubtype: *mut i16, plength: *mut i32, pprecision: *mut i16, pscale: *mut i16, pnullable: *mut i16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLGetDiagField(handletype: i16, handle: *mut ::core::ffi::c_void, recnumber: i16, diagidentifier: i16, diaginfo: *mut ::core::ffi::c_void, bufferlength: i16, stringlength: *mut i16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLGetDiagFieldA(fhandletype: i16, handle: *mut ::core::ffi::c_void, irecord: i16, fdiagfield: i16, rgbdiaginfo: *mut ::core::ffi::c_void, cbdiaginfomax: i16, pcbdiaginfo: *mut i16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLGetDiagFieldW(fhandletype: i16, handle: *mut ::core::ffi::c_void, irecord: i16, fdiagfield: i16, rgbdiaginfo: *mut ::core::ffi::c_void, cbbufferlength: i16, pcbstringlength: *mut i16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLGetDiagRec(handletype: i16, handle: *mut ::core::ffi::c_void, recnumber: i16, sqlstate: *mut u8, nativeerror: *mut i32, messagetext: *mut u8, bufferlength: i16, textlength: *mut i16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLGetDiagRecA(fhandletype: i16, handle: *mut ::core::ffi::c_void, irecord: i16, szsqlstate: *mut u8, pfnativeerror: *mut i32, szerrormsg: *mut u8, cberrormsgmax: i16, pcberrormsg: *mut i16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLGetDiagRecW(fhandletype: i16, handle: *mut ::core::ffi::c_void, irecord: i16, szsqlstate: *mut u16, pfnativeerror: *mut i32, szerrormsg: *mut u16, ccherrormsgmax: i16, pccherrormsg: *mut i16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLGetEnvAttr(environmenthandle: *mut ::core::ffi::c_void, attribute: i32, value: *mut ::core::ffi::c_void, bufferlength: i32, stringlength: *mut i32) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLGetFunctions(connectionhandle: *mut ::core::ffi::c_void, functionid: u16, supported: *mut u16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLGetInfo(connectionhandle: *mut ::core::ffi::c_void, infotype: u16, infovalue: *mut ::core::ffi::c_void, bufferlength: i16, stringlengthptr: *mut i16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLGetInfoA(hdbc: *mut ::core::ffi::c_void, finfotype: u16, rgbinfovalue: *mut ::core::ffi::c_void, cbinfovaluemax: i16, pcbinfovalue: *mut i16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLGetInfoW(hdbc: *mut ::core::ffi::c_void, finfotype: u16, rgbinfovalue: *mut ::core::ffi::c_void, cbinfovaluemax: i16, pcbinfovalue: *mut i16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SQLGetNextEnumeration(henumhandle: super::super::Foundation::HANDLE, prgenumdata: *mut u8, pienumlength: *mut i32) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLGetStmtAttr(statementhandle: *mut ::core::ffi::c_void, attribute: i32, value: *mut ::core::ffi::c_void, bufferlength: i32, stringlength: *mut i32) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLGetStmtAttrA(hstmt: *mut ::core::ffi::c_void, fattribute: i32, rgbvalue: *mut ::core::ffi::c_void, cbvaluemax: i32, pcbvalue: *mut i32) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLGetStmtAttrW(hstmt: *mut ::core::ffi::c_void, fattribute: i32, rgbvalue: *mut ::core::ffi::c_void, cbvaluemax: i32, pcbvalue: *mut i32) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLGetStmtOption(statementhandle: *mut ::core::ffi::c_void, option: u16, value: *mut ::core::ffi::c_void) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLGetTypeInfo(statementhandle: *mut ::core::ffi::c_void, datatype: i16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLGetTypeInfoA(statementhandle: *mut ::core::ffi::c_void, datatype: i16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLGetTypeInfoW(statementhandle: *mut ::core::ffi::c_void, datatype: i16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SQLInitEnumServers(pwchservername: super::super::Foundation::PWSTR, pwchinstancename: super::super::Foundation::PWSTR) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_System_Search`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SQLLinkedCatalogsA(param0: *mut ::core::ffi::c_void, param1: super::super::Foundation::PSTR, param2: i16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SQLLinkedCatalogsW(param0: *mut ::core::ffi::c_void, param1: super::super::Foundation::PWSTR, param2: i16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLLinkedServers(param0: *mut ::core::ffi::c_void) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLMoreResults(hstmt: *mut ::core::ffi::c_void) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLNativeSql(hdbc: *mut ::core::ffi::c_void, szsqlstrin: *const u8, cchsqlstrin: i32, szsqlstr: *mut u8, cchsqlstrmax: i32, pcbsqlstr: *mut i32) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLNativeSqlA(hdbc: *mut ::core::ffi::c_void, szsqlstrin: *const u8, cbsqlstrin: i32, szsqlstr: *mut u8, cbsqlstrmax: i32, pcbsqlstr: *mut i32) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLNativeSqlW(hdbc: *mut ::core::ffi::c_void, szsqlstrin: *const u16, cchsqlstrin: i32, szsqlstr: *mut u16, cchsqlstrmax: i32, pcchsqlstr: *mut i32) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLNumParams(hstmt: *mut ::core::ffi::c_void, pcpar: *mut i16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLNumResultCols(statementhandle: *mut ::core::ffi::c_void, columncount: *mut i16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLParamData(statementhandle: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn SQLParamOptions(hstmt: *mut ::core::ffi::c_void, crow: u64, pirow: *mut u64) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86",))]
    pub fn SQLParamOptions(hstmt: *mut ::core::ffi::c_void, crow: u32, pirow: *mut u32) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLPrepare(statementhandle: *mut ::core::ffi::c_void, statementtext: *const u8, textlength: i32) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLPrepareA(hstmt: *mut ::core::ffi::c_void, szsqlstr: *const u8, cbsqlstr: i32) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLPrepareW(hstmt: *mut ::core::ffi::c_void, szsqlstr: *const u16, cchsqlstr: i32) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLPrimaryKeys(hstmt: *mut ::core::ffi::c_void, szcatalogname: *const u8, cchcatalogname: i16, szschemaname: *const u8, cchschemaname: i16, sztablename: *const u8, cchtablename: i16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLPrimaryKeysA(hstmt: *mut ::core::ffi::c_void, szcatalogname: *const u8, cbcatalogname: i16, szschemaname: *const u8, cbschemaname: i16, sztablename: *const u8, cbtablename: i16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLPrimaryKeysW(hstmt: *mut ::core::ffi::c_void, szcatalogname: *const u16, cchcatalogname: i16, szschemaname: *const u16, cchschemaname: i16, sztablename: *const u16, cchtablename: i16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLProcedureColumns(hstmt: *mut ::core::ffi::c_void, szcatalogname: *const u8, cchcatalogname: i16, szschemaname: *const u8, cchschemaname: i16, szprocname: *const u8, cchprocname: i16, szcolumnname: *const u8, cchcolumnname: i16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLProcedureColumnsA(hstmt: *mut ::core::ffi::c_void, szcatalogname: *const u8, cbcatalogname: i16, szschemaname: *const u8, cbschemaname: i16, szprocname: *const u8, cbprocname: i16, szcolumnname: *const u8, cbcolumnname: i16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLProcedureColumnsW(hstmt: *mut ::core::ffi::c_void, szcatalogname: *const u16, cchcatalogname: i16, szschemaname: *const u16, cchschemaname: i16, szprocname: *const u16, cchprocname: i16, szcolumnname: *const u16, cchcolumnname: i16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLProcedures(hstmt: *mut ::core::ffi::c_void, szcatalogname: *const u8, cchcatalogname: i16, szschemaname: *const u8, cchschemaname: i16, szprocname: *const u8, cchprocname: i16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLProceduresA(hstmt: *mut ::core::ffi::c_void, szcatalogname: *const u8, cbcatalogname: i16, szschemaname: *const u8, cbschemaname: i16, szprocname: *const u8, cbprocname: i16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLProceduresW(hstmt: *mut ::core::ffi::c_void, szcatalogname: *const u16, cchcatalogname: i16, szschemaname: *const u16, cchschemaname: i16, szprocname: *const u16, cchprocname: i16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn SQLPutData(statementhandle: *mut ::core::ffi::c_void, data: *const ::core::ffi::c_void, strlen_or_ind: i64) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86",))]
    pub fn SQLPutData(statementhandle: *mut ::core::ffi::c_void, data: *const ::core::ffi::c_void, strlen_or_ind: i32) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn SQLRowCount(statementhandle: *const ::core::ffi::c_void, rowcount: *mut i64) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86",))]
    pub fn SQLRowCount(statementhandle: *const ::core::ffi::c_void, rowcount: *mut i32) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLSetConnectAttr(connectionhandle: *mut ::core::ffi::c_void, attribute: i32, value: *const ::core::ffi::c_void, stringlength: i32) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLSetConnectAttrA(hdbc: *mut ::core::ffi::c_void, fattribute: i32, rgbvalue: *const ::core::ffi::c_void, cbvalue: i32) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLSetConnectAttrW(hdbc: *mut ::core::ffi::c_void, fattribute: i32, rgbvalue: *const ::core::ffi::c_void, cbvalue: i32) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn SQLSetConnectOption(connectionhandle: *mut ::core::ffi::c_void, option: u16, value: u64) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86",))]
    pub fn SQLSetConnectOption(connectionhandle: *mut ::core::ffi::c_void, option: u16, value: u32) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn SQLSetConnectOptionA(hdbc: *mut ::core::ffi::c_void, foption: u16, vparam: u64) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86",))]
    pub fn SQLSetConnectOptionA(hdbc: *mut ::core::ffi::c_void, foption: u16, vparam: u32) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn SQLSetConnectOptionW(hdbc: *mut ::core::ffi::c_void, foption: u16, vparam: u64) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86",))]
    pub fn SQLSetConnectOptionW(hdbc: *mut ::core::ffi::c_void, foption: u16, vparam: u32) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLSetCursorName(statementhandle: *mut ::core::ffi::c_void, cursorname: *const u8, namelength: i16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLSetCursorNameA(hstmt: *mut ::core::ffi::c_void, szcursor: *const u8, cbcursor: i16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLSetCursorNameW(hstmt: *mut ::core::ffi::c_void, szcursor: *const u16, cchcursor: i16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLSetDescField(descriptorhandle: *mut ::core::ffi::c_void, recnumber: i16, fieldidentifier: i16, value: *const ::core::ffi::c_void, bufferlength: i32) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLSetDescFieldW(descriptorhandle: *mut ::core::ffi::c_void, recnumber: i16, fieldidentifier: i16, value: *mut ::core::ffi::c_void, bufferlength: i32) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn SQLSetDescRec(descriptorhandle: *mut ::core::ffi::c_void, recnumber: i16, r#type: i16, subtype: i16, length: i64, precision: i16, scale: i16, data: *mut ::core::ffi::c_void, stringlength: *mut i64, indicator: *mut i64) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86",))]
    pub fn SQLSetDescRec(descriptorhandle: *mut ::core::ffi::c_void, recnumber: i16, r#type: i16, subtype: i16, length: i32, precision: i16, scale: i16, data: *mut ::core::ffi::c_void, stringlength: *mut i32, indicator: *mut i32) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLSetEnvAttr(environmenthandle: *mut ::core::ffi::c_void, attribute: i32, value: *const ::core::ffi::c_void, stringlength: i32) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn SQLSetParam(statementhandle: *mut ::core::ffi::c_void, parameternumber: u16, valuetype: i16, parametertype: i16, lengthprecision: u64, parameterscale: i16, parametervalue: *const ::core::ffi::c_void, strlen_or_ind: *mut i64) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86",))]
    pub fn SQLSetParam(statementhandle: *mut ::core::ffi::c_void, parameternumber: u16, valuetype: i16, parametertype: i16, lengthprecision: u32, parameterscale: i16, parametervalue: *const ::core::ffi::c_void, strlen_or_ind: *mut i32) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn SQLSetPos(hstmt: *mut ::core::ffi::c_void, irow: u64, foption: u16, flock: u16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86",))]
    pub fn SQLSetPos(hstmt: *mut ::core::ffi::c_void, irow: u16, foption: u16, flock: u16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn SQLSetScrollOptions(hstmt: *mut ::core::ffi::c_void, fconcurrency: u16, crowkeyset: i64, crowrowset: u16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86",))]
    pub fn SQLSetScrollOptions(hstmt: *mut ::core::ffi::c_void, fconcurrency: u16, crowkeyset: i32, crowrowset: u16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLSetStmtAttr(statementhandle: *mut ::core::ffi::c_void, attribute: i32, value: *const ::core::ffi::c_void, stringlength: i32) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLSetStmtAttrW(hstmt: *mut ::core::ffi::c_void, fattribute: i32, rgbvalue: *mut ::core::ffi::c_void, cbvaluemax: i32) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn SQLSetStmtOption(statementhandle: *mut ::core::ffi::c_void, option: u16, value: u64) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86",))]
    pub fn SQLSetStmtOption(statementhandle: *mut ::core::ffi::c_void, option: u16, value: u32) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLSpecialColumns(statementhandle: *mut ::core::ffi::c_void, identifiertype: u16, catalogname: *const u8, namelength1: i16, schemaname: *const u8, namelength2: i16, tablename: *const u8, namelength3: i16, scope: u16, nullable: u16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLSpecialColumnsA(hstmt: *mut ::core::ffi::c_void, fcoltype: u16, szcatalogname: *const u8, cbcatalogname: i16, szschemaname: *const u8, cbschemaname: i16, sztablename: *const u8, cbtablename: i16, fscope: u16, fnullable: u16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLSpecialColumnsW(hstmt: *mut ::core::ffi::c_void, fcoltype: u16, szcatalogname: *const u16, cchcatalogname: i16, szschemaname: *const u16, cchschemaname: i16, sztablename: *const u16, cchtablename: i16, fscope: u16, fnullable: u16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLStatistics(statementhandle: *mut ::core::ffi::c_void, catalogname: *const u8, namelength1: i16, schemaname: *const u8, namelength2: i16, tablename: *const u8, namelength3: i16, unique: u16, reserved: u16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLStatisticsA(hstmt: *mut ::core::ffi::c_void, szcatalogname: *const u8, cbcatalogname: i16, szschemaname: *const u8, cbschemaname: i16, sztablename: *const u8, cbtablename: i16, funique: u16, faccuracy: u16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLStatisticsW(hstmt: *mut ::core::ffi::c_void, szcatalogname: *const u16, cchcatalogname: i16, szschemaname: *const u16, cchschemaname: i16, sztablename: *const u16, cchtablename: i16, funique: u16, faccuracy: u16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLTablePrivileges(hstmt: *mut ::core::ffi::c_void, szcatalogname: *const u8, cchcatalogname: i16, szschemaname: *const u8, cchschemaname: i16, sztablename: *const u8, cchtablename: i16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLTablePrivilegesA(hstmt: *mut ::core::ffi::c_void, szcatalogname: *const u8, cbcatalogname: i16, szschemaname: *const u8, cbschemaname: i16, sztablename: *const u8, cbtablename: i16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLTablePrivilegesW(hstmt: *mut ::core::ffi::c_void, szcatalogname: *const u16, cchcatalogname: i16, szschemaname: *const u16, cchschemaname: i16, sztablename: *const u16, cchtablename: i16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLTables(statementhandle: *mut ::core::ffi::c_void, catalogname: *const u8, namelength1: i16, schemaname: *const u8, namelength2: i16, tablename: *const u8, namelength3: i16, tabletype: *const u8, namelength4: i16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLTablesA(hstmt: *mut ::core::ffi::c_void, szcatalogname: *const u8, cbcatalogname: i16, szschemaname: *const u8, cbschemaname: i16, sztablename: *const u8, cbtablename: i16, sztabletype: *const u8, cbtabletype: i16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLTablesW(hstmt: *mut ::core::ffi::c_void, szcatalogname: *const u16, cchcatalogname: i16, szschemaname: *const u16, cchschemaname: i16, sztablename: *const u16, cchtablename: i16, sztabletype: *const u16, cchtabletype: i16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLTransact(environmenthandle: *mut ::core::ffi::c_void, connectionhandle: *mut ::core::ffi::c_void, completiontype: u16) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn bcp_batch(param0: *mut ::core::ffi::c_void) -> i32;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn bcp_bind(param0: *mut ::core::ffi::c_void, param1: *mut u8, param2: i32, param3: i32, param4: *mut u8, param5: i32, param6: i32, param7: i32) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn bcp_colfmt(param0: *mut ::core::ffi::c_void, param1: i32, param2: u8, param3: i32, param4: i32, param5: *mut u8, param6: i32, param7: i32) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn bcp_collen(param0: *mut ::core::ffi::c_void, param1: i32, param2: i32) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn bcp_colptr(param0: *mut ::core::ffi::c_void, param1: *mut u8, param2: i32) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn bcp_columns(param0: *mut ::core::ffi::c_void, param1: i32) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn bcp_control(param0: *mut ::core::ffi::c_void, param1: i32, param2: *mut ::core::ffi::c_void) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn bcp_done(param0: *mut ::core::ffi::c_void) -> i32;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn bcp_exec(param0: *mut ::core::ffi::c_void, param1: *mut i32) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn bcp_getcolfmt(param0: *mut ::core::ffi::c_void, param1: i32, param2: i32, param3: *mut ::core::ffi::c_void, param4: i32, param5: *mut i32) -> i16;
    #[doc = "*Required features: `Win32_System_Search`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn bcp_initA(param0: *mut ::core::ffi::c_void, param1: super::super::Foundation::PSTR, param2: super::super::Foundation::PSTR, param3: super::super::Foundation::PSTR, param4: i32) -> i16;
    #[doc = "*Required features: `Win32_System_Search`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn bcp_initW(param0: *mut ::core::ffi::c_void, param1: super::super::Foundation::PWSTR, param2: super::super::Foundation::PWSTR, param3: super::super::Foundation::PWSTR, param4: i32) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn bcp_moretext(param0: *mut ::core::ffi::c_void, param1: i32, param2: *mut u8) -> i16;
    #[doc = "*Required features: `Win32_System_Search`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn bcp_readfmtA(param0: *mut ::core::ffi::c_void, param1: super::super::Foundation::PSTR) -> i16;
    #[doc = "*Required features: `Win32_System_Search`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn bcp_readfmtW(param0: *mut ::core::ffi::c_void, param1: super::super::Foundation::PWSTR) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn bcp_sendrow(param0: *mut ::core::ffi::c_void) -> i16;
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn bcp_setcolfmt(param0: *mut ::core::ffi::c_void, param1: i32, param2: i32, param3: *mut ::core::ffi::c_void, param4: i32) -> i16;
    #[doc = "*Required features: `Win32_System_Search`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn bcp_writefmtA(param0: *mut ::core::ffi::c_void, param1: super::super::Foundation::PSTR) -> i16;
    #[doc = "*Required features: `Win32_System_Search`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn bcp_writefmtW(param0: *mut ::core::ffi::c_void, param1: super::super::Foundation::PWSTR) -> i16;
    #[doc = "*Required features: `Win32_System_Search`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn dbprtypeA(param0: i32) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_System_Search`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn dbprtypeW(param0: i32) -> super::super::Foundation::PWSTR;
}
