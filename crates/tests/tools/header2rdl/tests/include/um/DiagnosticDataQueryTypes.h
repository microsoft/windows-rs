

/* this ALWAYS GENERATED file contains the definitions for the interfaces */


 /* File created by MIDL compiler version 8.01.0628 */
/* @@MIDL_FILE_HEADING(  ) */



/* verify that the <rpcndr.h> version is high enough to compile this file*/
#ifndef __REQUIRED_RPCNDR_H_VERSION__
#define __REQUIRED_RPCNDR_H_VERSION__ 501
#endif

/* verify that the <rpcsal.h> version is high enough to compile this file*/
#ifndef __REQUIRED_RPCSAL_H_VERSION__
#define __REQUIRED_RPCSAL_H_VERSION__ 100
#endif

#include "rpc.h"
#include "rpcndr.h"

#ifndef __RPCNDR_H_VERSION__
#error this stub requires an updated version of <rpcndr.h>
#endif /* __RPCNDR_H_VERSION__ */


#ifndef __diagnosticdataquerytypes_h__
#define __diagnosticdataquerytypes_h__

#if defined(_MSC_VER) && (_MSC_VER >= 1020)
#pragma once
#endif

#ifndef DECLSPEC_XFGVIRT
#if defined(_CONTROL_FLOW_GUARD_XFG)
#define DECLSPEC_XFGVIRT(base, func) __declspec(xfg_virtual(base, func))
#else
#define DECLSPEC_XFGVIRT(base, func)
#endif
#endif

/* Forward Declarations */ 

/* header files for imported files */
#include "wtypes.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_diagnosticdataquerytypes_0000_0000 */
/* [local] */ 

//+-------------------------------------------------------------------------
//
// Copyright (c) Microsoft Corporation
//
//--------------------------------------------------------------------------
//
//  IDL for defining Diagnostic API structures necessary for public consumption. This, along with
//  DiagnosticDataQueryTypes.h, are the only headers that should be necessary for consuming the Diagnostic Data Query APIs.
//  New function definition must be added at the end of the file.
//
//--------------------------------------------------------------------------


extern RPC_IF_HANDLE RpcClient___MIDL_itf_diagnosticdataquerytypes_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_diagnosticdataquerytypes_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_diagnosticdataquerytypes_0000_0000_v0_0_s_ifspec;

#ifndef __DiagnosticDataQueryTypes_INTERFACE_DEFINED__
#define __DiagnosticDataQueryTypes_INTERFACE_DEFINED__

/* interface DiagnosticDataQueryTypes */
/* [version][uuid] */ 

typedef 
enum tagDdqAccessLevel
    {
        NoData	= 0,
        CurrentUserData	= 1,
        AllUserData	= 2
    } 	DdqAccessLevel;

typedef struct tagDIAGNOSTIC_DATA_RECORD
    {
    INT64 rowId;
    UINT64 timestamp;
    UINT64 eventKeywords;
    LPWSTR fullEventName;
    LPWSTR providerGroupGuid;
    LPWSTR producerName;
    /* [size_is] */ INT32 *privacyTags;
    UINT32 privacyTagCount;
    /* [size_is] */ INT32 *categoryIds;
    UINT32 categoryIdCount;
    BOOL isCoreData;
    LPWSTR extra1;
    LPWSTR extra2;
    LPWSTR extra3;
    } 	DIAGNOSTIC_DATA_RECORD;

typedef struct tagDIAGNOSTIC_DATA_SEARCH_CRITERIA
    {
    /* [size_is] */ LPCWSTR *producerNames;
    UINT32 producerNameCount;
    LPCWSTR textToMatch;
    /* [size_is] */ const INT32 *categoryIds;
    UINT32 categoryIdCount;
    /* [size_is] */ const INT32 *privacyTags;
    UINT32 privacyTagCount;
    BOOL coreDataOnly;
    } 	DIAGNOSTIC_DATA_SEARCH_CRITERIA;

typedef struct tagDIAGNOSTIC_DATA_EVENT_TAG_DESCRIPTION
    {
    INT32 privacyTag;
    LPWSTR name;
    LPWSTR description;
    } 	DIAGNOSTIC_DATA_EVENT_TAG_DESCRIPTION;

typedef struct tagDIAGNOSTIC_DATA_EVENT_PRODUCER_DESCRIPTION
    {
    LPWSTR name;
    } 	DIAGNOSTIC_DATA_EVENT_PRODUCER_DESCRIPTION;

typedef struct tagDIAGNOSTIC_DATA_EVENT_CATEGORY_DESCRIPTION
    {
    INT32 id;
    LPWSTR name;
    } 	DIAGNOSTIC_DATA_EVENT_CATEGORY_DESCRIPTION;

typedef struct tagDIAGNOSTIC_DATA_EVENT_TAG_STATS
    {
    INT32 privacyTag;
    UINT32 eventCount;
    } 	DIAGNOSTIC_DATA_EVENT_TAG_STATS;

typedef struct tagDIAGNOSTIC_DATA_EVENT_BINARY_STATS
    {
    LPWSTR moduleName;
    LPWSTR friendlyModuleName;
    UINT32 eventCount;
    UINT64 uploadSizeBytes;
    } 	DIAGNOSTIC_DATA_EVENT_BINARY_STATS;

typedef struct tagDIAGNOSTIC_DATA_GENERAL_STATS
    {
    UINT32 optInLevel;
    UINT64 transcriptSizeBytes;
    UINT64 oldestEventTimestamp;
    UINT32 totalEventCountLast24Hours;
    FLOAT averageDailyEvents;
    } 	DIAGNOSTIC_DATA_GENERAL_STATS;

typedef struct tagDIAGNOSTIC_DATA_EVENT_TRANSCRIPT_CONFIGURATION
    {
    UINT32 hoursOfHistoryToKeep;
    UINT32 maxStoreMegabytes;
    UINT32 requestedMaxStoreMegabytes;
    } 	DIAGNOSTIC_DATA_EVENT_TRANSCRIPT_CONFIGURATION;

typedef struct tagDIAGNOSTIC_REPORT_PARAMETER
    {
    WCHAR name[ 129 ];
    WCHAR value[ 260 ];
    } 	DIAGNOSTIC_REPORT_PARAMETER;

typedef struct tagDIAGNOSTIC_REPORT_SIGNATURE
    {
    WCHAR eventName[ 65 ];
    DIAGNOSTIC_REPORT_PARAMETER parameters[ 10 ];
    } 	DIAGNOSTIC_REPORT_SIGNATURE;

typedef struct tagDIAGNOSTIC_REPORT_DATA
    {
    DIAGNOSTIC_REPORT_SIGNATURE signature;
    GUID bucketId;
    GUID reportId;
    FILETIME creationTime;
    ULONGLONG sizeInBytes;
    LPWSTR cabId;
    DWORD reportStatus;
    GUID reportIntegratorId;
    /* [size_is] */ LPWSTR *fileNames;
    DWORD fileCount;
    LPWSTR friendlyEventName;
    LPWSTR applicationName;
    LPWSTR applicationPath;
    LPWSTR description;
    LPWSTR bucketIdString;
    UINT64 legacyBucketId;
    LPWSTR reportKey;
    } 	DIAGNOSTIC_REPORT_DATA;

typedef /* [context_handle] */ void *DIAGNOSTIC_DATA_QUERY_SESSION;

typedef /* [ref] */  __RPC_ref_pointer DIAGNOSTIC_DATA_QUERY_SESSION *PDIAGNOSTIC_DATA_QUERY_SESSION;



extern RPC_IF_HANDLE RpcClient_DiagnosticDataQueryTypes_v1_0_c_ifspec;
extern RPC_IF_HANDLE DiagnosticDataQueryTypes_v1_0_c_ifspec;
extern RPC_IF_HANDLE DiagnosticDataQueryTypes_v1_0_s_ifspec;
#endif /* __DiagnosticDataQueryTypes_INTERFACE_DEFINED__ */

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


