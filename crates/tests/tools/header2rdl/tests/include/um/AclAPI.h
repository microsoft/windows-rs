/*++ BUILD Version: 0001    // Increment this if a change has global effects

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    aclapi.h

Abstract:

    Public
    Structure/constant definitions and typedefines for the Win32 Access
    Control APIs

--*/
#ifndef __ACCESS_CONTROL_API__
#define __ACCESS_CONTROL_API__

#include <winapifamily.h>


#include <windows.h>
#include <accctrl.h>

#ifdef __cplusplus
extern "C" {
#endif

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

//
// Progress Function:
// Caller of tree operation implements this Progress function, then
// passes its function pointer to tree operation.
// Tree operation invokes Progress function to provide progress and error
// information to the caller during the potentially long execution
// of the tree operation.  Tree operation provides the name of the object
// last processed and the error status of the operation on that object.
// Tree operation also passes the current InvokeSetting value.
// Caller may change the InvokeSetting value, for example, from "Always"
// to "Only On Error."
//

typedef VOID (CALLBACK *FN_PROGRESS) (
    _In_ LPWSTR                     pObjectName,    // name of object just processed
    _In_ DWORD                      Status,         // status of operation on object
    _Inout_ PPROG_INVOKE_SETTING    pInvokeSetting, // Never, always,
    _In_ PVOID                      Args,           // Caller specific data
    _In_ BOOL                       SecuritySet     // Whether security was set
    );

WINADVAPI
DWORD
WINAPI
SetEntriesInAclA(
    _In_ ULONG               cCountOfExplicitEntries,
    _In_reads_opt_(cCountOfExplicitEntries)  PEXPLICIT_ACCESS_A  pListOfExplicitEntries,
    _In_opt_  PACL           OldAcl,
    _Out_ PACL              * NewAcl
    );
WINADVAPI
DWORD
WINAPI
SetEntriesInAclW(
    _In_ ULONG               cCountOfExplicitEntries,
    _In_reads_opt_(cCountOfExplicitEntries)  PEXPLICIT_ACCESS_W  pListOfExplicitEntries,
    _In_opt_  PACL           OldAcl,
    _Out_ PACL              * NewAcl
    );
#ifdef UNICODE
#define SetEntriesInAcl  SetEntriesInAclW
#else
#define SetEntriesInAcl  SetEntriesInAclA
#endif // !UNICODE


WINADVAPI
DWORD
WINAPI
GetExplicitEntriesFromAclA(
    _In_ PACL                  pacl,
    _Out_ PULONG                pcCountOfExplicitEntries,
    _Outptr_result_buffer_(*pcCountOfExplicitEntries) PEXPLICIT_ACCESS_A  * pListOfExplicitEntries
    );
WINADVAPI
DWORD
WINAPI
GetExplicitEntriesFromAclW(
    _In_ PACL                  pacl,
    _Out_ PULONG                pcCountOfExplicitEntries,
    _Outptr_result_buffer_(*pcCountOfExplicitEntries) PEXPLICIT_ACCESS_W  * pListOfExplicitEntries
    );
#ifdef UNICODE
#define GetExplicitEntriesFromAcl  GetExplicitEntriesFromAclW
#else
#define GetExplicitEntriesFromAcl  GetExplicitEntriesFromAclA
#endif // !UNICODE

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

WINADVAPI
DWORD
WINAPI
GetEffectiveRightsFromAclA(
    _In_  PACL          pacl,
    _In_  PTRUSTEE_A    pTrustee,
    _Out_ PACCESS_MASK  pAccessRights
    );
WINADVAPI
DWORD
WINAPI
GetEffectiveRightsFromAclW(
    _In_  PACL          pacl,
    _In_  PTRUSTEE_W    pTrustee,
    _Out_ PACCESS_MASK  pAccessRights
    );
#ifdef UNICODE
#define GetEffectiveRightsFromAcl  GetEffectiveRightsFromAclW
#else
#define GetEffectiveRightsFromAcl  GetEffectiveRightsFromAclA
#endif // !UNICODE


WINADVAPI
DWORD
WINAPI
GetAuditedPermissionsFromAclA(
    _In_  PACL          pacl,
    _In_  PTRUSTEE_A    pTrustee,
    _Out_ PACCESS_MASK  pSuccessfulAuditedRights,
    _Out_ PACCESS_MASK  pFailedAuditRights
    );
WINADVAPI
DWORD
WINAPI
GetAuditedPermissionsFromAclW(
    _In_  PACL          pacl,
    _In_  PTRUSTEE_W    pTrustee,
    _Out_ PACCESS_MASK  pSuccessfulAuditedRights,
    _Out_ PACCESS_MASK  pFailedAuditRights
    );
#ifdef UNICODE
#define GetAuditedPermissionsFromAcl  GetAuditedPermissionsFromAclW
#else
#define GetAuditedPermissionsFromAcl  GetAuditedPermissionsFromAclA
#endif // !UNICODE

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

WINADVAPI
DWORD
WINAPI
GetNamedSecurityInfoA(
    _In_  LPCSTR               pObjectName,
    _In_  SE_OBJECT_TYPE         ObjectType,
    _In_  SECURITY_INFORMATION   SecurityInfo,
    _Out_opt_       PSID         * ppsidOwner,
    _Out_opt_       PSID         * ppsidGroup,
    _Out_opt_       PACL         * ppDacl,
    _Out_opt_       PACL         * ppSacl,
    _Out_ PSECURITY_DESCRIPTOR   * ppSecurityDescriptor
    );
WINADVAPI
DWORD
WINAPI
GetNamedSecurityInfoW(
    _In_  LPCWSTR               pObjectName,
    _In_  SE_OBJECT_TYPE         ObjectType,
    _In_  SECURITY_INFORMATION   SecurityInfo,
    _Out_opt_       PSID         * ppsidOwner,
    _Out_opt_       PSID         * ppsidGroup,
    _Out_opt_       PACL         * ppDacl,
    _Out_opt_       PACL         * ppSacl,
    _Out_ PSECURITY_DESCRIPTOR   * ppSecurityDescriptor
    );
#ifdef UNICODE
#define GetNamedSecurityInfo  GetNamedSecurityInfoW
#else
#define GetNamedSecurityInfo  GetNamedSecurityInfoA
#endif // !UNICODE

WINADVAPI
DWORD
WINAPI
GetSecurityInfo(
    _In_  HANDLE                 handle,
    _In_  SE_OBJECT_TYPE         ObjectType,
    _In_  SECURITY_INFORMATION   SecurityInfo,
    _Out_opt_ PSID                 * ppsidOwner,
    _Out_opt_ PSID                 * ppsidGroup,
    _Out_opt_ PACL                 * ppDacl,
    _Out_opt_ PACL                 * ppSacl,
    _Out_opt_ PSECURITY_DESCRIPTOR * ppSecurityDescriptor
    );

WINADVAPI
DWORD
WINAPI
SetNamedSecurityInfoA(
    _In_ LPSTR               pObjectName,
    _In_ SE_OBJECT_TYPE        ObjectType,
    _In_ SECURITY_INFORMATION  SecurityInfo,
    _In_opt_ PSID              psidOwner,
    _In_opt_ PSID              psidGroup,
    _In_opt_ PACL              pDacl,
    _In_opt_ PACL              pSacl
    );
WINADVAPI
DWORD
WINAPI
SetNamedSecurityInfoW(
    _In_ LPWSTR               pObjectName,
    _In_ SE_OBJECT_TYPE        ObjectType,
    _In_ SECURITY_INFORMATION  SecurityInfo,
    _In_opt_ PSID              psidOwner,
    _In_opt_ PSID              psidGroup,
    _In_opt_ PACL              pDacl,
    _In_opt_ PACL              pSacl
    );
#ifdef UNICODE
#define SetNamedSecurityInfo  SetNamedSecurityInfoW
#else
#define SetNamedSecurityInfo  SetNamedSecurityInfoA
#endif // !UNICODE

WINADVAPI
DWORD
WINAPI
SetSecurityInfo(
    _In_     HANDLE                handle,
    _In_     SE_OBJECT_TYPE        ObjectType,
    _In_     SECURITY_INFORMATION  SecurityInfo,
    _In_opt_ PSID                  psidOwner,
    _In_opt_ PSID                  psidGroup,
    _In_opt_ PACL                  pDacl,
    _In_opt_ PACL                  pSacl
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

WINADVAPI
DWORD
WINAPI
GetInheritanceSourceA(
    _In_ LPSTR                  pObjectName,
    _In_ SE_OBJECT_TYPE           ObjectType,
    _In_ SECURITY_INFORMATION     SecurityInfo,
    _In_ BOOL                     Container,
    _In_reads_opt_(GuidCount) GUID	** pObjectClassGuids,
    _In_ DWORD                    GuidCount,
    _In_ PACL                     pAcl,
    _In_opt_ PFN_OBJECT_MGR_FUNCTS    pfnArray,
    _In_ PGENERIC_MAPPING         pGenericMapping,
    _Out_ PINHERITED_FROMA         pInheritArray
    );
WINADVAPI
DWORD
WINAPI
GetInheritanceSourceW(
    _In_ LPWSTR                  pObjectName,
    _In_ SE_OBJECT_TYPE           ObjectType,
    _In_ SECURITY_INFORMATION     SecurityInfo,
    _In_ BOOL                     Container,
    _In_reads_opt_(GuidCount) GUID	** pObjectClassGuids,
    _In_ DWORD                    GuidCount,
    _In_ PACL                     pAcl,
    _In_opt_ PFN_OBJECT_MGR_FUNCTS    pfnArray,
    _In_ PGENERIC_MAPPING         pGenericMapping,
    _Out_ PINHERITED_FROMW         pInheritArray
    );
#ifdef UNICODE
#define GetInheritanceSource  GetInheritanceSourceW
#else
#define GetInheritanceSource  GetInheritanceSourceA
#endif // !UNICODE

WINADVAPI
DWORD
WINAPI
FreeInheritedFromArray(
    _In_reads_(AceCnt) PINHERITED_FROMW pInheritArray,
    _In_ USHORT AceCnt,
    _In_opt_ PFN_OBJECT_MGR_FUNCTS   pfnArray OPTIONAL
    );

WINADVAPI
DWORD
WINAPI
TreeResetNamedSecurityInfoA(
    _In_     LPSTR               pObjectName,
    _In_     SE_OBJECT_TYPE       ObjectType,
    _In_     SECURITY_INFORMATION SecurityInfo,
    _In_opt_ PSID                 pOwner,
    _In_opt_ PSID                 pGroup,
    _In_opt_ PACL                 pDacl,
    _In_opt_ PACL                 pSacl,
    _In_     BOOL                 KeepExplicit,
    _In_opt_ FN_PROGRESS          fnProgress,
    _In_     PROG_INVOKE_SETTING  ProgressInvokeSetting,
    _In_opt_ PVOID                Args
    );
WINADVAPI
DWORD
WINAPI
TreeResetNamedSecurityInfoW(
    _In_     LPWSTR               pObjectName,
    _In_     SE_OBJECT_TYPE       ObjectType,
    _In_     SECURITY_INFORMATION SecurityInfo,
    _In_opt_ PSID                 pOwner,
    _In_opt_ PSID                 pGroup,
    _In_opt_ PACL                 pDacl,
    _In_opt_ PACL                 pSacl,
    _In_     BOOL                 KeepExplicit,
    _In_opt_ FN_PROGRESS          fnProgress,
    _In_     PROG_INVOKE_SETTING  ProgressInvokeSetting,
    _In_opt_ PVOID                Args
    );
#ifdef UNICODE
#define TreeResetNamedSecurityInfo  TreeResetNamedSecurityInfoW
#else
#define TreeResetNamedSecurityInfo  TreeResetNamedSecurityInfoA
#endif // !UNICODE


#if (NTDDI_VERSION >= NTDDI_VISTA)
WINADVAPI
DWORD
WINAPI
TreeSetNamedSecurityInfoA(
    _In_     LPSTR               pObjectName,
    _In_     SE_OBJECT_TYPE       ObjectType,
    _In_     SECURITY_INFORMATION SecurityInfo,
    _In_opt_ PSID                 pOwner,
    _In_opt_ PSID                 pGroup,
    _In_opt_ PACL                 pDacl,
    _In_opt_ PACL                 pSacl,
    _In_     DWORD                dwAction,
    _In_opt_ FN_PROGRESS          fnProgress,
    _In_     PROG_INVOKE_SETTING  ProgressInvokeSetting,
    _In_opt_ PVOID                Args
    );
WINADVAPI
DWORD
WINAPI
TreeSetNamedSecurityInfoW(
    _In_     LPWSTR               pObjectName,
    _In_     SE_OBJECT_TYPE       ObjectType,
    _In_     SECURITY_INFORMATION SecurityInfo,
    _In_opt_ PSID                 pOwner,
    _In_opt_ PSID                 pGroup,
    _In_opt_ PACL                 pDacl,
    _In_opt_ PACL                 pSacl,
    _In_     DWORD                dwAction,
    _In_opt_ FN_PROGRESS          fnProgress,
    _In_     PROG_INVOKE_SETTING  ProgressInvokeSetting,
    _In_opt_ PVOID                Args
    );
#ifdef UNICODE
#define TreeSetNamedSecurityInfo  TreeSetNamedSecurityInfoW
#else
#define TreeSetNamedSecurityInfo  TreeSetNamedSecurityInfoA
#endif // !UNICODE

#endif // (NTDDI_VERSION >= NTDDI_VISTA)

//----------------------------------------------------------------------------
// The following API are provided for trusted servers to use to
// implement access control on their own objects.
//----------------------------------------------------------------------------

WINADVAPI
DWORD
WINAPI
BuildSecurityDescriptorA(
    _In_opt_ PTRUSTEE_A              pOwner,
    _In_opt_ PTRUSTEE_A              pGroup,
    _In_ ULONG                   cCountOfAccessEntries,
    _In_reads_opt_(cCountOfAccessEntries) PEXPLICIT_ACCESS_A      pListOfAccessEntries,
    _In_ ULONG                   cCountOfAuditEntries,
    _In_reads_opt_(cCountOfAuditEntries) PEXPLICIT_ACCESS_A      pListOfAuditEntries,
    _In_opt_ PSECURITY_DESCRIPTOR    pOldSD,
    _Out_ PULONG                  pSizeNewSD,
    _Outptr_result_bytebuffer_(*pSizeNewSD) PSECURITY_DESCRIPTOR  * pNewSD
    );
WINADVAPI
DWORD
WINAPI
BuildSecurityDescriptorW(
    _In_opt_ PTRUSTEE_W              pOwner,
    _In_opt_ PTRUSTEE_W              pGroup,
    _In_ ULONG                   cCountOfAccessEntries,
    _In_reads_opt_(cCountOfAccessEntries) PEXPLICIT_ACCESS_W      pListOfAccessEntries,
    _In_ ULONG                   cCountOfAuditEntries,
    _In_reads_opt_(cCountOfAuditEntries) PEXPLICIT_ACCESS_W      pListOfAuditEntries,
    _In_opt_ PSECURITY_DESCRIPTOR    pOldSD,
    _Out_ PULONG                  pSizeNewSD,
    _Outptr_result_bytebuffer_(*pSizeNewSD) PSECURITY_DESCRIPTOR  * pNewSD
    );
#ifdef UNICODE
#define BuildSecurityDescriptor  BuildSecurityDescriptorW
#else
#define BuildSecurityDescriptor  BuildSecurityDescriptorA
#endif // !UNICODE

WINADVAPI
DWORD
WINAPI
LookupSecurityDescriptorPartsA(
    _Out_opt_ PTRUSTEE_A         * ppOwner,
    _Out_opt_ PTRUSTEE_A         * ppGroup,
    _Out_opt_ PULONG               pcCountOfAccessEntries,
    _Outptr_result_buffer_maybenull_(*pcCountOfAccessEntries) PEXPLICIT_ACCESS_A * ppListOfAccessEntries,
    _Out_opt_ PULONG               pcCountOfAuditEntries,
    _Outptr_result_buffer_maybenull_(*pcCountOfAuditEntries) PEXPLICIT_ACCESS_A * ppListOfAuditEntries,
    _In_  PSECURITY_DESCRIPTOR pSD
    );
WINADVAPI
DWORD
WINAPI
LookupSecurityDescriptorPartsW(
    _Out_opt_ PTRUSTEE_W         * ppOwner,
    _Out_opt_ PTRUSTEE_W         * ppGroup,
    _Out_opt_ PULONG               pcCountOfAccessEntries,
    _Outptr_result_buffer_maybenull_(*pcCountOfAccessEntries) PEXPLICIT_ACCESS_W * ppListOfAccessEntries,
    _Out_opt_ PULONG               pcCountOfAuditEntries,
    _Outptr_result_buffer_maybenull_(*pcCountOfAuditEntries) PEXPLICIT_ACCESS_W * ppListOfAuditEntries,
    _In_  PSECURITY_DESCRIPTOR pSD
    );
#ifdef UNICODE
#define LookupSecurityDescriptorParts  LookupSecurityDescriptorPartsW
#else
#define LookupSecurityDescriptorParts  LookupSecurityDescriptorPartsA
#endif // !UNICODE


//----------------------------------------------------------------------------
// The following helper API are provided for building
// access control structures.
//----------------------------------------------------------------------------

WINADVAPI
VOID
WINAPI
BuildExplicitAccessWithNameA(
    _Inout_    PEXPLICIT_ACCESS_A  pExplicitAccess,
    _In_opt_   LPSTR             pTrusteeName,
    _In_       DWORD               AccessPermissions,
    _In_       ACCESS_MODE         AccessMode,
    _In_       DWORD               Inheritance
    );
WINADVAPI
VOID
WINAPI
BuildExplicitAccessWithNameW(
    _Inout_    PEXPLICIT_ACCESS_W  pExplicitAccess,
    _In_opt_   LPWSTR             pTrusteeName,
    _In_       DWORD               AccessPermissions,
    _In_       ACCESS_MODE         AccessMode,
    _In_       DWORD               Inheritance
    );
#ifdef UNICODE
#define BuildExplicitAccessWithName  BuildExplicitAccessWithNameW
#else
#define BuildExplicitAccessWithName  BuildExplicitAccessWithNameA
#endif // !UNICODE

WINADVAPI
VOID
WINAPI
BuildImpersonateExplicitAccessWithNameA(
    _Inout_ PEXPLICIT_ACCESS_A  pExplicitAccess,
    _In_opt_ LPSTR             pTrusteeName,
    _In_opt_ PTRUSTEE_A          pTrustee,
    _In_    DWORD               AccessPermissions,
    _In_    ACCESS_MODE         AccessMode,
    _In_    DWORD               Inheritance
    );
WINADVAPI
VOID
WINAPI
BuildImpersonateExplicitAccessWithNameW(
    _Inout_ PEXPLICIT_ACCESS_W  pExplicitAccess,
    _In_opt_ LPWSTR             pTrusteeName,
    _In_opt_ PTRUSTEE_W          pTrustee,
    _In_    DWORD               AccessPermissions,
    _In_    ACCESS_MODE         AccessMode,
    _In_    DWORD               Inheritance
    );
#ifdef UNICODE
#define BuildImpersonateExplicitAccessWithName  BuildImpersonateExplicitAccessWithNameW
#else
#define BuildImpersonateExplicitAccessWithName  BuildImpersonateExplicitAccessWithNameA
#endif // !UNICODE

WINADVAPI
VOID
WINAPI
BuildTrusteeWithNameA(
    _Inout_	PTRUSTEE_A  pTrustee,
    _In_opt_ LPSTR     pName
    );
WINADVAPI
VOID
WINAPI
BuildTrusteeWithNameW(
    _Inout_	PTRUSTEE_W  pTrustee,
    _In_opt_ LPWSTR     pName
    );
#ifdef UNICODE
#define BuildTrusteeWithName  BuildTrusteeWithNameW
#else
#define BuildTrusteeWithName  BuildTrusteeWithNameA
#endif // !UNICODE

WINADVAPI
VOID
WINAPI
BuildImpersonateTrusteeA(
    _Inout_ PTRUSTEE_A  pTrustee,
    _In_opt_ PTRUSTEE_A  pImpersonateTrustee
    );
WINADVAPI
VOID
WINAPI
BuildImpersonateTrusteeW(
    _Inout_ PTRUSTEE_W  pTrustee,
    _In_opt_ PTRUSTEE_W  pImpersonateTrustee
    );
#ifdef UNICODE
#define BuildImpersonateTrustee  BuildImpersonateTrusteeW
#else
#define BuildImpersonateTrustee  BuildImpersonateTrusteeA
#endif // !UNICODE

WINADVAPI
VOID
WINAPI
BuildTrusteeWithSidA(
    _Inout_ PTRUSTEE_A  pTrustee,
    _In_opt_ PSID        pSid
    );
WINADVAPI
VOID
WINAPI
BuildTrusteeWithSidW(
    _Inout_ PTRUSTEE_W  pTrustee,
    _In_opt_ PSID        pSid
    );
#ifdef UNICODE
#define BuildTrusteeWithSid  BuildTrusteeWithSidW
#else
#define BuildTrusteeWithSid  BuildTrusteeWithSidA
#endif // !UNICODE

WINADVAPI
VOID
WINAPI
BuildTrusteeWithObjectsAndSidA(
    _Inout_ PTRUSTEE_A         pTrustee,
    _In_opt_ POBJECTS_AND_SID   pObjSid,
    _In_opt_ GUID             * pObjectGuid,
    _In_opt_ GUID             * pInheritedObjectGuid,
    _In_opt_ PSID               pSid
    );
WINADVAPI
VOID
WINAPI
BuildTrusteeWithObjectsAndSidW(
    _Inout_ PTRUSTEE_W         pTrustee,
    _In_opt_ POBJECTS_AND_SID   pObjSid,
    _In_opt_ GUID             * pObjectGuid,
    _In_opt_ GUID             * pInheritedObjectGuid,
    _In_opt_ PSID               pSid
    );
#ifdef UNICODE
#define BuildTrusteeWithObjectsAndSid  BuildTrusteeWithObjectsAndSidW
#else
#define BuildTrusteeWithObjectsAndSid  BuildTrusteeWithObjectsAndSidA
#endif // !UNICODE

WINADVAPI
VOID
WINAPI
BuildTrusteeWithObjectsAndNameA(
    _Inout_ PTRUSTEE_A          pTrustee,
    _In_opt_ POBJECTS_AND_NAME_A pObjName,
    _In_opt_ SE_OBJECT_TYPE      ObjectType,
    _In_opt_ LPSTR             ObjectTypeName,
    _In_opt_ LPSTR             InheritedObjectTypeName,
    _In_opt_ LPSTR             Name
    );
WINADVAPI
VOID
WINAPI
BuildTrusteeWithObjectsAndNameW(
    _Inout_ PTRUSTEE_W          pTrustee,
    _In_opt_ POBJECTS_AND_NAME_W pObjName,
    _In_opt_ SE_OBJECT_TYPE      ObjectType,
    _In_opt_ LPWSTR             ObjectTypeName,
    _In_opt_ LPWSTR             InheritedObjectTypeName,
    _In_opt_ LPWSTR             Name
    );
#ifdef UNICODE
#define BuildTrusteeWithObjectsAndName  BuildTrusteeWithObjectsAndNameW
#else
#define BuildTrusteeWithObjectsAndName  BuildTrusteeWithObjectsAndNameA
#endif // !UNICODE

WINADVAPI
LPSTR
WINAPI
GetTrusteeNameA(
    _In_ PTRUSTEE_A  pTrustee
    );
WINADVAPI
LPWSTR
WINAPI
GetTrusteeNameW(
    _In_ PTRUSTEE_W  pTrustee
    );
#ifdef UNICODE
#define GetTrusteeName  GetTrusteeNameW
#else
#define GetTrusteeName  GetTrusteeNameA
#endif // !UNICODE

WINADVAPI
TRUSTEE_TYPE
WINAPI
GetTrusteeTypeA(
    _In_opt_ PTRUSTEE_A  pTrustee
    );
WINADVAPI
TRUSTEE_TYPE
WINAPI
GetTrusteeTypeW(
    _In_opt_ PTRUSTEE_W  pTrustee
    );
#ifdef UNICODE
#define GetTrusteeType  GetTrusteeTypeW
#else
#define GetTrusteeType  GetTrusteeTypeA
#endif // !UNICODE

WINADVAPI
TRUSTEE_FORM
WINAPI
GetTrusteeFormA(
    _In_ PTRUSTEE_A  pTrustee
    );
WINADVAPI
TRUSTEE_FORM
WINAPI
GetTrusteeFormW(
    _In_ PTRUSTEE_W  pTrustee
    );
#ifdef UNICODE
#define GetTrusteeForm  GetTrusteeFormW
#else
#define GetTrusteeForm  GetTrusteeFormA
#endif // !UNICODE

WINADVAPI
MULTIPLE_TRUSTEE_OPERATION
WINAPI
GetMultipleTrusteeOperationA(
    _In_opt_ PTRUSTEE_A  pTrustee
    );
WINADVAPI
MULTIPLE_TRUSTEE_OPERATION
WINAPI
GetMultipleTrusteeOperationW(
    _In_opt_ PTRUSTEE_W  pTrustee
    );
#ifdef UNICODE
#define GetMultipleTrusteeOperation  GetMultipleTrusteeOperationW
#else
#define GetMultipleTrusteeOperation  GetMultipleTrusteeOperationA
#endif // !UNICODE

WINADVAPI
PTRUSTEE_A
WINAPI
GetMultipleTrusteeA(
    _In_opt_ PTRUSTEE_A  pTrustee
    );
WINADVAPI
PTRUSTEE_W
WINAPI
GetMultipleTrusteeW(
    _In_opt_ PTRUSTEE_W  pTrustee
    );
#ifdef UNICODE
#define GetMultipleTrustee  GetMultipleTrusteeW
#else
#define GetMultipleTrustee  GetMultipleTrusteeA
#endif // !UNICODE

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

//
// Temporary requirement for the technology preview, no longer required
//
#define AccProvInit(err)

#ifdef __cplusplus
}
#endif


#endif // __ACCESS_CONTROL_API__

