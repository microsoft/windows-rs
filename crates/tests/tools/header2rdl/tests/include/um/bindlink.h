/*++
Copyright (c) Microsoft Corporation.  All rights reserved.
Module Name:
    bindlink.h
Abstract:
    This file contains public APIs for the creation and removal of
    bind links.
Environment:
    User mode.
--*/

#ifndef BINDLINK_H
#define BINDLINK_H

#if _MSC_VER > 1000
#pragma once
#endif

#pragma region Desktop Family

#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#if defined(BINDLINK_API_IMPL) || (defined(NTDDI_WIN10_CU) && (NTDDI_VERSION >= NTDDI_WIN10_CU))

typedef enum CREATE_BIND_LINK_FLAGS
{
    CREATE_BIND_LINK_FLAG_NONE         = 0x00000000,
    CREATE_BIND_LINK_FLAG_READ_ONLY    = 0x00000001,
    CREATE_BIND_LINK_FLAG_MERGED       = 0x00000002,
} CREATE_BIND_LINK_FLAGS;
 
DEFINE_ENUM_FLAG_OPERATORS(CREATE_BIND_LINK_FLAGS);

STDAPI
CreateBindLink(
    _In_ PCWSTR virtualPath,
    _In_ PCWSTR backingPath,
    CREATE_BIND_LINK_FLAGS createBindLinkFlags,
    UINT32 exceptionCount,
    _In_reads_opt_(exceptionCount) PCWSTR* const exceptionPaths);

STDAPI
RemoveBindLink(
    _In_ PCWSTR virtualPath);

#pragma endregion

#endif /* #if defined(BINDLINK_API_IMPL) || (defined(NTDDI_WIN10_CU) && (NTDDI_VERSION > = NTDDI_WIN10_CU)) */
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#endif /* #ifndef BINDLINK_H */
