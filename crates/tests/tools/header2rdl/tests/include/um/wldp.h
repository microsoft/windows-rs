/*

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    wldp.h

Abstract:

    Interface for the Windows Lockdown Policy APIs.

*/

#ifndef _WLDP_H_INCLUDED_
#define _WLDP_H_INCLUDED_

#if defined (_MSC_VER)
#pragma once
#endif
#include <winapifamily.h>
#include <objidl.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#define WLDP_DLL                                   L"WLDP.DLL"
#define WLDP_GETLOCKDOWNPOLICY_FN                   "WldpGetLockdownPolicy"
#define WLDP_ISCLASSINAPPROVEDLIST_FN               "WldpIsClassInApprovedList"
#define WLDP_SETDYNAMICCODETRUST_FN                 "WldpSetDynamicCodeTrust"
#define WLDP_ISDYNAMICCODEPOLICYENABLED_FN          "WldpIsDynamicCodePolicyEnabled"
#define WLDP_QUERYDANAMICCODETRUST_FN               "WldpQueryDynamicCodeTrust"
#define WLDP_QUERYDYNAMICCODETRUST_FN               "WldpQueryDynamicCodeTrust"
#define WLDP_QUERYWINDOWSLOCKDOWNMODE_FN            "WldpQueryWindowsLockdownMode"
#define WLDP_SETWINDOWSLOCKDOWNRESTRICTION_FN       "WldpSetWindowsLockdownRestriction"
#define WLDP_QUERYDEVICESECURITYINFORMATION_FN      "WldpQueryDeviceSecurityInformation"
#define WLDP_QUERYWINDOWSLOCKDOWNRESTRICTION_FN     "WldpQueryWindowsLockdownRestriction"
#define WLDP_ISAPPAPPROVEDBYPOLICY_FN               "WldpIsAppApprovedByPolicy"
#define WLDP_QUERYPOLICYSETTINGENABLED_FN           "WldpQueryPolicySettingEnabled"
#define WLDP_QUERYPOLICYSETTINGENABLED2_FN          "WldpQueryPolicySettingEnabled2"
#define WLDP_ISWCOSPRODUCTIONCONFIGURATION_FN       "WldpIsWcosProductionConfiguration"
#define WLDP_RESETWCOSPRODUCTIONCONFIGURATION_FN    "WldpResetWcosProductionConfiguration"
#define WLDP_ISPRODUCTIONCONFIGURATION_FN           "WldpIsProductionConfiguration"
#define WLDP_RESETPRODUCTIONCONFIGURATION_FN        "WldpResetProductionConfiguration"
#define WLDP_CANEXECUTEBUFFER_FN                    "WldpCanExecuteBuffer"
#define WLDP_CANEXECUTEFILE_FN                      "WldpCanExecuteFile"
#define WLDP_CANEXECUTEBUFFER_FN                    "WldpCanExecuteBuffer"
#define WLDP_CANEXECUTEFILEFROMDETACHEDSIGNATURE_FN "WldpCanExecuteFileFromDetachedSignature"
#define WLDP_QUERYSECURITYPOLICY_FN                 "WldpQuerySecurityPolicy"
//
//  Policy state bits.
//

/*
 Policy  | UMCI | Audit | Exclusion || Definition |
 State   |  On  |  On   |   On      ||            |
 --------|------|-------|-----------||------------|
 Off     |   0  |   *   |     *     ||   0 0 0    |
 Debug   |   1  |   *   |     1     ||   1 0 1    |
 Audit   |   1  |   1   |     *     ||   1 1 0    |
 Enforce |   1  |   0   |     0     ||   1 0 0    |
*/

//
// Flags for WLDP_LOCKDOWN_VALUE.
//

#define WLDP_LOCKDOWN_UNDEFINED            (0x00000000)
#define WLDP_LOCKDOWN_DEFINED_FLAG         (0x80000000)
#define WLDP_LOCKDOWN_CONFIG_CI_FLAG       (0x00000001)
#define WLDP_LOCKDOWN_CONFIG_CI_AUDIT_FLAG (0x00000002) // == CODEINTEGRITYPOLICY_OPTION_AUDIT_ENABLED
#define WLDP_LOCKDOWN_UMCIENFORCE_FLAG     (0x00000004) // == CODEINTEGRITY_OPTION_UMCI_ENABLED
#define WLDP_LOCKDOWN_AUDIT_FLAG           (0x00000008) // == CODEINTEGRITY_OPTION_UMCI_AUDITMODE_ENABLED
#define WLDP_LOCKDOWN_EXCLUSION_FLAG       (0x00000010) // == CODEINTEGRITY_OPTION_UMCI_EXCLUSIONPATHS_ENABLED
#define WLDP_LOCKDOWN_ALL_FLAGS            (WLDP_LOCKDOWN_DEFINED_FLAG | \
                                            WLDP_LOCKDOWN_CONFIG_CI_FLAG | \
                                            WLDP_LOCKDOWN_CONFIG_CI_AUDIT_FLAG | \
                                            WLDP_LOCKDOWN_UMCIENFORCE_FLAG | \
                                            WLDP_LOCKDOWN_AUDIT_FLAG | \
                                            WLDP_LOCKDOWN_EXCLUSION_FLAG)
//
// These are the only recornized policy status flag combinations of this feature.
// All other combinations will be reverted to WLDP_LOCKDOWN_DEFAULT.
//

#define WLDP_LOCKDOWN_OFF                  (WLDP_LOCKDOWN_DEFINED_FLAG)
#define WLDP_LOCKDOWN_DEBUG                (WLDP_LOCKDOWN_DEFINED_FLAG | \
                                            WLDP_LOCKDOWN_UMCIENFORCE_FLAG | \
                                            WLDP_LOCKDOWN_EXCLUSION_FLAG)
#define WLDP_LOCKDOWN_AUDIT                (WLDP_LOCKDOWN_DEFINED_FLAG | \
                                            WLDP_LOCKDOWN_UMCIENFORCE_FLAG | \
                                            WLDP_LOCKDOWN_AUDIT_FLAG | \
                                            WLDP_LOCKDOWN_EXCLUSION_FLAG)
#define WLDP_LOCKDOWN_ENFORCE              (WLDP_LOCKDOWN_DEFINED_FLAG | \
                                            WLDP_LOCKDOWN_UMCIENFORCE_FLAG)
#define WLDP_LOCKDOWN_CONFIG_CI            (WLDP_LOCKDOWN_DEFINED_FLAG | \
                                            WLDP_LOCKDOWN_CONFIG_CI_FLAG | \
                                            WLDP_LOCKDOWN_UMCIENFORCE_FLAG)
#define WLDP_LOCKDOWN_CONFIG_CI_AUDIT      (WLDP_LOCKDOWN_DEFINED_FLAG | \
                                            WLDP_LOCKDOWN_CONFIG_CI_FLAG | \
                                            WLDP_LOCKDOWN_CONFIG_CI_AUDIT_FLAG | \
                                            WLDP_LOCKDOWN_UMCIENFORCE_FLAG)
//
// State comparison masks. These masks are used to strip off wild card bits.
//

#define WLDP_LOCKDOWN_OFF_MASK             (WLDP_LOCKDOWN_DEFINED_FLAG | \
                                            WLDP_LOCKDOWN_UMCIENFORCE_FLAG)
#define WLDP_LOCKDOWN_DEBUG_MASK           (WLDP_LOCKDOWN_DEFINED_FLAG | \
                                            WLDP_LOCKDOWN_UMCIENFORCE_FLAG | \
                                            WLDP_LOCKDOWN_AUDIT_FLAG | \
                                            WLDP_LOCKDOWN_EXCLUSION_FLAG)
#define WLDP_LOCKDOWN_AUDIT_MASK           (WLDP_LOCKDOWN_DEFINED_FLAG | \
                                            WLDP_LOCKDOWN_UMCIENFORCE_FLAG | \
                                            WLDP_LOCKDOWN_AUDIT_FLAG | \
                                            WLDP_LOCKDOWN_EXCLUSION_FLAG)
#define WLDP_LOCKDOWN_ENFORCE_MASK         (WLDP_LOCKDOWN_DEFINED_FLAG | \
                                            WLDP_LOCKDOWN_UMCIENFORCE_FLAG | \
                                            WLDP_LOCKDOWN_AUDIT_FLAG | \
                                            WLDP_LOCKDOWN_EXCLUSION_FLAG)
#define WLDP_LOCKDOWN_CONFIG_CI_MASK       (WLDP_LOCKDOWN_DEFINED_FLAG | \
                                            WLDP_LOCKDOWN_CONFIG_CI_FLAG | \
                                            WLDP_LOCKDOWN_UMCIENFORCE_FLAG)
#define WLDP_LOCKDOWN_CONFIG_CI_AUDIT_MASK (WLDP_LOCKDOWN_DEFINED_FLAG | \
                                            WLDP_LOCKDOWN_CONFIG_CI_FLAG | \
                                            WLDP_LOCKDOWN_CONFIG_CI_AUDIT_FLAG | \
                                            WLDP_LOCKDOWN_UMCIENFORCE_FLAG)
//
// Policy state checking macros.
//

#define WLDP_LOCKDOWN_IS_OFF(_PolicyState)          ((_PolicyState & WLDP_LOCKDOWN_OFF_MASK) == WLDP_LOCKDOWN_OFF)
#define WLDP_LOCKDOWN_IS_DEBUG(_PolicyState)        ((_PolicyState & WLDP_LOCKDOWN_DEBUG_MASK) == WLDP_LOCKDOWN_DEBUG)
#define WLDP_LOCKDOWN_IS_AUDIT(_PolicyState)        ((_PolicyState & WLDP_LOCKDOWN_AUDIT_MASK) == WLDP_LOCKDOWN_AUDIT)
#define WLDP_LOCKDOWN_IS_ENFORCE(_PolicyState)      ((_PolicyState & WLDP_LOCKDOWN_ENFORCE_MASK) == WLDP_LOCKDOWN_ENFORCE)
#define WLDP_LOCKDOWN_IS_CONFIG_CI(_PolicyState)    ((_PolicyState & WLDP_LOCKDOWN_CONFIG_CI_MASK) == WLDP_LOCKDOWN_CONFIG_CI)
#define WLDP_LOCKDOWN_IS_CONFIG_CI_AUDIT(_PolicyState) ((_PolicyState & WLDP_LOCKDOWN_CONFIG_CI_AUDIT_MASK) == WLDP_LOCKDOWN_CONFIG_CI_AUDIT)
#define WLDP_LOCKDOWN_IS_UMCIENFORCE(_PolicyState)  ((_PolicyState & WLDP_LOCKDOWN_UMCIENFORCE_FLAG) == WLDP_LOCKDOWN_UMCIENFORCE_FLAG)

//
//  Host types.
//

typedef enum WLDP_HOST
{
    WLDP_HOST_RUNDLL32 = 0,
    WLDP_HOST_SVCHOST,
    WLDP_HOST_MAX
} WLDP_HOST;

//
//  Host ID types.
//

typedef enum WLDP_HOST_ID
{
    WLDP_HOST_ID_UNKNOWN = 0,
    WLDP_HOST_ID_GLOBAL,
    WLDP_HOST_ID_VBA,
    WLDP_HOST_ID_WSH,
    WLDP_HOST_ID_POWERSHELL,
    WLDP_HOST_ID_IE,
    WLDP_HOST_ID_MSI,
    WLDP_HOST_ID_ALL,
    WLDP_HOST_ID_MAX
} WLDP_HOST_ID;

//
// Secure Setting Decision Location names for telemetry
// 

typedef enum DECISION_LOCATION
{
    DECISION_LOCATION_REFRESH_GLOBAL_DATA = 0,
    DECISION_LOCATION_PARAMETER_VALIDATION,
    DECISION_LOCATION_AUDIT,
    DECISION_LOCATION_FAILED_CONVERT_GUID,
    DECISION_LOCATION_ENTERPRISE_DEFINED_CLASS_ID,
    DECISION_LOCATION_GLOBAL_BUILT_IN_LIST,
    DECISION_LOCATION_PROVIDER_BUILT_IN_LIST,
    DECISION_LOCATION_ENFORCE_STATE_LIST,
    DECISION_LOCATION_NOT_FOUND,
    DECISION_LOCATION_UNKNOWN
} DECISION_LOCATION;

//
// Secure Setting Key values
//

typedef enum WLDP_KEY
{
    KEY_UNKNOWN = 0,
    KEY_OVERRIDE,
    KEY_ALL_KEYS
} WLDP_KEY;

//
// Secure Setting ValueName values
//

typedef enum VALUENAME
{
    VALUENAME_UNKNOWN = 0,
    VALUENAME_ENTERPRISE_DEFINED_CLASS_ID,
    VALUENAME_BUILT_IN_LIST
} VALUENAME;

//
//  Windows Lockdown mode.
//

typedef enum WLDP_WINDOWS_LOCKDOWN_MODE
{
    WLDP_WINDOWS_LOCKDOWN_MODE_UNLOCKED = 0,
    WLDP_WINDOWS_LOCKDOWN_MODE_TRIAL,
    WLDP_WINDOWS_LOCKDOWN_MODE_LOCKED,
    WLDP_WINDOWS_LOCKDOWN_MODE_MAX,
} WLDP_WINDOWS_LOCKDOWN_MODE, *PWLDP_WINDOWS_LOCKDOWN_MODE;

typedef enum WLDP_WINDOWS_LOCKDOWN_RESTRICTION
{
    WLDP_WINDOWS_LOCKDOWN_RESTRICTION_NONE = 0,
    WLDP_WINDOWS_LOCKDOWN_RESTRICTION_NOUNLOCK,
    WLDP_WINDOWS_LOCKDOWN_RESTRICTION_NOUNLOCK_PERMANENT,
    WLDP_WINDOWS_LOCKDOWN_RESTRICTION_MAX,
} WLDP_WINDOWS_LOCKDOWN_RESTRICTION, *PWLDP_WINDOWS_LOCKDOWN_RESTRICTION;

//
// WldpQueryPolicySettingEnabled Setting Enum
//

typedef enum WLDP_POLICY_SETTING
{
    WLDP_POLICY_SETTING_AV_PERF_MODE = 1000,
} WLDP_POLICY_SETTING, *PWLDP_POLICY_SETTING;

//
//  WLDP_HOST_INFORMATION Version.
//

#define WLDP_HOST_INFORMATION_REVISION              (0x00000001)

//
//  Host information structure.
//

typedef struct WLDP_HOST_INFORMATION
{
    DWORD dwRevision;      // Has to be WLDP_HOST_INFORMATION_REVISION.
    WLDP_HOST_ID dwHostId; // Enum value from WLDP_HOST_ID.
    PCWSTR szSource;       // Full path and script name with extension. NULL for WLDP_HOST_ID_GLOBAL or manual script execution.
    HANDLE hSource;        // Additionally to the name, the caller may specify a handle to the file that is used for validation.
} WLDP_HOST_INFORMATION, *PWLDP_HOST_INFORMATION;

//
//  Device Security Information.
//

typedef struct WLDP_DEVICE_SECURITY_INFORMATION
{
    DWORD UnlockIdSize; // UnlockId size in byte
    PBYTE UnlockId; // Device specific UnlockId if exists
    DWORD ManufacturerIDLength; // ManufacturerId string size in byte
    PWSTR _Field_size_bytes_(ManufacturerIDLength) ManufacturerID; // ManufacturerID on device if exists
} WLDP_DEVICE_SECURITY_INFORMATION, *PWLDP_DEVICE_SECURITY_INFORMATION;

//
// Call the library to get the lockdown state relative to the host and script/msi to be used.
// When called with WLDP_HOST_INFORMATION.szSource = NULL, the generic policy for the host is returned.
// When called with WLDP_HOST_INFORMATION.dwHostId = WLDP_HOST_ID_GLOBAL, WLDP_HOST_INFORMATION.szSource
// has to be NULL and the function will return the global system policy.
// The dwFlag WLDP_FLAGS_SKIPSIGNATUREVALIDATION may be used to skip the SaferIdentifyLevel() validation
// which is going to ignore then if a script is signed
//

//
// Possible values for dwFlags for WldpGetLockdownPolicy API.
//

#define WLDP_FLAGS_SKIPSIGNATUREVALIDATION          (0x00000100)

//
// Enumeration types for WldpCanExecute{File,Stream,Buffer}
//
typedef enum WLDP_EXECUTION_POLICY {
    WLDP_EXECUTION_POLICY_BLOCKED,
    WLDP_EXECUTION_POLICY_ALLOWED,
    WLDP_EXECUTION_POLICY_REQUIRE_SANDBOX,
} WLDP_EXECUTION_POLICY;

typedef enum WLDP_EXECUTION_EVALUATION_OPTIONS {
    WLDP_EXECUTION_EVALUATION_OPTION_NONE = 0x0,
    WLDP_EXECUTION_EVALUATION_OPTION_EXECUTE_IN_INTERACTIVE_SESSION = 0x1,
} WLDP_EXECUTION_EVALUATION_OPTIONS;

DEFINE_ENUM_FLAG_OPERATORS(WLDP_EXECUTION_EVALUATION_OPTIONS);

// Batch Script Host, for example, cmd
// {5BAEA1D6-6F1C-488E-8490-347FA5C5067F}
EXTERN_GUID(WLDP_HOST_CMD,
0x5baea1d6, 0x6f1c, 0x488e, 0x84, 0x90, 0x34, 0x7f, 0xa5, 0xc5, 0x6, 0x7f);

// Powershell Script Host
// {8E9AAA7C-198B-4879-AE41-A50D47AD6458}
EXTERN_GUID(WLDP_HOST_POWERSHELL,
    0x8e9aaa7c, 0x198b, 0x4879, 0xae, 0x41, 0xa5, 0xd, 0x47, 0xad, 0x64, 0x58);

// Python Script Host
// {BFD557EF-2448-42EC-810B-0D9F09352D4A}
EXTERN_GUID(WLDP_HOST_PYTHON,
    0xbfd557ef, 0x2448, 0x42ec, 0x81, 0xb, 0xd, 0x9f, 0x9, 0x35, 0x2d, 0x4a);

// Windows Script Host, for example, cscript, wscript.
// {D30B84C5-29CE-4FF3-86EC-A30007A82E49}
EXTERN_GUID(WLDP_HOST_WINDOWS_SCRIPT_HOST,
    0xd30b84c5, 0x29ce, 0x4ff3, 0x86, 0xec, 0xa3, 0x0, 0x7, 0xa8, 0x2e, 0x49);

// Standalone Javascript Host, for example, nodejs
// {5629F0D5-1CCA-4FED-A1A3-36A8C18D74C0}
EXTERN_GUID(WLDP_HOST_JAVASCRIPT,
    0x5629f0d5, 0x1cca, 0x4fed, 0xa1, 0xa3, 0x36, 0xa8, 0xc1, 0x8d, 0x74, 0xc0);

// HTML Engine, for example, mshtml
// {B35A71B6-FE56-48D6-9543-2DFF0ECDED66}
EXTERN_GUID(WLDP_HOST_HTML,
    0xb35a71b6, 0xfe56, 0x48d6, 0x95, 0x43, 0x2d, 0xff, 0xe, 0xcd, 0xed, 0x66);

// XML Engine, for example, msxml
// {5594BE58-C6BF-4295-82F4-D494D20E3A36}
EXTERN_GUID(WLDP_HOST_XML,
    0x5594be58, 0xc6bf, 0x4295, 0x82, 0xf4, 0xd4, 0x94, 0xd2, 0xe, 0x3a, 0x36);

// Microsoft Standard Installer
// {624EB611-6E7E-4EEC-9BFE-F0ECDBFCF390}
EXTERN_GUID(WLDP_HOST_MSI,
    0x624eb611, 0x6e7e, 0x4eec, 0x9b, 0xfe, 0xf0, 0xec, 0xdb, 0xfc, 0xf3, 0x90);

// Catch-all for custom objects without a subject interface package
// {626CBEC3-E1FA-4227-9800-ED210274CF7C}
EXTERN_GUID(WLDP_HOST_OTHER,
    0x626cbec3, 0xe1fa, 0x4227, 0x98, 0x0, 0xed, 0x21, 0x2, 0x74, 0xcf, 0x7c);

#if NTDDI_VERSION >= NTDDI_WIN8

STDAPI
WldpGetLockdownPolicy(
    _In_opt_ PWLDP_HOST_INFORMATION hostInformation,
    _Out_ PDWORD lockdownState,
    _In_ DWORD lockdownFlags
    );

//
// Call the library to validate if a particular CLSID is safe to be called.
//

STDAPI
WldpIsClassInApprovedList(
    _In_ REFCLSID classID,
    _In_ PWLDP_HOST_INFORMATION hostInformation,
    _Out_ PBOOL isApproved,
    _In_ DWORD optionalFlags
    );

#endif /* NTDDI_VERSION >= NTDDI_WIN8 */

#if NTTDI_VERSION >= NTTDI_WIN10_RS1

//
// This routine queries secure setting of active Device Guard policies.
//

typedef struct _UNICODE_STRING  UNICODE_STRING, *PUNICODE_STRING;

//
// Secure setting types.
//

typedef enum WLDP_SECURE_SETTING_VALUE_TYPE
{
    WLDP_SECURE_SETTING_VALUE_TYPE_BOOLEAN = 0,
    WLDP_SECURE_SETTING_VALUE_TYPE_ULONG,
    WLDP_SECURE_SETTING_VALUE_TYPE_BINARY,
    WLDP_SECURE_SETTING_VALUE_TYPE_STRING
} WLDP_SECURE_SETTING_VALUE_TYPE, *PWLDP_SECURE_SETTING_VALUE_TYPE;

STDAPI
WldpQuerySecurityPolicy(
    _In_ const UNICODE_STRING * providerName,
    _In_ const UNICODE_STRING * keyName,
    _In_ const UNICODE_STRING * valueName,
    _Out_ PWLDP_SECURE_SETTING_VALUE_TYPE valueType,
    _Out_writes_bytes_opt_(*valueSize) PVOID valueAddress,
    _Inout_ PULONG valueSize
    );

typedef HRESULT(WINAPI *PWLDP_QUERYSECURITYPOLICY_API)(
    _In_ const UNICODE_STRING * providerName,
    _In_ const UNICODE_STRING * keyName,
    _In_ const UNICODE_STRING * valueName,
    _Out_ PWLDP_SECURE_SETTING_VALUE_TYPE valueType,
    _Out_writes_bytes_opt_(*valueSize) PVOID valueAddress,
    _Inout_ PULONG valueSize
    );

#endif /* NTTDI_VERSION >= NTTDI_WIN10_RS1 */

#if NTDDI_VERSION >= NTDDI_WIN10_RS4

//
// This routine sets trust claim for a dynamic code file.
// Caller should make the file exclusive access before calling the API.
//

STDAPI
WldpSetDynamicCodeTrust(
    _In_ HANDLE fileHandle
    );

typedef HRESULT(WINAPI *PWLDP_SETDYNAMICCODETRUST_API)(_In_ HANDLE hFileHandle);

//
// This routine gets dynamic code security policy enforcement status.
//

STDAPI
WldpIsDynamicCodePolicyEnabled(
    _Out_ PBOOL isEnabled
    );

typedef HRESULT(WINAPI *PWLDP_ISDYNAMICCODEPOLICYENABLED_API)(_Out_ PBOOL pbEnabled);

//
// This routine queries if in-memory or on-disk dynamic code is trusted 
// or not.
// 

STDAPI
WldpQueryDynamicCodeTrust(
    _When_(baseImage != NULL, _In_opt_)
    _When_(baseImage == NULL, _In_)
    HANDLE fileHandle,
    _When_(fileHandle == NULL, _In_reads_bytes_opt_(imageSize))
    _When_(fileHandle != NULL, _In_reads_bytes_(imageSize))
    PVOID baseImage,
    _In_ ULONG imageSize
    );

typedef HRESULT(WINAPI *PWLDP_QUERYDYNAMICODETRUST_API)(
    _When_(baseImage == NULL, _In_)
    HANDLE fileHandle,
    _When_(fileHandle == NULL, _In_reads_bytes_opt_(imageSize))
    _When_(fileHandle != NULL, _In_reads_bytes_(imageSize))
    PVOID baseImage,
    _In_ ULONG imageSize
    );

//
// This routine queries Windows Lockdown mode.
// 

HRESULT
WINAPI
WldpQueryWindowsLockdownMode(
    _Out_ PWLDP_WINDOWS_LOCKDOWN_MODE lockdownMode
    );

typedef HRESULT(WINAPI *PWLDP_QUERYWINDOWSLOCKDOWNMODE_API)(
    _Out_ PWLDP_WINDOWS_LOCKDOWN_MODE lockdownMode);

//
// This routine queries Device Security Information
//


STDAPI
WldpQueryDeviceSecurityInformation(
    _Out_writes_to_opt_(informationLength, *returnLength) PWLDP_DEVICE_SECURITY_INFORMATION information,
    _In_ DWORD informationLength,
    _Out_ DWORD* returnLength
    );

typedef HRESULT(WINAPI* PWLDP_QUERYDEVICESECURITYINFORMATION_API)(
    _Out_writes_to_opt_(informationLength, *returnLength) PWLDP_DEVICE_SECURITY_INFORMATION information,
    _In_ DWORD informationLength,
    _Out_ DWORD* returnLength);

//
// This routine queries CI lock down restriction. 
//

HRESULT
WINAPI
WldpQueryWindowsLockdownRestriction(
    _Out_ PWLDP_WINDOWS_LOCKDOWN_RESTRICTION LockdownRestriction
);

typedef HRESULT(WINAPI *PWLDP_QUERYWINDOWSLOCKDOWNRESTRICTION_API)(
    _Out_ PWLDP_WINDOWS_LOCKDOWN_RESTRICTION LockdownRestriction);

//
// This routine sets CI lock down restriction.
//

HRESULT
WINAPI
WldpSetWindowsLockdownRestriction(
    _In_ WLDP_WINDOWS_LOCKDOWN_RESTRICTION LockdownRestriction
    );

typedef HRESULT(WINAPI *PWLDP_SETWINDOWSLOCKDOWNRESTRICTION_API)(
    _In_ WLDP_WINDOWS_LOCKDOWN_RESTRICTION LockdownRestriction);

#endif /* NTDDI_VERSION >= NTDDI_WIN10_RS4 */

#if NTDDI_VERSION >= NTDDI_WIN10_RS5

//
// This routine queries if a particular PackageFamilyName would pass the currently installed CIPolicy
//
HRESULT
WINAPI
WldpIsAppApprovedByPolicy(
    _In_ PCWSTR PackageFamilyName,
    _In_ ULONGLONG PackageVersion
);

typedef HRESULT(WINAPI *PWLDP_ISAPPAPPROVEDBYPOLICY_API)(
    _In_ PCWSTR PackageFamilyName,
    _In_ ULONGLONG PackageVersion
    );

#endif /* NTDDI_VERSION >= NTDDI_WIN10_RS5 */

#if NTDDI_VERSION >= NTDDI_WIN10_MN

HRESULT
WINAPI
WldpQueryPolicySettingEnabled(
    _In_ WLDP_POLICY_SETTING Setting,
    _Out_ PBOOL Enabled
);

typedef HRESULT(WINAPI *PWLDP_QUERYPOLICYSETTINGENABLED_API)(
    _In_ WLDP_POLICY_SETTING Setting,
    _Out_ PBOOL Enabled
    );

HRESULT
WINAPI
WldpQueryPolicySettingEnabled2(
    _In_ PCWSTR SettingString,
    _Out_ PBOOL Enabled
);

typedef HRESULT(WINAPI *PWLDP_QUERYPOLICYSETTINGENABLED2_API)(
    _In_ PCWSTR Setting,
    _Out_ PBOOL Enabled
    );

//
//    This function checks the security watermark state of 10x system
//
HRESULT
WINAPI
WldpIsWcosProductionConfiguration(
    _Out_ PBOOL IsProductionConfiguration
);

typedef HRESULT(WINAPI *PWLDP_ISWCOSPRODUCTIONCONFIGURATION_API)(_Out_ PBOOL IsProductionConfiguration);

HRESULT
WINAPI
WldpResetWcosProductionConfiguration();

typedef HRESULT(WINAPI *PWLDP_RESETWCOSPRODUCTIONCONFIGURATION_API)();

//
//    This function checks the security configuration state of the non-10x system
//
HRESULT
WINAPI
WldpIsProductionConfiguration(
    _Out_ PBOOL IsProductionConfiguration
);

typedef HRESULT(WINAPI *PWLDP_ISPRODUCTIONCONFIGURATION_API)(_Out_ PBOOL IsProductionConfiguration);

HRESULT
WINAPI
WldpResetProductionConfiguration(VOID);

typedef HRESULT(WINAPI *PWLDP_RESETPRODUCTIONCONFIGURATION_API)(VOID);

#endif /* NTDDI_VERSION >= NTDDI_WIN10_MN */

#if NTDDI_VERSION >= NTDDI_WIN10_NI

STDAPI
WldpCanExecuteFile(
    _In_ REFGUID host,
    _In_ WLDP_EXECUTION_EVALUATION_OPTIONS options,
    _In_ HANDLE fileHandle,
    _In_opt_ PCWSTR auditInfo,
    _Out_ WLDP_EXECUTION_POLICY *result
);

typedef HRESULT(WINAPI *PWLDP_CANEXECUTEFILE_API)(
    _In_ REFGUID host,
    _In_ WLDP_EXECUTION_EVALUATION_OPTIONS options,
    _In_ HANDLE fileHandle,
    _In_opt_ PCWSTR auditInfo,
    _Out_ WLDP_EXECUTION_POLICY *result
);

STDAPI
WldpCanExecuteBuffer(
    _In_ REFGUID host,
    _In_ WLDP_EXECUTION_EVALUATION_OPTIONS options,
    _In_reads_(bufferSize) const BYTE *buffer,
    _In_ ULONG bufferSize,
    _In_opt_ PCWSTR auditInfo,
    _Out_ WLDP_EXECUTION_POLICY *result
);

typedef HRESULT(WINAPI *PWLDP_CANEXECUTEBUFFER_API)(
    _In_ REFGUID host,
    _In_ WLDP_EXECUTION_EVALUATION_OPTIONS options,
    _In_reads_(bufferSize) const BYTE *buffer,
    _In_ ULONG bufferSize,
    _In_opt_ PCWSTR auditInfo,
    _Out_ WLDP_EXECUTION_POLICY *result
);

STDAPI
WldpCanExecuteStream(
    _In_ REFGUID host,
    _In_ WLDP_EXECUTION_EVALUATION_OPTIONS options,
    _In_ IStream *stream,
    _In_opt_ PCWSTR auditInfo,
    _Out_ WLDP_EXECUTION_POLICY *result
);

typedef HRESULT(WINAPI *PWLDP_CANEXECUTESTREAM_API)(
    _In_ REFGUID host,
    _In_ WLDP_EXECUTION_EVALUATION_OPTIONS options,
    _In_ IStream *stream,
    _In_opt_ PCWSTR auditInfo,
    _Out_ WLDP_EXECUTION_POLICY *result
);

#endif /* NTDDI_VERSION >= NTDDI_WIN10_NI */

#if NTDDI_VERSION >= NTDDI_WIN10_CU

STDAPI
WldpCanExecuteFileFromDetachedSignature(
    _In_ REFGUID host,
    _In_ WLDP_EXECUTION_EVALUATION_OPTIONS options,
    _In_ HANDLE contentFileHandle,
    _In_ HANDLE signatureFileHandle,
    _In_opt_ PCWSTR auditInfo,
    _Out_ WLDP_EXECUTION_POLICY* result
);

typedef HRESULT(WINAPI *PWLDP_CANEXECUTEFILEFROMDETACHEDSIGNATURE_API)(
    _In_ REFGUID host,
    _In_ WLDP_EXECUTION_EVALUATION_OPTIONS options,
    _In_ HANDLE contentFileHandle,
    _In_ HANDLE signatureFileHandle,
    _In_opt_ PCWSTR auditInfo,
    _Out_ WLDP_EXECUTION_POLICY* result
);

#endif /* NTDDI_VERSION >= NTDDI_WIN10_CU */

#if NTDDI_VERSION >= NTDDI_WIN11_GA

STDAPI
WldpGetApplicationSettingBoolean(
    _In_ PCWSTR id,
    _In_ PCWSTR setting,
    _Out_ BOOL *result
    );

typedef HRESULT(WINAPI *PWLDP_GETAPPLICATIONSETTINGBOOLEAN_API)(
    _In_ PCWSTR id,
    _In_ PCWSTR setting,
    _Out_ BOOL *result
    );

STDAPI
WldpGetApplicationSettingStringList(
    _In_ PCWSTR id,
    _In_ PCWSTR setting,
    _In_ SIZE_T dataCount,
    _Out_ SIZE_T *requiredCount,
    _Out_writes_to_opt_(dataCount, *requiredCount) PZZWSTR result
    );

typedef HRESULT(WINAPI *PWLDP_GETAPPLICATIONSETTINGSTRINGLIST_API)(
    _In_ PCWSTR id,
    _In_ PCWSTR setting,
    _In_ SIZE_T dataCount,
    _Out_ SIZE_T *requiredCount,
    _Out_writes_to_opt_(dataCount, *requiredCount) PZZWSTR result
    );

STDAPI
WldpGetApplicationSettingStringSet(
    _In_ PCWSTR id,
    _In_ PCWSTR setting,
    _In_ SIZE_T dataCount,
    _Out_ SIZE_T *requiredCount,
    _Out_writes_to_opt_(dataCount, *requiredCount) PZZWSTR result
    );

typedef HRESULT(WINAPI *PWLDP_GETAPPLICATIONSETTINGSTRINGSET_API)(
    _In_ PCWSTR id,
    _In_ PCWSTR setting,
    _In_ SIZE_T dataCount,
    _Out_ SIZE_T *requiredCount,
    _Out_writes_to_opt_(dataCount, *requiredCount) PZZWSTR result
    );

#endif /* NTDDI_VERSION >= NTDDI_WIN11_GA */

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif /* !_WLDP_H_INCLUDED_ */

