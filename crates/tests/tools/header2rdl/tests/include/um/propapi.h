#include <winapifamily.h>

//+-------------------------------------------------------------------------
//
//  Microsoft Windows
//  Copyright (C) Microsoft Corporation, 1992-2006.
//
//  File:       propapi.h
//
//  Contents:   Structured storage properties APIs
//
//--------------------------------------------------------------------------


#ifndef _PROPAPI_H_
#define _PROPAPI_H_

#pragma once

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#ifdef __cplusplus
extern "C" {
#endif

typedef VOID* NTPROP;

#include <propidl.h>

EXTERN_C
_Success_(TRUE)  /* Raises status on failure */
ULONG __stdcall
StgPropertyLengthAsVariant(
            _In_reads_bytes_(cbProp) const SERIALIZEDPROPERTYVALUE* pProp,
            _In_ ULONG cbProp,
            _In_ USHORT CodePage,
            _Reserved_ BYTE bReserved);

#ifdef __cplusplus
}
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // ifndef _PROPAPI_H_
