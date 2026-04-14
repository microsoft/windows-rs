//
//    Copyright (C) Microsoft.  All rights reserved.
//
#ifndef PhysicalMonitorEnumerationAPI_h
#define PhysicalMonitorEnumerationAPI_h

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#include <d3d9.h>

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus 

#pragma pack( push, 1  )    

/******************************************************************************
    Physical Monitor Type Definitions
******************************************************************************/

typedef _Return_type_success_(return == TRUE) BOOL _BOOL;

/******************************************************************************
    Physical Monitor Constants
******************************************************************************/

// A physical monitor description is always an array of 128 characters.  Some
// of the characters may not be used.
#define PHYSICAL_MONITOR_DESCRIPTION_SIZE                   128

/******************************************************************************
    Physical Monitor Structures 
******************************************************************************/
typedef struct _PHYSICAL_MONITOR
{
    HANDLE hPhysicalMonitor;
    WCHAR szPhysicalMonitorDescription[PHYSICAL_MONITOR_DESCRIPTION_SIZE];
} PHYSICAL_MONITOR, *LPPHYSICAL_MONITOR;

/******************************************************************************
    Physical Monitor Enumeration Functions
******************************************************************************/
_BOOL WINAPI GetNumberOfPhysicalMonitorsFromHMONITOR
    ( 
    HMONITOR hMonitor, 
    _Out_ LPDWORD pdwNumberOfPhysicalMonitors
    );
HRESULT WINAPI GetNumberOfPhysicalMonitorsFromIDirect3DDevice9
    (
    _In_ IDirect3DDevice9* pDirect3DDevice9,
    _Out_ LPDWORD pdwNumberOfPhysicalMonitors
    );

_BOOL WINAPI GetPhysicalMonitorsFromHMONITOR
    ( 
    _In_ HMONITOR hMonitor,
    _In_ DWORD dwPhysicalMonitorArraySize,
    _Out_writes_(dwPhysicalMonitorArraySize) LPPHYSICAL_MONITOR pPhysicalMonitorArray
    );
HRESULT WINAPI GetPhysicalMonitorsFromIDirect3DDevice9
    ( 
    _In_ IDirect3DDevice9* pDirect3DDevice9,
    _In_ DWORD dwPhysicalMonitorArraySize,
    _Out_writes_(dwPhysicalMonitorArraySize) LPPHYSICAL_MONITOR pPhysicalMonitorArray
    );

_BOOL WINAPI DestroyPhysicalMonitor( _In_ HANDLE hMonitor );
_BOOL WINAPI DestroyPhysicalMonitors
    ( 
    _In_ DWORD dwPhysicalMonitorArraySize,
    _In_reads_(dwPhysicalMonitorArraySize) LPPHYSICAL_MONITOR pPhysicalMonitorArray
    );

#pragma pack( pop )

#ifdef __cplusplus
}
#endif // __cplusplus 


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // PhysicalMonitorEnumerationAPI_h
