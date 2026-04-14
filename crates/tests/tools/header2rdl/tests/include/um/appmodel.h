/***************************************************
*                                                  *
*   Copyright (C) Microsoft. All rights reserved.  *
*                                                  *
***************************************************/
//
// API Set Contract:
//
//    api-ms-win-appmodel-runtime-l1
//
// Abstract:
//
//    This header file provides API function signatures and
//    corollary type declarations for the Windows AppModel
//    Runtime component.
//
////

#if defined(_MSC_VER)
#if _MSC_VER > 1000
#pragma once
#endif // _MSC_VER > 1000
#endif // defined(_MSC_VER)

#ifndef _APPMODEL_H_
#define _APPMODEL_H_

#if defined(_CONTRACT_GEN)
#include <nt.h>
#include <ntrtl.h>
#include <nturtl.h>
#include <windows.h>
#endif // defined(_CONTRACT_GEN)

#if defined(_MSC_VER)
#if _MSC_VER >= 1200
#pragma warning(push)
#endif // _MSC_VER >= 1200
#pragma warning(disable:4201) /* nonstandard extension used : nameless struct/union */
#endif // defined(_MSC_VER)

#include <minappmodel.h>

#if defined(__cplusplus)
extern "C" {
#endif // defined(__cplusplus)

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)

/* ---------------------------------------------------------------- */

// Identity Types
#include <pshpack4.h>

typedef struct PACKAGE_VERSION {
    union {
        UINT64 Version;
        struct {
            USHORT Revision;
            USHORT Build;
            USHORT Minor;
            USHORT Major;
        } DUMMYSTRUCTNAME;
    } DUMMYUNIONNAME;
} PACKAGE_VERSION;

typedef struct PACKAGE_ID {
    UINT32          reserved;
    UINT32          processorArchitecture;
    PACKAGE_VERSION version;
    PWSTR           name;
    PWSTR           publisher;
    PWSTR           resourceId;
    PWSTR           publisherId;
} PACKAGE_ID;

#include <poppack.h>

// Identity Functions

WINBASEAPI
_Success_(return == ERROR_SUCCESS)
LONG
WINAPI
GetCurrentPackageId(
    _Inout_ UINT32* bufferLength,
    _Out_writes_bytes_opt_(*bufferLength) BYTE* buffer
    );

WINBASEAPI
_Success_(return == ERROR_SUCCESS)
LONG
WINAPI
GetCurrentPackageFullName(
    _Inout_ UINT32* packageFullNameLength,
    _Out_writes_opt_(*packageFullNameLength) PWSTR packageFullName
    );

WINBASEAPI
_Success_(return == ERROR_SUCCESS)
LONG
WINAPI
GetCurrentPackageFamilyName(
    _Inout_ UINT32* packageFamilyNameLength,
    _Out_writes_opt_(*packageFamilyNameLength) PWSTR packageFamilyName
    );

WINBASEAPI
_Success_(return == ERROR_SUCCESS)
LONG
WINAPI
GetCurrentPackagePath(
    _Inout_ UINT32* pathLength,
    _Out_writes_opt_(*pathLength) PWSTR path
    );

WINBASEAPI
_Success_(return == ERROR_SUCCESS)
LONG
WINAPI
GetPackageId(
    _In_ HANDLE hProcess,
    _Inout_ UINT32* bufferLength,
    _Out_writes_bytes_opt_(*bufferLength) BYTE* buffer
    );

WINBASEAPI
_Success_(return == ERROR_SUCCESS)
LONG
WINAPI
GetPackageFullName(
    _In_ HANDLE hProcess,
    _Inout_ UINT32* packageFullNameLength,
    _Out_writes_opt_(*packageFullNameLength) PWSTR packageFullName
    );

//TODO:8645770 Change 0x0101 to 0x0102 once THRESHOLD constants are available
WINBASEAPI
_Check_return_
_Success_(return == ERROR_SUCCESS)
LONG
WINAPI
GetPackageFullNameFromToken(
    _In_ HANDLE token,
    _Inout_ UINT32* packageFullNameLength,
    _Out_writes_opt_(*packageFullNameLength) PWSTR packageFullName
    );

WINBASEAPI
_Success_(return == ERROR_SUCCESS)
LONG
WINAPI
GetPackageFamilyName(
    _In_ HANDLE hProcess,
    _Inout_ UINT32* packageFamilyNameLength,
    _Out_writes_opt_(*packageFamilyNameLength) PWSTR packageFamilyName
    );

//TODO:8645770 Change 0x0101 to 0x0102 once THRESHOLD constants are available
WINBASEAPI
_Check_return_
_Success_(return == ERROR_SUCCESS)
LONG
WINAPI
GetPackageFamilyNameFromToken(
    _In_ HANDLE token,
    _Inout_ UINT32* packageFamilyNameLength,
    _Out_writes_opt_(*packageFamilyNameLength) PWSTR packageFamilyName
    );

WINBASEAPI
_Success_(return == ERROR_SUCCESS)
LONG
WINAPI
GetPackagePath(
    _In_ const PACKAGE_ID* packageId,
    _Reserved_ const UINT32 reserved,
    _Inout_ UINT32* pathLength,
    _Out_writes_opt_(*pathLength) PWSTR path
    );

WINBASEAPI
_Success_(return == ERROR_SUCCESS)
LONG
WINAPI
GetPackagePathByFullName(
    _In_ PCWSTR packageFullName,
    _Inout_ UINT32* pathLength,
    _Out_writes_opt_(*pathLength) PWSTR path
    );

WINBASEAPI
_Success_(return == ERROR_SUCCESS)
LONG
WINAPI
GetStagedPackagePathByFullName(
    _In_ PCWSTR packageFullName,
    _Inout_ UINT32* pathLength,
    _Out_writes_opt_(*pathLength) PWSTR path
    );

#if NTDDI_VERSION >= NTDDI_WIN10_19H1
typedef enum PackagePathType
{
    PackagePathType_Install = 0,
    PackagePathType_Mutable = 1,
    PackagePathType_Effective = 2,

#if NTDDI_VERSION >= NTDDI_WIN10_VB
    PackagePathType_MachineExternal = 3,
    PackagePathType_UserExternal = 4,
    PackagePathType_EffectiveExternal = 5
#endif // NTDDI_VERSION >= NTDDI_WIN10_VB
} PackagePathType;

WINBASEAPI
_Success_(return == ERROR_SUCCESS)
LONG
WINAPI
GetPackagePathByFullName2(
    _In_ PCWSTR packageFullName,
    _In_ PackagePathType packagePathType,
    _Inout_ UINT32* pathLength,
    _Out_writes_opt_(*pathLength) PWSTR path
    );

WINBASEAPI
_Success_(return == ERROR_SUCCESS)
LONG
WINAPI
GetStagedPackagePathByFullName2(
    _In_ PCWSTR packageFullName,
    _In_ PackagePathType packagePathType,
    _Inout_ UINT32* pathLength,
    _Out_writes_opt_(*pathLength) PWSTR path
    );

WINBASEAPI
_Success_(return == ERROR_SUCCESS)
LONG
WINAPI
GetCurrentPackageInfo2(
    _In_ const UINT32 flags,
    _In_ PackagePathType packagePathType,
    _Inout_ UINT32* bufferLength,
    _Out_writes_bytes_opt_(*bufferLength) BYTE* buffer,
    _Out_opt_ UINT32* count
    );

WINBASEAPI
_Success_(return == ERROR_SUCCESS)
LONG
WINAPI
GetCurrentPackagePath2(
    _In_ PackagePathType packagePathType,
    _Inout_ UINT32* pathLength,
    _Out_writes_opt_(*pathLength) PWSTR path
    );

#endif // NTDDI_VERSION >= NTDDI_WIN10_19H1

/* ---------------------------------------------------------------- */

// Application Identity Functions

WINBASEAPI
_Success_(return == ERROR_SUCCESS)
LONG
WINAPI
GetCurrentApplicationUserModelId(
    _Inout_ UINT32* applicationUserModelIdLength,
    _Out_writes_opt_(*applicationUserModelIdLength) PWSTR applicationUserModelId
    );

WINBASEAPI
_Success_(return == ERROR_SUCCESS)
LONG
WINAPI
GetApplicationUserModelId(
    _In_ HANDLE hProcess,
    _Inout_ UINT32* applicationUserModelIdLength,
    _Out_writes_opt_(*applicationUserModelIdLength) PWSTR applicationUserModelId
    );

//TODO:8645770 Change 0x0101 to 0x0102 once THRESHOLD constants are available
WINBASEAPI
_Check_return_
_Success_(return == ERROR_SUCCESS)
LONG
WINAPI
GetApplicationUserModelIdFromToken(
    _In_ HANDLE token,
    _Inout_ UINT32* applicationUserModelIdLength,
    _Out_writes_opt_(*applicationUserModelIdLength) PWSTR applicationUserModelId
    );

/* ---------------------------------------------------------------- */

// Verification Functions

//TODO:8645770 Change 0x0101 to 0x0102 once THRESHOLD constants are available
WINBASEAPI
_Check_return_
_Success_(return == ERROR_SUCCESS)
LONG
WINAPI
VerifyPackageFullName(
    _In_ PCWSTR packageFullName
    );

WINBASEAPI
_Check_return_
_Success_(return == ERROR_SUCCESS)
LONG
WINAPI
VerifyPackageFamilyName(
    _In_ PCWSTR packageFamilyName
    );

WINBASEAPI
_Check_return_
_Success_(return == ERROR_SUCCESS)
LONG
WINAPI
VerifyPackageId(
    _In_ const PACKAGE_ID* packageId
    );

WINBASEAPI
_Check_return_
_Success_(return == ERROR_SUCCESS)
LONG
WINAPI
VerifyApplicationUserModelId(
    _In_ PCWSTR applicationUserModelId
    );

WINBASEAPI
_Check_return_
_Success_(return == ERROR_SUCCESS)
LONG
WINAPI
VerifyPackageRelativeApplicationId(
    _In_ PCWSTR packageRelativeApplicationId
    );

/* ---------------------------------------------------------------- */

// Conversion Functions

WINBASEAPI
_Check_return_
_Success_(return == ERROR_SUCCESS)
LONG
WINAPI
PackageIdFromFullName(
    _In_ PCWSTR packageFullName,
    _In_ const UINT32 flags,
    _Inout_ UINT32* bufferLength,
    _Out_writes_bytes_opt_(*bufferLength) BYTE* buffer
    );

WINBASEAPI
_Check_return_
_Success_(return == ERROR_SUCCESS)
LONG
WINAPI
PackageFullNameFromId(
    _In_ const PACKAGE_ID* packageId,
    _Inout_ UINT32* packageFullNameLength,
    _Out_writes_opt_(*packageFullNameLength) PWSTR packageFullName
    );

WINBASEAPI
_Check_return_
_Success_(return == ERROR_SUCCESS)
LONG
WINAPI
PackageFamilyNameFromId(
    _In_ const PACKAGE_ID* packageId,
    _Inout_ UINT32* packageFamilyNameLength,
    _Out_writes_opt_(*packageFamilyNameLength) PWSTR packageFamilyName
    );

WINBASEAPI
_Check_return_
_Success_(return == ERROR_SUCCESS)
LONG
WINAPI
PackageFamilyNameFromFullName(
    _In_ PCWSTR packageFullName,
    _Inout_ UINT32* packageFamilyNameLength,
    _Out_writes_opt_(*packageFamilyNameLength) PWSTR packageFamilyName
    );

WINBASEAPI
_Check_return_
_Success_(return == ERROR_SUCCESS)
LONG
WINAPI
PackageNameAndPublisherIdFromFamilyName(
    _In_ PCWSTR packageFamilyName,
    _Inout_ UINT32* packageNameLength,
    _Out_writes_opt_(*packageNameLength) PWSTR packageName,
    _Inout_ UINT32* packagePublisherIdLength,
    _Out_writes_opt_(*packagePublisherIdLength) PWSTR packagePublisherId
    );

WINBASEAPI
_Check_return_
_Success_(return == ERROR_SUCCESS)
LONG
WINAPI
FormatApplicationUserModelId(
    _In_ PCWSTR packageFamilyName,
    _In_ PCWSTR packageRelativeApplicationId,
    _Inout_ UINT32* applicationUserModelIdLength,
    _Out_writes_opt_(*applicationUserModelIdLength) PWSTR applicationUserModelId
    );

WINBASEAPI
_Check_return_
_Success_(return == ERROR_SUCCESS)
LONG
WINAPI
ParseApplicationUserModelId(
    _In_ PCWSTR applicationUserModelId,
    _Inout_ UINT32* packageFamilyNameLength,
    _Out_writes_opt_(*packageFamilyNameLength) PWSTR packageFamilyName,
    _Inout_ UINT32* packageRelativeApplicationIdLength,
    _Out_writes_opt_(*packageRelativeApplicationIdLength) PWSTR packageRelativeApplicationId
    );

/* ---------------------------------------------------------------- */

// Lookup Functions

WINBASEAPI
_Check_return_
_Success_(return == ERROR_SUCCESS)
_On_failure_(_Unchanged_(*count))
_On_failure_(_Unchanged_(*bufferLength))
LONG
WINAPI
GetPackagesByPackageFamily(
    _In_ PCWSTR packageFamilyName,
    _Inout_ UINT32* count,
    _Out_writes_opt_(*count) PWSTR* packageFullNames,
    _Inout_ UINT32* bufferLength,
    _Out_writes_opt_(*bufferLength) WCHAR* buffer
    );

/* Any combination of PACKAGE_FILTER_* */
WINBASEAPI
_Check_return_
_Success_(return == ERROR_SUCCESS)
_On_failure_(_Unchanged_(*count))
_On_failure_(_Unchanged_(*bufferLength))
LONG
WINAPI
FindPackagesByPackageFamily(
    _In_ PCWSTR packageFamilyName,
    _In_ UINT32 packageFilters,
    _Inout_ UINT32* count,
    _Out_writes_opt_(*count) PWSTR* packageFullNames,
    _Inout_ UINT32* bufferLength,
    _Out_writes_opt_(*bufferLength) WCHAR* buffer,
    _Out_writes_opt_(*count) UINT32* packageProperties
    );

typedef enum PackageOrigin
{
    PackageOrigin_Unknown           = 0,
    PackageOrigin_Unsigned          = 1,
    PackageOrigin_Inbox             = 2,
    PackageOrigin_Store             = 3,
    PackageOrigin_DeveloperUnsigned = 4,
    PackageOrigin_DeveloperSigned   = 5,
    PackageOrigin_LineOfBusiness    = 6
} PackageOrigin;

WINBASEAPI
_Check_return_
_Success_(return == ERROR_SUCCESS)
LONG
WINAPI
GetStagedPackageOrigin(
    _In_ PCWSTR packageFullName,
    _Out_ PackageOrigin* origin
    );

/* ---------------------------------------------------------------- */

// Package Constants

#define PACKAGE_PROPERTY_FRAMEWORK          0x00000001
#define PACKAGE_PROPERTY_RESOURCE           0x00000002
#define PACKAGE_PROPERTY_BUNDLE             0x00000004
#define PACKAGE_PROPERTY_OPTIONAL           0x00000008
#define PACKAGE_FILTER_HEAD                 0x00000010
#define PACKAGE_FILTER_DIRECT               0x00000020
#define PACKAGE_FILTER_RESOURCE             0x00000040
#define PACKAGE_FILTER_BUNDLE               0x00000080
#define PACKAGE_INFORMATION_BASIC           0x00000000
#define PACKAGE_INFORMATION_FULL            0x00000100
#define PACKAGE_PROPERTY_DEVELOPMENT_MODE   0x00010000
#define PACKAGE_FILTER_OPTIONAL             0x00020000
#define PACKAGE_PROPERTY_IS_IN_RELATED_SET  0x00040000
#define PACKAGE_FILTER_IS_IN_RELATED_SET    PACKAGE_PROPERTY_IS_IN_RELATED_SET
#define PACKAGE_PROPERTY_STATIC             0x00080000
#define PACKAGE_FILTER_STATIC               PACKAGE_PROPERTY_STATIC
#define PACKAGE_PROPERTY_DYNAMIC            0x00100000
#define PACKAGE_FILTER_DYNAMIC              PACKAGE_PROPERTY_DYNAMIC
#if NTDDI_VERSION >= NTDDI_WIN10_MN
#define PACKAGE_PROPERTY_HOSTRUNTIME        0x00200000
#define PACKAGE_FILTER_HOSTRUNTIME          PACKAGE_PROPERTY_HOSTRUNTIME
#endif // NTDDI_VERSION >= NTDDI_WIN10_MN

#if defined(NTDDI_VERSION) && (NTDDI_VERSION >= NTDDI_WINBLUE)
#pragma deprecated("PACKAGE_FILTER_ALL_LOADED")
#endif // defined(NTDDI_VERSION) && (NTDDI_VERSION >= NTDDI_WINBLUE)
// Use PACKAGE_FILTER_HEAD|PACKAGE_FILTER_DIRECT instead of PACKAGE_FILTER_ALL_LOADED
#define PACKAGE_FILTER_ALL_LOADED   0

// Dependency Types

typedef struct _PACKAGE_INFO_REFERENCE {
    void * reserved;
} * PACKAGE_INFO_REFERENCE;

#include <pshpack4.h>

typedef struct PACKAGE_INFO {
    UINT32     reserved;
    UINT32     flags;
    PWSTR      path;
    PWSTR      packageFullName;
    PWSTR      packageFamilyName;
    PACKAGE_ID packageId;
} PACKAGE_INFO;

#include <poppack.h>

// Dependency Functions

WINBASEAPI
_Success_(return == ERROR_SUCCESS)
LONG
WINAPI
GetCurrentPackageInfo(
    _In_ const UINT32 flags,
    _Inout_ UINT32* bufferLength,
    _Out_writes_bytes_opt_(*bufferLength) BYTE* buffer,
    _Out_opt_ UINT32* count
    );

WINBASEAPI
_Success_(return == ERROR_SUCCESS)
LONG
WINAPI
OpenPackageInfoByFullName(
    _In_ PCWSTR packageFullName,
    _Reserved_ const UINT32 reserved,
    _Out_ PACKAGE_INFO_REFERENCE* packageInfoReference
    );

//TODO:8645770 Change 0x0101 to 0x0102 once THRESHOLD constants are available
WINBASEAPI
_Check_return_
_Success_(return == ERROR_SUCCESS)
LONG
WINAPI
OpenPackageInfoByFullNameForUser(
    _In_opt_ PSID userSid,
    _In_ PCWSTR packageFullName,
    _Reserved_ const UINT32 reserved,
    _Out_ PACKAGE_INFO_REFERENCE* packageInfoReference
    );

WINBASEAPI
_Success_(return == ERROR_SUCCESS)
LONG
WINAPI
ClosePackageInfo(
    _In_ PACKAGE_INFO_REFERENCE packageInfoReference
    );

WINBASEAPI
_Success_(return == ERROR_SUCCESS)
LONG
WINAPI
GetPackageInfo(
    _In_ PACKAGE_INFO_REFERENCE packageInfoReference,
    _In_ const UINT32 flags,
    _Inout_ UINT32* bufferLength,
    _Out_writes_bytes_opt_(*bufferLength) BYTE* buffer,
    _Out_opt_ UINT32* count
    );

WINBASEAPI
_Success_(return == ERROR_SUCCESS)
LONG
WINAPI
GetPackageApplicationIds(
    _In_ PACKAGE_INFO_REFERENCE packageInfoReference,
    _Inout_ UINT32* bufferLength,
    _Out_writes_bytes_opt_(*bufferLength) BYTE* buffer,
    _Out_opt_ UINT32* count
    );

#if NTDDI_VERSION >= NTDDI_WIN10_19H1
WINBASEAPI
_Success_(return == ERROR_SUCCESS)
LONG
WINAPI
GetPackageInfo2(
    _In_ PACKAGE_INFO_REFERENCE packageInfoReference,
    _In_ const UINT32 flags,
    _In_ PackagePathType packagePathType,
    _Inout_ UINT32* bufferLength,
    _Out_writes_bytes_opt_(*bufferLength) BYTE* buffer,
    _Out_opt_ UINT32* count
    );

#endif // NTDDI_VERSION >= NTDDI_WIN10_19H1

WINBASEAPI
_Check_return_
HRESULT
WINAPI
CheckIsMSIXPackage(
    _In_ PCWSTR packageFullName,
    _Out_ BOOL* isMSIXPackage
    );

/* ---------------------------------------------------------------- */

// Dynamic Dependencies

#if NTDDI_VERSION >= NTDDI_WIN10_CO

typedef enum CreatePackageDependencyOptions
{
    CreatePackageDependencyOptions_None = 0,

    /// Disable dependency resolution when pinning a package dependency.
    CreatePackageDependencyOptions_DoNotVerifyDependencyResolution = 0x00000001,

     /// Define the package dependency for the system, accessible to all users
     /// (default is the package dependency is defined for a specific user).
     /// This option requires the caller has adminitrative privileges.
    CreatePackageDependencyOptions_ScopeIsSystem = 0x00000002,
} CreatePackageDependencyOptions;
DEFINE_ENUM_FLAG_OPERATORS(CreatePackageDependencyOptions)

typedef enum PackageDependencyLifetimeKind
{
    /// The current process is the lifetime artifact. The package dependency
    /// is implicitly deleted when the process terminates.
    PackageDependencyLifetimeKind_Process = 0,

    /// The lifetime artifact is an absolute filename or path.
    /// The package dependency is implicitly deleted when this is deleted.
    PackageDependencyLifetimeKind_FilePath = 1,

    /// The lifetime artifact is a registry key in the format
    /// 'root\\subkey' where root is one of the following: HKLM, HKCU, HKCR, HKU.
    /// The package dependency is implicitly deleted when this is deleted.
    PackageDependencyLifetimeKind_RegistryKey = 2,
} PackageDependencyLifetimeKind;

typedef enum AddPackageDependencyOptions
{
    AddPackageDependencyOptions_None                   = 0,
    AddPackageDependencyOptions_PrependIfRankCollision = 0x00000001,
} AddPackageDependencyOptions;
DEFINE_ENUM_FLAG_OPERATORS(AddPackageDependencyOptions)

#if NTDDI_VERSION >= NTDDI_WIN10_GA
typedef enum AddPackageDependencyOptions2
{
    AddPackageDependencyOptions2_None                       = 0,
    AddPackageDependencyOptions2_PrependIfRankCollision     = 0x00000001,
    AddPackageDependencyOptions2_SpecifiedPackageFamilyOnly = 0x00000002,
} AddPackageDependencyOptions2;
DEFINE_ENUM_FLAG_OPERATORS(AddPackageDependencyOptions2)
#endif // NTDDI_VERSION >= NTDDI_WIN10_GA

#define PACKAGE_DEPENDENCY_RANK_DEFAULT 0

typedef enum PackageDependencyProcessorArchitectures
{
    PackageDependencyProcessorArchitectures_None    = 0,
    PackageDependencyProcessorArchitectures_Neutral = 0x00000001,
    PackageDependencyProcessorArchitectures_X86     = 0x00000002,
    PackageDependencyProcessorArchitectures_X64     = 0x00000004,
    PackageDependencyProcessorArchitectures_Arm     = 0x00000008,
    PackageDependencyProcessorArchitectures_Arm64   = 0x00000010,
    PackageDependencyProcessorArchitectures_X86A64  = 0x00000020,
} PackageDependencyProcessorArchitectures;
DEFINE_ENUM_FLAG_OPERATORS(PackageDependencyProcessorArchitectures)

#if !defined(__PACKAGEDEPENDENCY_CONTEXT_DEFINED__)
#define __PACKAGEDEPENDENCY_CONTEXT_DEFINED__
DECLARE_HANDLE(PACKAGEDEPENDENCY_CONTEXT);
#endif // !defined(__PACKAGEDEPENDENCY_CONTEXT_DEFINED__)

/// Define a package dependency. The criteria for a PackageDependency
/// (package family name, minimum version, etc)
/// may match multiple packages, but ensures Deployment won't remove
/// a package if it's the only one satisfying the PackageDependency.
///
/// @note A package matching a PackageDependency pin can still be removed
///       as long as there's another package that satisfies the PackageDependency.
///       For example, if Fwk-v1 is installed and a PackageDependency specifies
///       MinVersion=1 and then Fwk-v2 is installed, Deployment could remove
///       Fwk-v1 because Fwk-v2 will satisfy the PackageDependency. After Fwk-v1
///       is removed Deployment won't remove Fwk-v2 because it's the only package
///       satisfying the PackageDependency. Thus Fwk-v1 and Fwk-v2 (and any other
///       package matching the PackageDependency) are 'loosely pinned'. Deployment
///       guarantees it won't remove a package if it would make a PackageDependency
///       unsatisfied.
///
/// A PackageDependency specifies criteria (package family, minimum version, etc)
/// and not a specific package. Deployment reserves the right to use a different
/// package (e.g. higher version) to satisfy the PackageDependency if/when
/// one becomes available.
///
/// @param user the user scope of the package dependency. If NULL the caller's
///        user context is used. MUST be NULL if CreatePackageDependencyOptions_ScopeIsSystem
///        is specified
/// @param lifetimeArtifact MUST be NULL if lifetimeKind=PackageDependencyLifetimeKind_Process
/// @param packageDependencyId allocated via HeapAlloc; use HeapFree to deallocate
///
/// @note TryCreatePackageDependency() fails if the PackageDependency cannot be resolved to a specific
///       package. This package resolution check is skipped if
///       CreatePackageDependencyOptions_DoNotVerifyDependencyResolution is specified. This is useful
///       for installers running as user contexts other than the target user (e.g. installers
///       running as LocalSystem).
/// @see TryCreatePackageDependency2
WINBASEAPI
HRESULT
WINAPI
TryCreatePackageDependency(
    PSID user,
    _In_ PCWSTR packageFamilyName,
    PACKAGE_VERSION minVersion,
    PackageDependencyProcessorArchitectures packageDependencyProcessorArchitectures,
    PackageDependencyLifetimeKind lifetimeKind,
    PCWSTR lifetimeArtifact,
    CreatePackageDependencyOptions options,
    _Outptr_result_maybenull_ PWSTR* packageDependencyId
    );

#if NTDDI_VERSION >= NTDDI_WIN11_GA

/// Define a package dependency. The criteria for a PackageDependency
/// (package family name, minimum version, etc)
/// may match multiple packages, but ensures Deployment won't remove
/// a package if it's the only one satisfying the PackageDependency.
///
/// @note A package matching a PackageDependency pin can still be removed
///       as long as there's another package that satisfies the PackageDependency.
///       For example, if Fwk-v1 is installed and a PackageDependency specifies
///       MinVersion=1 and then Fwk-v2 is installed, Deployment could remove
///       Fwk-v1 because Fwk-v2 will satisfy the PackageDependency. After Fwk-v1
///       is removed Deployment won't remove Fwk-v2 because it's the only package
///       satisfying the PackageDependency. Thus Fwk-v1 and Fwk-v2 (and any other
///       package matching the PackageDependency) are 'loosely pinned'. Deployment
///       guarantees it won't remove a package if it would make a PackageDependency
///       unsatisfied.
///
/// A PackageDependency specifies criteria (package family, minimum version, etc)
/// and not a specific package. Deployment reserves the right to use a different
/// package (e.g. higher version) to satisfy the PackageDependency if/when
/// one becomes available.
///
/// @param user the user scope of the package dependency. If NULL the caller's
///        user context is used. MUST be NULL if CreatePackageDependencyOptions_ScopeIsSystem
///        is specified
/// @param lifetimeArtifact MUST be NULL if lifetimeKind=PackageDependencyLifetimeKind_Process
/// @param packageDependencyId allocated via HeapAlloc; use HeapFree to deallocate
///
/// @note TryCreatePackageDependency2() fails if the PackageDependency cannot be resolved to a specific
///       package. This package resolution check is skipped if
///       CreatePackageDependencyOptions_DoNotVerifyDependencyResolution is specified. This is useful
///       for installers running as user contexts other than the target user (e.g. installers
///       running as LocalSystem).
/// @see TryCreatePackageDependency
WINBASEAPI
HRESULT
WINAPI
TryCreatePackageDependency2(
    PSID user,
    _In_ PCWSTR packageFamilyName,
    PACKAGE_VERSION minVersion,
    PackageDependencyProcessorArchitectures packageDependencyProcessorArchitectures,
    PackageDependencyLifetimeKind lifetimeKind,
    PCWSTR lifetimeArtifact,
    CreatePackageDependencyOptions options,
    const FILETIME* lifetimeExpiration,
    _Outptr_result_maybenull_ PWSTR* packageDependencyId
    );

#endif // NTDDI_VERSION >= NTDDI_WIN11_GA

/// Undefine a package dependency. Removing a pin on a PackageDependency is typically done at uninstall-time.
/// This implicitly occurs if the package dependency's 'lifetime artifact' (specified via TryCreatePackageDependency)
/// is deleted. Packages that are not referenced by other packages and have no pins are elegible to be removed.
///
/// @warn DeletePackageDependency() requires the caller have administrative privileges
///       if the package dependency was pinned with CreatePackageDependencyOptions_ScopeIsSystem.
WINBASEAPI
HRESULT
WINAPI
DeletePackageDependency(
    _In_ PCWSTR packageDependencyId
    );

/// Resolve a previously-pinned PackageDependency to a specific package and
/// add it to the invoking process' package graph. Once the dependency has
/// been added other code-loading methods (LoadLibrary, CoCreateInstance, etc)
/// can find the binaries in the resolved package.
///
/// Package resolution is specific to a user and can return different values
/// for different users on a system.
///
/// Each successful AddPackageDependency() adds the resolve packaged to the
/// calling process' package graph, even if already present. There is no
/// duplicate 'detection' or 'filtering' applied by the API (multiple
/// references from a package is not harmful). Once resolution is complete
/// the package dependency stays resolved for that user until the last reference across
/// all processes for that user is removed via RemovePackageDependency (or
/// process termination).
///
/// AddPackageDependency() adds the resolved package to the caller's package graph,
/// per the rank specified. A process' package graph is a list of packages sorted by
/// rank in ascending order (-infinity...0...+infinity). If package(s) are present in the
/// package graph with the same rank as the call to AddPackageDependency the resolved
/// package is (by default) added after others of the same rank. To add a package
/// before others o the same rank, specify AddPackageDependencyOptions_PrependIfRankCollision.
///
/// Every AddPackageDependency can be balanced by a RemovePackageDependency
/// to remove the entry from the package graph. If the process terminates all package
/// references are removed, but any pins stay behind.
///
/// AddPackageDependency adds the resolved package to the process' package
/// graph, per the rank and options parameters. The process' package
/// graph is used to search for DLLs (per Dynamic-Link Library Search Order),
/// WinRT objects and other resources; the caller can now load DLLs, activate
/// WinRT objects and use other resources from the framework package until
/// RemovePackageDependency is called. The packageDependencyId parameter
/// must match a package dependency defined for the calling user or the
/// system (i.e. pinned with CreatePackageDependencyOptions_ScopeIsSystem) else
/// an error is returned.
///
/// @param packageDependencyContext valid until passed to RemovePackageDependency()
/// @param packageFullName allocated via HeapAlloc; use HeapFree to deallocate
/// @see AddPackageDependency2()
WINBASEAPI
HRESULT
WINAPI
AddPackageDependency(
    _In_ PCWSTR packageDependencyId,
    INT32 rank,
    AddPackageDependencyOptions options,
    _Out_ PACKAGEDEPENDENCY_CONTEXT* packageDependencyContext,
    _Outptr_opt_result_maybenull_ PWSTR* packageFullName
    );

#if NTDDI_VERSION >= NTDDI_WIN11_GA

/// Resolve a previously-pinned PackageDependency to a specific package and
/// add it to the invoking process' package graph. Once the dependency has
/// been added other code-loading methods (LoadLibrary, CoCreateInstance, etc)
/// can find the binaries in the resolved package.
///
/// Package resolution is specific to a user and can return different values
/// for different users on a system.
///
/// Each successful AddPackageDependency2() adds the resolve packaged to the
/// calling process' package graph, even if already present. There is no
/// duplicate 'detection' or 'filtering' applied by the API (multiple
/// references from a package is not harmful). Once resolution is complete
/// the package dependency stays resolved for that user until the last reference across
/// all processes for that user is removed via RemovePackageDependency (or
/// process termination).
///
/// AddPackageDependency2() adds the resolved package to the caller's package graph,
/// per the rank specified. A process' package graph is a list of packages sorted by
/// rank in ascending order (-infinity...0...+infinity). If package(s) are present in the
/// package graph with the same rank as the call to AddPackageDependency2 the resolved
/// package is (by default) added after others of the same rank. To add a package
/// before others o the same rank, specify AddPackageDependencyOptions2_PrependIfRankCollision.
///
/// Every AddPackageDependency2 can be balanced by a RemovePackageDependency
/// to remove the entry from the package graph. If the process terminates all package
/// references are removed, but any pins stay behind.
///
/// AddPackageDependency2 adds the resolved package to the process' package
/// graph, per the rank and options parameters. The process' package
/// graph is used to search for DLLs (per Dynamic-Link Library Search Order),
/// WinRT objects and other resources; the caller can now load DLLs, activate
/// WinRT objects and use other resources from the framework package until
/// RemovePackageDependency is called. The packageDependencyId parameter
/// must match a package dependency defined for the calling user or the
/// system (i.e. pinned with CreatePackageDependencyOptions_ScopeIsSystem) else
/// an error is returned.
///
/// @param packageDependencyContext valid until passed to RemovePackageDependency()
/// @param packageFullName allocated via HeapAlloc; use HeapFree to deallocate
/// @see AddPackageDependency()
WINBASEAPI
HRESULT
WINAPI
AddPackageDependency2(
    _In_ PCWSTR packageDependencyId,
    INT32 rank,
    AddPackageDependencyOptions2 options,
    _Out_ PACKAGEDEPENDENCY_CONTEXT* packageDependencyContext,
    _Outptr_opt_result_maybenull_ PWSTR* packageFullName
    );

#endif // NTDDI_VERSION >= NTDDI_WIN11_GA

/// Remove a resolved PackageDependency from the current process' package graph
/// (i.e. undo AddPackageDependency). Used at runtime (i.e. the moral equivalent
/// of Windows' RemoveDllDirectory()).
///
/// @note This does not unload loaded resources (DLLs etc). After removing
///        a package dependency any files loaded from the package can continue
///        to be used; future file resolution will fail to see the removed
///        package dependency.
WINBASEAPI
HRESULT
WINAPI
RemovePackageDependency(
    _In_ PACKAGEDEPENDENCY_CONTEXT packageDependencyContext
    );

/// Return the package full name that would be used if the
/// PackageDependency were to be resolved. Does not add the
/// package to the process graph.
///
/// @param packageFullName allocated via HeapAlloc; use HeapFree to deallocate.
///                        If the package dependency cannot be resolved the function
///                        succeeds but packageFullName is NULL.
/// @see GetResolvedPackageFullNameForPackageDependency2
WINBASEAPI
HRESULT
WINAPI
GetResolvedPackageFullNameForPackageDependency(
    _In_ PCWSTR packageDependencyId,
    _Outptr_result_maybenull_ PWSTR* packageFullName
    );

#if NTDDI_VERSION >= NTDDI_WIN11_GA

/// Return the package full name that would be used if the
/// PackageDependency were to be resolved. Does not add the
/// package to the process graph.
///
/// @param packageFullName allocated via HeapAlloc; use HeapFree to deallocate.
/// @return E_INVALIDARG if packageDependencyId is not a valid package dependency.
/// @see GetResolvedPackageFullNameForPackageDependency
WINBASEAPI
HRESULT
WINAPI
GetResolvedPackageFullNameForPackageDependency2(
    _In_ PCWSTR packageDependencyId,
    _Outptr_result_maybenull_ PWSTR* packageFullName
    );

#endif // NTDDI_VERSION >= NTDDI_WIN11_GA

/// Return the package dependency for the context.
///
/// @param packageDependencyId allocated via HeapAlloc; use HeapFree to deallocate.
///                            If the package dependency context cannot be resolved
///                            the function succeeds but packageDependencyId is NULL.
WINBASEAPI
HRESULT
WINAPI
GetIdForPackageDependencyContext(
    _In_ PACKAGEDEPENDENCY_CONTEXT packageDependencyContext,
    _Outptr_result_maybenull_ PWSTR* packageDependencyId
    );

#endif // NTDDI_VERSION >= NTDDI_WIN10_CO

#if NTDDI_VERSION >= NTDDI_WIN10_NI

/// Returns the package graph's current revision ID.
WINBASEAPI
UINT32
WINAPI
GetPackageGraphRevisionId(
    );

#endif // NTDDI_VERSION >= NTDDI_WIN10_NI

typedef struct FindPackageDependencyCriteria {

    /// Match package dependencies for this user if not NULL.
    /// Match package dependencies for the current user if NULL (and ScopeIsSystem=FALSE).
    /// @note This MUST be NULL if ScopeIsSystem=TRUE.
    /// @note Admin privilege is required if User is not NULL and not the current user.
    PSID User;

    /// Match package dependencies created with CreatePackageDependencyOptions_ScopeIsSystem this is TRUE.
    /// @note Admin privilege is required if ScopeIsSystem is TRUE.
    BOOL ScopeIsSystem;

    /// Match package dependencies with this package family. Ignore if NULL or empty ("").
    PCWSTR PackageFamilyName;

} FindPackageDependencyCriteria;

#if NTDDI_VERSION >= NTDDI_WIN11_GA

/// Retrieve package dependencies.
/// @param packageDependencyIds allocated via HeapAlloc; use HeapFree to deallocate
///
/// @see FindPackageDependencyCriteria
/// @see TryCreatePackageDependency2
WINBASEAPI
HRESULT
WINAPI
FindPackageDependency(
    _In_ const FindPackageDependencyCriteria* findPackageDependencyCriteria,
    _Out_ UINT32* packageDependencyIdsCount,
    _Outptr_result_buffer_maybenull_(*packageDependencyIdsCount) PWSTR** packageDependencyIds
    );

/// Retrieve information about a package dependency.
///
/// @param user allocated via HeapAlloc; use HeapFree to deallocate
/// @param packageFamilyName allocated via HeapAlloc; use HeapFree to deallocate
/// @param lifetimeArtifact allocated via HeapAlloc; use HeapFree to deallocate
/// @param lifetimeExpiration if specified, the value is zero if expiration date is not set.
/// @note Admin privilege is required the package dependency was created with CreatePackageDependencyOptions_ScopeIsSystem or user is not NULL and not the current user.
/// @see FindPackageDependency
WINBASEAPI
HRESULT
WINAPI
GetPackageDependencyInformation(
    _In_ PCWSTR packageDependencyId,
    _Outptr_opt_result_maybenull_ PSID* user,
    _Outptr_opt_result_maybenull_ PWSTR* packageFamilyName,
    _Out_opt_ PACKAGE_VERSION* minVersion,
    _Out_opt_ PackageDependencyProcessorArchitectures* packageDependencyProcessorArchitectures,
    _Out_opt_ PackageDependencyLifetimeKind* lifetimeKind,
    _Outptr_opt_result_maybenull_ PWSTR* lifetimeArtifact,
    _Out_opt_ CreatePackageDependencyOptions* options,
    _Out_opt_ FILETIME* lifetimeExpiration
    );

/// Retrieve the list of processes using a package dependency.
///
/// @param user the user scope of the package dependency. If NULL the caller's
///        user context is used. MUST be NULL if scopeIsSystem=TRUE.
/// @param processIdsCount allocated via HeapAlloc; use HeapFree to deallocate
/// @param processIds allocated via HeapAlloc; use HeapFree to deallocate
/// @note Admin privilege is required if scopeIsSystem=TRUE or user is not NULL and not the current user.
/// @see FindPackageDependency
WINBASEAPI
HRESULT
WINAPI
GetProcessesUsingPackageDependency(
    _In_ PCWSTR packageDependencyId,
    _In_opt_ PSID user,
    _In_ BOOL scopeIsSystem,
    _Out_ UINT32* processIdsCount,
    _Outptr_result_buffer_maybenull_(*processIdsCount) DWORD** processIds
    );

#endif // NTDDI_VERSION >= NTDDI_WIN11_GA

/* ---------------------------------------------------------------- */

// Appmodel Policy

typedef enum AppPolicyLifecycleManagement
{
    AppPolicyLifecycleManagement_Unmanaged = 0,
    AppPolicyLifecycleManagement_Managed = 1
} AppPolicyLifecycleManagement;

WINBASEAPI
_Check_return_
_Success_(return == ERROR_SUCCESS)
LONG
WINAPI
AppPolicyGetLifecycleManagement(
    _In_ HANDLE processToken,
    _Out_ AppPolicyLifecycleManagement* policy
    );

typedef enum AppPolicyWindowingModel
{
    AppPolicyWindowingModel_None = 0,
    AppPolicyWindowingModel_Universal = 1,
    AppPolicyWindowingModel_ClassicDesktop = 2,
    AppPolicyWindowingModel_ClassicPhone = 3
} AppPolicyWindowingModel;

WINBASEAPI
_Check_return_
_Success_(return == ERROR_SUCCESS)
LONG
WINAPI
AppPolicyGetWindowingModel(
    _In_ HANDLE processToken,
    _Out_ AppPolicyWindowingModel* policy
    );

typedef enum AppPolicyMediaFoundationCodecLoading
{
    AppPolicyMediaFoundationCodecLoading_All = 0,
    AppPolicyMediaFoundationCodecLoading_InboxOnly = 1
} AppPolicyMediaFoundationCodecLoading;

WINBASEAPI
_Check_return_
_Success_(return == ERROR_SUCCESS)
LONG
WINAPI
AppPolicyGetMediaFoundationCodecLoading(
    _In_ HANDLE processToken,
    _Out_ AppPolicyMediaFoundationCodecLoading* policy
    );

typedef enum AppPolicyClrCompat
{
    AppPolicyClrCompat_Other = 0,
    AppPolicyClrCompat_ClassicDesktop = 1,
    AppPolicyClrCompat_Universal = 2,
    AppPolicyClrCompat_PackagedDesktop = 3
} AppPolicyClrCompat;

WINBASEAPI
_Check_return_
_Success_(return == ERROR_SUCCESS)
LONG
WINAPI
AppPolicyGetClrCompat(
    _In_ HANDLE processToken,
    _Out_ AppPolicyClrCompat* policy
    );

typedef enum AppPolicyThreadInitializationType
{
    AppPolicyThreadInitializationType_None = 0,
    AppPolicyThreadInitializationType_InitializeWinRT = 1,
} AppPolicyThreadInitializationType;

WINBASEAPI
_Check_return_
_Success_(return == ERROR_SUCCESS)
LONG
WINAPI
AppPolicyGetThreadInitializationType(
    _In_ HANDLE processToken,
    _Out_ AppPolicyThreadInitializationType* policy
    );

typedef enum AppPolicyShowDeveloperDiagnostic
{
    AppPolicyShowDeveloperDiagnostic_None = 0,
    AppPolicyShowDeveloperDiagnostic_ShowUI = 1,
} AppPolicyShowDeveloperDiagnostic;

WINBASEAPI
_Check_return_
_Success_(return == ERROR_SUCCESS)
LONG
WINAPI
AppPolicyGetShowDeveloperDiagnostic(
    _In_ HANDLE processToken,
    _Out_ AppPolicyShowDeveloperDiagnostic* policy
    );

typedef enum AppPolicyProcessTerminationMethod
{
    AppPolicyProcessTerminationMethod_ExitProcess = 0,
    AppPolicyProcessTerminationMethod_TerminateProcess = 1,
} AppPolicyProcessTerminationMethod;

WINBASEAPI
_Check_return_
_Success_(return == ERROR_SUCCESS)
LONG
WINAPI
AppPolicyGetProcessTerminationMethod(
    _In_ HANDLE processToken,
    _Out_ AppPolicyProcessTerminationMethod* policy
    );

typedef enum AppPolicyCreateFileAccess
{
    AppPolicyCreateFileAccess_Full = 0,
    AppPolicyCreateFileAccess_Limited = 1,
} AppPolicyCreateFileAccess;

WINBASEAPI
_Check_return_
_Success_(return == ERROR_SUCCESS)
LONG
WINAPI
AppPolicyGetCreateFileAccess(
    _In_ HANDLE processToken,
    _Out_ AppPolicyCreateFileAccess* policy
    );

/* ---------------------------------------------------------------- */

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)
#pragma endregion

#if defined(__cplusplus)
} // end extern "C"
#endif // defined(__cplusplus)

#if defined(_MSC_VER)
#if _MSC_VER >= 1200
#pragma warning(pop)
#endif // _MSC_VER >= 1200
#endif // defined(_MSC_VER)

#endif // _APPMODEL_H_
