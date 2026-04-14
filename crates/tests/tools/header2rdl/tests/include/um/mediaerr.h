//------------------------------------------------------------------------------
// File: MediaErr.h
//
// Desc: Shell error codes
//
// Copyright (c) 1999 - 2001, Microsoft Corporation.  All rights reserved.
//------------------------------------------------------------------------------


#ifndef _MEDIAERR_H_
#define _MEDIAERR_H_
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#define DMO_E_INVALIDSTREAMINDEX (HRESULT)0x80040201
#define DMO_E_INVALIDTYPE        (HRESULT)0x80040202
#define DMO_E_TYPE_NOT_SET       (HRESULT)0x80040203
#define DMO_E_NOTACCEPTING       (HRESULT)0x80040204
#define DMO_E_TYPE_NOT_ACCEPTED  (HRESULT)0x80040205
#define DMO_E_NO_MORE_ITEMS      (HRESULT)0x80040206


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // _MEDIAERR_H_
