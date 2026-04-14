/********************************************************
*                                                       *
*   Copyright (C) Microsoft. All rights reserved.       *
*                                                       *
********************************************************/

//-----------------------------------------------------------------------------
// File:		odbcss.h
//
// Contents:
//
// Comments:	This is the application include file for the SQL Server driver specific defines.
//
//-----------------------------------------------------------------------------

#ifndef __ODBCSS
#define __ODBCSS
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#ifdef __cplusplus
extern "C" { 			/* Assume C declarations for C++   */
#endif  /* __cplusplus */

//	Useful defines
#define SQL_MAX_SQLSERVERNAME	128		// max SQL Server identifier length

//	SQLSetConnectOption/SQLSetStmtOption driver specific defines.
//	Microsoft has 1200 thru 1249 reserved for Microsoft SQL Server driver usage.

//	Connection Options
#define SQL_COPT_SS_BASE				1200
#define SQL_COPT_SS_REMOTE_PWD			(SQL_COPT_SS_BASE+1) // dbrpwset SQLSetConnectOption only
#define SQL_COPT_SS_USE_PROC_FOR_PREP	(SQL_COPT_SS_BASE+2) // Use create proc for SQLPrepare
#define SQL_COPT_SS_INTEGRATED_SECURITY	(SQL_COPT_SS_BASE+3) // Force integrated security on login
#define SQL_COPT_SS_PRESERVE_CURSORS	(SQL_COPT_SS_BASE+4) // Preserve server cursors after SQLTransact
#define SQL_COPT_SS_USER_DATA			(SQL_COPT_SS_BASE+5) // dbgetuserdata/dbsetuserdata
#define SQL_COPT_SS_ENLIST_IN_DTC		SQL_ATTR_ENLIST_IN_DTC // Enlist in a DTC transaction
#define SQL_COPT_SS_ENLIST_IN_XA		SQL_ATTR_ENLIST_IN_XA // Enlist in a XA transaction
//SQL_ATTR_CONNECTION_DEAD 1209 (It will return current connection status, it will not go to server)
#define SQL_COPT_SS_FALLBACK_CONNECT	(SQL_COPT_SS_BASE+10) // Enables FallBack connections
#define SQL_COPT_SS_PERF_DATA			(SQL_COPT_SS_BASE+11) // Used to access SQL Server ODBC driver performance data
#define SQL_COPT_SS_PERF_DATA_LOG		(SQL_COPT_SS_BASE+12) // Used to set the logfile name for the Performance data
#define SQL_COPT_SS_PERF_QUERY_INTERVAL (SQL_COPT_SS_BASE+13) // Used to set the query logging threshold in milliseconds.
#define SQL_COPT_SS_PERF_QUERY_LOG		(SQL_COPT_SS_BASE+14) // Used to set the logfile name for saving queryies.
#define SQL_COPT_SS_PERF_QUERY			(SQL_COPT_SS_BASE+15) // Used to start and stop query logging.
#define SQL_COPT_SS_PERF_DATA_LOG_NOW	(SQL_COPT_SS_BASE+16) // Used to make a statistics log entry to disk.
#define SQL_COPT_SS_QUOTED_IDENT		(SQL_COPT_SS_BASE+17) // Enable/Disable Quoted Identifiers
#define SQL_COPT_SS_ANSI_NPW			(SQL_COPT_SS_BASE+18) // Enable/Disable ANSI NULL, Padding and Warnings
#define SQL_COPT_SS_BCP					(SQL_COPT_SS_BASE+19) // Allow BCP usage on connection
#define SQL_COPT_SS_TRANSLATE			(SQL_COPT_SS_BASE+20) // Perform code page translation
#define SQL_COPT_SS_ATTACHDBFILENAME	(SQL_COPT_SS_BASE+21) // File name to be attached as a database
#define SQL_COPT_SS_CONCAT_NULL			(SQL_COPT_SS_BASE+22) // Enable/Disable CONCAT_NULL_YIELDS_NULL
#define SQL_COPT_SS_ENCRYPT             (SQL_COPT_SS_BASE+23) // Allow strong encryption for data

#define SQL_COPT_SS_MAX_USED			SQL_COPT_SS_ENCRYPT

//	Statement Options
#define SQL_SOPT_SS_BASE				1225
#define SQL_SOPT_SS_TEXTPTR_LOGGING		(SQL_SOPT_SS_BASE+0) // Text pointer logging
#define SQL_SOPT_SS_CURRENT_COMMAND		(SQL_SOPT_SS_BASE+1) // dbcurcmd SQLGetStmtOption only
#define SQL_SOPT_SS_HIDDEN_COLUMNS		(SQL_SOPT_SS_BASE+2) // Expose FOR BROWSE hidden columns
#define SQL_SOPT_SS_NOBROWSETABLE		(SQL_SOPT_SS_BASE+3) // Set NOBROWSETABLE option
#define SQL_SOPT_SS_REGIONALIZE			(SQL_SOPT_SS_BASE+4) // Regionalize output character conversions
#define SQL_SOPT_SS_CURSOR_OPTIONS		(SQL_SOPT_SS_BASE+5) // Server cursor options
#define SQL_SOPT_SS_NOCOUNT_STATUS      (SQL_SOPT_SS_BASE+6) // Real vs. Not Real row count indicator
#define SQL_SOPT_SS_DEFER_PREPARE		(SQL_SOPT_SS_BASE+7) // Defer prepare until necessary

#define SQL_SOPT_SS_MAX_USED			SQL_SOPT_SS_DEFER_PREPARE

#define SQL_COPT_SS_BASE_EX   1240
#define SQL_COPT_SS_BROWSE_CONNECT		(SQL_COPT_SS_BASE_EX+1) // Browse connect mode of operation
#define SQL_COPT_SS_BROWSE_SERVER		(SQL_COPT_SS_BASE_EX+2) // Single Server browse request.
#define SQL_COPT_SS_WARN_ON_CP_ERROR	(SQL_COPT_SS_BASE_EX+3) // Issues warning when data from the server
																// had a loss during code page conversion.

#define SQL_COPT_SS_CONNECTION_DEAD		(SQL_COPT_SS_BASE_EX+4) // dbdead SQLGetConnectOption only
																// It will try to ping the server.
																// Expensive connection check

#define SQL_COPT_SS_BROWSE_CACHE_DATA   (SQL_COPT_SS_BASE_EX+5) //Determines if we should cache browse info
																//Used when returned buffer is greater then ODBC limit (32K)

#define SQL_COPT_SS_RESET_CONNECTION	(SQL_COPT_SS_BASE_EX+6)	//When this option is set, we will perform connection reset
																//on next packet


#define SQL_COPT_SS_EX_MAX_USED			SQL_COPT_SS_RESET_CONNECTION

//	Defines for use with SQL_COPT_SS_USE_PROC_FOR_PREP
#define SQL_UP_OFF		0L			//	Procedures won't be used for prepare
#define SQL_UP_ON		1L			//	Procedures will be used for prepare
#define SQL_UP_ON_DROP	2L			//	Temp procedures will be explicitly dropped
#define SQL_UP_DEFAULT	SQL_UP_ON

//	Defines for use with SQL_COPT_SS_INTEGRATED_SECURITY - Pre-Connect Option only
#define SQL_IS_OFF		0L			//	Integrated security isn't used
#define SQL_IS_ON		1L			//	Integrated security is used
#define SQL_IS_DEFAULT	SQL_IS_OFF

//	Defines for use with SQL_COPT_SS_PRESERVE_CURSORS
#define SQL_PC_OFF		0L			//	Cursors are closed on SQLTransact
#define SQL_PC_ON		1L			//	Cursors remain open on SQLTransact
#define SQL_PC_DEFAULT	SQL_PC_OFF

//	Defines for use with SQL_COPT_SS_USER_DATA
#define SQL_UD_NOTSET	NULL			//	No user data pointer set

//	Defines for use with SQL_COPT_SS_TRANSLATE
#define SQL_XL_OFF		0L			//	Code page translation is not performed
#define SQL_XL_ON		1L			//	Code page translation is performed
#define SQL_XL_DEFAULT	SQL_XL_ON

//	Defines for use with SQL_COPT_SS_FALLBACK_CONNECT - Pre-Connect Option only
#define SQL_FB_OFF		0L			//	FallBack connections are disabled
#define SQL_FB_ON		1L			//	FallBack connections are enabled
#define SQL_FB_DEFAULT	SQL_FB_OFF

//	Defines for use with SQL_COPT_SS_BCP - Pre-Connect Option only
#define SQL_BCP_OFF		0L			//	BCP is not allowed on connection
#define SQL_BCP_ON		1L			//	BCP is allowed on connection
#define SQL_BCP_DEFAULT	SQL_BCP_OFF

//	Defines for use with SQL_COPT_SS_QUOTED_IDENT
#define SQL_QI_OFF		0L			//	Quoted identifiers are enable
#define SQL_QI_ON		1L			//	Quoted identifiers are disabled
#define SQL_QI_DEFAULT	SQL_QI_ON

//	Defines for use with SQL_COPT_SS_ANSI_NPW - Pre-Connect Option only
#define SQL_AD_OFF		0L			//	ANSI NULLs, Padding and Warnings are enabled
#define SQL_AD_ON		1L			//	ANSI NULLs, Padding and Warnings are disabled
#define SQL_AD_DEFAULT	SQL_AD_ON

//	Defines for use with SQL_COPT_SS_CONCAT_NULL - Pre-Connect Option only
#define SQL_CN_OFF	  0L		  //  CONCAT_NULL_YIELDS_NULL is off
#define SQL_CN_ON	  1L		  //  CONCAT_NULL_YIELDS_NULL is on
#define SQL_CN_DEFAULT	SQL_CN_ON


//	Defines for use with SQL_SOPT_SS_TEXTPTR_LOGGING
#define SQL_TL_OFF		0L			//	No logging on text pointer ops
#define SQL_TL_ON		1L			//	Logging occurs on text pointer ops
#define SQL_TL_DEFAULT	SQL_TL_ON

//	Defines for use with SQL_SOPT_SS_HIDDEN_COLUMNS
#define SQL_HC_OFF		0L		  //  FOR BROWSE columns are hidden
#define SQL_HC_ON		1L		  //  FOR BROWSE columns are exposed
#define SQL_HC_DEFAULT	SQL_HC_OFF

//	Defines for use with SQL_SOPT_SS_NOBROWSETABLE
#define SQL_NB_OFF		0L			//	NO_BROWSETABLE is off
#define SQL_NB_ON		1L			//	NO_BROWSETABLE is on
#define SQL_NB_DEFAULT	SQL_NB_OFF

//	Defines for use with SQL_SOPT_SS_REGIONALIZE
#define SQL_RE_OFF		0L			//	No regionalization occurs on output character conversions
#define SQL_RE_ON		1L			//	Regionalization occurs on output character conversions
#define SQL_RE_DEFAULT	SQL_RE_OFF

//	Defines for use with SQL_SOPT_SS_CURSOR_OPTIONS
#define SQL_CO_OFF		0L			//	Clear all cursor options
#define SQL_CO_FFO		1L			//	Fast-forward cursor will be used
#define SQL_CO_AF		2L			//	Autofetch on cursor open
#define SQL_CO_FFO_AF	(SQL_CO_FFO|SQL_CO_AF)	//	Fast-forward cursor with autofetch
#define SQL_CO_FIREHOSE_AF 4L       //  Auto fetch on fire-hose cursors
#define SQL_CO_DEFAULT	SQL_CO_OFF

//SQL_SOPT_SS_NOCOUNT_STATUS
#define SQL_NC_OFF      0L
#define SQL_NC_ON       1L

//SQL_SOPT_SS_DEFER_PREPARE
#define SQL_DP_OFF      0L
#define SQL_DP_ON       1L

//SQL_COPT_SS_ENCRYPT
#define SQL_EN_OFF      0L
#define SQL_EN_ON       1L

//SQL_COPT_SS_BROWSE_CONNECT
#define SQL_MORE_INFO_NO  0L
#define SQL_MORE_INFO_YES 1L

//SQL_COPT_SS_BROWSE_CACHE_DATA
#define SQL_CACHE_DATA_NO   0L
#define SQL_CACHE_DATA_YES  1L

//SQL_COPT_SS_RESET_CONNECTION
#define SQL_RESET_YES  1L

//SQL_COPT_SS_WARN_ON_CP_ERROR
#define SQL_WARN_NO   0L
#define SQL_WARN_YES  1L

//	Defines returned by SQL_ATTR_CURSOR_TYPE/SQL_CURSOR_TYPE
#define SQL_CURSOR_FAST_FORWARD_ONLY	8	//	Only returned by SQLGetStmtAttr/Option


//	SQLColAttributes driver specific defines.
//	SQLSet/GetDescField driver specific defines.
//	Microsoft has 1200 thru 1249 reserved for Microsoft SQL Server driver usage.

#define SQL_CA_SS_BASE				1200
#define SQL_CA_SS_COLUMN_SSTYPE		(SQL_CA_SS_BASE+0)	//	dbcoltype/dbalttype
#define SQL_CA_SS_COLUMN_UTYPE		(SQL_CA_SS_BASE+1)	//	dbcolutype/dbaltutype
#define SQL_CA_SS_NUM_ORDERS		(SQL_CA_SS_BASE+2)	//	dbnumorders
#define SQL_CA_SS_COLUMN_ORDER		(SQL_CA_SS_BASE+3)	//	dbordercol
#define SQL_CA_SS_COLUMN_VARYLEN	(SQL_CA_SS_BASE+4)	//	dbvarylen
#define SQL_CA_SS_NUM_COMPUTES		(SQL_CA_SS_BASE+5)	//	dbnumcompute
#define SQL_CA_SS_COMPUTE_ID		(SQL_CA_SS_BASE+6)	//	dbnextrow status return
#define SQL_CA_SS_COMPUTE_BYLIST	(SQL_CA_SS_BASE+7)	//	dbbylist
#define SQL_CA_SS_COLUMN_ID			(SQL_CA_SS_BASE+8)	//	dbaltcolid
#define SQL_CA_SS_COLUMN_OP			(SQL_CA_SS_BASE+9)	//	dbaltop
#define SQL_CA_SS_COLUMN_SIZE		(SQL_CA_SS_BASE+10)	//	dbcollen
#define SQL_CA_SS_COLUMN_HIDDEN		(SQL_CA_SS_BASE+11) //	Column is hidden (FOR BROWSE)
#define SQL_CA_SS_COLUMN_KEY		(SQL_CA_SS_BASE+12) //	Column is key column (FOR BROWSE)
//#define SQL_DESC_BASE_COLUMN_NAME_OLD	(SQL_CA_SS_BASE+13) //This is defined at another location.
#define SQL_CA_SS_COLUMN_COLLATION	(SQL_CA_SS_BASE+14) //	Column collation (only for chars)
#define SQL_CA_SS_VARIANT_TYPE      (SQL_CA_SS_BASE+15)
#define SQL_CA_SS_VARIANT_SQL_TYPE  (SQL_CA_SS_BASE+16)
#define SQL_CA_SS_VARIANT_SERVER_TYPE (SQL_CA_SS_BASE+17)
#define SQL_CA_SS_MAX_USED			(SQL_CA_SS_BASE+18)




//	SQL Server Data Type Tokens.
//	New types for 6.0 and later servers
/* SQL Server Data Type Tokens. */
#define SQLTEXT 			0x23
#define SQLVARBINARY		0x25
#define SQLINTN 			0x26
#define SQLVARCHAR			0x27
#define SQLBINARY			0x2d
#define SQLIMAGE			0x22
#define SQLCHARACTER		0x2f
#define SQLINT1 			0x30
#define SQLBIT				0x32
#define SQLINT2 			0x34
#define SQLINT4 			0x38
#define SQLMONEY			0x3c
#define SQLDATETIME 		0x3d
#define SQLFLT8 			0x3e
#define SQLFLTN 			0x6d
#define SQLMONEYN			0x6e
#define SQLDATETIMN 		0x6f
#define SQLFLT4 			0x3b
#define SQLMONEY4			0x7a
#define SQLDATETIM4 		0x3a
//	New types for 6.0 and later servers
#define SQLDECIMAL			0x6a
#define SQLNUMERIC			0x6c
//	New types for 7.0 and later servers
#define SQLUNIQUEID			0x24
#define SQLBIGCHAR			0xaf
#define SQLBIGVARCHAR		0xa7
#define SQLBIGBINARY		0xad
#define SQLBIGVARBINARY		0xa5
#define SQLBITN				0x68
#define SQLNCHAR			0xef
#define SQLNVARCHAR 		0xe7
#define SQLNTEXT			0x63
// New for 7.x
#define SQLINT8	            0x7f
#define SQLVARIANT          0x62

//	User Data Type definitions.
//	Returned by SQLColAttributes/SQL_CA_SS_COLUMN_UTYPE.
#define SQLudtBINARY			3
#define SQLudtBIT				16
#define SQLudtBITN				0
#define SQLudtCHAR				1
#define SQLudtDATETIM4			22
#define SQLudtDATETIME			12
#define SQLudtDATETIMN			15
#define SQLudtDECML 			24
#define SQLudtDECMLN			26
#define SQLudtFLT4				23
#define SQLudtFLT8				8
#define SQLudtFLTN				14
#define SQLudtIMAGE 			20
#define SQLudtINT1				5
#define SQLudtINT2				6
#define SQLudtINT4				7
#define SQLudtINTN				13
#define SQLudtMONEY 			11
#define SQLudtMONEY4			21
#define SQLudtMONEYN			17
#define SQLudtNUM				10
#define SQLudtNUMN				25
#define SQLudtSYSNAME			18
#define SQLudtTEXT				19
#define SQLudtTIMESTAMP 		80
#define SQLudtUNIQUEIDENTIFIER	0
#define SQLudtVARBINARY 		4
#define SQLudtVARCHAR			2
#define MIN_USER_DATATYPE		256

//	Aggregate operator types.
//	Returned by SQLColAttributes/SQL_CA_SS_COLUMN_OP.
#define SQLAOPSTDEV 	0x30	// Standard deviation
#define SQLAOPSTDEVP	0x31	// Standard deviation population
#define SQLAOPVAR		0x32	// Variance
#define SQLAOPVARP		0x33	// Variance population
#define SQLAOPCNT		0x4b	// Count
#define SQLAOPSUM		0x4d	// Sum
#define SQLAOPAVG		0x4f	// Average
#define SQLAOPMIN		0x51	// Min
#define SQLAOPMAX		0x52	// Max
#define SQLAOPANY		0x53	// Any
#define SQLAOPNOOP		0x56	// None


//	SQLGetInfo driver specific defines.
//	Microsoft has 1151 thru 1200 reserved for Microsoft SQL Server driver usage.

#define SQL_INFO_SS_FIRST		1199
#define SQL_INFO_SS_NETLIB_NAMEW (SQL_INFO_SS_FIRST+0) //  dbprocinfo
#define SQL_INFO_SS_NETLIB_NAMEA (SQL_INFO_SS_FIRST+1) //  dbprocinfo
#define SQL_INFO_SS_MAX_USED	SQL_INFO_SS_NETLIB_NAMEA
#ifdef UNICODE
#define SQL_INFO_SS_NETLIB_NAME		SQL_INFO_SS_NETLIB_NAMEW
#else
#define SQL_INFO_SS_NETLIB_NAME		SQL_INFO_SS_NETLIB_NAMEA
#endif


//	Driver specific SQL type defines.
//	Microsoft has -150 thru -199 reserved for Microsoft SQL Server driver usage.
#define SQL_SS_VARIANT    -150


//	SQLGetDiagField driver specific defines.
//	Microsoft has -1150 thru -1199 reserved for Microsoft SQL Server driver usage.

#define SQL_DIAG_SS_BASE		(-1150)
#define SQL_DIAG_SS_MSGSTATE	(SQL_DIAG_SS_BASE)
#define SQL_DIAG_SS_SEVERITY	(SQL_DIAG_SS_BASE-1)
#define SQL_DIAG_SS_SRVNAME 	(SQL_DIAG_SS_BASE-2)
#define SQL_DIAG_SS_PROCNAME	(SQL_DIAG_SS_BASE-3)
#define SQL_DIAG_SS_LINE		(SQL_DIAG_SS_BASE-4)


//	SQLGetDiagField/SQL_DIAG_DYNAMIC_FUNCTION_CODE driver specific defines.
//	Microsoft has -200 thru -299 reserved for Microsoft SQL Server driver usage.

#define SQL_DIAG_DFC_SS_BASE					(-200)
#define SQL_DIAG_DFC_SS_ALTER_DATABASE			(SQL_DIAG_DFC_SS_BASE-0)
#define SQL_DIAG_DFC_SS_CHECKPOINT				(SQL_DIAG_DFC_SS_BASE-1)
#define SQL_DIAG_DFC_SS_CONDITION				(SQL_DIAG_DFC_SS_BASE-2)
#define SQL_DIAG_DFC_SS_CREATE_DATABASE 		(SQL_DIAG_DFC_SS_BASE-3)
#define SQL_DIAG_DFC_SS_CREATE_DEFAULT			(SQL_DIAG_DFC_SS_BASE-4)
#define SQL_DIAG_DFC_SS_CREATE_PROCEDURE		(SQL_DIAG_DFC_SS_BASE-5)
#define SQL_DIAG_DFC_SS_CREATE_RULE 			(SQL_DIAG_DFC_SS_BASE-6)
#define SQL_DIAG_DFC_SS_CREATE_TRIGGER			(SQL_DIAG_DFC_SS_BASE-7)
#define SQL_DIAG_DFC_SS_CURSOR_DECLARE			(SQL_DIAG_DFC_SS_BASE-8)
#define SQL_DIAG_DFC_SS_CURSOR_OPEN 			(SQL_DIAG_DFC_SS_BASE-9)
#define SQL_DIAG_DFC_SS_CURSOR_FETCH			(SQL_DIAG_DFC_SS_BASE-10)
#define SQL_DIAG_DFC_SS_CURSOR_CLOSE			(SQL_DIAG_DFC_SS_BASE-11)
#define SQL_DIAG_DFC_SS_DEALLOCATE_CURSOR		(SQL_DIAG_DFC_SS_BASE-12)
#define SQL_DIAG_DFC_SS_DBCC					(SQL_DIAG_DFC_SS_BASE-13)
#define SQL_DIAG_DFC_SS_DISK					(SQL_DIAG_DFC_SS_BASE-14)
#define SQL_DIAG_DFC_SS_DROP_DATABASE			(SQL_DIAG_DFC_SS_BASE-15)
#define SQL_DIAG_DFC_SS_DROP_DEFAULT			(SQL_DIAG_DFC_SS_BASE-16)
#define SQL_DIAG_DFC_SS_DROP_PROCEDURE			(SQL_DIAG_DFC_SS_BASE-17)
#define SQL_DIAG_DFC_SS_DROP_RULE				(SQL_DIAG_DFC_SS_BASE-18)
#define SQL_DIAG_DFC_SS_DROP_TRIGGER			(SQL_DIAG_DFC_SS_BASE-19)
#define SQL_DIAG_DFC_SS_DUMP_DATABASE			(SQL_DIAG_DFC_SS_BASE-20)
#define SQL_DIAG_DFC_SS_DUMP_TABLE				(SQL_DIAG_DFC_SS_BASE-21)
#define SQL_DIAG_DFC_SS_DUMP_TRANSACTION		(SQL_DIAG_DFC_SS_BASE-22)
#define SQL_DIAG_DFC_SS_GOTO					(SQL_DIAG_DFC_SS_BASE-23)
#define SQL_DIAG_DFC_SS_INSERT_BULK 			(SQL_DIAG_DFC_SS_BASE-24)
#define SQL_DIAG_DFC_SS_KILL					(SQL_DIAG_DFC_SS_BASE-25)
#define SQL_DIAG_DFC_SS_LOAD_DATABASE			(SQL_DIAG_DFC_SS_BASE-26)
#define SQL_DIAG_DFC_SS_LOAD_HEADERONLY 		(SQL_DIAG_DFC_SS_BASE-27)
#define SQL_DIAG_DFC_SS_LOAD_TABLE				(SQL_DIAG_DFC_SS_BASE-28)
#define SQL_DIAG_DFC_SS_LOAD_TRANSACTION		(SQL_DIAG_DFC_SS_BASE-29)
#define SQL_DIAG_DFC_SS_PRINT					(SQL_DIAG_DFC_SS_BASE-30)
#define SQL_DIAG_DFC_SS_RAISERROR				(SQL_DIAG_DFC_SS_BASE-31)
#define SQL_DIAG_DFC_SS_READTEXT				(SQL_DIAG_DFC_SS_BASE-32)
#define SQL_DIAG_DFC_SS_RECONFIGURE 			(SQL_DIAG_DFC_SS_BASE-33)
#define SQL_DIAG_DFC_SS_RETURN					(SQL_DIAG_DFC_SS_BASE-34)
#define SQL_DIAG_DFC_SS_SELECT_INTO 			(SQL_DIAG_DFC_SS_BASE-35)
#define SQL_DIAG_DFC_SS_SET 					(SQL_DIAG_DFC_SS_BASE-36)
#define SQL_DIAG_DFC_SS_SET_IDENTITY_INSERT 	(SQL_DIAG_DFC_SS_BASE-37)
#define SQL_DIAG_DFC_SS_SET_ROW_COUNT			(SQL_DIAG_DFC_SS_BASE-38)
#define SQL_DIAG_DFC_SS_SET_STATISTICS			(SQL_DIAG_DFC_SS_BASE-39)
#define SQL_DIAG_DFC_SS_SET_TEXTSIZE			(SQL_DIAG_DFC_SS_BASE-40)
#define SQL_DIAG_DFC_SS_SETUSER 				(SQL_DIAG_DFC_SS_BASE-41)
#define SQL_DIAG_DFC_SS_SHUTDOWN				(SQL_DIAG_DFC_SS_BASE-42)
#define SQL_DIAG_DFC_SS_TRANS_BEGIN 			(SQL_DIAG_DFC_SS_BASE-43)
#define SQL_DIAG_DFC_SS_TRANS_COMMIT			(SQL_DIAG_DFC_SS_BASE-44)
#define SQL_DIAG_DFC_SS_TRANS_PREPARE			(SQL_DIAG_DFC_SS_BASE-45)
#define SQL_DIAG_DFC_SS_TRANS_ROLLBACK			(SQL_DIAG_DFC_SS_BASE-46)
#define SQL_DIAG_DFC_SS_TRANS_SAVE				(SQL_DIAG_DFC_SS_BASE-47)
#define SQL_DIAG_DFC_SS_TRUNCATE_TABLE			(SQL_DIAG_DFC_SS_BASE-48)
#define SQL_DIAG_DFC_SS_UPDATE_STATISTICS		(SQL_DIAG_DFC_SS_BASE-49)
#define SQL_DIAG_DFC_SS_UPDATETEXT				(SQL_DIAG_DFC_SS_BASE-50)
#define SQL_DIAG_DFC_SS_USE 					(SQL_DIAG_DFC_SS_BASE-51)
#define SQL_DIAG_DFC_SS_WAITFOR 				(SQL_DIAG_DFC_SS_BASE-52)
#define SQL_DIAG_DFC_SS_WRITETEXT				(SQL_DIAG_DFC_SS_BASE-53)
#define SQL_DIAG_DFC_SS_DENY					(SQL_DIAG_DFC_SS_BASE-54)
#define SQL_DIAG_DFC_SS_SET_XCTLVL				(SQL_DIAG_DFC_SS_BASE-55)

//	Severity codes for SQL_DIAG_SS_SEVERITY
#define	EX_ANY			0
#define	EX_INFO			10
#define EX_MAXISEVERITY EX_INFO
#define	EX_MISSING		11
#define	EX_TYPE			12
#define	EX_DEADLOCK		13
#define	EX_PERMIT		14
#define	EX_SYNTAX		15
#define	EX_USER			16
#define	EX_RESOURCE		17
#define	EX_INTOK		18
#define	MAXUSEVERITY	EX_INTOK
#define	EX_LIMIT		19
#define	EX_CMDFATAL		20
#define	MINFATALERR		EX_CMDFATAL
#define	EX_DBFATAL		21
#define	EX_TABCORRUPT	22
#define	EX_DBCORRUPT	23
#define	EX_HARDWARE		24
#define	EX_CONTROL		25

//	Internal server datatypes - used when binding to SQL_C_BINARY
#ifndef MAXNUMERICLEN	// Resolve ODS/DBLib conflicts
// DB-Library datatypes
#define DBMAXCHAR		(8000+1)	// Max length of DBVARBINARY and DBVARCHAR, etc. +1 for zero byte
#define MAXNAME 		(SQL_MAX_SQLSERVERNAME+1)	// Max server identifier length including zero byte

#ifdef UNICODE
typedef wchar_t			DBCHAR;
#else
typedef char            DBCHAR;
#endif
typedef unsigned char   DBBINARY;
typedef unsigned char   DBTINYINT;
typedef short           DBSMALLINT;
typedef unsigned short  DBUSMALLINT;
typedef double          DBFLT8;
typedef unsigned char   DBBIT;
typedef unsigned char   DBBOOL;
typedef float           DBFLT4;

typedef DBFLT4 DBREAL;
typedef UINT   DBUBOOL;

typedef struct dbvarychar
{
	DBSMALLINT  len;
	DBCHAR      str[DBMAXCHAR];
} DBVARYCHAR;

typedef struct dbvarybin
{
	DBSMALLINT  len;
	BYTE        array[DBMAXCHAR];
} DBVARYBIN;

typedef struct dbmoney
{						// Internal representation of MONEY data type
	LONG  mnyhigh;		// Money value *10,000 (High 32 bits/signed)
	ULONG mnylow;		// Money value *10,000 (Low 32 bits/unsigned)
} DBMONEY;

typedef struct dbdatetime
{						// Internal representation of DATETIME data type
	LONG  dtdays;		// No of days since Jan-1-1900 (maybe negative)
	ULONG dttime;		// No. of 300 hundredths of a second since midnight
} DBDATETIME;

typedef struct dbdatetime4
{						// Internal representation of SMALLDATETIME data type
	USHORT numdays; 	// No of days since Jan-1-1900
	USHORT nummins; 	// No. of minutes since midnight
} DBDATETIM4;

typedef LONG DBMONEY4;	// Internal representation of SMALLMONEY data type
						// Money value *10,000

#define DBNUM_PREC_TYPE BYTE
#define DBNUM_SCALE_TYPE BYTE
#define DBNUM_VAL_TYPE BYTE

#if (ODBCVER < 0x0300)
#define MAXNUMERICLEN 16

typedef struct dbnumeric
{							// Internal representation of NUMERIC data type
	DBNUM_PREC_TYPE   precision;			// Precision
	DBNUM_SCALE_TYPE  scale;				// Scale
	BYTE			  sign; 				// Sign (1 if positive, 0 if negative)
	DBNUM_VAL_TYPE	  val[MAXNUMERICLEN];	// Value
} DBNUMERIC;
typedef DBNUMERIC DBDECIMAL;// Internal representation of DECIMAL data type
#else	//	Use ODBC 3.0 definitions since same as DBLib
#define MAXNUMERICLEN SQL_MAX_NUMERIC_LEN
typedef SQL_NUMERIC_STRUCT DBNUMERIC;
typedef SQL_NUMERIC_STRUCT DBDECIMAL;
#endif

#endif //	MAXNUMERICLEN

#ifndef INT
typedef int INT;
typedef long            DBINT;
#ifndef _LPCBYTE_DEFINED
#define _LPCBYTE_DEFINED
typedef const LPBYTE	LPCBYTE;
#endif
typedef DBINT *			LPDBINT;
#endif

/*****************************************************************
 This struct is a global used for
 gathering statistical data on the driver.
 Access to this structure is controlled via the
 pStatCrit;
******************************************************************/

typedef struct sqlperf
{
	// Application Profile Statistics
	DWORD TimerResolution;
	DWORD SQLidu;
	DWORD SQLiduRows;
	DWORD SQLSelects;
	DWORD SQLSelectRows;
	DWORD Transactions;
	DWORD SQLPrepares;
	DWORD ExecDirects;
	DWORD SQLExecutes;
	DWORD CursorOpens;
	DWORD CursorSize;
	DWORD CursorUsed;
	LDOUBLE PercentCursorUsed;
	LDOUBLE AvgFetchTime;
	LDOUBLE AvgCursorSize;
	LDOUBLE AvgCursorUsed;
	DWORD SQLFetchTime;
	DWORD SQLFetchCount;
	DWORD CurrentStmtCount;
	DWORD MaxOpenStmt;
	DWORD SumOpenStmt;

	// Connection Statistics
	DWORD CurrentConnectionCount;
	DWORD MaxConnectionsOpened;
	DWORD SumConnectionsOpened;
	DWORD SumConnectiontime;
	LDOUBLE AvgTimeOpened;

	// Network Statistics
	DWORD ServerRndTrips;
	DWORD BuffersSent;
	DWORD BuffersRec;
	DWORD BytesSent;
	DWORD BytesRec;

	// Time Statistics;
	DWORD msExecutionTime;
	DWORD msNetWorkServerTime;

} 	SQLPERF;

// The following are options for SQL_COPT_SS_PERF_DATA and SQL_COPT_SS_PERF_QUERY
#define SQL_PERF_START	1			// Starts the driver sampling performance data.
#define SQL_PERF_STOP	2			// Stops the counters from sampling performance data.

// The following are defines for SQL_COPT_SS_PERF_DATA_LOG
#define SQL_SS_DL_DEFAULT	TEXT("STATS.LOG")

// The following are defines for SQL_COPT_SS_PERF_QUERY_LOG
#define SQL_SS_QL_DEFAULT	TEXT("QUERY.LOG")

// The following are defines for SQL_COPT_SS_PERF_QUERY_INTERVAL
#define SQL_SS_QI_DEFAULT	30000	//	30,000 milliseconds

//	ODBC BCP prototypes and defines

//	Return codes
#define SUCCEED 		1
#define FAIL			0
#define SUCCEED_ABORT	2
#define SUCCEED_ASYNC	3

//	Transfer directions
#define DB_IN			1	// Transfer from client to server
#define DB_OUT			2	// Transfer from server to client

//	bcp_control option
#define BCPMAXERRS		1	// Sets max errors allowed
#define BCPFIRST		2	// Sets first row to be copied out
#define BCPLAST 		3	// Sets number of rows to be copied out
#define BCPBATCH		4	// Sets input batch size
#define BCPKEEPNULLS	5	// Sets to insert NULLs for empty input values
#define BCPABORT		6	// Sets to have bcpexec return SUCCEED_ABORT
#define BCPODBC 		7	// Sets ODBC canonical character output
#define BCPKEEPIDENTITY	8	// Sets IDENTITY_INSERT on
#define BCP6xFILEFMT	9	// DEPRECATED: Sets 6x file format on
#define BCPHINTSA		10	// Sets server BCP hints (ANSI string)
#define BCPHINTSW		11	// Sets server BCP hints (UNICODE string)
#define BCPFILECP		12	// Sets clients code page for the file
#define BCPUNICODEFILE	13	// Sets that the file contains unicode header
#define BCPTEXTFILE		14	// Sets BCP mode to expect a text file and to detect Unicode or ANSI automatically
#define BCPFILEFMT		15	// Sets file format version

//	BCPFILECP values
//	Any valid code page that is installed on the client can be passed plus:
#define BCPFILECP_ACP	0	// Data in file is in Windows code page
#define BCPFILECP_OEMCP	1	// Data in file is in OEM code page (default)
#define BCPFILECP_RAW	(-1)// Data in file is in Server code page (no conversion)

//	bcp_collen definition
#define SQL_VARLEN_DATA (-10)	//	Use default length for column

// BCP functions
DBINT	SQL_API bcp_batch (HDBC);
RETCODE SQL_API bcp_bind (HDBC, LPCBYTE, INT, DBINT, LPCBYTE, INT, INT, INT);
RETCODE SQL_API bcp_colfmt (HDBC, INT, BYTE, INT, DBINT, LPCBYTE, INT, INT);
RETCODE SQL_API bcp_collen (HDBC, DBINT, INT);
RETCODE SQL_API bcp_colptr (HDBC, LPCBYTE, INT);
RETCODE SQL_API bcp_columns (HDBC, INT);
RETCODE SQL_API bcp_control (HDBC, INT, void *);
DBINT	SQL_API bcp_done (HDBC);
RETCODE SQL_API bcp_exec (HDBC, LPDBINT);
RETCODE SQL_API bcp_getcolfmt (HDBC, INT, INT, void *, INT, INT *);
RETCODE SQL_API bcp_initA (HDBC, LPCSTR, LPCSTR, LPCSTR, INT);
RETCODE SQL_API bcp_initW (HDBC, LPCWSTR, LPCWSTR, LPCWSTR, INT);
RETCODE SQL_API bcp_moretext (HDBC, DBINT, LPCBYTE);
RETCODE SQL_API bcp_readfmtA (HDBC, LPCSTR);
RETCODE SQL_API bcp_readfmtW (HDBC, LPCWSTR);
RETCODE SQL_API bcp_sendrow (HDBC);
RETCODE SQL_API bcp_setcolfmt (HDBC, INT, INT, void *, INT);
RETCODE SQL_API bcp_writefmtA (HDBC, LPCSTR);
RETCODE SQL_API bcp_writefmtW (HDBC, LPCWSTR);
CHAR *	SQL_API dbprtypeA (INT);
WCHAR * SQL_API dbprtypeW (INT);

#ifdef UNICODE
#define bcp_init		bcp_initW
#define bcp_readfmt		bcp_readfmtW
#define bcp_writefmt	bcp_writefmtW
#define dbprtype		dbprtypeW
#define BCPHINTS		BCPHINTSW

#else
#define bcp_init		bcp_initA
#define bcp_readfmt		bcp_readfmtA
#define bcp_writefmt	bcp_writefmtA
#define dbprtype		dbprtypeA
#define BCPHINTS		BCPHINTSA
#endif

//	SQL Server catalog extensions for distributed queries
SQLRETURN SQL_API SQLLinkedServers (SQLHSTMT);
SQLRETURN SQL_API SQLLinkedCatalogsA (SQLHSTMT, LPCSTR, SWORD);
SQLRETURN SQL_API SQLLinkedCatalogsW (SQLHSTMT, LPCWSTR, SWORD);

//	SQL Server extensions for server enumeration
HANDLE   SQL_API SQLInitEnumServers(_In_ LPWSTR pwchServerName, _In_ LPWSTR pwchInstanceName);
RETCODE  SQL_API SQLGetNextEnumeration (HANDLE hEnumHandle,BYTE * prgEnumData,INT * piEnumLength);
RETCODE  SQL_API SQLCloseEnumServers (HANDLE hEnumHandle);

#ifdef UNICODE
#define SQLLinkedCatalogs	SQLLinkedCatalogsW
#else
#define SQLLinkedCatalogs	SQLLinkedCatalogsA
#endif

//  BCP column format properties
#define BCP_FMT_TYPE			0x01
#define BCP_FMT_INDICATOR_LEN	0x02
#define BCP_FMT_DATA_LEN		0x03
#define	BCP_FMT_TERMINATOR		0x04
#define BCP_FMT_SERVER_COL		0x05
#define BCP_FMT_COLLATION		0x06
#define BCP_FMT_COLLATION_ID	0x07

//	The following options have been deprecated

#define SQL_FAST_CONNECT				(SQL_COPT_SS_BASE+0)
//	Defines for use with SQL_FAST_CONNECT - only useable before connecting
#define SQL_FC_OFF		0L			//	Fast connect is off
#define SQL_FC_ON		1L			//	Fast connect is on
#define SQL_FC_DEFAULT	SQL_FC_OFF
#define SQL_COPT_SS_ANSI_OEM			(SQL_COPT_SS_BASE+6)
#define SQL_AO_OFF						0L
#define SQL_AO_ON						1L
#define SQL_AO_DEFAULT					SQL_AO_OFF

//	Define old names
#define SQL_REMOTE_PWD					SQL_COPT_SS_REMOTE_PWD
#define SQL_USE_PROCEDURE_FOR_PREPARE	SQL_COPT_SS_USE_PROC_FOR_PREP
#define SQL_INTEGRATED_SECURITY 		SQL_COPT_SS_INTEGRATED_SECURITY
#define SQL_PRESERVE_CURSORS			SQL_COPT_SS_PRESERVE_CURSORS
#define SQL_TEXTPTR_LOGGING 			SQL_SOPT_SS_TEXTPTR_LOGGING
#define SQL_CA_SS_BASE_COLUMN_NAME		SQL_DESC_BASE_COLUMN_NAME
#define SQLDECIMALN						0x6a
#define SQLNUMERICN 					0x6c

#ifdef __cplusplus
}									 /* End of extern "C" { */
#endif  /* __cplusplus */

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif

//	End of odbcss.h
