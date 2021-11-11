#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Win32_System_Search_Common")]
pub mod Common;
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn ODBCGetTryWaitValue();
    #[doc = "*Required features: `Win32_System_Search`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ODBCSetTryWaitValue();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLAllocConnect();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLAllocEnv();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLAllocHandle();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLAllocHandleStd();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLAllocStmt();
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn SQLBindCol();
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86",))]
    pub fn SQLBindCol();
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn SQLBindParam();
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86",))]
    pub fn SQLBindParam();
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn SQLBindParameter();
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86",))]
    pub fn SQLBindParameter();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLBrowseConnect();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLBrowseConnectA();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLBrowseConnectW();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLBulkOperations();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLCancel();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLCancelHandle();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLCloseCursor();
    #[doc = "*Required features: `Win32_System_Search`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SQLCloseEnumServers();
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn SQLColAttribute();
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86",))]
    pub fn SQLColAttribute();
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn SQLColAttributeA();
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86",))]
    pub fn SQLColAttributeA();
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn SQLColAttributeW();
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86",))]
    pub fn SQLColAttributeW();
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn SQLColAttributes();
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86",))]
    pub fn SQLColAttributes();
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn SQLColAttributesA();
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86",))]
    pub fn SQLColAttributesA();
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn SQLColAttributesW();
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86",))]
    pub fn SQLColAttributesW();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLColumnPrivileges();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLColumnPrivilegesA();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLColumnPrivilegesW();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLColumns();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLColumnsA();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLColumnsW();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLCompleteAsync();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLConnect();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLConnectA();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLConnectW();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLCopyDesc();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLDataSources();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLDataSourcesA();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLDataSourcesW();
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn SQLDescribeCol();
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86",))]
    pub fn SQLDescribeCol();
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn SQLDescribeColA();
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86",))]
    pub fn SQLDescribeColA();
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn SQLDescribeColW();
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86",))]
    pub fn SQLDescribeColW();
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn SQLDescribeParam();
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86",))]
    pub fn SQLDescribeParam();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLDisconnect();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLDriverConnect();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLDriverConnectA();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLDriverConnectW();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLDrivers();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLDriversA();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLDriversW();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLEndTran();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLError();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLErrorA();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLErrorW();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLExecDirect();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLExecDirectA();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLExecDirectW();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLExecute();
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn SQLExtendedFetch();
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86",))]
    pub fn SQLExtendedFetch();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLFetch();
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn SQLFetchScroll();
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86",))]
    pub fn SQLFetchScroll();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLForeignKeys();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLForeignKeysA();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLForeignKeysW();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLFreeConnect();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLFreeEnv();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLFreeHandle();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLFreeStmt();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLGetConnectAttr();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLGetConnectAttrA();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLGetConnectAttrW();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLGetConnectOption();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLGetConnectOptionA();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLGetConnectOptionW();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLGetCursorName();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLGetCursorNameA();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLGetCursorNameW();
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn SQLGetData();
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86",))]
    pub fn SQLGetData();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLGetDescField();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLGetDescFieldA();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLGetDescFieldW();
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn SQLGetDescRec();
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86",))]
    pub fn SQLGetDescRec();
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn SQLGetDescRecA();
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86",))]
    pub fn SQLGetDescRecA();
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn SQLGetDescRecW();
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86",))]
    pub fn SQLGetDescRecW();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLGetDiagField();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLGetDiagFieldA();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLGetDiagFieldW();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLGetDiagRec();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLGetDiagRecA();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLGetDiagRecW();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLGetEnvAttr();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLGetFunctions();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLGetInfo();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLGetInfoA();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLGetInfoW();
    #[doc = "*Required features: `Win32_System_Search`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SQLGetNextEnumeration();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLGetStmtAttr();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLGetStmtAttrA();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLGetStmtAttrW();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLGetStmtOption();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLGetTypeInfo();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLGetTypeInfoA();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLGetTypeInfoW();
    #[doc = "*Required features: `Win32_System_Search`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SQLInitEnumServers();
    #[doc = "*Required features: `Win32_System_Search`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SQLLinkedCatalogsA();
    #[doc = "*Required features: `Win32_System_Search`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SQLLinkedCatalogsW();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLLinkedServers();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLMoreResults();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLNativeSql();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLNativeSqlA();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLNativeSqlW();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLNumParams();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLNumResultCols();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLParamData();
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn SQLParamOptions();
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86",))]
    pub fn SQLParamOptions();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLPrepare();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLPrepareA();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLPrepareW();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLPrimaryKeys();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLPrimaryKeysA();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLPrimaryKeysW();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLProcedureColumns();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLProcedureColumnsA();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLProcedureColumnsW();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLProcedures();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLProceduresA();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLProceduresW();
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn SQLPutData();
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86",))]
    pub fn SQLPutData();
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn SQLRowCount();
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86",))]
    pub fn SQLRowCount();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLSetConnectAttr();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLSetConnectAttrA();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLSetConnectAttrW();
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn SQLSetConnectOption();
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86",))]
    pub fn SQLSetConnectOption();
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn SQLSetConnectOptionA();
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86",))]
    pub fn SQLSetConnectOptionA();
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn SQLSetConnectOptionW();
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86",))]
    pub fn SQLSetConnectOptionW();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLSetCursorName();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLSetCursorNameA();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLSetCursorNameW();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLSetDescField();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLSetDescFieldW();
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn SQLSetDescRec();
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86",))]
    pub fn SQLSetDescRec();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLSetEnvAttr();
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn SQLSetParam();
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86",))]
    pub fn SQLSetParam();
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn SQLSetPos();
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86",))]
    pub fn SQLSetPos();
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn SQLSetScrollOptions();
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86",))]
    pub fn SQLSetScrollOptions();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLSetStmtAttr();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLSetStmtAttrW();
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn SQLSetStmtOption();
    #[doc = "*Required features: `Win32_System_Search`*"]
    #[cfg(any(target_arch = "x86",))]
    pub fn SQLSetStmtOption();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLSpecialColumns();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLSpecialColumnsA();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLSpecialColumnsW();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLStatistics();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLStatisticsA();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLStatisticsW();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLTablePrivileges();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLTablePrivilegesA();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLTablePrivilegesW();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLTables();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLTablesA();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLTablesW();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn SQLTransact();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn bcp_batch();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn bcp_bind();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn bcp_colfmt();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn bcp_collen();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn bcp_colptr();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn bcp_columns();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn bcp_control();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn bcp_done();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn bcp_exec();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn bcp_getcolfmt();
    #[doc = "*Required features: `Win32_System_Search`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn bcp_initA();
    #[doc = "*Required features: `Win32_System_Search`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn bcp_initW();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn bcp_moretext();
    #[doc = "*Required features: `Win32_System_Search`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn bcp_readfmtA();
    #[doc = "*Required features: `Win32_System_Search`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn bcp_readfmtW();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn bcp_sendrow();
    #[doc = "*Required features: `Win32_System_Search`*"]
    pub fn bcp_setcolfmt();
    #[doc = "*Required features: `Win32_System_Search`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn bcp_writefmtA();
    #[doc = "*Required features: `Win32_System_Search`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn bcp_writefmtW();
    #[doc = "*Required features: `Win32_System_Search`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn dbprtypeA();
    #[doc = "*Required features: `Win32_System_Search`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn dbprtypeW();
}
