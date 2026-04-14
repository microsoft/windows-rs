/******************************************************************************
*                                                                             *
* ual.h - Interface for the Windows User Access Logging API                   *
*                                                                             *
* Copyright (c) Microsoft Corporation.    All Rights Reserved                 *
*                                                                             *
******************************************************************************/
#ifndef _UAL_H_INCLUDED
#define _UAL_H_INCLUDED

#include <winapifamily.h>
#include <winsock2.h>

#if defined (_MSC_VER)
#pragma once
#endif

#ifdef __cplusplus
extern "C" {
#endif /* __cplusplus */

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#if NTDDI_VERSION >= NTDDI_WIN8

typedef struct tagUAL_DATA_BLOB
{
    DWORD               Size;
    GUID                RoleGuid;
    GUID                TenantId;
    SOCKADDR_STORAGE    Address;
    WCHAR               UserName[MAX_PATH];
} UAL_DATA_BLOB, *PUAL_DATA_BLOB;

HRESULT
WINAPI
UalStart(
    _In_ PUAL_DATA_BLOB Data);

HRESULT
WINAPI
UalStop(
    _In_ PUAL_DATA_BLOB Data);
    
HRESULT
WINAPI
UalInstrument(
    _In_ PUAL_DATA_BLOB Data);
    
HRESULT
WINAPI
UalRegisterProduct(
    _In_z_ const WCHAR* wszProductName,
    _In_z_ const WCHAR* wszRoleName,
    _In_z_ const WCHAR* wszGuid);

#endif /* NTDDI_VERSION >= NTDDI_WIN8 */

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#ifdef __cplusplus
}
#endif  /* __cplusplus */

#endif /* !_UAL_H_INCLUDED */

