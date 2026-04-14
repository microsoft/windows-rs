//+-------------------------------------------------------------------
//
//  Microsoft Windows
//  Copyright (C) Microsoft Corporation, 1993-1999.
//
//  File:        msports.h
//
//  Contents:    public header file for COM name arbitration database
//               and Advanced dialog override and invocation
//
//--------------------------------------------------------------------

#ifndef _MSPORTS_H
#define _MSPORTS_H

#if _MSC_VER > 1000
#pragma once
#endif
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#ifdef __cplusplus
extern "C" {
#endif

#ifdef SERIAL_ADVANCED_SETTINGS

/*++

Routine Description:

    Displays the advanced properties dialog for the COM port specified by
    DeviceInfoSet and DeviceInfoData.

Arguments:

    ParentHwnd  - the parent window of the window to be displayed

    DeviceInfoSet, DeviceInfoData - SetupDi structures representing the COM port

Return Value:

    ERROR_SUCCESS if the dialog was shown

  --*/
LONG
SerialDisplayAdvancedSettings(_In_ HWND             ParentHwnd,
                              _In_ HDEVINFO         DeviceInfoSet,
                              _In_ PSP_DEVINFO_DATA DeviceInfoData
                              );

/*++

Routine Description:

    Prototype to allow serial port vendors to override the advanced dialog
    represented by the COM port specified by DeviceInfoSet and DeviceInfoData.

    To override the advanced page, place a value named EnumAdvancedDialog under
    the same key in which you would put your EnumPropPages32 value.  The format
    of the value is exactly the same as Enum...32 as well.

Arguments:

    ParentHwnd  - the parent window of the window to be displayed

    HidePollingUI - If TRUE, hide all UI that deals with polling.

    DeviceInfoSet, DeviceInfoData - SetupDi structures representing the COM port

    Reserved - Unused

Return Value:

    TRUE if the user pressed OK, FALSE if Cancel was pressed
--*/

typedef
BOOL
(*PPORT_ADVANCED_DIALOG) (
    _In_ HWND             ParentHwnd,
    _In_ BOOL             HidePollingUI,
    _In_ HDEVINFO         DeviceInfoSet,
    _In_ PSP_DEVINFO_DATA DeviceInfoData,
    _In_opt_ PVOID        Reserved
    );

#endif

DECLARE_HANDLE(HCOMDB);
typedef HCOMDB *PHCOMDB;
#define HCOMDB_INVALID_HANDLE_VALUE ((HCOMDB) INVALID_HANDLE_VALUE)

//
// Minimum through maximum number of COM names arbitered
//
#define COMDB_MIN_PORTS_ARBITRATED 256
#define COMDB_MAX_PORTS_ARBITRATED 4096

LONG
WINAPI
ComDBOpen (
    _Out_ PHCOMDB PHComDB
    );
/*++

Routine Description:

    Opens name data base, and returns a handle to be used in future calls.

Arguments:

    None.

Return Value:

    INVALID_HANDLE_VALUE if the call fails, otherwise a valid handle

    If INVALID_HANDLE_VALUE, call GetLastError() to get details (??)

--*/

LONG
WINAPI
ComDBClose (
    _In_ HCOMDB HComDB
    );
/*++

Routine Description:

    frees a handle to the database returned from OpenComPortDataBase

Arguments:

    Handle returned from OpenComPortDataBase.

Return Value:

    None

--*/

#define CDB_REPORT_BITS      0x0
#define CDB_REPORT_BYTES     0x1

LONG
WINAPI
ComDBGetCurrentPortUsage (
    _In_        HCOMDB   HComDB,
    _Out_writes_bytes_opt_(BufferSize) PBYTE Buffer,
    _In_        DWORD    BufferSize,
    _In_        ULONG    ReportType, // CDB_REPORT value
    _Out_opt_   LPDWORD  MaxPortsReported
    );
/*++

Routine Description:

    if Buffer is NULL, than MaxPortsReported will contain the max number of ports
        the DB will report (this value is NOT the number of bytes need for Buffer).
        ReportType is ignored in this case.

    if ReportType == CDB_REPORT_BITS
        returns a bit array indicating if a comX name is claimed.
        ie, Bit 0 of Byte 0 is com1, bit 1 of byte 0 is com2 and so on.

        BufferSize >= MaxPortsReported / 8


    if ReportType == CDB_REPORT_BYTES
        returns a byte array indicating if a comX name is claimed.  Zero unused, non zero
        used, ie, byte 0 is com1, byte 1 is com2, etc

        BufferSize >= MaxPortsReported

Arguments:

    Handle returned from OpenComPortDataBase.

    Buffer pointes to memory to place bit array

    BufferSize   Size of buffer in bytes

    MaxPortsReported    Pointer to DWORD that holds the number of bytes in buffer filled in

Return Value:

    returns ERROR_SUCCESS if successful.
            ERROR_NOT_CONNECTED cannot connect to DB
            ERROR_MORE_DATA if buffer not large enough

--*/


LONG
WINAPI
ComDBClaimNextFreePort (
    _In_ HCOMDB   HComDB,
    _Out_ LPDWORD ComNumber
    );
/*++

Routine Description:

    returns the first free COMx value

Arguments:

    Handle returned from OpenComPortDataBase.

Return Value:


    returns ERROR_SUCCESS if successful. or other ERROR_ if not

    if successful, then ComNumber will be that next free com value and claims it in the database


--*/



LONG
WINAPI
ComDBClaimPort (
    _In_        HCOMDB   HComDB,
    _In_        DWORD    ComNumber,
    _In_        BOOL     ForceClaim,
    _Out_opt_   PBOOL    Forced
    );
/*++

Routine Description:

    Attempts to claim a com name in the database

Arguments:

    DataBaseHandle - returned from OpenComPortDataBase.

    ComNumber      - The port value to be claimed

    Force          - If TRUE, will force the port to be claimed even if in use already

    Forced         - will reflect the event that the claim was forced

Return Value:


    returns ERROR_SUCCESS if port name was not already claimed, or if it was claimed
                          and Force was TRUE.

            ERROR_SHARING_VIOLATION if port name is use and Force is false


--*/

LONG
WINAPI
ComDBReleasePort (
    _In_ HCOMDB   HComDB,
    _In_ DWORD    ComNumber
    );
/*++

Routine Description:

    Releases the port in the database

Arguments:

    DatabaseHandle - returned from OpenComPortDataBase.

    ComNumber      - port to be unclaimed in database

Return Value:


    returns ERROR_SUCCESS if successful
            ERROR_CANTWRITE if the changes cannot be committed
            ERROR_INVALID_PARAMETER if ComNumber is greater than the number of
                                    ports arbitrated


--*/

LONG
WINAPI
ComDBResizeDatabase (
    _In_ HCOMDB   HComDB,
    _In_ DWORD    NewSize
    );
/*++

Routine Description:

    Resizes the database to the new size.  To get the current size, call
    ComDBGetCurrentPortUsage with a Buffer == NULL.

Arguments:

    DatabaseHandle - returned from OpenComPortDataBase.

    NewSize        - must be a multiple of 1024, with a max of 4096

Return Value:


    returns ERROR_SUCCESS if successful
            ERROR_CANTWRITE if the changes cannot be committed
            ERROR_BAD_LENGTH if NewSize is not greater than the current size or
                             NewSize is greater than COMDB_MAX_PORTS_ARBITRATED

--*/


#ifdef __cplusplus
}
#endif



#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // _MSPORTS_H
