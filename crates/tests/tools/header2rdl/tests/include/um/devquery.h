
/*++

Copyright (c) Microsoft Corporation

Abstract:

    This module contains the Device Object API definitions
    that are used by client applications.

*/

#ifndef _DEVQUERY_H_
#define _DEVQUERY_H_

#include <winapifamily.h>

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

#include <devquerydef.h>

#if (NTDDI_VERSION >= NTDDI_WIN8)

#if defined(__cplusplus)
extern "C" {
#endif // defined(__cplusplus)

DECLARE_HANDLE(HDEVQUERY);
typedef HDEVQUERY *PHDEVQUERY;

typedef VOID (WINAPI *PDEV_QUERY_RESULT_CALLBACK)(
    _In_ HDEVQUERY hDevQuery,
    _In_opt_ PVOID pContext,
    _In_ const DEV_QUERY_RESULT_ACTION_DATA *pActionData
);

_Check_return_ HRESULT WINAPI
DevCreateObjectQuery(
    _In_ DEV_OBJECT_TYPE ObjectType,
    _In_ ULONG QueryFlags,
    _In_ ULONG cRequestedProperties,
    _In_reads_opt_(cRequestedProperties) const DEVPROPCOMPKEY *pRequestedProperties,
    _In_ ULONG cFilterExpressionCount,
    _In_reads_opt_(cFilterExpressionCount) const DEVPROP_FILTER_EXPRESSION *pFilter,
    _In_ PDEV_QUERY_RESULT_CALLBACK pCallback,
    _In_opt_ PVOID pContext,
    _Out_ PHDEVQUERY phDevQuery);

#if (NTDDI_VERSION >= NTDDI_WINBLUE)

_Check_return_ HRESULT WINAPI
DevCreateObjectQueryEx(
    _In_ DEV_OBJECT_TYPE ObjectType,
    _In_ ULONG QueryFlags,
    _In_ ULONG cRequestedProperties,
    _In_reads_opt_(cRequestedProperties) const DEVPROPCOMPKEY *pRequestedProperties,
    _In_ ULONG cFilterExpressionCount,
    _In_reads_opt_(cFilterExpressionCount) const DEVPROP_FILTER_EXPRESSION *pFilter,
    _In_ ULONG cExtendedParameterCount,
    _In_reads_opt_(cExtendedParameterCount) const DEV_QUERY_PARAMETER *pExtendedParameters,
    _In_ PDEV_QUERY_RESULT_CALLBACK pCallback,
    _In_opt_ PVOID pContext,
    _Out_ PHDEVQUERY phDevQuery);

#endif

_Check_return_ HRESULT WINAPI
DevCreateObjectQueryFromId(
    _In_ DEV_OBJECT_TYPE ObjectType,
    _In_ PCWSTR pszObjectId,
    _In_ ULONG QueryFlags,
    _In_ ULONG cRequestedProperties,
    _In_reads_opt_(cRequestedProperties) const DEVPROPCOMPKEY *pRequestedProperties,
    _In_ ULONG cFilterExpressionCount,
    _In_reads_opt_(cFilterExpressionCount) const DEVPROP_FILTER_EXPRESSION *pFilter,
    _In_ PDEV_QUERY_RESULT_CALLBACK pCallback,
    _In_opt_ PVOID pContext,
    _Out_ PHDEVQUERY phDevQuery);

#if (NTDDI_VERSION >= NTDDI_WINBLUE)

_Check_return_ HRESULT WINAPI
DevCreateObjectQueryFromIdEx(
    _In_ DEV_OBJECT_TYPE ObjectType,
    _In_ PCWSTR pszObjectId,
    _In_ ULONG QueryFlags,
    _In_ ULONG cRequestedProperties,
    _In_reads_opt_(cRequestedProperties) const DEVPROPCOMPKEY *pRequestedProperties,
    _In_ ULONG cFilterExpressionCount,
    _In_reads_opt_(cFilterExpressionCount) const DEVPROP_FILTER_EXPRESSION *pFilter,
    _In_ ULONG cExtendedParameterCount,
    _In_reads_opt_(cExtendedParameterCount) const DEV_QUERY_PARAMETER *pExtendedParameters,
    _In_ PDEV_QUERY_RESULT_CALLBACK pCallback,
    _In_opt_ PVOID pContext,
    _Out_ PHDEVQUERY phDevQuery);

#endif

_Check_return_ HRESULT WINAPI
DevCreateObjectQueryFromIds(
    _In_ DEV_OBJECT_TYPE ObjectType,
    _In_ PCZZWSTR pszzObjectIds,
    _In_ ULONG QueryFlags,
    _In_ ULONG cRequestedProperties,
    _In_reads_opt_(cRequestedProperties) const DEVPROPCOMPKEY *pRequestedProperties,
    _In_ ULONG cFilterExpressionCount,
    _In_reads_opt_(cFilterExpressionCount) const DEVPROP_FILTER_EXPRESSION *pFilter,
    _In_ PDEV_QUERY_RESULT_CALLBACK pCallback,
    _In_opt_ PVOID pContext,
    _Out_ PHDEVQUERY phDevQuery);

#if (NTDDI_VERSION >= NTDDI_WINBLUE)

_Check_return_ HRESULT WINAPI
DevCreateObjectQueryFromIdsEx(
    _In_ DEV_OBJECT_TYPE ObjectType,
    _In_ PCZZWSTR pszzObjectIds,
    _In_ ULONG QueryFlags,
    _In_ ULONG cRequestedProperties,
    _In_reads_opt_(cRequestedProperties) const DEVPROPCOMPKEY *pRequestedProperties,
    _In_ ULONG cFilterExpressionCount,
    _In_reads_opt_(cFilterExpressionCount) const DEVPROP_FILTER_EXPRESSION *pFilter,
    _In_ ULONG cExtendedParameterCount,
    _In_reads_opt_(cExtendedParameterCount) const DEV_QUERY_PARAMETER *pExtendedParameters,
    _In_ PDEV_QUERY_RESULT_CALLBACK pCallback,
    _In_opt_ PVOID pContext,
    _Out_ PHDEVQUERY phDevQuery);

#endif

VOID WINAPI
DevCloseObjectQuery(
    _In_ HDEVQUERY hDevQuery);

_Check_return_ HRESULT WINAPI
DevGetObjects(
    _In_ DEV_OBJECT_TYPE ObjectType,
    _In_ ULONG QueryFlags,
    _In_ ULONG cRequestedProperties,
    _In_reads_opt_(cRequestedProperties) const DEVPROPCOMPKEY *pRequestedProperties,
    _In_ ULONG cFilterExpressionCount,
    _In_reads_opt_(cFilterExpressionCount) const DEVPROP_FILTER_EXPRESSION *pFilter,
    _Out_ PULONG pcObjectCount,
    _Outptr_result_buffer_maybenull_(*pcObjectCount) const DEV_OBJECT **ppObjects);

#if (NTDDI_VERSION >= NTDDI_WINBLUE)

_Check_return_ HRESULT WINAPI
DevGetObjectsEx(
    _In_ DEV_OBJECT_TYPE ObjectType,
    _In_ ULONG QueryFlags,
    _In_ ULONG cRequestedProperties,
    _In_reads_opt_(cRequestedProperties) const DEVPROPCOMPKEY *pRequestedProperties,
    _In_ ULONG cFilterExpressionCount,
    _In_reads_opt_(cFilterExpressionCount) const DEVPROP_FILTER_EXPRESSION *pFilter,
    _In_ ULONG cExtendedParameterCount,
    _In_reads_opt_(cExtendedParameterCount) const DEV_QUERY_PARAMETER *pExtendedParameters,
    _Out_ PULONG pcObjectCount,
    _Outptr_result_buffer_maybenull_(*pcObjectCount) const DEV_OBJECT **ppObjects);

#endif

VOID WINAPI
DevFreeObjects(
    _In_ ULONG cObjectCount,
    _In_reads_(cObjectCount) const DEV_OBJECT *pObjects);

_Check_return_ HRESULT WINAPI
DevGetObjectProperties(
    _In_ DEV_OBJECT_TYPE ObjectType,
    _In_ PCWSTR pszObjectId,
    _In_ ULONG QueryFlags,
    _In_ ULONG cRequestedProperties,
    _In_reads_(cRequestedProperties) const DEVPROPCOMPKEY *pRequestedProperties,
    _Out_ PULONG pcPropertyCount,
    _Outptr_result_buffer_(*pcPropertyCount) const DEVPROPERTY **ppProperties);

#if (NTDDI_VERSION >= NTDDI_WINBLUE)

_Check_return_ HRESULT WINAPI
DevGetObjectPropertiesEx(
    _In_ DEV_OBJECT_TYPE ObjectType,
    _In_ PCWSTR pszObjectId,
    _In_ ULONG QueryFlags,
    _In_ ULONG cRequestedProperties,
    _In_reads_(cRequestedProperties) const DEVPROPCOMPKEY *pRequestedProperties,
    _In_ ULONG cExtendedParameterCount,
    _In_reads_opt_(cExtendedParameterCount) const DEV_QUERY_PARAMETER *pExtendedParameters,
    _Out_ PULONG pcPropertyCount,
    _Outptr_result_buffer_(*pcPropertyCount) const DEVPROPERTY **ppProperties);

#endif

VOID WINAPI
DevFreeObjectProperties(
    _In_ ULONG cPropertyCount,
    _In_reads_(cPropertyCount) const DEVPROPERTY *pProperties);

_Check_return_ const DEVPROPERTY* WINAPI
DevFindProperty(
    _In_ const DEVPROPKEY *pKey,
    _In_ DEVPROPSTORE Store,
    _In_ PCWSTR pszLocaleName,
    _In_ ULONG cProperties,
    _In_reads_opt_(cProperties) const DEVPROPERTY *pProperties);

#if defined(__cplusplus)
}
#endif // defined(__cplusplus)

#endif // NTDDI_VERSION
#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)
#endif // _DEVQUERY_H_

