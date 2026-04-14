/********************************************************
*                                                       *
*   Copyright (C) Microsoft. All rights reserved.       *
*                                                       *
********************************************************/

//-----------------------------------------------------------------------------
// File:            sqlspi.h
//
// Contents:        This is the header for driver writers to support new ODBC 
//                  features. Application writers should not include 
//                  this header file
//                  Please include <sql.h> and <sqlext.h> before including this 
//                  file
//
//-----------------------------------------------------------------------------

#if defined(_MSC_VER) && (_MSC_VER > 1000)
#pragma once
#endif

#ifndef __SQLSPI__
#define __SQLSPI__

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#ifdef __cplusplus
extern "C" {           // Assume C declarations for C++
#endif                 // End of __cplusplus 

// SQL_SPI is just a marker for "Service Provider Interface", otherwise it is the same as API
// Application should not call functions that are marked as SQL_SPI directly
#define SQL_SPI  SQL_API

//-------------------- ODBC Connection Info Handle -----------------------------
// handle for storing connection information for ODBC driver connection pooling
#define SQL_HANDLE_DBC_INFO_TOKEN               6       // Handle type, used in SQLAllocHandle
typedef SQLHANDLE SQLHDBC_INFO_TOKEN;

//-------------------- ODBC Pool ID for driver-aware pooling -----------------------------
typedef SQLULEN     POOLID;
typedef DWORD_PTR   TRANSID;

//-------------------- Driver-aware Connection Pooling --------------------------
// We define a few scores with special meaning
// But driver can return any score between 0 and 100
typedef DWORD SQLConnPoolRating;
#define SQL_CONN_POOL_RATING_BEST               100     // the best of the rating
#define SQL_CONN_POOL_RATING_GOOD_ENOUGH        99      // the rating is good enough and we can stop rating
#define SQL_CONN_POOL_RATING_USELESS            0       // the candidate connection must not be reused for the current request

// SQLSetConnectAttr
#define SQL_ATTR_DBC_INFO_TOKEN                 118     // reset the pooled connection in case it is not a perfect match

// Set connection attributes into DBC info token
SQLRETURN SQL_SPI SQLSetConnectAttrForDbcInfoW(
    SQLHDBC_INFO_TOKEN  hDbcInfoToken,
    SQLINTEGER          Attribute,
    _In_reads_bytes_opt_(StringLength) 
    SQLPOINTER          Value,
    SQLINTEGER          StringLength);

// Set connection information for SQLDriverConnect
SQLRETURN SQL_SPI SQLSetDriverConnectInfoW(
    SQLHDBC_INFO_TOKEN  hDbcInfoToken,
    _In_reads_(cchConnStrIn) 
    SQLWCHAR            *szConnStrIn,
    SQLSMALLINT         cchConnStrIn);

// Set connection information for SQLConnect
SQLRETURN SQL_SPI SQLSetConnectInfoW
(
    SQLHDBC_INFO_TOKEN  hDbcInfoToken,
    _In_reads_(cchDSN) 
    SQLWCHAR            *szDSN,
    SQLSMALLINT         cchDSN,
    _In_reads_(cchUID) 
    SQLWCHAR            *szUID,
    SQLSMALLINT         cchUID,
    _In_reads_(cchAuthStr) 
    SQLWCHAR            *szAuthStr,
    SQLSMALLINT         cchAuthStr
);

// Get the pool ID for the token
SQLRETURN SQL_SPI SQLGetPoolID(
    SQLHDBC_INFO_TOKEN  hDbcInfoToken,
    _Out_
    POOLID*             pPoolID);

// Return how close hCandidateConnection matches with hRequest
// *pRating must be between SQL_CONN_POOL_RATING_USELESS and SQL_CONN_POOL_RATING_BEST (inclusively)
// If return value is not SQL_SUCCESS or *pRating > SQL_CONN_POOL_RATING_BEST, the candidate 
// connection will not be used any more in any other connection request
// If fRequiresTransactionEnlistment is TRUE, transId represents the DTC transaction ID that 
// should be enlisted to (transId == 0 means unenlistment). Otherwise, transId should be ignored
SQLRETURN SQL_SPI SQLRateConnection(
    SQLHDBC_INFO_TOKEN  hRequest,
    SQLHDBC             hCandidateConnection,
    BOOL                fRequiresTransactionEnlistment,
    TRANSID             transId,
    _Out_
    SQLConnPoolRating   *pRating);

// Create a physical connection 
// If application is calling SQLDriverConnect, szConnStrOut is non-NULL at input. 
// Otherwise, it will be set to NULL
SQLRETURN SQL_SPI SQLPoolConnectW(
    SQLHDBC             hdbc,
    SQLHDBC_INFO_TOKEN  hDbcInfoToken,
    _Out_writes_opt_(cchConnStrOutMax)
    SQLWCHAR            *szConnStrOut, 
    SQLSMALLINT         cchConnStrOutMax,
    _Out_opt_
    SQLSMALLINT         *pcchConnStrOut);

// Clean up a pool Id that was timed out
// poolID [in]: the pool ID (under EnvironmentHandle) to be cleaned
SQLRETURN SQL_SPI SQLCleanupConnectionPoolID(
    SQLHENV             EnvironmentHandle,
    POOLID              poolID);

//-----------------------------------------------------------------------------
// functions for ANSI drivers

// Set connection attributes into DBC info token
SQLRETURN SQL_SPI SQLSetConnectAttrForDbcInfoA(
    SQLHDBC_INFO_TOKEN  hDbcInfoToken,
    SQLINTEGER          Attribute,
    _In_reads_bytes_opt_(StringLength) 
    SQLPOINTER          Value,
    SQLINTEGER          StringLength);

// Set connection information for SQLDriverConnect
SQLRETURN SQL_SPI SQLSetDriverConnectInfoA(
    SQLHDBC_INFO_TOKEN  hDbcInfoToken,
    _In_reads_(cchConnStrIn) 
    SQLCHAR             *szConnStrIn,
    SQLSMALLINT         cchConnStrIn);

// Set connection information for SQLConnect
SQLRETURN SQL_SPI SQLSetConnectInfoA
(
    SQLHDBC_INFO_TOKEN  hDbcInfoToken,
    _In_reads_(cchDSN) 
    SQLCHAR             *szDSN,
    SQLSMALLINT         cchDSN,
    _In_reads_(cchUID) 
    SQLCHAR             *szUID,
    SQLSMALLINT         cchUID,
    _In_reads_(cchAuthStr) 
    SQLCHAR             *szAuthStr,
    SQLSMALLINT         cchAuthStr
);

// Create a physical connection 
// If application is calling SQLDriverConnect, szConnStrOut is non-NULL at input. 
// Otherwise, it will be set to NULL
SQLRETURN SQL_SPI SQLPoolConnectA(
    SQLHDBC             hdbc,
    SQLHDBC_INFO_TOKEN  hDbcInfoToken,
    _Out_writes_opt_(cchConnStrOutMax)
    SQLCHAR             *szConnStrOut, 
    SQLSMALLINT         cchConnStrOutMax,
    _Out_opt_
    SQLSMALLINT         *pcchConnStrOut);

//-----------------------------------------------------------------------------
// Unicode mapping
// Define SQL_NOUNICODEMAP to disable the mapping
#if (!defined(SQL_NOUNICODEMAP) && defined(UNICODE))
#define SQLSetConnectAttrForDbcInfo     SQLSetConnectAttrForDbcInfoW
#define SQLSetDriverConnectInfo         SQLSetDriverConnectInfoW
#define SQLSetConnectInfo               SQLSetConnectInfoW
#define SQLPoolConnect                  SQLPoolConnectW
#else
#define SQLSetConnectAttrForDbcInfo     SQLSetConnectAttrForDbcInfoA
#define SQLSetDriverConnectInfo         SQLSetDriverConnectInfoA
#define SQLSetConnectInfo               SQLSetConnectInfoA
#define SQLPoolConnect                  SQLPoolConnectA
#endif      // End of #if (!defined(SQL_NOUNICODEMAP) && defined(UNICODE))
//------------------------------------------------------------------------------

//-------------------- Async Notification --------------------------
#if (ODBCVER >= 0x0380)
#define SQL_ATTR_ASYNC_DBC_NOTIFICATION_CALLBACK  120
#define SQL_ATTR_ASYNC_DBC_NOTIFICATION_CONTEXT   121

#define SQL_ATTR_ASYNC_STMT_NOTIFICATION_CALLBACK 30
#define SQL_ATTR_ASYNC_STMT_NOTIFICATION_CONTEXT  31

typedef SQLRETURN (SQL_API *SQL_ASYNC_NOTIFICATION_CALLBACK)(_In_ SQLPOINTER pContext, BOOL fLast);
#endif /* ODBCVER >= 0x0380 */


#ifdef __cplusplus
}                   // End of extern "C" {
#endif              // End of #ifdef __cplusplus


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif              // End of #ifndef __SQLSPI__
