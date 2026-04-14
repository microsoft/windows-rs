/*++

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    hwreqchkapi.h

Abstract:

    The header that defines the interface used by the HWREQCHK APIs

--*/

#ifndef _hwreqchkapi_h_
#define _hwreqchkapi_h_

#ifdef _MSC_VER
#pragma once
#endif  // _MSC_VER

#include <wtypes.h>
#include <wldp.h>

#if defined(__cplusplus)
extern "C" {
#endif // defined(__cplusplus)

#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

// The maximum supported size for property, requirement and rule names
#define HWREQCHK_MAX_PROPERTY_VALUE              256
#define HWREQCHK_MAX_HARDWARE_REQUIREMENT_NAME   64
#define HWREQCHK_MAX_HARDWARE_RULE_NAME          64

typedef enum HWREQCHK_TARGET_RELEASE
{
    HWREQCHK_TARGET_RELEASE_INVALID,
    HWREQCHK_TARGET_RELEASE_WIN11_VNEXT, // For newer versions of Windows not yet released
    HWREQCHK_TARGET_RELEASE_WIN11_21H2,
    HWREQCHK_TARGET_RELEASE_WIN11_22H2,
    HWREQCHK_TARGET_RELEASE_SRV2022_21H2
} HWREQCHK_TARGET_RELEASE;

typedef enum HWREQCHK_PRODUCT_TYPE
{
    HWREQCHK_PRODUCT_INVALID,
    HWREQCHK_PRODUCT_DEFAULT,
    HWREQCHK_PRODUCT_DESKTOP,
    HWREQCHK_PRODUCT_SERVER,
    HWREQCHK_PRODUCT_IOTENTERPRISE,
    HWREQCHK_PRODUCT_IOT,
    HWREQCHK_PRODUCT_IOTENTERPRISE_LTSC
} HWREQCHK_PRODUCT_TYPE;

typedef enum HWREQCHK_CPU_VENDOR
{
    HWREQCHK_CPU_VENDOR_OTHER                     = 0,
    HWREQCHK_CPU_VENDOR_AMD                       = 1,
    HWREQCHK_CPU_VENDOR_INTEL                     = 2,
    HWREQCHK_CPU_VENDOR_QUALCOMM                  = 3
} HWREQCHK_CPU_VENDOR;

typedef struct HWREQCHK_DEVICE_HARDWARE_EVALUATION
{
    WCHAR RuleName[HWREQCHK_MAX_HARDWARE_RULE_NAME];
    BOOL Succeeded;
} HWREQCHK_DEVICE_HARDWARE_EVALUATION;

typedef struct HWREQCHK_DEVICE_HARDWARE_REQUIREMENT
{
    HWREQCHK_TARGET_RELEASE TargetRelease;
    HWREQCHK_PRODUCT_TYPE ProductType;
    ULONG Revision;
    BOOL IsLatestRequirementForProductType;
    WCHAR RequirementName[HWREQCHK_MAX_HARDWARE_REQUIREMENT_NAME];
} HWREQCHK_DEVICE_HARDWARE_REQUIREMENT;

typedef struct HWREQCHK_DEVICE_HARDWARE_SYSINFO
{
    //
    // Hardware System inventory information
    //

    // Supported processor feature information
    BOOL SSE2ProcessorSupport;
    BOOL NXProcessorSupport;
    BOOL CompareExchange128Support;
    BOOL LahfSahfSupport;
    BOOL PrefetchWSupport;
    BOOL ArmV81ProcessorSupport;

    // UEFI/Firmware security feature information
    BOOL SecureBootCapable;
    ULONG TpmVersion;

    // Device hardware information
    ULONG RamMB;
    ULONG SystemDiskSizeMB;

    // CPU specific information
    ULONG CpuMhz;
    ULONG CpuCoreCount;
    ULONG CpuFamily;
    ULONG CpuModel;
    ULONG CpuStepping;
    ULONG Platform;
    HWREQCHK_CPU_VENDOR CpuVendor;
    ULONG Architecture;
    WCHAR ProcessorName[HWREQCHK_MAX_PROPERTY_VALUE];

    // OS specific information
    BOOL IsServer;
    WLDP_WINDOWS_LOCKDOWN_MODE LockdownMode;
    ULONG ProductOS;
    WCHAR ProductName[HWREQCHK_MAX_PROPERTY_VALUE];
} HWREQCHK_DEVICE_HARDWARE_SYSINFO;

HRESULT
WINAPI
EvaluateHardwareRequirement(
    _In_        const HWREQCHK_DEVICE_HARDWARE_REQUIREMENT* hardwareRequirement,
    _Out_       BOOL* evaluationResult,
    _Out_opt_   HWREQCHK_DEVICE_HARDWARE_EVALUATION** constraintsEvaluated,
    _Out_opt_   ULONG* constraintEvaluationCount);

HRESULT
WINAPI
GetLatestHardwareRequirement(
    _In_    HWREQCHK_PRODUCT_TYPE productType,
    _Out_   HWREQCHK_DEVICE_HARDWARE_REQUIREMENT* deviceHardwareRequirement);

HRESULT
WINAPI
GetHardwareRequirements(
    _Out_   HWREQCHK_DEVICE_HARDWARE_REQUIREMENT** deviceHardwareRequirements,
    _Out_   ULONG* requirementCount);

HRESULT
WINAPI
GetHardwareRequirementSystemInfo(
    _Out_   HWREQCHK_DEVICE_HARDWARE_SYSINFO* deviceHardwareSystemInfo);

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */

#if defined(__cplusplus)
}
#endif // defined(__cplusplus)

#endif // _hwreqchkapi_h_