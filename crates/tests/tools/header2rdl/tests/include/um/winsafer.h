/*++

Copyright (c) 1997-2001  Microsoft Corporation

Module Name:

    winsafer.h

Abstract:

    This file implements the publicly exported functions, data types,
    data structures, and definitions usable by programs that directly
    interact with the Windows SAFER APIs.

--*/

#ifndef _WINSAFER_H
#define _WINSAFER_H

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#include <guiddef.h>
#include <wincrypt.h>


#ifdef __cplusplus
extern "C" {
#endif

//
// Opaque datatype for representing handles to Safer objects.
//

DECLARE_HANDLE(SAFER_LEVEL_HANDLE);


//
// Constants to represent scope with SaferCreateLevel and others.
//

#define SAFER_SCOPEID_MACHINE  1
#define SAFER_SCOPEID_USER     2


//
// Pre-defined levels that can be used with SaferCreateLevel
//

#define SAFER_LEVELID_FULLYTRUSTED 0x40000
#define SAFER_LEVELID_NORMALUSER   0x20000
#define SAFER_LEVELID_CONSTRAINED  0x10000
#define SAFER_LEVELID_UNTRUSTED    0x01000
#define SAFER_LEVELID_DISALLOWED   0x00000

//
// Flags to use when creating/opening a Level with SaferCreateLevel
//

#define SAFER_LEVEL_OPEN   1


//
// Maximum string size.
//

#define SAFER_MAX_FRIENDLYNAME_SIZE 256
#define SAFER_MAX_DESCRIPTION_SIZE  256
#define SAFER_MAX_HASH_SIZE         64

//
// Flags to use with SaferComputeTokenFromLevel.
//

#define SAFER_TOKEN_NULL_IF_EQUAL   0x00000001
#define SAFER_TOKEN_COMPARE_ONLY    0x00000002
#define SAFER_TOKEN_MAKE_INERT      0x00000004
#define SAFER_TOKEN_WANT_FLAGS      0x00000008

//
// Flags for specifying what criteria within SAFER_CODE_PROPERTIES to evaluate
// when finding code identity with SaferIdentifyLevel.
//

#define SAFER_CRITERIA_IMAGEPATH    0x00001
#define SAFER_CRITERIA_NOSIGNEDHASH 0x00002
#define SAFER_CRITERIA_IMAGEHASH    0x00004
#define SAFER_CRITERIA_AUTHENTICODE 0x00008
#define SAFER_CRITERIA_URLZONE      0x00010
#define SAFER_CRITERIA_APPX_PACKAGE 0x00020
#define SAFER_CRITERIA_IMAGEPATH_NT 0x01000

//
// Code image information structure passed to SaferIdentifyLevel.
//

#include <pshpack8.h>

typedef struct _SAFER_CODE_PROPERTIES_V1
{

    //
    // Must be initialized to be the size of this structure,
    // for the purposes of future/backwards compatibility.
    //

    DWORD cbSize;

    //
    // Must be initialized to the types of criteria that should
    // be considered when evaluating this structure.  This can be
    // a combination of the SAFER_CRITERIA_xxxx flags.  If not enough
    // of the structure elements needed to evaluate the criteria
    // types indicated were supplied, then some of those criteria
    // flags may be silently ignored.  Specifying 0 for this value
    // will cause the entire structure's contents to be ignored.
    //

    DWORD dwCheckFlags;

    //
    // Optionally specifies the fully-qualified path and filename
    // to be used for discrimination checks based on the path.
    // The ImagePath will additionally be used to open and read the
    // file to identify any other discrimination criteria that was
    // unsupplied in this structure.
    //

    LPCWSTR ImagePath;

    //
    // Optionally specifies a file handle that has been opened to
    // code image with at least GENERIC_READ access.  The handle will
    // be used instead of explicitly opening the file again to compute
    // other discrimination criteria that was unsupplied in this structure.
    //

    HANDLE hImageFileHandle;

    //
    // Optionally specifies the pre-determined Internet Explorer
    // security zone.  These enums are defined within urlmon.h
    // For example: URLZONE_LOCAL_MACHINE, URLZONE_INTRANET,
    //   URLZONE_TRUSTED, URLZONE_INTERNET, or URLZONE_UNTRUSTED.
    //

    DWORD UrlZoneId;

    //
    // Optionally specifies the pre-computed hash of the image.
    // The supplied hash is interpreted as being valid if ImageSize
    // is non-zero and dwImageHashSize is non-zero and HashAlgorithm
    // represents a valid hashing algorithm from wincrypt.h
    //
    // If the supplied hash fails to meet the conditions above, then
    // the hash will be automatically computed against:
    //      1) by using ImageSize and pByteBlock if both are non-zero.
    //      2) by using hImageFileHandle if it is non-null.
    //      3) by attempting to open ImagePath if it is non-null.
    //

    BYTE ImageHash[SAFER_MAX_HASH_SIZE];
    DWORD dwImageHashSize;
    LARGE_INTEGER ImageSize;
    ALG_ID HashAlgorithm;

    //
    // Optionally specifies a memory block of memory representing
    // the image for which the trust is being requested for.  When
    // this member is specified, ImageSize must also be supplied.
    //

    LPBYTE pByteBlock;

    //
    // Optionally gives the arguments used for Authenticode signer
    // certificate verification.  These arguments are supplied to the
    // WinVerifyTrust() API and control the user-interface prompting
    // to accept untrusted certificates.
    //

    HWND hWndParent;
    DWORD dwWVTUIChoice;

} SAFER_CODE_PROPERTIES_V1, *PSAFER_CODE_PROPERTIES_V1;

typedef struct _SAFER_CODE_PROPERTIES_V2
{

    //
    // Begin SAFER_CODE_PROPERTIES_V1
    // These must be present for backwards compatability
    //

    //
    // Must be initialized to be the size of this structure,
    // for the purposes of future/backwards compatibility.
    //

    DWORD cbSize;

    //
    // Must be initialized to the types of criteria that should
    // be considered when evaluating this structure.  This can be
    // a combination of the SAFER_CRITERIA_xxxx flags.  If not enough
    // of the structure elements needed to evaluate the criteria
    // types indicated were supplied, then some of those criteria
    // flags may be silently ignored.  Specifying 0 for this value
    // will cause the entire structure's contents to be ignored.
    //

    DWORD dwCheckFlags;

    //
    // Optionally specifies the fully-qualified path and filename
    // to be used for discrimination checks based on the path.
    // The ImagePath will additionally be used to open and read the
    // file to identify any other discrimination criteria that was
    // unsupplied in this structure.
    //

    LPCWSTR ImagePath;

    //
    // Optionally specifies a file handle that has been opened to
    // code image with at least GENERIC_READ access.  The handle will
    // be used instead of explicitly opening the file again to compute
    // other discrimination criteria that was unsupplied in this structure.
    //

    HANDLE hImageFileHandle;

    //
    // Optionally specifies the pre-determined Internet Explorer
    // security zone.  These enums are defined within urlmon.h
    // For example: URLZONE_LOCAL_MACHINE, URLZONE_INTRANET,
    //   URLZONE_TRUSTED, URLZONE_INTERNET, or URLZONE_UNTRUSTED.
    //

    DWORD UrlZoneId;

    //
    // Optionally specifies the pre-computed hash of the image.
    // The supplied hash is interpreted as being valid if ImageSize
    // is non-zero and dwImageHashSize is non-zero and HashAlgorithm
    // represents a valid hashing algorithm from wincrypt.h
    //
    // If the supplied hash fails to meet the conditions above, then
    // the hash will be automatically computed against:
    //      1) by using ImageSize and pByteBlock if both are non-zero.
    //      2) by using hImageFileHandle if it is non-null.
    //      3) by attempting to open ImagePath if it is non-null.
    //

    BYTE ImageHash[SAFER_MAX_HASH_SIZE];
    DWORD dwImageHashSize;
    LARGE_INTEGER ImageSize;
    ALG_ID HashAlgorithm;

    //
    // Optionally specifies a memory block of memory representing
    // the image for which the trust is being requested for.  When
    // this member is specified, ImageSize must also be supplied.
    //

    LPBYTE pByteBlock;

    //
    // Optionally gives the arguments used for Authenticode signer
    // certificate verification.  These arguments are supplied to the
    // WinVerifyTrust() API and control the user-interface prompting
    // to accept untrusted certificates.
    //

    HWND hWndParent;
    DWORD dwWVTUIChoice;

    //
    // End SAFER_CODE_PROPERTIES_V1
    //

    //
    // Package properties for Modern applications
    //

    LPCWSTR PackageMoniker;
    LPCWSTR PackagePublisher;
    LPCWSTR PackageName;
    ULONG64 PackageVersion;
    BOOL PackageIsFramework;

} SAFER_CODE_PROPERTIES_V2, *PSAFER_CODE_PROPERTIES_V2;

typedef SAFER_CODE_PROPERTIES_V2 SAFER_CODE_PROPERTIES, *PSAFER_CODE_PROPERTIES;

#include <poppack.h>


//
// Masks for the per-identity WinSafer flags
//

#define SAFER_POLICY_JOBID_MASK                  0xFF000000
#define SAFER_POLICY_JOBID_CONSTRAINED           0x04000000
#define SAFER_POLICY_JOBID_UNTRUSTED             0x03000000
#define SAFER_POLICY_ONLY_EXES                   0x00010000
#define SAFER_POLICY_SANDBOX_INERT               0x00020000
#define SAFER_POLICY_HASH_DUPLICATE              0x00040000
#define SAFER_POLICY_ONLY_AUDIT                  0x00001000
#define SAFER_POLICY_BLOCK_CLIENT_UI             0x00002000
#define SAFER_POLICY_UIFLAGS_MASK                0x000000FF
#define SAFER_POLICY_UIFLAGS_INFORMATION_PROMPT  0x00000001
#define SAFER_POLICY_UIFLAGS_OPTION_PROMPT       0x00000002
#define SAFER_POLICY_UIFLAGS_HIDDEN              0x00000004

//
// Information classes on the overall policy that can be queried
// with SaferSet/GetPolicyInformation and set at different
// policy scopes based on access of the caller.
//

typedef enum _SAFER_POLICY_INFO_CLASS
{

    //
    // Accesses the list of all Levels defined in a policy.
    // The corresponding data element is a buffer that is filled
    // with multiple DWORDs, each representing the LevelIds that
    // are defined within this scope.
    //

    SaferPolicyLevelList = 1,

    //
    // for transparent enforcement of policy in the execution
    // framework -- will be used by native code execution but can
    // be used by any policy enforcement environment.
    // Corresponding data element is a DWORD holding a Boolean value.
    //

    SaferPolicyEnableTransparentEnforcement,

    //
    // Returns the name of the Level that has been designed
    // as the default level within the specified scope.
    // The corresponding data element is a single DWORD buffer
    // representing the LevelId of the default Level.  If no
    // level has been configured to be the default, then the
    // GetInfo API will return FALSE and GetLastError will
    // return ERROR_NOT_FOUND.
    //

    SaferPolicyDefaultLevel,

    //
    // Returns whether Code Identities or Default Level within the
    // user scope can be considered during identification.
    //

    SaferPolicyEvaluateUserScope,

    //
    // Control Flags for safer policy scope.
    //

    SaferPolicyScopeFlags,

    //
    // Flags which indicate "audit" or "block client UI" rules
    //

    SaferPolicyDefaultLevelFlags,

    //
    // Flag which indicates if certificate rules are applied in CreateProcess
    //

    SaferPolicyAuthenticodeEnabled

} SAFER_POLICY_INFO_CLASS;


//
// Enumerations used for retrieving specific information about a
// single authorization Level via SaferGet/SetInformationFromLevel.
//

typedef enum _SAFER_OBJECT_INFO_CLASS
{

    SaferObjectLevelId = 1,               // get: DWORD
    SaferObjectScopeId,                   // get: DWORD
    SaferObjectFriendlyName,              // get/set: LPCWSTR
    SaferObjectDescription,               // get/set: LPCWSTR
    SaferObjectBuiltin,                   // get: DWORD boolean

    SaferObjectDisallowed,                // get: DWORD boolean
    SaferObjectDisableMaxPrivilege,       // get: DWORD boolean
    SaferObjectInvertDeletedPrivileges,   // get: DWORD boolean
    SaferObjectDeletedPrivileges,         // get: TOKEN_PRIVILEGES
    SaferObjectDefaultOwner,              // get: TOKEN_OWNER
    SaferObjectSidsToDisable,             // get: TOKEN_GROUPS
    SaferObjectRestrictedSidsInverted,    // get: TOKEN_GROUPS
    SaferObjectRestrictedSidsAdded,       // get: TOKEN_GROUPS

    //
    // To enumerate all identities, call GetInfo with
    //      SaferObjectAllIdentificationGuids.
    //

    SaferObjectAllIdentificationGuids,    // get: SAFER_IDENTIFICATION_GUIDS

    //
    // To create a new identity, call SetInfo with
    //      SaferObjectSingleIdentification with a new
    //      unique GUID that you have generated.
    // To get details on a single identity, call GetInfo with
    //      SaferObjectSingleIdentification with desired GUID.
    // To modify details of a single identity, call SetInfo with
    //      SaferObjectSingleIdentification with desired info and GUID.
    // To delete an identity, call SetInfo with
    //      SaferObjectSingleIdentification with the
    //      header.dwIdentificationType set to 0.
    //

    SaferObjectSingleIdentification,      // get/set: SAFER_IDENTIFICATION_*

    SaferObjectExtendedError              // get: DWORD dwError

} SAFER_OBJECT_INFO_CLASS;


//
// Structures and enums used by the SaferGet/SetLevelInformation APIs.
//

#include <pshpack8.h>

typedef enum _SAFER_IDENTIFICATION_TYPES
{

    SaferIdentityDefault,
    SaferIdentityTypeImageName = 1,
    SaferIdentityTypeImageHash,
    SaferIdentityTypeUrlZone,
    SaferIdentityTypeCertificate

} SAFER_IDENTIFICATION_TYPES;

typedef struct _SAFER_IDENTIFICATION_HEADER
{
    //
    // indicates the type of the structure, one of SaferIdentityType*
    //

    SAFER_IDENTIFICATION_TYPES dwIdentificationType;

    //
    // size of the whole structure, not just the common header.
    //

    DWORD cbStructSize;

    //
    // the unique GUID of the Identity in question.
    //

    GUID IdentificationGuid;

    //
    // last change of this identification.
    //

    FILETIME lastModified;

} SAFER_IDENTIFICATION_HEADER, *PSAFER_IDENTIFICATION_HEADER;

typedef struct _SAFER_PATHNAME_IDENTIFICATION
{
    //
    // header.dwIdentificationType must be SaferIdentityTypeImageName
    // header.cbStructSize must be sizeof(SAFER_PATHNAME_IDENTIFICATION)
    //

    SAFER_IDENTIFICATION_HEADER header;

    //
    // user-entered description
    //

    WCHAR Description[SAFER_MAX_DESCRIPTION_SIZE];

    //
    // filepath or name, possibly with vars
    //

    PWCHAR ImageName;

    //
    // any combo of SAFER_POLICY_* flags
    //

    DWORD dwSaferFlags;

} SAFER_PATHNAME_IDENTIFICATION, *PSAFER_PATHNAME_IDENTIFICATION;

typedef struct _SAFER_HASH_IDENTIFICATION
{
    //
    // header.dwIdentificationType must be SaferIdentityTypeImageHash
    // header.cbStructSize must be sizeof(SAFER_HASH_IDENTIFICATION)
    //

    SAFER_IDENTIFICATION_HEADER header;

    //
    // user-entered friendly name, initially from file's resources.
    //
    WCHAR Description[SAFER_MAX_DESCRIPTION_SIZE];

    //
    // user-entered description.
    //

    WCHAR FriendlyName[SAFER_MAX_FRIENDLYNAME_SIZE];

    //
    // amount of ImageHash actually used, in bytes (MD5 is 16 bytes).
    //

    DWORD HashSize;

    //
    // computed hash data itself.
    //

    BYTE ImageHash[SAFER_MAX_HASH_SIZE];

    //
    // algorithm in which the hash was computed (CALG_MD5, etc).
    //

    ALG_ID HashAlgorithm;

    //
    // size of the original file in bytes.
    //

    LARGE_INTEGER ImageSize;

    //
    // any combo of SAFER_POLICY_* flags
    //

    DWORD dwSaferFlags;

} SAFER_HASH_IDENTIFICATION, *PSAFER_HASH_IDENTIFICATION;

typedef struct _SAFER_HASH_IDENTIFICATION2
{
    //
    // Start by including the original structure. It contains number of bytes
    // in this particular structure.
    //

    SAFER_HASH_IDENTIFICATION hashIdentification;

    //
    // amount of ImageHash actually used, in bytes (SHA256 is 32 bytes).
    //

    DWORD HashSize;

    //
    // computed hash data itself.
    //

    BYTE ImageHash[SAFER_MAX_HASH_SIZE];

    //
    // algorithm in which the hash was computed (CALG_SHA256).
    //

    ALG_ID HashAlgorithm;

} SAFER_HASH_IDENTIFICATION2, *PSAFER_HASH_IDENTIFICATION2;

typedef struct _SAFER_URLZONE_IDENTIFICATION
{
    //
    // header.dwIdentificationType must be SaferIdentityTypeUrlZone
    // header.cbStructSize must be sizeof(SAFER_URLZONE_IDENTIFICATION)
    //

    SAFER_IDENTIFICATION_HEADER header;

    //
    // any single URLZONE_* from urlmon.h
    //

    DWORD UrlZoneId;

    //
    // any combo of SAFER_POLICY_* flags
    //

    DWORD dwSaferFlags;

} SAFER_URLZONE_IDENTIFICATION, *PSAFER_URLZONE_IDENTIFICATION;


#include <poppack.h>

//
// Functions related to querying and setting the global policy
// controls to disable transparent enforcement, and perform level
// enumeration operations.
//

WINADVAPI
BOOL WINAPI
SaferGetPolicyInformation(
    _In_ DWORD                   dwScopeId,
    _In_ SAFER_POLICY_INFO_CLASS SaferPolicyInfoClass,
    _In_ DWORD                   InfoBufferSize,
    _Out_writes_bytes_(InfoBufferSize) PVOID InfoBuffer,
    _Out_ PDWORD                 InfoBufferRetSize,
    _Reserved_ LPVOID            lpReserved
    );

WINADVAPI
BOOL WINAPI
SaferSetPolicyInformation(
    _In_ DWORD                          dwScopeId,
    _In_ SAFER_POLICY_INFO_CLASS        SaferPolicyInfoClass,
    _In_ DWORD                          InfoBufferSize,
    _In_reads_bytes_(InfoBufferSize) PVOID   InfoBuffer,
    _Reserved_ LPVOID                   lpReserved
    );

//
// Functions to open or close a handle to a Safer Level.
//

WINADVAPI
BOOL WINAPI
SaferCreateLevel(
    _In_ DWORD                dwScopeId,
    _In_ DWORD                dwLevelId,
    _In_ DWORD                OpenFlags,
    _Outptr_ SAFER_LEVEL_HANDLE * pLevelHandle,
    _Reserved_ LPVOID               lpReserved
    );

WINADVAPI
BOOL WINAPI
SaferCloseLevel(
    _In_ SAFER_LEVEL_HANDLE hLevelHandle
    );

WINADVAPI
BOOL WINAPI
SaferIdentifyLevel(
    _In_ DWORD              dwNumProperties,
    _In_reads_opt_(dwNumProperties) PSAFER_CODE_PROPERTIES pCodeProperties,
    _Outptr_ SAFER_LEVEL_HANDLE  * pLevelHandle,
    _In_opt_ LPVOID         lpReserved
    );

WINADVAPI
BOOL WINAPI
SaferComputeTokenFromLevel(
    _In_ SAFER_LEVEL_HANDLE LevelHandle,
    _In_opt_ HANDLE         InAccessToken,
    _Out_ PHANDLE           OutAccessToken,
    _In_ DWORD              dwFlags,
    _Inout_opt_ LPVOID      lpReserved
    );

WINADVAPI
BOOL WINAPI
SaferGetLevelInformation(
        _In_ SAFER_LEVEL_HANDLE                 LevelHandle,
        _In_ SAFER_OBJECT_INFO_CLASS            dwInfoType,
        _Out_writes_bytes_opt_(dwInBufferSize) LPVOID lpQueryBuffer,
        _In_ DWORD                              dwInBufferSize,
        _Out_ LPDWORD                           lpdwOutBufferSize
        );

WINADVAPI
BOOL WINAPI
SaferSetLevelInformation(
    _In_ SAFER_LEVEL_HANDLE             LevelHandle,
    _In_ SAFER_OBJECT_INFO_CLASS        dwInfoType,
    _In_reads_bytes_(dwInBufferSize) LPVOID  lpQueryBuffer,
    _In_ DWORD                          dwInBufferSize
    );

//
// This function performs logging of messages to the Application
// event log.  This is called by the hooks within CreateProcess,
// ShellExecute and cmd when a lower trust evaluation result occurs.
//

WINADVAPI
BOOL WINAPI
SaferRecordEventLogEntry(
    _In_ SAFER_LEVEL_HANDLE hLevel,
    _In_ LPCWSTR            szTargetPath,
    _Reserved_ LPVOID       lpReserved
    );


WINADVAPI
BOOL WINAPI
SaferiIsExecutableFileType(
    _In_ LPCWSTR szFullPathname,
    _In_ BOOLEAN bFromShellExecute
    );

#define SRP_POLICY_EXE        L"EXE"
#define SRP_POLICY_DLL        L"DLL"
#define SRP_POLICY_MSI        L"MSI"
#define SRP_POLICY_SCRIPT     L"SCRIPT"
#define SRP_POLICY_SHELL      L"SHELL"
#define SRP_POLICY_NOV2       L"IGNORESRPV2"
#define SRP_POLICY_APPX       L"APPX"
#define SRP_POLICY_WLDPMSI    L"WLDPMSI"
#define SRP_POLICY_WLDPSCRIPT L"WLDPSCRIPT"
#define SRP_POLICY_WLDPCONFIGCI L"WLDPCONFIGCI"
#define SRP_POLICY_MANAGEDINSTALLER L"MANAGEDINSTALLER"

#ifdef __cplusplus
}
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif
