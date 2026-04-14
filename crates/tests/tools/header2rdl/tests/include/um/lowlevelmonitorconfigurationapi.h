//
//    Copyright (C) Microsoft.  All rights reserved.
//
#ifndef LowLevelMonitorConfigurationAPI_h
#define LowLevelMonitorConfigurationAPI_h

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#include <PhysicalMonitorEnumerationAPI.h>

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus 

#pragma pack( push, 1  )

typedef struct _MC_TIMING_REPORT
{
    DWORD dwHorizontalFrequencyInHZ;
    DWORD dwVerticalFrequencyInHZ;
    BYTE bTimingStatusByte;

} MC_TIMING_REPORT, *LPMC_TIMING_REPORT;

typedef enum _MC_VCP_CODE_TYPE
{
    MC_MOMENTARY,
    MC_SET_PARAMETER

} MC_VCP_CODE_TYPE, *LPMC_VCP_CODE_TYPE;

_BOOL WINAPI GetVCPFeatureAndVCPFeatureReply
    (
    HANDLE hMonitor,
    BYTE bVCPCode,
    _Out_opt_ LPMC_VCP_CODE_TYPE pvct,
    _Out_ LPDWORD pdwCurrentValue, 
    _Out_opt_ LPDWORD pdwMaximumValue
    );

_BOOL WINAPI SetVCPFeature( HANDLE hMonitor, BYTE bVCPCode, DWORD dwNewValue );

_BOOL WINAPI SaveCurrentSettings( HANDLE hMonitor );

_BOOL WINAPI GetCapabilitiesStringLength( HANDLE hMonitor, _Out_ LPDWORD pdwCapabilitiesStringLengthInCharacters );

_BOOL WINAPI CapabilitiesRequestAndCapabilitiesReply 
    (
    HANDLE hMonitor, 
    _Out_writes_(dwCapabilitiesStringLengthInCharacters) LPSTR pszASCIICapabilitiesString, 
    DWORD dwCapabilitiesStringLengthInCharacters
    );

_BOOL WINAPI GetTimingReport( HANDLE hMonitor, _Out_ LPMC_TIMING_REPORT pmtrMonitorTimingReport );

#pragma pack( pop )

#ifdef __cplusplus
}
#endif // __cplusplus 


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // LowLevelMonitorConfigurationAPI_h
