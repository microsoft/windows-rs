/*++

Copyright (c) 2009 Microsoft Corporation

Module Name:

    sas.h

Abstract:

    This header defines SendSAS().

Author:

    felixk 3/8/2009

--*/

#pragma once
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#ifdef __cplusplus
extern "C"
{
#endif

VOID
WINAPI
SendSAS(
    _In_ BOOL AsUser );

#ifdef __cplusplus
}
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

