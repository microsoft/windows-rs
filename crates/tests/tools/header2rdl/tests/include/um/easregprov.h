/*++ 

Copyright (c) Microsoft Corporation

Module Name:

    easregprov.h

Abstract:

    This file contains the function which a 3rd Party Encryption software installation 
    package calls to register & unregister their provider that supports EAS.

Environment:

    User Mode - Win32

Notes:

--*/

//
// User Function
//

#if NTDDI_VERSION >= NTDDI_WINBLUE 

#ifndef _EASREGPROV_
#define _EASREGPROV_

#ifdef _MSC_VER
#pragma once
#endif

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#ifdef __cplusplus
extern "C" {
#endif

HRESULT
WINAPI
EasRegisterEncryptionProvider(
    _In_  LPCWSTR  pcwzDllPath
    );

HRESULT
WINAPI
EasUnRegisterEncryptionProvider(
    );

#ifdef __cplusplus
}
#endif

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#pragma endregion

#endif // _EASREGPROV_

#endif // NTDDI_VERSION >= NTDDI_WINBLUE 