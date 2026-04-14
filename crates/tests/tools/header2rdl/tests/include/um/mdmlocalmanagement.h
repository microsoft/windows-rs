/*++ 

Copyright (c) Microsoft Corporation

Notes:

--*/

#if (NTDDI_VERSION >= NTDDI_WIN10_RS2)

#ifndef _MDM_LOCAL_MANAGEMENT_
#define _MDM_LOCAL_MANAGEMENT_

#ifdef _MSC_VER
#pragma once
#endif

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

/*++

Routine Description:

    This API is used to register a device with Local MDM Management synchronously.
    If a device is already registered, out parameter alreadyRegister is set to 
    TRUE and function returns S_OK. In all other cases, out parameter alreadyRegistered
    is set to FALSE.

Return Value:

    HRESULT indicating success or failure.
--*/
STDAPI
RegisterDeviceWithLocalManagement(
    _Out_opt_ BOOL* alreadyRegistered);

/*++

Routine Description:

    This function is used to execute a SyncML. 
    The device must invoke RegisterDeviceWithLocalManagement prior to calling this function.

Arguments:

    syncMLRequest - Null terminated string containing SyncML request.

    syncMLResult - Null terminated string containing SyncML result. 
                       Caller is responsible releasing memory allocated with LocalFree.

Return Value:

    HRESULT indicating success or failure.
--*/
STDAPI
ApplyLocalManagementSyncML(
    _In_                          PCWSTR syncMLRequest,
    _Outptr_opt_result_maybenull_ PWSTR* syncMLResult
    );

/*++

Routine Description:

    This function is used to unregister a device with Local MDM Management synchronously.

Return Value:

    HRESULT indicating success or failure.
--*/
STDAPI
UnregisterDeviceWithLocalManagement();

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // _MDM_LOCAL_MANAGEMENT_

#endif // NTDDI_VERSION >= NTDDI_WIN10_RS2