/********************************************************
*                                                       *
*   Copyright (C) Microsoft. All rights reserved.       *
*                                                       *
********************************************************/

//-----------------------------------------------------------------------------
// File:			sqltypes.h
//
// Contents: 		This file defines the types used in ODBC
//
// Comments:
//
//-----------------------------------------------------------------------------

#if defined(_MSC_VER) && (_MSC_VER > 1000)
#pragma once
#endif

#ifndef __SQLTYPES
#define __SQLTYPES

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


/* if ODBCVER is not defined, default to ODBC 3.80 */
#ifndef ODBCVER
#define ODBCVER	0x0380
#endif  /* ODBCVER */

#ifdef __cplusplus
extern "C" { 			/* Assume C declarations for C++   */
#endif  /* __cplusplus */

/* environment specific definitions */
#ifndef EXPORT
#define EXPORT
#endif

#ifdef WIN32
#define SQL_API  __stdcall
#else
#define SQL_API
#endif

#ifndef RC_INVOKED

/* API declaration data types */
typedef unsigned char   SQLCHAR;
#if (ODBCVER >= 0x0300)
typedef signed char     SQLSCHAR;
typedef unsigned char   SQLDATE;
typedef unsigned char   SQLDECIMAL;
typedef double          SQLDOUBLE;
typedef double          SQLFLOAT;
#endif
typedef long            SQLINTEGER;
typedef unsigned long   SQLUINTEGER;

#ifdef _WIN64
typedef INT64           SQLLEN;
typedef UINT64          SQLULEN;
typedef UINT64          SQLSETPOSIROW;
#else
#define SQLLEN          SQLINTEGER
#define SQLULEN         SQLUINTEGER
#define SQLSETPOSIROW   SQLUSMALLINT
#endif

//For Backward compatibility
#ifdef WIN32
typedef SQLULEN			SQLROWCOUNT;
typedef SQLULEN			SQLROWSETSIZE;
typedef SQLULEN			SQLTRANSID;
typedef SQLLEN			SQLROWOFFSET;
#endif

#if (ODBCVER >= 0x0300)
typedef unsigned char   SQLNUMERIC;
#endif
typedef void *          SQLPOINTER;
#if (ODBCVER >= 0x0300)
typedef float           SQLREAL;
#endif
typedef short           SQLSMALLINT;
typedef unsigned short  SQLUSMALLINT;
#if (ODBCVER >= 0x0300)
typedef unsigned char   SQLTIME;
typedef unsigned char   SQLTIMESTAMP;
typedef unsigned char   SQLVARCHAR;
#endif

/* function return type */
typedef SQLSMALLINT     SQLRETURN;

/* generic data structures */
#if (ODBCVER >= 0x0300)
#if defined(WIN32) || defined(_WIN64)
typedef void*					SQLHANDLE;
#else
typedef SQLINTEGER              SQLHANDLE;
#endif	/* defined(WIN32) || defined(_WIN64) */
typedef SQLHANDLE               SQLHENV;
typedef SQLHANDLE               SQLHDBC;
typedef SQLHANDLE               SQLHSTMT;
typedef SQLHANDLE               SQLHDESC;
#else //ODBCVER < 0x0300
#if defined(WIN32) || defined(_WIN64)
typedef void*					SQLHENV;
typedef void*					SQLHDBC;
typedef void*					SQLHSTMT;
#else
typedef SQLINTEGER              SQLHENV;
typedef SQLINTEGER              SQLHDBC;
typedef SQLINTEGER              SQLHSTMT;
#endif  /* defined(WIN32) || defined(_WIN64) */
#endif /* ODBCVER >= 0x0300 */

/* SQL portable types for C */
typedef unsigned char           UCHAR;
typedef signed char             SCHAR;
typedef SCHAR                   SQLSCHAR;
typedef long int                SDWORD;
typedef short int               SWORD;
typedef unsigned long int       UDWORD;
typedef unsigned short int      UWORD;
#ifndef _WIN64
typedef UDWORD                  SQLUINTEGER;
#endif

typedef signed long             SLONG;
typedef signed short            SSHORT;
typedef unsigned long           ULONG;
typedef unsigned short          USHORT;
typedef double                  SDOUBLE;
typedef double            		LDOUBLE;
typedef float                   SFLOAT;

typedef void*              		PTR;

typedef void*              		HENV;
typedef void*              		HDBC;
typedef void*              		HSTMT;

typedef signed short            RETCODE;

#if defined(WIN32) || defined(OS2)
typedef HWND                    SQLHWND;
#elif defined (UNIX)
typedef Widget                  SQLHWND;
#else
/* placehold for future O/S GUI window handle definition */
typedef SQLPOINTER              SQLHWND;
#endif

#ifndef	__SQLDATE
#define	__SQLDATE
/* transfer types for DATE, TIME, TIMESTAMP */
typedef struct tagDATE_STRUCT
{
        SQLSMALLINT    year;
        SQLUSMALLINT   month;
        SQLUSMALLINT   day;
} DATE_STRUCT;

#if (ODBCVER >= 0x0300)
typedef DATE_STRUCT	SQL_DATE_STRUCT;
#endif  /* ODBCVER >= 0x0300 */

typedef struct tagTIME_STRUCT
{
        SQLUSMALLINT   hour;
        SQLUSMALLINT   minute;
        SQLUSMALLINT   second;
} TIME_STRUCT;

#if (ODBCVER >= 0x0300)
typedef TIME_STRUCT	SQL_TIME_STRUCT;
#endif /* ODBCVER >= 0x0300 */

typedef struct tagTIMESTAMP_STRUCT
{
        SQLSMALLINT    year;
        SQLUSMALLINT   month;
        SQLUSMALLINT   day;
        SQLUSMALLINT   hour;
        SQLUSMALLINT   minute;
        SQLUSMALLINT   second;
        SQLUINTEGER    fraction;
} TIMESTAMP_STRUCT;

#if (ODBCVER >= 0x0300)
typedef TIMESTAMP_STRUCT	SQL_TIMESTAMP_STRUCT;
#endif  /* ODBCVER >= 0x0300 */


/*
 * enumerations for DATETIME_INTERVAL_SUBCODE values for interval data types
 * these values are from SQL-92
 */

#if (ODBCVER >= 0x0300)
typedef enum
{
	SQL_IS_YEAR						= 1,
	SQL_IS_MONTH					= 2,
	SQL_IS_DAY						= 3,
	SQL_IS_HOUR						= 4,
	SQL_IS_MINUTE					= 5,
	SQL_IS_SECOND					= 6,
	SQL_IS_YEAR_TO_MONTH			= 7,
	SQL_IS_DAY_TO_HOUR				= 8,
	SQL_IS_DAY_TO_MINUTE			= 9,
	SQL_IS_DAY_TO_SECOND			= 10,
	SQL_IS_HOUR_TO_MINUTE			= 11,
	SQL_IS_HOUR_TO_SECOND			= 12,
	SQL_IS_MINUTE_TO_SECOND			= 13
} SQLINTERVAL;

#endif  /* ODBCVER >= 0x0300 */

#if (ODBCVER >= 0x0300)
typedef struct tagSQL_YEAR_MONTH
{
		SQLUINTEGER		year;
		SQLUINTEGER		month;
} SQL_YEAR_MONTH_STRUCT;

typedef struct tagSQL_DAY_SECOND
{
		SQLUINTEGER		day;
		SQLUINTEGER		hour;
		SQLUINTEGER		minute;
		SQLUINTEGER		second;
		SQLUINTEGER		fraction;
} SQL_DAY_SECOND_STRUCT;

typedef struct tagSQL_INTERVAL_STRUCT
{
	SQLINTERVAL		interval_type;
	SQLSMALLINT		interval_sign;
	union {
		SQL_YEAR_MONTH_STRUCT		year_month;
		SQL_DAY_SECOND_STRUCT		day_second;
	} intval;

} SQL_INTERVAL_STRUCT;

#endif  /* ODBCVER >= 0x0300 */

#endif	/* __SQLDATE	*/

/* the ODBC C types for SQL_C_SBIGINT and SQL_C_UBIGINT */
#if (ODBCVER >= 0x0300)
#if (_MSC_VER >= 900)
#define ODBCINT64	__int64
#endif

/* If using other compilers, define ODBCINT64 to the
	approriate 64 bit integer type */
#ifdef ODBCINT64
typedef ODBCINT64	SQLBIGINT;
typedef unsigned ODBCINT64	SQLUBIGINT;
#endif
#endif  /* ODBCVER >= 0x0300 */

/* internal representation of numeric data type */
#if (ODBCVER >= 0x0300)
#define SQL_MAX_NUMERIC_LEN		16
typedef struct tagSQL_NUMERIC_STRUCT
{
	SQLCHAR		precision;
	SQLSCHAR	scale;
	SQLCHAR		sign;	/* 1 if positive, 0 if negative */
	SQLCHAR		val[SQL_MAX_NUMERIC_LEN];
} SQL_NUMERIC_STRUCT;
#endif  /* ODBCVER >= 0x0300 */

#if (ODBCVER >= 0x0350)
#ifdef GUID_DEFINED
typedef GUID	SQLGUID;
#else
/* size is 16 */
typedef struct  tagSQLGUID
{
    DWORD Data1;
    WORD Data2;
    WORD Data3;
    BYTE Data4[ 8 ];
} SQLGUID;
#endif  /* GUID_DEFINED */
#endif  /* ODBCVER >= 0x0350 */

typedef SQLULEN         BOOKMARK;

#ifdef _WCHAR_T_DEFINED
typedef wchar_t SQLWCHAR;
#else
typedef unsigned short SQLWCHAR;
#endif

#ifdef UNICODE
typedef SQLWCHAR        SQLTCHAR;
#else
typedef SQLCHAR         SQLTCHAR;
#endif  /* UNICODE */


#endif     /* RC_INVOKED */


#ifdef __cplusplus
}                                    /* End of extern "C" { */
#endif  /* __cplusplus */

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif /* #ifndef __SQLTYPES */
