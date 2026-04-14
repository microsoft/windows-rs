/*++

Copyright (c) Microsoft Corporation.  All rights reserved.

Module Name:

    resapi.h

Abstract:

    This module defines the interface exported by Windows Clusters resources.

Revision History:

--*/

#include <winapifamily.h>

#pragma region Desktop Family or FailoverCluster Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_CLUSTER)

#ifndef _RESAPI_DEFINES_
#define _RESAPI_DEFINES_

#if _MSC_VER > 1000
#pragma once
#endif

#include <windows.h>
#include <winsvc.h>
#include <clusapi.h>
#include <stdio.h>

#ifdef __cplusplus
extern "C" {
#endif

#define IN
#define OUT

//
// Definitions for entrypoints exported by a resource DLL.
//
#define STARTUP_ROUTINE "Startup"

#define CLRES_V1_FUNCTION_SIZE   sizeof(CLRES_V1_FUNCTIONS)
#define CLRES_VERSION_V1_00    0x100


#define CLRES_V1_FUNCTION_TABLE( _Name,                     \
                                 _Version,                  \
                                 _Prefix,                   \
                                 _Arbitrate,                \
                                 _Release,                  \
                                 _ResControl,               \
                                 _ResTypeControl            \
                                 )                          \
CLRES_FUNCTION_TABLE _Name = { CLRES_V1_FUNCTION_SIZE,      \
                               _Version,                    \
                               _Prefix##Open,               \
                               _Prefix##Close,              \
                               _Prefix##Online,             \
                               _Prefix##Offline,            \
                               _Prefix##Terminate,          \
                               _Prefix##LooksAlive,         \
                               _Prefix##IsAlive,            \
                               _Arbitrate,                  \
                               _Release,                    \
                               _ResControl,                 \
                               _ResTypeControl }


#if CLUSAPI_VERSION >= CLUSAPI_VERSION_WINDOWS8

#define STARTUP_EX_ROUTINE "StartupEx"

#define CLRES_V2_FUNCTION_SIZE   sizeof(CLRES_V2_FUNCTIONS)
#define CLRES_VERSION_V2_00    0x200
#define CLRES_V2_FUNCTION_TABLE_SET( _Name,                 \
                                     _Version,              \
                                     _Prefix,               \
                                     _Arbitrate,            \
                                     _Release,              \
                                     _ResControl,           \
                                     _ResTypeControl,       \
                                     _LooksAlive,           \
                                     _IsAlive,              \
                                     _Cancel                \
                                     )                      \
_Name.TableSize = CLRES_V2_FUNCTION_SIZE;                   \
_Name.Version = _Version;                                   \
_Name.V2Functions.Open = _Prefix##OpenV2;                   \
_Name.V2Functions.Close = _Prefix##Close;                   \
_Name.V2Functions.Online = _Prefix##OnlineV2;               \
_Name.V2Functions.Offline = _Prefix##OfflineV2;             \
_Name.V2Functions.Terminate = _Prefix##Terminate;           \
_Name.V2Functions.LooksAlive= _LooksAlive;                  \
_Name.V2Functions.IsAlive = _IsAlive;                       \
_Name.V2Functions.Arbitrate = _Arbitrate;                   \
_Name.V2Functions.Release = _Release;                       \
_Name.V2Functions.ResourceControl = _ResControl;            \
_Name.V2Functions.ResourceTypeControl = _ResTypeControl;    \
_Name.V2Functions.Cancel = _Cancel;


#define CLRES_V3_FUNCTION_SIZE    sizeof(CLRES_V3_FUNCTIONS)
#define CLRES_VERSION_V3_00    0x300
#define CLRES_V3_FUNCTION_TABLE_SET( _Name,                 \
                                     _Version,              \
                                     _Prefix,               \
                                     _Arbitrate,            \
                                     _Release,              \
                                     _BeginResourceControl,           \
                                     _BeginResourceTypeControl,       \
                                     _LooksAlive,           \
                                     _IsAlive,              \
                                     _Cancel               \
                                     )                      \
_Name.TableSize = CLRES_V3_FUNCTION_SIZE;                   \
_Name.Version = _Version;                                   \
_Name.V3Functions.Open = _Prefix##OpenV2;                   \
_Name.V3Functions.Close = _Prefix##Close;                   \
_Name.V3Functions.Online = _Prefix##OnlineV2;               \
_Name.V3Functions.Offline = _Prefix##OfflineV2;             \
_Name.V3Functions.Terminate = _Prefix##Terminate;           \
_Name.V3Functions.LooksAlive= _LooksAlive;                  \
_Name.V3Functions.IsAlive = _IsAlive;                       \
_Name.V3Functions.Arbitrate = _Arbitrate;                   \
_Name.V3Functions.Release = _Release;                       \
_Name.V3Functions.Cancel = _Cancel;                         \
_Name.V3Functions.BeginResourceControl= _BeginResourceControl; \
_Name.V3Functions.BeginResourceTypeControl= _BeginResourceTypeControl

#endif

#if CLUSAPI_VERSION >= CLUSAPI_VERSION_WINTHRESHOLD

#define CLRES_V4_FUNCTION_SIZE    sizeof(CLRES_V4_FUNCTIONS)
#define CLRES_VERSION_V4_00    0x400
#define CLRES_V4_FUNCTION_TABLE_SET( _Name, \
                                     _Version, \
                                     _Prefix, \
                                     _Arbitrate, \
                                     _Release, \
                                     _BeginResCtrl, \
                                     _BeginResTypeCtrl, \
                                     _LooksAlive, \
                                     _IsAlive, \
                                     _Cancel, \
                                     _BeginResCtrlAsUser, \
                                     _BeginResTypeCtrlAsUser \
                                     ) \
_Name.TableSize = CLRES_V4_FUNCTION_SIZE; \
_Name.Version = _Version; \
_Name.V4Functions.Open = _Prefix##OpenV2; \
_Name.V4Functions.Close = _Prefix##Close; \
_Name.V4Functions.Online = _Prefix##OnlineV2; \
_Name.V4Functions.Offline = _Prefix##OfflineV2; \
_Name.V4Functions.Terminate = _Prefix##Terminate; \
_Name.V4Functions.LooksAlive= _LooksAlive; \
_Name.V4Functions.IsAlive = _IsAlive; \
_Name.V4Functions.Arbitrate = _Arbitrate; \
_Name.V4Functions.Release = _Release; \
_Name.V4Functions.Cancel = _Cancel; \
_Name.V4Functions.BeginResourceControl= _BeginResCtrl; \
_Name.V4Functions.BeginResourceTypeControl= _BeginResTypeCtrl; \
_Name.V4Functions.BeginResourceControlAsUser= _BeginResCtrlAsUser; \
_Name.V4Functions.BeginResourceTypeControlAsUser= _BeginResTypeCtrlAsUser

#endif //CLUSAPI_VERSION >= CLUSAPI_VERSION_WINTHRESHOLD


#endif // ifndef _RESAPI_DEFINES_

#ifndef _RESAPI_
#define _RESAPI_

//
// Define a RESID
//

typedef PVOID RESID;

//
// Define a RESOURCE_HANDLE
//

typedef HANDLE   RESOURCE_HANDLE;

//
// Define the Resource Status structure.
//

#define ClusterResourceCannotComeOnlineOnThisNode ( CLUSTER_RESOURCE_STATE ) ( ClusterResourcePending - 1 )
#define ClusterResourceCannotComeOnlineOnAnyNode  ( CLUSTER_RESOURCE_STATE ) ( ClusterResourcePending - 2 )

typedef struct RESOURCE_STATUS {
    CLUSTER_RESOURCE_STATE  ResourceState;
    DWORD           CheckPoint;
    DWORD           WaitHint;
    HANDLE          EventHandle;
} RESOURCE_STATUS, *PRESOURCE_STATUS;


#if CLUSAPI_VERSION >= CLUSAPI_VERSION_WINDOWS8

// To be used with Control Code CLUSCTL_RESOURCE_TYPE_GET_LOCAL_NODE_UTILIZATION_INFO Only:
// Return with an array of NodeIntelligenceInfoElement(s)
struct NodeUtilizationInfoElement
{
    ULONGLONG Id;
    ULONGLONG AvailableMemory;
    ULONGLONG AvailableMemoryAfterReclamation;
};

// To be used with Control Code CLUSCTL_RESOURCE_GET_UTILIZATION_INFO Only:
// Return with an array of ResourceIntelligenceInfoElement(s)
struct ResourceUtilizationInfoElement
{
    ULONGLONG PhysicalNumaId;
    ULONGLONG CurrentMemory;
};

typedef enum VM_RESDLL_CONTEXT {
    VmResdllContextTurnOff =            0,
    VmResdllContextSave =               1,
    VmResdllContextShutdown =           2,
    VmResdllContextShutdownForce =      3,
    VmResdllContextLiveMigration =      4
} VM_RESDLL_CONTEXT, *PVM_RESDLL_CONTEXT;

typedef enum RESDLL_CONTEXT_OPERATION_TYPE {
    ResdllContextOperationTypeFailback,
    ResdllContextOperationTypeDrain,
    ResdllContextOperationTypeDrainFailure,
    ResdllContextOperationTypeEmbeddedFailure,
    ResdllContextOperationTypePreemption,
    ResdllContextOperationTypeNetworkDisconnect,
    ResdllContextOperationTypeNetworkDisconnectMoveRetry
} RESDLL_CONTEXT_OPERATION_TYPE, *PRESDLL_CONTEXT_OPERATION_TYPE;

#define CLUSCTL_GET_OPERATION_CONTEXT_PARAMS_VERSION_1  1

typedef struct GET_OPERATION_CONTEXT_PARAMS {
    DWORD                            Size;
    DWORD                            Version;
    RESDLL_CONTEXT_OPERATION_TYPE    Type;
    DWORD                            Priority;
} GET_OPERATION_CONTEXT_PARAMS, *PGET_OPERATION_CONTEXT_PARAMS;

#define CLUSRES_GET_OPERATION_CONTEXT_FLAGS       { CLUSRES_NAME_GET_OPERATION_CONTEXT_FLAGS,    NULL, CLUSPROP_FORMAT_DWORD, 0, 0, 0xFFFFFFFF, RESUTIL_PROPITEM_REQUIRED, 0 }
#define CLUSRES_NAME_GET_OPERATION_CONTEXT_FLAGS L"Flags"

#define RESOURCE_SPECIFIC_STATUS_PROP_ITEM       { CLUSREG_NAME_RES_STATUS,    NULL, CLUSPROP_FORMAT_SZ,                0, 0, 0, RESUTIL_PROPITEM_IN_MEMORY, 0 }
#define RESOURCE_SPECIFIC_DATA1_PROP_ITEM        { CLUSREG_NAME_RES_DATA1,     NULL, CLUSPROP_FORMAT_ULARGE_INTEGER,    0, 0, 0, RESUTIL_PROPITEM_READ_ONLY | RESUTIL_PROPITEM_IN_MEMORY, 0 }
#define RESOURCE_SPECIFIC_DATA2_PROP_ITEM        { CLUSREG_NAME_RES_DATA2,     NULL, CLUSPROP_FORMAT_ULARGE_INTEGER,    0, 0, 0, RESUTIL_PROPITEM_READ_ONLY | RESUTIL_PROPITEM_IN_MEMORY, 0 }

#define CLUSRESDLL_STATUS_OFFLINE_BUSY                           0x00000001
#define CLUSRESDLL_STATUS_OFFLINE_SOURCE_THROTTLED               0x00000002
#define CLUSRESDLL_STATUS_OFFLINE_DESTINATION_THROTTLED          0x00000004
#define CLUSRESDLL_STATUS_OFFLINE_DESTINATION_REJECTED           0x00000008
#define CLUSRESDLL_STATUS_INSUFFICIENT_MEMORY                    0x00000010
#define CLUSRESDLL_STATUS_INSUFFICIENT_PROCESSOR                 0x00000020
#define CLUSRESDLL_STATUS_INSUFFICIENT_OTHER_RESOURCES           0x00000040
#define CLUSRESDLL_STATUS_INVALID_PARAMETERS                     0x00000080
#define CLUSRESDLL_STATUS_NETWORK_NOT_AVAILABLE                  0x00000100
#define CLUSRESDLL_STATUS_DO_NOT_COLLECT_WER_REPORT              0x40000000

// Use high bit to indicate dump now
#define CLUSRESDLL_STATUS_DUMP_NOW                               0x80000000

typedef struct RESOURCE_STATUS_EX {
    CLUSTER_RESOURCE_STATE      ResourceState;
    DWORD                       CheckPoint;
    HANDLE                      EventHandle;
    DWORD                       ApplicationSpecificErrorCode;
    DWORD                       Flags;
    DWORD                       WaitHint;
} RESOURCE_STATUS_EX, *PRESOURCE_STATUS_EX;

typedef
DWORD
(_stdcall *PSET_RESOURCE_STATUS_ROUTINE_EX) (
    IN RESOURCE_HANDLE ResourceHandle,
    IN PRESOURCE_STATUS_EX ResourceStatus
    );

#define ResUtilInitializeResourceStatusEx( _resource_status_ ) \
    ZeroMemory( _resource_status_, sizeof(RESOURCE_STATUS_EX) )

#endif

#define ResUtilInitializeResourceStatus( _resource_status_ ) \
    ZeroMemory( _resource_status_, sizeof(RESOURCE_STATUS) )

//
// Define Resource DLL callback method for updating the state of a resource.
//

typedef
DWORD
(_stdcall *PSET_RESOURCE_STATUS_ROUTINE) (
    IN RESOURCE_HANDLE ResourceHandle,
    IN PRESOURCE_STATUS ResourceStatus
    );

//
// Define Resource DLL callback method for notifying that a quorum
// resource DLL failed to hold the quorum resource.
//
typedef
VOID
(_stdcall *PQUORUM_RESOURCE_LOST) (
    IN RESOURCE_HANDLE Resource
    );

//
// Define Resource DLL callback method for logging events.
//
typedef enum LOG_LEVEL {
    LOG_INFORMATION,
    LOG_WARNING,
    LOG_ERROR,
    LOG_SEVERE
} LOG_LEVEL, *PLOG_LEVEL;

typedef
VOID
(_stdcall *PLOG_EVENT_ROUTINE) (
    IN RESOURCE_HANDLE ResourceHandle,
    IN LOG_LEVEL LogLevel,
    IN LPCWSTR FormatString,
    ...
    );

typedef
RESID
(_stdcall *POPEN_ROUTINE) (
    IN LPCWSTR ResourceName,
    IN HKEY ResourceKey,
    IN RESOURCE_HANDLE ResourceHandle
    );

typedef
VOID
(_stdcall *PCLOSE_ROUTINE) (
    IN RESID Resource
    );

typedef
DWORD
(_stdcall *PONLINE_ROUTINE) (
    IN RESID Resource,
    IN OUT LPHANDLE EventHandle
    );

typedef
DWORD
(_stdcall *POFFLINE_ROUTINE) (
    IN RESID Resource
    );

typedef
VOID
(_stdcall *PTERMINATE_ROUTINE) (
    IN RESID Resource
    );

typedef
BOOL
(_stdcall *PIS_ALIVE_ROUTINE) (
    IN RESID Resource
    );

typedef
BOOL
(_stdcall *PLOOKS_ALIVE_ROUTINE) (
    IN RESID Resource
    );

typedef
DWORD
(_stdcall *PARBITRATE_ROUTINE) (
    IN RESID Resource,
    IN PQUORUM_RESOURCE_LOST LostQuorumResource
    );

typedef
DWORD
(_stdcall *PRELEASE_ROUTINE) (
    IN RESID Resource
    );

typedef
DWORD
(_stdcall *PRESOURCE_CONTROL_ROUTINE) (
    IN RESID Resource,
    IN DWORD ControlCode,
    IN PVOID InBuffer,
    IN DWORD InBufferSize,
    OUT PVOID OutBuffer,
    IN DWORD OutBufferSize,
    OUT LPDWORD BytesReturned
    );

typedef
DWORD
(_stdcall *PRESOURCE_TYPE_CONTROL_ROUTINE) (
    IN LPCWSTR ResourceTypeName,
    IN DWORD ControlCode,
    IN PVOID InBuffer,
    IN DWORD InBufferSize,
    OUT PVOID OutBuffer,
    IN DWORD OutBufferSize,
    OUT LPDWORD BytesReturned
    );


#if CLUSAPI_VERSION >= CLUSAPI_VERSION_WINDOWS8

#define CLUS_RESDLL_OPEN_RECOVER_MONITOR_STATE 0x00000001

typedef
RESID
(_stdcall *POPEN_V2_ROUTINE) (
    _In_ LPCWSTR ResourceName,
    _In_ HKEY ResourceKey,
    _In_ RESOURCE_HANDLE ResourceHandle,
    _In_ DWORD OpenFlags
    );

#define CLUS_RESDLL_ONLINE_RECOVER_MONITOR_STATE          0x00000001
#define CLUS_RESDLL_ONLINE_IGNORE_RESOURCE_STATUS         0x00000002
#define CLUS_RESDLL_ONLINE_RETURN_TO_SOURCE_NODE_ON_ERROR 0x00000004
#define CLUS_RESDLL_ONLINE_RESTORE_ONLINE_STATE           0x00000008
#define CLUS_RESDLL_ONLINE_IGNORE_NETWORK_CONNECTIVITY    0x00000010

typedef
DWORD
(_stdcall *PONLINE_V2_ROUTINE) (
    _In_ RESID Resource,
    _Out_ LPHANDLE EventHandle,
    _In_ DWORD OnlineFlags,
    _In_reads_bytes_opt_(InBufferSize) PBYTE InBuffer,
    _In_ DWORD InBufferSize,
    _In_ DWORD Reserved /*Reserved*/

    );


#define CLUS_RESDLL_OFFLINE_IGNORE_RESOURCE_STATUS                    0x00000001
#define CLUS_RESDLL_OFFLINE_RETURN_TO_SOURCE_NODE_ON_ERROR            0x00000002
#define CLUS_RESDLL_OFFLINE_QUEUE_ENABLED                             0x00000004
#define CLUS_RESDLL_OFFLINE_RETURNING_TO_SOURCE_NODE_BECAUSE_OF_ERROR 0x00000008
#define CLUS_RESDLL_OFFLINE_DUE_TO_EMBEDDED_FAILURE                   0x00000010
#define CLUS_RESDLL_OFFLINE_IGNORE_NETWORK_CONNECTIVITY               0x00000020
#define CLUS_RESDLL_OFFLINE_DO_NOT_UPDATE_PERSISTENT_STATE            0x00000040

typedef
DWORD
(_stdcall *POFFLINE_V2_ROUTINE) (
    _In_ RESID Resource,
    _In_opt_ LPCWSTR DestinationNodeName,
    _In_ DWORD OfflineFlags,
    _In_reads_bytes_opt_(InBufferSize) PBYTE InBuffer,
    _In_ DWORD InBufferSize,
    _In_ DWORD Reserved /*Reserved*/
    );

typedef
DWORD
(_stdcall *PCANCEL_ROUTINE) (
    IN RESID Resource,
    IN DWORD CancelFlags_RESERVED /*RESERVED*/
    );

typedef INT64 PRESTYPE_CTL_CTX;
typedef INT64 PRES_CTL_CTX;

typedef
DWORD
(_stdcall *PBEGIN_RESCALL_ROUTINE)(
    IN RESID Resource,
    IN DWORD ControlCode,
    IN PVOID InBuffer,
    IN DWORD InBufferSize,
    OUT PVOID OutBuffer,
    IN DWORD OutBufferSize,
    OUT LPDWORD BytesReturned,
    IN PRES_CTL_CTX context,
    OUT PBOOL ReturnedAsynchronously
    );

typedef
DWORD
(_stdcall *PBEGIN_RESTYPECALL_ROUTINE)(
    IN LPCWSTR ResourceTypeName,
    IN DWORD ControlCode,
    IN PVOID InBuffer,
    IN DWORD InBufferSize,
    OUT PVOID OutBuffer,
    IN DWORD OutBufferSize,
    OUT LPDWORD BytesReturned,
    IN PRESTYPE_CTL_CTX context,
    OUT PBOOL ReturnedAsynchronously
    );
#endif

typedef enum _RESOURCE_EXIT_STATE {
    ResourceExitStateContinue,
    ResourceExitStateTerminate,
    ResourceExitStateMax
} RESOURCE_EXIT_STATE;

#if CLUSAPI_VERSION >= CLUSAPI_VERSION_WINTHRESHOLD

typedef
DWORD
(_stdcall *PBEGIN_RESCALL_AS_USER_ROUTINE)(
    IN RESID Resource,
    IN HANDLE TokenHandle,
    IN DWORD ControlCode,
    IN PVOID InBuffer,
    IN DWORD InBufferSize,
    OUT PVOID OutBuffer,
    IN DWORD OutBufferSize,
    OUT LPDWORD BytesReturned,
    IN PRES_CTL_CTX context,
    OUT PBOOL ReturnedAsynchronously
    );

typedef
DWORD
(_stdcall *PBEGIN_RESTYPECALL_AS_USER_ROUTINE)(
    IN LPCWSTR ResourceTypeName,
    IN HANDLE TokenHandle,
    IN DWORD ControlCode,
    IN PVOID InBuffer,
    IN DWORD InBufferSize,
    OUT PVOID OutBuffer,
    IN DWORD OutBufferSize,
    OUT LPDWORD BytesReturned,
    IN PRESTYPE_CTL_CTX context,
    OUT PBOOL ReturnedAsynchronously
    );

#define CLUS_RESDLL_OPEN_DONT_DELETE_TEMP_DISK 0x00000002

#define RESTYPE_MONITOR_SHUTTING_DOWN_NODE_STOP         0x00000001
#define RESTYPE_MONITOR_SHUTTING_DOWN_CLUSSVC_CRASH     0x00000002

#endif //CLUSAPI_VERSION >= CLUSAPI_VERSION_WINTHRESHOLD


//***************************************************************
//
// Define the Function Table Format
//
//***************************************************************

//
// Version 1 Resource DLL Interface definition
//
typedef struct CLRES_V1_FUNCTIONS {
    POPEN_ROUTINE Open;
    PCLOSE_ROUTINE Close;
    PONLINE_ROUTINE Online;
    POFFLINE_ROUTINE Offline;
    PTERMINATE_ROUTINE Terminate;
    PLOOKS_ALIVE_ROUTINE LooksAlive;
    PIS_ALIVE_ROUTINE IsAlive;
    PARBITRATE_ROUTINE Arbitrate;
    PRELEASE_ROUTINE Release;
    PRESOURCE_CONTROL_ROUTINE ResourceControl;
    PRESOURCE_TYPE_CONTROL_ROUTINE ResourceTypeControl;
} CLRES_V1_FUNCTIONS, *PCLRES_V1_FUNCTIONS;

//
// Version 2 Resource DLL Interface definition
//

#if CLUSAPI_VERSION >= CLUSAPI_VERSION_WINDOWS8


typedef struct CLRES_V2_FUNCTIONS {
    POPEN_V2_ROUTINE Open;
    PCLOSE_ROUTINE Close;
    PONLINE_V2_ROUTINE Online;
    POFFLINE_V2_ROUTINE Offline;
    PTERMINATE_ROUTINE Terminate;
    PLOOKS_ALIVE_ROUTINE LooksAlive;
    PIS_ALIVE_ROUTINE IsAlive;
    PARBITRATE_ROUTINE Arbitrate;
    PRELEASE_ROUTINE Release;
    PRESOURCE_CONTROL_ROUTINE ResourceControl;
    PRESOURCE_TYPE_CONTROL_ROUTINE ResourceTypeControl;
    PCANCEL_ROUTINE Cancel;
} CLRES_V2_FUNCTIONS, *PCLRES_V2_FUNCTIONS;

//
// Version 3 Resource DLL Interface definition
//

typedef struct CLRES_V3_FUNCTIONS {
    POPEN_V2_ROUTINE Open;
    PCLOSE_ROUTINE Close;
    PONLINE_V2_ROUTINE Online;
    POFFLINE_V2_ROUTINE Offline;
    PTERMINATE_ROUTINE Terminate;
    PLOOKS_ALIVE_ROUTINE LooksAlive;
    PIS_ALIVE_ROUTINE IsAlive;
    PARBITRATE_ROUTINE Arbitrate;
    PRELEASE_ROUTINE Release;
    PBEGIN_RESCALL_ROUTINE BeginResourceControl;
    PBEGIN_RESTYPECALL_ROUTINE BeginResourceTypeControl;
    PCANCEL_ROUTINE Cancel;
} CLRES_V3_FUNCTIONS, *PCLRES_V3_FUNCTIONS;
#endif

#if CLUSAPI_VERSION >= CLUSAPI_VERSION_WINTHRESHOLD
//
// Version 4 Resource DLL Interface definition
//

typedef struct CLRES_V4_FUNCTIONS {
    POPEN_V2_ROUTINE Open;
    PCLOSE_ROUTINE Close;
    PONLINE_V2_ROUTINE Online;
    POFFLINE_V2_ROUTINE Offline;
    PTERMINATE_ROUTINE Terminate;
    PLOOKS_ALIVE_ROUTINE LooksAlive;
    PIS_ALIVE_ROUTINE IsAlive;
    PARBITRATE_ROUTINE Arbitrate;
    PRELEASE_ROUTINE Release;
    PBEGIN_RESCALL_ROUTINE BeginResourceControl;
    PBEGIN_RESTYPECALL_ROUTINE BeginResourceTypeControl;
    PCANCEL_ROUTINE Cancel;
    PBEGIN_RESCALL_AS_USER_ROUTINE BeginResourceControlAsUser;
    PBEGIN_RESTYPECALL_AS_USER_ROUTINE BeginResourceTypeControlAsUser;
} CLRES_V4_FUNCTIONS, *PCLRES_V4_FUNCTIONS;

#endif //CLUSAPI_VERSION >= CLUSAPI_VERSION_WINTHRESHOLD

//
// Resource DLL Function Table definition
//
#if _MSC_VER >= 1200
#pragma warning(push)
#endif
#pragma warning( disable : 4201 ) // nonstandard extension used : nameless struct/union
typedef struct CLRES_FUNCTION_TABLE {
    DWORD   TableSize;
    DWORD   Version;
    union {
        CLRES_V1_FUNCTIONS V1Functions;
#if CLUSAPI_VERSION >= CLUSAPI_VERSION_WINDOWS8
        CLRES_V2_FUNCTIONS V2Functions;
        CLRES_V3_FUNCTIONS V3Functions;
#endif
#if CLUSAPI_VERSION >= CLUSAPI_VERSION_WINTHRESHOLD
        CLRES_V4_FUNCTIONS V4Functions;
#endif
    } DUMMYUNIONNAME;
} CLRES_FUNCTION_TABLE, *PCLRES_FUNCTION_TABLE;
#if _MSC_VER >= 1200
#pragma warning(pop)
#else
#pragma warning( default : 4201 ) // nonstandard extension used : nameless struct/union
#endif

//
// Define the Resource DLL Startup routine. This is the only routine
// that should be exported from a resource DLL.
//

//
// Calculate the byte offset of a field in a structure of type type.
//

#ifndef FIELD_OFFSET
#define FIELD_OFFSET(type, field)    ((LONG)&(((type *)0)->field))
#endif

//
// large ints need more space than what was originally allocated.
//
typedef struct RESUTIL_LARGEINT_DATA {
    LARGE_INTEGER   Default;
    LARGE_INTEGER   Minimum;
    LARGE_INTEGER   Maximum;
} RESUTIL_LARGEINT_DATA, *PRESUTIL_LARGEINT_DATA;

typedef struct RESUTIL_ULARGEINT_DATA {
    ULARGE_INTEGER  Default;
    ULARGE_INTEGER  Minimum;
    ULARGE_INTEGER  Maximum;
} RESUTIL_ULARGEINT_DATA, *PRESUTIL_ULARGEINT_DATA;

typedef struct RESUTIL_FILETIME_DATA {
    FILETIME  Default;
    FILETIME  Minimum;
    FILETIME  Maximum;
} RESUTIL_FILETIME_DATA, *PRESUTIL_FILETIME_DATA;

//
// Property list structures and functions
//
typedef struct RESUTIL_PROPERTY_ITEM {
    LPWSTR  Name;               // Property name
    LPWSTR  KeyName;            // Name of value in cluster database
    DWORD   Format;             // Format: REG_SZ, REG_DWORD, etc.
    union {
        DWORD_PTR               DefaultPtr;
        DWORD                   Default;     // Default value
        LPVOID                  lpDefault;
        PRESUTIL_LARGEINT_DATA  LargeIntData;
        PRESUTIL_ULARGEINT_DATA ULargeIntData;
        PRESUTIL_FILETIME_DATA  FileTimeData;
    } DUMMYUNIONNAME;
    DWORD   Minimum;            // Minimum value
    DWORD   Maximum;            // Maximum value
    DWORD   Flags;              // Flags for this item
#define RESUTIL_PROPITEM_READ_ONLY    0x00000001  // Property is read-only
#define RESUTIL_PROPITEM_REQUIRED     0x00000002  // Property is required
#define RESUTIL_PROPITEM_SIGNED       0x00000004  // Numeric property is signed (defaults to unsigned)
#define RESUTIL_PROPITEM_IN_MEMORY    0x00000008  // Node-Local In-Memory Properties are not stored in Cluster Database

    DWORD   Offset;             // Byte offset to value in parameter block
                                //   Assumes MULTI_SZ and BINARY parameters
                                //   are LPWSTRs followed by DWORDs for length
} RESUTIL_PROPERTY_ITEM, *PRESUTIL_PROPERTY_ITEM;


typedef
DWORD
(_stdcall *PSTARTUP_ROUTINE) (
    IN LPCWSTR ResourceType,
    IN DWORD MinVersionSupported,
    IN DWORD MaxVersionSupported,
    IN PSET_RESOURCE_STATUS_ROUTINE SetResourceStatus,
    IN PLOG_EVENT_ROUTINE LogEvent,
    OUT PCLRES_FUNCTION_TABLE *FunctionTable
    );

#if CLUSAPI_VERSION >= CLUSAPI_VERSION_WINDOWS8

typedef enum _FAILURE_TYPE {
    FAILURE_TYPE_GENERAL            = 0,
    FAILURE_TYPE_EMBEDDED           = 1,
    FAILURE_TYPE_NETWORK_LOSS       = 2
} FAILURE_TYPE, *PFAILURE_TYPE;

typedef enum CLUSTER_RESOURCE_APPLICATION_STATE
{
    ClusterResourceApplicationStateUnknown = 1,
    ClusterResourceApplicationOSHeartBeat,
    ClusterResourceApplicationReady
} CLUSTER_RESOURCE_APPLICATION_STATE;

typedef
DWORD
(_stdcall *PSET_RESOURCE_LOCKED_MODE_ROUTINE) (
    _In_ RESOURCE_HANDLE ResourceHandle,
    _In_ BOOL LockedModeEnabled,
    _In_ DWORD LockedModeReason
);

typedef
DWORD
(_stdcall *PSIGNAL_FAILURE_ROUTINE) (
    _In_ RESOURCE_HANDLE ResourceHandle,
    _In_ FAILURE_TYPE FailureType,
    _In_ DWORD ApplicationSpecificErrorCode
);

typedef
DWORD
(_stdcall *PSET_RESOURCE_INMEMORY_NODELOCAL_PROPERTIES_ROUTINE) (
    _In_ RESOURCE_HANDLE ResourceHandle,
    _In_ unsigned char * propertyListBuffer,
    _In_ DWORD propertyListBufferSize
);

typedef
DWORD
(_stdcall *PEND_CONTROL_CALL) (
    _In_ const PRES_CTL_CTX context,
    _In_ DWORD status
);

typedef
DWORD
(_stdcall *PEND_TYPE_CONTROL_CALL) (
    _In_ const PRESTYPE_CTL_CTX context,
    _In_ DWORD status
);

typedef
DWORD
(_stdcall *PEXTEND_RES_CONTROL_CALL) (
    _In_ const PRES_CTL_CTX context,
    _In_ DWORD newTimeoutInMs
);

typedef
DWORD
(_stdcall *PEXTEND_RES_TYPE_CONTROL_CALL) (
    _In_ const PRESTYPE_CTL_CTX context,
    _In_ DWORD newTimeoutInMs
);

typedef
DWORD
(_stdcall *PRAISE_RES_TYPE_NOTIFICATION) (
    _In_ LPCWSTR ResourceType,
    _In_reads_bytes_(payloadSize) const PBYTE pPayload,
    _In_ DWORD payloadSize
);

typedef
DWORD
(_stdcall *PCHANGE_RESOURCE_PROCESS_FOR_DUMPS) (
    _In_ RESOURCE_HANDLE resource,
    _In_ LPCWSTR processName,
    _In_ DWORD processId,
    _In_ BOOL isAdd
    );
typedef
DWORD
(_stdcall *PCHANGE_RES_TYPE_PROCESS_FOR_DUMPS) (
    _In_ LPCWSTR resourceTypeName,
    _In_ LPCWSTR processName,
    _In_ DWORD processId,
    _In_ BOOL isAdd
    );

typedef
DWORD
(_stdcall *PSET_INTERNAL_STATE) (
    _In_ RESOURCE_HANDLE,
    _In_ CLUSTER_RESOURCE_APPLICATION_STATE stateType,
    _In_ BOOL active
    );

#define LOCKED_MODE_FLAGS_DONT_REMOVE_FROM_MOVE_QUEUE 0x00000001

typedef
DWORD
(_stdcall *PSET_RESOURCE_LOCKED_MODE_EX_ROUTINE) (
    _In_ RESOURCE_HANDLE ResourceHandle,
    _In_ BOOL LockedModeEnabled,
    _In_ DWORD LockedModeReason,
    _In_ DWORD LockedModeFlags
);

typedef
DWORD
(_stdcall *PREQUEST_DUMP_ROUTINE)(
    _In_ RESOURCE_HANDLE ResourceHandle,
    _In_ BOOL DumpDueToCallInProgress,
    _In_ DWORD DumpDelayInMs
);

#define CLUSRES_DISABLE_WPR_WATCHDOG_FOR_ONLINE_CALLS  0x00000001
#define CLUSRES_DISABLE_WPR_WATCHDOG_FOR_OFFLINE_CALLS 0x00000002

typedef
DWORD
(_stdcall *PSET_RESOURCE_WPR_POLICY_ROUTINE)(
    _In_ RESOURCE_HANDLE ResourceHandle,
    _In_ DWORD WprPolicyFlags
);

#define ARM_WPR_WATCHDOG_USING_CURRENT_START_AFTER 0xffffffffffffffffULL

typedef
DWORD
(_stdcall *PARM_WPR_WATCHDOG_FOR_CURRENT_RESOURCE_CALL_ROUTINE)(
    _In_ RESOURCE_HANDLE ResourceHandle,
    _In_ ULONGLONG TimeoutInMs
);

typedef struct CLRES_CALLBACK_FUNCTION_TABLE {
    PLOG_EVENT_ROUTINE                                                         LogEvent;
    PSET_RESOURCE_STATUS_ROUTINE_EX                                            SetResourceStatusEx;
    PSET_RESOURCE_LOCKED_MODE_ROUTINE                                          SetResourceLockedMode;
    PSIGNAL_FAILURE_ROUTINE                                                    SignalFailure;
    PSET_RESOURCE_INMEMORY_NODELOCAL_PROPERTIES_ROUTINE                        SetResourceInMemoryNodeLocalProperties;
    PEND_CONTROL_CALL                                                          EndControlCall;
    PEND_TYPE_CONTROL_CALL                                                     EndTypeControlCall;
    PEXTEND_RES_CONTROL_CALL                                                   ExtendControlCall;
    PEXTEND_RES_TYPE_CONTROL_CALL                                              ExtendTypeControlCall;
    PRAISE_RES_TYPE_NOTIFICATION                                               RaiseResTypeNotification;
    PCHANGE_RESOURCE_PROCESS_FOR_DUMPS                                         ChangeResourceProcessForDumps;
    PCHANGE_RES_TYPE_PROCESS_FOR_DUMPS                                         ChangeResTypeProcessForDumps;
    PSET_INTERNAL_STATE                                                        SetInternalState;
    PSET_RESOURCE_LOCKED_MODE_EX_ROUTINE                                       SetResourceLockedModeEx;
    PREQUEST_DUMP_ROUTINE                                                      RequestDump;
    PSET_RESOURCE_WPR_POLICY_ROUTINE                                           SetResourceWprPolicy;
    PARM_WPR_WATCHDOG_FOR_CURRENT_RESOURCE_CALL_ROUTINE                        ArmWprWatchdogForCurrentResourceCall;
}CLRES_CALLBACK_FUNCTION_TABLE, *PCLRES_CALLBACK_FUNCTION_TABLE;

typedef
DWORD
(_stdcall *PSTARTUP_EX_ROUTINE) (
    IN LPCWSTR                            ResourceType,
    IN DWORD                              MinVersionSupported,
    IN DWORD                              MaxVersionSupported,
    IN PCLRES_CALLBACK_FUNCTION_TABLE     MonitorCallbackFunctions,
    OUT PCLRES_FUNCTION_TABLE *           ResourceDllInterfaceFunctions
    );

#endif
//
// Define layout of shared memory used for tracking Resource Monitor state.
//
typedef enum RESOURCE_MONITOR_STATE {
    RmonInitializing,
    RmonIdle,
    RmonStartingResource,
    RmonInitializingResource,
    RmonOnlineResource,
    RmonOfflineResource,
    RmonShutdownResource,
    RmonDeletingResource,
    RmonIsAlivePoll,
    RmonLooksAlivePoll,
    RmonArbitrateResource,
    RmonReleaseResource,
    RmonResourceControl,
    RmonResourceTypeControl,
    RmonTerminateResource,
    RmonDeadlocked
} RESOURCE_MONITOR_STATE;

typedef struct MONITOR_STATE {
    LARGE_INTEGER LastUpdate;
    RESOURCE_MONITOR_STATE State;
    HANDLE ActiveResource;
    BOOL   ResmonStop;
} MONITOR_STATE, *PMONITOR_STATE;

// used for packaging up post-upgrade state information
// note that in some error cases, the old version fields can be 0
typedef struct POST_UPGRADE_VERSION_INFO {
    DWORD newMajorVersion;
    DWORD newUpgradeVersion;
    DWORD oldMajorVersion;
    DWORD oldUpgradeVersion;
    DWORD reserved;
} POST_UPGRADE_VERSION_INFO, *PPOST_UPGRADE_VERSION_INFO;

// Cluster Health Fault Structs

#define CLUSTER_HEALTH_FAULT_PROPERTY_NAME L"ClusterHealth"

#define CLUSTER_HEALTH_FAULT_ARGS 7
#define CLUSTER_HEALTH_FAULT_ID 0
#define CLUSTER_HEALTH_FAULT_ERRORTYPE 1
#define CLUSTER_HEALTH_FAULT_ERRORCODE 2
#define CLUSTER_HEALTH_FAULT_DESCRIPTION 3
#define CLUSTER_HEALTH_FAULT_PROVIDER 4
#define CLUSTER_HEALTH_FAULT_FLAGS 5
#define CLUSTER_HEALTH_FAULT_RESERVED 6

#define CLUSTER_HEALTH_FAULT_ID_LABEL L"Id"
#define CLUSTER_HEALTH_FAULT_ERRORTYPE_LABEL L"ErrorType"
#define CLUSTER_HEALTH_FAULT_ERRORCODE_LABEL L"ErrorCode"
#define CLUSTER_HEALTH_FAULT_DESCRIPTION_LABEL L"Description"
#define CLUSTER_HEALTH_FAULT_PROVIDER_LABEL L"Provider"
#define CLUSTER_HEALTH_FAULT_FLAGS_LABEL L"Flags"
#define CLUSTER_HEALTH_FAULT_RESERVED_LABEL L"Reserved"

typedef struct _CLUSTER_HEALTH_FAULT{
    LPWSTR Id;
    DWORD ErrorType;
    DWORD ErrorCode;
    LPWSTR Description;
    LPWSTR Provider;
    DWORD Flags;
    DWORD Reserved;
} CLUSTER_HEALTH_FAULT, *PCLUSTER_HEALTH_FAULT;

typedef struct _CLUSTER_HEALTH_FAULT_ARRAY
{
    DWORD numFaults;
    CLUSTER_HEALTH_FAULT * faults;
} CLUSTER_HEALTH_FAULT_ARRAY, *PCLUSTER_HEALTH_FAULT_ARRAY;

//
//Health fault utility routines
//
DWORD
InitializeClusterHealthFault(
    _Inout_ CLUSTER_HEALTH_FAULT* clusterHealthFault
    );

DWORD
InitializeClusterHealthFaultArray(
    _Inout_ CLUSTER_HEALTH_FAULT_ARRAY* clusterHealthFaultArray
    );

DWORD
FreeClusterHealthFault(
    _Inout_ CLUSTER_HEALTH_FAULT* clusterHealthFault
    );

DWORD
FreeClusterHealthFaultArray(
    _Inout_ CLUSTER_HEALTH_FAULT_ARRAY* clusterHealthFaultArray
    );

DWORD
WINAPI
ClusGetClusterHealthFaults(
    _In_ HCLUSTER hCluster,
    _Out_ CLUSTER_HEALTH_FAULT_ARRAY* objects,
    _In_ DWORD flags);

DWORD
WINAPI
ClusRemoveClusterHealthFault(
    _In_ HCLUSTER hCluster,
    _In_ LPCWSTR id,
    _In_ DWORD flags
);

DWORD
WINAPI
ClusAddClusterHealthFault(
    _In_ HCLUSTER hCluster,
    _In_ CLUSTER_HEALTH_FAULT* failure,
    _In_ DWORD /*flags*/
);

//
// Resource Utility Routines
//

DWORD
WINAPI
ResUtilStartResourceService(
    IN LPCWSTR pszServiceName,
    OUT LPSC_HANDLE phServiceHandle
    );

typedef DWORD
(WINAPI * PRESUTIL_START_RESOURCE_SERVICE)(
    IN LPCWSTR pszServiceName,
    OUT LPSC_HANDLE phServiceHandle
    );

DWORD
WINAPI
ResUtilVerifyResourceService(
    IN LPCWSTR pszServiceName
    );

typedef DWORD
(WINAPI * PRESUTIL_VERIFY_RESOURCE_SERVICE)(
    IN LPCWSTR pszServiceName
    );

DWORD
WINAPI
ResUtilStopResourceService(
    IN LPCWSTR pszServiceName
    );

typedef DWORD
(WINAPI * PRESUTIL_STOP_RESOURCE_SERVICE)(
    IN LPCWSTR pszServiceName
    );

DWORD
WINAPI
ResUtilVerifyService(
    IN SC_HANDLE hServiceHandle
    );

typedef DWORD
(WINAPI * PRESUTIL_VERIFY_SERVICE)(
    IN SC_HANDLE hServiceHandle
    );

DWORD
WINAPI
ResUtilStopService(
    IN SC_HANDLE hServiceHandle
    );

typedef DWORD
(WINAPI * PRESUTIL_STOP_SERVICE)(
    IN SC_HANDLE hServiceHandle
    );

DWORD
WINAPI
ResUtilCreateDirectoryTree(
    IN LPCWSTR pszPath
    );

typedef DWORD
(WINAPI * PRESUTIL_CREATE_DIRECTORY_TREE)(
    IN LPCWSTR pszPath
    );

BOOL
WINAPI
ResUtilIsPathValid(
    IN LPCWSTR pszPath
    );

typedef BOOL
(WINAPI * PRESUTIL_IS_PATH_VALID)(
    IN LPCWSTR pszPath
    );

DWORD
WINAPI
ResUtilEnumProperties(
    _In_ const PRESUTIL_PROPERTY_ITEM pPropertyTable,
    _Out_writes_bytes_to_(cbOutPropertiesSize, *pcbBytesReturned) LPWSTR pszOutProperties,
    _In_ DWORD cbOutPropertiesSize,
    _Out_ LPDWORD pcbBytesReturned,
    _Out_ LPDWORD pcbRequired
    );

typedef DWORD
(WINAPI * PRESUTIL_ENUM_PROPERTIES)(
    _In_ const PRESUTIL_PROPERTY_ITEM pPropertyTable,
    _Out_writes_bytes_to_(cbOutPropertiesSize, *pcbBytesReturned) LPWSTR pszOutProperties,
    _In_ DWORD cbOutPropertiesSize,
    _Out_ LPDWORD pcbBytesReturned,
    _Out_ LPDWORD pcbRequired
    );

DWORD
WINAPI
ResUtilEnumPrivateProperties(
    _In_ HKEY hkeyClusterKey,
    _Out_writes_bytes_to_(cbOutPropertiesSize, *pcbBytesReturned) LPWSTR pszOutProperties,
    _In_ DWORD cbOutPropertiesSize,
    _Out_ LPDWORD pcbBytesReturned,
    _Out_ LPDWORD pcbRequired
    );

typedef DWORD
(WINAPI * PRESUTIL_ENUM_PRIVATE_PROPERTIES)(
    _In_ HKEY hkeyClusterKey,
    _Out_writes_bytes_to_(cbOutPropertiesSize, *pcbBytesReturned) LPWSTR pszOutProperties,
    _In_ DWORD cbOutPropertiesSize,
    _Out_ LPDWORD pcbBytesReturned,
    _Out_ LPDWORD pcbRequired
    );

DWORD
WINAPI
ResUtilGetProperties(
    _In_ HKEY hkeyClusterKey,
    _In_ const PRESUTIL_PROPERTY_ITEM pPropertyTable,
    _Out_writes_bytes_to_(cbOutPropertyListSize, *pcbBytesReturned) PVOID pOutPropertyList,
    _In_ DWORD cbOutPropertyListSize,
    _Out_ LPDWORD pcbBytesReturned,
    _Out_ LPDWORD pcbRequired
    );

typedef DWORD
(WINAPI * PRESUTIL_GET_PROPERTIES)(
    _In_ HKEY hkeyClusterKey,
    _In_ const PRESUTIL_PROPERTY_ITEM pPropertyTable,
    _Out_writes_bytes_to_(cbOutPropertyListSize, *pcbBytesReturned) PVOID pOutPropertyList,
    _In_ DWORD cbOutPropertyListSize,
    _Out_ LPDWORD pcbBytesReturned,
    _Out_ LPDWORD pcbRequired
    );

DWORD
WINAPI
ResUtilGetAllProperties(
    _In_ HKEY hkeyClusterKey,
    _In_ const PRESUTIL_PROPERTY_ITEM pPropertyTable,
    _Out_writes_bytes_to_(cbOutPropertyListSize, *pcbBytesReturned) PVOID pOutPropertyList,
    _In_ DWORD cbOutPropertyListSize,
    _Out_ LPDWORD pcbBytesReturned,
    _Out_ LPDWORD pcbRequired
    );

typedef DWORD
(WINAPI * PRESUTIL_GET_ALL_PROPERTIES)(
    _In_ HKEY hkeyClusterKey,
    _In_ const PRESUTIL_PROPERTY_ITEM pPropertyTable,
    _Out_writes_bytes_to_(cbOutPropertyListSize, *pcbBytesReturned) PVOID pOutPropertyList,
    _In_ DWORD cbOutPropertyListSize,
    _Out_ LPDWORD pcbBytesReturned,
    _Out_ LPDWORD pcbRequired
    );

DWORD
WINAPI
ResUtilGetPrivateProperties(
    _In_ HKEY hkeyClusterKey,
    _Out_writes_bytes_to_(cbOutPropertyListSize, *pcbBytesReturned) PVOID pOutPropertyList,
    _In_ DWORD cbOutPropertyListSize,
    _Out_ LPDWORD pcbBytesReturned,
    _Out_ LPDWORD pcbRequired
    );

typedef DWORD
(WINAPI * PRESUTIL_GET_PRIVATE_PROPERTIES)(
    _In_ HKEY hkeyClusterKey,
    _Out_writes_bytes_to_(cbOutPropertyListSize, *pcbBytesReturned) PVOID pOutPropertyList,
    _In_ DWORD cbOutPropertyListSize,
    _Out_ LPDWORD pcbBytesReturned,
    _Out_ LPDWORD pcbRequired
    );

DWORD
WINAPI
ResUtilGetPropertySize(
    IN HKEY hkeyClusterKey,
    IN const PRESUTIL_PROPERTY_ITEM pPropertyTableItem,
    IN OUT LPDWORD pcbOutPropertyListSize,
    IN OUT LPDWORD pnPropertyCount
    );

typedef DWORD
(WINAPI * PRESUTIL_GET_PROPERTY_SIZE)(
    IN HKEY hkeyClusterKey,
    IN const PRESUTIL_PROPERTY_ITEM pPropertyTableItem,
    IN OUT LPDWORD pcbOutPropertyListSize,
    IN OUT LPDWORD pnPropertyCount
    );

_Success_( return == ERROR_SUCCESS )
DWORD
WINAPI
ResUtilGetProperty(
    _In_ HKEY hkeyClusterKey,
    _In_ const PRESUTIL_PROPERTY_ITEM pPropertyTableItem,
    _Inout_ _At_(*pOutPropertyItem, _Pre_readable_byte_size_(*pcbOutPropertyItemSize) _Post_readable_byte_size_(*pcbOutPropertyItemSize)) PVOID *pOutPropertyItem,
    _Inout_ LPDWORD pcbOutPropertyItemSize
    );

typedef
_Success_( return == ERROR_SUCCESS )
DWORD
(WINAPI * PRESUTIL_GET_PROPERTY)(
    _In_ HKEY hkeyClusterKey,
    _In_ const PRESUTIL_PROPERTY_ITEM pPropertyTableItem,
    _Inout_ _At_(*pOutPropertyItem, _Pre_readable_byte_size_(*pcbOutPropertyItemSize) _Post_readable_byte_size_(*pcbOutPropertyItemSize)) PVOID *pOutPropertyItem,
    _Inout_ LPDWORD pcbOutPropertyItemSize
    );

DWORD
WINAPI
ResUtilVerifyPropertyTable(
    _In_ const PRESUTIL_PROPERTY_ITEM pPropertyTable,
    _Reserved_ PVOID Reserved,
    _In_ BOOL bAllowUnknownProperties,
    _In_reads_bytes_(cbInPropertyListSize) const PVOID pInPropertyList,
    _In_ DWORD cbInPropertyListSize,
    _Out_opt_ LPBYTE pOutParams
    );

typedef DWORD
(WINAPI * PRESUTIL_VERIFY_PROPERTY_TABLE)(
    _In_ const PRESUTIL_PROPERTY_ITEM pPropertyTable,
    _Reserved_ PVOID Reserved,
    _In_ BOOL bAllowUnknownProperties,
    _In_reads_bytes_(cbInPropertyListSize) const PVOID pInPropertyList,
    _In_ DWORD cbInPropertyListSize,
    _Out_opt_ LPBYTE pOutParams
    );

DWORD
WINAPI
ResUtilSetPropertyTable(
    _In_ HKEY hkeyClusterKey,
    _In_ const PRESUTIL_PROPERTY_ITEM pPropertyTable,
    _Reserved_ PVOID Reserved,
    _In_ BOOL bAllowUnknownProperties,
    _In_reads_bytes_(cbInPropertyListSize) const PVOID pInPropertyList,
    _In_ DWORD cbInPropertyListSize,
    _Out_opt_ LPBYTE pOutParams
    );

typedef DWORD
(WINAPI * PRESUTIL_SET_PROPERTY_TABLE)(
    _In_ HKEY hkeyClusterKey,
    _In_ const PRESUTIL_PROPERTY_ITEM pPropertyTable,
    _Reserved_ PVOID Reserved,
    _In_ BOOL bAllowUnknownProperties,
    _In_reads_bytes_(cbInPropertyListSize) const PVOID pInPropertyList,
    _In_ DWORD cbInPropertyListSize,
    _Out_opt_ LPBYTE pOutParams
    );

DWORD
WINAPI
ResUtilSetPropertyTableEx(
    IN HKEY hkeyClusterKey,
    IN const PRESUTIL_PROPERTY_ITEM pPropertyTable,
    IN PVOID Reserved,
    IN BOOL bAllowUnknownProperties,
    IN const PVOID pInPropertyList,
    IN DWORD cbInPropertyListSize,
    IN BOOL bForceWrite,
    OUT OPTIONAL LPBYTE pOutParams
    );

typedef DWORD
(WINAPI * PRESUTIL_SET_PROPERTY_TABLE_EX)(
    IN HKEY hkeyClusterKey,
    IN const PRESUTIL_PROPERTY_ITEM pPropertyTable,
    IN PVOID Reserved,
    IN BOOL bAllowUnknownProperties,
    IN const PVOID pInPropertyList,
    IN DWORD cbInPropertyListSize,
    IN BOOL bForceWrite,
    OUT OPTIONAL LPBYTE pOutParams
    );

DWORD
WINAPI
ResUtilSetPropertyParameterBlock(
    IN HKEY hkeyClusterKey,
    IN const PRESUTIL_PROPERTY_ITEM pPropertyTable,
    IN PVOID Reserved,
    IN const LPBYTE pInParams,
    IN const PVOID pInPropertyList,
    IN DWORD cbInPropertyListSize,
    OUT OPTIONAL LPBYTE pOutParams
    );

typedef DWORD
(WINAPI * PRESUTIL_SET_PROPERTY_PARAMETER_BLOCK)(
    IN HKEY hkeyClusterKey,
    IN const PRESUTIL_PROPERTY_ITEM pPropertyTable,
    IN PVOID Reserved,
    IN const LPBYTE pInParams,
    IN const PVOID pInPropertyList,
    IN DWORD cbInPropertyListSize,
    OUT OPTIONAL LPBYTE pOutParams
    );

DWORD
WINAPI
ResUtilSetPropertyParameterBlockEx(
    IN HKEY hkeyClusterKey,
    IN const PRESUTIL_PROPERTY_ITEM pPropertyTable,
    IN PVOID Reserved,
    IN const LPBYTE pInParams,
    IN const PVOID pInPropertyList,
    IN DWORD cbInPropertyListSize,
    IN BOOL bForceWrite,
    OUT OPTIONAL LPBYTE pOutParams
    );

typedef DWORD
(WINAPI * PRESUTIL_SET_PROPERTY_PARAMETER_BLOCK_EX)(
    IN HKEY hkeyClusterKey,
    IN const PRESUTIL_PROPERTY_ITEM pPropertyTable,
    IN PVOID Reserved,
    IN const LPBYTE pInParams,
    IN const PVOID pInPropertyList,
    IN DWORD cbInPropertyListSize,
    IN BOOL bForceWrite,
    OUT OPTIONAL LPBYTE pOutParams
    );

DWORD
WINAPI
ResUtilSetUnknownProperties(
    _In_ HKEY hkeyClusterKey,
    _In_ const PRESUTIL_PROPERTY_ITEM pPropertyTable,
    _In_reads_bytes_(cbInPropertyListSize) const PVOID pInPropertyList,
    _In_ DWORD cbInPropertyListSize
    );

typedef DWORD
(WINAPI * PRESUTIL_SET_UNKNOWN_PROPERTIES)(
    _In_ HKEY hkeyClusterKey,
    _In_ const PRESUTIL_PROPERTY_ITEM pPropertyTable,
    _In_reads_bytes_(cbInPropertyListSize) const PVOID pInPropertyList,
    _In_ DWORD cbInPropertyListSize
    );

DWORD
WINAPI
ResUtilGetPropertiesToParameterBlock(
    _In_ HKEY hkeyClusterKey,
    _In_ const PRESUTIL_PROPERTY_ITEM pPropertyTable,
    _Inout_ LPBYTE pOutParams,
    _In_ BOOL bCheckForRequiredProperties,
    _Outptr_result_maybenull_ LPWSTR * pszNameOfPropInError
    );

typedef DWORD
(WINAPI * PRESUTIL_GET_PROPERTIES_TO_PARAMETER_BLOCK)(
    IN HKEY hkeyClusterKey,
    IN const PRESUTIL_PROPERTY_ITEM pPropertyTable,
    OUT LPBYTE pOutParams,
    IN BOOL bCheckForRequiredProperties,
    OUT OPTIONAL LPWSTR * pszNameOfPropInError
    );

typedef DWORD
(WINAPI * PRESUTIL_GET_PROPERTIES_TO_PARAMETER_BLOCK)(
    IN HKEY hkeyClusterKey,
    IN const PRESUTIL_PROPERTY_ITEM pPropertyTable,
    OUT LPBYTE pOutParams,
    IN BOOL bCheckForRequiredProperties,
    OUT OPTIONAL LPWSTR * pszNameOfPropInError
    );

DWORD
WINAPI
ResUtilPropertyListFromParameterBlock(
    _In_ const PRESUTIL_PROPERTY_ITEM pPropertyTable,
    _Out_writes_bytes_to_opt_(*pcbOutPropertyListSize, *pcbBytesReturned ) PVOID  pOutPropertyList,
    _Inout_ LPDWORD pcbOutPropertyListSize,
    _In_ const LPBYTE pInParams,
    _Out_ LPDWORD pcbBytesReturned,
    _Out_ LPDWORD pcbRequired
    );

typedef DWORD
(WINAPI * PRESUTIL_PROPERTY_LIST_FROM_PARAMETER_BLOCK)(
    _In_ const PRESUTIL_PROPERTY_ITEM pPropertyTable,
    _Out_writes_bytes_to_opt_(*pcbOutPropertyListSize, *pcbBytesReturned ) PVOID  pOutPropertyList,
    _Inout_ LPDWORD pcbOutPropertyListSize,
    _In_ const LPBYTE pInParams,
    _Out_ LPDWORD pcbBytesReturned,
    _Out_ LPDWORD pcbRequired
    );

DWORD
WINAPI
ResUtilDupParameterBlock(
    OUT LPBYTE pOutParams,
    IN const LPBYTE pInParams,
    IN const PRESUTIL_PROPERTY_ITEM pPropertyTable
    );

typedef DWORD
(WINAPI * PRESUTIL_DUP_PARAMETER_BLOCK)(
    OUT LPBYTE pOutParams,
    IN const LPBYTE pInParams,
    IN const PRESUTIL_PROPERTY_ITEM pPropertyTable
    );

void
WINAPI
ResUtilFreeParameterBlock(
    IN OUT LPBYTE pOutParams,
    IN const LPBYTE pInParams,
    IN const PRESUTIL_PROPERTY_ITEM pPropertyTable
    );

typedef void
(WINAPI * PRESUTIL_FREE_PARAMETER_BLOCK)(
    IN OUT LPBYTE pOutParams,
    IN const LPBYTE pInParams,
    IN const PRESUTIL_PROPERTY_ITEM pPropertyTable
    );

DWORD
WINAPI
ResUtilAddUnknownProperties(
    IN HKEY hkeyClusterKey,
    IN const PRESUTIL_PROPERTY_ITEM pPropertyTable,
    IN OUT PVOID pOutPropertyList,
    IN DWORD pcbOutPropertyListSize,
    IN OUT LPDWORD pcbBytesReturned,
    IN OUT LPDWORD pcbRequired
    );

typedef DWORD
(WINAPI * PRESUTIL_ADD_UNKNOWN_PROPERTIES)(
    IN HKEY hkeyClusterKey,
    IN const PRESUTIL_PROPERTY_ITEM pPropertyTable,
    IN OUT PVOID pOutPropertyList,
    IN DWORD pcbOutPropertyListSize,
    IN OUT LPDWORD pcbBytesReturned,
    IN OUT LPDWORD pcbRequired
    );

DWORD
WINAPI
ResUtilSetPrivatePropertyList(
    _In_ HKEY hkeyClusterKey,
    _In_reads_bytes_(cbInPropertyListSize) const PVOID pInPropertyList,
    _In_ DWORD cbInPropertyListSize
    );

typedef DWORD
(WINAPI * PRESUTIL_SET_PRIVATE_PROPERTY_LIST)(
    _In_ HKEY hkeyClusterKey,
    _In_reads_bytes_(cbInPropertyListSize) const PVOID pInPropertyList,
    _In_ DWORD cbInPropertyListSize
    );

DWORD
WINAPI
ResUtilVerifyPrivatePropertyList(
    _In_reads_bytes_(cbInPropertyListSize) const PVOID pInPropertyList,
    _In_ DWORD cbInPropertyListSize
    );

typedef DWORD
(WINAPI * PRESUTIL_VERIFY_PRIVATE_PROPERTY_LIST)(
    _In_reads_bytes_(cbInPropertyListSize) const PVOID pInPropertyList,
    _In_ DWORD cbInPropertyListSize
    );

PWSTR
WINAPI
ResUtilDupString(
    IN LPCWSTR pszInString
    );

typedef PWSTR
(WINAPI * PRESUTIL_DUP_STRING)(
    IN LPCWSTR pszInString
    );

DWORD
WINAPI
ResUtilGetBinaryValue(
    _In_ HKEY hkeyClusterKey,
    _In_ LPCWSTR pszValueName,
    _Outptr_result_bytebuffer_to_maybenull_(*pcbOutValueSize, *pcbOutValueSize) LPBYTE *ppbOutValue,
    _Out_ LPDWORD pcbOutValueSize
    );

typedef DWORD
(WINAPI * PRESUTIL_GET_BINARY_VALUE)(
    _In_ HKEY hkeyClusterKey,
    _In_ LPCWSTR pszValueName,
    _Outptr_result_bytebuffer_to_maybenull_(*pcbOutValueSize, *pcbOutValueSize) LPBYTE *ppbOutValue,
    _Out_ LPDWORD pcbOutValueSize
    );

LPWSTR
WINAPI
ResUtilGetSzValue(
    IN HKEY hkeyClusterKey,
    IN LPCWSTR pszValueName
    );

typedef LPWSTR
(WINAPI * PRESUTIL_GET_SZ_VALUE)(
    IN HKEY hkeyClusterKey,
    IN LPCWSTR pszValueName
    );

LPWSTR
WINAPI
ResUtilGetExpandSzValue(
    IN HKEY hkeyClusterKey,
    IN LPCWSTR pszValueName,
    IN BOOL bExpand
    );

typedef LPWSTR
(WINAPI * PRESUTIL_GET_EXPAND_SZ_VALUE)(
    IN HKEY hkeyClusterKey,
    IN LPCWSTR pszValueName,
    IN BOOL bExpand
    );

FORCEINLINE
DWORD
WINAPI_INLINE
ResUtilGetMultiSzValue(
    _In_ HKEY hkeyClusterKey,
    _In_ LPCWSTR pszValueName,
    _Outptr_result_bytebuffer_to_maybenull_(*pcbOutValueSize, *pcbOutValueSize) LPWSTR *ppszOutValue,
    _Out_ LPDWORD pcbOutValueSize
    )
{
    return ResUtilGetBinaryValue( hkeyClusterKey, pszValueName, (LPBYTE *) ppszOutValue, pcbOutValueSize );
}

DWORD
WINAPI
ResUtilGetDwordValue(
    _In_ HKEY hkeyClusterKey,
    _In_ LPCWSTR pszValueName,
    _Out_ LPDWORD pdwOutValue,
    _In_ DWORD dwDefaultValue
    );

typedef DWORD
(WINAPI * PRESUTIL_GET_DWORD_VALUE)(
    _In_ HKEY hkeyClusterKey,
    _In_ LPCWSTR pszValueName,
    _Out_ LPDWORD pdwOutValue,
    _In_ DWORD dwDefaultValue
    );

DWORD
WINAPI
ResUtilGetQwordValue(
    _In_ HKEY hkeyClusterKey,
    _In_ LPCWSTR pszValueName,
    _Out_ PULONGLONG pqwOutValue,
    _In_ ULONGLONG qwDefaultValue
    );

typedef DWORD
(WINAPI * PRESUTIL_GET_QWORD_VALUE)(
    _In_ HKEY hkeyClusterKey,
    _In_ LPCWSTR pszValueName,
    _Out_ PULONGLONG pqwOutValue,
    _In_ ULONGLONG qwDefaultValue
    );

DWORD
WINAPI
ResUtilSetBinaryValue(
    _In_ HKEY hkeyClusterKey,
    _In_ LPCWSTR pszValueName,
    _In_reads_bytes_(cbNewValueSize) const LPBYTE pbNewValue,
    _In_ DWORD cbNewValueSize,
    _Inout_opt_ _At_(*ppbOutValue, _When_(*ppbOutValue != NULL, _Pre_readable_byte_size_(*pcbOutValueSize)) _Post_readable_byte_size_(*pcbOutValueSize))
            LPBYTE *ppbOutValue,
    _Inout_ LPDWORD pcbOutValueSize
    );

typedef DWORD
(WINAPI * PRESUTIL_SET_BINARY_VALUE)(
    _In_ HKEY hkeyClusterKey,
    _In_ LPCWSTR pszValueName,
    _In_reads_bytes_(cbNewValueSize) const LPBYTE pbNewValue,
    _In_ DWORD cbNewValueSize,
    _Inout_opt_ _At_(*ppbOutValue, _When_(*ppbOutValue != NULL, _Pre_readable_byte_size_(*pcbOutValueSize)) _Post_readable_byte_size_(*pcbOutValueSize))
            LPBYTE *ppbOutValue,
    _Inout_ LPDWORD pcbOutValueSize
    );

DWORD
WINAPI
ResUtilSetSzValue(
    _In_ HKEY hkeyClusterKey,
    _In_ LPCWSTR pszValueName,
    _In_ LPCWSTR pszNewValue,
    _Inout_opt_ LPWSTR *ppszOutString
    );

typedef DWORD
(WINAPI * PRESUTIL_SET_SZ_VALUE)(
    _In_ HKEY hkeyClusterKey,
    _In_ LPCWSTR pszValueName,
    _In_ LPCWSTR pszNewValue,
    _Inout_opt_ LPWSTR *ppszOutString
    );

DWORD
WINAPI
ResUtilSetExpandSzValue(
    _In_ HKEY hkeyClusterKey,
    _In_ LPCWSTR pszValueName,
    _In_ LPCWSTR pszNewValue,
    _Inout_opt_ LPWSTR *ppszOutString
    );

typedef DWORD
(WINAPI * PRESUTIL_SET_EXPAND_SZ_VALUE)(
    _In_ HKEY hkeyClusterKey,
    _In_ LPCWSTR pszValueName,
    _In_ LPCWSTR pszNewValue,
    _Inout_opt_ LPWSTR *ppszOutString
    );

DWORD
WINAPI
ResUtilSetMultiSzValue(
    _In_ HKEY hkeyClusterKey,
    _In_ LPCWSTR pszValueName,
    _In_reads_bytes_(cbNewValueSize) LPCWSTR pszNewValue,
    _In_ DWORD cbNewValueSize,
    _Outptr_opt_result_bytebuffer_to_(*pcbOutValueSize, *pcbOutValueSize) LPWSTR *ppszOutValue,
    _Inout_opt_ LPDWORD pcbOutValueSize
    );

typedef DWORD
(WINAPI * PRESUTIL_SET_MULTI_SZ_VALUE)(
    _In_ HKEY hkeyClusterKey,
    _In_ LPCWSTR pszValueName,
    _In_reads_bytes_(cbNewValueSize) LPCWSTR pszNewValue,
    _In_ DWORD cbNewValueSize,
    _Outptr_opt_result_bytebuffer_to_(*pcbOutValueSize, *pcbOutValueSize) LPWSTR *ppszOutValue,
    _Inout_opt_ LPDWORD pcbOutValueSize
    );

DWORD
WINAPI
ResUtilSetDwordValue(
    IN HKEY hkeyClusterKey,
    IN LPCWSTR pszValueName,
    IN DWORD dwNewValue,
    IN OUT LPDWORD pdwOutValue
    );

typedef DWORD
(WINAPI * PRESUTIL_SET_DWORD_VALUE)(
    IN HKEY hkeyClusterKey,
    IN LPCWSTR pszValueName,
    IN DWORD dwNewValue,
    IN OUT LPDWORD pdwOutValue
    );

DWORD
WINAPI
ResUtilSetQwordValue(
    _In_ HKEY hkeyClusterKey,
    _In_ LPCWSTR pszValueName,
    _In_ ULONGLONG qwNewValue,
    _Inout_opt_ PULONGLONG pqwOutValue
    );

typedef DWORD
(WINAPI * PRESUTIL_SET_QWORD_VALUE)(
    _In_ HKEY hkeyClusterKey,
    _In_ LPCWSTR pszValueName,
    _In_ ULONGLONG qwNewValue,
    _Inout_opt_ PULONGLONG pqwOutValue
    );

DWORD
WINAPI
ResUtilSetValueEx(
    _In_ HKEY hkeyClusterKey,
    _In_ LPCWSTR valueName,
    _In_ DWORD valueType,
    _In_reads_bytes_(valueSize) const LPBYTE valueData,
    _In_ DWORD valueSize,
    _In_ DWORD flags
    );

DWORD
WINAPI
ResUtilGetBinaryProperty(
    _Outptr_result_bytebuffer_(*pcbOutValueSize) LPBYTE *ppbOutValue,
    _Out_ LPDWORD pcbOutValueSize,
    _In_ const PCLUSPROP_BINARY pValueStruct,
    _In_reads_bytes_opt_( cbOldValueSize ) const LPBYTE pbOldValue,
    _In_ DWORD cbOldValueSize,
    _Inout_ _At_(*ppPropertyList, _Pre_readable_byte_size_(*pcbPropertyListSize) _Post_readable_byte_size_(*pcbPropertyListSize)) LPBYTE *ppPropertyList,
    _Inout_ LPDWORD pcbPropertyListSize
    );

typedef DWORD
(WINAPI * PRESUTIL_GET_BINARY_PROPERTY)(
    _Outptr_result_bytebuffer_(*pcbOutValueSize) LPBYTE *ppbOutValue,
    _Out_ LPDWORD pcbOutValueSize,
    _In_ const PCLUSPROP_BINARY pValueStruct,
    _In_reads_bytes_opt_( cbOldValueSize ) const LPBYTE pbOldValue,
    _In_ DWORD cbOldValueSize,
    _Inout_ _At_(*ppPropertyList, _Pre_readable_byte_size_(*pcbPropertyListSize) _Post_readable_byte_size_(*pcbPropertyListSize)) LPBYTE *ppPropertyList,
    _Inout_ LPDWORD pcbPropertyListSize
    );

DWORD
WINAPI
ResUtilGetSzProperty(
    _Outptr_ LPWSTR *ppszOutValue,
    _In_ const PCLUSPROP_SZ pValueStruct,
    _In_opt_ LPCWSTR pszOldValue,
    _Inout_ _At_(*ppPropertyList, _Pre_readable_byte_size_(*pcbPropertyListSize) _Post_readable_byte_size_(*pcbPropertyListSize)) LPBYTE *ppPropertyList,
    _Inout_ LPDWORD pcbPropertyListSize
    );

typedef DWORD
(WINAPI * PRESUTIL_GET_SZ_PROPERTY)(
    _Outptr_ LPWSTR *ppszOutValue,
    _In_ const PCLUSPROP_SZ pValueStruct,
    _In_opt_ LPCWSTR pszOldValue,
    _Inout_ _At_(*ppPropertyList, _Pre_readable_byte_size_(*pcbPropertyListSize) _Post_readable_byte_size_(*pcbPropertyListSize)) LPBYTE *ppPropertyList,
    _Inout_ LPDWORD pcbPropertyListSize
    );

DWORD
WINAPI
ResUtilGetMultiSzProperty(
    _Outptr_result_bytebuffer_(*pcbOutValueSize) LPWSTR *ppszOutValue,
    _Out_ LPDWORD pcbOutValueSize,
    _In_ const PCLUSPROP_SZ pValueStruct,
    _In_reads_bytes_opt_(cbOldValueSize) LPCWSTR pszOldValue,
    _In_ DWORD cbOldValueSize,
    _Inout_ _At_(*ppPropertyList, _Pre_readable_byte_size_(*pcbPropertyListSize) _Post_readable_byte_size_(*pcbPropertyListSize)) LPBYTE *ppPropertyList,
    _Inout_ LPDWORD pcbPropertyListSize
    );

typedef DWORD
(WINAPI * PRESUTIL_GET_MULTI_SZ_PROPERTY)(
    _Outptr_result_bytebuffer_(*pcbOutValueSize) LPWSTR *ppszOutValue,
    _Out_ LPDWORD pcbOutValueSize,
    _In_ const PCLUSPROP_SZ pValueStruct,
    _In_reads_bytes_opt_(cbOldValueSize) LPCWSTR pszOldValue,
    _In_ DWORD cbOldValueSize,
    _Inout_ _At_(*ppPropertyList, _Pre_readable_byte_size_(*pcbPropertyListSize) _Post_readable_byte_size_(*pcbPropertyListSize)) LPBYTE *ppPropertyList,
    _Inout_ LPDWORD pcbPropertyListSize
    );

DWORD
WINAPI
ResUtilGetDwordProperty(
    OUT LPDWORD pdwOutValue,
    IN const PCLUSPROP_DWORD pValueStruct,
    IN DWORD dwOldValue,
    IN DWORD dwMinimum,
    IN DWORD dwMaximum,
    OUT LPBYTE * ppPropertyList,
    OUT LPDWORD pcbPropertyListSize
    );

typedef DWORD
(WINAPI * PRESUTIL_GET_DWORD_PROPERTY)(
    OUT LPDWORD pdwOutValue,
    IN const PCLUSPROP_DWORD pValueStruct,
    IN DWORD dwOldValue,
    IN DWORD dwMinimum,
    IN DWORD dwMaximum,
    OUT LPBYTE * ppPropertyList,
    OUT LPDWORD pcbPropertyListSize
    );

DWORD
WINAPI
ResUtilGetLongProperty(
    OUT LPLONG plOutValue,
    IN const PCLUSPROP_LONG pValueStruct,
    IN LONG lOldValue,
    IN LONG lMinimum,
    IN LONG lMaximum,
    OUT LPBYTE * ppPropertyList,
    OUT LPDWORD pcbPropertyListSize
    );

typedef DWORD
(WINAPI * PRESUTIL_GET_LONG_PROPERTY)(
    OUT LPLONG plOutValue,
    IN const PCLUSPROP_LONG pValueStruct,
    IN LONG lOldValue,
    IN LONG lMinimum,
    IN LONG lMaximum,
    OUT LPBYTE * ppPropertyList,
    OUT LPDWORD pcbPropertyListSize
    );

DWORD
WINAPI
ResUtilGetFileTimeProperty(
    OUT LPFILETIME pftOutValue,
    IN const PCLUSPROP_FILETIME pValueStruct,
    IN FILETIME ftOldValue,
    IN FILETIME ftMinimum,
    IN FILETIME ftMaximum,
    OUT LPBYTE * ppPropertyList,
    OUT LPDWORD pcbPropertyListSize
    );

typedef DWORD
(WINAPI * PRESUTIL_GET_FILETIME_PROPERTY)(
    OUT LPFILETIME pftOutValue,
    IN const PCLUSPROP_FILETIME pValueStruct,
    IN FILETIME ftOldValue,
    IN FILETIME ftMinimum,
    IN FILETIME ftMaximum,
    OUT LPBYTE * ppPropertyList,
    OUT LPDWORD pcbPropertyListSize
    );

LPVOID
WINAPI
ResUtilGetEnvironmentWithNetName(
    _In_ HRESOURCE hResource
    );

typedef LPVOID
(WINAPI * PRESUTIL_GET_ENVIRONMENT_WITH_NET_NAME)(
    _In_ HRESOURCE hResource
    );

DWORD
WINAPI
ResUtilFreeEnvironment(
    IN LPVOID lpEnvironment
    );

typedef DWORD
(WINAPI * PRESUTIL_FREE_ENVIRONMENT)(
    IN LPVOID lpEnvironment
    );

LPWSTR
WINAPI
ResUtilExpandEnvironmentStrings(
    IN LPCWSTR pszSrc
    );

typedef LPWSTR
(WINAPI * PRESUTIL_EXPAND_ENVIRONMENT_STRINGS)(
    IN LPCWSTR pszSrc
    );

DWORD
WINAPI
ResUtilSetResourceServiceEnvironment(
    IN LPCWSTR pszServiceName,
    IN HRESOURCE hResource,
    IN PLOG_EVENT_ROUTINE pfnLogEvent,
    IN RESOURCE_HANDLE hResourceHandle
    );

typedef DWORD
(WINAPI * PRESUTIL_SET_RESOURCE_SERVICE_ENVIRONMENT)(
    IN LPCWSTR pszServiceName,
    IN HRESOURCE hResource,
    IN PLOG_EVENT_ROUTINE pfnLogEvent,
    IN RESOURCE_HANDLE hResourceHandle
    );

DWORD
WINAPI
ResUtilRemoveResourceServiceEnvironment(
    IN  LPCWSTR             pszServiceName,
    IN  PLOG_EVENT_ROUTINE  pfnLogEvent,
    IN  RESOURCE_HANDLE     hResourceHandle
    );

typedef DWORD
(WINAPI * PRESUTIL_REMOVE_RESOURCE_SERVICE_ENVIRONMENT)(
    IN  LPCWSTR             pszServiceName,
    IN  PLOG_EVENT_ROUTINE  pfnLogEvent,
    IN  RESOURCE_HANDLE     hResourceHandle
    );

DWORD
WINAPI
ResUtilSetResourceServiceStartParameters(
    IN LPCWSTR pszServiceName,
    IN SC_HANDLE schSCMHandle,
    IN OUT LPSC_HANDLE phService,
    IN PLOG_EVENT_ROUTINE pfnLogEvent,
    IN RESOURCE_HANDLE hResourceHandle
    );

typedef DWORD
(WINAPI * PRESUTIL_SET_RESOURCE_SERVICE_START_PARAMETERS)(
    IN LPCWSTR pszServiceName,
    IN SC_HANDLE schSCMHandle,
    IN OUT LPSC_HANDLE phService,
    IN PLOG_EVENT_ROUTINE pfnLogEvent,
    IN RESOURCE_HANDLE hResourceHandle
    );

DWORD
WINAPI
ResUtilFindSzProperty(
    _In_reads_bytes_(cbPropertyListSize) const PVOID pPropertyList,
    _In_ DWORD cbPropertyListSize,
    _In_ LPCWSTR pszPropertyName,
    _Outptr_opt_ LPWSTR *pszPropertyValue
    );

typedef DWORD
(WINAPI * PRESUTIL_FIND_SZ_PROPERTY)(
    _In_reads_bytes_(cbPropertyListSize) const PVOID pPropertyList,
    _In_ DWORD cbPropertyListSize,
    _In_ LPCWSTR pszPropertyName,
    _Outptr_opt_ LPWSTR *pszPropertyValue
    );

DWORD
WINAPI
ResUtilFindExpandSzProperty(
    _In_reads_bytes_(cbPropertyListSize) const PVOID pPropertyList,
    _In_ DWORD cbPropertyListSize,
    _In_ LPCWSTR pszPropertyName,
    _Outptr_opt_ LPWSTR *pszPropertyValue
    );

typedef DWORD
(WINAPI * PRESUTIL_FIND_EXPAND_SZ_PROPERTY)(
    _In_reads_bytes_(cbPropertyListSize) const PVOID pPropertyList,
    _In_ DWORD cbPropertyListSize,
    _In_ LPCWSTR pszPropertyName,
    _Outptr_opt_ LPWSTR *pszPropertyValue
    );

DWORD
WINAPI
ResUtilFindExpandedSzProperty(
    _In_reads_bytes_(cbPropertyListSize) const PVOID pPropertyList,
    _In_ DWORD cbPropertyListSize,
    _In_ LPCWSTR pszPropertyName,
    _Outptr_opt_ LPWSTR *pszPropertyValue
    );

typedef DWORD
(WINAPI * PRESUTIL_FIND_EXPANDED_SZ_PROPERTY)(
    _In_reads_bytes_(cbPropertyListSize) const PVOID pPropertyList,
    _In_ DWORD cbPropertyListSize,
    _In_ LPCWSTR pszPropertyName,
    _Outptr_opt_ LPWSTR *pszPropertyValue
    );

DWORD
WINAPI
ResUtilFindDwordProperty(
    _In_reads_bytes_(cbPropertyListSize) const PVOID pPropertyList,
    _In_ DWORD cbPropertyListSize,
    _In_ LPCWSTR pszPropertyName,
    _Out_ LPDWORD pdwPropertyValue
    );

typedef DWORD
(WINAPI * PRESUTIL_FIND_DWORD_PROPERTY)(
    _In_reads_bytes_(cbPropertyListSize) const PVOID pPropertyList,
    _In_ DWORD cbPropertyListSize,
    _In_ LPCWSTR pszPropertyName,
    _Out_ LPDWORD pdwPropertyValue
    );

DWORD
WINAPI
ResUtilFindBinaryProperty(
    _In_reads_bytes_(cbPropertyListSize) const PVOID pPropertyList,
    _In_ DWORD cbPropertyListSize,
    _In_ LPCWSTR pszPropertyName,
    _Outptr_opt_result_bytebuffer_to_(*pcbPropertyValueSize, *pcbPropertyValueSize) LPBYTE *pbPropertyValue,
    _Out_opt_ LPDWORD pcbPropertyValueSize
    );

typedef DWORD
(WINAPI * PRESUTIL_FIND_BINARY_PROPERTY)(
    _In_reads_bytes_(cbPropertyListSize) const PVOID pPropertyList,
    _In_ DWORD cbPropertyListSize,
    _In_ LPCWSTR pszPropertyName,
    _Outptr_opt_result_bytebuffer_to_(*pcbPropertyValueSize, *pcbPropertyValueSize) LPBYTE *pbPropertyValue,
    _Out_opt_ LPDWORD pcbPropertyValueSize
    );

DWORD
WINAPI
ResUtilFindMultiSzProperty(
    _In_reads_bytes_(cbPropertyListSize) const PVOID pPropertyList,
    _In_ DWORD cbPropertyListSize,
    _In_ LPCWSTR pszPropertyName,
    _Outptr_result_bytebuffer_to_(*pcbPropertyValueSize, *pcbPropertyValueSize) LPWSTR *pszPropertyValue,
    _Out_ LPDWORD pcbPropertyValueSize
    );

typedef DWORD
(WINAPI * PRESUTIL_FIND_MULTI_SZ_PROPERTY)(
    _In_reads_bytes_(cbPropertyListSize) const PVOID pPropertyList,
    _In_ DWORD cbPropertyListSize,
    _In_ LPCWSTR pszPropertyName,
    _Outptr_result_bytebuffer_to_(*pcbPropertyValueSize, *pcbPropertyValueSize) LPWSTR *pszPropertyValue,
    _Out_ LPDWORD pcbPropertyValueSize
    );

DWORD
WINAPI
ResUtilFindLongProperty(
    _In_reads_bytes_(cbPropertyListSize) const PVOID pPropertyList,
    _In_ DWORD cbPropertyListSize,
    _In_ LPCWSTR pszPropertyName,
    _Out_ LPLONG plPropertyValue
    );

typedef DWORD
(WINAPI * PRESUTIL_FIND_LONG_PROPERTY)(
    _In_reads_bytes_(cbPropertyListSize) const PVOID pPropertyList,
    _In_ DWORD cbPropertyListSize,
    _In_ LPCWSTR pszPropertyName,
    _Out_ LPLONG plPropertyValue
    );

DWORD
WINAPI
ResUtilFindULargeIntegerProperty(
    _In_reads_bytes_(cbPropertyListSize) const PVOID pPropertyList,
    _In_ DWORD cbPropertyListSize,
    _In_ LPCWSTR pszPropertyName,
    _Out_ ULONGLONG *plPropertyValue
    );

typedef DWORD
(WINAPI * PRESUTIL_FIND_ULARGEINTEGER_PROPERTY)(
    _In_reads_bytes_(cbPropertyListSize) const PVOID pPropertyList,
    _In_ DWORD cbPropertyListSize,
    _In_ LPCWSTR pszPropertyName,
    _Out_ ULONGLONG *plPropertyValue
    );

DWORD
WINAPI
ResUtilFindFileTimeProperty(
    _In_reads_bytes_(cbPropertyListSize) const PVOID pPropertyList,
    _In_ DWORD cbPropertyListSize,
    _In_ LPCWSTR pszPropertyName,
    _Out_ LPFILETIME pftPropertyValue
    );

typedef DWORD
(WINAPI * PRESUTIL_FIND_FILETIME_PROPERTY)(
    _In_reads_bytes_(cbPropertyListSize) const PVOID pPropertyList,
    _In_ DWORD cbPropertyListSize,
    _In_ LPCWSTR pszPropertyName,
    _Out_ LPFILETIME pftPropertyValue
    );


//
// Common worker thread routines that allow a pending operation to
// be cancelled with CORRECT synchronization.
//
typedef struct CLUS_WORKER {
    HANDLE hThread;
    BOOL Terminate;
} CLUS_WORKER, *PCLUS_WORKER;

typedef DWORD (WINAPI *PWORKER_START_ROUTINE)(
    PCLUS_WORKER pWorker,
    LPVOID lpThreadParameter
    );

DWORD
WINAPI
ClusWorkerCreate(
    OUT PCLUS_WORKER lpWorker,
    IN PWORKER_START_ROUTINE lpStartAddress,
    IN PVOID lpParameter
    );

typedef DWORD
(WINAPI * PCLUSAPI_CLUS_WORKER_CREATE)(
    OUT PCLUS_WORKER lpWorker,
    IN PWORKER_START_ROUTINE lpStartAddress,
    IN PVOID lpParameter
    );

BOOL
WINAPI
ClusWorkerCheckTerminate(
    IN PCLUS_WORKER lpWorker
    );

typedef BOOL
(WINAPI * PCLUSAPIClusWorkerCheckTerminate)(
    IN PCLUS_WORKER lpWorker
    );

VOID
WINAPI
ClusWorkerTerminate(
    _In_ PCLUS_WORKER lpWorker
    );

typedef VOID
(WINAPI * PCLUSAPI_CLUS_WORKER_TERMINATE)(
    _In_ PCLUS_WORKER lpWorker
    );

DWORD
WINAPI
ClusWorkerTerminateEx(
    _Inout_ PCLUS_WORKER ClusWorker,
    _In_ DWORD TimeoutInMilliseconds,
    _In_ BOOL WaitOnly
    );

DWORD
ClusWorkersTerminate(
    _Inout_updates_(ClusWorkersCount) PCLUS_WORKER * ClusWorkers,
    _In_ size_t const ClusWorkersCount,
    _In_ DWORD TimeoutInMilliseconds,
    _In_ BOOL WaitOnly
    );

//Define enumerate resource callback function. This gets called for each resource enumerated
//by ResUtilEnumResources
typedef   DWORD (*LPRESOURCE_CALLBACK)( HRESOURCE, HRESOURCE , PVOID );
typedef   DWORD (*LPRESOURCE_CALLBACK_EX)( HCLUSTER, HRESOURCE, HRESOURCE , PVOID );
typedef   DWORD (*LPGROUP_CALLBACK_EX)( HCLUSTER, HGROUP, HGROUP , PVOID );
typedef   DWORD (*LPNODE_CALLBACK)( HCLUSTER, HNODE, CLUSTER_NODE_STATE, PVOID );


BOOL
WINAPI
ResUtilResourcesEqual(
    IN HRESOURCE    hSelf,
    IN HRESOURCE    hResource
    );


typedef BOOL
(WINAPI * PRESUTIL_RESOURCES_EQUAL)(
    IN HRESOURCE    hSelf,
    IN HRESOURCE    hResource
    );


BOOL
WINAPI
ResUtilResourceTypesEqual(
    IN LPCWSTR      lpszResourceTypeName,
    IN HRESOURCE    hResource
    );


typedef BOOL
(WINAPI * PRESUTIL_RESOURCE_TYPES_EQUAL)(
    IN LPCWSTR      lpszResourceTypeName,
    IN HRESOURCE    hResource
    );

BOOL
WINAPI
ResUtilIsResourceClassEqual(
    IN PCLUS_RESOURCE_CLASS_INFO    prci,
    IN HRESOURCE                    hResource
    );

typedef BOOL
(WINAPI * PRESUTIL_IS_RESOURCE_CLASS_EQUAL)(
    IN PCLUS_RESOURCE_CLASS_INFO    prci,
    IN HRESOURCE                    hResource
    );

DWORD
WINAPI
ResUtilEnumResources(
    IN HRESOURCE            hSelf,
    IN LPCWSTR              lpszResTypeName,
    IN LPRESOURCE_CALLBACK  pResCallBack,
    IN PVOID                pParameter
    );

typedef DWORD
(WINAPI * PRESUTIL_ENUM_RESOURCES)(
    IN HRESOURCE            hSelf,
    IN LPCWSTR              lpszResTypeName,
    IN LPRESOURCE_CALLBACK  pResCallBack,
    IN PVOID                pParameter
    );

DWORD
WINAPI
ResUtilEnumResourcesEx(
    IN HCLUSTER                 hCluster,
    IN HRESOURCE                hSelf,
    IN LPCWSTR                  lpszResTypeName,
    IN LPRESOURCE_CALLBACK_EX   pResCallBack,
    IN PVOID                    pParameter
    );

typedef DWORD
(WINAPI * PRESUTIL_ENUM_RESOURCES_EX)(
    IN HCLUSTER                 hCluster,
    IN HRESOURCE                hSelf,
    IN LPCWSTR                  lpszResTypeName,
    IN LPRESOURCE_CALLBACK_EX   pResCallBack,
    IN PVOID                    pParameter
    );

HRESOURCE
WINAPI
ResUtilGetResourceDependency(
    IN HANDLE       hSelf,
    IN LPCWSTR      lpszResourceType
    );

typedef HRESOURCE
(WINAPI * PRESUTIL_GET_RESOURCE_DEPENDENCY)(
    IN HANDLE       hSelf,
    IN LPCWSTR      lpszResourceType
    );

HRESOURCE
WINAPI
ResUtilGetResourceDependencyByName(
    IN HCLUSTER hCluster,
    IN HANDLE   hSelf,
    IN LPCWSTR  lpszResourceType,
    IN BOOL     bRecurse
    );

typedef HRESOURCE
(WINAPI * PRESUTIL_GET_RESOURCE_DEPENDENCY_BY_NAME)(
    IN HCLUSTER hCluster,
    IN HANDLE   hSelf,
    IN LPCWSTR  lpszResourceType,
    IN BOOL     bRecurse
    );

HRESOURCE
WINAPI
ResUtilGetResourceDependencyByClass(
    IN HCLUSTER                     hCluster,
    IN HANDLE                       hSelf,
    IN PCLUS_RESOURCE_CLASS_INFO    prci,
    IN BOOL                         bRecurse
    );

typedef HRESOURCE
(WINAPI * PRESUTIL_GET_RESOURCE_DEPENDENCY_BY_CLASS)(
    IN HCLUSTER                     hCluster,
    IN HANDLE                       hSelf,
    IN PCLUS_RESOURCE_CLASS_INFO    prci,
    IN BOOL                         bRecurse
    );

HRESOURCE
WINAPI
ResUtilGetResourceNameDependency(
    IN LPCWSTR      lpszResourceName,
    IN LPCWSTR      lpszResourceType
    );

typedef HRESOURCE
(WINAPI * PRESUTIL_GET_RESOURCE_NAME_DEPENDENCY)(
    IN LPCWSTR      lpszResourceName,
    IN LPCWSTR      lpszResourceType
    );

_Success_( return == ERROR_SUCCESS )
DWORD
WINAPI
ResUtilGetResourceDependentIPAddressProps(
    _In_ HRESOURCE hResource,
    _Out_writes_to_(*pcchAddress, *pcchAddress) LPWSTR pszAddress,
    _Inout_ DWORD *pcchAddress,
    _Out_writes_to_(*pcchSubnetMask, *pcchSubnetMask) LPWSTR pszSubnetMask,
    _Inout_ DWORD *pcchSubnetMask,
    _Out_writes_to_(*pcchNetwork, *pcchNetwork) LPWSTR pszNetwork,
    _Inout_ DWORD *pcchNetwork
    );

typedef
_Success_( return == ERROR_SUCCESS )
DWORD
(WINAPI * PRESUTIL_GET_RESOURCE_DEPENDENTIP_ADDRESS_PROPS)(
    _In_ HRESOURCE hResource,
    _Out_writes_to_(*pcchAddress, *pcchAddress) LPWSTR pszAddress,
    _Inout_ DWORD *pcchAddress,
    _Out_writes_to_(*pcchSubnetMask, *pcchSubnetMask) LPWSTR pszSubnetMask,
    _Inout_ DWORD *pcchSubnetMask,
    _Out_writes_to_(*pcchNetwork, *pcchNetwork) LPWSTR pszNetwork,
    _Inout_ DWORD *pcchNetwork
    );

_Success_( return == ERROR_SUCCESS )
DWORD
WINAPI
ResUtilFindDependentDiskResourceDriveLetter(
    _In_ HCLUSTER hCluster,              // handle to cluster
    _In_ HRESOURCE hResource,            // handle to resource to query for dependencies
    _Out_writes_to_(*pcchDriveLetter, *pcchDriveLetter) LPWSTR pszDriveLetter,       // buffer to store drive letter (ex. "X:\0")
    _Inout_ DWORD * pcchDriveLetter      // IN size of the pszDriveLetter buffer, OUT size of buffer required
    );

typedef
_Success_( return == ERROR_SUCCESS ) DWORD
(WINAPI * PRESUTIL_FIND_DEPENDENT_DISK_RESOURCE_DRIVE_LETTER)(
    _In_ HCLUSTER hCluster,              // handle to cluster
    _In_ HRESOURCE hResource,            // handle to resource to query for dependencies
    _Out_writes_to_(*pcchDriveLetter, *pcchDriveLetter) LPWSTR pszDriveLetter,       // buffer to store drive letter (ex. "X:\0")
    _Inout_ DWORD *pcchDriveLetter       // IN size of the pszDriveLetter buffer, OUT size of buffer required
    );

DWORD
WINAPI
ResUtilTerminateServiceProcessFromResDll(
    _In_ DWORD dwServicePid,
    _In_ BOOL bOffline,
    _Out_ PDWORD pdwResourceState,
    _In_ PLOG_EVENT_ROUTINE pfnLogEvent,
    _In_ RESOURCE_HANDLE hResourceHandle
    );

typedef DWORD
(WINAPI * PRESUTIL_TERMINATE_SERVICE_PROCESS_FROM_RES_DLL)(
    _In_ DWORD dwServicePid,
    _In_ BOOL bOffline,
    _Out_ PDWORD pdwResourceState,
    _In_ PLOG_EVENT_ROUTINE pfnLogEvent,
    _In_ RESOURCE_HANDLE hResourceHandle
    );

DWORD
WINAPI
ResUtilGetPropertyFormats(
    _In_ const PRESUTIL_PROPERTY_ITEM pPropertyTable,
    _Out_writes_bytes_to_(cbPropertyFormatListSize, *pcbBytesReturned) PVOID pOutPropertyFormatList,
    _In_ DWORD cbPropertyFormatListSize,
    _Out_ LPDWORD pcbBytesReturned,
    _Out_ LPDWORD pcbRequired
    );

typedef DWORD
(WINAPI * PRESUTIL_GET_PROPERTY_FORMATS)(
    _In_ const PRESUTIL_PROPERTY_ITEM pPropertyTable,
    _Out_writes_bytes_to_(cbPropertyFormatListSize, *pcbBytesReturned) PVOID pOutPropertyFormatList,
    _In_ DWORD cbPropertyFormatListSize,
    _Out_ LPDWORD pcbBytesReturned,
    _Out_ LPDWORD pcbRequired
    );


DWORD
WINAPI
ResUtilGetCoreClusterResources(
    _In_ HCLUSTER hCluster,
    _Out_ HRESOURCE *phClusterNameResource,
    _Out_ HRESOURCE *phClusterIPAddressResource,
    _Out_ HRESOURCE *phClusterQuorumResource
    );


typedef DWORD
(WINAPI * PRESUTIL_GET_CORE_CLUSTER_RESOURCES)(
    _In_ HCLUSTER hCluster,
    _Out_ HRESOURCE *phClusterNameResource,
    _Out_ HRESOURCE *phClusterIPAddressResource,
    _Out_ HRESOURCE *phClusterQuorumResource
    );

DWORD
WINAPI
ResUtilGetResourceName(
    _In_ HRESOURCE hResource,
    _Out_writes_to_(*pcchResourceNameInOut, *pcchResourceNameInOut) PWSTR pszResourceName,
    _Inout_ DWORD *pcchResourceNameInOut
    );

typedef DWORD
(WINAPI * PRESUTIL_GET_RESOURCE_NAME)(
    _In_ HRESOURCE hResource,
    _Out_writes_to_(*pcchResourceNameInOut, *pcchResourceNameInOut) PWSTR pszResourceName,
    _Inout_ DWORD *pcchResourceNameInOut
    );

typedef enum _CLUSTER_ROLE {
    ClusterRoleDHCP,
    ClusterRoleDTC,
    ClusterRoleFileServer,
    ClusterRoleGenericApplication,
    ClusterRoleGenericScript,
    ClusterRoleGenericService,
    ClusterRoleISCSINameServer,             // Maps to CLUS_RESTYPE_NAME_ISNS
    ClusterRoleMSMQ,
    ClusterRoleNFS,
    ClusterRolePrintServer,
    ClusterRoleStandAloneNamespaceServer,   // FileShare with bit set
    ClusterRoleVolumeShadowCopyServiceTask,
    ClusterRoleWINS,
    ClusterRoleTaskScheduler,
    ClusterRoleNetworkFileSystem,
    ClusterRoleDFSReplicatedFolder,
    ClusterRoleDistributedFileSystem,
    ClusterRoleDistributedNetworkName,
    ClusterRoleFileShare,
    ClusterRoleFileShareWitness,
    ClusterRoleHardDisk,
    ClusterRoleIPAddress,
    ClusterRoleIPV6Address,
    ClusterRoleIPV6TunnelAddress,
    ClusterRoleISCSITargetServer,
    ClusterRoleNetworkName,
    ClusterRolePhysicalDisk,                // Same as ClusterRoleHardDisk
    ClusterRoleSODAFileServer,
    ClusterRoleStoragePool,
    ClusterRoleVirtualMachine,
    ClusterRoleVirtualMachineConfiguration,
    ClusterRoleVirtualMachineReplicaBroker,
    ClusterRoleKeyValueStore
} CLUSTER_ROLE;

typedef enum _CLUSTER_ROLE_STATE {
    ClusterRoleUnknown = -1,
    ClusterRoleClustered,
    ClusterRoleUnclustered
} CLUSTER_ROLE_STATE;

_Success_(return >= 0) // != ClusterRoleUnknown
CLUSTER_ROLE_STATE
WINAPI
ResUtilGetClusterRoleState(
    _In_ HCLUSTER       hCluster,
    _In_ CLUSTER_ROLE   eClusterRole
    );

// TODO: should we implement versioning here like ClusApi - maybe overkill for now.
BOOL
WINAPI
ClusterIsPathOnSharedVolume(
    _In_ LPCWSTR lpszPathName
);

typedef BOOL
(WINAPI * PCLUSTER_IS_PATH_ON_SHARED_VOLUME)(
    _In_ LPCWSTR lpszPathName
    );

BOOL
WINAPI
ClusterGetVolumePathName(
    _In_   LPCWSTR lpszFileName,
    _Out_  LPWSTR lpszVolumePathName,
    _In_   DWORD cchBufferLength
    );

typedef BOOL
(WINAPI * PCLUSTER_GET_VOLUME_PATH_NAME)(
    _In_   LPCWSTR lpszFileName,
    _Out_  LPWSTR lpszVolumePathName,
    _In_   DWORD cchBufferLength
    );

BOOL
WINAPI
ClusterGetVolumeNameForVolumeMountPoint(
    _In_   LPCWSTR lpszVolumeMountPoint,
    _Out_  LPWSTR lpszVolumeName,
    _In_   DWORD cchBufferLength
    );

typedef BOOL
(WINAPI * PCLUSTER_GET_VOLUME_NAME_FOR_VOLUME_MOUNT_POINT)(
    _In_   LPCWSTR lpszVolumeMountPoint,
    _Out_  LPWSTR lpszVolumeName,
    _In_   DWORD cchBufferLength
    );

DWORD
WINAPI
ClusterPrepareSharedVolumeForBackup(
    _In_ LPCWSTR lpszFileName,
    _Out_ LPWSTR lpszVolumePathName,
    _Inout_ LPDWORD lpcchVolumePathName,
    _Out_ LPWSTR lpszVolumeName,
    _Inout_ LPDWORD lpcchVolumeName
    );

typedef DWORD
(WINAPI * PCLUSTER_PREPARE_SHARED_VOLUME_FOR_BACKUP)(
    _In_ LPCWSTR lpszFileName,
    _Out_ LPWSTR lpszVolumePathName,
    _Inout_ LPDWORD lpcchVolumePathName,
    _Out_ LPWSTR lpszVolumeName,
    _Inout_ LPDWORD lpcchVolumeName
    );

DWORD
WINAPI
ClusterClearBackupStateForSharedVolume(
    _In_ LPCWSTR lpszVolumePathName
    );

typedef DWORD
(WINAPI * PCLUSTER_CLEAR_BACKUP_STATE_FOR_SHARED_VOLUME)(
    _In_ LPCWSTR lpszVolumePathName
    );

// Begin updates to ResUtil functions that accept access level
#if ( CLUSAPI_VERSION >= CLUSAPI_VERSION_WINDOWS8 )
DWORD
WINAPI
ResUtilSetResourceServiceStartParametersEx(
    IN      LPCWSTR             pszServiceName,
    IN      SC_HANDLE           schSCMHandle,
    IN OUT  LPSC_HANDLE         phService,
    IN      DWORD               dwDesiredAccess,
    IN      PLOG_EVENT_ROUTINE  pfnLogEvent,
    IN      RESOURCE_HANDLE     hResourceHandle
    );

typedef DWORD
(WINAPI * PRESUTIL_SET_RESOURCE_SERVICE_START_PARAMETERS_EX)(
    IN      LPCWSTR             pszServiceName,
    IN      SC_HANDLE           schSCMHandle,
    IN OUT  LPSC_HANDLE         phService,
    IN      DWORD               dwDesiredAccess,
    IN      PLOG_EVENT_ROUTINE  pfnLogEvent,
    IN      RESOURCE_HANDLE     hResourceHandle
    );

DWORD
WINAPI
ResUtilEnumResourcesEx2(
    IN HCLUSTER                 hCluster,
    IN HRESOURCE                hSelf,
    IN LPCWSTR                  lpszResTypeName,
    IN LPRESOURCE_CALLBACK_EX   pResCallBack,
    IN PVOID                    pParameter,
    IN DWORD                    dwDesiredAccess
    );

typedef DWORD
(WINAPI * PRESUTIL_ENUM_RESOURCES_EX2)(
    IN HCLUSTER                 hCluster,
    IN HRESOURCE                hSelf,
    IN LPCWSTR                  lpszResTypeName,
    IN LPRESOURCE_CALLBACK_EX   pResCallBack,
    IN PVOID                    pParameter,
    IN DWORD                    dwDesiredAccess
    );

HRESOURCE
WINAPI
ResUtilGetResourceDependencyEx(
    IN HANDLE       hSelf,
    IN LPCWSTR      lpszResourceType,
    IN DWORD        dwDesiredAccess
    );

typedef HRESOURCE
(WINAPI * PRESUTIL_GET_RESOURCE_DEPENDENCY_EX)(
    IN HANDLE       hSelf,
    IN LPCWSTR      lpszResourceType,
    IN DWORD        dwDesiredAccess
    );

HRESOURCE
WINAPI
ResUtilGetResourceDependencyByNameEx(
    IN HCLUSTER hCluster,
    IN HANDLE   hSelf,
    IN LPCWSTR  lpszResourceType,
    IN BOOL     bRecurse,
    IN DWORD    dwDesiredAccess
    );

typedef HRESOURCE
(WINAPI * PRESUTIL_GET_RESOURCE_DEPENDENCY_BY_NAME_EX)(
    IN HCLUSTER hCluster,
    IN HANDLE   hSelf,
    IN LPCWSTR  lpszResourceType,
    IN BOOL     bRecurse,
    IN DWORD    dwDesiredAccess
    );

HRESOURCE
WINAPI
ResUtilGetResourceDependencyByClassEx(
    IN HCLUSTER                     hCluster,
    IN HANDLE                       hSelf,
    IN PCLUS_RESOURCE_CLASS_INFO    prci,
    IN BOOL                         bRecurse,
    IN DWORD                        dwDesiredAccess
    );

typedef HRESOURCE
(WINAPI * PRESUTIL_GET_RESOURCE_DEPENDENCY_BY_CLASS_EX)(
    IN HCLUSTER                     hCluster,
    IN HANDLE                       hSelf,
    IN PCLUS_RESOURCE_CLASS_INFO    prci,
    IN BOOL                         bRecurse,
    IN DWORD                        dwDesiredAccess
    );

HRESOURCE
WINAPI
ResUtilGetResourceNameDependencyEx(
    IN LPCWSTR      lpszResourceName,
    IN LPCWSTR      lpszResourceType,
    IN DWORD        dwDesiredAccess
    );

typedef HRESOURCE
(WINAPI * PRESUTIL_GET_RESOURCE_NAME_DEPENDENCY_EX)(
    IN LPCWSTR      lpszResourceName,
    IN LPCWSTR      lpszResourceType,
    IN DWORD        dwDesiredAccess
    );

DWORD
WINAPI
ResUtilGetCoreClusterResourcesEx(
    _In_        HCLUSTER    hClusterIn,
    _Out_opt_   HRESOURCE  *phClusterNameResourceOut,
    _Out_opt_   HRESOURCE  *phClusterQuorumResourceOut,
    _In_        DWORD       dwDesiredAccess
    );

typedef DWORD
(WINAPI * PRESUTIL_GET_CORE_CLUSTER_RESOURCES_EX)(
    _In_  HCLUSTER    hClusterIn,
    _Out_ HRESOURCE *phClusterNameResourceOut,
    _Out_ HRESOURCE *phClusterIPAddressResourceOut,
    _Out_ HRESOURCE *phClusterQuorumResourceOut,
    _In_   DWORD       dwDesiredAccess
    );

#define CLUS_CREATE_CRYPT_CONTAINER_NOT_FOUND       0x0001

typedef struct _HCLUSCRYPTPROVIDER *HCLUSCRYPTPROVIDER;

HCLUSCRYPTPROVIDER
WINAPI
OpenClusterCryptProvider(
    _In_ LPCWSTR lpszResource,
    _In_ LPCTSTR lpszProvider,
    _In_ DWORD dwType,
    _In_ DWORD dwFlags
    );

typedef HCLUSCRYPTPROVIDER
(WINAPI * POPEN_CLUSTER_CRYPT_PROVIDER)(
    _In_ LPCWSTR lpszResource,
    _In_ LPCTSTR lpszProvider,
    _In_ DWORD dwType,
    _In_ DWORD dwFlags
    );

HCLUSCRYPTPROVIDER
WINAPI
OpenClusterCryptProviderEx(
    _In_ LPCWSTR lpszResource,
    _In_ LPCWSTR lpszKeyname,
    _In_ LPCTSTR lpszProvider,
    _In_ DWORD dwType,
    _In_ DWORD dwFlags
    );

typedef HCLUSCRYPTPROVIDER
(WINAPI * POPEN_CLUSTER_CRYPT_PROVIDEREX)(
    _In_ LPCWSTR lpszResource,
    _In_ LPCWSTR lpszKeyname,
    _In_ LPCTSTR lpszProvider,
    _In_ DWORD dwType,
    _In_ DWORD dwFlags
    );

DWORD
WINAPI
CloseClusterCryptProvider(
    _In_ HCLUSCRYPTPROVIDER hClusCryptProvider
    );

typedef DWORD
(WINAPI * PCLOSE_CLUSTER_CRYPT_PROVIDER)(
    _In_ HCLUSCRYPTPROVIDER hClusCryptProvider
    );

DWORD
WINAPI
ClusterEncrypt(
    _In_ HCLUSCRYPTPROVIDER hClusCryptProvider,
    _In_reads_(cbData) PBYTE pData,
    _In_ DWORD cbData,
    _Outptr_result_bytebuffer_(*pcbData) PBYTE* ppData,
    _Out_ PDWORD pcbData
    );

typedef DWORD
(WINAPI * PCLUSTER_ENCRYPT)(
    _In_ HCLUSCRYPTPROVIDER hClusCryptProvider,
    _In_reads_(cbData) PBYTE pData,
    _In_ DWORD cbData,
    _Outptr_result_bytebuffer_(*pcbData) PBYTE* ppData,
    _Out_ PDWORD pcbData
    );

DWORD
WINAPI
ClusterDecrypt(
    _In_ HCLUSCRYPTPROVIDER hClusCryptProvider,
    _In_reads_(cbData) PBYTE pCryptInput,
    _In_ DWORD cbCryptInput,
    _Outptr_result_bytebuffer_(*pcbCryptOutput) PBYTE* ppCryptOutput,
    _Out_ PDWORD pcbCryptOutput
    );

typedef DWORD
(WINAPI * PCLUSTER_DECRYPT)(
    _In_ HCLUSCRYPTPROVIDER hClusCryptProvider,
    _In_reads_(cbData) PBYTE pCryptInput,
    _In_ DWORD cbCryptInput,
    _Outptr_result_bytebuffer_(*pcbCryptOutput) PBYTE* ppCryptOutput,
    _Out_ PDWORD pcbCryptOutput
    );

DWORD
WINAPI
FreeClusterCrypt(
    _In_ PVOID pCryptInfo
    );

typedef DWORD
(WINAPI * PFREE_CLUSTER_CRYPT)(
    _In_ PVOID pCryptInfo
    );

DWORD
WINAPI
ResUtilVerifyShutdownSafe(
    _In_  DWORD flags,
    _In_  DWORD reason,
    _Out_ PDWORD pResult
    );

typedef DWORD
(WINAPI * PRES_UTIL_VERIFY_SHUTDOWN_SAFE)(
    _In_  DWORD flags,
    _In_  DWORD reason,
    _Out_ PDWORD pResult
    );

#endif // ( CLUSAPI_VERSION >= CLUSAPI_VERSION_WINDOWS8 )

typedef struct _PaxosTagCStruct {
    unsigned __int64 __padding__PaxosTagVtable; //0

    unsigned __int64 __padding__NextEpochVtable; //8
    unsigned __int64 __padding__NextEpoch_DateTimeVtable; //16
    unsigned __int64 NextEpoch_DateTime_ticks; //24
    int NextEpoch_Value;//32
    DWORD __padding__BoundryNextEpoch;

    unsigned __int64 __padding__EpochVtable;//40
    unsigned __int64 __padding__Epoch_DateTimeVtable;//48
    unsigned __int64 Epoch_DateTime_ticks;//56
    int Epoch_Value;//64
    DWORD __padding__BoundryEpoch;//68

    int Sequence;//80
    DWORD __padding__BoundrySequence;//84
} PaxosTagCStruct, *PPaxosTagCStruct;


typedef struct _WitnessTagUpdateHelper
{
    int Version;
    PaxosTagCStruct paxosToSet;
    PaxosTagCStruct paxosToValidate;
}WitnessTagUpdateHelper;

typedef struct _WitnessTagHelper
{
    int Version;
    PaxosTagCStruct paxosToValidate;
}WitnessTagHelper;

BOOL ResUtilPaxosComparer(const PaxosTagCStruct *const left, const PaxosTagCStruct * const right);

BOOL ResUtilLeftPaxosIsLessThanRight(const PaxosTagCStruct *const left, const PaxosTagCStruct * const right);

DWORD ResUtilsDeleteKeyTree(HKEY key, LPCWSTR keyName, BOOL treatNoKeyAsError);

DWORD
WINAPI
ResUtilGroupsEqual(
    IN HGROUP    hSelf,
    IN HGROUP    hGroup,
    OUT BOOL*    pEqual
    );

DWORD
WINAPI
ResUtilEnumGroups(
    IN HCLUSTER             hCluster,
    IN HGROUP               hSelf,
    IN LPGROUP_CALLBACK_EX  pResCallBack,
    IN PVOID                pParameter
    );

DWORD
WINAPI
ResUtilEnumGroupsEx(
    IN HCLUSTER                 hCluster,
    IN HGROUP               hSelf,
    IN CLUSGROUP_TYPE           groupType,
    IN LPGROUP_CALLBACK_EX      pResCallBack,
    IN PVOID                    pParameter
    );


DWORD
WINAPI
ResUtilDupGroup(IN HGROUP group, OUT HGROUP *copy);

DWORD
WINAPI
ResUtilGetClusterGroupType(IN HGROUP hGroup, OUT CLUSGROUP_TYPE * groupType);

HGROUP
WINAPI
ResUtilGetCoreGroup(IN HCLUSTER hCluster);

DWORD
WINAPI
ResUtilResourceDepEnum(
    IN HRESOURCE                hSelf,
    IN DWORD                    enumType,
    IN LPRESOURCE_CALLBACK_EX   pResCallBack,
    IN PVOID                    pParameter
    );
DWORD
WINAPI
ResUtilDupResource(IN HRESOURCE group, OUT HRESOURCE *copy);

DWORD
WINAPI
ResUtilGetClusterId( IN HCLUSTER hCluster,OUT GUID *guid );


DWORD
WINAPI
ResUtilNodeEnum(
    IN HCLUSTER             hCluster,
    IN LPNODE_CALLBACK      pNodeCallBack,
    IN PVOID                pParameter
    );

#ifdef __cplusplus
}
#endif


#endif // ifdef _RESAPI_DEFINES_


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_CLUSTER) */
#pragma endregion
