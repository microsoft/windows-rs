/*****************************************************************************\
*                                                                             *
* powrprof.h - - Interface for powrprof.dll, the power policy applicator      *
*                                                                             *
* Version 1.0                                                                 *
*                                                                             *
* Copyright (c) Microsoft Corporation. All rights reserved.                   *
*                                                                             *
\*****************************************************************************/

#if _MSC_VER > 1000
#pragma once
#endif

#ifndef _POWRPROF_H_
#define _POWRPROF_H_

#include <powerbase.h>
#include <powersetting.h>
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#ifdef __cplusplus
extern "C" {
#endif

// Registry storage structures for the GLOBAL_POWER_POLICY data. There are two
// structures, GLOBAL_MACHINE_POWER_POLICY and GLOBAL_USER_POWER_POLICY. the
// GLOBAL_MACHINE_POWER_POLICY stores per machine data for which there is no UI.
// GLOBAL_USER_POWER_POLICY stores the per user data.

typedef struct _GLOBAL_MACHINE_POWER_POLICY{
    ULONG                   Revision;
    SYSTEM_POWER_STATE      LidOpenWakeAc;
    SYSTEM_POWER_STATE      LidOpenWakeDc;
    ULONG                   BroadcastCapacityResolution;
} GLOBAL_MACHINE_POWER_POLICY, *PGLOBAL_MACHINE_POWER_POLICY;

typedef struct _GLOBAL_USER_POWER_POLICY{
    ULONG                   Revision;
    POWER_ACTION_POLICY     PowerButtonAc;
    POWER_ACTION_POLICY     PowerButtonDc;
    POWER_ACTION_POLICY     SleepButtonAc;
    POWER_ACTION_POLICY     SleepButtonDc;
    POWER_ACTION_POLICY     LidCloseAc;
    POWER_ACTION_POLICY     LidCloseDc;
    SYSTEM_POWER_LEVEL      DischargePolicy[NUM_DISCHARGE_POLICIES];
    ULONG                   GlobalFlags;
} GLOBAL_USER_POWER_POLICY, *PGLOBAL_USER_POWER_POLICY;

// Structure to manage global power policies at the user level. This structure
// contains data which is common across all power policy profiles.

typedef struct _GLOBAL_POWER_POLICY{
    GLOBAL_USER_POWER_POLICY    user;
    GLOBAL_MACHINE_POWER_POLICY mach;
} GLOBAL_POWER_POLICY, *PGLOBAL_POWER_POLICY;

// Registry storage structures for the POWER_POLICY data. There are three
// structures, MACHINE_POWER_POLICY, MACHINE_PROCESSOR_POWER_POLICY and
// USER_POWER_POLICY. the MACHINE_POWER_POLICY stores per machine data for
// which there is no UI.  USER_POWER_POLICY stores the per user data.

typedef struct _MACHINE_POWER_POLICY{
    ULONG                   Revision;       // 1

    // meaning of power action "sleep"
    SYSTEM_POWER_STATE      MinSleepAc;
    SYSTEM_POWER_STATE      MinSleepDc;
    SYSTEM_POWER_STATE      ReducedLatencySleepAc;
    SYSTEM_POWER_STATE      ReducedLatencySleepDc;

    // parameters for dozing
    ULONG                   DozeTimeoutAc;
    ULONG                   DozeTimeoutDc;
    ULONG                   DozeS4TimeoutAc;
    ULONG                   DozeS4TimeoutDc;

    // processor policies
    UCHAR                   MinThrottleAc;
    UCHAR                   MinThrottleDc;
    UCHAR                   pad1[2];
    POWER_ACTION_POLICY     OverThrottledAc;
    POWER_ACTION_POLICY     OverThrottledDc;

} MACHINE_POWER_POLICY, *PMACHINE_POWER_POLICY;

//
// deprecated
//

typedef struct _MACHINE_PROCESSOR_POWER_POLICY {
    ULONG                   Revision;       // 1

    PROCESSOR_POWER_POLICY  ProcessorPolicyAc;
    PROCESSOR_POWER_POLICY  ProcessorPolicyDc;

} MACHINE_PROCESSOR_POWER_POLICY, *PMACHINE_PROCESSOR_POWER_POLICY;

typedef struct _USER_POWER_POLICY {
    ULONG                   Revision;       // 1

    // "system idle" detection
    POWER_ACTION_POLICY     IdleAc;
    POWER_ACTION_POLICY     IdleDc;
    ULONG                   IdleTimeoutAc;
    ULONG                   IdleTimeoutDc;
    UCHAR                   IdleSensitivityAc;
    UCHAR                   IdleSensitivityDc;

    // Throttling Policy
    UCHAR                   ThrottlePolicyAc;
    UCHAR                   ThrottlePolicyDc;

    // meaning of power action "sleep"
    SYSTEM_POWER_STATE      MaxSleepAc;
    SYSTEM_POWER_STATE      MaxSleepDc;

    // For future use
    ULONG                   Reserved[2];

    // video policies
    ULONG                   VideoTimeoutAc;
    ULONG                   VideoTimeoutDc;

    // hard disk policies
    ULONG                   SpindownTimeoutAc;
    ULONG                   SpindownTimeoutDc;

    // processor policies
    BOOLEAN                 OptimizeForPowerAc;
    BOOLEAN                 OptimizeForPowerDc;
    UCHAR                   FanThrottleToleranceAc;
    UCHAR                   FanThrottleToleranceDc;
    UCHAR                   ForcedThrottleAc;
    UCHAR                   ForcedThrottleDc;

} USER_POWER_POLICY, *PUSER_POWER_POLICY;

// Structure to manage power policies at the user level. This structure
// contains data which is unique across power policy profiles.

typedef struct _POWER_POLICY {
    USER_POWER_POLICY       user;
    MACHINE_POWER_POLICY    mach;
} POWER_POLICY, *PPOWER_POLICY;

// Constants for GlobalFlags

#define EnableSysTrayBatteryMeter   0x01
#define EnableMultiBatteryDisplay   0x02
#define EnablePasswordLogon         0x04
#define EnableWakeOnRing            0x08
#define EnableVideoDimDisplay       0x10

//
// Power setting attribute flags
//

#define POWER_ATTRIBUTE_HIDE        0x00000001
#define POWER_ATTRIBUTE_SHOW_AOAC   0x00000002

// This constant is passed as a uiID to WritePwrScheme.
#define NEWSCHEME (UINT)-1

// Prototype for EnumPwrSchemes callback proceedures.

typedef
BOOLEAN
CALLBACK
PWRSCHEMESENUMPROC_V1 (
    _In_ UINT Index,
    _In_ DWORD NameSize,
    _In_reads_bytes_(NameSize) LPTSTR Name,
    _In_ DWORD DescriptionSize,
    _In_reads_bytes_(DescriptionSize) LPTSTR Description,
    _In_ PPOWER_POLICY Policy,
    _Inout_opt_ LPARAM Context
    );

typedef
BOOLEAN
CALLBACK
PWRSCHEMESENUMPROC_V2 (
    _In_ UINT Index,
    _In_ DWORD NameSize,
    _In_reads_bytes_(NameSize) LPWSTR Name,
    _In_ DWORD DescriptionSize,
    _In_reads_bytes_(DescriptionSize) LPWSTR Description,
    _In_ PPOWER_POLICY Policy,
    _Inout_opt_ LPARAM Context
    );

#if (NTDDI_VERSION >= NTDDI_VISTA)

typedef PWRSCHEMESENUMPROC_V2 *PWRSCHEMESENUMPROC;

#else

typedef PWRSCHEMESENUMPROC_V1 *PWRSCHEMESENUMPROC;

#endif

// Public function prototypes

STDAPI_(BOOLEAN)
GetPwrDiskSpindownRange(
    _Out_ PUINT puiMax,
    _Out_ PUINT puiMin
    );

STDAPI_(BOOLEAN)
EnumPwrSchemes(
    _In_ PWRSCHEMESENUMPROC lpfn,
    _In_ LPARAM lParam
    );

STDAPI_(BOOLEAN)
ReadGlobalPwrPolicy(
    _In_ PGLOBAL_POWER_POLICY pGlobalPowerPolicy
    );

STDAPI_(BOOLEAN)
ReadPwrScheme(
    _In_ UINT uiID,
    _Out_ PPOWER_POLICY pPowerPolicy
    );

STDAPI_(BOOLEAN)
WritePwrScheme(
    _In_ PUINT puiID,
    _In_ LPCWSTR lpszSchemeName,
    _In_opt_ LPCWSTR lpszDescription,
    _In_ PPOWER_POLICY lpScheme
    );

STDAPI_(BOOLEAN)
WriteGlobalPwrPolicy(
        _In_ PGLOBAL_POWER_POLICY pGlobalPowerPolicy
    );

STDAPI_(BOOLEAN)
DeletePwrScheme(
        _In_ UINT uiID
        );

STDAPI_(BOOLEAN)
GetActivePwrScheme(
        _Out_ PUINT puiID
    );

STDAPI_(BOOLEAN)
SetActivePwrScheme(
    _In_ UINT uiID,
    _In_opt_ PGLOBAL_POWER_POLICY pGlobalPowerPolicy,
    _In_opt_ PPOWER_POLICY pPowerPolicy
    );

STDAPI_(BOOLEAN)
IsPwrSuspendAllowed(
        VOID
        );

STDAPI_(BOOLEAN)
IsPwrHibernateAllowed(
        VOID
        );

STDAPI_(BOOLEAN)
IsPwrShutdownAllowed(
        VOID
        );

STDAPI_(BOOLEAN)
IsAdminOverrideActive(
    _In_ PADMINISTRATOR_POWER_POLICY papp
    );

STDAPI_(BOOLEAN)
SetSuspendState(
    _In_ BOOLEAN bHibernate,
    _In_ BOOLEAN bForce,
    _In_ BOOLEAN bWakeupEventsDisabled
    );

STDAPI_(BOOLEAN)
GetCurrentPowerPolicies(
    _Out_ PGLOBAL_POWER_POLICY pGlobalPowerPolicy,
    _Out_ PPOWER_POLICY pPowerPolicy
    );

STDAPI_(BOOLEAN)
CanUserWritePwrScheme(
        VOID
        );

#if (NTDDI_VERSION >= NTDDI_WINXP)
//
// deprecated.
//
STDAPI_(BOOLEAN)
ReadProcessorPwrScheme(
    _In_ UINT uiID,
    _Out_ PMACHINE_PROCESSOR_POWER_POLICY pMachineProcessorPowerPolicy
    );
#endif

#if (NTDDI_VERSION >= NTDDI_WINXP)
//
// deprecated.
//
STDAPI_(BOOLEAN)
WriteProcessorPwrScheme(
    _In_ UINT uiID,
    _In_ PMACHINE_PROCESSOR_POWER_POLICY pMachineProcessorPowerPolicy
    );
#endif

STDAPI_(BOOLEAN)
ValidatePowerPolicies(
    _Inout_opt_ PGLOBAL_POWER_POLICY pGlobalPowerPolicy,
    _Inout_opt_ PPOWER_POLICY pPowerPolicy
    );

//
// Enum which defines which field inside of a
// power setting is being accessed.
//
typedef enum _POWER_DATA_ACCESSOR {

        //
        // Used by read/write and enumeration engines
        //
        ACCESS_AC_POWER_SETTING_INDEX = 0,
        ACCESS_DC_POWER_SETTING_INDEX,
        ACCESS_FRIENDLY_NAME,
        ACCESS_DESCRIPTION,
        ACCESS_POSSIBLE_POWER_SETTING,
        ACCESS_POSSIBLE_POWER_SETTING_FRIENDLY_NAME,
        ACCESS_POSSIBLE_POWER_SETTING_DESCRIPTION,
        ACCESS_DEFAULT_AC_POWER_SETTING,
        ACCESS_DEFAULT_DC_POWER_SETTING,
        ACCESS_POSSIBLE_VALUE_MIN,
        ACCESS_POSSIBLE_VALUE_MAX,
        ACCESS_POSSIBLE_VALUE_INCREMENT,
        ACCESS_POSSIBLE_VALUE_UNITS,
        ACCESS_ICON_RESOURCE,
        ACCESS_DEFAULT_SECURITY_DESCRIPTOR,
        ACCESS_ATTRIBUTES,

        //
        // Used by enumeration engine.
        //
        ACCESS_SCHEME,
        ACCESS_SUBGROUP,
        ACCESS_INDIVIDUAL_SETTING,

        //
        // Used by access check
        //
        ACCESS_ACTIVE_SCHEME,
        ACCESS_CREATE_SCHEME,

        //
        // Used by override ranges.
        //

        ACCESS_AC_POWER_SETTING_MAX,
        ACCESS_DC_POWER_SETTING_MAX,
        ACCESS_AC_POWER_SETTING_MIN,
        ACCESS_DC_POWER_SETTING_MIN,

        //
        // Used by enumeration engine.
        //

        ACCESS_PROFILE,
        ACCESS_OVERLAY_SCHEME,
        ACCESS_POWER_MODE = ACCESS_OVERLAY_SCHEME,
        ACCESS_ACTIVE_OVERLAY_SCHEME

} POWER_DATA_ACCESSOR, *PPOWER_DATA_ACCESSOR;

//
// =========================================
// Power Scheme APIs
// =========================================
//

#define DEVICE_NOTIFY_CALLBACK 2

typedef
ULONG
CALLBACK
DEVICE_NOTIFY_CALLBACK_ROUTINE (
    _In_opt_ PVOID Context,
    _In_ ULONG Type,
    _In_ PVOID Setting
    );
typedef DEVICE_NOTIFY_CALLBACK_ROUTINE* PDEVICE_NOTIFY_CALLBACK_ROUTINE;

typedef struct _DEVICE_NOTIFY_SUBSCRIBE_PARAMETERS {
    PDEVICE_NOTIFY_CALLBACK_ROUTINE Callback;
    PVOID Context;
} DEVICE_NOTIFY_SUBSCRIBE_PARAMETERS, *PDEVICE_NOTIFY_SUBSCRIBE_PARAMETERS;



#if (NTDDI_VERSION >= NTDDI_WIN7)
STDAPI_(BOOLEAN)
PowerIsSettingRangeDefined (
    _In_opt_ CONST GUID *SubKeyGuid OPTIONAL,
    _In_opt_ CONST GUID *SettingGuid OPTIONAL
    );
#endif


#if (NTDDI_VERSION >= NTDDI_WIN7)
STDAPI_(DWORD)
PowerSettingAccessCheckEx (
    _In_ POWER_DATA_ACCESSOR AccessFlags,
    _In_opt_ CONST GUID *PowerGuid,
    _In_ REGSAM AccessType
    );
#endif


#if (NTDDI_VERSION >= NTDDI_VISTA)
STDAPI_(DWORD)
PowerSettingAccessCheck (
    _In_ POWER_DATA_ACCESSOR AccessFlags,
    _In_opt_ CONST GUID *PowerGuid
    );
#endif

//
// Power Mode Functions
//

#if (NTDDI_VERSION >= NTDDI_WIN11_GA)

STDAPI_(DWORD)
PowerGetUserConfiguredACPowerMode(
    _Out_ GUID *PowerModeGuid
    );

STDAPI_(DWORD)
PowerGetUserConfiguredDCPowerMode(
    _Out_ GUID *PowerModeGuid
    );

STDAPI_(DWORD)
PowerSetUserConfiguredACPowerMode(
    _In_ const GUID *PowerModeGuid
    );

STDAPI_(DWORD)
PowerSetUserConfiguredDCPowerMode(
    _In_ const GUID *PowerModeGuid
    );

#endif //#if (NTDDI_VERSION >= NTDDI_WIN11_GA)

//
// Read functions.
//
#if (NTDDI_VERSION >= NTDDI_VISTA)
STDAPI_(DWORD)
PowerReadACValueIndex (
    _In_opt_ HKEY RootPowerKey,
    _In_opt_ CONST GUID *SchemeGuid,
    _In_opt_ CONST GUID *SubGroupOfPowerSettingsGuid,
    _In_opt_ CONST GUID *PowerSettingGuid,
    _Out_ LPDWORD AcValueIndex
    );
#endif

#if (NTDDI_VERSION >= NTDDI_VISTA)
STDAPI_(DWORD)
PowerReadDCValueIndex (
    _In_opt_ HKEY RootPowerKey,
    _In_opt_ CONST GUID *SchemeGuid,
    _In_opt_ CONST GUID *SubGroupOfPowerSettingsGuid,
    _In_opt_ CONST GUID *PowerSettingGuid,
    _Out_ LPDWORD DcValueIndex
    );
#endif

#if (NTDDI_VERSION >= NTDDI_VISTA)
STDAPI_(DWORD)
PowerReadFriendlyName (
    _In_opt_ HKEY RootPowerKey,
    _In_opt_ CONST GUID *SchemeGuid,
    _In_opt_ CONST GUID *SubGroupOfPowerSettingsGuid,
    _In_opt_ CONST GUID *PowerSettingGuid,
    _Out_writes_bytes_opt_(*BufferSize) PUCHAR Buffer,
    _Inout_ LPDWORD BufferSize
    );
#endif

#if (NTDDI_VERSION >= NTDDI_VISTA)
STDAPI_(DWORD)
PowerReadDescription (
    _In_opt_ HKEY RootPowerKey,
    _In_opt_ CONST GUID *SchemeGuid,
    _In_opt_ CONST GUID *SubGroupOfPowerSettingsGuid,
    _In_opt_ CONST GUID *PowerSettingGuid,
    _Out_writes_bytes_opt_(*BufferSize) PUCHAR Buffer,
    _Inout_ LPDWORD BufferSize
    );
#endif

#if (NTDDI_VERSION >= NTDDI_VISTA)
STDAPI_(DWORD)
PowerReadPossibleValue (
    _In_opt_ HKEY RootPowerKey,
    _In_opt_ CONST GUID *SubGroupOfPowerSettingsGuid,
    _In_opt_ CONST GUID *PowerSettingGuid,
    _Out_opt_ PULONG Type,
    _In_ ULONG PossibleSettingIndex,
    _Out_writes_bytes_opt_(*BufferSize) PUCHAR Buffer,
    _Inout_ LPDWORD BufferSize
    );
#endif

#if (NTDDI_VERSION >= NTDDI_VISTA)
STDAPI_(DWORD)
PowerReadPossibleFriendlyName (
    _In_opt_ HKEY RootPowerKey,
    _In_opt_ CONST GUID *SubGroupOfPowerSettingsGuid,
    _In_opt_ CONST GUID *PowerSettingGuid,
    _In_ ULONG PossibleSettingIndex,
    _Out_writes_bytes_opt_(*BufferSize) PUCHAR Buffer,
    _Inout_ LPDWORD BufferSize
    );
#endif

#if (NTDDI_VERSION >= NTDDI_VISTA)
STDAPI_(DWORD)
PowerReadPossibleDescription (
    _In_opt_ HKEY RootPowerKey,
    _In_opt_ CONST GUID *SubGroupOfPowerSettingsGuid,
    _In_opt_ CONST GUID *PowerSettingGuid,
    _In_ ULONG PossibleSettingIndex,
    _Out_writes_bytes_opt_(*BufferSize) PUCHAR Buffer,
    _Inout_ LPDWORD BufferSize
    );
#endif

#if (NTDDI_VERSION >= NTDDI_VISTA)
STDAPI_(DWORD)
PowerReadValueMin (
    _In_opt_ HKEY RootPowerKey,
    _In_opt_ CONST GUID *SubGroupOfPowerSettingsGuid,
    _In_opt_ CONST GUID *PowerSettingGuid,
    _Out_ LPDWORD ValueMinimum
    );
#endif

#if (NTDDI_VERSION >= NTDDI_VISTA)
STDAPI_(DWORD)
PowerReadValueMax (
    _In_opt_ HKEY RootPowerKey,
    _In_opt_ CONST GUID *SubGroupOfPowerSettingsGuid,
    _In_opt_ CONST GUID *PowerSettingGuid,
    _Out_ LPDWORD ValueMaximum
    );
#endif

#if (NTDDI_VERSION >= NTDDI_VISTA)
STDAPI_(DWORD)
PowerReadValueIncrement (
    _In_opt_ HKEY RootPowerKey,
    _In_opt_ CONST GUID *SubGroupOfPowerSettingsGuid,
    _In_opt_ CONST GUID *PowerSettingGuid,
    _Out_ LPDWORD ValueIncrement
    );
#endif

#if (NTDDI_VERSION >= NTDDI_VISTA)
STDAPI_(DWORD)
PowerReadValueUnitsSpecifier (
    _In_opt_ HKEY RootPowerKey,
    _In_opt_ CONST GUID *SubGroupOfPowerSettingsGuid,
    _In_opt_ CONST GUID *PowerSettingGuid,
    _Out_writes_bytes_opt_(*BufferSize) UCHAR *Buffer,
    _Inout_ LPDWORD BufferSize
    );
#endif

#if (NTDDI_VERSION >= NTDDI_VISTA)
STDAPI_(DWORD)
PowerReadACDefaultIndex (
    _In_opt_ HKEY RootPowerKey,
    _In_ CONST GUID *SchemePersonalityGuid,
    _In_opt_ CONST GUID *SubGroupOfPowerSettingsGuid OPTIONAL,
    _In_ CONST GUID *PowerSettingGuid,
    _Out_ LPDWORD AcDefaultIndex
    );
#endif

#if (NTDDI_VERSION >= NTDDI_VISTA)
STDAPI_(DWORD)
PowerReadDCDefaultIndex (
    _In_opt_ HKEY RootPowerKey,
    _In_ CONST GUID *SchemePersonalityGuid,
    _In_opt_ CONST GUID *SubGroupOfPowerSettingsGuid,
    _In_ CONST GUID *PowerSettingGuid,
    _Out_ LPDWORD DcDefaultIndex
    );
#endif

#if (NTDDI_VERSION >= NTDDI_VISTA)
STDAPI_(DWORD)
PowerReadIconResourceSpecifier (
    _In_opt_ HKEY RootPowerKey,
    _In_opt_ CONST GUID *SchemeGuid,
    _In_opt_ CONST GUID *SubGroupOfPowerSettingsGuid,
    _In_opt_ CONST GUID *PowerSettingGuid,
    _Out_writes_bytes_opt_(*BufferSize) PUCHAR Buffer,
    _Inout_ LPDWORD BufferSize
    );
#endif

#if (NTDDI_VERSION >= NTDDI_VISTA)
STDAPI_(DWORD)
PowerReadSettingAttributes (
    _In_opt_ CONST GUID *SubGroupGuid,
    _In_opt_ CONST GUID *PowerSettingGuid
    );
#endif

//
// Write functions.
//
#if (NTDDI_VERSION >= NTDDI_VISTA)
STDAPI_(DWORD)
PowerWriteFriendlyName (
    _In_opt_ HKEY RootPowerKey,
    _In_ CONST GUID *SchemeGuid,
    _In_opt_ CONST GUID *SubGroupOfPowerSettingsGuid,
    _In_opt_ CONST GUID *PowerSettingGuid,
    _In_reads_bytes_(BufferSize) UCHAR *Buffer,
    _In_ DWORD BufferSize
    );
#endif

#if (NTDDI_VERSION >= NTDDI_VISTA)
STDAPI_(DWORD)
PowerWriteDescription (
    _In_opt_ HKEY RootPowerKey,
    _In_ CONST GUID *SchemeGuid,
    _In_opt_ CONST GUID *SubGroupOfPowerSettingsGuid,
    _In_opt_ CONST GUID *PowerSettingGuid,
    _In_reads_bytes_(BufferSize) UCHAR *Buffer,
    _In_ DWORD BufferSize
    );
#endif

#if (NTDDI_VERSION >= NTDDI_VISTA)
STDAPI_(DWORD)
PowerWritePossibleValue (
    _In_opt_ HKEY RootPowerKey,
    _In_opt_ CONST GUID *SubGroupOfPowerSettingsGuid,
    _In_opt_ CONST GUID *PowerSettingGuid,
    _In_ ULONG Type,
    _In_ ULONG PossibleSettingIndex,
    _In_reads_bytes_(BufferSize) UCHAR *Buffer,
    _In_ DWORD BufferSize
    );
#endif

#if (NTDDI_VERSION >= NTDDI_VISTA)
STDAPI_(DWORD)
PowerWritePossibleFriendlyName (
    _In_opt_ HKEY RootPowerKey,
    _In_opt_ CONST GUID *SubGroupOfPowerSettingsGuid,
    _In_opt_ CONST GUID *PowerSettingGuid,
    _In_ ULONG PossibleSettingIndex,
    _In_reads_bytes_(BufferSize) UCHAR *Buffer,
    _In_ DWORD BufferSize
    );
#endif

#if (NTDDI_VERSION >= NTDDI_VISTA)
STDAPI_(DWORD)
PowerWritePossibleDescription (
    _In_opt_ HKEY RootPowerKey,
    _In_opt_ CONST GUID *SubGroupOfPowerSettingsGuid,
    _In_opt_ CONST GUID *PowerSettingGuid,
    _In_ ULONG PossibleSettingIndex,
    _In_reads_bytes_(BufferSize) UCHAR *Buffer,
    _In_ DWORD BufferSize
    );
#endif

#if (NTDDI_VERSION >= NTDDI_VISTA)
STDAPI_(DWORD)
PowerWriteValueMin (
    _In_opt_ HKEY RootPowerKey,
    _In_opt_ CONST GUID *SubGroupOfPowerSettingsGuid,
    _In_opt_ CONST GUID *PowerSettingGuid,
    _In_ DWORD ValueMinimum
    );
#endif

#if (NTDDI_VERSION >= NTDDI_VISTA)
STDAPI_(DWORD)
PowerWriteValueMax (
    _In_opt_ HKEY RootPowerKey,
    _In_opt_ CONST GUID *SubGroupOfPowerSettingsGuid,
    _In_opt_ CONST GUID *PowerSettingGuid,
    _In_ DWORD ValueMaximum
    );
#endif

#if (NTDDI_VERSION >= NTDDI_VISTA)
STDAPI_(DWORD)
PowerWriteValueIncrement (
    _In_opt_ HKEY RootPowerKey,
    _In_opt_ CONST GUID *SubGroupOfPowerSettingsGuid,
    _In_opt_ CONST GUID *PowerSettingGuid,
    _In_ DWORD ValueIncrement
    );
#endif

#if (NTDDI_VERSION >= NTDDI_VISTA)
STDAPI_(DWORD)
PowerWriteValueUnitsSpecifier (
    _In_opt_ HKEY RootPowerKey,
    _In_opt_ CONST GUID *SubGroupOfPowerSettingsGuid,
    _In_opt_ CONST GUID *PowerSettingGuid,
    _In_reads_bytes_(BufferSize) UCHAR *Buffer,
    _In_ DWORD BufferSize
    );
#endif

#if (NTDDI_VERSION >= NTDDI_VISTA)
STDAPI_(DWORD)
PowerWriteACDefaultIndex (
    _In_opt_ HKEY RootSystemPowerKey,
    _In_ CONST GUID *SchemePersonalityGuid,
    _In_opt_ CONST GUID *SubGroupOfPowerSettingsGuid,
    _In_ CONST GUID *PowerSettingGuid,
    _In_ DWORD DefaultAcIndex
    );
#endif

#if (NTDDI_VERSION >= NTDDI_VISTA)
STDAPI_(DWORD)
PowerWriteDCDefaultIndex (
    _In_opt_ HKEY RootSystemPowerKey,
    _In_ CONST GUID *SchemePersonalityGuid,
    _In_opt_ CONST GUID *SubGroupOfPowerSettingsGuid,
    _In_ CONST GUID *PowerSettingGuid,
    _In_ DWORD DefaultDcIndex
    );
#endif

#if (NTDDI_VERSION >= NTDDI_VISTA)
STDAPI_(DWORD)
PowerWriteIconResourceSpecifier (
    _In_opt_ HKEY RootPowerKey,
    _In_ CONST GUID *SchemeGuid,
    _In_opt_ CONST GUID *SubGroupOfPowerSettingsGuid,
    _In_opt_ CONST GUID *PowerSettingGuid,
    _In_reads_bytes_(BufferSize) UCHAR *Buffer,
    _In_ DWORD BufferSize
    );
#endif

#if (NTDDI_VERSION >= NTDDI_VISTA)
STDAPI_(DWORD)
PowerWriteSettingAttributes (
    _In_opt_ CONST GUID *SubGroupGuid,
    _In_opt_ CONST GUID *PowerSettingGuid,
    _In_ DWORD Attributes
    );
#endif

#if (NTDDI_VERSION >= NTDDI_VISTA)
STDAPI_(DWORD)
PowerDuplicateScheme (
    _In_opt_ HKEY RootPowerKey,
    _In_ const GUID *SourceSchemeGuid,
    _Inout_ GUID **DestinationSchemeGuid
    );
#endif

#if (NTDDI_VERSION >= NTDDI_VISTA)
STDAPI_(DWORD)
PowerImportPowerScheme (
    _In_opt_ HKEY RootPowerKey,
    _In_ LPCWSTR ImportFileNamePath,
    _Inout_ GUID **DestinationSchemeGuid
    );
#endif

#if (NTDDI_VERSION >= NTDDI_VISTA)
STDAPI_(DWORD)
PowerDeleteScheme (
    _In_opt_ HKEY RootPowerKey,
    _In_ CONST GUID *SchemeGuid
    );
#endif

#if (NTDDI_VERSION >= NTDDI_VISTA)
STDAPI_(DWORD)
PowerRemovePowerSetting (
   _In_ CONST GUID *PowerSettingSubKeyGuid,
   _In_ CONST GUID *PowerSettingGuid
   );
#endif

#if (NTDDI_VERSION >= NTDDI_VISTA)
STDAPI_(DWORD)
PowerCreateSetting (
    _In_opt_ HKEY RootSystemPowerKey,
    _In_ CONST GUID *SubGroupOfPowerSettingsGuid,
    _In_ CONST GUID *PowerSettingGuid
    );
#endif

#if (NTDDI_VERSION >= NTDDI_VISTA)
STDAPI_(DWORD)
PowerCreatePossibleSetting (
    _In_opt_ HKEY RootSystemPowerKey,
    _In_ CONST GUID *SubGroupOfPowerSettingsGuid,
    _In_ CONST GUID *PowerSettingGuid,
    _In_ ULONG PossibleSettingIndex
    );
#endif

//
// Enumerate Functions.
//
#if (NTDDI_VERSION >= NTDDI_VISTA)
STDAPI_(DWORD)
PowerEnumerate (
    _In_opt_ HKEY RootPowerKey,
    _In_opt_ CONST GUID *SchemeGuid,
    _In_opt_ CONST GUID *SubGroupOfPowerSettingsGuid,
    _In_ POWER_DATA_ACCESSOR AccessFlags,
    _In_ ULONG Index,
    _Out_writes_bytes_opt_(*BufferSize) UCHAR *Buffer,
    _Inout_ DWORD *BufferSize
    );
#endif

#if (NTDDI_VERSION >= NTDDI_VISTA)
STDAPI_(DWORD)
PowerOpenUserPowerKey (
    _Out_ HKEY *phUserPowerKey,
    _In_ REGSAM Access,
    _In_ BOOL OpenExisting
    );
#endif

#if (NTDDI_VERSION >= NTDDI_VISTA)
STDAPI_(DWORD)
PowerOpenSystemPowerKey (
    _Out_ HKEY *phSystemPowerKey,
    _In_ REGSAM Access,
    _In_ BOOL OpenExisting
    );
#endif

#if (NTDDI_VERSION >= NTDDI_VISTA)
STDAPI_(DWORD)
PowerCanRestoreIndividualDefaultPowerScheme (
    _In_ CONST GUID *SchemeGuid
    );
#endif

#if (NTDDI_VERSION >= NTDDI_VISTA)
STDAPI_(DWORD)
PowerRestoreIndividualDefaultPowerScheme (
    _In_ CONST GUID *SchemeGuid
    );
#endif

#if (NTDDI_VERSION >= NTDDI_VISTA)
STDAPI_(DWORD)
PowerRestoreDefaultPowerSchemes(
    VOID
    );
#endif

#if (NTDDI_VERSION >= NTDDI_VISTA)
STDAPI_(DWORD)
PowerReplaceDefaultPowerSchemes(
    VOID
    );
#endif

#if (NTDDI_VERSION >= NTDDI_VISTA)
STDAPI_(POWER_PLATFORM_ROLE)
PowerDeterminePlatformRole(
    VOID
    );
#endif

//
// =========================================
// Device Power APIs
// =========================================
//
//
// ========================================================
// These flags tell us how to interpret a query of
// device power.  Use these (or a combination of these)
// for the QueryInterpretationFlags parameter sent into
// DevicePowerEnumDevices().
//
// They'll ask us for some devices that supports some
// D state.  These flags help us determine what the
// user really wants.  I.e. does he want the device
// name, or the hardware ID.  Does he want some device
// that supports some D state, or does he want us to
// go see if that devices supports some S state? ...
// ========================================================
//

//
// Return HardwareID instead of the friendly device name, which
// is the default.
//
#define DEVICEPOWER_HARDWAREID             (0x80000000)

//
//
// 'and' the requested power states.  I.e. if the user requested
// that we query devices supporting D1 and D3, this flag says
// "give me devices that support D1 *and* D3".  Without this
// flag, the query would be interpreted as "give me devices
// that support D1 *or* D3".
//
#define DEVICEPOWER_AND_OPERATION          (0x40000000)

// Only preform the query on devices that are present in the system.
//
#define DEVICEPOWER_FILTER_DEVICES_PRESENT (0x20000000)

//
// Only preform the query on devices that are actual hardware.
//
#define DEVICEPOWER_FILTER_HARDWARE        (0x10000000)

//
// Only preform the query on devices that are armed to wake the
// system from a sleep state.
//
#define DEVICEPOWER_FILTER_WAKEENABLED     (0x08000000)

//
// Only preform the query on devices that are capable of being programmed
// to wake the system from a sleep state.
//
#define DEVICEPOWER_FILTER_WAKEPROGRAMMABLE (0x04000000)

//
// Go find the device who's name is specifed by an input
// parameter, then see if it's got the capabilities specified
// in QueryFlags.
//
#define DEVICEPOWER_FILTER_ON_NAME         (0x02000000)



//
// Define flags to set/clear capabilities in the devices.
//

//
// Enable the device to wake the system from a sleep state.
//
#define DEVICEPOWER_SET_WAKEENABLED (0x00000001)

//
// Disable the device from waking the system from a sleep
// state.
//
#define DEVICEPOWER_CLEAR_WAKEENABLED (0x00000002)




//
// For the QueryFlags parameter, the user should send in
// one of the PDCAP_D*_SUPPORTED flags found in ntpoapi.h.
//
// #define PDCAP_D0_SUPPORTED              0x00000001
// #define PDCAP_D1_SUPPORTED              0x00000002
// #define PDCAP_D2_SUPPORTED              0x00000004
// #define PDCAP_D3_SUPPORTED              0x00000008
// #define PDCAP_WAKE_FROM_D0_SUPPORTED    0x00000010
// #define PDCAP_WAKE_FROM_D1_SUPPORTED    0x00000020
// #define PDCAP_WAKE_FROM_D2_SUPPORTED    0x00000040
// #define PDCAP_WAKE_FROM_D3_SUPPORTED    0x00000080
// #define PDCAP_WARM_EJECT_SUPPORTED      0x00000100
//
// The user can also send in a combination of these values
// to do queries that match either or both of the specified
// flags.  E.g. if the user used (PDCAP_D1_SUPPORTED | PDCAP_D3_SUPPORTED),
// then the query would return a device that supported *either*
// D1 or D3.  The user could specify QUERY_AND_OPERATION in
// QueryInterpretationFlags to get a device that supported
// D1 *and* D3.
//

//
// We also support querying on S states.  Although devices don't
// really understand system states, we can use the D-to-S state
// mappings to derive this.  For example, if we ask a device if
// he supports S2, we can derive that by looking at his D-to-S
// mappings.  Suppose the device has a D-to-S state mapping
// that looks like this:
// S0 -> D0
// S1 -> D3
// S2 -> D3
// S3 -> D3
// S4 -> D3
// S5 -> D3
//
// We see that S2 maps to D3 on this device.  Does this device
// actually support D3?  We can find that out.  If so, then we
// say that this device supports S2.
//
// We have to make up the S-state vectors because there's no
// existing values like the PDCAP_D* values.
//
#define PDCAP_S0_SUPPORTED              0x00010000
#define PDCAP_S1_SUPPORTED              0x00020000
#define PDCAP_S2_SUPPORTED              0x00040000
#define PDCAP_S3_SUPPORTED              0x00080000
#define PDCAP_WAKE_FROM_S0_SUPPORTED    0x00100000
#define PDCAP_WAKE_FROM_S1_SUPPORTED    0x00200000
#define PDCAP_WAKE_FROM_S2_SUPPORTED    0x00400000
#define PDCAP_WAKE_FROM_S3_SUPPORTED    0x00800000
#define PDCAP_S4_SUPPORTED              0x01000000
#define PDCAP_S5_SUPPORTED              0x02000000

//
// Function prototypes
//
#if (NTDDI_VERSION >= NTDDI_WS03)
STDAPI_(BOOLEAN)
DevicePowerEnumDevices(
    _In_ ULONG  QueryIndex,
    _In_ ULONG  QueryInterpretationFlags,
    _In_ ULONG  QueryFlags,
    _Out_writes_bytes_opt_(*pBufferSize) PBYTE  pReturnBuffer,
    _Inout_ PULONG pBufferSize
    );
#endif

#if (NTDDI_VERSION >= NTDDI_WS03)
STDAPI_(DWORD)
DevicePowerSetDeviceState(
    _In_ LPCWSTR DeviceDescription,
    _In_ ULONG SetFlags,
    _In_opt_ PVOID SetData
    );
#endif

#if (NTDDI_VERSION >= NTDDI_WS03)
STDAPI_(BOOLEAN)
DevicePowerOpen(
    _In_opt_ ULONG DebugMask
    );
#endif

#if (NTDDI_VERSION >= NTDDI_WS03)
STDAPI_(BOOLEAN)
DevicePowerClose(
    VOID
    );
#endif

//
// Thermal event notifications
//

#define THERMAL_EVENT_VERSION 1

typedef struct _THERMAL_EVENT {
    ULONG Version;
    ULONG Size;
    ULONG Type;
    ULONG Temperature;
    ULONG TripPointTemperature;
    LPWSTR Initiator; 
} THERMAL_EVENT, *PTHERMAL_EVENT; 

#if (NTDDI_VERSION >= NTDDI_WINBLUE)
STDAPI_(DWORD)
PowerReportThermalEvent (
    _In_ PTHERMAL_EVENT Event
    );
#endif

#ifdef __cplusplus
}
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // _POWRPROF_H_
