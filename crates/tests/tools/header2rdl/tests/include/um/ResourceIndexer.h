/*
* Copyright (c) Microsoft Corporation. All rights reserved.
*/

#pragma once
#ifndef RESOURCEINDEXER_H
#define RESOURCEINDEXER_H

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#include <windows.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct
{
    PWSTR name;
    PWSTR value;
} IndexedResourceQualifier;

STDAPI CreateResourceIndexer(
    _In_ PCWSTR projectRoot,
    _In_opt_ PCWSTR extensionDllPath,
    _Outptr_ PVOID* ppResourceIndexer);

void DestroyResourceIndexer(_In_opt_ PVOID resourceIndexer);

STDAPI IndexFilePath(
    _In_ PVOID resourceIndexer,
    _In_ PCWSTR filePath,
    _Outptr_ PWSTR* ppResourceUri,
    _Out_ ULONG* pQualifierCount,
    _Out_writes_(*pQualifierCount) IndexedResourceQualifier** ppQualifiers);

void DestroyIndexedResults(
    _In_opt_ PWSTR resourceUri,
    _In_ ULONG qualifierCount,
    _In_reads_opt_(qualifierCount) IndexedResourceQualifier* qualifiers);

#ifdef __cplusplus
}
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif /* !RESOURCEINDEXER_H */
