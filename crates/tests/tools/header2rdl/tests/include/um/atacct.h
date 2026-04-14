
/*++

Copyright (c) 2002  Microsoft Corporation

Module Name:

    atacct.h

Abstract:

    Prototypes for the AT Service Account API.

Revision History:

--*/

#ifndef _ATACCT_H_
#define _ATACCT_H_
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#ifdef __cplusplus
extern "C" {
#endif

STDAPI
GetNetScheduleAccountInformation(
    IN    LPCWSTR pwszServerName,
    OUT   DWORD ccAccount,
    OUT _Out_writes_(ccAccount)  WCHAR wszAccount[]
    );

STDAPI
SetNetScheduleAccountInformation(
    IN    LPCWSTR pwszServerName,
    IN    LPCWSTR pwszAccount,
    IN    LPCWSTR pwszPassword
    );

#ifdef __cplusplus
}
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // _ATACCT_H_
