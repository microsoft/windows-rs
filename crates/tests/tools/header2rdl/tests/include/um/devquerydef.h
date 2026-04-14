/*++

Copyright (c) Microsoft Corporation

Abstract:

    This module contains the Device Object API shared definitions.

*/

#pragma once

#include <winapifamily.h>

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

#include <devfiltertypes.h>

#if (NTDDI_VERSION >= NTDDI_WIN8)

#if defined(__cplusplus)
extern "C" {
#endif // defined(__cplusplus)

typedef 
#ifdef MIDL_PASS
[v1_enum] 
#endif
enum 
_DEV_OBJECT_TYPE
{
    DevObjectTypeUnknown,
    DevObjectTypeDeviceInterface,
    DevObjectTypeDeviceContainer,
    DevObjectTypeDevice,
    DevObjectTypeDeviceInterfaceClass,
    DevObjectTypeAEP,
    DevObjectTypeAEPContainer,
    DevObjectTypeDeviceInstallerClass,
    DevObjectTypeDeviceInterfaceDisplay,
    DevObjectTypeDeviceContainerDisplay,
    DevObjectTypeAEPService,
    DevObjectTypeDevicePanel,
    DevObjectTypeAEPProtocol,
} DEV_OBJECT_TYPE, *PDEV_OBJECT_TYPE;

typedef 
#ifdef MIDL_PASS
[v1_enum] 
#endif
enum _DEV_QUERY_FLAGS
{
    DevQueryFlagNone = 0x0,
    DevQueryFlagUpdateResults = 0x1,
    DevQueryFlagAllProperties = 0x2,
    DevQueryFlagLocalize = 0x4,
    DevQueryFlagAsyncClose = 0x8
} DEV_QUERY_FLAGS, *PDEV_QUERY_FLAGS;

typedef
#ifdef MIDL_PASS
[v1_enum] 
#endif
enum _DEV_QUERY_STATE
{
    DevQueryStateInitialized,
    DevQueryStateEnumCompleted,
    DevQueryStateAborted,
    DevQueryStateClosed
} DEV_QUERY_STATE, *PDEV_QUERY_STATE;

typedef 
#ifdef MIDL_PASS
[v1_enum] 
#endif
enum _DEV_QUERY_RESULT_ACTION
{
    DevQueryResultStateChange,
    DevQueryResultAdd,
    DevQueryResultUpdate,
    DevQueryResultRemove
} DEV_QUERY_RESULT_ACTION, *PDEV_QUERY_RESULT_ACTION;

typedef struct _DEV_OBJECT
{
    DEV_OBJECT_TYPE ObjectType;
    __in PCWSTR pszObjectId;
#ifdef MIDL_PASS
    [range(0, 10000)]
#endif
    __in ULONG cPropertyCount;
#ifdef MIDL_PASS
    [size_is(cPropertyCount)]
#endif
    __field_ecount_opt(cPropertyCount) const DEVPROPERTY *pProperties;
} DEV_OBJECT, *PDEV_OBJECT;

typedef struct _DEV_QUERY_RESULT_ACTION_DATA
{
    DEV_QUERY_RESULT_ACTION Action;
    
#ifdef MIDL_PASS
    [switch_is(Action)]
    [switch_type(DEV_QUERY_RESULT_ACTION)]
#endif 
    union _DEV_QUERY_RESULT_UPDATE_PAYLOAD
    {
#ifdef MIDL_PASS
        [case (DevQueryResultStateChange)]
#endif 
        DEV_QUERY_STATE State;
#ifdef MIDL_PASS
        [case (DevQueryResultAdd, DevQueryResultUpdate, DevQueryResultRemove)]
#endif 
        DEV_OBJECT DeviceObject;
    } Data;
} DEV_QUERY_RESULT_ACTION_DATA, *PDEV_QUERY_RESULT_ACTION_DATA;

#if (NTDDI_VERSION >= NTDDI_WINBLUE)

typedef struct _DEV_QUERY_PARAMETER {
    DEVPROPKEY Key;
    DEVPROPTYPE Type;
    ULONG BufferSize;
#ifdef MIDL_PASS
    [size_is(BufferSize)] PBYTE Buffer;
#else
    __field_bcount_opt(BufferSize) PVOID Buffer;
#endif
} DEV_QUERY_PARAMETER, *PDEV_QUERY_PARAMETER;

#endif

#if defined(__cplusplus)
}
#endif // defined(__cplusplus)

#endif // NTDDI_VERSION
 
#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)
